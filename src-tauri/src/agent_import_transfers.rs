//! The transfer pairing view where the tools meet it: what a build is refused
//! for, and the `transfers` section of a build, of a statement of only skipped
//! rows, and of the first page of a verification.
//!
//! The view itself is `transfer_pairing_view`, a pure function of the facts the
//! journal gives (`statement_facts`). Here it is run under the import admission
//! lock, with a build about to be written read as the newest record, so a
//! refusal is decided before anything is written and the section a build
//! returns is the one it was admitted on. Every row is named by its
//! `bridge_txn_id` and date: no amount, reference, instrument, party or ledger
//! leaves this module.
use super::*;
use crate::agent::transfer_pairing::{
    transfer_pairing_view, BatchId, BatchStatus, ContraKey, DoubleBookedClass, SkipState,
    TransferView,
};
use statement_facts::{
    BuildLine, Candidate, FactsError, StatementFacts, TransferFacts, TransferFactsError,
};

/// How many days apart the two statements may print one transfer and still
/// pair it by date. Zero until a real statement pair has been measured: k comes
/// from that pair (the design's C12), not from synthetic rows, so only the same
/// printed day matches today.
pub(super) const TRANSFER_K_DAYS: u32 = 0;

/// The argument that names skipped rows a build posts anyway.
pub(super) const WITHDRAW_SKIPS_KEY: &str = "withdraw_skips";

/// The `withdraw_skips` argument, taken out of the build's arguments. Each is a
/// statement row id, once, and no more than a build can hold vouchers.
pub(super) fn take_withdraw_skips(args: &mut Value) -> Result<BTreeSet<String>, String> {
    let invalid = || "argument_schema_invalid".to_string();
    let Some(value) = args
        .as_object_mut()
        .and_then(|map| map.remove(WITHDRAW_SKIPS_KEY))
    else {
        return Ok(BTreeSet::new());
    };
    let items = value.as_array().ok_or_else(invalid)?;
    if items.len() > MAX_VOUCHERS {
        return Err(invalid());
    }
    let mut rows = BTreeSet::new();
    for item in items {
        let row = item
            .as_str()
            .filter(|row| bridge_bank_statement::proposals::is_statement_txn_id(row))
            .ok_or_else(invalid)?;
        if !rows.insert(row.to_string()) {
            return Err(invalid());
        }
    }
    Ok(rows)
}

// ---- the summary the section is made of ----------------------------------------

/// What an open row of the section reads. Each label is written here and
/// nowhere else, and a state cannot be added without its label and its step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OpenState {
    NotFound,
    Ambiguous,
    OtherSideUnverified,
    NotEstablished,
    ReversedSameDay,
    MatchedUnconfirmed,
    ReferenceDiffers,
    HeldElsewhere,
    DoubleBooked,
}

impl OpenState {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::NotFound => "not_found",
            Self::Ambiguous => "ambiguous",
            Self::OtherSideUnverified => "other_side_unverified",
            Self::NotEstablished => "not_established",
            Self::ReversedSameDay => "reversed_same_day",
            Self::MatchedUnconfirmed => "matched_unconfirmed",
            Self::ReferenceDiffers => "reference_differs",
            Self::HeldElsewhere => "held_elsewhere",
            Self::DoubleBooked => "double_booked",
        }
    }

    /// What to do about an open row of this state, for the assistant to follow.
    /// A row waiting on a batch not verified yet is told by whether that batch
    /// was sent; this is the step for rows of both kinds.
    #[cfg(test)]
    pub(super) fn step(self) -> &'static str {
        self.step_for(Unverified {
            sent: true,
            never_sent: true,
        })
    }

    /// [`Self::step`], for the rows waiting on a batch not verified yet that
    /// are of the kinds given.
    pub(super) fn step_for(self, unverified: Unverified) -> &'static str {
        match self {
            Self::OtherSideUnverified => unverified.pick(
                SECTION_UNVERIFIED_NEVER_SENT_STEP,
                SECTION_UNVERIFIED_SENT_STEP,
                SECTION_UNVERIFIED_EITHER_STEP,
            ),
            Self::NotFound => "The other account's statement for this date went through ComplyEaze Bridge and holds no Contra for this row dated the same day; a Contra dated another day is not looked for. If no batch of that statement covers this date, build it. If that statement skipped this row too, withdraw the skip on exactly one side. Do not enter this transfer in Tally by hand: a later post of the other statement would book it twice.",
            Self::Ambiguous => "Each batch named holds a Contra that could carry this row: more than one Contra could, or the one that could is also claimed by another row or booked twice. ComplyEaze Bridge does not choose; check in Tally which Contra carries this transfer. A Contra in a batch not yet verified is not in Tally until that batch is imported and verified.",
            Self::ReversedSameDay => "This row, or the other account's matching row, has an equal amount going the other way the same day on its own statement, so a reversal and a pair of real transfers look alike. Where a batch is named, it holds a Contra that could carry this row. Check in Tally which rows are transfers. Name the row in withdraw_skips only when the person confirms the other account's statement does not carry it.",
            Self::MatchedUnconfirmed => "A verified Contra of the same day and amount, booked from the other account's statement, is in the batch named. Whether it is the only one is not established, because the statement records of the two accounts do not both cover this date. Keep the skip. The row can be decided only once statement records of both accounts cover this date: for each account that needs a statement whose rows begin before this date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on this date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). Until then the row stays as it is.",
            Self::ReferenceDiffers => "A Contra of the same day and amount in the batch named, booked from the other account's statement, carries a different reference from this row. ComplyEaze Bridge cannot tell whether it is this row's other side. If that batch is not verified yet, run verify_import on it first. Then check that Contra in Tally: keep the skip if it carries this transfer, and name the row in withdraw_skips only when the person confirms it does not.",
            Self::HeldElsewhere => "The batch named posts this row, which a statement skipped. Verify that batch; once it is found posted the skip is retired.",
            Self::DoubleBooked => "Two statements each booked this transfer as a Contra. Check both in Tally and keep one.",
            Self::NotEstablished => NOT_ESTABLISHED_STEP,
        }
    }
}

