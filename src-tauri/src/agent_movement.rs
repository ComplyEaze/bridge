//! Movement for the local MCP adapter.
use super::failure_code::MovementCurrencyCode;
use super::voucher_window::{WindowScope, LEDGER_SCOPE_NEEDS_DAY_SPLIT};
use super::*;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::ledger_scope::{LedgerScopeName, LedgerScopeNameError};

/// A ledger-scoped read found no vouchers. Not taken as proof of none (#485): the whole book's
/// window decides, through the same empty-window corroboration as any other read.
pub(super) const LEDGER_SCOPE_EMPTY_NOT_ESTABLISHED: &str = "ledger_scope_empty_not_established";
/// A ledger-scoped row without a line on the ledger: the scoped read did not do what it was
/// admitted for, so nothing it returned is summed.
pub(super) const LEDGER_SCOPE_ROW_WITHOUT_LINE: &str = "ledger_scope_row_without_line";

/// Why a movement read went to the book's window though a ledger was named: a closed set of
/// reasons that hold no data from Tally or the caller, each with the sentence the result carries
/// beside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum WholeBookReason {
    NameOutsideScopedAlphabet,
    NameHoldsFormulaWord,
    PlanNeedsDaySplit,
    ScopedReadFoundNothing,
}

impl WholeBookReason {
    pub(super) const fn code(self) -> &'static str {
        match self {
            Self::NameOutsideScopedAlphabet => "ledger_name_outside_scoped_alphabet",
            Self::NameHoldsFormulaWord => "ledger_name_holds_formula_word",
            Self::PlanNeedsDaySplit => LEDGER_SCOPE_NEEDS_DAY_SPLIT,
            Self::ScopedReadFoundNothing => LEDGER_SCOPE_EMPTY_NOT_ESTABLISHED,
        }
    }

    /// Whether a ledger-scoped read was sent and then given up, so that the call's `read_cost`
    /// (which counts the book's window reads) leaves it out.
    const fn abandons_a_scoped_read(self) -> bool {
        matches!(self, Self::PlanNeedsDaySplit | Self::ScopedReadFoundNothing)
    }

    /// What a refusal of the book's window read carries beside its code (its own note: no figure
    /// was taken, and the book's window may not have been read).
    const fn report(self) -> ScopeReport {
        ScopeReport {
            basis: "whole_book",
            reason: Some(self.code()),
            note: self.refusal_note(),
        }
    }

    /// The sentence beside a refusal: that the read refused is the book's window, which was the
    /// read to make because of the reason, and not a read by the ledger. It says nothing was taken
    /// and nothing about what that read found.
    const fn refusal_note(self) -> &'static str {
        match self {
            Self::NameOutsideScopedAlphabet => {
                "The named ledger's name is not one the ledger-scoped read sends (a character, spacing or length it does not admit), so the whole book's vouchers for the period were the read to make, and this refusal is that read's. No figure was taken."
            }
            Self::NameHoldsFormulaWord => {
                "The named ledger's name holds a word that Tally's formulas read as an operator, and the ledger-scoped read has not been observed on such a name, so the whole book's vouchers for the period were the read to make, and this refusal is that read's. No figure was taken."
            }
            Self::PlanNeedsDaySplit => {
                "A day of the named ledger's vouchers was too heavy for one ledger-scoped read, so the whole book's vouchers for the period were the read to make, and this refusal is that read's. No figure was taken."
            }
            Self::ScopedReadFoundNothing => {
                "The ledger-scoped read found no vouchers for the named ledger, which is not taken as proof that there are none, so the whole book's vouchers for the period were the read to make, and this refusal is that read's. No figure was taken."
            }
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
pub(super) fn scope_name_of(name: &str) -> Result<LedgerScopeName, WholeBookReason> {
    LedgerScopeName::new(name).map_err(|refusal| match refusal {
        LedgerScopeNameError::FormulaWord => WholeBookReason::NameHoldsFormulaWord,
        LedgerScopeNameError::Empty
        | LedgerScopeNameError::TooLong
        | LedgerScopeNameError::Spacing
        | LedgerScopeNameError::Character => WholeBookReason::NameOutsideScopedAlphabet,
    })
}

/// `window_scope.read_cost_excludes` says what the result's `read_cost` leaves out: the
/// ledger-scoped attempt that was sent and given up (`abandoned`). It is put only beside a
/// `read_cost`: a quick call carries no block and nothing to qualify.
fn settle_read_cost_exclusion(result: &mut serde_json::Map<String, Value>, abandoned: bool) {
    if !abandoned || !result.contains_key("read_cost") {
        return;
    }
    if let Some(scope) = result
        .get_mut("window_scope")
        .and_then(Value::as_object_mut)
    {
        scope.insert(
            "read_cost_excludes".to_string(),
            json!("first_ledger_scoped_attempt"),
        );
    }
}

/// A refusal of the book's window read, read for a ledger that was named, says so (#485).
fn tag_window_refusal(failure: ToolFailure, whole_book: Option<WholeBookReason>) -> ToolFailure {
    match whole_book {
        Some(reason) => failure.with_window_scope(reason.report()),
        None => failure,
    }
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
            let MovementOpening {
                ledgers,
                extent: opening_extent,
                several_currency,
                evidence: ledger_evidence,
            } = self
                .read_movement_opening(&identity, opening_date.clone())
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
            // Why the book's window is read though a ledger was named, once that is decided: a
            // refusal of that read says so too (#485).
            let mut whole_book = None;
            let scope_name = match selected.as_ref().map(|found| scope_name_of(found.name())) {
                Some(Ok(name)) => Some(name),
                Some(Err(reason)) => {
                    whole_book = Some(reason);
                    None
                }
                None => None,
            };
            // A book with several Currency masters reaches here only at its first day (#683,
            // §8.2n). Rules 1 and 3 set ledgers aside before any voucher is read, and a named
            // ledger that is set aside refuses before the window is read.
            let mut set_aside = match several_currency.as_ref() {
                Some(first_day) => {
                    let period = crate::tally::runtime::TrialBalancePeriod::new(from.clone(), to.clone())
                        .map_err(|_| ToolFailure::from(MovementCurrencyCode::InvalidDateRange))?;
                    let (trial_balance, extent) = self
                        .runtime
                        .fetch_trial_balance_with_extent(
                            self.tally_config(),
                            &identity,
                            period,
                            crate::tally::runtime::TrialBalanceCurrencyScope::BaseCurrencyLedgersOnly,
                        )
                        .await
                        .map_err(|error| ToolFailure::from_runtime("ledger_movement_read_failed", error))?;
                    evidence = combine_evidence(
                        evidence.clone(),
                        evidence_from_runtime_read(trial_balance.evidence.clone()),
                    );
                    if extent != opening_extent {
                        return Err(MovementCurrencyCode::SnapshotDrifted.into());
                    }
                    let set_aside =
                        currency::CurrencySetAside::before_window(
                            first_day,
                            &trial_balance.ledger_scope,
                            &trial_balance.report,
                            &ledgers,
                        )?;
                    set_aside.refuse_named(selected.as_ref().map(|found| found.name()))?;
                    Some(set_aside)
                }
                None => None,
            };
            let composites = if set_aside.is_some() {
                MovementComposites::NameLedger
            } else {
                MovementComposites::Refuse
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
                    composites,
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
                            composites,
                        )
                        .await
                        .map_err(|failure| tag_window_refusal(failure, whole_book))?;
                    (read, WindowScope::Book)
                }
                // The book's window read, after a name the scoped read does not send, or any
                // read when no ledger was named (`whole_book` is then `None`).
                Err(failure) => return Err(tag_window_refusal(failure, whole_book)),
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
            let corroborating = self.read_movement_opening(&identity, opening_date).await?;
            evidence = combine_evidence(evidence.clone(), corroborating.evidence);
            // The two catalogues are compared ledger by ledger below, openings included, so a
            // rule-3 set that moved between them refuses as drifted; so does a book whose
            // Currency admission changed.
            if corroborating.several_currency.is_some() != set_aside.is_some() {
                return Err(MovementCurrencyCode::SnapshotDrifted.into());
            }
            let corroborating_ledgers = corroborating.ledgers;
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
                    composites,
                )
                .await
                .map_err(|failure| match WholeBookReason::of_failure(&failure) {
                    // The same parts, read again, now answer differently: the book moved.
                    Some(_) => {
                        let mut drifted = ToolFailure::from("voucher_snapshot_drifted".to_string());
                        drifted.evidence = failure.evidence;
                        tag_window_refusal(drifted, whole_book)
                    }
                    None => tag_window_refusal(failure, whole_book),
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
            // Rule 2, from the window both reads agreed on; then a named ledger it set aside
            // refuses too.
            if let Some(set_aside) = set_aside.as_mut() {
                set_aside.add_composite_entries(&vouchers);
                set_aside.refuse_named(selected.as_ref().map(|found| found.name()))?;
            }
            let mut movement =
                BTreeMap::<String, (Option<String>, Option<String>, String, String, usize)>::new();
            for ledger in ledgers {
                // A set-aside ledger is left out before summing, so its entries are skipped
                // below as an unselected ledger's are, and no figure of it is returned.
                if set_aside
                    .as_ref()
                    .is_some_and(|set_aside| set_aside.rule_of(&ledger.name).is_some())
                {
                    continue;
                }
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
                    let amount = entry.plain_amount()?;
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
            // On this path the result is always partial, as its rules are measured on one book
            // (§8.2n); an unobserved opening names the reason when both hold.
            let partial_reason = if opening_unobserved {
                Some("opening_balance_not_observed")
            } else if set_aside.is_some() {
                Some("base_currency_ledgers_only")
            } else {
                None
            };
            if let Some(reason) = partial_reason {
                evidence.state = "partial";
                evidence.reason_code = Some(reason.to_string());
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
            let mut payload = json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {"state": if partial_reason.is_some() {"partial"} else {"complete"}, "partial_reason": partial_reason, "ledger_match": selected.as_ref().map(|found| found.to_json(self.settings.redaction)), "ledgers": rows, "offset": offset, "next_offset": next_offset, "voucher_rows_observed": voucher_rows_observed, "window_scope": window_scope, "balance_basis": "tally_period_opening_plus_direct_voucher_movement", "evidence_method": "runtime_ledger_opening_at_from_plus_literal_window_entries"}});
            if let (Some(set_aside), Some(result)) =
                (set_aside.as_ref(), payload["result"].as_object_mut())
            {
                set_aside.frame(result, self.settings.redaction);
            }
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
                // The `read_cost` is the book's window reads: a scoped attempt that was given up
                // is not in it.
                settle_read_cost_exclusion(
                    result,
                    whole_book.is_some_and(WholeBookReason::abandons_a_scoped_read),
                );
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
            .map_err(movement_catalogue_failure)?;
        Ok((ledgers, evidence_from_runtime_read(evidence)))
    }

    /// `ledger_movement`'s own catalogue read: as [`Self::read_movement_ledgers`], except that a
    /// book with several Currency masters is admitted at its first day (#683).
    async fn read_movement_opening(
        &self,
        identity: &VerifiedCompanyIdentity,
        from: bridge_tally_core::TallyDate,
    ) -> Result<MovementOpening, ToolFailure> {
        let read = self
            .runtime
            .fetch_movement_ledger_opening_at(self.tally_config(), identity, from)
            .await
            .map_err(movement_catalogue_failure)?;
        Ok(MovementOpening {
            ledgers: read.ledgers,
            extent: read.extent,
            several_currency: read.several_currency,
            evidence: evidence_from_runtime_read(read.evidence),
        })
    }

    /// Each day's net movement on `ledger` over a window, a debit positive, read
    /// once. The caller ties the sum to Tally's own closing, which a read that left
    /// a voucher out cannot pass (#1761).
    pub(super) async fn read_ledger_day_moves(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        ledger: &str,
        window: (bridge_tally_core::TallyDate, bridge_tally_core::TallyDate),
    ) -> Result<(BTreeMap<String, bridge_tally_core::ExactDecimal>, Evidence), ToolFailure> {
        let plan = || WindowPlanSource::Estimate { known_marks: None };
        let scope_name = scope_name_of(ledger).ok();
        let scope = scope_name
            .as_ref()
            .map_or(WindowScope::Book, WindowScope::Ledger);
        let mut attempt = None;
        let read = match self
            .read_movement_vouchers(
                identity,
                company,
                scope,
                window.clone(),
                plan(),
                None,
                MovementComposites::Refuse,
            )
            .await
        {
            Err(failure)
                if matches!(scope, WindowScope::Ledger(_))
                    && WholeBookReason::of_failure(&failure).is_some() =>
            {
                attempt = failure.evidence.map(|evidence| *evidence);
                self.read_movement_vouchers(
                    identity,
                    company,
                    WindowScope::Book,
                    window,
                    plan(),
                    None,
                    MovementComposites::Refuse,
                )
                .await?
            }
            read => read?,
        };
        let moves = day_moves_of(&read.page.rows, ledger)?;
        let evidence = [attempt, read.preflight, Some(read.evidence), read.closing]
            .into_iter()
            .flatten()
            .reduce(combine_evidence)
            .expect("the data evidence is present");
        Ok((moves, evidence))
    }

    /// Read the movement window, bounded by the pre-flight of protocol
    /// reference §11c. The returned evidence covers the voucher source alone —
    /// what the two snapshot reads compare — and the pre-flight comes back
    /// beside it, with the ranges read so the closing read can replay them and
    /// be comparable part for part.
    #[allow(clippy::too_many_arguments)]
    async fn read_movement_vouchers(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        scope: WindowScope<'_>,
        (from, to): (TallyDate, TallyDate),
        source: WindowPlanSource,
        known_marks: Option<CompanyMarks>,
        composites: MovementComposites,
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
                |xml| match composites {
                    MovementComposites::Refuse => {
                        parse_agent_changed_rows(xml, identity.company_guid())
                    }
                    MovementComposites::NameLedger => {
                        parse_agent_changed_rows_naming_composite_ledgers(
                            xml,
                            identity.company_guid(),
                        )
                    }
                },
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

/// A catalogue that outlived its deadline or the response cap says which, beside the operation's
/// own code (`code` keeps naming what failed). Its own causes, not the transport's window ones: the
/// catalogue lists every ledger in the book and does not shrink with the voucher window, so an
/// agent told the window was too large would narrow the dates and send the same read again.
fn movement_catalogue_failure(error: anyhow::Error) -> ToolFailure {
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
}

/// `ledger_movement`'s catalogue at `from`, with what the runtime admitted it under.
struct MovementOpening {
    ledgers: Vec<TallyLedger>,
    extent: bridge_tally_protocol::outstandings_shared::CompanyBookExtent,
    several_currency: Option<crate::tally::runtime::SeveralCurrencyFirstDay>,
    evidence: Evidence,
}

/// What a movement window read does with a currency composite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MovementComposites {
    /// Fails the window (`voucher_amount_invalid`, `bill_allocation_amount_invalid`): every
    /// caller but one.
    Refuse,
    /// Keeps a composite entry with no figure, to set its ledger aside: `ledger_movement` on a
    /// several-currency book at its first day only (#683).
    NameLedger,
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
#[serde(from = "ParsedMovementEntry")]
pub(super) struct MovementEntry {
    ledger_name: String,
    amount: MovementAmount,
    is_deemed_positive: bool,
}

/// An entry's amount. A currency composite carries no figure, so no code can sum it: it only
/// names its ledger, to be set aside (#683).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum MovementAmount {
    Plain(String),
    Composite,
}

impl MovementEntry {
    /// The entry's figure; a composite has none (`voucher_amount_invalid`).
    fn plain_amount(&self) -> Result<bridge_tally_core::ExactDecimal, String> {
        match &self.amount {
            MovementAmount::Plain(amount) => bridge_tally_core::ExactDecimal::parse(amount.clone())
                .map_err(|_| "voucher_amount_invalid".to_string()),
            MovementAmount::Composite => Err("voucher_amount_invalid".to_string()),
        }
    }
}

/// An entry as the voucher parse emits it. `currency_composite` is present only under the
/// movement's own composite policy, and only on a composite entry.
#[derive(serde::Deserialize)]
struct ParsedMovementEntry {
    #[serde(rename = "ledger")]
    ledger_name: String,
    amount: String,
    #[serde(deserialize_with = "parse_movement_polarity")]
    is_deemed_positive: bool,
    #[serde(default)]
    currency_composite: bool,
}

impl From<ParsedMovementEntry> for MovementEntry {
    fn from(entry: ParsedMovementEntry) -> Self {
        Self {
            ledger_name: entry.ledger_name,
            amount: if entry.currency_composite {
                MovementAmount::Composite
            } else {
                MovementAmount::Plain(entry.amount)
            },
            is_deemed_positive: entry.is_deemed_positive,
        }
    }
}

fn parse_movement_polarity<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = <String as serde::Deserialize>::deserialize(deserializer)?;
    required_tally_bool(Some(&value)).map_err(serde::de::Error::custom)
}

