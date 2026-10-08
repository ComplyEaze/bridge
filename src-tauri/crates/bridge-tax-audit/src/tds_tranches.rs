// SPDX-License-Identifier: Apache-2.0
//! The reference implementation's `tds_tranches.crossing_tranches`: when each credit to a payee
//! attracts TDS, as the threshold sections word it (the reference's reading, pending a CA's
//! confirmation, from s.194C(5) and its proviso).
//!
//! * A credit over the single-sum limit attracts TDS on its own date.
//! * Once the year's aggregate exceeds its limit, every credit does: the earlier credits not yet
//!   taxed from the date the aggregate is crossed, each later one on its own date.
//! * A section with no single-sum limit (s.194J per category, s.194-I per month, s.194H) has only
//!   the aggregate test.
//! * The aggregate is GROSS and a crossing is never undone: a credit of zero or less (a reversal)
//!   lowers nothing and un-crosses nothing.
//!
//! The reference's `BASIS_TEXT` is not ported: only `tds_interest_201` quotes it.

use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_primitives::TallyDate;

use crate::error::{AuditError, Result};

/// One day's tranche: the credits that became deductible on `day`, their total, their vouchers'
/// ids (sorted, unique) and each credit as (own date, paise), sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tranche<'a, K: ?Sized = str> {
    pub day: TallyDate,
    pub paise: i64,
    pub ids: Vec<&'a K>,
    pub credits: Vec<(TallyDate, i64)>,
}

/// `credits` are (date, voucher id, net paise), the id the caller's own and unique per voucher
/// (`tds_payees` passes its [`crate::book::VoucherKey`], as the reference's passes its per-voucher
/// key). Returns the tranches by day. Refuses with no limit at all, as the reference raises, and
/// on an i64 overflow.
pub fn crossing_tranches<'a, K: Ord + ?Sized>(
    credits: &[(TallyDate, &'a K, i64)],
    single_limit: Option<i64>,
    aggregate_limit: Option<i64>,
    test_id: &str,
) -> Result<Vec<Tranche<'a, K>>> {
    if single_limit.is_none() && aggregate_limit.is_none() {
        return Err(AuditError::Config(format!(
            "{test_id}: crossing_tranches needs a single-sum limit, an aggregate limit, or both"
        )));
    }
    let overflow = || AuditError::Config(format!("{test_id}: an amount overflows i64 paise"));
    let mut sorted: Vec<&(TallyDate, &'a K, i64)> = credits.iter().collect();
    // The reference sorts by (date, id); Python's str order is code-point order, as Rust's is.
    sorted.sort_by(|a, b| (&a.0, a.1).cmp(&(&b.0, b.1)));
    // (own date, id, paise, deductible day once known)
    let mut entries: Vec<(&TallyDate, &'a K, i64, Option<TallyDate>)> = Vec::new();
    let mut running: i64 = 0;
    let mut crossed = false;
    for (d, id, paise) in sorted {
        if *paise <= 0 {
            continue;
        }
        running = running.checked_add(*paise).ok_or_else(overflow)?;
        let single = single_limit.is_some_and(|limit| *paise > limit);
        entries.push((d, id, *paise, (crossed || single).then(|| d.clone())));
        if !crossed && aggregate_limit.is_some_and(|limit| running > limit) {
            crossed = true;
            for e in &mut entries {
                if e.3.is_none() {
                    e.3 = Some(d.clone());
                }
            }
        }
    }
    let mut by_day: BTreeMap<TallyDate, Vec<(&TallyDate, &'a K, i64)>> = BTreeMap::new();
    for (own, id, paise, on) in entries {
        if let Some(day) = on {
            by_day.entry(day).or_default().push((own, id, paise));
        }
    }
    by_day
        .into_iter()
        .map(|(day, es)| {
            let paise = es
                .iter()
                .try_fold(0_i64, |a, e| a.checked_add(e.2))
                .ok_or_else(overflow)?;
            let ids: BTreeSet<&'a K> = es.iter().map(|e| e.1).collect();
            let mut own: Vec<(TallyDate, i64)> = es.iter().map(|e| (e.0.clone(), e.2)).collect();
            own.sort();
            Ok(Tranche {
                day,
                paise,
                ids: ids.into_iter().collect(),
                credits: own,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> TallyDate {
        TallyDate::parse(s.to_string()).unwrap()
    }

    /// The reference's reading, case by case: a single-sum credit on its own date; the earlier
    /// credits from the crossing date; later ones on their own; a reversal lowering nothing.
    #[test]
    fn credits_become_deductible_as_the_reference_reads_the_limits() {
        let credits = [
            (d("20250410"), "b", 20),
            (d("20250405"), "a", 50),
            (d("20250501"), "r", -40),
            (d("20250601"), "c", 45),
            (d("20250701"), "e", 10),
        ];
        // single 30, aggregate 100: a (50) is over the single limit on its own date; b (20) waits;
        // the reversal is skipped and un-crosses nothing; c crosses the aggregate (115) on 20250601,
        // taking b with it; e is after the crossing, on its own date.
        let t = crossing_tranches(&credits, Some(30), Some(100), "t").unwrap();
        let got: Vec<(&str, i64, Vec<&str>)> = t
            .iter()
            .map(|x| (x.day.as_str(), x.paise, x.ids.clone()))
            .collect();
        assert_eq!(
            got,
            vec![
                ("20250405", 50, vec!["a"]),
                ("20250601", 65, vec!["b", "c"]),
                ("20250701", 10, vec!["e"]),
            ]
        );
        assert_eq!(t[1].credits, vec![(d("20250410"), 20), (d("20250601"), 45)]);
        // Never crossed, no single limit: nothing is deductible.
        assert!(crossing_tranches(&credits, None, Some(1_000), "t")
            .unwrap()
            .is_empty());
        // Exactly at a limit is not over it.
        let at = [(d("20250405"), "a", 30)];
        assert!(crossing_tranches(&at, Some(30), Some(30), "t")
            .unwrap()
            .is_empty());
        assert!(matches!(
            crossing_tranches(&at, None, None, "t"),
            Err(AuditError::Config(m)) if m.contains("needs a single-sum limit, an aggregate limit, or both")
        ));
    }
}
