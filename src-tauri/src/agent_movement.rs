//! Movement for the local MCP adapter.
use super::voucher_window::{WindowScope, LEDGER_SCOPE_NEEDS_DAY_SPLIT};
use super::*;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::ledger_scope::{LedgerScopeName, LedgerScopeNameError};

/// A ledger-scoped read found no vouchers. Not taken as proof of none (#485): the whole book's
/// window decides, through the same empty-window corroboration as any other read.
const LEDGER_SCOPE_EMPTY_NOT_ESTABLISHED: &str = "ledger_scope_empty_not_established";
/// A ledger-scoped row without a line on the ledger: the scoped read did not do what it was
/// admitted for, so nothing it returned is summed.
const LEDGER_SCOPE_ROW_WITHOUT_LINE: &str = "ledger_scope_row_without_line";

/// Why a movement read went to the book's window though a ledger was named: a closed set of
/// reasons that hold no data from Tally or the caller, each with the sentence the result carries
/// beside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WholeBookReason {
    NameOutsideScopedAlphabet,
    NameHoldsFormulaWord,
    PlanNeedsDaySplit,
    ScopedReadFoundNothing,
}

impl WholeBookReason {
    const fn code(self) -> &'static str {
        match self {
            Self::NameOutsideScopedAlphabet => "ledger_name_outside_scoped_alphabet",
            Self::NameHoldsFormulaWord => "ledger_name_holds_formula_word",
            Self::PlanNeedsDaySplit => LEDGER_SCOPE_NEEDS_DAY_SPLIT,
            Self::ScopedReadFoundNothing => LEDGER_SCOPE_EMPTY_NOT_ESTABLISHED,
        }
    }

    const fn note(self) -> &'static str {
        match self {
            Self::NameOutsideScopedAlphabet => {
                "The named ledger's name is not one the ledger-scoped read sends (a character, spacing or length it does not admit), so the whole book's vouchers for the period were read and this ledger's figures taken from them. The read is larger and slower."
            }
            Self::NameHoldsFormulaWord => {
                "The named ledger's name holds a word that Tally's formulas read as an operator, and the ledger-scoped read has not been observed on such a name, so the whole book's vouchers for the period were read and this ledger's figures taken from them. The read is larger and slower."
            }
            Self::PlanNeedsDaySplit => {
                "A day of the named ledger's vouchers was too heavy for one ledger-scoped read, so the whole book's vouchers for the period were read and this ledger's figures taken from them. The read is larger and slower."
            }
            Self::ScopedReadFoundNothing => {
                "The ledger-scoped read found no vouchers for the named ledger in the period, which is not taken as proof that there are none: the whole book's vouchers for the period were read and checked instead."
            }
        }
    }

    /// The fallback a failed ledger-scoped read asks for, from its code alone.
    fn of_failure(failure: &ToolFailure) -> Option<Self> {
        match failure.code.as_str() {
            LEDGER_SCOPE_NEEDS_DAY_SPLIT => Some(Self::PlanNeedsDaySplit),
            LEDGER_SCOPE_EMPTY_NOT_ESTABLISHED => Some(Self::ScopedReadFoundNothing),
            _ => None,
        }
    }
}

/// The scope a catalogue ledger name reads by, or the reason it reads the book's window instead.
fn scope_name_of(name: &str) -> Result<LedgerScopeName, WholeBookReason> {
    LedgerScopeName::new(name).map_err(|refusal| match refusal {
        LedgerScopeNameError::FormulaWord => WholeBookReason::NameHoldsFormulaWord,
        LedgerScopeNameError::Empty
        | LedgerScopeNameError::TooLong
        | LedgerScopeNameError::Spacing
        | LedgerScopeNameError::Character => WholeBookReason::NameOutsideScopedAlphabet,
    })
}

/// What the result says about a read by the ledger's own vouchers. It states what was compared,
/// not that the two reads agree: the lab has compared them for one party ledger over a month
/// and for one sales ledger on a day, and not for post-dated, optional, cancelled or zero-amount
/// vouchers, nor for a ledger that holds most of the book.
const LEDGER_SCOPED_NOTE: &str = "Only the vouchers Tally lists under the named ledger were read, not the whole book's. Compared with a whole-book read for one party ledger over a month and one sales ledger on a day only; a voucher Tally leaves out of its list for the ledger would not be seen.";