/// Whether the batches that a refusal or a section names as not verified yet
/// were sent to Tally or never were. A batch that was never sent can be retired
/// by reading Tally for it; a sent one is settled by verifying it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Unverified {
    pub(super) sent: bool,
    pub(super) never_sent: bool,
}

impl Unverified {
    pub(super) fn of(sent: bool) -> Self {
        Self {
            sent,
            never_sent: !sent,
        }
    }

    pub(super) fn and(self, other: Self) -> Self {
        Self {
            sent: self.sent || other.sent,
            never_sent: self.never_sent || other.never_sent,
        }
    }

    /// The text for the kinds present; for both, or for none, the one that
    /// covers both.
    fn pick(
        self,
        never_sent: &'static str,
        sent: &'static str,
        either: &'static str,
    ) -> &'static str {
        match (self.sent, self.never_sent) {
            (true, false) => sent,
            (false, true) => never_sent,
            _ => either,
        }
    }
}

const SECTION_UNVERIFIED_NEVER_SENT_STEP: &str = "The other account's Contra for this row is in the batch named, which is not verified yet and was never sent. Run verify_import on that batch. If it will not be sent, run verify_import on it with if_absent set to retire: ComplyEaze Bridge retires it only if Tally does not hold it. Do not build or post either statement again for this row.";
const SECTION_UNVERIFIED_SENT_STEP: &str = "The other account's Contra for this row is in the batch named, which was sent to Tally and has not been settled. Run verify_import on that batch. Do not build or post either statement again for this row.";
const SECTION_UNVERIFIED_EITHER_STEP: &str = "The other account's Contra for this row is in the batch named, which is not verified yet. Run verify_import on that batch. A batch that was never sent and will not be can be run with if_absent set to retire: ComplyEaze Bridge retires it only if Tally does not hold it; a batch that was sent to Tally cannot be retired yet. Do not build or post either statement again for this row.";

/// The batches that hold the Contras that could carry a row, each once: the
/// row is listed once for each. A row nothing could carry is listed once,
/// with no batch.
fn batches_holding(candidates: &[ContraKey]) -> BTreeSet<Option<&BatchId>> {
    if candidates.is_empty() {
        return [None].into();
    }
    candidates
        .iter()
        .map(|candidate| Some(&candidate.batch))
        .collect()
}

/// A skipped row, or a Contra booked twice, that is not paired.
pub(super) struct OpenRow {
    id: String,
    date: TallyDate,
    state: OpenState,
    batch: Option<String>,
    /// The batch was sent to Tally. Meaningful for a row waiting on a batch
    /// not verified yet.
    sent: bool,
}

/// What the view said, as counts and open rows, before it is cut to a budget.
pub(super) enum TransferSummary {
    /// The view could not be computed or read.
    NotEstablished,
    Computed {
        paired: usize,
        skips_open: usize,
        retired: usize,
        /// Ordered by date, then id.
        open: Vec<OpenRow>,
    },
}

impl TransferSummary {
    pub(super) fn of(facts: &TransferFacts, view: &TransferView) -> Self {
        let mut paired = 0;
        let mut skips_open = facts.held_elsewhere.len();
        let mut open = Vec::new();
        let dates: BTreeMap<_, _> = facts
            .skips
            .iter()
            .map(|skip| (&skip.key, &skip.date))
            .collect();
        for (key, state) in &view.skips {
            let (state, batches): (OpenState, BTreeSet<Option<&BatchId>>) = match state {
                SkipState::Paired { .. } => {
                    paired += 1;
                    continue;
                }
                SkipState::NotFound => (OpenState::NotFound, [None].into()),
                SkipState::Ambiguous { candidates } => {
                    (OpenState::Ambiguous, batches_holding(candidates))
                }
                SkipState::OtherSideUnverified { batch } => {
                    (OpenState::OtherSideUnverified, [Some(batch)].into())
                }
                SkipState::NotEstablished => (OpenState::NotEstablished, [None].into()),
                SkipState::MatchedUnconfirmed { contra } => {
                    (OpenState::MatchedUnconfirmed, [Some(&contra.batch)].into())
                }
                SkipState::ReversedSameDay { candidates } => {
                    (OpenState::ReversedSameDay, batches_holding(candidates))
                }
                // one row for each batch that holds such a Contra
                SkipState::ReferenceDiffers { candidates } => (
                    OpenState::ReferenceDiffers,
                    candidates
                        .iter()
                        .map(|candidate| Some(&candidate.batch))
                        .collect(),
                ),
            };
            skips_open += 1;
            if let Some(date) = dates.get(key) {
                open.extend(batches.into_iter().map(|batch| OpenRow {
                    id: key.row.as_str().to_string(),
                    date: (*date).clone(),
                    state,
                    batch: batch.map(|batch| batch.as_str().to_string()),
                    sent: batch.is_some_and(|batch| facts.sent_batches.contains(batch)),
                }));
            }
        }
        for (key, holder, date) in &facts.held_elsewhere {
            open.push(OpenRow {
                id: key.row.as_str().to_string(),
                date: date.clone(),
                state: OpenState::HeldElsewhere,
                batch: Some(holder.as_str().to_string()),
                sent: false,
            });
        }
        // Each Contra of a pair booked from two statements is listed, naming the
        // other statement's batch.
        let contra_dates: BTreeMap<_, _> = facts
            .contras
            .iter()
            .map(|contra| (&contra.key, &contra.date))
            .collect();
        for booked in &view.double_booked {
            for (this, other) in [
                (&booked.first, &booked.second),
                (&booked.second, &booked.first),
            ] {
                if let Some(date) = contra_dates.get(this) {
                    open.push(OpenRow {
                        id: this.txn.as_str().to_string(),
                        date: (*date).clone(),
                        state: OpenState::DoubleBooked,
                        batch: Some(other.batch.as_str().to_string()),
                        sent: false,
                    });
                }
            }
        }
        open.sort_by(|left, right| (&left.date, &left.id).cmp(&(&right.date, &right.id)));
        Self::Computed {
            paired,
            skips_open,
            retired: facts.retired_skips,
            open,
        }
    }

