//! Thin MCP presentation of the shared stock summary read.
use super::trial_balance::page_boundary;
use super::*;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::native_stock_summary::{
    NativeFlag, NativeInventoryFlags, NativeItemCountCrossCheck, NativeStockGate, NativeStockItem,
    NativeStockTotals, StockSummaryAsOf,
};
use std::collections::HashSet;

/// The most items one call may name; the published schema states the same bound.
const MAX_ITEM_FILTER: usize = 50;

/// The longest GUID `items` may name, in characters; the published schema's
/// `maxLength` is the same.
const MAX_ITEM_GUID_CHARS: usize = 64;

/// Beside `value_sum`, whenever `totals` is returned: the values were added with
/// the signs Tally sent, and what a negative value means is unmeasured.
const VALUE_SUM_SIGNS: &str = "as_sent_meaning_unmeasured";

const VERIFICATION: &str = "stable_paired_sources_with_company_mode_and_extent_guards";

const NOT_ATOMIC: &str =
    "Not an atomic snapshot: paired reads and an unchanged book extent detect observed change only";

const ONE_PERIOD_MEASURED: &str = "Only the period ending 31 March 2026 has been measured: a 31 March of another year is admitted, sharing the request shape but not the measurement";

/// Fewer rows were read than Tally counts stock items. The numbers travel in
/// the refusal's `counts`.
const ROWS_BELOW_ITEM_COUNT: &str = "stock_summary_rows_below_item_count";

/// What is and is not checked, per field, in one closed vocabulary: `checked`,
/// `not_checked`, `withheld`. Only the value total is compared with a second
/// source; a quantity is read but never returned. Tally's item count is a
/// cross-check that refuses a short list; it is not proof of a complete one.
fn checks() -> Value {
    json!({
        "closing_value_total": "checked",
        "item_list_complete": "not_checked",
        "closing_value_each": "not_checked",
        "closing_quantity": "withheld",
        "name_parent_unit": "not_checked",
        "as_of_honoured": "not_checked",
    })
}

/// Why a completed read returns no stock rows.
#[derive(Clone, Copy)]
enum Withheld {
    Differs,
    ReportShowsNoValue,
    NotComparable,
}

