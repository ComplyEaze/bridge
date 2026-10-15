//! Thin MCP presentation of the shared masters read.
use super::trial_balance::page_boundary;
use super::*;
use crate::tally::runtime::{MastersKind, MastersRows};
use bridge_tally_protocol::gst_details::GstDetailsObservation;
use bridge_tally_protocol::native_masters::{
    NativeMasterDetail, NativeMasterKind, NativeMasterRow, NativeNumberingMethod,
};

impl Server {
    pub(super) async fn masters(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        // Before any read: a refused kind costs no Tally request.
        let kind = MastersKind::parse(required_string(args, "kind")?)
            .ok_or_else(|| "argument_invalid:kind".to_string())?;
        let (company, identity, mut prior) = self.verified_company(guid).await?;
        let offset = arg_usize(args, "offset", 0)?;
        let limit =
            arg_positive_usize(args, "limit", self.settings.max_rows)?.min(self.settings.max_rows);
        let snapshot_id = optional_string(args, "snapshot_id")?;
        let listing = ListingKind::Masters { kind };
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
                let (read, extent) = self
                    .runtime
                    .fetch_masters_with_extent(self.tally_config(), &identity, kind)
                    .await
                    .map_err(|error| {
                        ToolFailure::from_runtime("masters_read_failed", error)
                            .with_prior_evidence(prior.clone())
                    })?;
                let rows = match &read.rows {
                    MastersRows::Native(masters) => masters
                        .rows
                        .iter()
                        .map(|row| native_row(row, marks_party_text(kind)))
                        .collect::<Vec<_>>(),
                    MastersRows::Groups(groups) => groups
                        .iter()
                        .map(|group| {
                            json!({
                                "name": group.name,
                                "parent": group.parent.nonempty_returned_text(),
                                "reserved_name": group.reserved_name,
                            })
                        })
                        .collect::<Vec<_>>(),
                };
                self.hold_listing(ListingSnapshot::new(
                    &identity,
                    listing,
                    extent,
                    rows,
                    None,
                    Value::Null,
                    evidence_from_runtime_read(read.evidence),
                ))?
            }
        };
        // A page served from a snapshot records only what it sent: the
        // identity and extent reads, not its first page's read again.
        let evidence = match reused {
            Some(_) => prior,
            None => combine_evidence(prior, snapshot.evidence.clone()),
        };
        let total = snapshot.rows.len();
        let rows = snapshot
            .rows
            .iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        let (truncated, next_offset) = page_boundary(offset, rows.len(), total);
        let mut limitations = vec![
            "Not an atomic snapshot: paired reads and an unchanged book extent detect observed change only",
            "One kind per call; no counts or hints; a master's aliases are not returned",
            if marks_party_text(kind) {
                "Under mask_parties, godown, stock-group, stock-item, cost-centre and cost-category names, parents and categories are masked, because a job-work godown, a supplier-named stock group or stock item or a customer-named cost centre can carry a party's name; Tally's reserved root as a parent is left as it is"
            } else {
                "Voucher-type, unit and account-group names are not masked by mask_parties: they are configuration labels, not counterparties (godown, stock-group, stock-item, cost-centre and cost-category names are masked)"
            },
        ];
        match kind {
            MastersKind::Native(NativeMasterKind::VoucherTypes) => limitations.extend([
                "numbering_method is the type-level value as Tally reported it, not the voucher type's numbering method: it read `None` for types set to Automatic and to Manual on the screen (one synthetic book, Windows). It can differ from the number series that decides what a supplied number does; do not pass it to voucher_presence's numbering. A value other than Automatic, Manual or Default is returned raw under `unrecognised`",
                "`default` is Tally's reported value, not evidence that the type numbers automatically",
                "The company's voucher-type count (NUMVOUCHERTYPES) did not equal the rows returned on two books (35 vs 26, 33 vs 24), and on one book equalled the number-series count (inferred to count series, unmeasured): do not check these rows against it",
                "The completeness of this list is unverified: absence from it is not evidence that a voucher type is absent from the book",
                "Voucher types are read whole with no size check before the read; rows, AlterIDs and response size are checked after it",
            ]),
            MastersKind::Native(NativeMasterKind::CostCentres | NativeMasterKind::CostCategories) => {
                limitations.extend([
                    "Cost centres and categories are returned whether or not the company's Cost Centres setting is on: a book with the setting off was read with its two centres and two categories and a book with it on with its three centres, one under another, and its one category (two synthetic books, the same shape); this call does not return the setting",
                    "An empty list of cost centres does not say whether the feature is off or no centre is defined; one book with none defined answered an empty list",
                    "An empty list of cost categories is refused, on the expectation that the predefined Primary Cost Category always exists: it was present in the two books whose categories were captured (one with the setting at No, one at Yes), and the categories of a book with none defined were not captured",
                    "ComplyEaze Bridge does not return how a voucher was allocated to a cost centre: no tool reads those allocations yet, so their absence from any other tool's answer says nothing",
                    "Measured on three synthetic books for cost centres and two for cost categories (the first two on one release; the third was captured in a separate sitting, and that sitting's own status read recorded TallyPrime 7.1, licence Silver, Education mode off; the status answer is not committed); centres nested more than one level deep, a centre with an alias beyond its own name and a larger book are not measured, and a larger book is refused when its master mark is too large for the response budget",
                ]);
            }
            MastersKind::Native(NativeMasterKind::StockItems) => limitations.extend(STOCK_ITEM_LIMITATIONS),
            MastersKind::Groups => limitations.push(
                "Groups are read whole from the group snapshot, with no size check before the read",
            ),
            _ => {}
        }
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": {
                    "state": "observed", "basis": "tally_native_masters",
                    "company_guid": guid, "kind": kind.as_str(),
                    "masters": rows, "total": total,
                    "offset": offset, "next_offset": next_offset,
                    "verification": "stable_paired_source_with_company_mode_and_extent_guards",
                    "snapshot": snapshot.describe(reused.is_some()),
                    "limitations": limitations,
                },
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated,
        })
    }
}

