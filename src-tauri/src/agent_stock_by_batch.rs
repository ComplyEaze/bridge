//! Thin MCP presentation of the stock-by-batch read (#2018).
use super::stock_summary::item_guids;
use super::*;
use crate::tally::runtime::{StockByBatchItem, StockByBatchRead};
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::native_stock_by_batch::{GodownSubtotal, StockQueryTie};
use bridge_tally_protocol::native_stock_query::{StockQueryBatch, StockQueryLine};
use bridge_tally_protocol::native_stock_summary::{
    NativeFlag, NativeQuantityRead, NativeStockQuantity, StockSummaryAsOf,
};

/// The most items one call may name; the published schema states the same bound. Each is one
/// Stock Query, read one at a time, so the call's cost grows with it. A choice, not a
/// measurement.
pub(super) const MAX_STOCK_BY_BATCH_ITEMS: usize = 20;

const VERIFICATION: &str = "each_item_tied_to_its_closing_in_one_company_mode_and_extent_bracket";
const VERIFICATION_STOPPED: &str = "stopped_before_the_closing_reads";

/// Tally's own text for stock kept in no named batch.
const DEFAULT_BATCH: &str = "Primary Batch";

/// What the result holds, and what nothing checked.
const LIMITATIONS: [&str; 9] = [
    "Quantities only: Stock Query returns no value, rate, expiry or manufacturing date, and no last purchase or last sale is returned",
    "Each item's lines are tied as a whole to its closing quantity in Tally's stock item list for the same period (`state`); no line, godown or batch is checked on its own, and nothing is compared with any voucher, because a Stock Journal moves stock between godowns without an invoice goods line",
    "Godown totals (`godowns`) are the sums of an item's own Stock Query lines per godown, compared with nothing; they are not Tally's Godown Summary. Each names its godown through its first line (`lines`)",
    "An empty Stock Query answer is never read as zero by itself: Tally sends the same empty answer for an item with no stock, for one that never moved and for a name it does not know. It is `no_stock` only beside an explicit zero closing in Tally's stock item list, and `no_movement` beside no closing at all, which is not a statement that the item holds none. PARTIAL: both seen in one sitting of one synthetic book",
    "Units are compared as Tally writes them, and never converted. An item with an alternate unit is `not_comparable`: Stock Query writes its base unit only, and its closing is written in both",
    "Godown and batch names are compared as sent: two that differ only in letter case are two",
    "Tally's failure answer is read as Tally's failure (`stock_query_tally_reported_failure`); no Stock Query answer has been seen in that shape (UNVERIFIED)",
    "Measured on TallyPrime 7.2 on one synthetic book, at three month ends. A year after the books' first day it answered closings, in a month where no closing moved; a month where one moves is not measured, nor are other releases and books",
    "Under mask_parties an item's `name` and every godown and named batch are masked, because they are text a person types, which can carry a customer's or supplier's name; `Primary Batch` is Tally's own text and is not masked, and `guid` is not masked",
];

fn quantity_json(quantity: &NativeStockQuantity) -> Value {
    json!({"amount": quantity.amount, "unit": quantity.unit})
}

/// The closing as Tally's stock item list sent it: empty is not zero, and a compound unit is
/// unread, not a quantity.
fn closing_json(closing: &NativeQuantityRead) -> Value {
    match closing {
        NativeQuantityRead::Read(quantity) => {
            json!({"state": "read", "amount": quantity.amount, "unit": quantity.unit})
        }
        NativeQuantityRead::Empty => json!({"state": "empty"}),
        NativeQuantityRead::Unread => json!({"state": "unread"}),
    }
}

/// One line as returned: its godown and a named batch go out under the party-name marker, as the
/// voucher goods lines carry the same fields (#2015).
fn line_json(line: &StockQueryLine) -> Value {
    let mut batch = match &line.batch {
        StockQueryBatch::Default => json!({"kind": "default", "name": DEFAULT_BATCH}),
        StockQueryBatch::Named(name) => json!({"kind": "named", "name": name}),
    };
    if matches!(line.batch, StockQueryBatch::Named(_)) {
        mark_party_field(&mut batch, "name");
    }
    let mut row = json!({
        "godown": line.godown,
        "batch": batch,
        "quantity": quantity_json(&line.quantity),
    });
    mark_party_field(&mut row, "godown");
    row
}

/// A godown's total, keyed by its lines' positions, never by its name.
fn godown_json(subtotal: &GodownSubtotal) -> Value {
    json!({
        "lines": subtotal.lines,
        "quantity": subtotal.quantity.as_ref().map(quantity_json),
        "source": "stock_query_lines_added",
    })
}

/// The item's identity as returned: `name` masked under mask_parties, `guid` plain. Its parent is
/// not returned.
fn item_head(guid: &str, name: Option<&str>, state: &str) -> Value {
    let mut row = json!({"guid": guid, "name": name, "state": state});
    if name.is_some() {
        mark_party_field(&mut row, "name");
    }
    row
}