impl Server {
    pub(super) async fn ledger_movement(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        if from > to {
            return Err("invalid_date_range".to_string().into());
        }
        let (company, identity, mut evidence) = self.verified_company(guid).await?;
        let result: Result<ToolOutcome, ToolFailure> = async {
            let books_from = normalized_date(
                company
                    .books_from
                    .as_deref()
                    .ok_or_else(|| "company_identity_incomplete".to_string())?,
            )?;
            ensure_movement_window_within_books(&from, &books_from)?;
            let opening_date = from.clone();
            let (ledgers, ledger_evidence) = self
                .read_movement_ledgers(&identity, opening_date.clone())
                .await?;
            evidence = combine_evidence(evidence.clone(), ledger_evidence);
            // The named ledger is resolved against the first catalogue, before
            // any voucher is read: a misspelt name refuses here, not after the
            // whole window has been read twice. The corroborating catalogue
            // below still has to agree before anything is returned.
            let selected = optional_string(args, "ledger")?
                .map(|name| {
                    resolve_ledger_or_refuse(
                        ledgers.iter().map(|ledger| ledger.name.as_str()),
                        &name,
                        self.settings.redaction,
                    )
                })
                .transpose()?;
            // The one place a ledger enters a request (#485): the catalogue's own spelling of
            // the ledger just resolved, never the argument. A name the scoped read does not
            // admit reads the book's window instead, and the result says so.
            let mut whole_book = None;
            let scope_name = match selected.as_ref().map(|found| scope_name_of(found.name())) {
                Some(Ok(name)) => Some(name),
                Some(Err(reason)) => {
                    whole_book = Some(reason);
                    None
                }
                None => None,
            };
            let scoped_first = scope_name.as_ref().map_or(WindowScope::Book, WindowScope::Ledger);
            let opening = self
                .read_movement_vouchers(
                    &identity,
                    &company.name,
                    scoped_first,
                    (from.clone(), to.clone()),
                    WindowPlanSource::Estimate { known_marks: None },
                    None,
                )
                .await;
            let (opening_read, scope) = match opening {
                Ok(read) => (read, scoped_first),
                Err(failure) if matches!(scoped_first, WindowScope::Ledger(_)) => {
                    let Some(reason) = WholeBookReason::of_failure(&failure) else {
                        return Err(failure);
                    };
                    whole_book = Some(reason);
                    if let Some(attempt) = failure.evidence {
                        evidence = combine_evidence(evidence.clone(), *attempt);
                    }
                    let read = self
                        .read_movement_vouchers(
                            &identity,
                            &company.name,
                            WindowScope::Book,
                            (from.clone(), to.clone()),
                            WindowPlanSource::Estimate { known_marks: None },
                            None,
                        )
                        .await?;
                    (read, WindowScope::Book)
                }
                Err(failure) => return Err(failure),
            };
            let MovementPage {
                rows: vouchers,
                observed_rows: voucher_rows_observed,
            } = opening_read.page;
            if let Some(preflight) = opening_read.preflight {
                evidence = combine_evidence(evidence.clone(), preflight);
            }
            let read_evidence = opening_read.evidence;
            let voucher_snapshot = read_evidence.response_sha256.clone();
            evidence = combine_evidence(evidence.clone(), read_evidence);
            if let Some(closing) = opening_read.closing {
                evidence = combine_evidence(evidence.clone(), closing);
            }
            let (corroborating_ledgers, corroboration_evidence) =
                self.read_movement_ledgers(&identity, opening_date).await?;
            evidence = combine_evidence(evidence.clone(), corroboration_evidence);
            // An in-window posting need not change the period opening. Repeat
            // the voucher source across the final ledger read as well; an
            // AlterID high-water alone cannot establish deletion stability.
            // Replayed, not planned again: the two snapshots are compared by
            // hash, so they must be read in the same parts. The replay carries
            // the first read's witness and closes against its marks, so a
            // voucher created above the first read's AlterID ceilings refuses
            // it, where the two old-ceiling snapshots alone would still match.
            let marks = opening_read.witness.as_ref().map(|witness| witness.marks);
            let opening_timings = opening_read.timings;
            let closing_read = self
                .read_movement_vouchers(
                    &identity,
                    &company.name,
                    scope,
                    (from, to),
                    WindowPlanSource::replay_of(opening_read.reads, opening_read.witness),
                    marks,
                )
                .await
                .map_err(|failure| match WholeBookReason::of_failure(&failure) {
                    // The same parts, read again, now answer differently: the book moved.
                    Some(_) => {
                        let mut drifted = ToolFailure::from("voucher_snapshot_drifted".to_string());
                        drifted.evidence = failure.evidence;
                        drifted
                    }
                    None => failure,
                })?;
            let closing_voucher_evidence = closing_read.evidence;
            let closing_snapshot = closing_voucher_evidence.response_sha256.clone();
            evidence = combine_evidence(evidence.clone(), closing_voucher_evidence);
            if let Some(closing) = closing_read.closing {
                evidence = combine_evidence(evidence.clone(), closing);
            }
            if voucher_snapshot != closing_snapshot {
                return Err("voucher_snapshot_drifted".to_string().into());
            }
            validate_movement_snapshot(&ledgers, &corroborating_ledgers, &vouchers)?;
            let mut movement =
                BTreeMap::<String, (Option<String>, Option<String>, String, String, usize)>::new();
            for ledger in ledgers {
                if selected
                    .as_ref()
                    .is_none_or(|found| found.name() == ledger.name)
                {
                    movement.insert(
                        ledger.name,
                        (
                            ledger.parent.returned_text().map(str::to_string),
                            ledger.opening_balance,
                            "0".into(),
                            "0".into(),
                            0,
                        ),
                    );
                }
            }
            for voucher in &vouchers {
                let mut touched = std::collections::BTreeSet::new();
                for entry in &voucher.ledger_entries {
                    let Some(record) = movement.get_mut(&entry.ledger_name) else {
                        // The full catalogue was corroborated before selection;
                        // this entry belongs to a known, unselected ledger.
                        continue;
                    };
                    let amount = bridge_tally_core::ExactDecimal::parse(entry.amount.clone())
                        .map_err(|_| "voucher_amount_invalid".to_string())?;
                    let (is_debit, magnitude) =
                        movement_entry_effect(&amount, entry.is_deemed_positive);
                    if is_debit {
                        record.2 = add_decimal(&record.2, &format!("-{magnitude}"))?;
                    } else {
                        record.3 = add_decimal(&record.3, &magnitude)?;
                    }
                    touched.insert(entry.ledger_name.clone());
                }
                for ledger in touched {
                    if let Some(record) = movement.get_mut(&ledger) {
                        record.4 += 1;
                    }
                }
            }
            let (rows, opening_unobserved) = movement
                .into_iter()
                .map(
                    |(name, (parent, opening, debit, credit, vouchers_touching))| {
                        ledger_movement_row(
                            LedgerMovementRow {
                                name,
                                parent,
                                opening,
                                debit,
                                credit,
                                vouchers_touching,
                            },
                            self.settings.redaction,
                        )
                    },
                )
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .fold(
                    (Vec::new(), false),
                    |(mut rows, any_partial), (row, partial)| {
                        rows.push(row);
                        (rows, any_partial || partial)
                    },
                );
            if opening_unobserved {
                evidence.state = "partial";
                evidence.reason_code = Some("opening_balance_not_observed".to_string());
            }
            let offset = arg_usize(args, "offset", 0)?;
            let limit =
                arg_positive_usize(args, "limit", self.settings.max_rows)?.min(self.settings.max_rows);
            let total = rows.len();
            let rows = rows
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect::<Vec<_>>();
            let truncated = offset.saturating_add(rows.len()) < total;
            let next_offset = truncated.then_some(offset + rows.len());
            let window_scope = match (&selected, whole_book) {
                (None, _) => Value::Null,
                (Some(_), None) => json!({"basis": "ledger_scoped", "vouchers_read": voucher_rows_observed, "note": LEDGER_SCOPED_NOTE}),
                (Some(_), Some(reason)) => {
                    json!({"basis": "whole_book", "reason": reason.code(), "note": reason.note()})
                }
            };
            let mut payload = json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {"state": if opening_unobserved {"partial"} else {"complete"}, "partial_reason": opening_unobserved.then_some("opening_balance_not_observed"), "ledger_match": selected.as_ref().map(|found| found.to_json(self.settings.redaction)), "ledgers": rows, "offset": offset, "next_offset": next_offset, "voucher_rows_observed": voucher_rows_observed, "window_scope": window_scope, "balance_basis": "tally_period_opening_plus_direct_voucher_movement", "evidence_method": "runtime_ledger_opening_at_from_plus_literal_window_entries"}});
            // What the two window reads cost, when it is worth saying, goes in the
            // result beside the ledgers (#1239); the replay is the second read.
            let mut cost = json!({});
            super::read_cost::add_read_cost_of_replayed(
                &mut cost,
                super::read_cost::smallest_page_len_of(&payload, "ledgers"),
                (&opening_timings, &closing_read.timings),
                super::read_cost::Ended::Read,
                self.settings.max_bytes,
            );
            if let (Some(result), Some(cost)) = (payload["result"].as_object_mut(), cost.as_object()) {
                result.extend(cost.iter().map(|(key, value)| (key.clone(), value.clone())));
            }
            Ok(ToolOutcome {
                payload,
                evidence: evidence.clone(),
                company_guid: Some(guid.to_string()),
                truncated,
            })
        }
        .await;
        result.map_err(|failure| failure.with_prior_evidence(evidence))
    }

    pub(super) async fn read_movement_ledgers(
        &self,
        identity: &VerifiedCompanyIdentity,
        from: bridge_tally_core::TallyDate,
    ) -> Result<(Vec<TallyLedger>, Evidence), ToolFailure> {
        let (ledgers, evidence) = self
            .runtime
            .fetch_ledger_opening_at_with_evidence(self.tally_config(), identity, from)
            .await
            .map_err(|error| {
                // A catalogue that outlived its deadline or the response cap says
                // which, beside the operation's own code (`code` keeps naming
                // what failed). Its own causes, not the transport's window ones:
                // the catalogue lists every ledger in the book and does not shrink
                // with the voucher window, so an agent told the window was too
                // large would narrow the dates and send the same read again.
                let cause = window_too_large_code(&error).and_then(|code| match code {
                    "request_deadline_exceeded" => Some(MOVEMENT_CATALOGUE_DEADLINE_EXCEEDED),
                    "response_size_limit_exceeded" => Some(MOVEMENT_CATALOGUE_TOO_LARGE),
                    _ => None,
                });
                let mut failure = ToolFailure::from_runtime("ledger_movement_read_failed", error);
                if failure.cause.is_none() {
                    failure.cause = cause;
                }
                failure
            })?;
        Ok((ledgers, evidence_from_runtime_read(evidence)))
    }

    /// Read the movement window, bounded by the pre-flight of protocol
    /// reference §11c. The returned evidence covers the voucher source alone —
    /// what the two snapshot reads compare — and the pre-flight comes back
    /// beside it, with the ranges read so the closing read can replay them and
    /// be comparable part for part.
    async fn read_movement_vouchers(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        scope: WindowScope<'_>,
        (from, to): (TallyDate, TallyDate),
        source: WindowPlanSource,
        known_marks: Option<CompanyMarks>,
    ) -> Result<MovementWindowRead, ToolFailure> {
        let company = ValidatedCompanyName::new(company.to_string())
            .map_err(|_| "company_name_invalid".to_string())?;
        let range = ValidatedDateRange::new(from.as_str(), to.as_str())
            .map_err(|_| "invalid_date_range".to_string())?;
        let shape = VoucherReadShape::Movement;
        let read = self
            .read_scoped_voucher_window(
                identity,
                scope,
                company.as_str(),
                &from,
                &to,
                shape,
                source,
                WindowReadLimits::for_shape(shape),
                |xml| parse_agent_changed_rows(xml, identity.company_guid()),
            )
            .await?;
        let preflight = read.preflight_evidence;
        let closing = read.closing_evidence;
        let timings = read.timings;
        let witness = read.witness;
        let marks = witness
            .as_ref()
            .map(|witness| witness.marks)
            .or(known_marks);
        let reads = read.reads;
        let mut evidence = read.evidence;
        let voucher_rows = read.rows;
        let result: Result<MovementPage, ToolFailure> = async {
            let page = parse_scoped_movement_rows(
                voucher_rows,
                scope,
                range.from_yyyymmdd(),
                range.to_yyyymmdd(),
            )?;
            if page.observed_rows == 0 && matches!(scope, WindowScope::Ledger(_)) {
                // The corroboration below asks whether the whole book is empty over the
                // window; a ledger with no vouchers in a busy window is not that. Nothing is
                // concluded from a scoped answer of none.
                return Err(LEDGER_SCOPE_EMPTY_NOT_ESTABLISHED.to_string().into());
            }
            if page.observed_rows == 0 {
                let (corroboration, partial, reason) = self
                    .corroborate_empty_voucher_read(identity, company.as_str(), &from, &to, marks)
                    .await?;
                evidence = combine_evidence(evidence.clone(), corroboration);
                if partial {
                    return Err(reason.unwrap_or("empty_uncorroborated").to_string().into());
                }
            }
            Ok(page)
        }
        .await;
        match result {
            Ok(page) => Ok(MovementWindowRead {
                page,
                evidence,
                preflight,
                closing,
                reads,
                witness,
                timings,
            }),
            Err(failure) => Err(failure.with_prior_evidence(
                [preflight, Some(evidence), closing]
                    .into_iter()
                    .flatten()
                    .reduce(combine_evidence)
                    .expect("the data evidence is present"),
            )),
        }
    }
}