/// What a `stock_items` answer says about its two lists. The size line names the
/// master mark above which the kind is refused: the response budget over the
/// kind's worst row (`masters_worst_row_bytes`), asserted equal in the tests.
const STOCK_ITEM_LIMITATIONS: [&str; 7] = [
    "hsn and gst are the item's own dated lists as Tally sent them: entries sorted by applicable_from, each as sent, one observation per list (entries, unreadable with its defect, or not_observed when the row had no such list). Nothing chooses the code or the rate for a date, adds a rate up or fills in a value Tally did not send",
    "An empty list (entries: []) or a missing one (not_observed) is not a statement that the item has no HSN code or no GST rate: what its stock group or the company holds is not read here. One thing was measured, on one book: after a code was keyed on a stock group, the group's four other items still carried empty lists. Not measured: a GST rate on a group or on the company, an HSN code on the company, and whether an item that defers to a coded group inherits its code. Every row says so with inherited_from_group_or_company: not_read",
    "Tally adds an HSN entry and a GST entry of its own, with source `As per Company/Stock Group`, no code or rate and, as its date, the book's first day (1 Apr 2025 on the one book measured; no other book's date is measured), when an item is first saved: recognise it by its source, not by that date; such an entry says the item defers to its group or the company, not that a code or rate exists",
    "state_name and valuation_type carry the reserved-character marker U+FFFD `#4;` where Tally sends its own `Any` and `Not Applicable`; a rate is absent, empty, zero or a value, never merged",
    "ineligible_itc is Tally's GSTINELIGIBLEITC as sent: true for Yes, false for No, null when the element was absent, never filled in. On one item it was Yes on the entry whose screen row read `Eligible for Input Tax Credit: No`, and absent on the entry whose screen row read Yes, which fits an ineligibility flag (PARTIAL: one book, two rows, screen against wire). What an absent element means for the tax is not known: do not read null as eligible or as ineligible, and do not read true as eligible. On an entry whose source is `As per Company/Stock Group` (the entry Tally adds itself on a first save; a person can also choose that source) a flag Tally sent is withheld: ineligible_itc is null and ineligible_itc_withheld is `tally_default_entry`, because the Yes seen on such an entry was never compared with the screen (both screen readings are of rows a person edited or keyed, and a row's origin and its flag cannot be told apart on one item); a flag Tally did not send stays null with no marker, and an entry with any other source is sent as it is",
    "A list with a defect (an undated entry, a repeated field, a rate that is not a plain decimal, two different entries on one date, or an entry holding only content that is not modelled, such as text, a reference or CDATA directly in the list) is returned as unreadable with its defect for that item's list only; a Tally ERROR, markup inside a value or a cut-off list refuses the whole read. HSN codes and rates are not names. An HSN description is filled by Tally from its tariff list and a person can edit it, so under mask_parties it is masked like an item name",
    "Measured on one synthetic book, TallyPrime 7.1 Silver (PARTIAL): a state row other than `Any`, slab rates, another release, a larger book and a zero-row stock-item answer are not measured. Items are read only when the master mark is at most 489 (the response budget over an assumed worst-case item row), so a limit of 489 refuses, by arithmetic, both client books measured (marks of about 100,000 and 300,000; not tried on them); a larger book is refused as masters_too_large. An item row is refused as masters_row_exceeds_bound past 16,344 UTF-16 units (about seven further GST entries of the measured size), whatever its state rows",
];

/// The `source` Tally gives the HSN and GST entries it adds itself when an
/// item is first saved: the entry defers to the item's group or the company.
const TALLY_OWN_ENTRY_SOURCE: &str = "As per Company/Stock Group";

