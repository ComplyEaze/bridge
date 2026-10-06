//! Server-side summaries over the rows of a `vouchers` window (#1230).
//!
//! The window is read, labelled and selected exactly as `vouchers` does; this file only adds
//! the arithmetic over the rows that survive, with no I/O. Every bucket is a plain sum of the
//! entries of the vouchers in it, so its total can be rebuilt from the vouchers `vouchers`
//! lists for the same arguments, and the result says which vouchers it left out and why.
use super::movement::movement_entry_effect;
use super::voucher_groups::{Placement, Placements, Primary};
use super::*;
use std::collections::{BTreeMap, BTreeSet};

/// How many vouchers each bucket names. The count is exact; the names are a sample that is
/// complete only when the bucket is small, and `vouchers` with the same arguments (and the
/// bucket's ledger, type or dates, where the existing selectors allow it) lists the rest.
pub(super) const MAX_VOUCHER_REFS_PER_BUCKET: usize = 5;

/// How many member ledgers a `group` or `primary_group` bucket names. The count is exact; the names
/// are the largest movements first and complete only when the bucket is small.
pub(super) const MAX_MEMBERS_PER_BUCKET: usize = 10;

/// How many groups a `group` summary lists in `subtree_totals`. The count is exact; the largest
/// movements come first.
pub(super) const MAX_SUBTREE_TOTALS: usize = 60;

/// Whether every group's `subtree_totals` figure is in the answer: at most the bound of groups were reached.
pub(super) fn subtree_totals_complete(total: usize) -> bool {
    total <= MAX_SUBTREE_TOTALS
}

/// What the buckets are grouped by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SummaryGroup {
    Ledger,
    Month,
    VoucherType,
    /// One bucket per immediate parent group of a ledger, as the ledger master shows it.
    Group,
    /// One bucket per group directly under the reserved root, keyed by its `RESERVEDNAME`.
    PrimaryGroup,
}

impl SummaryGroup {
    /// The grouping the arguments ask for, or `None` for an ordinary listing.
    pub(super) fn from_args(args: &Value) -> Result<Option<Self>, ToolFailure> {
        match optional_string(args, "summarise_by")?.as_deref() {
            None => Ok(None),
            Some("ledger") => Ok(Some(Self::Ledger)),
            Some("month") => Ok(Some(Self::Month)),
            Some("voucher_type") => Ok(Some(Self::VoucherType)),
            Some("group") => Ok(Some(Self::Group)),
            Some("primary_group") => Ok(Some(Self::PrimaryGroup)),
            Some(_) => Err(ToolFailure::from("summarise_by_invalid".to_string())),
        }
    }

    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Ledger => "ledger",
            Self::Month => "month",
            Self::VoucherType => "voucher_type",
            Self::Group => "group",
            Self::PrimaryGroup => "primary_group",
        }
    }

    /// Whether the grouping needs each ledger's place in the group tree.
    pub(super) fn needs_placements(self) -> bool {
        matches!(self, Self::Group | Self::PrimaryGroup)
    }
}

/// A grouping and, when the window was narrowed to one ledger, that ledger's resolved name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SummaryRequest {
    pub(super) group: SummaryGroup,
    pub(super) selected_ledger: Option<String>,
    /// Each ledger's place in the group tree, read before and after the window and equal both
    /// times, for `group` and `primary_group`; `None` for every other grouping.
    pub(super) placements: Option<Arc<Placements>>,
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
    /// Which entries a bucket adds: `all_entries` (every entry of the vouchers the window,
    /// selectors and search selected), or `selected_ledger` for a month or type bucket of a
    /// window narrowed to one ledger.
    pub(super) entries_counted: &'static str,
    /// How many of the summarised vouchers Tally flagged post-dated (`Yes`): they are summed, not
    /// set aside. Together with `post_dated_flag_absent` it bounds what a report-date cut would move.
    pub(super) post_dated_included: usize,
    /// How many of the summarised vouchers carry no post-dated flag at all. With the fetch list
    /// in use Tally asserts the flag on every voucher (protocol reference 8.2c), so a non-zero
    /// value means a source that did not report it; only then is a zero in `post_dated_included`
    /// not proof that none are post-dated.
    pub(super) post_dated_flag_absent: usize,
    /// For a `group` summary: each group on any bucket's chain with the total of everything under it,
    /// descendants included, largest movement first (at most [`MAX_SUBTREE_TOTALS`]), and how many there
    /// are. They overlap (a ledger counts under every group above it), so they do not add up to `totals`.
    pub(super) subtree_totals: Vec<Value>,
    pub(super) subtree_total_count: usize,
}