/// One bounded movement window read. See `read_movement_vouchers`.
struct MovementWindowRead {
    page: MovementPage,
    /// The voucher source (and any empty-window corroboration) alone.
    evidence: Evidence,
    preflight: Option<Evidence>,
    /// The closing bracket, after the data parts.
    closing: Option<Evidence>,
    reads: Vec<WindowPart>,
    /// What the corroborating replay must carry.
    witness: Option<WindowWitness>,
    /// What this window read did, for the call's `read_cost` (#1239).
    timings: super::voucher_window::WindowReadTimings,
}

/// Which column a ledger entry moves, taken from `AMOUNT`'s own sign.
///
/// Never from `ISDEEMEDPOSITIVE`. That flag records the column the entry was made
/// in, and on a rounding ledger it legitimately disagrees with the sign. Discarding
/// the sign and re-deriving it from the flag moved a real book's rounding ledger to
/// -3.42 against Tally's own -2.06; a movement that does not tie to the trial
/// balance is worse than one that refuses to answer.
///
/// A zero amount carries no direction to read, so it stays on the side the flag
/// observed rather than having one invented for it.
pub(super) fn movement_entry_is_debit(
    amount: &bridge_tally_core::ExactDecimal,
    is_deemed_positive: bool,
) -> bool {
    if amount.is_zero() {
        is_deemed_positive
    } else {
        amount.is_negative()
    }
}