/// A stock item's GST observation under the key Tally's element means. The
/// shared entry type serialises Tally's `GSTINELIGIBLEITC` as `itc_eligible`,
/// which is the key `ledger_masters` ships; for a stock item Tally's Yes was
/// seen where the screen's `Eligible for Input Tax Credit` read No, so it goes
/// out as `ineligible_itc` with the same value, `true`, `false` or `null` and
/// never a default. The ledger output is not changed: two keys for one Tally
/// element is deliberate until the ledger key is rectified (tracked in #1741).
///
/// On an entry whose source is Tally's own the flag is not sent on: the Yes seen there
/// was never compared with the screen (both screen readings are of rows a
/// person edited or keyed), so it goes out as `null` with
/// `ineligible_itc_withheld: tally_default_entry`.
fn stock_item_gst_json(gst: &GstDetailsObservation) -> Value {
    let mut value = json!(gst);
    if let Some(entries) = value.get_mut("entries").and_then(Value::as_array_mut) {
        for entry in entries {
            if let Some(object) = entry.as_object_mut() {
                let own =
                    object.get("source").and_then(Value::as_str) == Some(TALLY_OWN_ENTRY_SOURCE);
                if let Some(flag) = object.remove("itc_eligible") {
                    let withhold = own && !flag.is_null();
                    object.insert(
                        "ineligible_itc".to_string(),
                        if withhold { Value::Null } else { flag },
                    );
                    if withhold {
                        object.insert(
                            "ineligible_itc_withheld".to_string(),
                            json!("tally_default_entry"),
                        );
                    }
                }
            }
        }
    }
    value
}

/// Masks the description of every HSN entry: Tally fills it from its tariff
/// list, but a person can edit it, so it can carry a party's name.
fn mark_hsn_descriptions(hsn: &mut Value) {
    if let Some(entries) = hsn.get_mut("entries").and_then(Value::as_array_mut) {
        for entry in entries {
            // A null description keeps its key: only a text one is marked.
            if entry.get("description").is_some_and(Value::is_string) {
                mark_party_field(entry, "description");
            }
        }
    }
}

/// Whether `mask_parties` masks this kind's names and parents. Godown,
/// stock-group and stock-item names are text a user types, and a job-work
/// godown or a supplier-named stock group or item can carry a party's name, so
/// they go out under the party-name marker. Voucher-type and unit names are
/// Tally configuration labels, and account groups are read from the group
/// snapshot as the group tree is: none of these is marked.
const fn marks_party_text(kind: MastersKind) -> bool {
    matches!(
        kind,
        MastersKind::Native(
            NativeMasterKind::Godowns
                | NativeMasterKind::StockGroups
                | NativeMasterKind::StockItems
                | NativeMasterKind::CostCentres
                | NativeMasterKind::CostCategories
        )
    )
}

/// One native row as the tool returns it: the fields every kind has, then its
/// kind's own. `parent` is null where it does not apply or is absent. With
/// `mark_party_text`, `name` and `parent` carry the party-name marker; Tally's
/// reserved root as a parent is a fixed string, not user text, and stays plain.
fn native_row(row: &NativeMasterRow, mark_party_text: bool) -> Value {
    let mut json = json!({
        "name": row.name, "guid": row.guid,
        "master_id": row.master_id, "alter_id": row.alter_id,
        "parent": row.parent,
    });
    if mark_party_text {
        mark_party_field(&mut json, "name");
        // A null parent keeps its key: only a text parent that is not the reserved root is user text.
        if row
            .parent
            .as_deref()
            .is_some_and(|parent| !bridge_tally_protocol::is_tally_reserved_root(parent))
        {
            mark_party_field(&mut json, "parent");
        }
    }
    match &row.detail {
        NativeMasterDetail::Plain => {}
        NativeMasterDetail::VoucherType {
            active,
            optional,
            numbering,
        } => {
            json["active"] = json!(active);
            json["optional"] = json!(optional);
            json["numbering_method"] = match numbering {
                None => Value::Null,
                Some(NativeNumberingMethod::Automatic) => json!("automatic"),
                Some(NativeNumberingMethod::Manual) => json!("manual"),
                Some(NativeNumberingMethod::Default) => json!("default"),
                Some(NativeNumberingMethod::Unrecognised(raw)) => json!({"unrecognised": raw}),
            };
        }
        NativeMasterDetail::Unit {
            decimal_places,
            simple,
        } => {
            json["decimal_places"] = json!(decimal_places);
            json["simple"] = json!(simple);
        }
        NativeMasterDetail::StockItem { hsn, gst } => {
            // The key is on every row: an empty list is the item's own and says
            // nothing about its stock group or the company.
            json["hsn"] = json!(hsn);
            if mark_party_text {
                mark_hsn_descriptions(&mut json["hsn"]);
            }
            json["gst"] = stock_item_gst_json(gst);
            json["inherited_from_group_or_company"] = json!("not_read");
        }
        NativeMasterDetail::CostCentre { category } => {
            json["category"] = json!(category);
            if mark_party_text {
                mark_party_field(&mut json, "category");
            }
        }
        NativeMasterDetail::CostCategory {
            allocates_revenue,
            allocates_non_revenue,
            affects_stock,
        } => {
            json["allocates_revenue"] = json!(allocates_revenue);
            json["allocates_non_revenue"] = json!(allocates_non_revenue);
            json["affects_stock"] = json!(affects_stock);
        }
    }
    json
}

#[cfg(test)]
#[path = "agent_masters_tests.rs"]
mod tests;
