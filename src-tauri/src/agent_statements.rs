//! Thin MCP presentation of the shared statement read (#692).
use super::*;
use crate::reports::statements::{Established, TieOut};
use bridge_tally_protocol::native_statement_reports::NativeStatementKind;

/// Unclassified ledgers returned in full up to this many; the count is always
/// complete.
const MAX_UNCLASSIFIED_RETURNED: usize = 100;

impl Server {
    pub(super) async fn profit_and_loss(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        self.statement(args, NativeStatementKind::ProfitAndLoss, "profit_and_loss")
            .await
    }

    pub(super) async fn balance_sheet(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        self.statement(args, NativeStatementKind::BalanceSheet, "balance_sheet")
            .await
    }

    async fn statement(
        &self,
        args: &Value,
        kind: NativeStatementKind,
        tool: &str,
    ) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        let period = crate::tally::runtime::TrialBalancePeriod::new(from, to)
            .map_err(|_| "invalid_date_range".to_string())?;
        let (company, identity, prior) = self.verified_company(guid).await?;
        let read = self
            .runtime
            .fetch_statements(self.tally_config(), &identity, period, kind)
            .await
            .map_err(|error| {
                ToolFailure::from_runtime(&format!("{tool}_read_failed"), error)
                    .with_prior_evidence(prior.clone())
            })?;
        let trial_balance = read.trial_balance;
        let evidence = combine_evidence(prior, evidence_from_runtime_read(trial_balance.evidence));
        let derived = read.derived;
        let (lines, figures) = match kind {
            NativeStatementKind::ProfitAndLoss => (
                &derived.profit_and_loss,
                json!({
                    "gross_result": established_json(&derived.gross_result),
                    "net_result": established_json(&derived.net_result),
                }),
            ),
            NativeStatementKind::BalanceSheet => (
                &derived.balance_sheet,
                json!({
                    "profit_and_loss": {
                        "ledger": derived.profit_and_loss_ledger.as_ref().map(|ledger| json!({
                            "name": party_name(ledger.name.clone()),
                            "closing": ledger.closing,
                        })),
                        "carried": established_json(&derived.balance_sheet_profit_and_loss),
                    },
                }),
            ),
        };
        // The derived lines are the statement only once its result is
        // established; until then they are withheld, and the gates show how
        // each of Tally's own lines compared.
        // The headline's facts, from the same derived results the state is
        // built from, before anything of `derived` is moved.
        let (statement_kind, parts) = match kind {
            NativeStatementKind::ProfitAndLoss => (
                headline::StatementKind::ProfitAndLoss,
                vec![
                    (
                        headline::StatementPart::GrossResult,
                        headline::PartOutcome::of(&derived.gross_result),
                    ),
                    (
                        headline::StatementPart::NetResult,
                        headline::PartOutcome::of(&derived.net_result),
                    ),
                ],
            ),
            NativeStatementKind::BalanceSheet => (
                headline::StatementKind::BalanceSheet,
                vec![(
                    headline::StatementPart::BalanceSheetProfitAndLoss,
                    headline::PartOutcome::of(&derived.balance_sheet_profit_and_loss),
                )],
            ),
        };
        let statement_basis = headline::StatementBasis::new(
            statement_kind,
            trial_balance.from.clone(),
            trial_balance.to.clone(),
            parts,
            derived.profit_and_loss_tie.is_some(),
        );
        // One decision: the lines are withheld exactly when the headline says so.
        let lines = (!statement_basis.lines_withheld()).then_some(lines);
        let outcome = match kind {
            NativeStatementKind::ProfitAndLoss => {
                weakest(&[&derived.net_result, &derived.gross_result])
            }
            NativeStatementKind::BalanceSheet => &derived.balance_sheet_profit_and_loss,
        };
        let unclassified_total = derived.unclassified.len();
        let unclassified = derived
            .unclassified
            .into_iter()
            .take(MAX_UNCLASSIFIED_RETURNED)
            .map(|ledger| {
                json!({
                    "ledger": party_name(ledger.name), "guid": ledger.guid,
                    "reason": ledger.reason, "debit": ledger.debit,
                    "credit": ledger.credit, "closing": ledger.closing,
                })
            })
            .collect::<Vec<_>>();
        let mut result = json!({
                    "state": top_level(outcome).0,
                    "basis": "tally_native_trial_balance_classified_by_reserved_primary_group",
                    "from": trial_balance.from, "to": trial_balance.to,
                    "currency": trial_balance.currency, "read_at": trial_balance.read_at,
                    "lines": lines,
                    "result": figures,
                    "unclassified": unclassified,
                    "unclassified_total": unclassified_total,
                    "stock_ledger_count": derived.stock_ledger_count,
                    "balance_sheet_gate": tie_json(&derived.balance_sheet_tie),
                    "tie_out": derived.profit_and_loss_tie.as_ref().map(tie_json),
                    "verification": "stable_paired_sources_with_company_mode_and_extent_guards",
                    "limitations": [
                        "Each line sums the Trial Balance amounts Tally returned under one reserved primary group, and counts the empty amounts it left out",
                        "A ledger under a user-created primary group, or with an incomplete group chain, is listed in unclassified; while any carries an amount, no result is established",
                        "Closing stock is not derived: with a Stock-in-Hand balance no result is established",
                        "lines is null while this tool's result is not established, so a derived line is never shown as the statement; balance_sheet_gate and tie_out then show how each of Tally's own lines compared",
                        "Every result is established only if Tally's own Balance Sheet for the window ties line for line (balance_sheet_gate); gross and net also need Tally's own Profit and Loss to tie (tie_out), with one heading, Cost of Sales, allowed while it equals the derived cost of sales; a book with stock items is expected to refuse, and no inventory book has been measured",
                        "A book with more than one currency master is refused before the Trial Balance is read",
                        "A Tally line the derivation has no counterpart for, such as a heading with an amount or a difference in opening balances, refuses the results rather than being guessed at",
                        "Tally's own statements carry no company identity; they are bound only by the company, mode and book-extent checks around the read",
                        "The Balance Sheet gate has been measured over one full year on one book and one month on another; a window spanning more than one financial year is unmeasured",
                        "Not voucher-level reconciliation or an atomic snapshot",
                    ],
        });
        // The reason a caller reads beside the state is the one the nested result carries.
        if let (Some(reason), Some(result)) = (top_level(outcome).1, result.as_object_mut()) {
            result.insert("reason".to_string(), json!(reason));
        }
        let mut payload = json!({
            "company": company_json(&company, std::slice::from_ref(&company)),
            "result": result,
        });
        payload["headline"] =
            json!(statement_basis.headline(&headline::CompanyName::new(&company.name)));
        Ok(ToolOutcome {
            payload,
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: unclassified_total > MAX_UNCLASSIFIED_RETURNED,
        })
    }
}

