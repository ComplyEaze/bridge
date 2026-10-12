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
    transfer_pairing_view, DoubleBookedClass, SkipState, TransferView,
};
use statement_facts::{Candidate, TransferFacts, TransferFactsError};

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

/// A skipped row, or a Contra booked twice, that is not paired.
pub(super) struct OpenRow {
    id: String,
    date: TallyDate,
    state: &'static str,
    batch: Option<String>,
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
            let (label, batch) = match state {
                SkipState::Paired { .. } => {
                    paired += 1;
                    continue;
                }
                SkipState::NotFound => ("not_found", None),
                SkipState::Ambiguous { .. } => ("ambiguous", None),
                SkipState::OtherSideUnverified { batch } => {
                    ("other_side_unverified", Some(batch.as_str().to_string()))
                }
                SkipState::NotEstablished => ("not_established", None),
                SkipState::ReversedSameDay { .. } => ("reversed_same_day", None),
            };
            skips_open += 1;
            if let Some(date) = dates.get(key) {
                open.push(OpenRow {
                    id: key.row.as_str().to_string(),
                    date: (*date).clone(),
                    state: label,
                    batch,
                });
            }
        }
        for (key, holder, date) in &facts.held_elsewhere {
            open.push(OpenRow {
                id: key.row.as_str().to_string(),
                date: date.clone(),
                state: "held_elsewhere",
                batch: Some(holder.as_str().to_string()),
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
                        state: "double_booked",
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
                .entry(row.state)
                .or_insert_with(|| json!(next_step(row.state)));
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
                    "state": row.state, "batch_id": batch}),
                None => json!({"bridge_txn_id": row.id, "date": row.date, "state": row.state}),
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

const NOT_ESTABLISHED_STEP: &str = "Whether the other account's Contra exists is not established. Verify any batch of the other account's statement for this date first; build that statement only if no batch of it exists.";
/// The whole check could not run, which says nothing of any one row. A batch
/// that was written, or a verification, is checked again by verifying it.
const CHECK_NOT_RUN_STEP: &str =
    "The transfer check could not run this time; run verify_import on this batch again.";
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

/// What a section says when nothing of it fits: that the check is to be run
/// again. It claims no count.
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

/// What to do about an open row of each state, for the assistant to follow.
pub(super) fn next_step(state: &str) -> &'static str {
    match state {
        "other_side_unverified" => "The other account's Contra for this row is in the batch named, which is not verified yet. Run verify_import on that batch. Do not build or post either statement again for this row.",
        "not_found" => "The other account's statement for this date went through ComplyEaze Bridge and holds no Contra for this row. If no batch of that statement covers this date, build it. If that statement skipped this row too, withdraw the skip on exactly one side. Do not enter this transfer in Tally by hand: a later post of the other statement would book it twice.",
        "ambiguous" => "More than one Contra could carry this row. ComplyEaze Bridge does not choose; check in Tally which Contra carries this transfer.",
        "reversed_same_day" => "This row, or the other account's matching row, has an equal amount going the other way the same day on its own statement, so a reversal and a pair of real transfers look alike. Check in Tally which rows are transfers. Name the row in withdraw_skips only when the person confirms the other account's statement does not carry it.",
        "held_elsewhere" => "The batch named posts this row, which a statement skipped. Verify that batch; once it is found posted the skip is retired.",
        "double_booked" => "Two statements each booked this transfer as a Contra. Check both in Tally and keep one.",
        _ => NOT_ESTABLISHED_STEP,
    }
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
    next_step: &'static str,
}

impl TransferRefusal {
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
            next_step: TRANSFER_VIEW_NOT_ESTABLISHED_NEXT_STEP,
        })
    } else {
        Admission::Admitted(TransferSummary::NotEstablished)
    }
}