    /// The section, with as many open rows as `available` bytes hold once its
    /// frame is paid for. The rows that do not fit are counted, and the step for
    /// that says how to see them.
    pub(super) fn render(&self, available: usize) -> Value {
        let Self::Computed {
            paired,
            skips_open,
            retired,
            open,
        } = self
        else {
            return check_not_run_section(false);
        };
        let mut next_steps = serde_json::Map::new();
        // rows waiting on a batch not verified yet are told by whether that
        // batch was sent, over every such row, listed or not
        let unverified = open
            .iter()
            .filter(|row| row.state == OpenState::OtherSideUnverified)
            .fold(Unverified::default(), |kinds, row| {
                kinds.and(Unverified::of(row.sent))
            });
        for row in open {
            next_steps
                .entry(row.state.label())
                .or_insert_with(|| json!(row.state.step_for(unverified)));
        }
        let frame = json!({
            "state": "computed", "skips_paired": paired, "skips_open": skips_open,
            "open": [], "open_omitted": open.len(), "retired": retired,
            "next_steps": next_steps.clone(),
        });
        // the step for omitted rows is paid for whether or not it is needed
        let frame_len = frame.to_string().len() + OPEN_OMITTED_STEP.len() + 32;
        let mut room = available.saturating_sub(frame_len);
        let mut rows = Vec::new();
        for row in open {
            let rendered = match &row.batch {
                Some(batch) => json!({"bridge_txn_id": row.id, "date": row.date,
                    "state": row.state.label(), "batch_id": batch}),
                None => {
                    json!({"bridge_txn_id": row.id, "date": row.date, "state": row.state.label()})
                }
            };
            // `,` between rows
            let cost = rendered.to_string().len() + 1;
            if cost > room {
                break;
            }
            room -= cost;
            rows.push(rendered);
        }
        let omitted = open.len() - rows.len();
        if omitted > 0 {
            next_steps.insert("open_omitted".into(), json!(OPEN_OMITTED_STEP));
        }
        json!({
            "state": "computed", "skips_paired": paired, "skips_open": skips_open,
            "open": rows, "open_omitted": omitted, "retired": retired,
            "next_steps": next_steps,
        })
    }
}

const NOT_ESTABLISHED_STEP: &str = "Whether the other account's Contra exists is not established: the other account's statement for this date is not built and verified through ComplyEaze Bridge, or a Contra on that bank on this date is one the statement records cannot see or use (built from inline vouchers or by a release before them, recorded without a facts entry, or not two equal legs). Verify any batch of the other account's statement for this date first; build that statement only if no batch of it exists. If a batch without statement records already carries this transfer, keep the skip.";
/// The whole check could not run, which says nothing of any one row. A batch
/// that was written, or a verification, is checked again by verifying it.
const CHECK_NOT_RUN_STEP: &str =
    "Not run, or cut to fit: verify again; if it repeats, raise BRIDGE_AGENT_MAX_BYTES.";
/// A statement of only skipped rows has no batch to verify.
const CHECK_NOT_RUN_RECORDED_STEP: &str = "The skipped rows are recorded; the transfer check will show on the next build or verification for this company.";
const OPEN_OMITTED_STEP: &str =
    "Some open rows are not listed because the response is full. Raise BRIDGE_AGENT_MAX_BYTES to list every open row.";

/// The section with its first `keep` open rows, the rest counted in
/// `open_omitted`. The response cap's fitter uses it, so a section never makes
/// a call fail as too large.
pub(in crate::agent) fn trim_section(section: &Value, keep: usize) -> Value {
    let rows = section["open"].as_array().map_or(0, Vec::len);
    let keep = keep.min(rows);
    let mut trimmed = section.clone();
    if let Some(open) = trimmed["open"].as_array_mut() {
        open.truncate(keep);
    }
    let omitted = section["open_omitted"].as_u64().unwrap_or(0) + (rows - keep) as u64;
    trimmed["open_omitted"] = json!(omitted);
    if omitted > 0 {
        trimmed["next_steps"]["open_omitted"] = json!(OPEN_OMITTED_STEP);
    }
    trimmed
}

/// The counts alone, with the one step that says how to see the rest: for a
/// response so full that not even the step texts fit.
pub(in crate::agent) fn counts_only_section(section: &Value) -> Value {
    let mut counts = trim_section(section, 0);
    counts["next_steps"] = if counts["open_omitted"].as_u64().unwrap_or(0) > 0 {
        json!({"open_omitted": OPEN_OMITTED_STEP})
    } else {
        json!({})
    };
    counts
}

/// What a section says when the check could not run, or when not even its
/// counts fit beside the page: run the verification again, with a larger cap
/// if the note comes back. It claims no count.
pub(in crate::agent) fn check_not_run_section(recorded: bool) -> Value {
    json!({
        "state": "not_established",
        "next_steps": {"check_not_run": if recorded {
            CHECK_NOT_RUN_RECORDED_STEP
        } else {
            CHECK_NOT_RUN_STEP
        }},
    })
}

/// The bytes the section may use: what the response cap leaves after the
/// essential result, counting that a result is sent twice (as structured
/// content and as text), and never more than a quarter of the cap so the rest
/// of the answer keeps its room.
pub(super) fn transfers_budget(max_bytes: usize, essential_len: usize) -> usize {
    (max_bytes / 2)
        .saturating_sub(essential_len)
        .min(max_bytes / 4)
}

// ---- what a build is refused for -------------------------------------------------

/// A refusal in the shape of the build's other refusals.
pub(super) struct TransferRefusal {
    reason: &'static str,
    detail: Value,
    next_step: String,
}

