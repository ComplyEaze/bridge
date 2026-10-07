//! Thin MCP presentation of the shared cash flow read (#1232).
use super::*;
use crate::reports::cash_flow::CashFlowCheck;

const VERIFICATION: &str = "stable_paired_sources_with_company_mode_and_extent_guards";

/// What is and is not checked, in one closed vocabulary: `checked`, `differs`
/// (compared, and the figures disagree), `not_checked` and `withheld`. Only the
/// net total is compared with a second source; the split by month is Tally's own
/// and is withheld with the months, and the debit and credit columns are read
/// but never returned.
fn checks(check: &CashFlowCheck) -> Value {
    let (net_total, month_split) = match check {
        CashFlowCheck::Tied { .. } => ("checked", "not_checked"),
        CashFlowCheck::Differs { .. } => ("differs", "withheld"),
        CashFlowCheck::MoneyGroupUnmeasured { .. } | CashFlowCheck::NothingToCompare => {
            ("not_checked", "withheld")
        }
    };
    json!({
        "net_total": net_total,
        "month_split": month_split,
        "debit_and_credit_columns": "withheld",
    })
}

/// What this Cash Flow answer holds that has not been measured against Tally. The
/// measured shape is debit-only months, a negative closing and a window inside one
/// March-to-March year; anything else is named here, so the result says so itself
/// and not only the fixed `limitations`.
fn unmeasured_shape(
    cash_flow: &bridge_tally_protocol::native_cash_flow::NativeCashFlow,
) -> Vec<&'static str> {
    use bridge_tally_protocol::native_statement_reports::NativeStatementAmount::Present;
    let mut found = Vec::new();
    if cash_flow
        .rows
        .iter()
        .any(|row| matches!(&row.credit, Present(value) if !value.is_zero()))
    {
        found.push("credit_amount_present");
    }
    if cash_flow.rows.iter().any(
        |row| matches!(&row.closing, Present(value) if !value.is_zero() && !value.as_str().starts_with('-')),
    ) {
        found.push("positive_closing");
    }
    if cash_flow
        .rows
        .windows(2)
        .any(|pair| pair[0].month.month == 3 && pair[1].month.month == 4)
    {
        found.push("window_runs_from_march_into_april");
    }
    found
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
                tally_amounts,
                ledger_amounts,
                money_ledgers,
                unclassified_with_movement,
            } => (
                headline::CashFlowOutcome::Differs,
                // Two figures from Tally that disagree. Neither was confirmed and
                // neither is the cash movement: they leave under a name that says so.
                json!({
                    "state": "differs",
                    "use": "investigation_only",
                    // A side that carried no amount is null, not a zero.
                    "tally_cash_flow_months_added": tally_amounts.then_some(tally_net),
                    "trial_balance_cash_and_bank_ledgers": ledger_amounts.then_some(ledger_net),
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
            CashFlowCheck::NothingToCompare => (
                headline::CashFlowOutcome::NothingToCompare,
                json!({
                    "state": "not_checked",
                    "reason": "no_cash_or_bank_amount_on_either_side",
                }),
            ),
        };
        let unmeasured = unmeasured_shape(&read.cash_flow);
        let basis = headline::CashFlowBasis::new(
            trial_balance.from.clone(),
            trial_balance.to.clone(),
            outcome,
            !unmeasured.is_empty(),
        );
        // One decision: the months are withheld exactly when the headline says so.
        let months = (!basis.months_withheld()).then(|| {
            read.cash_flow
                .rows
                .iter()
                .map(|row| {
                    json!({
                        "month": format!("{:04}-{:02}", row.month.year, row.month.month),
                        // Tally's closing column for the month. On the measured book it
                        // was the month's own net movement, with the credit column empty.
                        "closing": row.closing,
                    })
                })
                .collect::<Vec<_>>()
        });
        let state = if basis.months_withheld() {
            "not_established"
        } else {
            "observed"
        };
        let basis_name = if !basis.months_withheld() {
            "tally_native_cash_flow_net_checked_against_trial_balance"
        } else {
            "tally_native_cash_flow_withheld"
        };
        let mut result = json!({
            "state": state,
            "basis": basis_name,
            "from": trial_balance.from, "to": trial_balance.to,
            "currency": trial_balance.currency, "read_at": trial_balance.read_at,
            "months": months,
            "net_total": net_total,
            // What this answer holds that was never measured against Tally: empty when it
            // has only the measured shape (debit-only months inside one March-to-March year).
            "unmeasured_in_this_answer": unmeasured,
            "checks": checks(&read.check),
            "verification": VERIFICATION,
            "limitations": [
                "This is Tally's own Cash Flow: the month-wise movement of the cash and bank ledgers, not a cash flow statement under AS 3",
                "A month's closing is Tally's closing column for that month. On every captured month (three synthetic books) it was that month's debit plus its credit, a debit being negative, with a credit column empty, with only a credit and with both",
                "A negative amount is a debit and a positive amount a credit, the trial balance's convention. That a credit here is money leaving the cash and bank ledgers is read from that convention; it was checked only through the net total, which tied with credits present",
                "Only the net total of the whole period is compared with the trial balance, over the ledgers this check counts as cash and bank: those under Cash-in-Hand and Bank Accounts, a group inside them included (a ledger under a group a user made inside one was not measured). The split into months is Tally's and is not checked, and a total can tie while one month is wrong",
                "The comparison does not show that Tally honoured the year of the dates for each month: a wrong-year answer is caught only if its net total differs",
                "A month Tally printed with empty amounts is returned as a month with an empty closing, which is not zero and does not say the month had no entries: whether a month with entries that cancel prints an empty closing has not been measured",
                "Tally's debit and credit columns are not returned: on the two books with credits, each column differed from the debit and credit totals of the cash and bank ledgers in the trial balance by the same amount while the net total tied, so only the net is compared and what the difference is has not been established",
                "A ledger under Bank OD A/c or Bank OCC A/c with movement in the period refuses the result: Tally's Cash Flow was seen counting one such ledger (debit only, one book), which this check does not yet count, so without the refusal the figures would differ by its net; the refusal was tested in code and has not been seen against Tally, and a Bank OD credit or a Bank OCC ledger was not measured. A ledger whose group could not be resolved is left out of the comparison and counted if the figures differ",
                "The period must be whole months, at most twelve, so that each row can be placed in its year, and must not start before the book does: a book that begins mid-month cannot have its first month read",
                "Measured on three synthetic books (Tally's answers) and two (this tool run against Tally: the net total tied on five windows, with a credit present and with a positive closing, and a quiet window was answered as nothing to compare); a month with entries that cancel, optional or post-dated vouchers, a window crossing a financial year, a later financial year, a several-currency book and a large book with cash activity are not measured. The report has no size check, and its cost on a large book is not known (one year of empty months on a large book answered at once): ask for one month first, and if a call times out do not repeat it",
                "Tally's own report carries no company identity; it is bound only by the company, mode and book-extent checks around the read",
                "Not voucher-level reconciliation or an atomic snapshot",
            ],
        });
        if let (Some(code), Some(object)) = (read.check.refusal_code(), result.as_object_mut()) {
            object.insert("reason".to_string(), json!(code));
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
