//! Thin MCP presentation of the shared cash flow read (#1232).
use super::*;
use crate::reports::cash_flow::CashFlowCheck;

const VERIFICATION: &str = "stable_paired_sources_with_company_mode_and_extent_guards";

/// What is and is not checked, in one closed vocabulary: `checked`,
/// `not_checked`, `withheld`. Only the net total is compared with a second
/// source; the split by month is Tally's own, and the debit and credit columns
/// are read but never returned.
fn checks(net_total_checked: bool) -> Value {
    json!({
        "net_total": if net_total_checked { "checked" } else { "not_checked" },
        "month_split": "not_checked",
        "debit_and_credit_columns": "withheld",
    })
}

impl Server {
    pub(super) async fn cash_flow(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        let period = crate::tally::runtime::TrialBalancePeriod::new(from.clone(), to.clone())
            .map_err(|_| "invalid_date_range".to_string())?;
        let (company, identity, prior) = self.verified_company(guid).await?;
        let read = self
            .runtime
            .fetch_cash_flow(self.tally_config(), &identity, period)
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("cash_flow_read_failed", error)
                    .with_prior_evidence(prior.clone())
            })?;
        let trial_balance = read.trial_balance;
        let evidence = combine_evidence(prior, evidence_from_runtime_read(trial_balance.evidence));
        let (outcome, net_total) = match &read.check {
            CashFlowCheck::Tied { net, money_ledgers } => (
                headline::CashFlowOutcome::Tied,
                json!({
                    "state": "checked",
                    "value": net,
                    "checked_against": "trial_balance_cash_and_bank_ledgers",
                    "money_ledgers": money_ledgers,
                }),
            ),
            CashFlowCheck::Differs {
                tally_net,
                ledger_net,
                money_ledgers,
                unclassified_with_movement,
            } => (
                headline::CashFlowOutcome::Differs,
                // Two figures from Tally that disagree. Neither was confirmed and
                // neither is the cash movement: they leave under a name that says so.
                json!({
                    "state": "differs",
                    "use": "investigation_only",
                    "tally_cash_flow_months_added": tally_net,
                    "trial_balance_cash_and_bank_ledgers": ledger_net,
                    "money_ledgers": money_ledgers,
                    "ledgers_with_movement_and_no_group_identity": unclassified_with_movement,
                }),
            ),
            CashFlowCheck::MoneyGroupUnmeasured { ledgers } => (
                headline::CashFlowOutcome::MoneyGroupUnmeasured,
                json!({
                    "state": "not_checked",
                    "reason": "bank_od_or_occ_ledger_has_movement",
                    "ledgers": ledgers,
                }),
            ),
        };
        let basis = headline::CashFlowBasis::new(
            trial_balance.from.clone(),
            trial_balance.to.clone(),
            outcome,
        );
        // One decision: the months are withheld exactly when the headline says so.
        let months = (!basis.months_withheld()).then(|| {
            read.cash_flow
                .rows
                .iter()
                .map(|row| {
                    json!({
                        "month": format!("{:04}-{:02}", row.month.year, row.month.month),
                        "net": row.closing,
                    })
                })
                .collect::<Vec<_>>()
        });
        let state = if basis.months_withheld() {
            "not_established"
        } else {
            "observed"
        };
        let mut result = json!({
            "state": state,
            "basis": "tally_native_cash_flow_checked_against_trial_balance",
            "from": trial_balance.from, "to": trial_balance.to,
            "currency": trial_balance.currency, "read_at": trial_balance.read_at,
            "months": months,
            "net_total": net_total,
            "checks": checks(!basis.months_withheld()),
            "verification": VERIFICATION,
            "limitations": [
                "This is Tally's own Cash Flow: the month-wise movement of the cash and bank ledgers, not a cash flow statement under AS 3",
                "A negative amount is a debit, which is cash and bank growing; the sign of a net outflow is the opposite by the trial balance's convention and has not been measured on a month with an outflow",
                "Only the net total of the whole period is compared with the trial balance (the ledgers under Cash-in-Hand and Bank Accounts, with any group inside them); the split into months is Tally's and is not checked, and a total can tie while one month is wrong",
                "A month Tally printed with empty amounts is returned as a month with an empty net, which is not zero",
                "Tally's debit and credit columns are not returned: how a contra is counted in them has not been measured",
                "A ledger under Bank OD A/c or Bank OCC A/c with movement in the period refuses the result: whether Tally's Cash Flow counts it has not been measured",
                "The period must be whole months, at most twelve, so that each row can be placed in its year",
                "Tally's own report carries no company identity; it is bound only by the company, mode and book-extent checks around the read",
                "Not voucher-level reconciliation or an atomic snapshot",
            ],
        });
        if let (
            CashFlowCheck::Differs { .. } | CashFlowCheck::MoneyGroupUnmeasured { .. },
            Some(code),
        ) = (&read.check, read.check.refusal_code())
        {
            if let Some(object) = result.as_object_mut() {
                object.insert("reason".to_string(), json!(code));
            }
        }
        let mut payload = json!({
            "company": company_json(&company, std::slice::from_ref(&company)),
            "result": result,
        });
        payload["headline"] = json!(basis.headline(&headline::CompanyName::new(&company.name)));
        Ok(ToolOutcome {
            payload,
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_cash_flow_tests.rs"]
mod tests;