/// A refusal made when a batch is sent, not when it is built.
const NOTHING_SENT: &str = "Nothing was sent to Tally and the approval was not used.";
const SEND_ENDING: &str = "This batch cannot be sent as built: build the statement again.";
const TRANSFER_VIEW_NOT_ESTABLISHED_SEND_STEP: &str = "Nothing was sent to Tally and the approval was not used. ComplyEaze Bridge could not read its own record of the skipped rows and Contras of this company, so it cannot tell whether this batch books a transfer twice. error, check and record_id name what failed, without any value from the record. Report it with the error named here: no batch of this company that posts a statement row can be sent until it is fixed, as it already blocks every build that needs the view.";
/// What a refusal under the admission lock says: the reason is its code alone.
pub(super) const TRANSFERS_CHANGED_UNDER_LOCK_STEP: &str = "The transfer records changed after this batch was checked and before it was sent; nothing was sent and the approval was not used. Call post_import again: it gives the whole reason before asking.";

impl TransferRefusal {
    pub(super) fn reason(&self) -> &'static str {
        self.reason
    }

    /// The same refusal for a batch about to be sent: one function maps every
    /// build step, so the two cannot drift.
    pub(super) fn for_send(self) -> Self {
        let next_step = if self.reason == "transfer_view_not_established" {
            TRANSFER_VIEW_NOT_ESTABLISHED_SEND_STEP.to_string()
        } else {
            // A step that speaks of the build it refuses has its own wording for a
            // batch about to be sent; the others speak of rows, and serve both.
            let middle = send_wording(self.reason).unwrap_or_else(|| {
                self.next_step
                    .strip_prefix(NOTHING_WRITTEN)
                    .unwrap_or(&self.next_step)
                    .trim()
            });
            format!("{NOTHING_SENT} {middle} {SEND_ENDING}")
        };
        Self { next_step, ..self }
    }

    /// The reason, what the refusal lists, and what to do about it.
    pub(super) fn into_parts(self) -> (&'static str, Value, String) {
        (self.reason, self.detail, self.next_step)
    }

    pub(super) fn outcome(
        self,
        company: &bridge_tally_protocol::TallyCompany,
        evidence: &Evidence,
        company_guid: &str,
    ) -> ToolOutcome {
        let mut result = json!({"state": "refused", "reason": self.reason});
        if let (Some(result), Some(detail)) = (result.as_object_mut(), self.detail.as_object()) {
            result.extend(detail.clone());
            result.insert("next_step".into(), json!(self.next_step));
        }
        ToolOutcome {
            payload: json!({"company": company_json(company, std::slice::from_ref(company)),
                "result": result}),
            evidence: evidence.clone(),
            company_guid: Some(company_guid.to_string()),
            truncated: false,
        }
    }
}

/// What the transfer check at send decided about a saved batch.
pub(super) struct SendCheck {
    /// Why the batch is refused, when it is.
    pub(super) refusal: Option<TransferRefusal>,
    /// When it is not refused: the retired batches the decision rests on,
    /// because counting any of them would refuse it. The send reads Tally for
    /// each before it goes on.
    pub(super) rests_on_retired: BTreeSet<String>,
}

impl SendCheck {
    fn clear() -> Self {
        Self {
            refusal: None,
            rests_on_retired: BTreeSet::new(),
        }
    }

    fn refused(refusal: TransferRefusal) -> Self {
        Self {
            refusal: Some(refusal),
            rests_on_retired: BTreeSet::new(),
        }
    }
}

const RETIRED_BATCH_FOUND_STEP: &str = "Nothing was sent to Tally and the approval was not used. This batch can be sent only because a batch of this company was retired after a read of Tally found it absent, and ComplyEaze Bridge read Tally again for that batch before sending: a voucher there carries the narration tag of a batch named in retired_batches. That batch counts again, so this batch cannot be sent as built. Run verify_import on the batch named in retired_batches, then build the statement again.";

/// The refusal of a send whose second read of Tally found a retired batch's own
/// tag: the batches it found are named.
pub(super) fn retired_batch_found(batches: &[String]) -> TransferRefusal {
    TransferRefusal {
        reason: "retired_batch_found_in_tally",
        detail: json!({"retired_batches": batches}),
        next_step: RETIRED_BATCH_FOUND_STEP.into(),
    }
}

const RETIRED_BATCH_NOT_READ_STEP: &str = "Nothing was sent to Tally and the approval was not used. This batch can be sent only because a batch of this company was retired after a read of Tally found it absent, and ComplyEaze Bridge could not read Tally again for that retired batch before sending: retired_batches names it and cause says why the read failed. The retired batch is still out of the company's transfer records, and this batch is not sent until it has been read again. Call post_import again; if cause names a refusal that a repeat will not change, report it with the cause named here.";

/// The refusal of a send whose second read of a retired batch failed, whatever
/// the failure: the batch it could not read, and why.
pub(super) fn retired_batch_not_read(batch: &str, cause: &str) -> TransferRefusal {
    TransferRefusal {
        reason: "retired_batch_not_read",
        detail: json!({"retired_batches": [batch], "cause": cause}),
        next_step: RETIRED_BATCH_NOT_READ_STEP.into(),
    }
}

pub(super) enum Admission {
    Admitted(TransferSummary),
    Refused(TransferRefusal),
}

/// A build, or a statement of only skipped rows, as the transfer checks see it.
pub(super) struct TransferInput {
    pub(super) company_guid: String,
    /// The build as the journal will read it once written.
    pub(super) candidate: Candidate,
    /// The `st-` ids of the rows the build skips.
    pub(super) skip_rows: BTreeSet<String>,
    /// The `st-` ids of the vouchers the build posts, of any type.
    pub(super) voucher_rows: BTreeSet<String>,
    pub(super) withdraw: BTreeSet<String>,
    /// The lineage an amendment belongs to.
    pub(super) own_lineage: Option<String>,
    /// The build carries a statement's facts.
    pub(super) carries_statement: bool,
    /// A batch already in the journal that is being judged again, the one about
    /// to be sent: its records are left out of the facts and it is the
    /// `candidate`, so the journal reads as it did when the batch was built, plus
    /// whatever was written since.
    pub(super) subject: Option<String>,
    /// Batches the journal retired that are read as if it had not: they count
    /// as they did before a read of Tally found them absent.
    pub(super) count_retired: BTreeSet<String>,
}

impl TransferInput {
    pub(super) fn candidate_build(&self) -> Option<&str> {
        match &self.candidate {
            Candidate::Build(line) => Some(line.batch_id.as_str()),
            Candidate::SkipOnly(_) => None,
        }
    }

