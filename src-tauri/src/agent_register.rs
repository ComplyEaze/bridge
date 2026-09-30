//! Purchase register for the local MCP adapter (#969).
//!
//! Lists the vouchers that touch a ledger under Duties & Taxes and says, per entry, what the
//! books record: tax only on a ledger whose master carries a recognised GST duty head, never
//! from a name and never from arithmetic. What the books do not say is listed as such.
//!
//! This file holds the classifier, which has no I/O: it takes the ledger masters as read and
//! the voucher rows as parsed, and returns rows. The reads and their snapshot binding are in
//! `Server::purchase_register`.
use bridge_tally_protocol::group_ancestry::{AncestryChain, AncestryGap, GroupIndex};
use bridge_tally_protocol::{GstDutyHeadObservation, PartyLedgerMasterRecord};
use std::collections::{BTreeMap, BTreeSet};

use super::*;

const DUTIES_AND_TAXES: &str = "Duties & Taxes";
const PURCHASE_ACCOUNTS: &str = "Purchase Accounts";

/// Which predefined group a ledger sits under, from its group chain and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LedgerGroup {
    DutiesAndTaxes,
    PurchaseAccounts,
    Other,
    /// The chain stopped before any predefined group: the ledger cannot be placed.
    Unresolved(AncestryGap),
}

/// What the classifier keeps of one ledger master. Everything here is an input to a row's
/// classification, so two listings that differ in any of it classify differently.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RegisterLedger {
    group: LedgerGroup,
    tax_type: Option<String>,
    head: GstDutyHeadObservation,
}

/// The ledger masters of one company as the register reads them. Equality is the drift
/// check: a listing read after the vouchers must equal the one read before.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MasterIndex {
    by_name: BTreeMap<String, RegisterLedger>,
    /// Ledgers the compliance read set aside by currency (foreign, or a base-currency
    /// ledger whose balance is a composite). A voucher that touches one cannot be classified.
    excluded: BTreeSet<String>,
}

impl MasterIndex {
    /// A name that repeats cannot be classified (Tally keeps ledger names unique per
    /// company), so it refuses as drift rather than picking one.
    pub(super) fn build<'a>(
        records: impl IntoIterator<Item = &'a PartyLedgerMasterRecord>,
        groups: &GroupIndex,
        excluded: impl IntoIterator<Item = String>,
    ) -> Result<Self, String> {
        let mut by_name = BTreeMap::new();
        for record in records {
            let parent = record.ledger.parent.returned_text().map(str::to_string);
            let chain = groups.ancestry_chain(parent.as_deref());
            let ledger = RegisterLedger {
                group: ledger_group(&chain),
                tax_type: record.fields.tax_type.returned_text().map(str::to_string),
                head: record.fields.gst_duty_head.clone(),
            };
            if by_name.insert(record.ledger.name.clone(), ledger).is_some() {
                return Err("ledger_snapshot_drifted".to_string());
            }
        }
        Ok(Self {
            by_name,
            excluded: excluded.into_iter().collect(),
        })
    }

    pub(super) fn len(&self) -> usize {
        self.by_name.len()
    }
}

/// The nearest predefined group in the chain, by its `RESERVEDNAME`, which a rename of the
/// book's own group does not change and which the walk reaches through user sub-groups.
fn ledger_group(chain: &AncestryChain) -> LedgerGroup {
    match chain
        .hops
        .iter()
        .map(|hop| hop.reserved_name.trim())
        .find(|reserved| !reserved.is_empty())
    {
        Some(DUTIES_AND_TAXES) => LedgerGroup::DutiesAndTaxes,
        Some(PURCHASE_ACCOUNTS) => LedgerGroup::PurchaseAccounts,
        Some(_) => LedgerGroup::Other,
        None => match chain.gap {
            Some(gap) => LedgerGroup::Unresolved(gap),
            None => LedgerGroup::Other,
        },
    }
}

fn gap_code(gap: AncestryGap) -> &'static str {
    match gap {
        AncestryGap::NoParent => "no_parent",
        AncestryGap::ReachedRoot => "reached_root",
        AncestryGap::GroupAbsent => "group_absent",
        AncestryGap::GroupNameRepeated => "group_name_repeated",
        AncestryGap::ReservedNameMissing => "reserved_name_missing",
        AncestryGap::Cycle => "cycle",
        AncestryGap::Exhausted => "exhausted",
    }
}

