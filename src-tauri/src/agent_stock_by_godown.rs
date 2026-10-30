//! Thin MCP presentation of the closing-stock-by-godown read (#2017).
use super::*;
use crate::tally::runtime::StockByGodownRead;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::native_godown_summary::{
    GodownNetQuantity, GodownQuantityCheck, NativeGodownSummary, NativeGodownTie,
    NativeItemByGodown, NativeItemGodowns,
};
use bridge_tally_protocol::native_stock_summary::{NativeQuantityRead, StockSummaryAsOf};

const VERIFICATION: &str = "stable_paired_reports_with_company_mode_and_extent_guards";

const LIMITATIONS: [&str; 7] = [
    "A godown's quantity is Tally's net of every item in that godown that shares a unit, never one item's closing: on the book measured, 316.667 Kgs was one item's 366.667 Kgs and another's -50.000 Kgs. It is empty for a godown holding items in several units, and the cause of an empty quantity is not measured",
    "Godowns are the ones directly under Tally's root: a godown under another godown has no line of its own, and its stock is in its parent's figures",
    "A value per item per godown is returned only for an item directly under the root, each godown at its own cost as Tally values it (not the item's average rate times the quantity), and only where the godown values and quantities add up to the item's own line. Tally gives no value per godown for an item inside a stock group, so none is returned for it",
    "Signs are kept as Tally's export carries them, as in stock_summary: a negative value is a debit, which is stock held, and Tally's screen shows it as positive",
    "An empty value or quantity is returned as empty and is not zero",
    "Measured on one synthetic book on TallyPrime 7.2 at two month ends (protocol reference section 12a.24); a batch-wise item under the root, a godown two levels down and other releases are not measured",
    "Three reports are each read twice inside one company, mode and book-extent bracket; the size is admitted on the book's master mark against a ceiling that is a stated guess. Education mode is refused. Not an atomic snapshot",
];

fn net_quantity_json(quantity: &GodownNetQuantity) -> Value {
    match quantity {
        GodownNetQuantity::NetAcrossItems(net) => {
            json!({"state": "net_across_items", "amount": net.amount.as_str(), "unit": net.unit})
        }
        GodownNetQuantity::Empty => json!({"state": "empty"}),
        GodownNetQuantity::Unread => json!({"state": "unread"}),
    }
}

fn quantity_json(quantity: &NativeQuantityRead) -> Value {
    match quantity {
        NativeQuantityRead::Read(read) => {
            json!({"state": "read", "amount": read.amount.as_str(), "unit": read.unit})
        }
        NativeQuantityRead::Empty => json!({"state": "empty"}),
        NativeQuantityRead::Unread => json!({"state": "unread"}),
    }
}

fn item_json(item: &NativeItemGodowns) -> Value {
    let mut entry = json!({
        "item": party_name(item.item.clone()),
        "state": item.outcome.code(),
    });
    let detail = match &item.outcome {
        NativeItemByGodown::Tied {
            value,
            quantity,
            godowns,
        } => json!({
            "value": value,
            "quantities": match quantity {
                GodownQuantityCheck::Added => "added_to_the_item_quantity",
                GodownQuantityCheck::NotCompared => "not_compared",
            },
            "godowns": godowns.iter().map(|line| json!({
                "godown": party_name(line.godown.clone()),
                "quantity": quantity_json(&line.quantity),
                "value": line.value,
            })).collect::<Vec<_>>(),
        }),
        NativeItemByGodown::ValuesDiffer {
            item_value,
            godown_sum,
        } => json!({"item_value": item_value, "godown_value_sum": godown_sum}),
        NativeItemByGodown::QuantitiesDiffer {
            item_quantity,
            godown_sum,
        } => json!({
            "item_quantity": {"amount": item_quantity.amount.as_str(), "unit": item_quantity.unit},
            "godown_quantity_sum": godown_sum,
        }),
        NativeItemByGodown::ValueEmpty
        | NativeItemByGodown::BatchNamed
        | NativeItemByGodown::GodownRepeated
        | NativeItemByGodown::NotInPlainReport => json!({}),
    };
    if let (Some(entry), Some(detail)) = (entry.as_object_mut(), detail.as_object()) {
        entry.extend(detail.clone());
    }
    entry
}

/// The result: godowns and items only where the godown total ties to the Stock
/// Summary; otherwise neither, with both totals.
fn result_json(read: &StockByGodownRead) -> Value {
    let (state, reason, tie_out) = match &read.tie {
        NativeGodownTie::Tied { total } => (
            "tied",
            None,
            json!({"godown_total": total, "stock_summary_total": total}),
        ),
        NativeGodownTie::Differs {
            godown_total,
            stock_summary_total,
        } => (
            "not_established",
            Some("godown_totals_differ"),
            json!({"godown_total": godown_total, "stock_summary_total": stock_summary_total}),
        ),
        NativeGodownTie::NotComparable {
            godown_total,
            stock_summary_total,
            godown_empty_amounts,
        } => (
            "not_established",
            Some("godown_totals_not_comparable"),
            json!({
                "godown_total": godown_total,
                "stock_summary_total": stock_summary_total,
                "godown_empty_amounts": godown_empty_amounts,
            }),
        ),
    };
    let tied = reason.is_none();
    let godowns = match (&read.godowns, tied) {
        (NativeGodownSummary::Lines { lines, .. }, true) => Value::Array(
            lines
                .iter()
                .map(|line| {
                    json!({
                        "godown": party_name(line.name.clone()),
                        "quantity": net_quantity_json(&line.quantity),
                        "value": line.amount,
                    })
                })
                .collect(),
        ),
        _ => Value::Null,
    };
    let root_items = match (&read.items, tied) {
        (Some(items), true) => Value::Array(items.iter().map(item_json).collect()),
        _ => Value::Null,
    };
    let mut result = json!({
        "state": state,
        "basis": "tally_native_godown_summary_and_exploded_stock_summary",
        "from": read.from,
        "to": read.to,
        "tie_out": tie_out,
        "godowns": godowns,
        "root_items": root_items,
        "checks": {
            "godown_total_against_stock_summary": "checked",
            "root_item_godown_values_against_the_item": "checked",
            "godown_names_against_godown_masters": "not_checked",
            "items_inside_stock_groups": "not_available",
        },
        "verification": VERIFICATION,
        "limitations": LIMITATIONS,
    });
    if let Some(object) = result.as_object_mut() {
        if let Some(reason) = reason {
            object.insert("reason".to_string(), json!(reason));
        } else if read.items.is_none() {
            object.insert(
                "root_items_reason".to_string(),
                json!("exploded_stock_summary_empty_is_not_a_finding"),
            );
        }
    }
    result
}

impl Server {
    pub(super) async fn stock_by_godown(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        // Before any read: an unmeasured date costs no Tally request.
        let as_of = normalized_date(required_string(args, "as_of")?)?;
        let as_of = StockSummaryAsOf::new(as_of)
            .map_err(|_| "stock_by_godown_as_of_not_measured".to_string())?;
        let (company, identity, prior) = self.verified_company(guid).await?;
        let today =
            TallyDate::parse(tally_host_today()).map_err(|_| "current_date_invalid".to_string())?;
        let read = self
            .runtime
            .fetch_stock_by_godown(self.tally_config(), &identity, as_of, today)
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("stock_by_godown_read_failed", error)
                    .with_prior_evidence(prior.clone())
            })?;
        let evidence = combine_evidence(prior, evidence_from_runtime_read(read.evidence.clone()));
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result_json(&read),
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_stock_by_godown_tests.rs"]
mod tests;