impl Withheld {
    fn reason(self) -> &'static str {
        match self {
            Self::Differs => "tally_stock_summary_differs",
            Self::ReportShowsNoValue => "tally_stock_summary_shows_no_value",
            Self::NotComparable => "stock_values_not_comparable",
        }
    }

    /// What a caller does next.
    fn remediation(self) -> &'static str {
        match self {
            Self::Differs => "The closing values of the stock items Bridge read and Tally's own Stock Summary for the same period disagree (`unchecked_comparison` shows what each side added up to; a side with no figure is null), so no item is returned. Do not retry: each was read twice and the book did not change. Give the user both sides and ask them to open the Stock Summary in Tally for `period`; present neither figure as the stock value.",
            Self::ReportShowsNoValue => "Stock items exist and carry closing values, but Tally's Stock Summary for the same period came back with no amount, so nothing was confirmed and no item is returned. Bridge cannot tell whether Tally left the report blank or it truly shows no stock. Ask the user to open the Stock Summary in Tally for `period`: if it shows stock, Bridge cannot read it on this Tally and stock_summary should not be retried.",
            Self::NotComparable => "Nothing could be compared. Either no stock item Bridge read carries a closing value and Tally's Stock Summary for the period shows no amount or a total of zero, or the items' values add up to zero and the Stock Summary shows no amount. Bridge returns no stock rows it could not check. Do not retry: each was read twice and the book did not change. Tell the user stock for this period has to be read in Tally.",
        }
    }
}

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
                // This read may end without holding anything (a refusal, or a
                // result that returns no item). An earlier read of this date is
                // dropped first, so that a later page is never served rows a
                // newer read did not confirm.
                self.drop_listing(&identity, &listing)
                    .map_err(|failure| failure.with_prior_evidence(prior.clone()))?;
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
                // `comparison` is what the check that did not hold compared.
                // Nothing checked its figures: they leave under a name and a
                // note that say so, never as `tie_out` or a total.
                let withheld = |why: Withheld, comparison: Value| {
                    let mut evidence = combine_evidence(prior.clone(), read_evidence.clone());
                    // `items` is withheld: say so in the state an agent reads
                    // first, as `vouchers` and `outstandings` do.
                    evidence.state = "partial";
                    evidence.reason_code = Some(why.reason().to_string());
                    ToolOutcome {
                        payload: json!({
                            "company": company_json(&company, std::slice::from_ref(&company)),
                            "result": {
                                "state": "not_established",
                                "reason": why.reason(),
                                "remediation": why.remediation(),
                                "company_guid": guid, "as_of": read.to,
                                "period": {"from": read.from, "to": read.to},
                                "inventory": read.inventory,
                                "unchecked_comparison": comparison,
                                "items": null,
                                "verification": VERIFICATION,
                                "limitations": [
                                    "No stock row is returned unless its value total was compared with Tally's own Stock Summary and found equal",
                                    "Any figure under `unchecked_comparison` is what the comparison that did not hold added up on each side. Nothing checked it: it is for investigation only and is not a stock value or a total",
                                    "This read is not held, and it replaces any earlier read of the same date: a later page reads afresh",
                                ],
                            },
                        }),
                        evidence,
                        company_guid: Some(guid.to_string()),
                        truncated: false,
                    }
                };
                let (items, frame): (&[NativeStockItem], Value) = match &read.gate {
                    NativeStockGate::RowsBelowItemCount { rows, tally_count } => {
                        return Err(rows_below_item_count(*rows, *tally_count)
                            .with_prior_evidence(combine_evidence(
                                prior.clone(),
                                read_evidence.clone(),
                            )));
                    }
                    // The report has a total the items do not add up to: a
                    // figure Tally contradicts is not shown.
                    NativeStockGate::Differs {
                        items_total,
                        report_total,
                    } => {
                        return Ok(withheld(
                            Withheld::Differs,
                            json!({
                                "state": "differs",
                                "use": "investigation_only",
                                "items_closing_values_added": items_total,
                                "tally_stock_summary_lines_added": report_total,
                                "note": "Two figures from Tally that disagree. Neither was confirmed and neither is the stock value. The items' side adds only the closing values present: an item with no closing value adds nothing to it.",
                            }),
                        ));
                    }
                    NativeStockGate::ReportShowsNoValue { items_total } => {
                        return Ok(withheld(
                            Withheld::ReportShowsNoValue,
                            json!({
                                "state": "report_shows_no_value",
                                "use": "investigation_only",
                                "items_closing_values_added": items_total,
                                "note": "What the items' closing values added up to. Tally's Stock Summary showed no amount to compare it with, so it was not confirmed and is not the stock value. It adds only the closing values present: an item with no closing value adds nothing to it.",
                            }),
                        ));
                    }
                    NativeStockGate::NotComparable => {
                        return Ok(withheld(
                            Withheld::NotComparable,
                            json!({"state": "not_comparable"}),
                        ));
                    }
                    NativeStockGate::NoStockItems => (
                        &[][..],
                        json!({
                            "state": "no_stock_items",
                            "basis": "This company has no stock items: Tally's own item count is 0, the stock item list is empty and Tally's Stock Summary is empty. This is about the items defined in the company, not about a date.",
                            "company_guid": guid,
                            "as_of": read.to, "period": {"from": read.from, "to": read.to},
                            "inventory": read.inventory,
                            "totals": {"item_count": 0},
                            "item_count_cross_check": {"status": "matched", "rows": 0, "tally_count": 0},
                            "verification": VERIFICATION,
                            "limitations": [
                                NOT_ATOMIC,
                                "Tally's own stock item count is 0 and the stock item list is empty. The Stock Summary came back empty, which agrees with them and by itself is not told apart from a report Tally did not render. Nothing else was checked",
                                ONE_PERIOD_MEASURED,
                            ],
                        }),
                    ),
                    NativeStockGate::ValueTotalMatched {
                        items,
                        totals,
                        total,
                        report_empty_amounts,
                        item_count,
                    } => (
                        items.as_slice(),
                        stock_frame(
                            guid,
                            (&read.from, &read.to),
                            &read.inventory,
                            inventory_basis(integrated),
                            totals,
                            item_count,
                            json!({
                                "state": "matched", "total": total,
                                "report_empty_amounts": report_empty_amounts,
                            }),
                        ),
                    ),
                };
                let rows = items.iter().map(stock_row).collect::<Vec<_>>();
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

/// The item GUIDs named by `items`: one to fifty, each nonblank, at most
/// sixty-four characters and unique ignoring ASCII case. An unknown GUID is not
/// an error here; the result lists it under `items_not_found`.
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
                .filter(|guid| {
                    !guid.trim().is_empty() && guid.chars().count() <= MAX_ITEM_GUID_CHARS
                })
                .ok_or_else(invalid)?;
            seen.insert(guid.to_ascii_lowercase())
                .then(|| guid.to_string())
                .ok_or_else(invalid)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