/// The classifier's output for one window: the register, the other voucher types that touch
/// Duties & Taxes (counted and listed, never dropped), and what has no class.
#[derive(Debug, PartialEq)]
pub(super) struct RegisterPage {
    pub(super) rows: Vec<Value>,
    pub(super) other_voucher_types: Vec<Value>,
    pub(super) unclassified_voucher_type: Vec<Value>,
    /// Vouchers that touch a ledger whose group cannot be resolved and no Duties & Taxes
    /// ledger: they may have nothing to do with tax, so they are named, not classified.
    pub(super) vouchers_with_unplaced_ledgers: Vec<Value>,
    /// Purchase and Debit Note vouchers with no entry on a Duties & Taxes ledger (exempt or
    /// unregistered purchases, or tax booked to a ledger filed elsewhere): counted, not dropped.
    pub(super) register_class_without_duties_taxes_entry: Vec<Value>,
    pub(super) vouchers_observed: usize,
}

/// Purchase and Debit Note are the register. Any other class that touches Duties & Taxes is
/// listed apart; whether it belongs in a return is the CA's call, not the tool's.
fn is_register_class(class: &str) -> bool {
    matches!(class, "Purchase" | "Debit Note")
}

/// Classify the vouchers of one window against the ledger masters.
///
/// A voucher entry that names a ledger the masters do not list refuses (the masters moved
/// between the two reads, or the window belongs to another book); it is never put in a
/// catch-all. `rows` are the parsed rows of the class-entry voucher shape, in window order.
pub(super) fn classify_register(
    index: &MasterIndex,
    rows: &[Value],
) -> Result<RegisterPage, String> {
    let mut page = RegisterPage {
        rows: Vec::new(),
        other_voucher_types: Vec::new(),
        unclassified_voucher_type: Vec::new(),
        vouchers_with_unplaced_ledgers: Vec::new(),
        register_class_without_duties_taxes_entry: Vec::new(),
        vouchers_observed: rows.len(),
    };
    for row in rows {
        let entries = row
            .get("amounts")
            .and_then(Value::as_array)
            .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
        let party = row.get("party").and_then(Value::as_str);
        let mut tax = Vec::new();
        let mut without_head = Vec::new();
        let mut unrecognised = Vec::new();
        let mut unresolved = Vec::new();
        let mut taxable = Vec::new();
        let mut party_entries = Vec::new();
        let mut other = Vec::new();
        let mut touched = Vec::new();
        let mut conflict = false;
        for entry in entries {
            let name = entry
                .get("ledger")
                .and_then(Value::as_str)
                .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
            let ledger = index.by_name.get(name).ok_or_else(|| {
                let code = if index.excluded.contains(name) {
                    "register_ledger_currency_excluded"
                } else {
                    "ledger_snapshot_drifted"
                };
                code.to_string()
            })?;
            match ledger.group {
                LedgerGroup::DutiesAndTaxes => {
                    touched.push(json!(name));
                    match &ledger.head {
                        GstDutyHeadObservation::Recognized { raw, head } => {
                            let mut item = entry.clone();
                            item["head"] = serde_json::to_value(head).unwrap_or(Value::Null);
                            item["raw_head"] = json!(raw);
                            tax.push(item);
                        }
                        GstDutyHeadObservation::NotTaxLedger { tax_type } => {
                            let mut item = entry.clone();
                            item["observation"] = json!("not_tax_ledger");
                            item["tax_type"] = json!(tax_type);
                            without_head.push(item);
                        }
                        GstDutyHeadObservation::Absent => {
                            // No head, and the ledger's own TAXTYPE is GST or was not
                            // reported: unlike not_tax_ledger, this may be a GST ledger
                            // whose head is missing.
                            let mut item = entry.clone();
                            item["observation"] = json!("absent");
                            item["tax_type"] = json!(ledger.tax_type);
                            without_head.push(item);
                        }
                        GstDutyHeadObservation::Unrecognized { raw } => {
                            let mut item = entry.clone();
                            item["observation"] = json!("unrecognized");
                            item["raw_head"] = json!(raw);
                            unrecognised.push(item);
                        }
                        GstDutyHeadObservation::Contradictory { tax_type, raw } => {
                            conflict = true;
                            let mut item = entry.clone();
                            item["observation"] = json!("contradictory");
                            item["tax_type"] = json!(tax_type);
                            item["raw_head"] = json!(raw);
                            unrecognised.push(item);
                        }
                    }
                }
                LedgerGroup::PurchaseAccounts => taxable.push(entry.clone()),
                LedgerGroup::Unresolved(gap) => {
                    let mut item = entry.clone();
                    item["ancestry_gap"] = json!(gap_code(gap));
                    unresolved.push(item);
                }
                LedgerGroup::Other => {
                    if party == Some(name) {
                        party_entries.push(entry.clone());
                    } else {
                        other.push(entry.clone());
                    }
                }
            }
        }
        let class = row.get("voucher_class").and_then(Value::as_str);
        let identity = json!({
            "date": row.get("date"),
            "voucher_number": row.get("voucher_number"),
            "voucher_type": row.get("voucher_type"),
            "voucher_class": class,
            "guid": row.get("guid"),
        });
        if touched.is_empty() && unresolved.is_empty() {
            if class.is_some_and(is_register_class) {
                page.register_class_without_duties_taxes_entry
                    .push(identity);
            }
            continue;
        }
        if !touched.is_empty() {
            match class {
                Some(class) if is_register_class(class) => {}
                Some(_) => {
                    let mut item = identity.clone();
                    item["duties_taxes_ledgers"] = Value::Array(touched);
                    page.other_voucher_types.push(item);
                    continue;
                }
                None => {
                    let mut item = identity.clone();
                    item["duties_taxes_ledgers"] = Value::Array(touched);
                    page.unclassified_voucher_type.push(item);
                    continue;
                }
            }
        }
        // A voucher that only touches a ledger that cannot be placed is not in the register
        // (it may be nothing to do with tax), but it is not dropped either: it is named.
        if touched.is_empty() {
            let mut item = identity.clone();
            item["unplaced_ledgers"] = Value::Array(unresolved);
            page.vouchers_with_unplaced_ledgers.push(item);
            continue;
        }
        let status = if conflict {
            "head_conflict"
        } else if !unrecognised.is_empty() {
            "has_unrecognised_head"
        } else if !unresolved.is_empty() {
            "has_unresolved_group"
        } else if !without_head.is_empty() {
            "has_entries_without_gst_head"
        } else if !other.is_empty() {
            "has_other_entries"
        } else {
            "complete"
        };
        let mut out = identity;
        for key in [
            "alter_id",
            "party",
            "party_gstin",
            "reference",
            "is_invoice",
            "cancelled",
            "optional",
            "post_dated",
        ] {
            if let Some(value) = row.get(key) {
                out[key] = value.clone();
            }
        }
        out["status"] = json!(status);
        out["tax_in_books"] = Value::Array(tax);
        out["duties_taxes_entries_without_gst_head"] = Value::Array(without_head);
        out["duties_taxes_entries_with_unrecognised_head"] = Value::Array(unrecognised);
        out["entries_on_ledgers_with_unresolved_group"] = Value::Array(unresolved);
        out["has_taxable_entry"] = json!(!taxable.is_empty());
        out["taxable_entries"] = Value::Array(taxable);
        out["party_entries"] = Value::Array(party_entries);
        out["other_entries"] = Value::Array(other);
        page.rows.push(out);
    }
    Ok(page)
}