/// Each day's net movement on `ledger` in `vouchers`, a debit positive.
pub(super) fn day_moves_of(
    vouchers: &[MovementVoucher],
    ledger: &str,
) -> Result<BTreeMap<String, bridge_tally_core::ExactDecimal>, String> {
    let invalid = |_| "voucher_amount_invalid".to_string();
    let mut moves = BTreeMap::<String, bridge_tally_core::ExactDecimal>::new();
    for voucher in vouchers {
        for entry in voucher
            .ledger_entries
            .iter()
            .filter(|entry| entry.ledger_name == ledger)
        {
            let amount = entry.plain_amount()?;
            let (is_debit, magnitude) = movement_entry_effect(&amount, entry.is_deemed_positive);
            let magnitude = bridge_tally_core::ExactDecimal::parse(magnitude).map_err(invalid)?;
            let day = moves
                .entry(voucher.date.clone())
                .or_insert_with(bridge_tally_core::ExactDecimal::zero);
            *day = if is_debit {
                day.checked_add(&magnitude)
            } else {
                day.checked_subtract(&magnitude)
            }
            .map_err(invalid)?;
        }
    }
    Ok(moves)
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
    //
    // A voucher holding a currency composite (#683) carries no figure for that entry, so it
    // cannot be checked here; its composite entries' ledgers are set aside, and only its plain
    // entries on kept ledgers are summed.
    for voucher in vouchers.iter().filter(|voucher| {
        !voucher.cancelled
            && !voucher.optional
            && !voucher.ledger_entries.is_empty()
            && voucher
                .ledger_entries
                .iter()
                .all(|entry| entry.amount != MovementAmount::Composite)
    }) {
        let total = voucher.ledger_entries.iter().try_fold(
            bridge_tally_core::ExactDecimal::zero(),
            |total, entry| {
                entry.plain_amount().and_then(|amount| {
                    total
                        .checked_add(&amount)
                        .map_err(|_| "voucher_amount_invalid".to_string())
                })
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

#[path = "agent_movement_currency.rs"]
mod currency;

#[cfg(test)]
#[path = "agent_movement_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "agent_movement_snapshot_tests.rs"]
mod snapshot_tests;
