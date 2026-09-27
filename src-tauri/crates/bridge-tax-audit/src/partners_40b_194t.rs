// SPDX-License-Identifier: Apache-2.0
//! Port of the reference engine's `partners_40b_194t` (firm/LLP only): s.40(b) interest on a
//! partner's capital against the allowable rate, remuneration credited, and s.194T TDS on payments
//! to partners.
//!
//! Applicability is read from the rules, never from the entity type's name: the test applies when
//! `[entity.<type>].s194t` is true or `s40b_interest_rate_bp` is above zero. When it does not, the
//! result is one figure (`applicable` = "no") and nothing else, and no voucher is read.
//!
//! A voucher's line on a partner's capital ledger is an interest (or remuneration) credit when that
//! same voucher also carries a line on the partner's own interest (or remuneration) ledger; those
//! lines are left out of the day-by-day capital walk. Allowable interest is simple interest at
//! min(deed rate, rules rate) on each day's capital, a day in debit contributing zero, over actual
//! days / 365, with three named sensitivities (days / 360, opening balance only, no reduction).
//!
//! A credit is read gross of the TDS on its own voucher, the TDS read only through the ledgers the
//! client's statutory dues classify as TDS payable, where the partner's capital is the only ledger
//! on the TDS's other side; otherwise the voucher is named as sharing its TDS. A voucher carrying
//! both the interest and the remuneration ledger is split only where exact, else counted as
//! interest and named; a set-off on the capital, and a voucher on the partner's ledgers touching no
//! capital, are named (the latter counted nowhere). A partner whose interest cannot be read exactly
//! has s.40(b) not computed, never an excess. The s.194T base is gross: a reversal lowers nothing.
//! The s.40(b)(v) remuneration ceiling is quoted from the rules' `[s40b_v]`, never applied as a
//! limit. `Walk` and `run` say how each is read.
//!
//! Every amount is carried in i128 (a year of capital-paise-days times a rate exceeds i64) and
//! each figure is checked back into i64.
//!
//! One narrowing, typed at the boundary: the deed's `interest_rate_bp` must be an integer. The
//! reference would also take a float (and fail on text); no real deed carries either.

use std::collections::{BTreeMap, BTreeSet};

use sha1::{Digest, Sha1};

use crate::book::{Book, Voucher};
use crate::depreciation::civil_day_number;
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::read::Window;
use crate::rules::Rules;
use crate::support::{overflow, rupees, voucher_label};
use crate::tds_payees::py_format_g;
use crate::PartnersConfig;

pub const TEST_ID: &str = "partners_40b_194t";
pub const VERSION: &str = "1";

/// The reference's `DEFAULT_S194T`, used (and said so) when the rules carry no `[s194t]` table.
const DEFAULT_S194T_RATE_BP: i64 = 1000;
const DEFAULT_S194T_LIMIT_PAISE: i64 = 2_000_000;

/// One `[partners.<key>]` entry, typed when the test runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partner {
    pub capital_ledgers: Vec<String>,
    pub interest_ledger: Option<String>,
    pub remuneration_ledger: Option<String>,
}

/// The partners and the deed's interest rate, typed from the bound `[partners]` table. A partner
/// without `capital_ledgers` is refused, as the reference's `p["capital_ledgers"]` raises; the
/// shapes of the three name locations were already checked when they were bound.
pub fn partners_config(cfg: &PartnersConfig) -> Result<(BTreeMap<String, Partner>, Option<i64>)> {
    let mut partners = BTreeMap::new();
    for (key, entry) in &cfg.partners {
        let t = entry.as_table().ok_or_else(|| {
            AuditError::Config(format!("{TEST_ID}: [partners].{key} is not a table"))
        })?;
        let text = |name: &str| -> Result<Option<String>> {
            t.get(name)
                .map(|v| {
                    v.as_str().map(str::to_string).ok_or_else(|| {
                        AuditError::Config(format!(
                            "{TEST_ID}: [partners].{key}.{name} is not a name"
                        ))
                    })
                })
                .transpose()
        };
        let capital_ledgers = t
            .get("capital_ledgers")
            .ok_or_else(|| {
                AuditError::Config(format!(
                    "{TEST_ID}: [partners].{key} has no capital_ledgers"
                ))
            })?
            .as_array()
            .and_then(|a| a.iter().map(|x| x.as_str().map(str::to_string)).collect())
            .ok_or_else(|| {
                AuditError::Config(format!(
                    "{TEST_ID}: [partners].{key}.capital_ledgers is not a list of names"
                ))
            })?;
        partners.insert(
            key.clone(),
            Partner {
                capital_ledgers,
                interest_ledger: text("interest_ledger")?,
                remuneration_ledger: text("remuneration_ledger")?,
            },
        );
    }
    let deed_rate = match &cfg.deed {
        None => None,
        Some(deed) => {
            let t = deed.as_table().ok_or_else(|| {
                AuditError::Config(format!("{TEST_ID}: [partners].deed is not a table"))
            })?;
            match t.get("interest_rate_bp") {
                None => None,
                Some(v) => Some(v.as_integer().ok_or_else(|| {
                    AuditError::Config(format!(
                        "{TEST_ID}: [partners].deed.interest_rate_bp is not an integer"
                    ))
                })?),
            }
        }
    };
    Ok((partners, deed_rate))
}

/// The reference's `_hash`: the first 8 hex characters of the key's SHA-1.
fn hash8(key: &str) -> String {
    let digest = Sha1::digest(key.as_bytes());
    crate::canonical::hex(&digest)[..8].to_string()
}

/// `numerator / denominator` rounded half-up; 0 when either is not positive.
fn round_half_up(numerator: i128, denominator: i128) -> i128 {
    if numerator <= 0 || denominator <= 0 {
        return 0;
    }
    (numerator + denominator / 2) / denominator
}

type Vouchers<'a> = BTreeMap<String, &'a Voucher>;