/// How many items of the side lists (other voucher types, unclassified, unplaced) one
/// response carries; the totals are exact whatever the cap.
const MAX_LISTED_SIDE_ITEMS: usize = 100;

fn bounded_list(items: &[Value]) -> Value {
    json!({
        "total": items.len(),
        "listed": items.iter().take(MAX_LISTED_SIDE_ITEMS).collect::<Vec<_>>(),
        "listed_truncated": items.len() > MAX_LISTED_SIDE_ITEMS,
    })
}

/// The party names a response may need to redact: the voucher's party, and the ledger of an
/// entry that is the party's own or has no known role. Tax and purchase ledger names are the
/// book's vocabulary, not parties.
pub(super) fn mark_register_row(mut row: Value) -> Value {
    mark_party_field(&mut row, "party");
    for list in [
        "party_entries",
        "other_entries",
        "entries_on_ledgers_with_unresolved_group",
    ] {
        if let Some(entries) = row.get_mut(list).and_then(Value::as_array_mut) {
            for entry in entries {
                mark_party_field(entry, "ledger");
            }
        }
    }
    row
}

/// A side-list item names ledgers it could not place, which may be a party's.
fn mark_register_side_item(mut item: Value) -> Value {
    if let Some(entries) = item
        .get_mut("unplaced_ledgers")
        .and_then(Value::as_array_mut)
    {
        for entry in entries {
            mark_party_field(entry, "ledger");
        }
    }
    item
}