/// The column a ledger entry adds to and the magnitude it adds there, shared by `ledger_movement`
/// and the voucher summaries so the two cannot disagree (#1251).
///
/// A zero amount, signed or not (`-0.00` parses), has no magnitude to carry a sign on: it adds `0`
/// to whichever side its flag names. Writing the magnitude of `-0.00` behind a minus sign gave
/// `--0.00`, which is not a number, and refused a valid read as `voucher_amount_invalid`.
pub(super) fn movement_entry_effect(
    amount: &bridge_tally_core::ExactDecimal,
    is_deemed_positive: bool,
) -> (bool, String) {
    let magnitude = if amount.is_zero() {
        "0".to_string()
    } else {
        amount.magnitude().as_str().to_string()
    };
    (
        movement_entry_is_debit(amount, is_deemed_positive),
        magnitude,
    )
}

fn validate_movement_snapshot(
    opening: &[TallyLedger],
    corroboration: &[TallyLedger],
    vouchers: &[MovementVoucher],
) -> Result<(), String> {
    let initial = opening
        .iter()
        .map(|ledger| (ledger.name.as_str(), ledger))
        .collect::<BTreeMap<_, _>>();
    let repeated = corroboration
        .iter()
        .map(|ledger| (ledger.name.as_str(), ledger))
        .collect::<BTreeMap<_, _>>();
    if initial.len() != opening.len()
        || repeated.len() != corroboration.len()
        || initial != repeated
        || vouchers
            .iter()
            .flat_map(|voucher| &voucher.ledger_entries)
            .any(|entry| !initial.contains_key(entry.ledger_name.as_str()))
    {
        return Err("ledger_snapshot_drifted".to_string());
    }
    Ok(())
}

