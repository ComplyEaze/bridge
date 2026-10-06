//! The bill trail and the unadjusted detail for one party (#945 slice C).
//!
//! Both are read from voucher rows already read and validated, and each is
//! tied out against what Tally's own bills reports and the ledger snapshot
//! said at the same as-of date:
//!
//! - a bill's trail is every allocation of one reference on the party's
//!   ledger, oldest first; its signed sum must equal that bill's balance in
//!   the native report, or zero for a bill the report no longer lists;
//! - the unadjusted detail lists the party's on-account, advance and pending
//!   note allocations; the signed sum of the on-account ones must equal the
//!   party's unallocated residual.
//!
//! Nothing here merges or guesses. A bill whose identity cannot be matched to
//! exactly one native row is `bill_identity_ambiguous`, a sum that does not tie
//! is `trail_does_not_tie` with both numbers, and a residual the vouchers do
//! not explain is `residual_not_explained_by_vouchers` with the difference;
//! whether that difference equals the ledger's opening balance is stated as a
//! fact and never called an opening. A [`BillTrail`] exists only for a bill
//! that tied (its constructor is private).
use super::*;
use bridge_tally_core::ExactDecimal;
use std::collections::BTreeMap;

/// A refusal to build a trail: the rows are not shaped as a trail needs, and a
/// guess would put a wrong allocation in front of an accountant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TrailRefusal(pub(super) &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AllocationKind {
    NewRef,
    AgstRef,
    Advance,
    OnAccount,
}

impl AllocationKind {
    fn parse(text: &str) -> Result<Self, TrailRefusal> {
        match text.trim() {
            "New Ref" => Ok(Self::NewRef),
            "Agst Ref" => Ok(Self::AgstRef),
            "Advance" => Ok(Self::Advance),
            "On Account" => Ok(Self::OnAccount),
            _ => Err(TrailRefusal("trail_allocation_type_unknown")),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::NewRef => "New Ref",
            Self::AgstRef => "Agst Ref",
            Self::Advance => "Advance",
            Self::OnAccount => "On Account",
        }
    }
}

/// One allocation of the party's ledger, with the voucher that carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TrailEntry {
    pub(super) date: String,
    pub(super) voucher_type: String,
    pub(super) voucher_number: Option<String>,
    pub(super) guid: String,
    pub(super) kind: AllocationKind,
    /// `None` for On Account.
    pub(super) reference: Option<String>,
    /// The bill's own date: the voucher's on a New Ref or Advance, the
    /// original bill's on an Agst Ref. Absent only on On Account.
    pub(super) bill_date: Option<String>,
    /// Signed as Tally writes it: a debit is negative.
    pub(super) amount: ExactDecimal,
}

impl TrailEntry {
    fn json(&self) -> Value {
        json!({
            "date": self.date,
            "voucher_type": self.voucher_type,
            "voucher_number": self.voucher_number,
            "guid": self.guid,
            "allocation_type": self.kind.label(),
            "amount": self.amount.as_str(),
            "bill_date": self.bill_date,
        })
    }
}