    /// A build that carries statement facts, posts a statement row, or asks to
    /// withdraw a skip is safe only if the facts can be read. Any other build is
    /// not about transfers.
    fn needs_the_view(&self) -> bool {
        self.carries_statement || !self.voucher_rows.is_empty() || !self.withdraw.is_empty()
    }
}

/// The facts could not be read: refused when the build needs them, otherwise
/// the build goes on and its section says the view is not established.
fn without_facts(needs_the_view: bool, error: &TransferFactsError) -> Admission {
    if needs_the_view {
        Admission::Refused(TransferRefusal {
            reason: "transfer_view_not_established",
            detail: json!({"error": error.variant(), "check": error.check(),
                "record_id": error.record_id()}),
            next_step: TRANSFER_VIEW_NOT_ESTABLISHED_NEXT_STEP.into(),
        })
    } else {
        Admission::Admitted(TransferSummary::NotEstablished)
    }
}

const SKIP_ROW_ALREADY_POSTED_NEXT_STEP: &str = "No file was written and nothing was recorded. This build skips a row that is already a voucher of a batch of this company: blocking_batch_id names it, and it was sent to Tally, was found posted, or is an earlier build of the batch being amended. A skip cannot take a voucher out of Tally, and it would say that another account's Contra carries a transfer this company already booked. Do not skip the rows listed in skipped_rows. If the batch named is not verified yet, run verify_import on it. Then parse the statement again with those rows not skipped (a row that batch holds is then refused as import_txn_already_posted) and build again.";
const SKIP_ROW_PAIRED_ELSEWHERE_NEXT_STEP: &str = "No file was written and nothing was recorded. This build posts a row that a statement of this company skipped as carried by the other account's Contra: skipped_rows lists them. Posting the row books its own bank, and if the other account's Contra exists the transfer is booked twice. Do not post these rows. Parse the statement again with those rows mapped skip, or build again naming a row in withdraw_skips only when the person confirms the other account's statement does not carry it; ask for each, and never name a row on their behalf.";
const WITHDRAW_SKIP_INVALID_NEXT_STEP: &str = "No file was written and nothing was recorded. Each row in invalid_rows is not both a skip that ComplyEaze Bridge holds for this company (in a batch sent to Tally or found posted, or in a statement of only skipped rows) and a voucher of this build. Build again with withdraw_skips naming only rows that are both, or without it.";
const TRANSFER_DOUBLE_BOOKED_NEXT_STEP: &str = "No file was written and nothing was recorded. A Contra in this build carries a transfer that the batch named in other_batch_id already booked as a Contra from the other account's statement (the references are equal). Posting both books it twice. Do not build this Contra: run verify_import on the other batch if it is not verified, then parse this statement again with that row skipped (treatment skip, naming the other account's ledger) and build again.";
const TRANSFER_VIEW_NOT_ESTABLISHED_NEXT_STEP: &str = "No file was written and nothing was recorded. ComplyEaze Bridge could not read its own record of the skipped rows and Contras of this company, so it cannot tell whether this build books a transfer twice. error, check and record_id name what failed, without any value from the record. ComplyEaze Bridge runs the same checks before it writes those records, so this comes from a defect in it, or from records a later release of ComplyEaze Bridge wrote in this folder: if a later release has been used here, build with that release; otherwise report it with the error named here, and do not build this statement again until it is fixed.";

/// What a refusal says of a batch about to be sent, for the reasons whose step
/// speaks of the build it refuses, between "nothing was sent" and the ending.
fn send_wording(reason: &str) -> Option<&'static str> {
    match reason {
        "skip_row_already_posted" => Some("This batch skips a row that is already a voucher of a batch of this company: blocking_batch_id names it, and it was sent to Tally, was found posted, or is an earlier build of the batch being amended. A skip cannot take a voucher out of Tally, and it would say that another account's Contra carries a transfer this company already booked. If the batch named is not verified yet, run verify_import on it. Then parse the statement again with the rows listed in skipped_rows not skipped (a row that batch holds is then refused as import_txn_already_posted)."),
        "skip_row_paired_elsewhere" => Some("This batch posts a row that a statement of this company skipped as carried by the other account's Contra: skipped_rows lists them. Posting the row books its own bank, and if the other account's Contra exists the transfer is booked twice. Parse the statement again with those rows mapped skip, or build again naming a row in withdraw_skips only when the person confirms the other account's statement does not carry it; ask for each, and never name a row on their behalf."),
        "withdraw_skip_invalid" => Some("Each row in invalid_rows is not both a skip that ComplyEaze Bridge holds for this company (in a batch sent to Tally or found posted, or in a statement of only skipped rows) and a voucher of this batch. Name in withdraw_skips only rows that are both, or none."),
        "transfer_double_booked" => Some("A Contra in this batch carries a transfer that the batch named in other_batch_id already booked as a Contra from the other account's statement (the references are equal). Sending both books it twice. Run verify_import on the other batch if it is not verified, then parse this statement again with that row skipped (treatment skip, naming the other account's ledger)."),
        _ => None,
    }
}

/// What the batch named beside a withdrawn row holds for it, which is why the
/// withdrawal is refused. The labels a refusal shares with the section are the
/// section's own, and a row need not carry the same one in both: a row the
/// section reads `ambiguous`, `reversed_same_day` or `reference_differs` is
/// listed in `paired_rows` as `other_side_unverified` when one of its
/// candidates is not verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Carrier {
    /// A verified Contra the row is paired with.
    Paired,
    /// A Contra that could carry the row, in a batch not yet verified.
    OtherSideUnverified,
    /// A verified Contra that matches the row by date, not established to be
    /// the only one.
    MatchedUnconfirmed,
}

const NOTHING_WRITTEN: &str = "No file was written and nothing was recorded.";

