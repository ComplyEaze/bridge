//! Thin MCP presentation of the shared stock summary read.
use super::trial_balance::page_boundary;
use super::*;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::native_stock_summary::{
    NativeFlag, NativeInventoryFlags, NativeStockGate, NativeStockTotals, StockSummaryAsOf,
};
use std::collections::HashSet;

/// The most items one call may name; the published schema states the same bound.
const MAX_ITEM_FILTER: usize = 50;

const VERIFICATION: &str = "stable_paired_sources_with_company_mode_and_extent_guards";

impl Server {
    pub(super) async fn stock_summary(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        // Before any read: an unsupported date or a filter that cannot be
        // honoured costs no Tally request.
        let as_of = TallyDate::parse(normalized_date(required_string(args, "as_of")?)?)
            .map_err(|_| "invalid_date".to_string())?;
        let as_of = StockSummaryAsOf::new(as_of).map_err(|error| error.code().to_string())?;
        let wanted = item_filter(args)?;
        let (company, identity, mut prior) = self.verified_company(guid).await?;
        let offset = arg_usize(args, "offset", 0)?;
        let limit =
            arg_positive_usize(args, "limit", self.settings.max_rows)?.min(self.settings.max_rows);
        let snapshot_id = optional_string(args, "snapshot_id")?;
        // The `items` filter is applied to the held read, so it is not part of
        // the listing's key: every filter over one date shares one snapshot.
        let listing = ListingKind::StockSummary {
            as_of: as_of.date().clone(),
        };
        // A first page always reads fresh; a later page is served from its
        // listing's snapshot only while the book's extent is unchanged (#630).
        let reused = self
            .continued_listing(
                &identity,
                &listing,
                offset,
                snapshot_id.as_deref(),
                &mut prior,
            )
            .await
            .map_err(|failure| failure.with_prior_evidence(prior.clone()))?;
        let snapshot = match reused.clone() {
            Some(held) => held,
            None => {
                let today = TallyDate::parse(tally_host_today())
                    .map_err(|_| "current_date_invalid".to_string())?;
                let (read, extent) = self
                    .runtime
                    .fetch_stock_summary_with_extent(self.tally_config(), &identity, as_of, today)
                    .await
                    .map_err(|error| {
                        ToolFailure::from_runtime("stock_summary_read_failed", error)
                            .with_prior_evidence(prior.clone())
                    })?;
                let read_evidence = evidence_from_runtime_read(read.evidence);
                let integrated = read.inventory.integrated;
                let build_frame = |basis: String, totals: &NativeStockTotals, tie_out: Value| {
                    stock_frame(
                        guid,
                        (&read.from, &read.to),
                        &read.inventory,
                        basis,
                        totals,
                        tie_out,
                    )
                };
                let (items, frame) = match &read.gate {
                    // Tally's own total contradicts the items: none is shown.
                    NativeStockGate::Differs {
                        items_total,
                        report_total,
                    } => {
                        return Ok(ToolOutcome {
                            payload: json!({
                                "company": company_json(&company, std::slice::from_ref(&company)),
                                "result": {
                                    "state": "not_established",
                                    "reason": "tally_stock_summary_differs",
                                    "company_guid": guid, "as_of": read.to,
                                    "period": {"from": read.from, "to": read.to},
                                    "inventory": read.inventory,
                                    "tie_out": {
                                        "state": "differs",
                                        "items_value_sum": items_total,
                                        "report_total": report_total,
                                    },
                                    "items": null,
                                    "verification": VERIFICATION,
                                    "limitations": [
                                        "The items' closing-value sum differs from Tally's own Stock Summary total, so no item is returned: a figure Tally contradicts is not shown",
                                        "Nothing is held to continue from; call again to read afresh",
                                    ],
                                },
                            }),
                            evidence: combine_evidence(prior, read_evidence),
                            company_guid: Some(guid.to_string()),
                            truncated: false,
                        });
                    }
                    NativeStockGate::Matched {
                        items,
                        totals,
                        total,
                        report_empty_amounts,
                    } => (
                        items,
                        build_frame(
                            inventory_basis(integrated, None),
                            totals,
                            json!({
                                "state": "matched", "total": total,
                                "report_empty_amounts": report_empty_amounts,
                            }),
                        ),
                    ),
                    NativeStockGate::NotChecked {
                        items,
                        totals,
                        reason,
                    } => (
                        items,
                        build_frame(
                            inventory_basis(integrated, Some(*reason)),
                            totals,
                            json!({"state": "not_checked", "reason": reason}),
                        ),
                    ),
                };
                let rows = items.iter().map(|item| json!(item)).collect::<Vec<_>>();
                self.hold_listing(ListingSnapshot::new(
                    &identity,
                    listing,
                    extent,
                    rows,
                    None,
                    frame,
                    read_evidence,
                ))?
            }
        };
        // A page served from a snapshot records only what it sent: the
        // identity and extent reads, not its first page's read again.
        let evidence = match reused {
            Some(_) => prior,
            None => combine_evidence(prior, snapshot.evidence.clone()),
        };
        let (selected, not_found) = match &wanted {
            None => (snapshot.rows.iter().collect::<Vec<_>>(), Vec::new()),
            Some(guids) => {
                let is = |row: &Value, guid: &str| {
                    row["guid"]
                        .as_str()
                        .is_some_and(|held| held.eq_ignore_ascii_case(guid))
                };
                (
                    snapshot
                        .rows
                        .iter()
                        .filter(|row| guids.iter().any(|guid| is(row, guid.as_str())))
                        .collect(),
                    guids
                        .iter()
                        .filter(|guid| !snapshot.rows.iter().any(|row| is(row, guid.as_str())))
                        .cloned()
                        .collect::<Vec<_>>(),
                )
            }
        };
        let total = selected.len();
        let page = selected
            .into_iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        let (truncated, next_offset) = page_boundary(offset, page.len(), total);
        let mut result = snapshot.frame.clone();
        result["items"] = json!(page);
        result["total"] = json!(total);
        result["offset"] = json!(offset);
        result["next_offset"] = json!(next_offset);
        result["snapshot"] = snapshot.describe(reused.is_some());
        result["limitations"] = json!([
            "Not an atomic snapshot: paired reads and an unchanged book extent detect observed change only",
            "Company totals at `as_of` only, with no godown or batch split: `negative_closing_quantity_count` does not count batch, godown or in-year negatives",
            "An empty quantity or value is not zero: each is counted (`empty_closing_quantity_count`, `empty_closing_value_count`), and `value_sum` is null with `partial` true when an item that holds stock has no closing value",
            "`totals` and `tie_out` cover the whole book, whatever `items` filters",
            "Small books only: a book whose master-alteration mark is over the admitted size is refused before any item is read",
            "Stock item names are not masked by mask_parties: they are not party names",
        ]);
        if !not_found.is_empty() {
            result["items_not_found"] = json!(not_found);
        }
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated,
        })
    }
}