/// The party's allocations in `rows`, in voucher order. Cancelled and optional
/// vouchers are left out, as Tally's own reports leave them out. Only entries
/// on `party`'s ledger count: a voucher's own party field is not consulted, so
/// an allocation on a second party's ledger in the same voucher is that
/// party's.
pub(super) fn entries_for_party(
    rows: &[Value],
    party: &str,
) -> Result<Vec<TrailEntry>, TrailRefusal> {
    let mut entries = Vec::new();
    for row in rows {
        if row.get("cancelled").and_then(Value::as_bool) == Some(true)
            || row.get("optional").and_then(Value::as_bool) == Some(true)
        {
            continue;
        }
        let date = row
            .get("date")
            .and_then(Value::as_str)
            .ok_or(TrailRefusal("trail_voucher_malformed"))?;
        let voucher_type = row
            .get("voucher_type")
            .and_then(Value::as_str)
            .ok_or(TrailRefusal("trail_voucher_malformed"))?;
        let guid = row
            .get("guid")
            .and_then(Value::as_str)
            .ok_or(TrailRefusal("trail_voucher_malformed"))?;
        let voucher_number = row
            .get("voucher_number")
            .and_then(Value::as_str)
            .map(str::to_string);
        let amounts = row
            .get("amounts")
            .and_then(Value::as_array)
            .ok_or(TrailRefusal("trail_voucher_malformed"))?;
        for amount in amounts {
            if amount.get("ledger").and_then(Value::as_str) != Some(party) {
                continue;
            }
            let allocations = amount
                .get("bill_allocations")
                .and_then(Value::as_array)
                .ok_or(TrailRefusal("trail_voucher_malformed"))?;
            for allocation in allocations {
                let kind = AllocationKind::parse(
                    allocation
                        .get("bill_type")
                        .and_then(Value::as_str)
                        .ok_or(TrailRefusal("trail_voucher_malformed"))?,
                )?;
                let value = ExactDecimal::parse(
                    allocation
                        .get("amount")
                        .and_then(Value::as_str)
                        .ok_or(TrailRefusal("trail_voucher_malformed"))?,
                )
                .map_err(|_| TrailRefusal("trail_amount_invalid"))?;
                let reference = match (kind, allocation.pointer("/reference/name")) {
                    (AllocationKind::OnAccount, _) => None,
                    (_, Some(Value::String(name))) if !name.trim().is_empty() => Some(name.clone()),
                    _ => return Err(TrailRefusal("trail_reference_missing")),
                };
                let bill_date = allocation
                    .get("bill_date")
                    .and_then(Value::as_str)
                    .map(str::to_string);
                if kind != AllocationKind::OnAccount && bill_date.is_none() {
                    return Err(TrailRefusal("trail_bill_date_missing"));
                }
                entries.push(TrailEntry {
                    date: date.to_string(),
                    voucher_type: voucher_type.to_string(),
                    voucher_number: voucher_number.clone(),
                    guid: guid.to_string(),
                    kind,
                    reference,
                    bill_date,
                    amount: value,
                });
            }
        }
    }
    // Oldest first, as the output promises; the sort is stable, so vouchers of
    // one day keep the order the window read returned them in.
    entries.sort_by(|left, right| left.date.cmp(&right.date));
    Ok(entries)
}

/// Tally's signed balance for an open bill: the native row carries a magnitude
/// and the report (direction) it came from, and a debit balance is negative.
fn signed_native(bill: &OpenBillRow) -> Result<ExactDecimal, TrailRefusal> {
    match bill.kind {
        ExposureDirection::Receivable => ExactDecimal::zero()
            .checked_subtract(&bill.amount)
            .map_err(|_| TrailRefusal("trail_amount_invalid")),
        ExposureDirection::Payable => Ok(bill.amount.clone()),
    }
}

fn sum(entries: &[&TrailEntry]) -> Result<ExactDecimal, TrailRefusal> {
    entries
        .iter()
        .try_fold(ExactDecimal::zero(), |total, entry| {
            total
                .checked_add(&entry.amount)
                .map_err(|_| TrailRefusal("trail_amount_invalid"))
        })
}

/// A bill whose trail tied out. It cannot be built any other way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BillTrail {
    reference: String,
    bill_date: String,
    entries: Vec<TrailEntry>,
    balance: ExactDecimal,
    /// `true` when Tally's own report lists the bill (open); `false` when the
    /// trail sums to zero and the report no longer lists it.
    listed_open: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BillOutcome {
    Tied(BillTrail),
    DoesNotTie {
        reference: String,
        bill_date: String,
        entries: Vec<TrailEntry>,
        trail_sum: ExactDecimal,
        native: Option<ExactDecimal>,
    },
    Ambiguous {
        reference: String,
        bill_dates_in_vouchers: Vec<String>,
        /// The bill date of each native row Tally lists for the reference,
        /// so a one-row mismatch shows both dates.
        native_bill_dates: Vec<String>,
        native_rows: usize,
        entries: Vec<TrailEntry>,
    },
}

impl BillOutcome {
    #[cfg(test)]
    pub(super) fn state(&self) -> &'static str {
        match self {
            Self::Tied(_) => "tied",
            Self::DoesNotTie { .. } => "trail_does_not_tie",
            Self::Ambiguous { .. } => "bill_identity_ambiguous",
        }
    }

    fn reference(&self) -> &str {
        match self {
            Self::Tied(trail) => &trail.reference,
            Self::DoesNotTie { reference, .. } | Self::Ambiguous { reference, .. } => reference,
        }
    }

    pub(super) fn json(&self, party: Value) -> Value {
        match self {
            Self::Tied(trail) => json!({
                "party": party,
                "reference": trail.reference,
                "bill_date": trail.bill_date,
                "state": "tied",
                "open": trail.listed_open,
                "balance": trail.balance.as_str(),
                "allocations": trail.entries.iter().map(TrailEntry::json).collect::<Vec<_>>(),
            }),
            Self::DoesNotTie {
                reference,
                bill_date,
                entries,
                trail_sum,
                native,
            } => json!({
                "party": party,
                "reference": reference,
                "bill_date": bill_date,
                "state": "trail_does_not_tie",
                "trail_sum": trail_sum.as_str(),
                "native_balance": native.as_ref().map(ExactDecimal::as_str),
                "allocations": entries.iter().map(TrailEntry::json).collect::<Vec<_>>(),
            }),
            Self::Ambiguous {
                reference,
                bill_dates_in_vouchers,
                native_bill_dates,
                native_rows,
                entries,
            } => json!({
                "party": party,
                "reference": reference,
                "state": "bill_identity_ambiguous",
                "bill_dates_in_vouchers": bill_dates_in_vouchers,
                "native_bill_dates": native_bill_dates,
                "native_rows": native_rows,
                "allocations": entries.iter().map(TrailEntry::json).collect::<Vec<_>>(),
            }),
        }
    }
}