/// What a read with fewer rows than Tally's own item count becomes: a refusal,
/// because the list may be incomplete. In the one delete measured the count
/// fell with the rows (§12a.13, one sample).
fn rows_below_item_count(rows: usize, tally_count: u64) -> ToolFailure {
    ToolFailure {
        counts: Some(Box::new(RowCounts {
            returned: rows as u64,
            counted: tally_count,
        })),
        ..ToolFailure::from(ROWS_BELOW_ITEM_COUNT.to_string())
    }
}

/// What a matched read is: what Tally said of `ISINTEGRATED`, that how the books
/// use these values is not measured, and exactly what was compared.
fn inventory_basis(integrated: NativeFlag) -> String {
    let reported = match integrated {
        NativeFlag::Yes => "Tally reported ISINTEGRATED Yes",
        NativeFlag::No => "Tally reported ISINTEGRATED No",
        NativeFlag::Unknown => "Tally did not send ISINTEGRATED",
    };
    format!(
        "{reported}. These are the stock items' closing values exactly as Tally sends them; how the books use them (as closing stock, or against a Stock-in-Hand ledger) is not measured. The closing values of all this company's stock items add up to the total of Tally's own Stock Summary for the period (`tie_out.total`). Only that total was compared: no item's value was checked on its own, and quantities are not returned because nothing checks them."
    )
}

/// One item as the tool returns it. Its `name` and its `parent` (a stock-group
/// name) are free text that can carry a customer's or supplier's name, so they
/// go out under the party-name marker, as `masters` does for stock groups;
/// Tally's reserved root as a parent is a fixed string and stays plain, and an
/// absent parent stays null. `guid` is the identity and stays plain. The item's
/// `opening` and every quantity are read and validated but not serialized
/// ([`NativeStockItem::opening`], [`NativeStockPosition::quantity`]).
fn stock_row(item: &NativeStockItem) -> Value {
    let mut row = json!(item);
    mark_party_field(&mut row, "name");
    if item
        .parent
        .as_deref()
        .is_some_and(|parent| !bridge_tally_protocol::is_tally_reserved_root(parent))
    {
        mark_party_field(&mut row, "parent");
    }
    row
}

/// Everything a page reports besides its items, held with the first page's read.
fn stock_frame(
    guid: &str,
    (from, to): (&TallyDate, &TallyDate),
    inventory: &NativeInventoryFlags,
    basis: String,
    totals: &NativeStockTotals,
    item_count: &NativeItemCountCrossCheck,
    tie_out: Value,
) -> Value {
    let mut totals = json!(totals);
    totals["value_sum_signs"] = json!(VALUE_SUM_SIGNS);
    json!({
        "state": "value_total_matched", "basis": basis, "company_guid": guid,
        "as_of": to, "period": {"from": from, "to": to},
        "inventory": inventory, "totals": totals, "tie_out": tie_out,
        "item_count_cross_check": item_count, "checks": checks(),
        "verification": VERIFICATION,
        "limitations": [
            NOT_ATOMIC,
            "Only the closing-value TOTAL is compared with Tally's own Stock Summary (`checks`): no item's value is checked on its own, so `value_total_matched` can stand beside `partial: true` when some items have no closing value",
            "Quantities are withheld: nothing checks them, so none is returned. A quantity Bridge could not read (a compound unit, or a unit with a space in it) is counted in `totals.closing_quantity_unread_count` and does not refuse the read",
            "An empty closing value is not zero: it is returned as null and counted (`empty_closing_value_count`), and `value_sum` is null with `partial` true whenever any item's closing value is empty. A value Tally sent as 0.00 is a value",
            "An item valued at zero or with no value adds nothing to either total, so nothing but Tally's own item count (`item_count_cross_check`) vouches for it; that count refuses the read only when it is higher than the rows read. It followed the one delete measured (one synthetic company, one sample), which is not proof of a complete list (`checks.item_list_complete`)",
            "Opening quantity and value are read but not returned, because their as-at date is unmeasured",
            "Values and their signs are exactly as Tally sends them: the one capture had items holding stock with a negative value beside others with a positive one, and what the sign means is unmeasured; `value_sum` adds the values as sent, signs included (`totals.value_sum_signs` says so)",
            "`totals`, `tie_out` and `item_count_cross_check` cover the whole book, whatever `items` filters",
            "Item names, parents and base units come from one source and are not checked against another",
            ONE_PERIOD_MEASURED,
            "A company split by year, whose sibling companies share its GUID, is refused (`company_flags_not_one_row`)",
            "Small books only: a book whose master-alteration mark is over the admitted size is refused before any item is read",
            "Under mask_parties an item's `name` and `parent` are masked, because stock-item and stock-group names are free text that can carry a customer's or supplier's name; Tally's reserved root as a parent is left as it is, and `guid` is not masked",
        ],
    })
}

#[cfg(test)]
#[path = "agent_stock_summary_tests.rs"]
mod tests;
