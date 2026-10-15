//! Thin MCP presentation of Tally's own month-wise Sales and Purchase Registers,
//! read by report name (#1232).
use super::*;
use bridge_tally_protocol::native_month_register::MonthRegisterKind;

const VERIFICATION: &str = "stable_paired_source_with_company_mode_and_extent_guards";

/// The schema's enum refuses any other value before the call reaches here
/// (`argument_invalid:kind`); this match gives the same code.
fn kind(args: &Value) -> Result<MonthRegisterKind, ToolFailure> {
    match required_string(args, "kind")? {
        "sales" => Ok(MonthRegisterKind::Sales),
        "purchase" => Ok(MonthRegisterKind::Purchase),
        _ => Err("argument_invalid:kind".to_string().into()),
    }
}

impl Server {
    pub(super) async fn month_register(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let kind = kind(args)?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        let (company, identity, prior) = self.verified_company(guid).await?;
        let read = self
            .runtime
            .fetch_month_register(
                self.tally_config(),
                &identity,
                kind,
                from.clone(),
                to.clone(),
            )
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("month_register_read_failed", error)
                    .with_prior_evidence(prior.clone())
            })?;
        let evidence = combine_evidence(prior, evidence_from_runtime_read(read.evidence));
        let months = read
            .register
            .rows
            .iter()
            .map(|row| {
                json!({
                    "month": format!("{:04}-{:02}", row.month.year, row.month.month),
                    "debit": row.dr,
                    "credit": row.cr,
                    "closing": row.cl,
                })
            })
            .collect::<Vec<_>>();
        let payload = json!({
            "company": company_json(&company, std::slice::from_ref(&company)),
            "result": {
                "state": "observed",
                "basis": "tally_month_wise_register_as_printed",
                "kind": kind,
                "report_name": kind.report_id(),
                "from": from, "to": to,
                "months": months,
                "checks": {
                    "against_register_screen": "not_checked",
                    "against_vouchers": "not_checked",
                    "against_trial_balance": "not_checked",
                },
                "verification": VERIFICATION,
                "limitations": [
                    "These are Tally's own month-wise register rows exactly as Tally printed them: nothing here ties them to the vouchers, to the trial balance or to the sales_register or purchase_register tools",
                    "Each month carries Tally's three columns as printed: debit, credit and closing. On the one synthetic book read (BRIDGE PILOT LAB, one financial year), the Sales Register's credit was the month's sales invoices, tax included, neither cancelled nor optional, and its closing was printed for August and September only, as a running total, with the other months empty; the Purchase Register's debit was the month's purchases as a negative amount, and its closing was printed for August and, unchanged, for September, a month with no purchase. That reading was made by the lab against that book's vouchers and is not applied here",
                    "An empty amount is not zero, and a month printed with empty amounts does not say the month had no vouchers",
                    "The period must be whole months, at most twelve, so that each row can be placed in its year, and must not start before the book does",
                    "Not measured: equality with Tally's own register screens, credit and debit notes, a book with stock items, a window shorter than a year or crossing years, a several-currency book, a large book, other releases than TallyPrime 7.1 Silver",
                    "Only the Sales and Purchase Registers are read: the Journal Register prints a count of vouchers rather than amounts, and the Receipt and Payment Registers have not been captured",
                    "Tally's own report carries no company identity; it is bound only by the company, mode and book-extent checks around the read",
                ],
            },
        });
        Ok(ToolOutcome {
            payload,
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_month_register_tests.rs"]
mod tests;