/// The item GUIDs named by `items`: one to fifty, each nonblank and unique
/// ignoring ASCII case. An unknown GUID is not an error here; the result lists
/// it under `items_not_found`.
fn item_filter(args: &Value) -> Result<Option<Vec<String>>, String> {
    let Some(value) = args.get("items") else {
        return Ok(None);
    };
    let invalid = || "argument_invalid:items".to_string();
    let values = value.as_array().ok_or_else(invalid)?;
    if values.is_empty() || values.len() > MAX_ITEM_FILTER {
        return Err(invalid());
    }
    let mut seen = HashSet::new();
    values
        .iter()
        .map(|value| {
            let guid = value
                .as_str()
                .filter(|guid| !guid.trim().is_empty())
                .ok_or_else(invalid)?;
            seen.insert(guid.to_ascii_lowercase())
                .then(|| guid.to_string())
                .ok_or_else(invalid)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

/// What the items are, given whether Tally says inventory is integrated with
/// the accounts, and, when the report could not be compared, that they are not
/// checked.
fn inventory_basis(integrated: NativeFlag, unchecked: Option<&str>) -> String {
    let basis = match integrated {
        NativeFlag::Yes => {
            "the books carry this item valuation as closing stock (Tally's Stock Summary ties to it)"
        }
        NativeFlag::No => {
            "the Balance Sheet uses the Stock-in-Hand ledger's entered closing stock; this is Tally's item valuation and is NOT reconciled to it"
        }
        NativeFlag::Unknown => {
            "Tally did not say whether inventory is integrated with the accounts, so whether the books carry this item valuation as closing stock is unknown"
        }
    };
    match unchecked {
        None => basis.to_string(),
        Some(reason) => format!(
            "{basis}. These figures were NOT checked against Tally's own Stock Summary total ({reason})"
        ),
    }
}

/// Everything a page reports besides its items, held with the first page's read.
fn stock_frame(
    guid: &str,
    (from, to): (&TallyDate, &TallyDate),
    inventory: &NativeInventoryFlags,
    basis: String,
    totals: &NativeStockTotals,
    tie_out: Value,
) -> Value {
    json!({
        "state": "observed", "basis": basis, "company_guid": guid,
        "as_of": to, "period": {"from": from, "to": to},
        "inventory": inventory, "totals": totals, "tie_out": tie_out,
        "verification": VERIFICATION,
    })
}

#[cfg(test)]
#[path = "agent_stock_summary_tests.rs"]
mod tests;
