//! Server-side summaries over the rows of a `vouchers` window (#1230).
//!
//! The window is read, labelled and selected exactly as `vouchers` does; this file only adds
//! the arithmetic over the rows that survive, with no I/O. Every bucket is a plain sum of the
//! entries of the vouchers in it, so its total can be rebuilt from the vouchers `vouchers`
//! lists for the same arguments, and the result says which vouchers it left out and why.
use super::movement::movement_entry_is_debit;
use super::*;
use std::collections::{BTreeMap, BTreeSet};

/// How many vouchers each bucket names. The count is exact; the names are a sample that is
/// complete only when the bucket is small, and `vouchers` with the same arguments (and the
/// bucket's ledger, type or dates) lists the rest.
pub(super) const MAX_VOUCHER_REFS_PER_BUCKET: usize = 5;

/// What the buckets are grouped by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SummaryGroup {
    Ledger,
    Month,
    VoucherType,
}

impl SummaryGroup {
    /// The grouping the arguments ask for, or `None` for an ordinary listing.
    pub(super) fn from_args(args: &Value) -> Result<Option<Self>, ToolFailure> {
        match optional_string(args, "summarise_by")?.as_deref() {
            None => Ok(None),
            Some("ledger") => Ok(Some(Self::Ledger)),
            Some("month") => Ok(Some(Self::Month)),
            Some("voucher_type") => Ok(Some(Self::VoucherType)),
            Some(_) => Err(ToolFailure::from("summarise_by_invalid".to_string())),
        }
    }

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Ledger => "ledger",
            Self::Month => "month",
            Self::VoucherType => "voucher_type",
        }
    }
}

/// A grouping and, when the window was narrowed to one ledger, that ledger's resolved name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SummaryRequest {
    pub(super) group: SummaryGroup,
    pub(super) selected_ledger: Option<String>,
}

/// One summarised window: the buckets in presentation order and what stands behind them.
pub(super) struct Summary {
    pub(super) buckets: Vec<Value>,
    /// Vouchers that fed a bucket.
    pub(super) vouchers_summarised: usize,
    /// Vouchers in the window left out of every bucket, by cause.
    pub(super) excluded: Value,
    /// The debit and credit totals over the same entries the buckets add up.
    pub(super) totals: Value,
    /// Which entries a bucket adds: `all`, or `selected_ledger` for a month or type bucket of
    /// a window narrowed to one ledger.
    pub(super) entries_counted: &'static str,
}

struct Bucket {
    vouchers: usize,
    debit: String,
    credit: String,
    refs: Vec<Value>,
}

impl Bucket {
    fn new() -> Self {
        Self {
            vouchers: 0,
            debit: "0".to_string(),
            credit: "0".to_string(),
            refs: Vec::new(),
        }
    }
}

struct EntryAmount<'a> {
    ledger: &'a str,
    /// `-magnitude` for a debit, `magnitude` for a credit, as `ledger_movement` reports them.
    debit: Option<String>,
    credit: Option<String>,
}

fn entry_amount(entry: &Value) -> Result<EntryAmount<'_>, String> {
    let ledger = entry["ledger"]
        .as_str()
        .ok_or_else(|| "voucher_amount_invalid".to_string())?;
    let amount = bridge_tally_core::ExactDecimal::parse(
        entry["amount"]
            .as_str()
            .ok_or_else(|| "voucher_amount_invalid".to_string())?
            .to_string(),
    )
    .map_err(|_| "voucher_amount_invalid".to_string())?;
    let deemed_positive = entry["is_deemed_positive"].as_str() == Some("Yes");
    let magnitude = amount.magnitude().as_str().to_string();
    Ok(if movement_entry_is_debit(&amount, deemed_positive) {
        EntryAmount {
            ledger,
            debit: Some(format!("-{magnitude}")),
            credit: None,
        }
    } else {
        EntryAmount {
            ledger,
            debit: None,
            credit: Some(magnitude),
        }
    })
}

fn voucher_ref(row: &Value) -> Value {
    json!({
        "date": row["date"], "voucher_type": row["voucher_type"],
        "voucher_number": row["voucher_number"], "guid": row["guid"],
    })
}

fn month_of(date: &str) -> Result<String, String> {
    if date.len() >= 6 && date.is_char_boundary(6) && date.bytes().all(|byte| byte.is_ascii_digit())
    {
        Ok(format!("{}-{}", &date[..4], &date[4..6]))
    } else {
        Err("voucher_date_invalid".to_string())
    }
}