fn item_json(result: &StockByBatchItem, basis: &mut headline::StockByBatchBasis) -> Value {
    match result {
        StockByBatchItem::Read {
            item,
            lines,
            subtotals,
            tie,
        } => {
            match tie {
                StockQueryTie::Tied(_) => basis.tied += 1,
                StockQueryTie::Differs { .. } => basis.differs += 1,
                StockQueryTie::NotComparable(_) => basis.not_comparable += 1,
                StockQueryTie::NoStock => basis.no_stock += 1,
                StockQueryTie::NoMovement => basis.no_movement += 1,
            }
            let mut row = item_head(&item.guid, Some(&item.name), tie.code());
            row["lines"] = json!(lines.iter().map(line_json).collect::<Vec<_>>());
            row["godowns"] = json!(subtotals.iter().map(godown_json).collect::<Vec<_>>());
            row["closing"] = closing_json(&item.closing.quantity);
            row["lines_total"] = match tie {
                StockQueryTie::Tied(total)
                | StockQueryTie::Differs {
                    lines_total: Some(total),
                } => quantity_json(total),
                _ => Value::Null,
            };
            if let StockQueryTie::NotComparable(why) = tie {
                row["not_comparable"] = json!(why.code());
            }
            row
        }
        StockByBatchItem::ItemNotFound { guid } => {
            basis.item_not_found += 1;
            item_head(guid, None, "item_not_found")
        }
        StockByBatchItem::NameUnmeasured { item } => {
            basis.name_unmeasured += 1;
            item_head(&item.guid, Some(&item.name), "name_unmeasured")
        }
        StockByBatchItem::Refused { item, code } => {
            basis.refused += 1;
            let mut row = item_head(&item.guid, Some(&item.name), "refused");
            row["code"] = json!(code);
            row
        }
        StockByBatchItem::NotRead { item, code } => {
            basis.not_read += 1;
            let mut row = item_head(&item.guid, Some(&item.name), "not_read");
            row["code"] = json!(code);
            row
        }
        StockByBatchItem::NotSent { item } => {
            basis.not_sent += 1;
            item_head(&item.guid, Some(&item.name), "not_sent")
        }
    }
}

fn batch_wise(flag: NativeFlag) -> &'static str {
    match flag {
        NativeFlag::Yes => "yes",
        NativeFlag::No => "no",
        NativeFlag::Unknown => "unknown",
    }
}

/// The result of a read, with its headline.
fn stock_by_batch_result(
    guid: &str,
    company: &TallyCompany,
    read: &StockByBatchRead,
) -> (Value, headline::Headline, bool) {
    let mut basis = headline::StockByBatchBasis {
        requested: read.items.len(),
        batch_wise_off: read.inventory.batchwise == NativeFlag::No,
        ..Default::default()
    };
    let items = read
        .items
        .iter()
        .map(|item| item_json(item, &mut basis))
        .collect::<Vec<_>>();
    let whole = basis.whole();
    let stopped = read.stopped.is_some();
    let result = json!({
        "state": if whole { "complete" } else { "partial" },
        "company_guid": guid,
        "as_of": read.to,
        "period": {"from": read.from, "to": read.to},
        "batch_wise": batch_wise(read.inventory.batchwise),
        "stopped": read.stopped.map(|code| json!({"code": code})),
        "items": items,
        "verification": if stopped { VERIFICATION_STOPPED } else { VERIFICATION },
        "limitations": LIMITATIONS,
    });
    let headline = basis.headline(&headline::CompanyName::new(&company.name), &read.to);
    (result, headline, whole)
}

impl Server {
    pub(super) async fn stock_by_batch(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        // Before any read: a date or a list that cannot be honoured costs no Tally request.
        let as_of = normalized_date(required_string(args, "as_of")?)?;
        // The month-end rule is stock_summary's; its refusal is named for this tool.
        let as_of = StockSummaryAsOf::new(as_of)
            .map_err(|_| "stock_by_batch_as_of_not_measured".to_string())?;
        let items = item_guids(args, MAX_STOCK_BY_BATCH_ITEMS)?
            .ok_or_else(|| "argument_invalid:items".to_string())?;
        let (company, identity, prior) = self.verified_company(guid).await?;
        let today =
            TallyDate::parse(tally_host_today()).map_err(|_| "current_date_invalid".to_string())?;
        let read = self
            .runtime
            .fetch_stock_by_batch(self.tally_config(), &identity, as_of, items, today)
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("stock_by_batch_read_failed", error)
                    .with_prior_evidence(prior.clone())
            })?;
        let (result, headline, whole) = stock_by_batch_result(guid, &company, &read);
        let mut evidence = combine_evidence(prior, evidence_from_runtime_read(read.evidence));
        if !whole {
            // Say so in the state an agent reads first, as `vouchers` does.
            evidence.state = "partial";
            evidence.reason_code = read.stopped.map(str::to_string);
        }
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "headline": headline,
                "result": result,
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_stock_by_batch_tests.rs"]
mod tests;