/// The trail of every named bill of `party` in `entries` (or of one
/// `reference`), each tied to `native_bills` (the party's rows of Tally's own
/// bills reports at the same as-of). Bills come back ordered by reference.
pub(super) fn bill_trails(
    party: &str,
    reference: Option<&str>,
    entries: &[TrailEntry],
    native_bills: &[OpenBillRow],
) -> Result<Vec<BillOutcome>, TrailRefusal> {
    let mut by_reference = BTreeMap::<&str, Vec<&TrailEntry>>::new();
    for entry in entries {
        if let Some(name) = entry.reference.as_deref() {
            if reference.is_none_or(|wanted| wanted == name) {
                by_reference.entry(name).or_default().push(entry);
            }
        }
    }
    // A reference asked for that no voucher in the window carries, but the
    // native report lists, is still a bill: its trail is empty and cannot tie.
    if let Some(wanted) = reference {
        by_reference.entry(wanted).or_default();
    }
    // The party's own rows of Tally's report, picked out once.
    let party_natives = native_bills
        .iter()
        .filter(|bill| bill.party == party)
        .collect::<Vec<_>>();
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::<String>::new();
    for (name, group) in by_reference {
        seen.insert(name.to_string());
        let natives = party_natives
            .iter()
            .copied()
            .filter(|bill| bill.reference == name)
            .collect::<Vec<_>>();
        // Every named allocation carries its bill date (`entries_for_party`
        // refuses one that does not); one without is refused here too rather
        // than given an empty date or left out of the comparison.
        let mut bill_dates = group
            .iter()
            .map(|entry| {
                entry
                    .bill_date
                    .clone()
                    .ok_or(TrailRefusal("trail_bill_date_missing"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        bill_dates.sort();
        bill_dates.dedup();
        let owned = group
            .iter()
            .map(|entry| (*entry).clone())
            .collect::<Vec<_>>();
        if group.is_empty() && natives.is_empty() {
            // Nothing to show and nothing to tie: the reference is unknown here.
            continue;
        }
        if bill_dates.len() > 1 || natives.len() > 1 {
            out.push(BillOutcome::Ambiguous {
                reference: name.to_string(),
                bill_dates_in_vouchers: bill_dates,
                native_bill_dates: natives.iter().map(|n| n.bill_date.clone()).collect(),
                native_rows: natives.len(),
                entries: owned,
            });
            continue;
        }
        let trail_sum = sum(&group)?;
        // `None` only for an empty group: every allocation in a group was
        // checked above to carry its date.
        let bill_date = bill_dates.first().cloned();
        match natives.first() {
            Some(native) => {
                if bill_date
                    .as_deref()
                    .is_some_and(|date| date != native.bill_date)
                {
                    // One native row, but it is not this bill: its date differs
                    // from the allocations' own bill date.
                    out.push(BillOutcome::Ambiguous {
                        reference: name.to_string(),
                        bill_dates_in_vouchers: bill_dates,
                        native_bill_dates: vec![native.bill_date.clone()],
                        native_rows: 1,
                        entries: owned,
                    });
                    continue;
                }
                let signed = signed_native(native)?;
                if signed.numeric_eq(&trail_sum) && !group.is_empty() {
                    out.push(BillOutcome::Tied(BillTrail {
                        reference: name.to_string(),
                        bill_date: native.bill_date.clone(),
                        entries: owned,
                        balance: trail_sum,
                        listed_open: true,
                    }));
                } else {
                    out.push(BillOutcome::DoesNotTie {
                        reference: name.to_string(),
                        bill_date: native.bill_date.clone(),
                        entries: owned,
                        trail_sum,
                        native: Some(signed),
                    });
                }
            }
            None => {
                // No native row: the group is not empty (an empty group with
                // no native row was skipped above), so it has its date.
                let bill_date = bill_date.ok_or(TrailRefusal("trail_bill_date_missing"))?;
                if trail_sum.is_zero() {
                    out.push(BillOutcome::Tied(BillTrail {
                        reference: name.to_string(),
                        bill_date,
                        entries: owned,
                        balance: trail_sum,
                        listed_open: false,
                    }));
                } else {
                    out.push(BillOutcome::DoesNotTie {
                        reference: name.to_string(),
                        bill_date,
                        entries: owned,
                        trail_sum,
                        native: None,
                    });
                }
            }
        }
    }
    // A bill Tally's report lists for the party that no voucher in the window
    // allocates to (an opening bill, which lives on the ledger, or a window that
    // starts after the bill) still appears: with no allocations it cannot tie.
    // Several rows for one reference are ambiguous here exactly as they are
    // when vouchers carry the reference, never one shown and the rest dropped.
    let mut native_only = BTreeMap::<&str, Vec<&OpenBillRow>>::new();
    for native in party_natives.iter().copied().filter(|bill| {
        !seen.contains(&bill.reference) && reference.is_none_or(|wanted| wanted == bill.reference)
    }) {
        native_only
            .entry(native.reference.as_str())
            .or_default()
            .push(native);
    }
    for (name, natives) in native_only {
        match natives.as_slice() {
            [native] => out.push(BillOutcome::DoesNotTie {
                reference: name.to_string(),
                bill_date: native.bill_date.clone(),
                entries: Vec::new(),
                trail_sum: ExactDecimal::zero(),
                native: Some(signed_native(native)?),
            }),
            _ => out.push(BillOutcome::Ambiguous {
                reference: name.to_string(),
                bill_dates_in_vouchers: Vec::new(),
                native_bill_dates: natives.iter().map(|n| n.bill_date.clone()).collect(),
                native_rows: natives.len(),
                entries: Vec::new(),
            }),
        }
    }
    out.sort_by(|left, right| left.reference().cmp(right.reference()));
    if reference.is_some() && out.is_empty() {
        // Neither a voucher in the window nor Tally's report knows it for this
        // party: an empty list would read as "a party with no bills".
        return Err(TrailRefusal("bill_reference_not_found"));
    }
    Ok(out)
}

/// How the party's on-account sum compares with its unallocated residual. A
/// residual exists only in the states where Tally's ledger snapshot listed a
/// row for the party, so an absent row can never read as a zero one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum UnadjustedTie {
    /// The ledger snapshot lists no unallocated amount for this ledger. A
    /// party whose residual is zero (zero residuals are not listed), a ledger
    /// that is not a party's and a name that matched no row are not told
    /// apart, so nothing is tied and no residual is shown.
    NoResidualRowForParty,
    /// The ledger keeps no bills: its vouchers carry no allocations to list.
    NotBillWiseLedger { residual: ExactDecimal },
    /// The on-account sum equals the residual. Two equal figures, not a proven
    /// composition: components that net to zero are not seen.
    Tied { residual: ExactDecimal },
    ResidualNotExplainedByVouchers {
        residual: ExactDecimal,
        /// `residual - on_account_sum`, never zero.
        difference: ExactDecimal,
        /// Whether that difference equals the ledger's own opening balance. A
        /// fact about two numbers, not a claim about what the difference is;
        /// `None` when Tally sent no opening.
        equals_opening_balance: Option<bool>,
    },
}

impl UnadjustedTie {
    fn state(&self) -> &'static str {
        match self {
            Self::NoResidualRowForParty => "no_residual_row_for_party",
            Self::NotBillWiseLedger { .. } => "not_bill_wise_ledger",
            Self::Tied { .. } => "tied",
            Self::ResidualNotExplainedByVouchers { .. } => "residual_not_explained_by_vouchers",
        }
    }

    fn residual(&self) -> Option<&ExactDecimal> {
        match self {
            Self::NoResidualRowForParty => None,
            Self::NotBillWiseLedger { residual }
            | Self::Tied { residual }
            | Self::ResidualNotExplainedByVouchers { residual, .. } => Some(residual),
        }
    }
}

/// The unadjusted detail of one party, with its tie-out.
///
/// Every row's `amount` is the allocation as it was made, never net of what
/// later allocations adjusted against its reference: no captured book shows how
/// an advance is later adjusted, so no netting is built on it. What is still
/// open on an advance or a note's reference is Tally's own figure, from its
/// bills reports at the same as-of, carried beside the row.
pub(super) struct UnadjustedDetail {
    pub(super) tie: UnadjustedTie,
    pub(super) on_account_sum: ExactDecimal,
    pub(super) rows: Vec<(&'static str, TrailEntry)>,
    /// The party's signed native balances by reference, for the advance and
    /// note rows: one per row Tally lists, so several mean an ambiguous bill.
    pub(super) native_by_reference: BTreeMap<String, Vec<ExactDecimal>>,
}

impl UnadjustedDetail {
    pub(super) fn state(&self) -> &'static str {
        self.tie.state()
    }

    pub(super) fn json(&self) -> Value {
        let mut value = json!({
            "state": self.state(),
            "residual": self.tie.residual().map(ExactDecimal::as_str),
            "on_account_sum": self.on_account_sum.as_str(),
            "row_amounts": "as_allocated",
            "rows": self.rows.iter().map(|(class, entry)| {
                let mut row = entry.json();
                row["class"] = json!(class);
                if let Some(reference) = &entry.reference {
                    row["reference"] = json!(reference);
                    // Tally's own open balance for the reference, signed as the
                    // allocations are: `null` when its bills reports do not list
                    // it at the as-of, or list it more than once.
                    let natives = self.native_by_reference.get(reference);
                    row["native_rows"] = json!(natives.map_or(0, Vec::len));
                    row["native_balance"] = json!(match natives.map(Vec::as_slice) {
                        Some([one]) => Some(one.as_str()),
                        _ => None,
                    });
                }
                row
            }).collect::<Vec<_>>(),
        });
        if let UnadjustedTie::ResidualNotExplainedByVouchers {
            difference,
            equals_opening_balance,
            ..
        } = &self.tie
        {
            value["difference"] = json!(difference.as_str());
            if let Some(equals) = equals_opening_balance {
                value["difference_equals_opening_balance"] = json!(equals);
            }
        }
        value
    }
}

/// On-account, advance and pending-note rows of `party`, tied against its
/// unallocated residual when Tally's ledger snapshot lists one. A party on a
/// ledger that keeps no bills returns state `not_bill_wise_ledger` and no rows:
/// its vouchers carry no allocations to list.
pub(super) fn unadjusted_detail(
    entries: &[TrailEntry],
    native_bills: &[OpenBillRow],
    party: &str,
    unallocated: Option<&UnallocatedParty>,
) -> Result<UnadjustedDetail, TrailRefusal> {
    let residual = unallocated
        .map(|row| match row.direction {
            ExposureDirection::Receivable => ExactDecimal::zero()
                .checked_subtract(&row.amount)
                .map_err(|_| TrailRefusal("trail_amount_invalid")),
            ExposureDirection::Payable => Ok(row.amount.clone()),
        })
        .transpose()?;
    if let (Some(residual), Some(crate::tally::UnallocatedComposition::NotBillWiseLedger)) =
        (&residual, unallocated.and_then(|row| row.composition))
    {
        return Ok(UnadjustedDetail {
            tie: UnadjustedTie::NotBillWiseLedger {
                residual: residual.clone(),
            },
            on_account_sum: ExactDecimal::zero(),
            rows: Vec::new(),
            native_by_reference: BTreeMap::new(),
        });
    }
    let mut party_natives = BTreeMap::<String, Vec<ExactDecimal>>::new();
    for bill in native_bills.iter().filter(|bill| bill.party == party) {
        party_natives
            .entry(bill.reference.clone())
            .or_default()
            .push(signed_native(bill)?);
    }
    let mut rows = Vec::new();
    let mut on_account = Vec::<&TrailEntry>::new();
    for entry in entries {
        match entry.kind {
            AllocationKind::OnAccount => {
                rows.push(("on_account", entry.clone()));
                on_account.push(entry);
            }
            AllocationKind::Advance => rows.push(("advance", entry.clone())),
            AllocationKind::NewRef
                if matches!(entry.voucher_type.as_str(), "Credit Note" | "Debit Note")
                    && entry
                        .reference
                        .as_deref()
                        .is_some_and(|reference| party_natives.contains_key(reference)) =>
            {
                rows.push(("pending_note_with_reference", entry.clone()));
            }
            _ => {}
        }
    }
    let on_account_sum = sum(&on_account)?;
    let tie = match residual {
        None => UnadjustedTie::NoResidualRowForParty,
        Some(residual) => {
            let difference = residual
                .checked_subtract(&on_account_sum)
                .map_err(|_| TrailRefusal("trail_amount_invalid"))?;
            if difference.is_zero() {
                UnadjustedTie::Tied { residual }
            } else {
                let equals_opening_balance = unallocated
                    .and_then(|row| row.opening_balance.as_ref())
                    .map(|opening| opening.numeric_eq(&difference));
                UnadjustedTie::ResidualNotExplainedByVouchers {
                    residual,
                    difference,
                    equals_opening_balance,
                }
            }
        }
    };
    // Kept only for the references the rows name.
    party_natives.retain(|reference, _| {
        rows.iter()
            .any(|(_, entry)| entry.reference.as_deref() == Some(reference.as_str()))
    });
    Ok(UnadjustedDetail {
        tie,
        on_account_sum,
        rows,
        native_by_reference: party_natives,
    })
}

/// Why a bill-trail answer lists what it lists. An empty list is never left
/// to read as a measured "this party has no bills".
pub(super) fn bill_trail_state(
    bills: &[BillOutcome],
    unallocated: Option<&UnallocatedParty>,
) -> &'static str {
    if !bills.is_empty() {
        "bills_listed"
    } else if unallocated.and_then(|row| row.composition)
        == Some(crate::tally::UnallocatedComposition::NotBillWiseLedger)
    {
        // The ledger snapshot says this ledger keeps no bills.
        "not_bill_wise_ledger"
    } else {
        // No voucher read allocates a named bill on this ledger and Tally's
        // bills reports list none for it: a ledger that keeps no bills, one
        // that is not a party's and a party with no bills are not told apart.
        "no_named_bill_for_party"
    }
}

/// The most allocation rows one detail answer carries. Past it the read is
/// refused instead of cut, so a detail is never a silently partial one: a bill
/// trail as `trail_too_large` (narrow it with `reference`), the unadjusted
/// detail as `unadjusted_detail_too_large` (nothing narrows it).
const MAX_DETAIL_ROWS: usize = 500;

/// Allocations a bill-trail answer carries, whatever each bill's state.
pub(super) fn trail_row_count(bills: &[BillOutcome]) -> usize {
    bills
        .iter()
        .map(|bill| match bill {
            BillOutcome::Tied(trail) => trail.entries.len(),
            BillOutcome::DoesNotTie { entries, .. } | BillOutcome::Ambiguous { entries, .. } => {
                entries.len()
            }
        })
        .sum()
}

/// Refuse, never cut, an answer of more than `cap` ([`MAX_DETAIL_ROWS`]) rows, with the
/// kind's own code: `reference` narrows a bill trail and nothing narrows the
/// unadjusted detail, so the two refusals carry different advice.
pub(super) fn within_detail_cap(
    kind: DetailKind,
    rows: usize,
    cap: usize,
) -> Result<(), TrailRefusal> {
    if rows > cap {
        return Err(TrailRefusal(match kind {
            DetailKind::BillTrail => "trail_too_large",
            DetailKind::Unadjusted => "unadjusted_detail_too_large",
        }));
    }
    Ok(())
}

/// Parse the three optional arguments of `outstandings` that ask for a party
/// detail, before any read: the codes are the tool's refusals.
pub(super) fn parse_detail_request(
    party: Option<&str>,
    detail: Option<&str>,
    reference: Option<&str>,
) -> Result<Option<DetailKind>, &'static str> {
    let kind = match (party, detail) {
        (None, None) => None,
        (Some(_), Some("bill_trail")) => Some(DetailKind::BillTrail),
        (Some(_), Some("unadjusted")) => Some(DetailKind::Unadjusted),
        (Some(_), None) => return Err("party_requires_detail"),
        (None, Some(_)) => return Err("detail_requires_party"),
        (Some(_), Some(_)) => return Err("invalid_detail"),
    };
    if reference.is_some() && kind != Some(DetailKind::BillTrail) {
        return Err("reference_requires_bill_trail");
    }
    Ok(kind)
}