impl Carrier {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Paired => "paired",
            Self::OtherSideUnverified => OpenState::OtherSideUnverified.label(),
            Self::MatchedUnconfirmed => OpenState::MatchedUnconfirmed.label(),
        }
    }

    /// What to do when every refused row is of this kind, after the sentence
    /// that nothing was written.
    fn step(self, unverified: Unverified) -> &'static str {
        match self {
            Self::Paired => "The other account's statement already carries this row as a Contra in the batch named in paired_rows. If that Contra is wrong, correct that statement's batch first; do not post this row.",
            Self::OtherSideUnverified => unverified.pick(
                "The other account's statement may carry this row as a Contra in the batch named in paired_rows, which is not verified yet and was never sent. Run verify_import on that batch first. If it will not be sent, run verify_import on it with if_absent set to retire: ComplyEaze Bridge retires it only if Tally does not hold it. Do not post this row.",
                "The other account's statement may carry this row as a Contra in the batch named in paired_rows. That batch was sent to Tally and ComplyEaze Bridge has not settled it. Run verify_import on that batch first. Do not post this row.",
                "The other account's statement may carry this row as a Contra in the batch named in paired_rows, which is not verified yet. Run verify_import on that batch first. A batch that was never sent and will not be can be run with if_absent set to retire: ComplyEaze Bridge retires it only if Tally does not hold it; a batch that was sent to Tally cannot be retired yet. Do not post this row.",
            ),
            Self::MatchedUnconfirmed => "A verified Contra of the same day and amount as this row, booked from the other account's statement, is in the batch named in paired_rows. Whether it is the only one is not established, so the skip stays: do not post this row. The row can be decided only once statement records of both accounts cover its date: for each account that needs a statement whose rows begin before that date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on that date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). Until then the row stays as it is.",
        }
    }

    /// The same, for a row of this kind among rows of other kinds.
    fn among(self, unverified: Unverified) -> &'static str {
        match self {
            Self::Paired => "the other account's statement already carries it as a Contra in the batch named in paired_rows: if that Contra is wrong, correct that statement's batch first.",
            Self::OtherSideUnverified => unverified.pick(
                "the other account's statement may carry it as a Contra in the batch named, which is not verified yet and was never sent: run verify_import on that batch first, or, if it will not be sent, run verify_import on it with if_absent set to retire.",
                "the other account's statement may carry it as a Contra in the batch named, which was sent to Tally and has not been settled: run verify_import on that batch first.",
                "the other account's statement may carry it as a Contra in the batch named, which is not verified yet: run verify_import on that batch first. One that was never sent and will not be can be run with if_absent set to retire, and one that was sent to Tally cannot be retired yet.",
            ),
            Self::MatchedUnconfirmed => "a verified Contra of the same day and amount, booked from the other account's statement, is in the batch named, and whether it is the only one is not established: keep the skip. The row can be decided only once statement records of both accounts cover its date: for each account that needs a statement whose rows begin before that date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on that date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). Until then the row stays as it is.",
        }
    }

    /// What to do about withdrawn rows that are refused, by every kind among
    /// them: the rows a list leaves out count too.
    pub(super) fn refusal_step(kinds: &BTreeSet<Self>, unverified: Unverified) -> String {
        let mut each = kinds.iter();
        if let (Some(only), None) = (each.next(), each.next()) {
            return format!("{NOTHING_WRITTEN} {}", only.step(unverified));
        }
        let rows: String = kinds
            .iter()
            .map(|kind| {
                format!(
                    " For a row with state {}, {}",
                    kind.label(),
                    kind.among(unverified)
                )
            })
            .collect();
        format!("{NOTHING_WRITTEN}{rows} Do not post these rows.")
    }
}

/// The most `st-` ids a refusal lists.
const REFUSAL_ROWS_LISTED: usize = 50;

fn rows_listed(rows: impl IntoIterator<Item = String>) -> (Vec<String>, usize) {
    let all: Vec<String> = rows.into_iter().collect();
    let listed: Vec<String> = all.iter().take(REFUSAL_ROWS_LISTED).cloned().collect();
    let omitted = all.len() - listed.len();
    (listed, omitted)
}

fn rows_listed_json(rows: Vec<Value>) -> (Vec<Value>, usize) {
    let omitted = rows.len().saturating_sub(REFUSAL_ROWS_LISTED);
    (
        rows.into_iter().take(REFUSAL_ROWS_LISTED).collect(),
        omitted,
    )
}

/// The batch holding the other Contra of a double booking that refuses the
/// build, when one of the two Contras is in `ours`.
fn double_booked_with<'a>(view: &'a TransferView, ours: &str) -> Option<&'a str> {
    view.double_booked.iter().find_map(|booked| {
        if booked.class != DoubleBookedClass::RefuseAtBuild {
            None
        } else if booked.first.batch.as_str() == ours {
            Some(booked.second.batch.as_str())
        } else if booked.second.batch.as_str() == ours {
            Some(booked.first.batch.as_str())
        } else {
            None
        }
    })
}

impl Server {
    /// The skip-only file, empty when there is none yet.
    fn skip_file_while_admitted(&self) -> Result<Box<dyn std::io::BufRead>, String> {
        let path = self.settings.data_dir.join(statement_facts::SKIPS_FILE);
        match super::super::local_file::open_local_file(&path, false) {
            Ok(file) => Ok(Box::new(std::io::BufReader::new(file))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok(Box::new(std::io::empty()))
            }
            Err(_) => Err("import_ledger_unavailable".to_string()),
        }
    }

    /// The facts of one company and the view over them, with a candidate read as
    /// the newest record when one is given.
    ///
    /// `subject`, `withdrawn` and `counted_retired` are as for
    /// `ledger::transfer_facts_scoped`.
    pub(super) fn transfer_view_while_admitted(
        &self,
        company_guid: &str,
        candidate: Option<Candidate>,
        subject: Option<&str>,
        withdrawn: &BTreeSet<String>,
        counted_retired: &BTreeSet<String>,
    ) -> Result<(TransferFacts, TransferView), TransferFactsError> {
        let journal: Box<dyn std::io::BufRead> = match self.import_journal_while_admitted() {
            Ok(Some(reader)) => Box::new(reader),
            Ok(None) => Box::new(std::io::empty()),
            Err(code) => return Err(TransferFactsError::JournalUnreadable { code }),
        };
        let skips = self
            .skip_file_while_admitted()
            .map_err(|code| TransferFactsError::SkipFileUnreadable { code })?;
        let facts = ledger::transfer_facts_scoped(
            journal,
            skips,
            company_guid,
            candidate,
            subject,
            withdrawn,
            counted_retired,
        )?;
        let view = transfer_pairing_view(
            &facts.skips,
            &facts.contras,
            &facts.coverage,
            TRANSFER_K_DAYS,
        )?;
        Ok((facts, view))
    }

