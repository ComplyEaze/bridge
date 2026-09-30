//! Thin MCP presentation of the shared masters read.
use super::trial_balance::page_boundary;
use super::*;
use crate::tally::runtime::{MastersKind, MastersRows};
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
                "Under mask_parties, godown and stock-group names and parents are masked, because a job-work godown or a supplier-named stock group can carry a party's name; Tally's reserved root as a parent is left as it is"
            } else {
                "Voucher-type, unit and account-group names are not masked by mask_parties: they are configuration labels, not counterparties (godown and stock-group names are masked)"
            },
        ];
        match kind {
            MastersKind::Native(NativeMasterKind::VoucherTypes) => limitations.extend([
                "numbering_method is as Tally reported it; a value other than Automatic, Manual or Default is returned raw under `unrecognised` (only those three seen, on one synthetic book)",
                "`default` is Tally's reported value, not evidence that the type numbers automatically",
                "The company's voucher-type count (NUMVOUCHERTYPES) did not equal the rows returned on two books (35 vs 26, 33 vs 24), and on one book equalled the number-series count (inferred to count series, unmeasured): do not check these rows against it",
                "The completeness of this list is unverified: absence from it is not evidence that a voucher type is absent from the book",
                "Voucher types are read whole with no size check before the read; rows, AlterIDs and response size are checked after it",
            ]),
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

/// Whether `mask_parties` masks this kind's names and parents. Godown and
/// stock-group names are free text a user types, and a job-work godown or a
/// supplier-named stock group can carry a party's name, so they go out under
/// the party-name marker. Voucher-type and unit names are Tally configuration
/// labels, and account groups are read from the group snapshot as the group
/// tree is: none of these is marked.
const fn marks_party_text(kind: MastersKind) -> bool {
    matches!(
        kind,
        MastersKind::Native(NativeMasterKind::Godowns | NativeMasterKind::StockGroups)
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
        if !row
            .parent
            .as_deref()
            .is_some_and(bridge_tally_protocol::is_tally_reserved_root)
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
    }
    json
}

#[cfg(test)]
#[path = "agent_masters_tests.rs"]
mod tests;