/// Where the party's voucher window starts: the books' beginning, or, for a
/// named bill, the earliest bill date Tally's report lists for that reference
/// (the earliest, so that two rows of one reference never lose the older one's
/// allocations).
pub(super) fn trail_window_start(
    kind: DetailKind,
    reference: Option<&str>,
    party: &str,
    open_bills: &[OpenBillRow],
    books_from: &str,
) -> String {
    match (kind, reference) {
        // An opening bill can carry a date before the books begin (reference
        // 12a.10); a window before the books is not one Bridge reads.
        (DetailKind::BillTrail, Some(reference)) => open_bills
            .iter()
            .filter(|bill| bill.party == party && bill.reference == reference)
            .map(|bill| bill.bill_date.clone())
            .min()
            .filter(|date| date.as_str() > books_from)
            .unwrap_or_else(|| books_from.to_string()),
        _ => books_from.to_string(),
    }
}

/// The limits one party detail is read and answered under.
#[derive(Clone, Copy, Debug)]
pub(super) struct DetailLimits {
    /// The most rows the answer carries ([`MAX_DETAIL_ROWS`] in production).
    pub(super) rows: usize,
    /// The window read's own limits (the planner's in production).
    pub(super) window: WindowReadLimits,
}

/// The window read refused as needing more data requests than one call may
/// spend (`voucher_window_too_many_reads`), told as the detail's own refusal:
/// each call has a different next step, so each has its own code, with the
/// window's code as the cause and its planned size kept beside it. A bill
/// trail can still be narrowed to one bill; one already narrowed, and the
/// unadjusted detail, cannot.
pub(super) fn window_too_large(
    mut failure: ToolFailure,
    kind: DetailKind,
    reference: Option<&str>,
) -> ToolFailure {
    const TOO_MANY_READS: &str = "voucher_window_too_many_reads";
    if failure.code == TOO_MANY_READS {
        failure.code = match (kind, reference) {
            (DetailKind::BillTrail, None) => "trail_window_too_large",
            (DetailKind::BillTrail, Some(_)) => "named_bill_window_too_large",
            (DetailKind::Unadjusted, _) => "unadjusted_window_too_large",
        }
        .to_string();
        failure.cause = Some(TOO_MANY_READS);
    }
    failure
}