/// Native collection rows are parsed through the same boundary as the change
/// feed. Bridge's report-format parser expects a different XML vocabulary.
#[derive(serde::Deserialize)]
pub(super) struct MovementVoucher {
    date: String,
    cancelled: bool,
    optional: bool,
    #[serde(rename = "amounts")]
    pub(super) ledger_entries: Vec<MovementEntry>,
}

#[derive(serde::Deserialize)]
pub(super) struct MovementEntry {
    #[serde(rename = "ledger")]
    ledger_name: String,
    amount: String,
    #[serde(deserialize_with = "parse_movement_polarity")]
    is_deemed_positive: bool,
}

fn parse_movement_polarity<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = <String as serde::Deserialize>::deserialize(deserializer)?;
    required_tally_bool(Some(&value)).map_err(serde::de::Error::custom)
}

#[cfg(test)]
pub(super) fn parse_movement_vouchers(
    xml: &str,
    from: &str,
    to: &str,
    company_guid: &str,
) -> Result<Vec<MovementVoucher>, String> {
    Ok(parse_movement_rows(parse_agent_changed_rows(xml, company_guid)?, from, to)?.rows)
}

struct MovementPage {
    rows: Vec<MovementVoucher>,
    observed_rows: usize,
}

#[cfg(test)]
fn parse_movement_rows(rows: Vec<Value>, from: &str, to: &str) -> Result<MovementPage, String> {
    parse_scoped_movement_rows(rows, WindowScope::Book, from, to)
}