    /// Decide a build, or a statement of only skipped rows, against the
    /// company's other batches and skips. Called with the exclusive admission
    /// lock held and before anything is written. `Err` is a journal that cannot
    /// be read; a refusal is `Ok`.
    pub(super) fn admit_transfers_while_admitted(
        &self,
        input: TransferInput,
    ) -> Result<Admission, String> {
        // C5b, from the journal alone: the view is not consulted.
        if !input.skip_rows.is_empty() {
            if let Some(journal) = self.import_journal_while_admitted()? {
                if let Some((batch, rows)) = ledger::rows_held_as_vouchers(
                    journal,
                    &input.company_guid,
                    input.own_lineage.as_deref(),
                    &input.skip_rows,
                )? {
                    let (listed, omitted) = rows_listed(rows);
                    return Ok(Admission::Refused(TransferRefusal {
                        reason: "skip_row_already_posted",
                        detail: json!({"blocking_batch_id": batch, "skipped_rows": listed,
                            "skipped_rows_omitted": omitted}),
                        next_step: SKIP_ROW_ALREADY_POSTED_NEXT_STEP.into(),
                    }));
                }
            }
        }
        // C5a: the skips there are before this build. A row this build
        // withdraws is judged as if no batch that was neither sent nor found
        // held it or booked it: this build proposes to book it itself.
        let (before, before_view) = match self.transfer_view_while_admitted(
            &input.company_guid,
            None,
            input.subject.as_deref(),
            &input.withdraw,
            &input.count_retired,
        ) {
            Ok(view) => view,
            Err(error) => return Ok(without_facts(input.needs_the_view(), &error)),
        };
        let live: BTreeSet<&str> = before
            .live_skips
            .iter()
            .map(|key| key.row.as_str())
            .collect();
        let invalid: Vec<String> = input
            .withdraw
            .iter()
            .filter(|row| !live.contains(row.as_str()) || !input.voucher_rows.contains(*row))
            .cloned()
            .collect();
        if !invalid.is_empty() {
            let (listed, omitted) = rows_listed(invalid);
            return Ok(Admission::Refused(TransferRefusal {
                reason: "withdraw_skip_invalid",
                detail: json!({"invalid_rows": listed, "invalid_rows_omitted": omitted}),
                next_step: WITHDRAW_SKIP_INVALID_NEXT_STEP.into(),
            }));
        }
        // A skip the other statement already carries, one that a single verified
        // Contra matches by date, or one a Contra may carry once it is verified,
        // is not one to withdraw. A person can confirm only what Tally holds, so
        // any candidate not yet verified refuses, whatever the state: each of
        // their batches is named. Not found, not established, and ambiguous,
        // reversed the same day or of another reference among verified Contras
        // only, are.
        // One row can be skipped under several keys (two endpoints, two bank
        // ledgers): every one of them is read, live or not, which can only
        // refuse more.
        let verified: BTreeSet<&ContraKey> = before
            .contras
            .iter()
            .filter(|contra| contra.status == BatchStatus::PostedVerified)
            .map(|contra| &contra.key)
            .collect();
        let carried: Vec<(&String, &BatchId, Carrier)> = input
            .withdraw
            .iter()
            .flat_map(|row| {
                let carriers: BTreeSet<(&BatchId, Carrier)> = before_view
                    .skips
                    .iter()
                    .filter(|(key, _)| key.row.as_str() == row)
                    .flat_map(|(_, state)| match state {
                        SkipState::Paired { contra, .. } => vec![(&contra.batch, Carrier::Paired)],
                        SkipState::MatchedUnconfirmed { contra } => {
                            vec![(&contra.batch, Carrier::MatchedUnconfirmed)]
                        }
                        SkipState::OtherSideUnverified { batch } => {
                            vec![(batch, Carrier::OtherSideUnverified)]
                        }
                        SkipState::Ambiguous { candidates }
                        | SkipState::ReversedSameDay { candidates }
                        | SkipState::ReferenceDiffers { candidates } => candidates
                            .iter()
                            .filter(|candidate| !verified.contains(candidate))
                            .map(|candidate| (&candidate.batch, Carrier::OtherSideUnverified))
                            .collect(),
                        SkipState::NotFound | SkipState::NotEstablished => Vec::new(),
                    })
                    .collect();
                carriers
                    .into_iter()
                    .map(move |(batch, carrier)| (row, batch, carrier))
            })
            .collect();
        if !carried.is_empty() {
            let kinds = carried.iter().map(|&(_, _, carrier)| carrier).collect();
            // a batch not verified yet that was sent is settled by verifying
            // it; one never sent can also be retired
            let unverified = carried
                .iter()
                .filter(|&&(_, _, carrier)| carrier == Carrier::OtherSideUnverified)
                .fold(Unverified::default(), |kinds, &(_, batch, _)| {
                    kinds.and(Unverified::of(before.sent_batches.contains(batch)))
                });
            let next_step = Carrier::refusal_step(&kinds, unverified);
            let rows = carried
                .into_iter()
                .map(|(row, batch, carrier)| {
                    json!({"bridge_txn_id": row, "batch_id": batch.as_str(),
                        "state": carrier.label()})
                })
                .collect();
            let (listed, omitted) = rows_listed_json(rows);
            return Ok(Admission::Refused(TransferRefusal {
                reason: "withdraw_skip_paired",
                detail: json!({"paired_rows": listed, "paired_rows_omitted": omitted}),
                next_step,
            }));
        }
        let paired_elsewhere: Vec<String> = input
            .voucher_rows
            .iter()
            .filter(|row| live.contains(row.as_str()) && !input.withdraw.contains(*row))
            .cloned()
            .collect();
        if !paired_elsewhere.is_empty() {
            let (listed, omitted) = rows_listed(paired_elsewhere);
            return Ok(Admission::Refused(TransferRefusal {
                reason: "skip_row_paired_elsewhere",
                detail: json!({"skipped_rows": listed, "skipped_rows_omitted": omitted}),
                next_step: SKIP_ROW_PAIRED_ELSEWHERE_NEXT_STEP.into(),
            }));
        }
        // C6: the view with this build in it.
        let candidate_batch = input.candidate_build().map(str::to_string);
        let needs_the_view = input.needs_the_view();
        let (facts, view) = match self.transfer_view_while_admitted(
            &input.company_guid,
            Some(input.candidate),
            input.subject.as_deref(),
            &BTreeSet::new(),
            &input.count_retired,
        ) {
            Ok(view) => view,
            Err(error) => return Ok(without_facts(needs_the_view, &error)),
        };
        let other_batch = candidate_batch
            .as_deref()
            .and_then(|ours| double_booked_with(&view, ours));
        if let Some(other) = other_batch {
            return Ok(Admission::Refused(TransferRefusal {
                reason: "transfer_double_booked",
                detail: json!({"other_batch_id": other}),
                next_step: TRANSFER_DOUBLE_BOOKED_NEXT_STEP.into(),
            }));
        }
        Ok(Admission::Admitted(TransferSummary::of(&facts, &view)))
    }

