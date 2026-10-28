//! Thin MCP presentation of Tally's own Negative Stock read (#1358).
use super::*;
use crate::tally::runtime::NegativeStockAnswer;
use bridge_tally_protocol::native_stock_summary::NativeQuantityRead;

const VERIFICATION: &str = "stable_paired_report_with_company_mode_and_extent_guards";

/// Whether the window starts on 1 April and ends on or before the following
/// 31 March: the only windows the report was captured for.
fn is_measured_window(from: &str, to: &str) -> bool {
    let Some(year) = from.get(..4).and_then(|year| year.parse::<u32>().ok()) else {
        return false;
    };
    from.get(4..) == Some("0401") && from <= to && to <= format!("{}0331", year + 1).as_str()
}

/// A quantity as Tally printed it. A quantity that is not `<number> <unit>` is
/// named unread, and an empty one empty: neither is zero.
fn quantity_json(quantity: &NativeQuantityRead) -> Value {
    match quantity {
        NativeQuantityRead::Read(read) => {
            json!({"state": "read", "amount": read.amount.as_str(), "unit": read.unit})
        }
        NativeQuantityRead::Empty => json!({"state": "empty"}),
        NativeQuantityRead::Unread => json!({"state": "unread"}),
    }
}

/// The sign a listed value carries (#2051). Measured on one synthetic book on TallyPrime 7.1 and
/// again on 7.2: the item at -3 Nos and a rate of 120.00 is exported with the value 360.00 while
/// Tally's screen shows (-)360.00. Protocol reference section 12a.13 records the wire rule.
const NEGATIVE_STOCK_VALUE_SIGN: &str = "A value keeps the sign Tally's export carries, as in the Stock Summary and the trial balance: a debit is negative and a credit is positive. The closing value of an item short of stock is a credit, so it reads positive where Tally's screen shows it as (-): on the one book measured, the item at -3 Nos and a rate of 120.00 has the value 360.00 and its screen shows (-)360.00 (TallyPrime 7.1, again on 7.2; protocol reference section 12a.13)";

impl Server {
    pub(super) async fn negative_stock(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        if from > to {
            return Err("invalid_date_range".to_string().into());
        }
        let (company, identity, prior) = self.verified_company(guid).await?;
        let read = self
            .runtime
            .fetch_negative_stock(self.tally_config(), &identity, from.clone(), to.clone())
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("negative_stock_read_failed", error)
                    .with_prior_evidence(prior.clone())
            })?;
        let evidence = combine_evidence(prior, evidence_from_runtime_read(read.evidence));
        let (listed, empty): (Vec<Value>, bool) = match &read.answer {
            NegativeStockAnswer::Listed(list) => (
                list.items
                    .iter()
                    .map(|item| {
                        json!({
                            "item": party_name(item.name.clone()),
                            "quantity": quantity_json(&item.quantity),
                            "rate": item.rate,
                            "value": item.value,
                        })
                    })
                    .collect(),
                false,
            ),
            NegativeStockAnswer::EmptyEnvelope => (Vec::new(), true),
        };
        let mut unmeasured = Vec::new();
        if !is_measured_window(from.as_str(), to.as_str()) {
            unmeasured.push("window_other_than_from_1_april_within_one_financial_year");
        }
        let mut result = json!({
            "state": if empty { "not_established" } else { "observed" },
            "basis": "tally_native_negative_stock",
            "from": read.from, "to": read.to,
            "count": listed.len(),
            "listed_items": listed,
            "unmeasured_in_this_answer": unmeasured,
            "checks": {
                "list_complete": "not_checked",
                "quantities_against_stock_summary": "not_checked",
            },
            "verification": VERIFICATION,
            "limitations": [
                "This is Tally's own Negative Stock list, returned as it is. Being listed is not a finding that the quantity is below zero: on one book Tally also listed items of positive quantity whose closing value was a credit, so read each item's quantity as printed. The rule by which Tally lists an item is not established and is not derived here",
                NEGATIVE_STOCK_VALUE_SIGN,
                "A rate or value Tally printed empty is returned empty and is not zero; on one book the rate and value were empty on an item listed for a window ending mid-year",
                "On the one book read to a mid-year date the quantity was as at the window's end, not the book's end. Windows that do not start on 1 April, cross a financial year or end after 31 March were not measured",
                "An empty answer is returned as not established and does not say that no item is negative: Tally's empty answer cannot be told from a report that was not rendered (on a book with stock items and none short it was an empty envelope)",
                "The list is not compared with the Stock Summary or any other report, so an item Tally did not list is not shown to have a quantity of zero or more",
                "The report has no size check: the read is admitted on the book's master mark against a ceiling that is a stated guess, and its cost on a large book is not known. It is one paired read (two requests), never retried; if a call times out do not repeat it in a loop",
                "Education mode is refused, and the window must not start before the book",
                "Tally's report carries no company identity; it is bound only by the company, mode and book-extent checks around the read",
                "Not voucher-level reconciliation or an atomic snapshot",
            ],
        });
        if empty {
            if let Some(object) = result.as_object_mut() {
                object.insert(
                    "reason".to_string(),
                    json!("negative_stock_empty_answer_is_not_a_finding"),
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
#[path = "agent_negative_stock_tests.rs"]
mod tests;
