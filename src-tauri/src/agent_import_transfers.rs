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
    transfer_pairing_view, with_days_read, BatchId, BatchStatus, ContraKey, DayReader,
    DoubleBookedClass, LedgerGuid, SkipFact, SkipState, TransferView,
};
use statement_facts::{
    BuildLine, Candidate, FactsError, HeldBy, StatementFacts, TransferFacts, TransferFactsError,
};

/// How many days apart the two statements may print one transfer and still
/// pair it by date. Zero until a real statement pair has been measured: k comes
/// from that pair (the design's C12), not from synthetic rows, so only the same
/// printed day matches today.
pub(super) const TRANSFER_K_DAYS: u32 = 0;
// A whole-day read settles a skip on its own day only (#1392): a k above 0
// needs the neighbouring days read too.
const _: () = assert!(TRANSFER_K_DAYS == 0);

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
    TallyUnchecked,
    Ambiguous,
    OtherSideUnverified,
    NotEstablished,
    ReversedSameDay,
    MatchedUnconfirmed,
    ReferenceDiffers,
    HeldElsewhere,
    PostedAgainstSkip,
    DoubleBooked,
}

impl OpenState {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::NotFound => "not_found",
            Self::TallyUnchecked => "tally_unchecked",
            Self::Ambiguous => "ambiguous",
            Self::OtherSideUnverified => "other_side_unverified",
            Self::NotEstablished => "not_established",
            Self::ReversedSameDay => "reversed_same_day",
            Self::MatchedUnconfirmed => "matched_unconfirmed",
            Self::ReferenceDiffers => "reference_differs",
            Self::HeldElsewhere => "held_elsewhere",
            Self::PostedAgainstSkip => "posted_against_skip",
            Self::DoubleBooked => "double_booked",
        }
    }

    /// What to do about an open row of this state, for the assistant to follow.
    pub(super) fn step(self) -> &'static str {
        match self {
            Self::OtherSideUnverified => "The other account's Contra for this row is in the batch named, which is not verified yet. Run verify_import on every batch named for this row; do not post this row. Build the other account's statement for this row's date again only if every batch named for this row was never sent to Tally, verify_import listed every one of their vouchers as not_found, and the person confirms none of their files was ever imported into Tally; then post the new build, verify it, and never import an old batch's file. If verify_import found any voucher of a batch named for this row, or refused to read one, or a file of one was imported, do not build the other account's statement again: check in Tally which of their vouchers are there, and no step inside ComplyEaze Bridge frees this row yet. If Tally rejected the send of a batch named for this row, building the other account's statement again is refused as import_txn_already_posted, and no step inside ComplyEaze Bridge frees this row yet.",
            Self::TallyUnchecked => "The other account's statement for this date went through ComplyEaze Bridge and holds no Contra for this row dated the same day, but whether Tally holds one is not established: ComplyEaze Bridge did not read every voucher of this date in Tally, or could not find one of the two ledgers in the ledger list it read, or read them all and found a voucher of this date with an entry on each of the two ledgers, which it does not compare with this row, so a Contra entered in Tally directly is not ruled out. Read the vouchers of this date with the vouchers tool and look for a voucher of the Contra voucher class, or of any other type such as a Journal or Payment, that carries this row's amount on the bank ledger the other account's statement was built for, and read its narration where it was not dropped; a voucher dated another day, or booked against another ledger, is not looked for, a transfer may print on different days, and a masked ledger name can read like another, so compare amounts and dates too. Tell the person what you found and keep the skip. Do not book this transfer in Tally another way, and withdraw the skip on one side only: a later post of the other statement would book it twice. Name the row in withdraw_skips only when the person confirms that no entry in Tally carries this transfer.",
            Self::NotFound => "The other account's statement for this date went through ComplyEaze Bridge and holds no Contra for this row dated the same day, and ComplyEaze Bridge read every voucher of this date in Tally and found no Contra between the two accounts that could carry it; a Contra dated another day, or booked against another ledger, is not looked for. If no batch of that statement covers this date, build it. If that statement skipped this row too, withdraw the skip on exactly one side, and only when the person confirms. Do not enter this transfer in Tally by hand: a later post of the other statement would book it twice.",
            Self::Ambiguous => "Each batch named holds a Contra that could carry this row: more than one Contra could, or the one that could is also claimed by another row or booked twice. ComplyEaze Bridge does not choose; check in Tally which Contra carries this transfer. A Contra in a batch not yet verified is not in Tally until that batch is imported and verified.",
            Self::ReversedSameDay => "This row, or the other account's matching row, has an equal amount going the other way the same day on its own statement, so a reversal and a pair of real transfers look alike. Where a batch is named, it holds a Contra that could carry this row. Check in Tally which rows are transfers. Name the row in withdraw_skips only when the person confirms the other account's statement does not carry it.",
            Self::MatchedUnconfirmed => "A verified Contra of the same day and amount, booked from the other account's statement, is in the batch named. Whether it is the only one is not established, because the statement records of the two accounts do not both cover this date. Keep the skip. The row can be decided only once statement records of both accounts cover this date: for each account that needs a statement whose rows begin before this date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on this date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). A statement build abandoned unsent, never sent to Tally and never verified, keeps its days uncovered, and no step inside ComplyEaze Bridge clears one yet. Until then the row stays as it is.",
            Self::ReferenceDiffers => "A Contra of the same day and amount in the batch named, booked from the other account's statement, carries a different reference from this row. ComplyEaze Bridge cannot tell whether it is this row's other side. If that batch is not verified yet, run verify_import on it first. Then check that Contra in Tally: keep the skip if it carries this transfer, and name the row in withdraw_skips only when the person confirms it does not.",
            Self::HeldElsewhere => "The batch named posts this row, which a statement skipped, and has not been found posted. Verify that batch. The skip is retired once every batch found posted that holds the row has a build in its lineage that recorded the withdrawal: a build that names this row in withdraw_skips records it, and it counts once that build is sent or found posted and was built after the newest record of the skip (a later build of the skipping statement records the skip again). If the batch named does not name this row in withdraw_skips and has not been sent, build the statement that posts it again naming this row in withdraw_skips, and do not send the batch named or import its file: found posted, either leaves the row as posted_against_skip, and once a build that skips the row has been sent or found posted, or a skip-only record holds the skip, post_import refuses that send as skip_row_paired_elsewhere. If the batch named does name this row, send it and verify it. A batch left unsent blocks the days of its rows for the other account's coverage until it is verified, and no step inside ComplyEaze Bridge clears that. A batch found posted without the withdrawal leaves the row as posted_against_skip.",
            Self::PostedAgainstSkip => "A statement skipped this row as carried by the other account's Contra, and the batch named, found posted, posts it as a voucher although no build in its lineage that was sent or found posted recorded its withdrawal after the newest record of the skip. If the other account's Contra also carries this transfer, it is booked twice. ComplyEaze Bridge does not compare the two, and no step inside ComplyEaze Bridge removes or changes a voucher already in Tally. The row leaves this list only when a build in that batch's lineage that names it in withdraw_skips is found posted after the newest record of the skip, and only when the person confirms the other account's statement does not carry it; a batch ComplyEaze Bridge posted itself cannot be amended, so for it no step inside ComplyEaze Bridge clears the row. Tell the person.",
            Self::DoubleBooked => "Two statements each booked this transfer as a Contra. No step inside ComplyEaze Bridge removes or changes a voucher already in Tally yet. If either batch named has not been sent to Tally, do not send it, and do not build or post anything further for this transfer; tell the person.",
            Self::NotEstablished => NOT_ESTABLISHED_STEP,
        }
    }
}

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
    /// A batch it names is one a recorded rollback covers (#1050).
    restored: bool,
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
        let mut skips_open = facts.held.len();
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
                SkipState::TallyUnchecked => (OpenState::TallyUnchecked, [None].into()),
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
                    restored: batch.is_some_and(|batch| facts.restored.contains(batch.as_str())),
                    batch: batch.map(|batch| batch.as_str().to_string()),
                }));
            }
        }
        for held in &facts.held {
            open.push(OpenRow {
                id: held.fact.key.row.as_str().to_string(),
                date: held.fact.date.clone(),
                state: match held.by {
                    HeldBy::UnverifiedBatch => OpenState::HeldElsewhere,
                    HeldBy::PostedBatch => OpenState::PostedAgainstSkip,
                },
                restored: facts.restored.contains(held.holder.as_str()),
                batch: Some(held.holder.as_str().to_string()),
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
                        // the row names the other batch, so the marker is its
                        restored: facts.restored.contains(other.batch.as_str()),
                        batch: Some(other.batch.as_str().to_string()),
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
        for row in open {
            next_steps
                .entry(row.state.label())
                .or_insert_with(|| json!(row.state.step()));
        }
        // paid for in the frame, like the state steps, whether or not the
        // marked row is listed; a section cut to its counts drops it with them
        if open.iter().any(|row| row.restored) {
            next_steps.insert("restored".into(), json!(RESTORED_NOTE));
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
            let mut rendered = match &row.batch {
                Some(batch) => json!({"bridge_txn_id": row.id, "date": row.date,
                    "state": row.state.label(), "batch_id": batch}),
                None => {
                    json!({"bridge_txn_id": row.id, "date": row.date, "state": row.state.label()})
                }
            };
            if row.restored {
                rendered["restored"] = json!(true);
            }
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
/// What a row marked restored means, whatever its state (#1050): the batch it
/// names is one a recorded rollback covers, which no step settles.
pub(super) const RESTORED_NOTE: &str = "A row marked restored names a batch whose vouchers a restore that ComplyEaze Bridge recorded may have removed from Tally. Whatever the row's state says, that batch stays unverified here even after verify_import, and no step inside ComplyEaze Bridge settles the row: do not name it in withdraw_skips, and do not build or amend its rows. Check in Tally whether that batch's voucher for the row is there, and tell the person. Do not enter this transfer in Tally by hand, and never import that batch's file: a later re-post of its rows would book it twice.";
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

/// The codes a transfer check refuses a send with.
pub(super) const SEND_REFUSALS: [&str; 6] = [
    "skip_row_already_posted",
    "withdraw_skip_invalid",
    "withdraw_skip_paired",
    "skip_row_paired_elsewhere",
    "transfer_double_booked",
    "transfer_view_not_established",
];

/// What the desktop is told for a transfer refusal that sent nothing: the
/// agent's step does not reach its webview, so the message says what it says.
pub(super) const TRANSFER_REFUSED_MESSAGE: &str = "Nothing was sent. The company's transfer records do not allow this batch to be sent as built: build it again. Asking for approval again refuses the same way.";
pub(super) const TRANSFER_VIEW_REFUSED_MESSAGE: &str = "Nothing was sent. ComplyEaze Bridge could not read its own record of the skipped rows and Contras of this company, so it cannot tell whether this batch books a transfer twice. Building it again meets the same record: report it, or send with the later release that wrote it.";

/// A refusal made when a batch is sent, not when it is built.
const NOTHING_SENT: &str =
    "Nothing was sent to Tally, and any approval of this batch is withdrawn.";
const SEND_ENDING: &str = "This batch cannot be sent as built: build it again.";
/// The ending of a withdrawal refused at send: the steps before it may speak
/// of building the other account's statement, so this one names its own build.
const SEND_ENDING_KEEP_SKIPPED: &str = "This batch cannot be sent as built: build this statement again with the rows in paired_rows still skipped, not named in withdraw_skips.";
const TRANSFER_VIEW_NOT_ESTABLISHED_SEND_STEP: &str = "Nothing was sent to Tally, and any approval of this batch is withdrawn. ComplyEaze Bridge could not read its own record of the skipped rows and Contras of this company, so it cannot tell whether this batch books a transfer twice. error, check and record_id name what failed, without any value from the record. ComplyEaze Bridge runs the same checks before it writes those records, so this comes from a defect in it, or from records a later release of ComplyEaze Bridge wrote in this folder: if a later release has been used here, send with that release; otherwise report it with the error named here. No batch of this company that posts a statement row can be sent until it is fixed, as it already blocks every build that needs the view.";
/// What a refusal under the admission lock says: the reason is its code alone.
pub(super) const TRANSFERS_CHANGED_UNDER_LOCK_STEP: &str = "The transfer records changed after this batch was checked and before it was sent; nothing was sent and any approval of this batch is withdrawn. Call post_import again: it gives the whole reason before asking.";

impl TransferRefusal {
    pub(super) fn reason(&self) -> &'static str {
        self.reason
    }

    /// The same refusal for a batch about to be sent: one function maps every
    /// build step, so the two cannot drift. Every one but
    /// `transfer_view_not_established` ends by saying the statement must be built
    /// again (a refused withdrawal, with its rows still skipped); that one has no
    /// such ending, because building it again meets the same unreadable record.
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
            let ending = if self.reason == "withdraw_skip_paired" {
                SEND_ENDING_KEEP_SKIPPED
            } else {
                SEND_ENDING
            };
            format!("{NOTHING_SENT} {middle} {ending}")
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

pub(super) enum Admission {
    Admitted(Admitted),
    Refused(TransferRefusal),
}

/// The facts a build was admitted on, or nothing when the view could not be
/// established. The view it was admitted on is not kept: the section's is the
/// one [`Admitted::summary_with_days_read`] computes.
pub(super) struct Admitted(Option<TransferFacts>);

impl Admitted {
    /// The section's summary, after reading the days of its `tally_unchecked`
    /// rows from Tally (#1392), with the evidence of the reads that came back.
    /// It starts from the facts the build was admitted on, and the days change
    /// only which of those rows read `not_found`, which no admission reads, so
    /// it is called once nothing is left to refuse. `ledgers` is the
    /// catalogue this call read: a skip's ledgers are named by their GUIDs,
    /// spelled as the voucher rows spell them, and a ledger not in it keeps
    /// its rows `tally_unchecked`.
    pub(super) async fn summary_with_days_read<R: DayReader>(
        self,
        reader: &R,
        identity: &crate::tally::VerifiedCompanyIdentity,
        company: &str,
        ledgers: &bridge_tally_protocol::StandardLedgerCatalog,
    ) -> (TransferSummary, Option<Evidence>) {
        let Some(facts) = self.0 else {
            return (TransferSummary::NotEstablished, None);
        };
        let names: BTreeMap<&str, &str> = ledgers
            .identified_parents()
            .map(|(name, guid, _)| (guid, name))
            .collect();
        match with_days_read(
            reader,
            identity,
            company,
            &facts.skips,
            &facts.contras,
            &facts.coverage,
            |guid: &LedgerGuid| names.get(guid.as_str()).map(|name| (*name).to_string()),
        )
        .await
        {
            Ok((view, reads)) => (TransferSummary::of(&facts, &view), reads.evidence),
            // the facts were admitted, so this is not reached
            Err(_) => (TransferSummary::NotEstablished, None),
        }
    }
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
        Admission::Admitted(Admitted(None))
    }
}

const SKIP_ROW_ALREADY_POSTED_NEXT_STEP: &str = "No file was written and nothing was recorded. This build skips a row that is already a voucher of a batch of this company: blocking_batch_id names it, and it was sent to Tally, was found posted, or is an earlier build of the batch being amended. A skip cannot take a voucher out of Tally, and it would say that another account's Contra carries a transfer this company already booked. Do not skip the rows listed in skipped_rows. If the batch named is not verified yet, run verify_import on it. Then parse the statement again with those rows not skipped (a row that batch holds is then refused as import_txn_already_posted) and build again.";
const SKIP_ROW_PAIRED_ELSEWHERE_NEXT_STEP: &str = "No file was written and nothing was recorded. This build posts a row that a statement of this company skipped as carried by the other account's Contra: skipped_rows lists them. Posting the row books its own bank, and if the other account's Contra exists the transfer is booked twice. Do not post these rows. Parse the statement again with those rows mapped skip, or build again naming a row in withdraw_skips only when the person confirms the other account's statement does not carry it; ask for each, and never name a row on their behalf.";
const WITHDRAW_SKIP_INVALID_NEXT_STEP: &str = "No file was written and nothing was recorded. Each row in invalid_rows is not both a skip that ComplyEaze Bridge holds for this company (in a batch, whether or not it was sent, or in a statement of only skipped rows) and a voucher of this build. Build again with withdraw_skips naming only rows that are both, or without it.";
const TRANSFER_DOUBLE_BOOKED_NEXT_STEP: &str = "No file was written and nothing was recorded. A Contra in this build carries a transfer that the batch named in other_batch_id already booked as a Contra from the other account's statement (the references are equal). Posting both books it twice. Do not build this Contra: run verify_import on the other batch if it is not verified, then parse this statement again with that row skipped (treatment skip, naming the other account's ledger) and build again.";
const TRANSFER_VIEW_NOT_ESTABLISHED_NEXT_STEP: &str = "No file was written and nothing was recorded. ComplyEaze Bridge could not read its own record of the skipped rows and Contras of this company, so it cannot tell whether this build books a transfer twice. error, check and record_id name what failed, without any value from the record. ComplyEaze Bridge runs the same checks before it writes those records, so this comes from a defect in it, or from records a later release of ComplyEaze Bridge wrote in this folder: if a later release has been used here, build with that release; otherwise report it with the error named here, and do not build this statement again until it is fixed.";

/// What a refusal says of a batch about to be sent, for the reasons whose step
/// speaks of the build it refuses, between "nothing was sent" and the ending.
fn send_wording(reason: &str) -> Option<&'static str> {
    match reason {
        "skip_row_already_posted" => Some("This batch skips a row that is already a voucher of a batch of this company: blocking_batch_id names it, and it was sent to Tally, was found posted, or is an earlier build of the batch being amended. A skip cannot take a voucher out of Tally, and it would say that another account's Contra carries a transfer this company already booked. If the batch named is not verified yet, run verify_import on it. Then parse the statement again with the rows listed in skipped_rows not skipped (a row that batch holds is then refused as import_txn_already_posted)."),
        "skip_row_paired_elsewhere" => Some("This batch posts a row that a statement of this company skipped as carried by the other account's Contra: skipped_rows lists them. Posting the row books its own bank, and if the other account's Contra exists the transfer is booked twice. Parse the statement again with those rows mapped skip, or build again naming a row in withdraw_skips only when the person confirms the other account's statement does not carry it; ask for each, and never name a row on their behalf."),
        "withdraw_skip_invalid" => Some("Each row in invalid_rows is not both a skip that ComplyEaze Bridge holds for this company (in a batch, whether or not it was sent, or in a statement of only skipped rows) and a voucher of this batch. Name in withdraw_skips only rows that are both, or none."),
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
    fn step(self) -> &'static str {
        match self {
            Self::Paired => "The other account's statement already carries this row as a Contra in the batch named in paired_rows (for a skip that a batch holds, it may carry it: ComplyEaze Bridge judges that skip alone, and another skipped row could share that Contra). If that Contra is wrong, correct that statement's batch first; do not post this row.",
            Self::OtherSideUnverified => "The other account's statement may carry this row as a Contra in a batch named in paired_rows, which is not verified yet. Run verify_import on every batch named for this row first; do not post this row. Build the other account's statement for this row's date again only if every batch named for this row was never sent to Tally, verify_import listed every one of their vouchers as not_found, and the person confirms none of their files was ever imported into Tally; then post the new build, verify it, and never import an old batch's file. If verify_import found any voucher of a batch named for this row, or refused to read one, or a file of one was imported, do not build the other account's statement again: check in Tally which of their vouchers are there, and no step inside ComplyEaze Bridge frees this row yet. If Tally rejected the send of a batch named for this row, building the other account's statement again is refused as import_txn_already_posted, and no step inside ComplyEaze Bridge frees this row yet.",
            Self::MatchedUnconfirmed => "A verified Contra of the same day and amount as this row, booked from the other account's statement, is in the batch named in paired_rows. Whether it is the only one is not established, so the skip stays: do not post this row. The row can be decided only once statement records of both accounts cover its date: for each account that needs a statement whose rows begin before that date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on that date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). A statement build abandoned unsent, never sent to Tally and never verified, keeps its days uncovered, and no step inside ComplyEaze Bridge clears one yet. Until then the row stays as it is.",
        }
    }

    /// The same, for a row of this kind among rows of other kinds.
    fn among(self) -> &'static str {
        match self {
            Self::Paired => "the other account's statement already carries it as a Contra in the batch named in paired_rows (for a skip that a batch holds, it may carry it: that skip is judged alone, and another skipped row could share that Contra): if that Contra is wrong, correct that statement's batch first.",
            Self::OtherSideUnverified => "the other account's statement may carry it as a Contra in a batch named, which is not verified yet: run verify_import on every batch named for this row first; build the other account's statement for this row's date again only if every batch named for this row was never sent to Tally, verify_import listed every one of their vouchers as not_found, and the person confirms none of their files was ever imported into Tally; then post the new build, verify it, and never import an old batch's file; if verify_import found any voucher of a batch named for this row, or refused to read one, or a file of one was imported, do not build the other account's statement again: check in Tally which of their vouchers are there, and no step inside ComplyEaze Bridge frees this row yet; if Tally rejected the send of a batch named for this row, building the other account's statement again is refused as import_txn_already_posted, and no step inside ComplyEaze Bridge frees this row yet.",
            Self::MatchedUnconfirmed => "a verified Contra of the same day and amount, booked from the other account's statement, is in the batch named, and whether it is the only one is not established: keep the skip. The row can be decided only once statement records of both accounts cover its date: for each account that needs a statement whose rows begin before that date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on that date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). A statement build abandoned unsent, never sent to Tally and never verified, keeps its days uncovered, and no step inside ComplyEaze Bridge clears one yet. Until then the row stays as it is.",
        }
    }

    /// [`Self::refusal_step`], with [`RESTORED_NOTE`] after it when a row it
    /// refuses names a batch a recorded rollback covers.
    pub(super) fn refusal_step_restored(kinds: &BTreeSet<Self>, restored: bool) -> String {
        let step = Self::refusal_step(kinds);
        if restored {
            format!("{step} {RESTORED_NOTE}")
        } else {
            step
        }
    }

    /// What to do about withdrawn rows that are refused, by every kind among
    /// them: the rows a list leaves out count too.
    pub(super) fn refusal_step(kinds: &BTreeSet<Self>) -> String {
        let mut each = kinds.iter();
        if let (Some(only), None) = (each.next(), each.next()) {
            return format!("{NOTHING_WRITTEN} {}", only.step());
        }
        let rows: String = kinds
            .iter()
            .map(|kind| format!(" For a row with state {}, {}", kind.label(), kind.among()))
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

/// The view over the held skips alone, against the company's Contras. A held
/// skip is judged without the skips that are in the main view: beside them, a
/// Contra they compete for could read ambiguous, which a withdrawal may pass.
pub(super) fn held_carrier_view(
    facts: &TransferFacts,
) -> Result<TransferView, crate::agent::transfer_pairing::PairingInputError> {
    let held: Vec<SkipFact> = facts
        .held
        .iter()
        .filter(|held| held.pairs)
        .map(|held| held.fact.clone())
        .collect();
    transfer_pairing_view(&held, &facts.contras, &facts.coverage, TRANSFER_K_DAYS)
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
    /// the newest record when one is given. `subject` is as for
    /// `ledger::transfer_facts`.
    pub(super) fn transfer_view_while_admitted(
        &self,
        company_guid: &str,
        candidate: Option<Candidate>,
        subject: Option<&str>,
    ) -> Result<(TransferFacts, TransferView), TransferFactsError> {
        let journal: Box<dyn std::io::BufRead> = match self.import_journal_while_admitted() {
            Ok(Some(reader)) => Box::new(reader),
            Ok(None) => Box::new(std::io::empty()),
            Err(code) => return Err(TransferFactsError::JournalUnreadable { code }),
        };
        let skips = self
            .skip_file_while_admitted()
            .map_err(|code| TransferFactsError::SkipFileUnreadable { code })?;
        let facts = ledger::transfer_facts(journal, skips, company_guid, candidate, subject)?;
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
        // C5a: the skips there are before this build.
        let (before, before_view) = match self.transfer_view_while_admitted(
            &input.company_guid,
            None,
            input.subject.as_deref(),
        ) {
            Ok(view) => view,
            Err(error) => return Ok(without_facts(input.needs_the_view(), &error)),
        };
        let live: BTreeSet<&str> = before
            .live_skips
            .iter()
            .map(|key| key.row.as_str())
            .collect();
        // A skip is withdrawn whether or not its statement was ever sent: a
        // statement parsed again with the row posted is a build that posts it.
        let mentioned: BTreeSet<&str> = before
            .skip_mentions
            .iter()
            .map(|key| key.row.as_str())
            .collect();
        let invalid: Vec<String> = input
            .withdraw
            .iter()
            .filter(|row| !mentioned.contains(row.as_str()) || !input.voucher_rows.contains(*row))
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
        // A skip whose row a batch holds is outside the view. Naming it again
        // is judged as any skip is, against the same Contras.
        let held_view = if input.withdraw.is_empty() {
            None
        } else {
            match held_carrier_view(&before) {
                Ok(view) => Some(view),
                Err(error) => {
                    return Ok(without_facts(input.needs_the_view(), &error.into()));
                }
            }
        };
        let carried: Vec<(&String, &BatchId, Carrier)> = input
            .withdraw
            .iter()
            .flat_map(|row| {
                let carriers: BTreeSet<(&BatchId, Carrier)> = before_view
                    .skips
                    .iter()
                    .chain(held_view.iter().flat_map(|view| view.skips.iter()))
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
                        SkipState::NotFound
                        | SkipState::TallyUnchecked
                        | SkipState::NotEstablished => Vec::new(),
                    })
                    .collect();
                carriers
                    .into_iter()
                    .map(move |(batch, carrier)| (row, batch, carrier))
            })
            .collect();
        if !carried.is_empty() {
            let kinds = carried.iter().map(|&(_, _, carrier)| carrier).collect();
            // the rows left out count too
            let restored = carried
                .iter()
                .any(|(_, batch, _)| before.restored.contains(batch.as_str()));
            let next_step = Carrier::refusal_step_restored(&kinds, restored);
            let rows = carried
                .into_iter()
                .map(|(row, batch, carrier)| {
                    let mut listed = json!({"bridge_txn_id": row, "batch_id": batch.as_str(),
                        "state": carrier.label()});
                    if before.restored.contains(batch.as_str()) {
                        listed["restored"] = json!(true);
                    }
                    listed
                })
                .collect();
            let (listed, omitted) = rows_listed_json(rows);
            return Ok(Admission::Refused(TransferRefusal {
                reason: "withdraw_skip_paired",
                detail: json!({"paired_rows": listed, "paired_rows_omitted": omitted}),
                next_step,
            }));
        }
        // A withdrawal excuses a row only while it counts against the skip, by the
        // rule the facts apply to a posted holder: built strictly after the
        // newest record of that skip. A build is newer than every record; a saved
        // batch being sent may not be, when a skip of the row was recorded since.
        let stale_withdrawals: BTreeSet<&str> = match (&input.candidate, &input.subject) {
            (Candidate::Build(line), Some(_)) => {
                let built = match statement_facts::built_instant(&line.batch_id, &line.built_at) {
                    Ok(built) => built,
                    Err(error) => return Ok(without_facts(input.needs_the_view(), &error)),
                };
                input
                    .withdraw
                    .iter()
                    .map(String::as_str)
                    .filter(|row| {
                        before
                            .live_skips
                            .iter()
                            .filter(|key| key.row.as_str() == *row)
                            .filter_map(|key| before.skip_built.get(key))
                            .any(|recorded| !statement_facts::withdrawal_counts(&built, recorded))
                    })
                    .collect()
            }
            _ => BTreeSet::new(),
        };
        let paired_elsewhere: Vec<String> = input
            .voucher_rows
            .iter()
            .filter(|row| {
                live.contains(row.as_str())
                    && (!input.withdraw.contains(*row) || stale_withdrawals.contains(row.as_str()))
            })
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
        Ok(Admission::Admitted(Admitted(Some(facts))))
    }

    /// A saved batch about to be sent, judged as its build was and against the
    /// journal as it is now. `None` when the batch is admitted, or is not about
    /// transfers at all (it posts no statement row, carries no statement and
    /// withdraws nothing), as the build decides it. The rows it withdrew are the
    /// ones its record holds (`withdrawn_skips`), whether or not it carries a
    /// statement. Called with the admission lock held, shared or exclusive.
    pub(super) fn transfers_refusal_for_send_while_admitted(
        &self,
        line: &ImportLedgerLine,
    ) -> Result<Option<TransferRefusal>, String> {
        let statement = match &line.statement {
            Some(recorded) => match serde_json::from_value::<StatementFacts>(recorded.clone()) {
                Ok(facts) => Some(facts),
                Err(_) => {
                    let error = TransferFactsError::Facts {
                        source_id: line.batch_id.clone(),
                        error: FactsError::Unparsable,
                    };
                    return Ok(match without_facts(true, &error) {
                        Admission::Refused(refusal) => Some(refusal.for_send()),
                        Admission::Admitted(_) => None,
                    });
                }
            },
            None => None,
        };
        // judged as it was built: the batch's own records are left out of the
        // journal (`subject`) and it is read again as the newest build
        let input = TransferInput {
            company_guid: canonical_batch_guid(&line.company_guid),
            candidate: Candidate::Build(Box::new(BuildLine::of(line))),
            skip_rows: statement
                .iter()
                .flat_map(|statement| &statement.skip_rows)
                .map(|skip| skip.bridge_txn_id.clone())
                .collect(),
            voucher_rows: line
                .vouchers
                .iter()
                .filter(|voucher| {
                    bridge_bank_statement::proposals::is_statement_txn_id(&voucher.bridge_txn_id)
                })
                .map(|voucher| voucher.bridge_txn_id.clone())
                .collect(),
            withdraw: line.withdrawn_skips.iter().flatten().cloned().collect(),
            own_lineage: line.amends_batch_id.clone(),
            carries_statement: statement.is_some(),
            subject: Some(line.batch_id.clone()),
        };
        if !input.needs_the_view() {
            return Ok(None);
        }
        Ok(match self.admit_transfers_while_admitted(input)? {
            Admission::Refused(refusal) => Some(refusal.for_send()),
            Admission::Admitted(_) => None,
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