/// The window was planned against the marks the masters were read under; the marks read after
/// it must be those marks, or the book moved while the window was read. (An undivided window
/// read reports no marks of its own, so this closing read is the check.)
fn window_drift(pinned: CompanyMarks, closing: CompanyMarks) -> Option<&'static str> {
    (closing != pinned).then_some("voucher_window_changed_during_read")
}

/// The masters read after the window must be the masters read before it, in everything that
/// decides a classification, and under the same marks. A voucher mark that moved is the
/// window's problem, not the masters'.
fn masters_drifted(first: &RegisterMasters, second: &RegisterMasters) -> Option<&'static str> {
    if second.marks.vouchers != first.marks.vouchers {
        Some("voucher_window_changed_during_read")
    } else if second.index != first.index || second.marks.masters != first.marks.masters {
        Some("ledger_snapshot_drifted")
    } else {
        None
    }
}

/// What the tool returns for one window, except `state` and `reason`, which depend on whether
/// an empty window was corroborated.
pub(super) struct RegisterResult {
    pub(super) result: Value,
    pub(super) truncated: bool,
}

/// Validate the window, classify its vouchers, page the register and mark and redact every
/// party name in every list of the response.
pub(super) fn register_result(
    index: &MasterIndex,
    rows: Vec<Value>,
    (from, to): (&str, &str),
    (offset, limit): (usize, usize),
    redaction: Redaction,
) -> Result<RegisterResult, String> {
    // An undivided window read is admitted by its caller against the window: rows dated
    // outside it would otherwise be returned as register rows.
    let rows = validate_then_filter_voucher_rows(rows, from, to, None)?;
    let page = classify_register(index, &rows)?;
    let total = page.rows.len();
    let (items, truncated, next_offset) = paginate(page.rows, offset, limit);
    let items = items
        .into_iter()
        .map(|row| redact_value(mark_register_row(row), redaction))
        .collect::<Vec<_>>();
    let side = |list: &[Value]| {
        let marked = list
            .iter()
            .cloned()
            .map(mark_register_side_item)
            .collect::<Vec<_>>();
        redact_value(bounded_list(&marked), redaction)
    };
    let result = json!({
        "profile": "agent_purchase_register_v1",
        "register_classes": ["Purchase", "Debit Note"],
        "items": items,
        "offset": offset,
        "next_offset": next_offset,
        "total": total,
        "vouchers_observed": page.vouchers_observed,
        "ledger_masters_observed": index.len(),
        "other_voucher_types_touching_duties_taxes": side(&page.other_voucher_types),
        "unclassified_voucher_type": side(&page.unclassified_voucher_type),
        "vouchers_with_unplaced_ledgers": side(&page.vouchers_with_unplaced_ledgers),
        "purchase_vouchers_without_duties_taxes_entry":
            side(&page.register_class_without_duties_taxes_entry),
        "coverage": "items are the Purchase and Debit Note vouchers that touch a ledger under Duties & Taxes; tax is taken only from the GST duty head recorded on a ledger master, never from a name or an amount; every other voucher type that touches those ledgers is listed apart (whether it belongs in a return is the CA's call); Purchase and Debit Note vouchers with no entry on a Duties & Taxes ledger are counted in purchase_vouchers_without_duties_taxes_entry, not returned as items",
    });
    Ok(RegisterResult { result, truncated })
}

/// One page of the register: the rows from `offset`, at most `limit`, whether more remain and
/// where the next page starts.
fn paginate(rows: Vec<Value>, offset: usize, limit: usize) -> (Vec<Value>, bool, Option<usize>) {
    let total = rows.len();
    let page = rows
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();
    let truncated = offset.saturating_add(page.len()) < total;
    let next_offset = truncated.then_some(offset + page.len());
    (page, truncated, next_offset)
}

/// The masters as one read saw them, with the marks the read was pinned under.
struct RegisterMasters {
    index: MasterIndex,
    marks: CompanyMarks,
    evidence: Evidence,
}