/// The top-level state and reason: `observed` only when this tool's result is
/// established, so the field an agent reads first does not say `observed` over a
/// refusal; the reason is the one the nested result carries (#984).
fn top_level(outcome: &Established) -> (&'static str, Option<&'static str>) {
    match outcome {
        Established::Established { .. } => ("observed", None),
        Established::NotEstablished { reason, .. } => ("not_established", Some(reason.as_str())),
    }
}

/// The result the top-level state follows: the first that is not established,
/// so the weaker outcome wins when a tool carries more than one.
fn weakest<'a>(results: &[&'a Established]) -> &'a Established {
    results
        .iter()
        .copied()
        .find(|result| matches!(result, Established::NotEstablished { .. }))
        .unwrap_or(results[0])
}

/// A result, with any line names that failed the gate masked as party names:
/// a Tally line can name a ledger (its Profit & Loss A/c line does).
fn established_json(result: &Established) -> Value {
    match result {
        Established::Established { value } => json!({"state": "established", "value": value}),
        Established::NotEstablished { reason, lines } => json!({
            "state": "not_established", "reason": reason.as_str(),
            "lines": lines.iter().cloned().map(party_name).collect::<Vec<_>>(),
        }),
    }
}

/// A tie-out with every line name masked as a party name, for the same reason.
fn tie_json(tie: &TieOut) -> Value {
    json!({
        "lines": tie.lines.iter().map(|line| {
            let mut entry = json!({
                "name": party_name(line.name.clone()),
                "tally_sub": line.tally_sub, "tally_main": line.tally_main,
            });
            // Flattened as in `TieLine`: `status`, and `reason` or `derived`.
            if let (Some(entry), Ok(Value::Object(status))) =
                (entry.as_object_mut(), serde_json::to_value(&line.status))
            {
                entry.extend(status);
            }
            entry
        }).collect::<Vec<_>>(),
        "derived_only": tie.derived_only.iter().cloned().map(party_name).collect::<Vec<_>>(),
    })
}

#[cfg(test)]
#[path = "agent_statements_tests.rs"]
mod tests;