/// Sums the rows into buckets.
///
/// Cancelled and optional vouchers, and vouchers with no accounting entry, are left out the way
/// `ledger_movement` leaves them out, and are counted in `excluded`. A voucher kept must
/// balance to zero over all its entries, whatever the grouping, or the whole summary is
/// refused: a bucket built from an unbalanced voucher would be a number nobody can tie out.
pub(super) fn summarise(rows: &[Value], request: &SummaryRequest) -> Result<Summary, String> {
    let (mut cancelled, mut optional, mut no_entries) = (0usize, 0usize, 0usize);
    let mut buckets: BTreeMap<String, Bucket> = BTreeMap::new();
    let mut vouchers_summarised = 0usize;
    let (mut total_debit, mut total_credit) = ("0".to_string(), "0".to_string());
    for row in rows {
        if row["cancelled"].as_bool() == Some(true) {
            cancelled += 1;
            continue;
        }
        if row["optional"].as_bool() == Some(true) {
            optional += 1;
            continue;
        }
        let entries = row["amounts"].as_array().map(Vec::as_slice).unwrap_or(&[]);
        if entries.is_empty() {
            no_entries += 1;
            continue;
        }
        let amounts = entries
            .iter()
            .map(entry_amount)
            .collect::<Result<Vec<_>, _>>()?;
        let mut balance = "0".to_string();
        for amount in &amounts {
            for part in amount.debit.iter().chain(amount.credit.iter()) {
                balance = add_decimal(&balance, part)?;
            }
        }
        if !bridge_tally_core::ExactDecimal::parse(balance)
            .map_err(|_| "voucher_amount_invalid".to_string())?
            .is_zero()
        {
            return Err("voucher_entries_unbalanced".to_string());
        }
        vouchers_summarised += 1;
        let counted = |amount: &EntryAmount<'_>| match (&request.selected_ledger, request.group) {
            (Some(selected), SummaryGroup::Month | SummaryGroup::VoucherType) => {
                amount.ledger == selected
            }
            _ => true,
        };
        let mut touched: BTreeSet<String> = BTreeSet::new();
        let voucher_key = match request.group {
            SummaryGroup::Ledger => None,
            SummaryGroup::Month => Some(month_of(row["date"].as_str().unwrap_or_default())?),
            SummaryGroup::VoucherType => Some(
                row["voucher_type"]
                    .as_str()
                    .ok_or_else(|| "voucher_type_missing".to_string())?
                    .to_string(),
            ),
        };
        for amount in amounts.iter().filter(|amount| counted(amount)) {
            let key = voucher_key
                .clone()
                .unwrap_or_else(|| amount.ledger.to_string());
            let bucket = buckets.entry(key.clone()).or_insert_with(Bucket::new);
            if let Some(debit) = &amount.debit {
                bucket.debit = add_decimal(&bucket.debit, debit)?;
                total_debit = add_decimal(&total_debit, debit)?;
            }
            if let Some(credit) = &amount.credit {
                bucket.credit = add_decimal(&bucket.credit, credit)?;
                total_credit = add_decimal(&total_credit, credit)?;
            }
            // A voucher counts once in a bucket however many of its entries land there.
            if touched.insert(key) {
                bucket.vouchers += 1;
                if bucket.refs.len() < MAX_VOUCHER_REFS_PER_BUCKET {
                    bucket.refs.push(voucher_ref(row));
                }
            }
        }
    }
    let mut shaped = buckets
        .into_iter()
        .map(|(key, bucket)| {
            let net = add_decimal(&bucket.debit, &bucket.credit)?;
            let gross = add_decimal(
                &bucket.credit,
                bridge_tally_core::ExactDecimal::parse(bucket.debit.clone())
                    .map_err(|_| "voucher_amount_invalid".to_string())?
                    .magnitude()
                    .as_str(),
            )?;
            let group = match request.group {
                SummaryGroup::Ledger => party_name_value(key.clone()),
                SummaryGroup::Month | SummaryGroup::VoucherType => json!(key),
            };
            let refs_complete = bucket.refs.len() == bucket.vouchers;
            Ok((
                key,
                gross,
                json!({
                    "group": group,
                    "vouchers": bucket.vouchers,
                    "debit": bucket.debit,
                    "credit": bucket.credit,
                    "net": net,
                    "voucher_refs": bucket.refs,
                    "voucher_refs_complete": refs_complete,
                }),
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    // Months run in calendar order; ledgers and types by the larger movement first, then name.
    if request.group != SummaryGroup::Month {
        shaped.sort_by(|left, right| {
            let by_gross = bridge_tally_core::ExactDecimal::parse(right.1.clone())
                .and_then(|right_gross| {
                    bridge_tally_core::ExactDecimal::parse(left.1.clone())
                        .map(|left_gross| right_gross.cmp_magnitude(&left_gross))
                })
                .unwrap_or(std::cmp::Ordering::Equal);
            by_gross.then_with(|| left.0.cmp(&right.0))
        });
    }
    Ok(Summary {
        buckets: shaped.into_iter().map(|(_, _, bucket)| bucket).collect(),
        vouchers_summarised,
        excluded: json!({"cancelled": cancelled, "optional": optional, "no_accounting_entries": no_entries}),
        totals: json!({"debit": total_debit, "credit": total_credit}),
        entries_counted: match (&request.selected_ledger, request.group) {
            (Some(_), SummaryGroup::Month | SummaryGroup::VoucherType) => "selected_ledger",
            _ => "all",
        },
    })
}

/// One page of buckets: from `offset`, at most `limit`, and no more than `byte_budget` bytes
/// serialized (at least one bucket, so a page always advances). The second value is whether
/// buckets remain after this page.
pub(super) fn page_buckets(
    summary: &Summary,
    offset: usize,
    limit: usize,
    byte_budget: usize,
) -> (Vec<Value>, bool) {
    let mut used = 0usize;
    let mut page = Vec::new();
    for bucket in summary.buckets.iter().skip(offset).take(limit) {
        used = used.saturating_add(bucket.to_string().len());
        if used > byte_budget && !page.is_empty() {
            break;
        }
        page.push(bucket.clone());
    }
    let truncated = offset.saturating_add(page.len()) < summary.buckets.len();
    (page, truncated)
}

#[cfg(test)]
#[path = "agent_voucher_summary_tests.rs"]
mod tests;
