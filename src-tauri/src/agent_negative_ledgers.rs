//! Thin MCP presentation of Tally's own Negative Ledgers read (#1358).
use super::*;
use bridge_tally_protocol::native_negative_ledgers::NegativeLedgerBalance;

const VERIFICATION: &str = "stable_paired_report_with_company_mode_and_extent_guards";

/// Whether the window is one whole financial year, 1 April to 31 March: the only
/// window the report was captured for.
fn is_a_whole_financial_year(from: &str, to: &str) -> bool {
    let (Some(from_year), Some(to_year)) = (from.get(..4), to.get(..4)) else {
        return false;
    };
    let (Ok(from_year), Ok(to_year)) = (from_year.parse::<u32>(), to_year.parse::<u32>()) else {
        return false;
    };
    from.get(4..) == Some("0401") && to.get(4..) == Some("0331") && to_year == from_year + 1
}

impl Server {
    pub(super) async fn negative_ledgers(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        if from > to {
            return Err("invalid_date_range".to_string().into());
        }
        let (company, identity, prior) = self.verified_company(guid).await?;
        let read = self
            .runtime
            .fetch_negative_ledgers(self.tally_config(), &identity, from.clone(), to.clone())
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("negative_ledgers_read_failed", error)
                    .with_prior_evidence(prior.clone())
            })?;
        let evidence = combine_evidence(prior, evidence_from_runtime_read(read.evidence));
        let ledgers: Vec<Value> = read
            .list
            .entries
            .iter()
            .map(|entry| {
                let (side, amount) = match &entry.balance {
                    NegativeLedgerBalance::Debit(amount) => ("debit", amount.as_str()),
                    NegativeLedgerBalance::Credit(amount) => ("credit", amount.as_str()),
                };
                json!({
                    "ledger": party_name(entry.name.clone()),
                    "side": side,
                    "amount": amount,
                })
            })
            .collect();
        let empty = ledgers.is_empty();
        let mut unmeasured = Vec::new();
        if !is_a_whole_financial_year(from.as_str(), to.as_str()) {
            unmeasured.push("window_other_than_a_whole_financial_year");
        }
        let mut result = json!({
            "state": if empty { "not_established" } else { "observed" },
            "basis": "tally_native_negative_ledgers",
            "from": read.from, "to": read.to,
            "count": ledgers.len(),
            "ledgers": ledgers,
            "unmeasured_in_this_answer": unmeasured,
            "checks": {
                "list_complete": "not_checked",
                "amounts_against_trial_balance": "not_checked",
            },
            "verification": VERIFICATION,
            "limitations": [
                "This is Tally's own Negative Ledgers list, returned as it is. The rule by which Tally lists a ledger is not established and is not derived here (on three books each listed ledger's balance sat on the side opposite its group), and on the first book measured a ledger under Suspense A/c with a credit balance was not listed while a creditor with a debit balance and a debtor with a credit balance were listed",
                "The list is not compared with the trial balance or with any other report, so a ledger Tally did not list is not shown to be absent, and a listed amount is not shown to equal the ledger's closing balance in another report (reported, not kept: on the first book both listed amounts matched Tally's screen and its trial balance; on two more books the one ledger listed on each matched the trial balance of the same year)",
                "`side` is the side of the closing balance Tally carried it on (a debit as a negative debit amount, a credit as a positive credit amount, the other column empty), whatever side the ledger's group usually carries. Tally's sign convention was seen on one book for both sides and on two more for the credit side; an answer that broke it, or listed both sides or no amount for a ledger, is refused",
                "An empty list is returned as empty and does not say that no ledger is negative: Tally's empty answer cannot be told from a report that was not rendered",
                "The report was captured for one whole financial year (1 April to 31 March) on one synthetic company, and read for a whole year on two more synthetic books; any other window is named in unmeasured_in_this_answer. What GST, stock, several currencies, or optional or post-dated vouchers do to the list was not measured: one of the two further books holds GST ledgers and one optional voucher, and neither book's settings were read",
                "The window must not start before the book, and Education mode is refused",
                "The report has no size check, and its cost on a large book is not known: the report is one paired read (two requests), never retried; ask for one financial year first, and if a call times out do not repeat it",
                "Tally's own report carries no company identity; it is bound only by the company, mode and book-extent checks around the read",
                "Not voucher-level reconciliation or an atomic snapshot",
            ],
        });
        if empty {
            if let Some(object) = result.as_object_mut() {
                object.insert(
                    "reason".to_string(),
                    json!("negative_ledgers_empty_answer_is_not_a_finding"),
                );
            }
        }
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_negative_ledgers_tests.rs"]
mod tests;