    /// A saved batch about to be sent, judged as its build was and against the
    /// journal as it is now. No refusal when the batch is admitted, or is not
    /// about transfers at all (it posts no statement row, carries no statement
    /// and withdraws nothing), as the build decides it. When it is admitted, the
    /// check also names the retired batches that decision rests on: those whose
    /// being counted would refuse it. Called with the admission lock held,
    /// shared or exclusive.
    pub(super) fn transfers_check_for_send_while_admitted(
        &self,
        line: &ImportLedgerLine,
    ) -> Result<SendCheck, String> {
        let statement = match &line.statement {
            Some(recorded) => match serde_json::from_value::<StatementFacts>(recorded.clone()) {
                Ok(facts) => Some(facts),
                Err(_) => {
                    let error = TransferFactsError::Facts {
                        source_id: line.batch_id.clone(),
                        error: FactsError::Unparsable,
                    };
                    return Ok(match without_facts(true, &error) {
                        Admission::Refused(refusal) => SendCheck::refused(refusal.for_send()),
                        Admission::Admitted(_) => SendCheck::clear(),
                    });
                }
            },
            None => None,
        };
        // judged as it was built, with the retired batches in `count_retired`
        // read as the journal did not retire them
        let judge = |count_retired: BTreeSet<String>| -> Result<Option<Admission>, String> {
            let mut candidate = BuildLine::of(line);
            candidate.status = "built".to_string();
            let input = TransferInput {
                company_guid: canonical_batch_guid(&line.company_guid),
                candidate: Candidate::Build(Box::new(candidate)),
                skip_rows: statement
                    .iter()
                    .flat_map(|statement| &statement.skip_rows)
                    .map(|skip| skip.bridge_txn_id.clone())
                    .collect(),
                voucher_rows: line
                    .vouchers
                    .iter()
                    .filter(|voucher| {
                        bridge_bank_statement::proposals::is_statement_txn_id(
                            &voucher.bridge_txn_id,
                        )
                    })
                    .map(|voucher| voucher.bridge_txn_id.clone())
                    .collect(),
                withdraw: statement
                    .iter()
                    .flat_map(|statement| statement.withdrawn.iter().cloned())
                    .collect(),
                own_lineage: line.amends_batch_id.clone(),
                carries_statement: statement.is_some(),
                subject: Some(line.batch_id.clone()),
                count_retired,
            };
            if !input.needs_the_view() {
                return Ok(None);
            }
            self.admit_transfers_while_admitted(input).map(Some)
        };
        let refuses =
            |admission: Option<Admission>| matches!(admission, Some(Admission::Refused(_)));
        match judge(BTreeSet::new())? {
            None => return Ok(SendCheck::clear()),
            Some(Admission::Refused(refusal)) => return Ok(SendCheck::refused(refusal.for_send())),
            Some(Admission::Admitted(_)) => {}
        }
        // Admitted with every retired batch out of the facts. It rests on one
        // whose being counted would refuse it. If none does alone but all
        // together do, it rests on all of them.
        let retired = match self.import_journal_while_admitted()? {
            Some(reader) => {
                ledger::retired_batches(reader, &canonical_batch_guid(&line.company_guid))?
            }
            None => BTreeSet::new(),
        };
        let mut rests_on = BTreeSet::new();
        for batch in &retired {
            if refuses(judge(BTreeSet::from([batch.clone()]))?) {
                rests_on.insert(batch.clone());
            }
        }
        if rests_on.is_empty() && !retired.is_empty() && refuses(judge(retired.clone())?) {
            rests_on = retired;
        }
        Ok(SendCheck {
            refusal: None,
            rests_on_retired: rests_on,
        })
    }

    /// The section for the first page of a verification: computed after the
    /// verdict is persisted, under a shared admission lock, and never able to
    /// change the verdict or fail the call. Contention or any failure is
    /// `not_established`.
    pub(super) fn transfers_for_verification(
        &self,
        company_guid: &str,
        essential_len: usize,
    ) -> Value {
        let summary = {
            let guard = self.lock_import_admission_shared();
            guard
                .ok()
                .and_then(|_lock| {
                    self.transfer_view_while_admitted(
                        &canonical_batch_guid(company_guid),
                        None,
                        None,
                        &BTreeSet::new(),
                        &BTreeSet::new(),
                    )
                    .ok()
                })
                .map_or(TransferSummary::NotEstablished, |(facts, view)| {
                    TransferSummary::of(&facts, &view)
                })
        };
        summary.render(transfers_budget(self.settings.max_bytes, essential_len))
    }
}

#[cfg(test)]
#[path = "agent_import_transfers_tests.rs"]
mod tests;