/// How a bucket of a group grouping is shown: its label and the fields that say which group it is.
struct GroupMeta {
    label: String,
    fields: serde_json::Map<String, Value>,
}

struct Member {
    first_seen: usize,
    debit: String,
    credit: String,
}

struct Bucket {
    /// How many buckets existed when this one was first filled: its place in the window's own
    /// order, which breaks a tie in movement without looking at the (maskable) label.
    first_seen: usize,
    vouchers: usize,
    debit: String,
    credit: String,
    refs: Vec<Value>,
    meta: Option<GroupMeta>,
    /// The ledgers whose entries fall in a group bucket, by name.
    members: BTreeMap<String, Member>,
}

impl Bucket {
    fn new(first_seen: usize, meta: Option<GroupMeta>) -> Self {
        Self {
            first_seen,
            vouchers: 0,
            debit: "0".to_string(),
            credit: "0".to_string(),
            refs: Vec::new(),
            meta,
            members: BTreeMap::new(),
        }
    }
}

/// A ledger's place in the group tree, or why it has none: a ledger the catalogue does not list, or
/// whose chain could not be walked, refuses the whole summary, so no bucket is ever short of an entry it
/// could not place. The two reads that made the placements were equal, so a missing ledger is not drift.
fn placement_of<'a>(request: &'a SummaryRequest, ledger: &str) -> Result<&'a Placement, String> {
    let placements = request
        .placements
        .as_ref()
        .ok_or_else(|| "summary_group_unresolved:no_placements".to_string())?;
    match placements.get(ledger) {
        None => Err("summary_group_unresolved:ledger_not_in_catalogue".to_string()),
        Some(Err(gap)) => Err(format!("summary_group_unresolved:{gap}")),
        Some(Ok(placement)) => Ok(placement),
    }
}

/// What identifies a bucket of a group grouping.
fn group_key(group: SummaryGroup, placement: &Placement) -> String {
    match group {
        SummaryGroup::PrimaryGroup => placement.primary().key,
        // The reserved root is the only placement with no group of its own; a user group may be
        // named like it, so the two never share a bucket.
        _ => match placement.chain.first() {
            Some(hop) => format!("group:{}", hop.name),
            None => "root".to_string(),
        },
    }
}

/// How a bucket of a group grouping is shown. Built once, when the bucket is first filled.
fn group_meta(group: SummaryGroup, placement: &Placement) -> GroupMeta {
    let hop = |name: &str, reserved: Option<&str>| json!({"name": name, "reserved_name": reserved});
    let Primary {
        name,
        reserved_name,
        ..
    } = placement.primary();
    let mut fields = serde_json::Map::new();
    if group == SummaryGroup::PrimaryGroup {
        fields.insert("reserved_name".to_string(), json!(reserved_name));
        return GroupMeta {
            label: name,
            fields,
        };
    }
    fields.insert(
        "reserved_name".to_string(),
        json!(placement.group_reserved_name()),
    );
    fields.insert(
        "chain".to_string(),
        Value::Array(
            placement
                .chain
                .iter()
                .map(|h| hop(&h.name, Some(&h.reserved_name)))
                .collect(),
        ),
    );
    fields.insert(
        "primary_group".to_string(),
        hop(&name, reserved_name.as_deref()),
    );
    GroupMeta {
        label: placement.group_name().to_string(),
        fields,
    }
}