const SKIP_ROW_ALREADY_POSTED_NEXT_STEP: &str = "No file was written and nothing was recorded. This build skips a row that is already a voucher of a batch of this company: blocking_batch_id names it, and it was sent to Tally, was found posted, or is an earlier build of the batch being amended. A skip cannot take a voucher out of Tally, and it would say that another account's Contra carries a transfer this company already booked. Do not skip the rows listed in skipped_rows. If the batch named is not verified yet, run verify_import on it. Then parse the statement again with those rows not skipped (a row that batch holds is then refused as import_txn_already_posted) and build again.";
const SKIP_ROW_PAIRED_ELSEWHERE_NEXT_STEP: &str = "No file was written and nothing was recorded. This build posts a row that a statement of this company skipped as carried by the other account's Contra: skipped_rows lists them. Posting the row books its own bank, and if the other account's Contra exists the transfer is booked twice. Do not post these rows. Parse the statement again with those rows mapped skip, or build again naming a row in withdraw_skips only when the person confirms the other account's statement does not carry it; ask for each, and never name a row on their behalf.";
const WITHDRAW_SKIP_INVALID_NEXT_STEP: &str = "No file was written and nothing was recorded. Each row in invalid_rows is not both a skip that ComplyEaze Bridge holds for this company (in a batch sent to Tally or found posted, or in a statement of only skipped rows) and a voucher of this build. Build again with withdraw_skips naming only rows that are both, or without it.";
const WITHDRAW_SKIP_PAIRED_NEXT_STEP: &str = "No file was written and nothing was recorded. The other account's statement already carries this row as a Contra in the batch named in paired_rows. If that Contra is wrong, correct that statement's batch first; do not post this row.";
const WITHDRAW_SKIP_UNVERIFIED_NEXT_STEP: &str = "No file was written and nothing was recorded. The other account's statement may carry this row as a Contra in the batch named in paired_rows, which is not verified yet. Run verify_import on that batch first; do not post this row.";
const WITHDRAW_SKIP_BOTH_NEXT_STEP: &str = "No file was written and nothing was recorded. For a row with state paired, the other account's statement already carries it as a Contra in the batch named in paired_rows: if that Contra is wrong, correct that statement's batch first. For a row with state other_side_unverified, the other account's statement may carry it as a Contra in the batch named, which is not verified yet: run verify_import on that batch first. Do not post these rows.";
const TRANSFER_DOUBLE_BOOKED_NEXT_STEP: &str = "No file was written and nothing was recorded. A Contra in this build carries a transfer that the batch named in other_batch_id already booked as a Contra from the other account's statement (the references are equal). Posting both books it twice. Do not build this Contra: run verify_import on the other batch if it is not verified, then parse this statement again with that row skipped (treatment skip, naming the other account's ledger) and build again.";
const TRANSFER_VIEW_NOT_ESTABLISHED_NEXT_STEP: &str = "No file was written and nothing was recorded. ComplyEaze Bridge could not read its own record of the skipped rows and Contras of this company, so it cannot tell whether this build books a transfer twice. error, check and record_id name what failed, without any value from the record. ComplyEaze Bridge runs the same checks before it writes those records, so this can come only from a defect in it: report it with the error named here, and do not build this statement again until it is fixed.";

/// What to do about withdrawn rows the other account's statement carries, by
/// whether its Contra is found posted or waits in a batch not yet verified.
fn withdraw_paired_step(rows: &[Value]) -> &'static str {
    let unverified = rows
        .iter()
        .filter(|row| row["state"] == "other_side_unverified")
        .count();
    if unverified == 0 {
        WITHDRAW_SKIP_PAIRED_NEXT_STEP
    } else if unverified == rows.len() {
        WITHDRAW_SKIP_UNVERIFIED_NEXT_STEP
    } else {
        WITHDRAW_SKIP_BOTH_NEXT_STEP
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
    fn transfer_view_while_admitted(
        &self,
        company_guid: &str,
        candidate: Option<Candidate>,
    ) -> Result<(TransferFacts, TransferView), TransferFactsError> {
        let journal: Box<dyn std::io::BufRead> = match self.import_journal_while_admitted() {
            Ok(Some(reader)) => Box::new(reader),
            Ok(None) => Box::new(std::io::empty()),
            Err(code) => return Err(TransferFactsError::JournalUnreadable { code }),
        };
        let skips = self
            .skip_file_while_admitted()
            .map_err(|code| TransferFactsError::SkipFileUnreadable { code })?;
        let facts = ledger::transfer_facts(journal, skips, company_guid, candidate)?;
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
                        next_step: SKIP_ROW_ALREADY_POSTED_NEXT_STEP,
                    }));
                }
            }
        }
        // C5a: the skips there are before this build.
        let (before, before_view) =
            match self.transfer_view_while_admitted(&input.company_guid, None) {
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
                next_step: WITHDRAW_SKIP_INVALID_NEXT_STEP,
            }));
        }
        // A skip the other statement already carries, or may carry once it is
        // verified, is not one to withdraw. Ambiguous, not found, not
        // established and reversed the same day are.
        let carried: Vec<Value> = input
            .withdraw
            .iter()
            .filter_map(|row| {
                before_view.skips.iter().find_map(|(key, state)| {
                    if key.row.as_str() != row {
                        return None;
                    }
                    let (batch, label) = match state {
                        SkipState::Paired { contra, .. } => (&contra.batch, "paired"),
                        SkipState::OtherSideUnverified { batch } => {
                            (batch, "other_side_unverified")
                        }
                        _ => return None,
                    };
                    Some(json!({"bridge_txn_id": row, "batch_id": batch.as_str(),
                        "state": label}))
                })
            })
            .collect();
        if !carried.is_empty() {
            let next_step = withdraw_paired_step(&carried);
            let (listed, omitted) = rows_listed_json(carried);
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
                next_step: SKIP_ROW_PAIRED_ELSEWHERE_NEXT_STEP,
            }));
        }
        // C6: the view with this build in it.
        let candidate_batch = input.candidate_build().map(str::to_string);
        let needs_the_view = input.needs_the_view();
        let (facts, view) =
            match self.transfer_view_while_admitted(&input.company_guid, Some(input.candidate)) {
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
                next_step: TRANSFER_DOUBLE_BOOKED_NEXT_STEP,
            }));
        }
        Ok(Admission::Admitted(TransferSummary::of(&facts, &view)))
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
                    self.transfer_view_while_admitted(&canonical_batch_guid(company_guid), None)
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