/// One partner's pass over the population (the reference's `compute_partner_walk`).
///
/// A credit is read GROSS of the TDS deducted on the same voucher where this partner's capital is
/// the only ledger on the TDS's opposite side (signed: a reversal takes out what the original
/// added); otherwise the voucher is named as sharing its TDS. A voucher carrying both the interest
/// and the remuneration ledger is split only where it is exact; otherwise it counts as interest and
/// is named. A voucher on one of the two ledgers that both credits and debits the capital is named.
/// A voucher on this partner's interest or remuneration ledger touching no partner's capital is
/// counted nowhere and named.
struct Walk<'a> {
    opening_paise: i128,
    /// (day number, delta paise), sorted by day.
    events: Vec<(i64, i128)>,
    interest_credited_paise: i128,
    interest_vouchers: Vouchers<'a>,
    remuneration_credited_paise: i128,
    remuneration_vouchers: Vouchers<'a>,
    interest_by_voucher: BTreeMap<String, i128>,
    remuneration_by_voucher: BTreeMap<String, i128>,
    shared_tds_vouchers: Vouchers<'a>,
    shared_tds_interest_vouchers: Vouchers<'a>,
    shared_tds_remuneration_vouchers: Vouchers<'a>,
    mixed_unsplit_vouchers: Vouchers<'a>,
    mixed_set_off_vouchers: Vouchers<'a>,
    off_capital_vouchers: Vouchers<'a>,
    off_capital_interest_vouchers: Vouchers<'a>,
    capital_set_off_vouchers: Vouchers<'a>,
}

fn walk<'a>(
    pop: &[&'a Voucher],
    book: &Book,
    p: &Partner,
    tds_ledgers: &BTreeSet<String>,
    all_capitals: &BTreeSet<String>,
) -> Walk<'a> {
    let capital: BTreeSet<&str> = p.capital_ledgers.iter().map(String::as_str).collect();
    // A ledger named twice in capital_ledgers counts twice, as the reference's sum over the list does.
    let opening_paise = p
        .capital_ledgers
        .iter()
        .filter_map(|n| book.tb.get(n))
        .map(|row| i128::from(row.opening_paise))
        .sum();
    let mut w = Walk {
        opening_paise,
        events: Vec::new(),
        interest_credited_paise: 0,
        interest_vouchers: BTreeMap::new(),
        remuneration_credited_paise: 0,
        remuneration_vouchers: BTreeMap::new(),
        interest_by_voucher: BTreeMap::new(),
        remuneration_by_voucher: BTreeMap::new(),
        shared_tds_vouchers: BTreeMap::new(),
        shared_tds_interest_vouchers: BTreeMap::new(),
        shared_tds_remuneration_vouchers: BTreeMap::new(),
        mixed_unsplit_vouchers: BTreeMap::new(),
        mixed_set_off_vouchers: BTreeMap::new(),
        off_capital_vouchers: BTreeMap::new(),
        off_capital_interest_vouchers: BTreeMap::new(),
        capital_set_off_vouchers: BTreeMap::new(),
    };
    let interest = p.interest_ledger.as_deref();
    let remuneration = p.remuneration_ledger.as_deref();
    for &v in pop {
        let guid = || v.guid.clone();
        // The capital lines' amounts, and the TDS added back as one more line where it applies.
        let mut lines_here: Vec<i128> = v
            .lines
            .iter()
            .filter(|l| capital.contains(l.ledger.as_str()))
            .map(|l| i128::from(l.amount_paise))
            .collect();
        if lines_here.is_empty() {
            let touches_capital = v
                .lines
                .iter()
                .any(|l| capital.contains(l.ledger.as_str()) || all_capitals.contains(&l.ledger));
            if !touches_capital {
                let on = |ledger: Option<&str>| {
                    ledger.is_some_and(|n| {
                        v.lines.iter().any(|l| l.ledger == n && l.amount_paise != 0)
                    })
                };
                let (on_int, on_rem) = (on(interest), on(remuneration));
                if on_int || on_rem {
                    w.off_capital_vouchers.insert(guid(), v);
                }
                if on_int {
                    w.off_capital_interest_vouchers.insert(guid(), v);
                }
            }
            continue;
        }
        let carries =
            |ledger: Option<&str>| ledger.is_some_and(|n| v.lines.iter().any(|l| l.ledger == n));
        let is_interest = carries(interest);
        let is_remuneration = carries(remuneration);
        let tds: i128 = -v
            .lines
            .iter()
            .filter(|l| tds_ledgers.contains(&l.ledger))
            .map(|l| i128::from(l.amount_paise))
            .sum::<i128>();
        if tds != 0 && (is_interest || is_remuneration) {
            // The ledgers on the TDS's own side must be this partner's capital alone.
            let opposite_alone = v
                .lines
                .iter()
                .filter(|l| {
                    i128::from(l.amount_paise).signum() * tds.signum() < 0
                        && !tds_ledgers.contains(&l.ledger)
                })
                .all(|l| capital.contains(l.ledger.as_str()));
            if opposite_alone {
                lines_here.push(-tds);
            } else {
                w.shared_tds_vouchers.insert(guid(), v);
                if is_interest {
                    w.shared_tds_interest_vouchers.insert(guid(), v);
                }
                if is_remuneration {
                    w.shared_tds_remuneration_vouchers.insert(guid(), v);
                }
            }
        }
        if is_interest && is_remuneration {
            // Split only where exact: every line is this partner's capital, TDS, or a debit to the
            // interest or remuneration ledger; the interest ledger's own debit is the interest.
            let c: i128 = -lines_here.iter().sum::<i128>();
            let int_debit: i128 = v
                .lines
                .iter()
                .filter(|l| Some(l.ledger.as_str()) == interest && l.amount_paise > 0)
                .map(|l| i128::from(l.amount_paise))
                .sum();
            let capital_debited = v
                .lines
                .iter()
                .any(|l| capital.contains(l.ledger.as_str()) && l.amount_paise > 0);
            let exact = v.lines.iter().all(|l| {
                capital.contains(l.ledger.as_str())
                    || tds_ledgers.contains(&l.ledger)
                    || ((Some(l.ledger.as_str()) == interest
                        || Some(l.ledger.as_str()) == remuneration)
                        && l.amount_paise >= 0)
            });
            let i_part = if int_debit > 0 && exact {
                c.min(int_debit)
            } else {
                w.mixed_unsplit_vouchers.insert(guid(), v);
                if capital_debited && c > 0 {
                    w.mixed_set_off_vouchers.insert(guid(), v);
                }
                c
            };
            for (part, is_int) in [(i_part, true), (c - i_part, false)] {
                if part != 0 {
                    let (by_v, vs) = if is_int {
                        (&mut w.interest_by_voucher, &mut w.interest_vouchers)
                    } else {
                        (&mut w.remuneration_by_voucher, &mut w.remuneration_vouchers)
                    };
                    *by_v.entry(guid()).or_insert(0) += part;
                    vs.insert(guid(), v);
                }
            }
            w.interest_credited_paise += i_part;
            w.remuneration_credited_paise += c - i_part;
            continue;
        }
        // A voucher on one of the two ledgers that both credits and debits the capital is read net
        // and named.
        let capital_lines = || {
            v.lines
                .iter()
                .filter(|l| capital.contains(l.ledger.as_str()))
        };
        if (is_interest || is_remuneration)
            && capital_lines().any(|l| l.amount_paise < 0)
            && capital_lines().any(|l| l.amount_paise > 0)
        {
            w.capital_set_off_vouchers.insert(guid(), v);
        }
        for amount in lines_here {
            if is_interest {
                w.interest_credited_paise -= amount;
                w.interest_vouchers.insert(guid(), v);
                *w.interest_by_voucher.entry(guid()).or_insert(0) -= amount;
            } else if is_remuneration {
                w.remuneration_credited_paise -= amount;
                w.remuneration_vouchers.insert(guid(), v);
                *w.remuneration_by_voucher.entry(guid()).or_insert(0) -= amount;
            } else {
                w.events.push((civil_day_number(&v.date), amount));
            }
        }
    }
    w.events.sort_by_key(|e| e.0);
    w
}