/// One group's total over the whole subtree under it, for a `group` summary: every ledger below it at
/// any depth, so a predefined group whose ledgers all sit in user sub-groups still has its figure.
struct Subtree {
    first_seen: usize,
    depth: usize,
    reserved_name: String,
    vouchers: usize,
    last_voucher: usize,
    debit: String,
    credit: String,
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
    let (is_debit, magnitude) = movement_entry_effect(&amount, deemed_positive);
    Ok(if is_debit {
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
    let mut subtrees: BTreeMap<String, Subtree> = BTreeMap::new();
    let mut vouchers_summarised = 0usize;
    let (mut post_dated_included, mut post_dated_flag_absent) = (0usize, 0usize);
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
        match row["post_dated"].as_bool() {
            Some(true) => post_dated_included += 1,
            Some(false) => {}
            None => post_dated_flag_absent += 1,
        }
        let counted = |amount: &EntryAmount<'_>| match (&request.selected_ledger, request.group) {
            (Some(selected), SummaryGroup::Month | SummaryGroup::VoucherType) => {
                amount.ledger == selected
            }
            _ => true,
        };
        let mut touched: BTreeSet<String> = BTreeSet::new();
        let voucher_key = match request.group {
            SummaryGroup::Ledger | SummaryGroup::Group | SummaryGroup::PrimaryGroup => None,
            SummaryGroup::Month => Some(month_of(row["date"].as_str().unwrap_or_default())?),
            SummaryGroup::VoucherType => Some(
                row["voucher_type"]
                    .as_str()
                    .ok_or_else(|| "voucher_type_missing".to_string())?
                    .to_string(),
            ),
        };
        for amount in amounts.iter().filter(|amount| counted(amount)) {
            let placement = if request.group.needs_placements() {
                Some(placement_of(request, amount.ledger)?)
            } else {
                None
            };
            let key = match (&voucher_key, placement) {
                (Some(key), _) => key.clone(),
                (None, Some(placement)) => group_key(request.group, placement),
                (None, None) => amount.ledger.to_string(),
            };
            let first_seen = buckets.len();
            let bucket = buckets.entry(key.clone()).or_insert_with(|| {
                Bucket::new(first_seen, placement.map(|p| group_meta(request.group, p)))
            });
            if let (SummaryGroup::Group, Some(placement)) = (request.group, placement) {
                // The entry also counts under every group above its own, once per voucher.
                for (position, hop) in placement.chain.iter().enumerate() {
                    let seen = subtrees.len();
                    let subtree = subtrees.entry(hop.name.clone()).or_insert_with(|| Subtree {
                        first_seen: seen,
                        depth: placement.chain.len() - position,
                        reserved_name: hop.reserved_name.clone(),
                        vouchers: 0,
                        last_voucher: usize::MAX,
                        debit: "0".to_string(),
                        credit: "0".to_string(),
                    });
                    if let Some(debit) = &amount.debit {
                        subtree.debit = add_decimal(&subtree.debit, debit)?;
                    }
                    if let Some(credit) = &amount.credit {
                        subtree.credit = add_decimal(&subtree.credit, credit)?;
                    }
                    if subtree.last_voucher != vouchers_summarised {
                        subtree.last_voucher = vouchers_summarised;
                        subtree.vouchers += 1;
                    }
                }
            }
            if request.group.needs_placements() {
                let seen = bucket.members.len();
                let member = bucket
                    .members
                    .entry(amount.ledger.to_string())
                    .or_insert_with(|| Member {
                        first_seen: seen,
                        debit: "0".to_string(),
                        credit: "0".to_string(),
                    });
                if let Some(debit) = &amount.debit {
                    member.debit = add_decimal(&member.debit, debit)?;
                }
                if let Some(credit) = &amount.credit {
                    member.credit = add_decimal(&member.credit, credit)?;
                }
            }
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
                // A group's name is shown as the book has it, like `parent` in `ledger_masters`.
                SummaryGroup::Group | SummaryGroup::PrimaryGroup => json!(bucket
                    .meta
                    .as_ref()
                    .map_or(key.as_str(), |meta| meta.label.as_str())),
            };
            let refs_complete = bucket.refs.len() == bucket.vouchers;
            let mut shown = json!({
                "group": group,
                "vouchers": bucket.vouchers,
                "debit": bucket.debit,
                "credit": bucket.credit,
                "net": net,
                "voucher_refs": bucket.refs,
                "voucher_refs_complete": refs_complete,
            });
            if let Some(meta) = bucket.meta {
                for (field, value) in meta.fields {
                    shown[field] = value;
                }
                let total = bucket.members.len();
                shown["members"] = members_shown(bucket.members)?;
                shown["members_total"] = json!(total);
                shown["members_complete"] = json!(total <= MAX_MEMBERS_PER_BUCKET);
            }
            Ok((bucket.first_seen, gross, shown))
        })
        .collect::<Result<Vec<_>, String>>()?;
    // Months run in calendar order; ledgers and types by the larger movement first, then by
    // where the bucket first appeared in the window. Never by name: under `mask_parties` the
    // order of tied rows would show the alphabetical order of the real names.
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
    let (subtree_totals, subtree_total_count) = subtree_totals_shown(subtrees)?;
    Ok(Summary {
        // `position` is the bucket's place in the whole ordering, so two buckets whose labels
        // read alike under masking can still be told apart and paged without a repeat.
        buckets: shaped
            .into_iter()
            .enumerate()
            .map(|(index, (_, _, mut bucket))| {
                bucket["position"] = json!(index + 1);
                bucket
            })
            .collect(),
        post_dated_included,
        post_dated_flag_absent,
        subtree_totals,
        subtree_total_count,
        vouchers_summarised,
        excluded: json!({"cancelled": cancelled, "optional": optional, "no_accounting_entries": no_entries}),
        totals: json!({"debit": total_debit, "credit": total_credit}),
        entries_counted: match (&request.selected_ledger, request.group) {
            (Some(_), SummaryGroup::Month | SummaryGroup::VoucherType) => "selected_ledger",
            _ => "all_entries",
        },
    })
}