fn parse_scoped_movement_rows(
    rows: Vec<Value>,
    scope: WindowScope<'_>,
    from: &str,
    to: &str,
) -> Result<MovementPage, String> {
    let observed_rows = rows.len();
    let vouchers = rows
        .into_iter()
        .map(|row| {
            serde_json::from_value::<MovementVoucher>(row)
                .map_err(|_| "ledger_movement_read_failed".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    // A ledger-scoped read promised the vouchers with a line on that ledger: a row without one
    // means the filter is not what it was admitted as, and nothing it returned is summed.
    if let WindowScope::Ledger(ledger) = scope {
        if vouchers.iter().any(|voucher| {
            !voucher
                .ledger_entries
                .iter()
                .any(|entry| entry.ledger_name == ledger.as_str())
        }) {
            return Err(LEDGER_SCOPE_ROW_WITHOUT_LINE.to_string());
        }
    }
    // Check the whole window before excluding cancelled/optional records.
    if vouchers
        .iter()
        .any(|voucher| voucher.date.as_str() < from || voucher.date.as_str() > to)
    {
        return Err("window_not_honoured".to_string());
    }
    // A voucher with no accounting entries cannot move a ledger balance, so it
    // is excluded the way a cancelled or optional one is rather than refusing
    // the window. A Stock Journal legitimately carries none, and one anywhere in
    // the range lost the whole range: seven monthly windows of a real book
    // succeeded while five failed, and every full-year window failed.
    //
    // This does not weaken the read. An *empty* ALLLEDGERENTRIES.LIST is the
    // only shape that reaches here as an empty list -- `agent_voucher_parse.rs`
    // refuses a row carrying some of its three fields and not others, which is
    // what a truncated response or a request-shape regression looks like, and
    // that refusal stays loud. So by this point "no entries" can only mean
    // Tally sent none, never that we failed to read them.
    for voucher in vouchers.iter().filter(|voucher| {
        !voucher.cancelled && !voucher.optional && !voucher.ledger_entries.is_empty()
    }) {
        let total = voucher.ledger_entries.iter().try_fold(
            bridge_tally_core::ExactDecimal::zero(),
            |total, entry| {
                bridge_tally_core::ExactDecimal::parse(entry.amount.clone())
                    .and_then(|amount| total.checked_add(&amount))
                    .map_err(|_| "voucher_amount_invalid".to_string())
            },
        )?;
        if !total.is_zero() {
            return Err("voucher_entries_unbalanced".to_string());
        }
    }
    Ok(MovementPage {
        rows: vouchers
            .into_iter()
            .filter(|voucher| {
                !voucher.cancelled && !voucher.optional && !voucher.ledger_entries.is_empty()
            })
            .collect(),
        observed_rows,
    })
}

#[cfg(test)]
#[path = "agent_movement_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "agent_movement_snapshot_tests.rs"]
mod snapshot_tests;