/// The reference's `s194t_credits`: each voucher's net interest plus remuneration to the partner,
/// with its date (the remuneration voucher wins a GUID both maps hold, as `{**a, **b}` does).
fn s194t_credits<'a>(w: &Walk<'a>) -> Vec<(&'a Voucher, String, i128)> {
    let mut vouchers = w.interest_vouchers.clone();
    vouchers.extend(w.remuneration_vouchers.clone());
    let mut amounts: BTreeMap<&String, i128> = BTreeMap::new();
    for by_voucher in [&w.interest_by_voucher, &w.remuneration_by_voucher] {
        for (g, paise) in by_voucher {
            *amounts.entry(g).or_insert(0) += paise;
        }
    }
    amounts
        .into_iter()
        .map(|(g, paise)| (vouchers[g], g.clone(), paise))
        .collect()
}

/// Sum over every day of the period of max(-(ledger balance), 0): each event applies on its own
/// day, before that day counts (the reference's `capital_paise_days`).
fn capital_paise_days(opening_paise: i128, events: &[(i64, i128)], period: &Window) -> i128 {
    let (start, end) = (civil_day_number(&period.from), civil_day_number(&period.to));
    let mut balance = opening_paise;
    let mut idx = 0;
    let mut total = 0;
    for day in start..=end {
        while idx < events.len() && events[idx].0 <= day {
            balance += events[idx].1;
            idx += 1;
        }
        let economic = -balance;
        if economic > 0 {
            total += economic;
        }
    }
    total
}

/// `days_paise * rate_bp / (denom_days * 10000)`, rounded half-up, floored at 0.
fn interest_from_capital_days(days_paise: i128, rate_bp: i64, denom_days: i128) -> i128 {
    if days_paise <= 0 || rate_bp <= 0 {
        return 0;
    }
    round_half_up(days_paise * i128::from(rate_bp), denom_days * 10_000)
}

/// Voucher refs sorted by GUID, labelled.
fn voucher_refs(vs: &BTreeMap<String, &Voucher>) -> Vec<EvidenceRef> {
    vs.iter()
        .map(|(g, v)| EvidenceRef::with_label("voucher", g, &voucher_label(v)))
        .collect()
}