/// What `outstandings` was asked for beyond the book-wide figures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DetailKind {
    BillTrail,
    Unadjusted,
}

impl DetailKind {
    fn label(self) -> &'static str {
        match self {
            Self::BillTrail => "bill_trail",
            Self::Unadjusted => "unadjusted",
        }
    }
}

impl Server {
    /// Reads the party's vouchers for the window the detail needs and builds it.
    ///
    /// `open_bills` and `unallocated` are the book-wide native figures already
    /// read at `as_of`, so the vouchers and the report are tied at one date. The
    /// window is the party's whole history up to `as_of` (from the books'
    /// beginning, or from the bill's own date for one named open bill), read
    /// through the same guarded window read `vouchers` uses, and filtered to the
    /// party on the client: nothing here trusts a voucher's own party field.
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn outstandings_detail(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &TallyCompany,
        as_of: &str,
        party_argument: &str,
        kind: DetailKind,
        reference: Option<&str>,
        open_bills: &[OpenBillRow],
        unallocated: &[UnallocatedParty],
    ) -> Result<(Value, Evidence), ToolFailure> {
        self.outstandings_detail_within(
            identity,
            company,
            as_of,
            party_argument,
            kind,
            reference,
            open_bills,
            unallocated,
            DetailLimits {
                rows: MAX_DETAIL_ROWS,
                window: WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard),
            },
        )
        .await
    }

    /// [`Self::outstandings_detail`] with its limits given, so that each of
    /// its refusals can be reached through the handler on a small captured
    /// window.
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn outstandings_detail_within(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &TallyCompany,
        as_of: &str,
        party_argument: &str,
        kind: DetailKind,
        reference: Option<&str>,
        open_bills: &[OpenBillRow],
        unallocated: &[UnallocatedParty],
        limits: DetailLimits,
    ) -> Result<(Value, Evidence), ToolFailure> {
        let (catalogue, mut evidence) = self.read_ledger_catalogue(identity, &company.name).await?;
        let resolved = resolve_ledger_or_refuse(
            catalogue.iter().map(String::as_str),
            party_argument,
            self.settings.redaction,
        )?;
        let party = resolved.name().to_string();
        let books_from = company
            .books_from
            .clone()
            .ok_or_else(|| ToolFailure::from("trail_books_from_missing".to_string()))?;
        let from = trail_window_start(kind, reference, &party, open_bills, &books_from);
        // A window that ends before it starts is refused by the window read
        // itself (`invalid_date_range`), before any request is sent.
        // The `vouchers` window read, with its own limits: in production the
        // planner's, so a window needing more requests than one call may spend
        // is refused, and told as this detail's own refusal.
        // Refused, it keeps the catalogue read's evidence before its own,
        // as the detail's later refusals do.
        let refused = |failure: ToolFailure| {
            with_evidence(window_too_large(failure, kind, reference), &evidence)
        };
        // The window comes from the book (a bill date or the books' start) and
        // the outstandings' as-of date: it is parsed here, where it enters the
        // window layer.
        let (from, as_of) = match (parse_window_date(&from), parse_window_date(as_of)) {
            (Ok(from), Ok(as_of)) => (from, as_of),
            (Err(failure), _) | (_, Err(failure)) => return Err(refused(failure)),
        };
        let read = self
            .read_voucher_window(
                identity,
                &company.name,
                &from,
                &as_of,
                VoucherReadShape::EntryWildcard,
                WindowPlanSource::Estimate { known_marks: None },
                limits.window,
                |xml| parse_agent_rows(xml, identity.company_guid()),
            )
            .await
            .map_err(refused)?;
        evidence = combine_evidence(evidence, read.all_evidence());
        let late = |failure: ToolFailure| with_evidence(failure, &evidence);
        let rows = read.rows;
        let vouchers_read = rows.len();
        let rows = validate_then_filter_voucher_rows(rows, from.as_str(), as_of.as_str(), None)
            .map_err(|code| late(ToolFailure::from(code)))?;
        let entries = entries_for_party(&rows, &party)
            .map_err(|refusal| late(ToolFailure::from(refusal.0.to_string())))?;
        let party_json = redact_value(party_name_value(party.clone()), self.settings.redaction);
        let window = json!({"from": from, "to": as_of, "company_vouchers_read": vouchers_read});
        let mut detail = party_detail(
            kind,
            &party,
            &party_json,
            reference,
            &entries,
            open_bills,
            unallocated,
            vouchers_read,
            limits.rows,
        )
        .map_err(|refusal| late(ToolFailure::from(refusal.0.to_string())))?;
        detail["party"] = party_json;
        detail["ledger_match"] = resolved.to_json(self.settings.redaction);
        detail["as_of"] = json!(as_of);
        detail["window"] = window;
        Ok((detail, evidence))
    }
}