/// The `subtree_totals` of a `group` summary: largest movement first, then in the order the groups were
/// first reached, never by name; at most [`MAX_SUBTREE_TOTALS`], and the exact count.
fn subtree_totals_shown(
    subtrees: BTreeMap<String, Subtree>,
) -> Result<(Vec<Value>, usize), String> {
    let count = subtrees.len();
    let mut ranked = subtrees
        .into_iter()
        .map(|(name, subtree)| {
            let gross = add_decimal(
                &subtree.credit,
                bridge_tally_core::ExactDecimal::parse(subtree.debit.clone())
                    .map_err(|_| "voucher_amount_invalid".to_string())?
                    .magnitude()
                    .as_str(),
            )?;
            Ok((subtree.first_seen, gross, name, subtree))
        })
        .collect::<Result<Vec<_>, String>>()?;
    ranked.sort_by(|left, right| {
        let by_gross = bridge_tally_core::ExactDecimal::parse(right.1.clone())
            .and_then(|right_gross| {
                bridge_tally_core::ExactDecimal::parse(left.1.clone())
                    .map(|left_gross| right_gross.cmp_magnitude(&left_gross))
            })
            .unwrap_or(std::cmp::Ordering::Equal);
        by_gross.then_with(|| left.0.cmp(&right.0))
    });
    let shown = ranked
        .into_iter()
        .take(MAX_SUBTREE_TOTALS)
        .map(|(_, _, name, subtree)| {
            let net = add_decimal(&subtree.debit, &subtree.credit)?;
            Ok(json!({
                "group": name, "reserved_name": subtree.reserved_name, "depth": subtree.depth,
                "vouchers": subtree.vouchers, "debit": subtree.debit, "credit": subtree.credit, "net": net,
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((shown, count))
}

/// The member ledgers of a group bucket, the largest movement first and then in the order they first
/// appeared in the window, never by name (a masked name must not reveal the alphabetical order), at
/// most [`MAX_MEMBERS_PER_BUCKET`]. A member's `debit` and `credit` are its entries in the bucket.
fn members_shown(members: BTreeMap<String, Member>) -> Result<Value, String> {
    let mut ranked = members
        .into_iter()
        .map(|(ledger, member)| {
            let gross = add_decimal(
                &member.credit,
                bridge_tally_core::ExactDecimal::parse(member.debit.clone())
                    .map_err(|_| "voucher_amount_invalid".to_string())?
                    .magnitude()
                    .as_str(),
            )?;
            Ok((member.first_seen, gross, ledger, member))
        })
        .collect::<Result<Vec<_>, String>>()?;
    ranked.sort_by(|left, right| {
        let by_gross = bridge_tally_core::ExactDecimal::parse(right.1.clone())
            .and_then(|right_gross| {
                bridge_tally_core::ExactDecimal::parse(left.1.clone())
                    .map(|left_gross| right_gross.cmp_magnitude(&left_gross))
            })
            .unwrap_or(std::cmp::Ordering::Equal);
        by_gross.then_with(|| left.0.cmp(&right.0))
    });
    Ok(Value::Array(
        ranked
            .into_iter()
            .take(MAX_MEMBERS_PER_BUCKET)
            .map(|(_, _, ledger, member)| {
                json!({"ledger": party_name_value(ledger), "debit": member.debit, "credit": member.credit})
            })
            .collect(),
    ))
}

/// One page of buckets: from `offset`, at most `limit`, and no more than `byte_budget` bytes
/// serialized (at least one bucket, so a page always advances). The second value is whether
/// buckets remain after this page. The caller passes a fifth of the response budget: the
/// response carries the page twice (structured and text copy) and the rest of the result once.
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