fn labels(vs: &BTreeMap<String, &Voucher>) -> String {
    vs.values()
        .map(|v| voucher_label(v))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The reference's `_shared_tds_note`: what the s.194T base may miss.
fn shared_tds_note(w: &Walk) -> Vec<String> {
    let mut notes = Vec::new();
    if !w.capital_set_off_vouchers.is_empty() {
        notes.push(format!(
            "{} interest or remuneration voucher(s) also debit this partner's capital ({}): if a \
debit is a withdrawal rather than TDS booked through the capital, this base is read net of it and \
may be understated (see the finding that s.40(b) is not computed).",
            w.capital_set_off_vouchers.len(),
            labels(&w.capital_set_off_vouchers)
        ));
    }
    if !w.shared_tds_vouchers.is_empty() {
        notes.push(format!(
            "{} interest or remuneration voucher(s) carry TDS but also credit another party ({}): \
their TDS is not added back, so the interest or remuneration counted may be understated by it.",
            w.shared_tds_vouchers.len(),
            labels(&w.shared_tds_vouchers)
        ));
    }
    if !w.mixed_unsplit_vouchers.is_empty() {
        let set_off = &w.mixed_set_off_vouchers;
        notes.push(format!(
            "{} voucher(s) carry both interest and remuneration and are not split ({}): see the \
s.40(b) not-computed finding. Their credit to this partner is counted here whole.{}",
            w.mixed_unsplit_vouchers.len(),
            labels(&w.mixed_unsplit_vouchers),
            if set_off.is_empty() {
                String::new()
            } else {
                format!(
                    " {} of them also debit this partner's capital: the credit is read net of that \
debit, so this base may be wrong in either direction.",
                    set_off.len()
                )
            }
        ));
    }
    notes
}

/// The reference's `_s40b_v_slab`: the s.40(b)(v) ceiling quoted from the rules, never a limit.
fn s40b_v_slab(rules: &Rules) -> String {
    let Some(t) = &rules.s40b_v else {
        return "The rules table holds no s.40(b)(v) slab.".to_string();
    };
    // i64 -> f64 is exact below 2^53 and the division correctly rounded, as Python's is.
    #[allow(clippy::cast_precision_loss)]
    let lakh = |p: i64| format!("₹{} lakh", py_format_g(p as f64 / 10_000_000_f64));
    #[allow(clippy::cast_precision_loss)]
    let pct = |bp: i64| py_format_g(bp as f64 / 100.0);
    format!(
        "The s.40(b)(v) ceiling ({}): on the first {} of book profit (or a loss), {} or {}% of the \
book profit, whichever is more; on the balance, {}%. It applies to the firm's book profit, which \
this test does not compute, so no limit is stated here.",
        t.authority,
        lakh(t.first_slab_paise),
        lakh(t.floor_paise),
        pct(t.first_slab_bp),
        pct(t.balance_bp)
    )
}

/// Run the test. `tds_ledgers` are the ledgers the client's statutory dues classify as TDS
/// payable: the s.194T finding reads the TDS on a partner's vouchers through them only.
pub fn run(
    book: &Book,
    rules: &Rules,
    period: &Window,
    entity_type: &str,
    cfg: &PartnersConfig,
    tds_ledgers: &BTreeSet<String>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let s40b_rate = rules.s40b_interest_rate_bp(entity_type)?;
    let s194t_on = rules.s194t_applies(entity_type)?;
    let applicable = s194t_on || s40b_rate > 0;
    r.fig(
        "applicable",
        Value::Text(if applicable { "yes" } else { "no" }.to_string()),
        Unit::Text,
        "Whether s.40(b)/s.194T apply to this engagement: read from rules.entity(...).\
s40b_interest_rate_bp and rules.entity(...).s194t for this engagement's entity_type -- never from \
the entity_type string itself. 'no' when both are absent/zero/false.",
        Vec::new(),
    );
    if !applicable {
        return Ok(r);
    }

    let pop = book.population()?;
    r.population_note = "Books population (optional, cancelled and post-dated vouchers excluded). \
A capital-ledger line is excluded from the daily-balance walk, and counted as interest/remuneration \
credited instead, iff that SAME voucher also carries a line on the partner's own interest or \
remuneration ledger -- identified from the voucher's other lines, never from a date, number or \
narration."
        .to_string();

    let (partners, deed_rate) = partners_config(cfg)?;
    // Each partner's figures are tagged `hash8(key)`, as the reference tags them, so two keys whose
    // tags coincide would name one figure twice. The reference raises there (its `fig` refuses a
    // duplicate id); refuse the same way before any partner figure is built, never panic in `fig`.
    let mut tags: BTreeMap<String, &str> = BTreeMap::new();
    for key in partners.keys() {
        let tag = hash8(key);
        if let Some(first) = tags.insert(tag.clone(), key) {
            return Err(AuditError::Config(format!(
                "{TEST_ID}: [partners].{first} and [partners].{key} share the figure tag {tag}; \
the reference refuses a duplicate figure id"
            )));
        }
    }
    let deed_missing = deed_rate.is_none();
    let rate_bp = deed_rate.map_or(s40b_rate, |d| d.min(s40b_rate));
    let (tds_rate_bp, tds_limit_paise, s194t_is_default) = match rules.s194t {
        Some(t) => (t.rate_bp, t.limit_paise, false),
        None => (DEFAULT_S194T_RATE_BP, DEFAULT_S194T_LIMIT_PAISE, true),
    };

    let f_rate = r.fig(
        "interest_rate_bp_used",
        Value::Int(rate_bp),
        Unit::BasisPoints,
        &match deed_rate {
            Some(d) => format!(
                "Rate used for s.40(b) allowable interest: min(deed interest_rate_bp {d}, rules \
s40b_interest_rate_bp {s40b_rate})."
            ),
            None => format!(
                "Deed interest rate not available; rules.s40b_interest_rate_bp used as an upper \
bound ({s40b_rate} bp), not a confirmed authorised rate."
            ),
        },
        Vec::new(),
    );
    if deed_missing {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/deed_missing"),
            clauses: vec!["s.40(b)".to_string(), "3CD-21(c)".to_string()],
            title: "Partnership deed not received; s.40(b) interest rate assumed at the statutory \
cap"
            .to_string(),
            facts: vec![("rate_used".to_string(), f_rate)],
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![format!(
                "s.40(b) allows interest only as authorised by, and not exceeding the amount \
specified in, the partnership deed; without the deed the actual authorised rate is unknown, so the \
statutory cap ({s40b_rate} bp) is used here as an upper bound on allowable interest, not a \
confirmed rate."
            )],
            ask_client: vec![
                "Provide the partnership deed (and any supplementary deed in force \
during the year) showing the authorised interest rate and remuneration clause."
                    .to_string(),
            ],
        });
    }

    let int = |x: i128| -> Result<Value> {
        i64::try_from(x)
            .map(Value::Int)
            .map_err(|_| overflow(TEST_ID))
    };
    let mut remuneration_facts: Vec<(String, String)> = Vec::new();
    let mut remuneration_notes: Vec<String> = Vec::new();
    let mut remuneration_evidence: BTreeMap<String, &Voucher> = BTreeMap::new();
    let mut excess_total: i128 = 0;
    let mut credited_total: i128 = 0;
    let mut not_computed_count = 0_usize;
    // A ledger configured as the interest or remuneration ledger of more than one partner.
    let mut ledger_uses: BTreeMap<&str, usize> = BTreeMap::new();
    for q in partners.values() {
        let own: BTreeSet<&str> = [
            q.interest_ledger.as_deref(),
            q.remuneration_ledger.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect();
        for led in own {
            *ledger_uses.entry(led).or_insert(0) += 1;
        }
    }
    let shared_ledgers: BTreeSet<&str> = ledger_uses
        .iter()
        .filter(|(_, n)| **n > 1)
        .map(|(l, _)| *l)
        .collect();
    let all_capitals: BTreeSet<String> = partners
        .values()
        .flat_map(|q| q.capital_ledgers.iter().cloned())
        .collect();

    for (key, p) in &partners {
        let h = hash8(key);
        let w = walk(&pop, book, p, tds_ledgers, &all_capitals);
        let days_primary = capital_paise_days(w.opening_paise, &w.events, period);
        let days_opening_only = capital_paise_days(w.opening_paise, &[], period);
        let reductions_skipped: Vec<(i64, i128)> =
            w.events.iter().copied().filter(|e| e.1 < 0).collect();
        let days_no_reduction = capital_paise_days(w.opening_paise, &reductions_skipped, period);
        let allowable_365 = interest_from_capital_days(days_primary, rate_bp, 365);
        let allowable_360 = interest_from_capital_days(days_primary, rate_bp, 360);
        let allowable_opening_only = interest_from_capital_days(days_opening_only, rate_bp, 365);
        let allowable_no_reduction = interest_from_capital_days(days_no_reduction, rate_bp, 365);
        let credited = w.interest_credited_paise;
        let excess = (credited - allowable_365).max(0);
        // A partner whose interest cannot be read exactly has s.40(b) NOT COMPUTED, and said so.
        let mut unknown: BTreeMap<String, &Voucher> = w.mixed_unsplit_vouchers.clone();
        unknown.extend(w.shared_tds_interest_vouchers.clone());
        unknown.extend(w.off_capital_interest_vouchers.clone());
        unknown.extend(w.capital_set_off_vouchers.clone());
        let unusable = if unknown.is_empty() {
            ""
        } else {
            " Not usable for s.40(b): s.40(b) is not computed for this partner (see the finding \
that says so)."
        };

        let ev_capital: Vec<EvidenceRef> = p
            .capital_ledgers
            .iter()
            .map(|n| EvidenceRef::new("ledger", n))
            .collect();
        r.fig(
            &format!("capital_opening_{h}"),
            int(w.opening_paise)?,
            Unit::Paise,
            &format!("Opening (1-4) balance of partner (tag {h})'s capital ledger(s), Dr+/Cr-."),
            ev_capital.clone(),
        );
        let f_allow = r.fig(
            &format!("allowable_interest_{h}"),
            int(allowable_365)?,
            Unit::Paise,
            &format!(
                "s.40(b) allowable interest for partner (tag {h}): simple interest at {rate_bp} bp \
on the capital balance walked day by day (opening TB balance, then every population voucher line \
on the capital ledger(s) other than an interest/remuneration credit, changing the balance ON its \
date), actual days / 365, a debit (negative) capital day contributing zero.{unusable}"
            ),
            ev_capital.clone(),
        );
        r.fig(
            &format!("allowable_interest_sensitivity_360day_{h}"),
            int(allowable_360)?,
            Unit::Paise,
            &format!(
                "Same daily-balance walk for partner (tag {h}), but actual days / 360.{unusable}"
            ),
            ev_capital.clone(),
        );
        r.fig(
            &format!("allowable_interest_sensitivity_opening_only_{h}"),
            int(allowable_opening_only)?,
            Unit::Paise,
            &format!(
                "s.40(b) interest for partner (tag {h}) as if the capital balance stayed at its \
opening value for the whole year (no intra-year voucher line applied at all).{unusable}"
            ),
            ev_capital.clone(),
        );
        r.fig(
            &format!("allowable_interest_sensitivity_no_reduction_{h}"),
            int(allowable_no_reduction)?,
            Unit::Paise,
            &format!(
                "s.40(b) interest for partner (tag {h}) as if capital were never reduced by a \
withdrawal or transfer (every debit line on the capital ledger(s) skipped; a credit/increase line, \
e.g. capital introduced, still applied).{unusable}"
            ),
            ev_capital,
        );

        let ev_interest_v = voucher_refs(&w.interest_vouchers);
        let f_credited = r.fig(
            &format!("interest_credited_{h}"),
            int(credited)?,
            Unit::Paise,
            &format!(
                "Interest actually credited to partner (tag {h})'s capital ledger(s), in a voucher \
also carrying a line on their interest_ledger.{unusable}"
            ),
            ev_interest_v.clone(),
        );
        credited_total += credited;
        let off = &w.off_capital_vouchers;
        // The listed vouchers on a ledger shared by partners, never the shared ledgers alone.
        let on_shared: BTreeSet<&str> = [
            p.interest_ledger.as_deref(),
            p.remuneration_ledger.as_deref(),
        ]
        .into_iter()
        .flatten()
        .filter(|l| shared_ledgers.contains(l))
        .collect();
        let off_shared = off
            .values()
            .filter(|v| {
                v.lines
                    .iter()
                    .any(|l| on_shared.contains(l.ledger.as_str()) && l.amount_paise != 0)
            })
            .count();
        if !off.is_empty() {
            let mut limits = vec![format!(
                "{} voucher(s) post to this partner's interest or remuneration ledger without \
touching any partner's capital ({}): paid by bank or cash, credited to a payable, or a reclass -- \
the books do not say which. They are not counted in the interest or remuneration credited, or in \
the s.194T base; any that is a payment or credit to this partner is missing from them.",
                off.len(),
                labels(off)
            )];
            if off_shared > 0 {
                limits.push(format!(
                    "{off_shared} of them {} on a ledger shared by partners, so whose each of those \
is is not judged either.",
                    if off_shared == 1 { "is" } else { "are" }
                ));
            }
            r.findings.push(Finding {
                id: format!("{TEST_ID}/off_capital/{h}"),
                clauses: vec!["s.194T".to_string(), "s.40(b)".to_string()],
                title: "Vouchers on a partner's interest or remuneration ledger that touch no \
partner's capital: not counted for this partner"
                    .to_string(),
                facts: Vec::new(),
                evidence: voucher_refs(off),
                confidence: Confidence::JudgementRequired,
                limits,
                ask_client: vec![
                    "For each voucher named, whether it is a payment or credit to a partner, and to \
which."
                        .to_string(),
                ],
            });
        }
        if unknown.is_empty() {
            let f_excess = r.fig(
                &format!("s40b_excess_{h}"),
                int(excess)?,
                Unit::Paise,
                &format!("interest_credited_{h} minus allowable_interest_{h}, floor 0."),
                Vec::new(),
            );
            excess_total += excess;
            if excess > 0 {
                r.findings.push(Finding {
                    id: format!("{TEST_ID}/s40b_excess/{h}"),
                    clauses: vec!["s.40(b)".to_string(), "3CD-21(c)".to_string()],
                    title: "Interest credited to a partner exceeds the s.40(b) allowable amount"
                        .to_string(),
                    facts: vec![
                        ("credited".to_string(), f_credited),
                        ("allowable".to_string(), f_allow),
                        ("excess".to_string(), f_excess),
                    ],
                    evidence: ev_interest_v,
                    confidence: if deed_missing {
                        Confidence::JudgementRequired
                    } else {
                        Confidence::Computed
                    },
                    limits: vec![if deed_missing {
                        "No deed was available; the excess shown uses the statutory cap as the \
assumed authorised rate -- confirm the deed's actual rate and terms before relying on this figure."
                            .to_string()
                    } else {
                        "Allowable interest here assumes the capital base is exactly the capital \
ledger(s) supplied and that interest was authorised for the whole year; confirm both against the \
deed."
                            .to_string()
                    }],
                    ask_client: vec![
                        "Confirm the deed's authorised interest rate and the capital \
base it applies to."
                            .to_string(),
                    ],
                });
            }
        } else {
            not_computed_count += 1;
            let mut reasons = Vec::new();
            if !w.mixed_unsplit_vouchers.is_empty() {
                reasons.push(format!(
                    "{} voucher(s) carry both the interest and the remuneration ledger and are not \
split ({}): each credits another ledger as well, debits this partner's capital, or reverses such a \
voucher. The books do not show which part is interest, so the interest counted may be wrong in \
either direction.",
                    w.mixed_unsplit_vouchers.len(),
                    labels(&w.mixed_unsplit_vouchers)
                ));
            }
            if !w.capital_set_off_vouchers.is_empty() {
                reasons.push(format!(
                    "{} interest or remuneration voucher(s) also debit this partner's capital ({}): \
the debit may be TDS booked through the capital or a withdrawal, which the books do not \
distinguish. It is kept out of the capital walk, so the allowable interest may be wrong; if it is \
a withdrawal, the interest counted and the s.194T base are read net of it and may be understated, \
even under the limit.",
                    w.capital_set_off_vouchers.len(),
                    labels(&w.capital_set_off_vouchers)
                ));
            }
            if !w.off_capital_interest_vouchers.is_empty() {
                reasons.push(format!(
                    "{} voucher(s) post to this partner's interest ledger without touching any \
partner's capital ({}): not counted in the interest credited, which may therefore be understated \
(see the off-capital finding).",
                    w.off_capital_interest_vouchers.len(),
                    labels(&w.off_capital_interest_vouchers)
                ));
            }
            if !w.shared_tds_interest_vouchers.is_empty() {
                reasons.push(format!(
                    "{} interest voucher(s) carry TDS but also credit another party ({}): the TDS \
is not divided between them, so the interest counted is net of an unknown part of it.",
                    w.shared_tds_interest_vouchers.len(),
                    labels(&w.shared_tds_interest_vouchers)
                ));
            }
            reasons.push(
                "No s.40(b) excess is stated for this partner, and none is listed in clause 21(c). \
Its interest credited and allowable-interest figures are marked not usable for s.40(b)."
                    .to_string(),
            );
            r.findings.push(Finding {
                id: format!("{TEST_ID}/s40b_not_computed/{h}"),
                clauses: vec!["s.40(b)".to_string(), "3CD-21(c)".to_string()],
                title: format!(
                    "s.40(b) not computed for partner (tag {h}): its interest credited cannot be \
read exactly; the CA computes it"
                ),
                facts: Vec::new(),
                evidence: voucher_refs(&unknown),
                confidence: Confidence::JudgementRequired,
                limits: reasons,
                ask_client: vec![
                    "The partner's interest for the year, with the interest and remuneration in \
each voucher named here and the TDS on each."
                        .to_string(),
                ],
            });
        }

        let f_rem = r.fig(
            &format!("remuneration_credited_{h}"),
            int(w.remuneration_credited_paise)?,
            Unit::Paise,
            &format!(
                "Remuneration credited to partner (tag {h}), in a voucher also carrying a line on \
their remuneration_ledger. The s.40(b)(v) book-profit limit is NOT computed here."
            ),
            voucher_refs(&w.remuneration_vouchers),
        );
        remuneration_facts.push((key.clone(), f_rem));
        let shared_rem = &w.shared_tds_remuneration_vouchers;
        if !shared_rem.is_empty() {
            remuneration_evidence.extend(shared_rem.clone());
            remuneration_notes.push(format!(
                "Partner (tag {h}): {} remuneration voucher(s) carry TDS but also credit another \
party ({}): the TDS is not divided between them, so remuneration_credited_{h} is net of an unknown \
part of it.",
                shared_rem.len(),
                labels(shared_rem)
            ));
        }

        // Gross: each voucher's net credit to the partner, a reversal lowering nothing.
        let credits = s194t_credits(&w);
        let subject_paise: i128 = credits.iter().map(|c| c.2).filter(|p| *p > 0).sum();
        let f_subject = r.fig(
            &format!("s194t_amount_credited_{h}"),
            int(subject_paise)?,
            Unit::Paise,
            &format!(
                "Interest + remuneration credited to partner (tag {h}) in the year, gross: each \
voucher's net credit to the partner, a reversal lowering nothing -- the s.194T base (salary, \
remuneration, commission, bonus and interest to a partner)."
            ),
            Vec::new(),
        );
        let tds_expected = round_half_up(subject_paise * i128::from(tds_rate_bp), 10_000);
        let f_tds = r.fig(
            &format!("s194t_tds_expected_{h}"),
            int(tds_expected)?,
            Unit::Paise,
            &format!("TDS @ {tds_rate_bp} bp on s194t_amount_credited_{h}."),
            Vec::new(),
        );
        // The remuneration voucher wins a GUID both maps hold, as `{**interest, **remuneration}`.
        let mut touched = w.interest_vouchers.clone();
        touched.extend(w.remuneration_vouchers.clone());
        // The TDS seen is on ANY voucher touching the partner (its capital, or its own interest or
        // remuneration ledger not shared by partners) with a line on a TDS-payable ledger.
        let ledgers: BTreeSet<&str> = [
            p.interest_ledger.as_deref(),
            p.remuneration_ledger.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect();
        let own: BTreeSet<&str> = p
            .capital_ledgers
            .iter()
            .map(String::as_str)
            .chain(
                ledgers
                    .iter()
                    .copied()
                    .filter(|l| !shared_ledgers.contains(l)),
            )
            .collect();
        let has_tds = |v: &Voucher| {
            v.lines
                .iter()
                .any(|l| tds_ledgers.contains(&l.ledger) && l.amount_paise != 0)
        };
        let mut seen: BTreeMap<String, &Voucher> = BTreeMap::new();
        let mut unattributed: BTreeMap<String, &Voucher> = BTreeMap::new();
        if !tds_ledgers.is_empty() {
            for &v in &pop {
                if v.lines.iter().any(|l| own.contains(l.ledger.as_str())) && has_tds(v) {
                    seen.insert(v.guid.clone(), v);
                }
            }
            for &v in &pop {
                if !seen.contains_key(&v.guid)
                    && has_tds(v)
                    && v.lines.iter().any(|l| {
                        ledgers.contains(l.ledger.as_str())
                            && shared_ledgers.contains(l.ledger.as_str())
                    })
                {
                    unattributed.insert(v.guid.clone(), v);
                }
            }
        }
        let tds_seen_paise: i128 = -seen
            .values()
            .flat_map(|v| v.lines.iter())
            .filter(|l| tds_ledgers.contains(&l.ledger))
            .map(|l| i128::from(l.amount_paise))
            .sum::<i128>();
        let seen_value = if tds_ledgers.is_empty() || (!unattributed.is_empty() && seen.is_empty())
        {
            "not judged"
        } else if seen.is_empty() {
            "no"
        } else {
            "yes"
        };
        r.fig(
            &format!("s194t_tds_ledger_seen_{h}"),
            Value::Text(seen_value.to_string()),
            Unit::Text,
            &format!(
                "Whether a line on a ledger the client's statutory dues classify as TDS payable \
appears on a voucher touching partner (tag {h})'s capital, interest or remuneration ledgers (\"not \
judged\" when none is classified, or when the only such lines are on a ledger shared by partners, \
off this partner's capital)."
            ),
            Vec::new(),
        );

        if subject_paise > i128::from(tds_limit_paise) {
            let mut limits = vec![
                "Books only: TAN registration, challans filed and any Form 26A \
route are not visible from vouchers."
                    .to_string(),
            ];
            limits.extend(shared_tds_note(&w));
            let mut reversals: Vec<(&Voucher, &String)> = credits
                .iter()
                .filter(|c| c.2 < 0)
                .map(|c| (c.0, &c.1))
                .collect();
            if !reversals.is_empty() {
                reversals.sort_by(|a, b| (&a.0.date, a.1).cmp(&(&b.0.date, b.1)));
                let net_paise = w.interest_credited_paise + w.remuneration_credited_paise;
                limits.push(format!(
                    "{} voucher(s) carry a negative net credit of interest and remuneration to this \
partner ({}): under the gross reading (owner, 25-Sep) they lower nothing, so this base ({}) \
exceeds the interest and remuneration credited net of them ({}).",
                    reversals.len(),
                    reversals
                        .iter()
                        .map(|(v, _)| voucher_label(v))
                        .collect::<Vec<_>>()
                        .join(", "),
                    rupees(subject_paise),
                    rupees(net_paise)
                ));
            }
            if s194t_is_default {
                limits.push(format!(
                    "The s.194T rate/limit used here (rate {tds_rate_bp} bp, limit \
{tds_limit_paise} paise) is a local prototype default (status=\"confirm\"), pending confirmation in \
the rules table -- not yet a verified rule."
                ));
            }
            let by_date = |vs: &BTreeMap<String, &Voucher>| -> Vec<String> {
                let mut ordered: Vec<(&String, &&Voucher)> = vs.iter().collect();
                ordered.sort_by(|a, b| (&a.1.date, a.0).cmp(&(&b.1.date, b.0)));
                ordered.into_iter().map(|(_, v)| voucher_label(v)).collect()
            };
            let first12 = |labels: &[String]| {
                format!(
                    "{}{}",
                    labels
                        .iter()
                        .take(12)
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", "),
                    if labels.len() > 12 {
                        format!(", and {} more", labels.len() - 12)
                    } else {
                        String::new()
                    }
                )
            };
            let not_judged =
                "Payments/credits to a partner over the s.194T threshold: whether TDS \
was deducted is not judged";
            let title = if tds_ledgers.is_empty() {
                limits.push(
                    "The client's statutory dues classify no ledger as TDS payable, so TDS on the \
partner's vouchers cannot be seen."
                        .to_string(),
                );
                not_judged.to_string()
            } else if !seen.is_empty() {
                limits.push(format!(
                    "TDS of {} (net) is seen on {} voucher(s) touching this partner: {}.",
                    rupees(tds_seen_paise),
                    seen.len(),
                    first12(&by_date(&seen))
                ));
                "Payments/credits to a partner over the s.194T threshold, with TDS seen on its \
vouchers: which credits it covers, and its deposit, are the CA's to determine"
                    .to_string()
            } else if !unattributed.is_empty() {
                not_judged.to_string()
            } else {
                "Payments/credits to a partner over the s.194T threshold with no TDS ledger line \
seen"
                    .to_string()
            };
            if !tds_ledgers.is_empty() && !unattributed.is_empty() {
                let l = by_date(&unattributed);
                limits.push(format!(
                    "TDS lines are seen on {} voucher(s) on an interest or remuneration ledger \
shared by partners that do not touch this partner's capital: {}. Which partner's TDS they are is \
not judged.",
                    l.len(),
                    first12(&l)
                ));
            }
            let mut evidence = touched.clone();
            evidence.extend(seen.clone());
            r.findings.push(Finding {
                id: format!("{TEST_ID}/s194t/{h}"),
                clauses: vec![
                    "s.194T".to_string(),
                    "3CD-34(a)".to_string(),
                    "3CD-34(c)".to_string(),
                ],
                title,
                facts: vec![
                    ("credited".to_string(), f_subject),
                    ("tds_expected".to_string(), f_tds),
                ],
                evidence: voucher_refs(&evidence),
                confidence: if seen.is_empty() {
                    Confidence::NeedsDocument
                } else {
                    Confidence::JudgementRequired
                },
                limits,
                ask_client: vec![
                    "Confirm the firm's TAN and whether TDS under s.194T was deposited (this is \
not visible from the books shown here if it went through a ledger not captured in \
touched_vouchers)."
                        .to_string(),
                    "If not deducted, confirm whether Form 26A relief is available (partner \
returned the income, tax paid)."
                        .to_string(),
                ],
            });
        }
    }

    r.fig(
        "s40b_excess_total",
        int(excess_total)?,
        Unit::Paise,
        &if not_computed_count == 0 {
            "Sum of s40b_excess_<partner> across all partners.".to_string()
        } else {
            format!(
                "Sum of s40b_excess_<partner> across the partners for whom it is computed: \
{not_computed_count} partner(s) not computed (their s40b_not_computed findings) are not in it."
            )
        },
        Vec::new(),
    );
    r.fig(
        "s194t_interest_credited_total",
        int(credited_total)?,
        Unit::Paise,
        "Sum of interest_credited_<partner> across all partners -- cross-check this against a \
shared interest_ledger's own TB movement when every partner uses the same ledger name.",
        Vec::new(),
    );

    if !remuneration_facts.is_empty() {
        let mut limits = vec![
            "The s.40(b)(v) remuneration ceiling is a slab on 'book profit' \
(s.28-44D profit plus remuneration debited, per Explanation 3), which this test does not compute; \
whether remuneration is authorised and quantified by the deed, and from what date, is a deed \
reading, not a books fact."
                .to_string(),
            s40b_v_slab(rules),
        ];
        limits.extend(remuneration_notes);
        r.findings.push(Finding {
            id: format!("{TEST_ID}/remuneration_book_profit_required"),
            clauses: vec!["s.40(b)(v)".to_string(), "3CD-21(c)".to_string()],
            title: "Remuneration to working partners: the s.40(b)(v) limit depends on book \
profit, not computed here"
                .to_string(),
            facts: remuneration_facts,
            evidence: voucher_refs(&remuneration_evidence),
            confidence: Confidence::JudgementRequired,
            limits,
            ask_client: vec![
                "Provide the computation of book profit under s.40(b) Explanation \
3, and the deed clause authorising and quantifying remuneration."
                    .to_string(),
            ],
        });
    }
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{LedgerLine, VoucherStatus};
    use bridge_tally_primitives::TallyDate;

    fn cfg(text: &str) -> PartnersConfig {
        let mut partners: BTreeMap<String, toml::Value> = toml::from_str::<toml::Table>(text)
            .unwrap()
            .into_iter()
            .collect();
        let deed = partners.remove("deed");
        PartnersConfig { partners, deed }
    }

    fn year() -> Window {
        Window {
            from: TallyDate::parse("20250401".to_string()).unwrap(),
            to: TallyDate::parse("20260331".to_string()).unwrap(),
        }
    }

    /// A book whose one voucher has an unknown status: the population refuses it.
    fn unreadable_book() -> Book {
        Book {
            company_name: "Invented".to_string(),
            company_guid: "invented".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: BTreeMap::new(),
            vouchers: vec![Voucher {
                guid: "g-1".to_string(),
                date: TallyDate::parse("20250501".to_string()).unwrap(),
                vtype: "Journal".to_string(),
                base_type: "Journal".to_string(),
                number: "1".to_string(),
                status: VoucherStatus::Unknown,
                lines: vec![LedgerLine {
                    ledger: "A Capital".to_string(),
                    amount_paise: -1,
                }],
                narration: String::new(),
                party_field: String::new(),
                masterid: None,
                inventory: Vec::new(),
                ..Default::default()
            }],
            tb: BTreeMap::new(),
            ..Default::default()
        }
    }

    #[test]
    fn the_partner_tag_is_the_references() {
        // As the reference's golden names partner_a: sha1("partner_a")[:8].
        assert_eq!(hash8("partner_a"), "d0893259");
    }

    #[test]
    fn a_partner_or_deed_the_reference_cannot_read_is_refused() {
        for text in [
            "[a]\ninterest_ledger = \"I\"\n",
            "deed = 5\n[a]\ncapital_ledgers = [\"A\"]\n",
            "[a]\ncapital_ledgers = [\"A\"]\n[deed]\ninterest_rate_bp = \"12%\"\n",
        ] {
            assert!(
                matches!(partners_config(&cfg(text)), Err(AuditError::Config(_))),
                "{text}"
            );
        }
        let (partners, rate) = partners_config(&cfg(
            "[a]\ncapital_ledgers = [\"A\", \"A\"]\n[deed]\ninterest_rate_bp = 1000\n",
        ))
        .unwrap();
        assert_eq!(partners["a"].capital_ledgers, ["A", "A"]);
        assert_eq!(rate, Some(1000));
    }

    #[test]
    fn two_partner_keys_sharing_a_figure_tag_are_refused_not_a_panic() {
        // SHA-1 of "p30395" and of "p89343" both begin 47ff8a3d.
        assert_eq!(hash8("p30395"), hash8("p89343"));
        let book = Book {
            vouchers: Vec::new(),
            ..unreadable_book()
        };
        let rules = Rules::vendored().unwrap();
        let two = "[p30395]\ncapital_ledgers = [\"A Capital\"]\n\
[p89343]\ncapital_ledgers = [\"B Capital\"]\n";
        match run(&book, &rules, &year(), "firm", &cfg(two), &BTreeSet::new()) {
            Err(AuditError::Config(m)) => {
                assert!(m.contains("[partners].p30395 and [partners].p89343"), "{m}");
                assert!(m.contains("47ff8a3d"), "{m}");
            }
            other => panic!("expected a Config refusal, got {other:?}"),
        }
        // Either key alone runs.
        let one = "[p30395]\ncapital_ledgers = [\"A Capital\"]\n";
        assert!(run(&book, &rules, &year(), "firm", &cfg(one), &BTreeSet::new()).is_ok());
    }

    #[test]
    fn not_applicable_reads_no_voucher_and_rules_without_entities_refuse() {
        let rules = Rules::vendored().unwrap();
        let r = run(
            &unreadable_book(),
            &rules,
            &year(),
            "company",
            &cfg(""),
            &BTreeSet::new(),
        )
        .unwrap();
        assert_eq!(r.figures.len(), 1);
        // A firm reads the population, which refuses the unknown status.
        assert!(run(
            &unreadable_book(),
            &rules,
            &year(),
            "firm",
            &cfg(""),
            &BTreeSet::new()
        )
        .is_err());
        // Either key alone makes the test apply (it then reads the population, which refuses).
        let only = |s40b: Option<i64>, s194t: Option<bool>| Rules {
            entity: Some(BTreeMap::from([(
                "x".to_string(),
                crate::rules::EntityRules {
                    s40b_interest_rate_bp: s40b,
                    s194t,
                },
            )])),
            ..rules.clone()
        };
        for r in [only(Some(1200), None), only(None, Some(true))] {
            assert!(run(
                &unreadable_book(),
                &r,
                &year(),
                "x",
                &cfg(""),
                &BTreeSet::new()
            )
            .is_err());
        }
        let neither = only(Some(0), Some(false));
        assert_eq!(
            run(
                &unreadable_book(),
                &neither,
                &year(),
                "x",
                &cfg(""),
                &BTreeSet::new()
            )
            .unwrap()
            .figures
            .len(),
            1
        );
        let without = Rules {
            entity: None,
            ..rules
        };
        assert!(matches!(
            run(
                &unreadable_book(),
                &without,
                &year(),
                "company",
                &cfg(""),
                &BTreeSet::new()
            ),
            Err(AuditError::Config(_))
        ));
    }
}