/// The detail object for one party, built from its allocations and the native
/// figures read at the same as-of; the handler adds the party, the as-of and
/// the window. Every state and refusal of the answer is decided here.
///
/// `vouchers_read` is how many vouchers the company window read returned. When
/// it is zero nothing is tied and nothing is listed: an empty read is not
/// corroborated here (the `vouchers` tool corroborates one with a wider read),
/// so it is reported as `window_returned_no_vouchers`, never as a trail of no
/// allocations or a residual the vouchers do not explain.
#[allow(clippy::too_many_arguments)]
pub(super) fn party_detail(
    kind: DetailKind,
    party: &str,
    party_json: &Value,
    reference: Option<&str>,
    entries: &[TrailEntry],
    open_bills: &[OpenBillRow],
    unallocated: &[UnallocatedParty],
    vouchers_read: usize,
    cap: usize,
) -> Result<Value, TrailRefusal> {
    if vouchers_read == 0 {
        return Ok(json!({
            "kind": kind.label(),
            "state": "window_returned_no_vouchers",
        }));
    }
    let row = unallocated.iter().find(|row| row.party == party);
    match kind {
        DetailKind::BillTrail => {
            let bills = bill_trails(party, reference, entries, open_bills)?;
            within_detail_cap(kind, trail_row_count(&bills), cap)?;
            Ok(json!({
                "kind": kind.label(),
                "state": bill_trail_state(&bills, row),
                "bills": bills.iter().map(|bill| bill.json(party_json.clone())).collect::<Vec<_>>(),
            }))
        }
        DetailKind::Unadjusted => {
            let detail = unadjusted_detail(entries, open_bills, party, row)?;
            within_detail_cap(kind, detail.rows.len(), cap)?;
            let mut value = detail.json();
            value["kind"] = json!(kind.label());
            Ok(value)
        }
    }
}

/// The refusal of a party detail on a partial read: `code` says the detail
/// needs a complete read, and the read's own reason stays in band beside it.
pub(super) fn detail_requires_a_complete_read(
    partial_reason: String,
    partial_reasons: Vec<&'static str>,
) -> ToolFailure {
    ToolFailure::from("detail_requires_a_complete_read".to_string()).with_incomplete_read(
        IncompleteRead {
            partial_reason,
            partial_reasons,
        },
    )
}

/// A refusal that follows the voucher read keeps that read's evidence, with
/// whatever the failure already carried.
pub(super) fn with_evidence(mut failure: ToolFailure, evidence: &Evidence) -> ToolFailure {
    failure.evidence = Some(Box::new(match failure.evidence.take() {
        Some(own) => combine_evidence(evidence.clone(), *own),
        None => evidence.clone(),
    }));
    failure
}

#[cfg(test)]
#[path = "agent_bill_trail_tests.rs"]
mod tests;