impl Server {
    pub(super) async fn purchase_register(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        if from > to {
            return Err("invalid_date_range".to_string().into());
        }
        let (company, identity, mut evidence) = self.verified_company(guid).await?;
        let result: Result<ToolOutcome, ToolFailure> = async {
            // The masters first, pinned under the marks of their own read; the window is then
            // planned against those marks, and the company's marks are read once more after
            // it. An undivided window read takes opening marks only, so this closing read is
            // what shows a master or voucher that changed while the window was read.
            let first = self.read_register_masters(&identity).await?;
            evidence = combine_evidence(evidence.clone(), first.evidence.clone());
            let window = self
                .read_entry_window_shaped(
                    &identity,
                    &company.name,
                    &from,
                    &to,
                    Some(first.marks),
                    VoucherReadShape::ClassEntryWildcard,
                )
                .await?;
            evidence = combine_evidence(evidence.clone(), window.all_evidence());
            let (marks_xml, closing_evidence, _) = self
                .post_read_observing_boundary(&identity, company_high_water_read(&company.name))
                .await?;
            evidence = combine_evidence(evidence.clone(), closing_evidence);
            let (vouchers, masters) = parse_company_marks(&marks_xml, identity.company_guid())?;
            let closing = CompanyMarks { vouchers, masters };
            if let Some(code) = window_drift(first.marks, closing) {
                return Err(code.to_string().into());
            }
            // The two marks can be unchanged by an edit that does not move them (a duty head
            // or a parent changed in Tally's own screens is unmeasured), so the masters are
            // read again and must classify exactly as they did.
            let second = self.read_register_masters(&identity).await?;
            evidence = combine_evidence(evidence.clone(), second.evidence.clone());
            if let Some(code) = masters_drifted(&first, &second) {
                return Err(code.to_string().into());
            }
            let mut state = "complete";
            let mut reason = None;
            if window.rows.is_empty() {
                let (corroboration, partial, corroboration_reason) = self
                    .corroborate_empty_voucher_read(
                        &identity,
                        &company.name,
                        &from,
                        &to,
                        None,
                        Some(first.marks),
                    )
                    .await?;
                evidence = combine_evidence(evidence.clone(), corroboration);
                if partial {
                    state = "partial";
                    reason = corroboration_reason;
                    evidence.state = "partial";
                    evidence.reason_code = corroboration_reason.map(str::to_string);
                }
            }
            let offset = arg_usize(args, "offset", 0)?;
            let limit = arg_positive_usize(args, "limit", self.settings.max_rows)?
                .min(self.settings.max_rows);
            let RegisterResult {
                mut result,
                truncated,
            } = register_result(
                &first.index,
                window.rows,
                (&from, &to),
                (offset, limit),
                self.settings.redaction,
            )?;
            result["state"] = json!(state);
            result["reason"] = json!(reason);
            let payload = json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            });
            Ok(ToolOutcome {
                payload,
                evidence: evidence.clone(),
                company_guid: Some(guid.to_string()),
                truncated,
            })
        }
        .await;
        result.map_err(|failure| failure.with_prior_evidence(evidence))
    }

    /// One fresh compliance read of the ledger masters, classified, with the marks of the
    /// extent the read was pinned under.
    async fn read_register_masters(
        &self,
        identity: &VerifiedCompanyIdentity,
    ) -> Result<RegisterMasters, ToolFailure> {
        let today = bridge_tally_core::TallyDate::parse(tally_host_today())
            .map_err(|_| ToolFailure::from("current_date_invalid".to_string()))?;
        let listing = self
            .runtime
            .fetch_agent_party_ledger_masters_with_evidence(self.tally_config(), identity, today)
            .await
            .map_err(|error| ToolFailure::from_runtime("party_ledger_master_read_failed", error))?;
        let marks = CompanyMarks {
            vouchers: listing
                .extent
                .voucher_alter_id_high_water()
                .map_or(0, |mark| mark.get()),
            masters: listing
                .extent
                .master_alter_id_high_water()
                .map(|mark| mark.get())
                .ok_or_else(|| ToolFailure::from("register_master_mark_unavailable".to_string()))?,
        };
        let groups = GroupIndex::build(listing.groups);
        let excluded = listing
            .foreign_currency_ledgers_excluded
            .into_iter()
            .map(|ledger| ledger.ledger)
            .chain(listing.mixed_currency_ledgers_excluded);
        let index = MasterIndex::build(listing.records.iter(), &groups, excluded)?;
        Ok(RegisterMasters {
            index,
            marks,
            evidence: evidence_from_runtime_read(listing.evidence),
        })
    }
}

#[cfg(test)]
#[path = "agent_register_tests.rs"]
mod tests;
