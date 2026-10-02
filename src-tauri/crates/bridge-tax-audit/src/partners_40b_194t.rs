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
//! capital, are named (the latter counted nowhere). Only a non-zero line touches a ledger. A
//! voucher on the partner's capital carrying its own interest or remuneration ledger, or another
//! partner's, is read exactly only in the forward plain shape (its own ledger debited, its own
//! capital credited, TDS credited on a TDS-payable ledger, balanced); any other is named, as is an
//! interest or remuneration voucher that also credits a ledger this test does not read. A partner
//! whose interest cannot be read exactly, or who has no interest ledger, has s.40(b) not computed,
//! never an excess. The s.194T base is gross: a reversal lowers nothing, and the s.194T question
//! is raised under the limit whenever the base may be wrong. The s.40(b)(v) remuneration ceiling
//! is quoted from the rules' `[s40b_v]`, never applied as a limit. `Walk` and `run` say how each
//! is read.
//!
//! The closing rule ([`S40B_EXCESS_COMPUTED`] = false): the s.40(b) excess is computed for NO
//! partner. Every partner of a firm or LLP gets the not-computed question, with the reasons and
//! vouchers this test can name; no excess figure or finding is produced, and the firm total is
//! withheld (`s40b_excess_total_not_computed` stands in its place). The computation stays, behind
//! the switch, reached by the tests through [`run_with`] instead of rebinding a module global.
//!
//! `[partners.deed] no_interest_authorised = true` records a deed that authorises no interest on
//! capital: the rate used is 0 and no deed-rate question is raised; the partner is never computed,
//! whatever the switch, and its finding says any interest paid or credited to it, through any
//! ledger, is disallowed in full, that this test does not look for it, and states no amount. For a
//! partner with no interest ledger and no voucher this test cannot read exactly, it asks the client
//! nothing about interest for s.40(b); its s.194T question, which applies whether or not interest
//! is allowed, still asks for the interest and remuneration credited. The flag is refused beside
//! any `interest_rate_bp` (0 included) and in any form but a boolean; `false` is the same as
//! absent.
//!
//! The s.194T TDS seen is a non-zero line on a TDS-payable ledger, on a voucher with a non-zero line
//! on the partner's capital or on its own interest or remuneration ledger not shared by partners.
//! TDS on a voucher on a shared ledger is listed for every sharing partner, attributed to no one,
//! unless its TDS side (net-signed, apart from the TDS) is partners' capitals alone; a net-nil TDS
//! is theirs when it touches a partner's capital.
//!
//! The configuration is refused, never read, where it could mislead (the reference's
//! `check_partners_config`, run after the applicability gate and before any voucher is read): a
//! deed that is not a table, a deed rate that is not a whole number of basis points 0 or more, the
//! flag above, a capital-ledger list that is empty, names "" or one ledger twice, a capital ledger
//! shared by two partners, a ledger not in the books, one ledger as both a partner's interest and
//! remuneration ledger, and an interest or remuneration ledger that is a partner's capital or a
//! TDS ledger. Where several are wrong, which is reported first can differ from the reference:
//! partners are visited in key order, where the reference visits them in the file's. A refusal's
//! text quotes the bad value as TOML prints it (`"yes"`, `true`), where the reference prints its
//! Python repr (`'yes'`, `True`); a missing `capital_ledgers` or a non-text ledger name is refused
//! by the reader with its own wording, before the checks above.
//!
//! Every amount is carried in i128 (a year of capital-paise-days times a rate exceeds i64) and
//! each figure is checked back into i64.

use std::collections::{BTreeMap, BTreeSet};

use sha1::{Digest, Sha1};

use crate::book::{Book, Voucher};
use crate::depreciation::civil_day_number;
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::ledger_ids::stable_ledger_tag;
use crate::read::Window;
use crate::rules::Rules;
use crate::support::{overflow, py_repr_str, rupees, voucher_label};
use crate::tds_payees::py_format_g;
use crate::PartnersConfig;

pub const TEST_ID: &str = "partners_40b_194t";
pub const VERSION: &str = "1";

/// The reference's `S40B_EXCESS_COMPUTED`: Lane D2's closing rule (28-Sep) computes the s.40(b)
/// excess for no partner, the computation kept behind this switch (see the module docs).
pub const S40B_EXCESS_COMPUTED: bool = false;

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

/// `[partners.deed]`, typed: the interest rate it authorises, and whether it records that the deed
/// authorises no interest on capital (`false` when the key is absent).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deed {
    pub interest_rate_bp: Option<i64>,
    pub no_interest_authorised: bool,
}

/// The partners and the deed, typed from the bound `[partners]` table. The deed is checked first,
/// as the reference's `check_partners_config` checks it: a table, its `interest_rate_bp` a whole
/// number of basis points 0 or more (a float, a boolean, text or a negative number is refused),
/// its `no_interest_authorised` a boolean, never `true` beside an `interest_rate_bp`. A partner
/// that is not a table or has no `capital_ledgers` list of names is refused; the list's contents
/// and the ledgers themselves are checked by [`check_partners_config`], against the book.
pub fn partners_config(cfg: &PartnersConfig) -> Result<(BTreeMap<String, Partner>, Option<Deed>)> {
    let deed = match &cfg.deed {
        None => None,
        Some(deed) => {
            let t = deed.as_table().ok_or_else(|| {
                AuditError::Config(format!(
                    "{TEST_ID}: [partners.deed] must be a table; got {deed}"
                ))
            })?;
            let interest_rate_bp = match t.get("interest_rate_bp") {
                None => None,
                Some(v) => match v.as_integer() {
                    Some(rate) if rate >= 0 => Some(rate),
                    _ => {
                        return Err(AuditError::Config(format!(
                            "{TEST_ID}: [partners.deed].interest_rate_bp must be a whole number \
of basis points, 0 or more; got {v}"
                        )))
                    }
                },
            };
            let no_interest_authorised = match t.get("no_interest_authorised") {
                None => false,
                Some(v) => v.as_bool().ok_or_else(|| {
                    AuditError::Config(format!(
                        "{TEST_ID}: [partners.deed].no_interest_authorised must be true or false; \
got {v}"
                    ))
                })?,
            };
            if no_interest_authorised && interest_rate_bp.is_some() {
                return Err(AuditError::Config(format!(
                    "{TEST_ID}: [partners.deed] sets no_interest_authorised and an \
interest_rate_bp; the deed either authorises no interest or authorises a rate"
                )));
            }
            Some(Deed {
                interest_rate_bp,
                no_interest_authorised,
            })
        }
    };
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
    Ok((partners, deed))
}

/// The reference's `check_partners_config`, for a firm or LLP (after the applicability gate,
/// before any voucher is read): each partner's `capital_ledgers` a non-empty list of distinct,
/// non-empty names, none shared with another partner; every ledger named in the books, by its
/// exact name; the interest and remuneration ledgers never the same ledger; and neither ever a
/// partner's capital ledger or a TDS ledger. Refused, never read, otherwise.
fn check_partners_config(
    partners: &BTreeMap<String, Partner>,
    book: &Book,
    tds_ledgers: &BTreeSet<String>,
) -> Result<()> {
    let refuse = |m: String| AuditError::Config(format!("{TEST_ID}: {m}"));
    let mut seen_caps: BTreeMap<&str, &str> = BTreeMap::new();
    for (key, p) in partners {
        let caps = &p.capital_ledgers;
        let distinct: BTreeSet<&str> = caps.iter().map(String::as_str).collect();
        if caps.is_empty() || caps.iter().any(String::is_empty) || distinct.len() != caps.len() {
            let listed: Vec<String> = caps.iter().map(|c| py_repr_str(c)).collect();
            return Err(refuse(format!(
                "[partners.{key}].capital_ledgers must be a non-empty list of distinct ledger \
names; got [{}]",
                listed.join(", ")
            )));
        }
        for c in caps {
            if let Some(other) = seen_caps.insert(c.as_str(), key.as_str()) {
                return Err(refuse(format!(
                    "[partners.{key}].capital_ledgers names {}, which is also partner {}'s capital \
ledger",
                    py_repr_str(c),
                    py_repr_str(other)
                )));
            }
        }
        for led in caps
            .iter()
            .chain(p.interest_ledger.iter())
            .chain(p.remuneration_ledger.iter())
        {
            if !book.ledgers.contains_key(led) {
                return Err(refuse(format!(
                    "[partners.{key}] names ledger {}, which is not in the books",
                    py_repr_str(led)
                )));
            }
        }
        if let (Some(il), Some(rl)) = (&p.interest_ledger, &p.remuneration_ledger) {
            if il == rl {
                return Err(refuse(format!(
                    "[partners.{key}] names {} as both its interest and its remuneration ledger",
                    py_repr_str(il)
                )));
            }
        }
    }
    for (key, p) in partners {
        for (role, led) in [
            ("interest_ledger", &p.interest_ledger),
            ("remuneration_ledger", &p.remuneration_ledger),
        ] {
            if let Some(led) = led {
                if seen_caps.contains_key(led.as_str()) || tds_ledgers.contains(led) {
                    return Err(refuse(format!(
                        "[partners.{key}].{role} {} is a partner's capital ledger or a TDS ledger",
                        py_repr_str(led)
                    )));
                }
            }
        }
    }
    Ok(())
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
/// counted nowhere and named. Only a non-zero line touches a ledger.
///
/// The plain-voucher gate: a voucher on this partner's capital carrying its own interest or
/// remuneration ledger, or any other partner's, is read exactly only in the forward plain shape --
/// its own interest or remuneration ledger debited, its own capital credited, TDS credited on a
/// TDS-payable ledger, balanced, every line non-zero. Any other is `nonplain_vouchers` (and
/// `nonplain_interest_vouchers` when it carries this partner's interest ledger or another
/// partner's ledger). An interest or remuneration voucher that also credits a ledger this test
/// does not read (no partner's capital, no TDS-payable ledger, neither of this partner's own
/// ledgers) is `unread_credit_vouchers`, with those ledgers.
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
    unread_credit_vouchers: BTreeMap<String, (&'a Voucher, BTreeSet<String>)>,
    unread_credit_interest_vouchers: Vouchers<'a>,
    nonplain_vouchers: Vouchers<'a>,
    nonplain_interest_vouchers: Vouchers<'a>,
}

fn walk<'a>(
    pop: &[&'a Voucher],
    book: &Book,
    p: &Partner,
    tds_ledgers: &BTreeSet<String>,
    all_capitals: &BTreeSet<String>,
    all_int_rem: &BTreeSet<String>,
) -> Walk<'a> {
    let capital: BTreeSet<&str> = p.capital_ledgers.iter().map(String::as_str).collect();
    // The reference's sum over the list; `check_partners_config` refuses a ledger named twice.
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
        unread_credit_vouchers: BTreeMap::new(),
        unread_credit_interest_vouchers: BTreeMap::new(),
        nonplain_vouchers: BTreeMap::new(),
        nonplain_interest_vouchers: BTreeMap::new(),
    };
    let interest = p.interest_ledger.as_deref();
    let remuneration = p.remuneration_ledger.as_deref();
    let own_ledgers: BTreeSet<&str> = [interest, remuneration].into_iter().flatten().collect();
    for &v in pop {
        let guid = || v.guid.clone();
        // The capital lines' amounts, and the TDS added back as one more line where it applies.
        // Only a non-zero line touches a ledger (a zero capital line let TDS be added back to it).
        let mut lines_here: Vec<i128> = v
            .lines
            .iter()
            .filter(|l| capital.contains(l.ledger.as_str()) && l.amount_paise != 0)
            .map(|l| i128::from(l.amount_paise))
            .collect();
        if lines_here.is_empty() {
            let any_capital = |n: &str| capital.contains(n) || all_capitals.contains(n);
            let touches_capital = v
                .lines
                .iter()
                .any(|l| any_capital(l.ledger.as_str()) && l.amount_paise != 0);
            // Another partner's capital on the voucher leaves it out only where the side opposite
            // this partner's interest and remuneration lines, apart from TDS, is partners' capitals
            // alone (the shared-TDS rule's test for whose TDS it is). A bank, a payable, or nothing
            // but TDS on that side may be this partner's payment: listed. Lines netting to nil on
            // those ledgers leave no side: the rule before.
            let own_net: i128 = v
                .lines
                .iter()
                .filter(|l| own_ledgers.contains(l.ledger.as_str()))
                .map(|l| i128::from(l.amount_paise))
                .sum();
            let side: BTreeSet<&str> = v
                .lines
                .iter()
                .filter(|l| {
                    i128::from(l.amount_paise).signum() * own_net.signum() < 0
                        && !tds_ledgers.contains(&l.ledger)
                })
                .map(|l| l.ledger.as_str())
                .collect();
            let partners_alone =
                own_net == 0 || (!side.is_empty() && side.iter().all(|n| any_capital(n)));
            if !(touches_capital && partners_alone) {
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
        // A zero line on the interest or remuneration ledger makes no such voucher.
        let carries = |ledger: Option<&str>| {
            ledger.is_some_and(|n| v.lines.iter().any(|l| l.ledger == n && l.amount_paise != 0))
        };
        let is_interest = carries(interest);
        let is_remuneration = carries(remuneration);
        // The plain-voucher gate (see `Walk`). Another partner's interest or remuneration ledger
        // never passes it: `check_partners_config` refuses one that is a capital or TDS ledger.
        let nz = || v.lines.iter().filter(|l| l.amount_paise != 0);
        let other_touch = nz()
            .any(|l| all_int_rem.contains(&l.ledger) && !own_ledgers.contains(l.ledger.as_str()));
        if is_interest || is_remuneration || other_touch {
            let plain = (is_interest || is_remuneration)
                && nz().map(|l| i128::from(l.amount_paise)).sum::<i128>() == 0
                && nz().all(|l| {
                    (own_ledgers.contains(l.ledger.as_str()) && l.amount_paise > 0)
                        || ((capital.contains(l.ledger.as_str())
                            || tds_ledgers.contains(&l.ledger))
                            && l.amount_paise < 0)
                });
            if !plain {
                w.nonplain_vouchers.insert(guid(), v);
                if is_interest || other_touch {
                    w.nonplain_interest_vouchers.insert(guid(), v);
                }
            }
        }
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
        // An interest or remuneration voucher that also credits a ledger this test does not read
        // credits the partner net of it: named, with the ledgers, never a smaller computed excess.
        let unread: BTreeSet<String> = v
            .lines
            .iter()
            .filter(|l| {
                l.amount_paise < 0
                    && !capital.contains(l.ledger.as_str())
                    && !all_capitals.contains(&l.ledger)
                    && !tds_ledgers.contains(&l.ledger)
                    && Some(l.ledger.as_str()) != interest
                    && Some(l.ledger.as_str()) != remuneration
            })
            .map(|l| l.ledger.clone())
            .collect();
        if !unread.is_empty() && (is_interest || is_remuneration) {
            if is_interest {
                w.unread_credit_interest_vouchers.insert(guid(), v);
            }
            w.unread_credit_vouchers.insert(guid(), (v, unread));
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

/// [`labels`] for a map whose values carry their voucher first.
fn labels_of<T>(vs: &BTreeMap<String, (&Voucher, T)>) -> String {
    vs.values()
        .map(|(v, _)| voucher_label(v))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The reference's `_shared_tds_note`: what the s.194T base may miss.
fn shared_tds_note(w: &Walk) -> Vec<String> {
    let mut notes = Vec::new();
    if !w.unread_credit_vouchers.is_empty() {
        notes.push(format!(
            "{} interest or remuneration voucher(s) also credit a ledger this test does not read \
({}): the partner may be credited net of it, so this base may be understated.",
            w.unread_credit_vouchers.len(),
            labels_of(&w.unread_credit_vouchers)
        ));
    }
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

/// Run the test under the closing rule in force ([`S40B_EXCESS_COMPUTED`]). `tds_ledgers` are the
/// ledgers the client's statutory dues classify as TDS payable: the s.194T finding reads the TDS on
/// a partner's vouchers through them only.
pub fn run(
    book: &Book,
    rules: &Rules,
    period: &Window,
    entity_type: &str,
    cfg: &PartnersConfig,
    tds_ledgers: &BTreeSet<String>,
) -> Result<TestResult> {
    run_with(
        book,
        rules,
        period,
        entity_type,
        cfg,
        tds_ledgers,
        S40B_EXCESS_COMPUTED,
    )
}

/// [`run`], with the closing rule's switch given: `excess_computed` true reaches the s.40(b)
/// computation the reference keeps behind `S40B_EXCESS_COMPUTED`.
#[allow(clippy::too_many_lines)] // one section per figure and finding, as the reference lays them out
pub fn run_with(
    book: &Book,
    rules: &Rules,
    period: &Window,
    entity_type: &str,
    cfg: &PartnersConfig,
    tds_ledgers: &BTreeSet<String>,
    excess_computed: bool,
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
    )?;
    if !applicable {
        return Ok(r);
    }

    // The configuration is refused, never read, where it could mislead: before any voucher.
    let (partners, deed) = partners_config(cfg)?;
    check_partners_config(&partners, book, tds_ledgers)?;
    let pop = book.population()?;
    r.population_note = "Books population (optional, cancelled and post-dated vouchers excluded). \
A capital-ledger line is excluded from the daily-balance walk, and counted as interest/remuneration \
credited instead, iff that SAME voucher also carries a line on the partner's own interest or \
remuneration ledger -- identified from the voucher's other lines, never from a date, number or \
narration."
        .to_string();

    // Each partner's figures are tagged `hash8(key)`, as the reference tags them, so two keys whose
    // tags coincide would name one figure twice. The reference raises there (its `fig` refuses a
    // duplicate id); refuse before any partner figure is built, naming both keys, rather than
    // leave `fig` to refuse the first repeated id.
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
    // A deed recorded as authorising no interest on capital: s.40(b) allows none, so the rate is
    // 0; the partner is still not computed.
    let no_interest = deed.is_some_and(|d| d.no_interest_authorised);
    let deed_rate = if no_interest {
        Some(0)
    } else {
        deed.and_then(|d| d.interest_rate_bp)
    };
    let deed_missing = deed_rate.is_none();
    // A deed is on file, its rate is not.
    let rate_not_recorded = deed.is_some() && deed_missing;
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
            _ if no_interest => "Rate used for s.40(b) allowable interest: 0, as the deed \
authorises no interest on capital (recorded in the configuration)."
                .to_string(),
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
    )?;
    if rate_not_recorded {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/deed_rate_missing"),
            clauses: vec!["s.40(b)".to_string(), "3CD-21(c)".to_string()],
            title: "Partnership deed recorded without its interest rate; s.40(b) interest rate \
assumed at the statutory cap"
                .to_string(),
            facts: vec![("rate_used".to_string(), f_rate)],
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![format!(
                "A deed is recorded for this firm, but not the interest rate it authorises. s.40(b) \
allows interest only as authorised by the deed, so the statutory cap ({s40b_rate} bp) is used here \
as an upper bound on allowable interest, not a confirmed rate."
            )],
            ask_client: vec![
                "The interest rate the partnership deed authorises (and any supplementary deed in \
force during the year)."
                    .to_string(),
            ],
        });
    } else if deed_missing {
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
    // Every partner's interest and remuneration ledger; "" is left out, as the reference's `if led`
    // leaves it out (the configuration check refuses it only where the books have no such ledger).
    let all_int_rem: BTreeSet<String> = partners
        .values()
        .flat_map(|q| {
            q.interest_ledger
                .iter()
                .chain(q.remuneration_ledger.iter())
                .filter(|l| !l.is_empty())
                .cloned()
        })
        .collect();

    if partners.is_empty() {
        // A firm with no partner configured is asked about, never a silent nil excess.
        r.findings.push(Finding {
            id: format!("{TEST_ID}/partners_not_configured"),
            clauses: vec![
                "s.40(b)".to_string(),
                "s.194T".to_string(),
                "3CD-21(c)".to_string(),
            ],
            title: "No partner is configured for this firm: s.40(b) interest and s.194T are not \
computed"
                .to_string(),
            facts: Vec::new(),
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "s.40(b) and s.194T apply to this entity, but the client's configuration names no \
partner, so no partner's capital, interest or remuneration is read. No excess is stated, and none \
is listed in clause 21(c)."
                    .to_string(),
            ],
            ask_client: vec![
                "The partners, and for each the capital, interest and remuneration ledgers."
                    .to_string(),
            ],
        });
    }

    for (key, p) in &partners {
        let h = hash8(key);
        let w = walk(&pop, book, p, tds_ledgers, &all_capitals, &all_int_rem);
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
        // A partner whose interest cannot be read exactly has s.40(b) NOT COMPUTED, and said so;
        // under the closing rule every partner is not computed.
        let mut unknown: BTreeMap<String, &Voucher> = w.mixed_unsplit_vouchers.clone();
        unknown.extend(w.shared_tds_interest_vouchers.clone());
        unknown.extend(w.off_capital_interest_vouchers.clone());
        unknown.extend(w.capital_set_off_vouchers.clone());
        unknown.extend(w.unread_credit_interest_vouchers.clone());
        unknown.extend(w.nonplain_vouchers.clone());
        // With no interest ledger its interest cannot be told from capital.
        let no_interest_ledger = p.interest_ledger.is_none();
        let read_fails = !unknown.is_empty() || no_interest_ledger;
        // Under the no-interest deed a partner is never computed, whatever the switch: interest
        // through any ledger is disallowed, and the configured interest ledger is only one of them.
        let not_computed = read_fails || no_interest || !excess_computed;
        let unusable = if not_computed {
            " Not usable for s.40(b): s.40(b) is not computed for this partner (see the finding \
that says so)."
        } else {
            ""
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
        )?;
        let no_remuneration_note = if p.remuneration_ledger.is_none() {
            " No remuneration ledger is configured for this partner: remuneration credits, if any, \
are read as capital; configure the remuneration ledger."
        } else {
            ""
        };
        let f_allow = r.fig(
            &format!("allowable_interest_{h}"),
            int(allowable_365)?,
            Unit::Paise,
            &format!(
                "s.40(b) allowable interest for partner (tag {h}): simple interest at {rate_bp} bp \
on the capital balance walked day by day (opening TB balance, then every population voucher line \
on the capital ledger(s) other than an interest/remuneration credit, changing the balance ON its \
date), actual days / 365, a debit (negative) capital day contributing zero.{unusable}\
{no_remuneration_note}"
            ),
            ev_capital.clone(),
        )?;
        r.fig(
            &format!("allowable_interest_sensitivity_360day_{h}"),
            int(allowable_360)?,
            Unit::Paise,
            &format!(
                "Same daily-balance walk for partner (tag {h}), but actual days / 360.{unusable}"
            ),
            ev_capital.clone(),
        )?;
        r.fig(
            &format!("allowable_interest_sensitivity_opening_only_{h}"),
            int(allowable_opening_only)?,
            Unit::Paise,
            &format!(
                "s.40(b) interest for partner (tag {h}) as if the capital balance stayed at its \
opening value for the whole year (no intra-year voucher line applied at all).{unusable}"
            ),
            ev_capital.clone(),
        )?;
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
        )?;

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
        )?;
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
touching this partner's capital, and are not booked against partners' capitals alone ({}): paid by \
bank or cash, credited to a payable, or a reclass -- \
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
                title: "Vouchers on a partner's interest or remuneration ledger not booked against \
partners' capitals alone: not counted for this partner"
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
        // The interest vouchers that also credit a ledger this test does not read, and those
        // ledgers, sorted (both named by the not-computed finding).
        let unread: BTreeMap<&String, &(&Voucher, BTreeSet<String>)> = w
            .unread_credit_vouchers
            .iter()
            .filter(|(g, _)| w.unread_credit_interest_vouchers.contains_key(*g))
            .collect();
        let unread_ledgers: BTreeSet<&String> =
            unread.values().flat_map(|(_, ls)| ls.iter()).collect();
        if not_computed {
            not_computed_count += 1;
            let mut reasons = Vec::new();
            if !excess_computed {
                reasons.push(
                    "This test computes the s.40(b) excess for no partner: its reading of a \
partner's interest from the books can miss or misread some bookings (another partner's interest or \
remuneration ledger crediting this partner's capital, or a ledger configured in two roles), so no \
excess it computed would be safe to list in clause 21(c). The CA computes the excess from the deed, \
the capital accounts and the interest credited. What this test could see is named below, and the \
interest credited and allowable-interest figures are working figures only."
                        .to_string(),
                );
            }
            if no_interest {
                reasons.push(format!(
                    "The deed authorises no interest on capital (as recorded in the configuration), \
so s.40(b) allows none: interest paid or credited to this partner, through any ledger (its \
remuneration ledger included), is disallowed in full. This test does not search the books for such \
interest{}, and states no disallowed amount.",
                    if no_interest_ledger {
                        ""
                    } else {
                        ", beyond reading its configured interest ledger as a working figure \
(marked not usable)"
                    }
                ));
            }
            if no_interest_ledger && !no_interest {
                reasons.push(
                    "No interest ledger is configured for this partner, so interest credited to its \
capital cannot be told from capital introduced: the capital walk reads every credit to it as \
capital, except remuneration on a configured remuneration ledger, and no interest credited is \
counted. If the deed authorises no interest on capital, the configuration can record that \
([partners.deed] no_interest_authorised)."
                        .to_string(),
                );
            }
            let other_shape: BTreeMap<String, &Voucher> = w
                .nonplain_vouchers
                .iter()
                .filter(|(g, _)| {
                    !w.capital_set_off_vouchers.contains_key(*g)
                        && !w.unread_credit_interest_vouchers.contains_key(*g)
                        && !w.shared_tds_interest_vouchers.contains_key(*g)
                })
                .map(|(g, v)| (g.clone(), *v))
                .collect();
            if !other_shape.is_empty() {
                reasons.push(format!(
                    "{} voucher(s) on this partner's capital are not of the plain shape this test \
reads exactly ({}): this partner's own interest or remuneration ledger debited, its own capital \
credited, and TDS on a ledger classified as TDS payable, balanced. Each carries a line this test \
does not read as plain, so the interest counted may be wrong in either direction.",
                    other_shape.len(),
                    labels(&other_shape)
                ));
            }
            if !w.nonplain_vouchers.is_empty() {
                reasons.push(
                    "This test reads only that plain shape. Correct bookings of other shapes are \
not computed here either: a year-end journal crediting several partners at once, a reversal or \
correction, interest on per-partner ledgers in one journal, one TDS line for several partners, TDS \
booked through the capital, or a round-off. The CA computes the excess from the vouchers named."
                        .to_string(),
                );
            }
            if !unread.is_empty() {
                let tags = unread_ledgers
                    .iter()
                    .map(|x| stable_ledger_tag(book, x))
                    .collect::<Result<Vec<_>>>()?;
                reasons.push(format!(
                    "{} interest voucher(s) also credit a ledger this test does not read ({}; \
ledger tag(s) {}): the partner may be credited net of it, so the interest counted may be \
understated. If the ledger is TDS on this partner's interest alone, classify it as TDS payable in \
the client's statutory dues and the interest is counted gross; on a voucher that credits several \
partners the TDS is not divided, and if it is a payable, a round-off or a payment, the CA reads the \
interest from the voucher.",
                    unread.len(),
                    unread
                        .values()
                        .map(|(v, _)| voucher_label(v))
                        .collect::<Vec<_>>()
                        .join(", "),
                    tags.join(", ")
                ));
            }
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
                    "{} voucher(s) post to this partner's interest ledger without touching this \
partner's capital, and are not booked against partners' capitals alone ({}): not counted in the interest credited, which may therefore be understated \
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
            let title =
                if no_interest && unknown.is_empty() && (no_interest_ledger || excess_computed) {
                    format!(
                    "s.40(b) not computed for partner (tag {h}): the deed authorises no interest \
on capital, so any interest is disallowed in full"
                )
                } else if read_fails {
                    format!(
                    "s.40(b) not computed for partner (tag {h}): its interest credited cannot be \
read exactly; the CA computes it"
                )
                } else {
                    format!(
                    "s.40(b) not computed for partner (tag {h}): this test computes the excess for \
no partner; the CA computes it"
                )
                };
            let mut evidence = voucher_refs(&unknown);
            evidence.extend(unread_ledgers.iter().map(|x| EvidenceRef::new("ledger", x)));
            if no_interest_ledger {
                evidence.extend(
                    p.capital_ledgers
                        .iter()
                        .map(|n| EvidenceRef::new("ledger", n)),
                );
            }
            let mut ask_client = Vec::new();
            // Asked for the computation unless the switch computes it (never under the no-interest
            // deed), or the deed authorises no interest and no interest ledger is configured
            // (nothing to ask).
            if !((excess_computed && !no_interest) || (no_interest && no_interest_ledger)) {
                ask_client.push(if no_interest {
                    "The s.40(b) computation for this partner: the interest credited (the deed \
authorises none, so all of it is disallowed)."
                        .to_string()
                } else {
                    "The s.40(b) computation for this partner: the deed's authorised interest \
rate, the capital balances through the year, and the interest credited."
                        .to_string()
                });
            }
            if !unknown.is_empty() {
                ask_client.push(
                    "The partner's interest for the year, with the interest and remuneration in \
each voucher named here and the TDS on each."
                        .to_string(),
                );
            }
            if !unread.is_empty() {
                ask_client.push(format!(
                    "If {} carries TDS on this partner's interest, classify it as TDS payable in \
your statutory dues; otherwise, what it is.",
                    unread_ledgers
                        .iter()
                        .map(|x| py_repr_str(x))
                        .collect::<Vec<_>>()
                        .join(" or ")
                ));
            }
            if no_interest_ledger && !no_interest {
                ask_client.push("The ledger that carries this partner's interest.".to_string());
            }
            r.findings.push(Finding {
                id: format!("{TEST_ID}/s40b_not_computed/{h}"),
                clauses: vec!["s.40(b)".to_string(), "3CD-21(c)".to_string()],
                title,
                facts: Vec::new(),
                evidence,
                confidence: Confidence::JudgementRequired,
                limits: reasons,
                ask_client,
            });
        } else {
            let f_excess = r.fig(
                &format!("s40b_excess_{h}"),
                int(excess)?,
                Unit::Paise,
                &format!("interest_credited_{h} minus allowable_interest_{h}, floor 0."),
                Vec::new(),
            )?;
            excess_total += excess;
            if excess > 0 {
                let mut limits = vec![if rate_not_recorded {
                    "The deed's interest rate is not recorded; the excess shown uses the statutory \
cap as the assumed authorised rate -- confirm the deed's actual rate and terms before relying on \
this figure."
                        .to_string()
                } else if deed_missing {
                    "No deed was available; the excess shown uses the statutory cap as the \
assumed authorised rate -- confirm the deed's actual rate and terms before relying on this figure."
                        .to_string()
                } else {
                    "Allowable interest here assumes the capital base is exactly the capital \
ledger(s) supplied and that interest was authorised for the whole year; confirm both against the \
deed."
                        .to_string()
                }];
                if p.remuneration_ledger.is_none() {
                    limits.push(
                        "No remuneration ledger is configured for this partner: remuneration \
credits, if any, are read as capital, which would raise the allowable interest and lower this \
excess; configure the remuneration ledger."
                            .to_string(),
                    );
                }
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
                    limits,
                    ask_client: vec![
                        "Confirm the deed's authorised interest rate and the capital \
base it applies to."
                            .to_string(),
                    ],
                });
            }
        }

        let f_rem = r.fig(
            &format!("remuneration_credited_{h}"),
            int(w.remuneration_credited_paise)?,
            Unit::Paise,
            &format!(
                "Remuneration credited to partner (tag {h}), in a voucher also carrying a line on \
their remuneration_ledger. The s.40(b)(v) book-profit limit is NOT computed here.{}",
                if w.mixed_unsplit_vouchers.is_empty() {
                    ""
                } else {
                    " A voucher carrying both the interest and the remuneration ledger that is not \
split is counted as interest, not here."
                }
            ),
            voucher_refs(&w.remuneration_vouchers),
        )?;
        remuneration_facts.push((key.clone(), f_rem));
        let mixed_here = &w.mixed_unsplit_vouchers;
        // A voucher is mixed only when it carries a remuneration ledger, so one is configured.
        if !mixed_here.is_empty() {
            // The remuneration inside an unsplit voucher is counted as interest.
            remuneration_evidence.extend(mixed_here.clone());
            remuneration_notes.push(format!(
                "Partner (tag {h}): {} voucher(s) carrying both the interest and the remuneration \
ledger are not split ({}): their credit is counted as interest, so remuneration_credited_{h} leaves \
out the remuneration in them.",
                mixed_here.len(),
                labels(mixed_here)
            ));
        }
        let unread_rem: BTreeMap<String, &Voucher> = w
            .unread_credit_vouchers
            .iter()
            .filter(|(g, _)| !w.unread_credit_interest_vouchers.contains_key(*g))
            .map(|(g, (v, _))| (g.clone(), *v))
            .collect();
        let other_rem: BTreeMap<String, &Voucher> = w
            .nonplain_vouchers
            .iter()
            .filter(|(g, _)| {
                !w.nonplain_interest_vouchers.contains_key(*g)
                    && !unread_rem.contains_key(*g)
                    && !w.shared_tds_remuneration_vouchers.contains_key(*g)
            })
            .map(|(g, v)| (g.clone(), *v))
            .collect();
        if !other_rem.is_empty() {
            remuneration_evidence.extend(other_rem.clone());
            remuneration_notes.push(format!(
                "Partner (tag {h}): {} remuneration voucher(s) are not of the plain shape this test \
reads exactly ({}): remuneration_credited_{h} may be wrong in either direction.",
                other_rem.len(),
                labels(&other_rem)
            ));
        }
        if !unread_rem.is_empty() {
            remuneration_evidence.extend(unread_rem.clone());
            remuneration_notes.push(format!(
                "Partner (tag {h}): {} remuneration voucher(s) also credit a ledger this test does \
not read ({}): remuneration_credited_{h} may be net of it. If that ledger is TDS on this partner's \
remuneration alone, classify it as TDS payable in the client's statutory dues.",
                unread_rem.len(),
                labels(&unread_rem)
            ));
        }
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

        // With no interest ledger, or a credit this test does not read, the base may be wrong, so
        // the s.194T question is raised whatever the base -- never a silence under the limit (a
        // non-plain voucher leaves the partner not computed, so `not_computed` covers it).
        let base_uncertain = not_computed
            || !w.off_capital_vouchers.is_empty()
            || !w.unread_credit_vouchers.is_empty();
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
remuneration, commission, bonus and interest to a partner).{}",
                if base_uncertain {
                    " It may be wrong in either direction: see this partner's s.194T question."
                } else {
                    ""
                }
            ),
            Vec::new(),
        )?;
        let tds_expected = round_half_up(subject_paise * i128::from(tds_rate_bp), 10_000);
        let f_tds = r.fig(
            &format!("s194t_tds_expected_{h}"),
            int(tds_expected)?,
            Unit::Paise,
            &format!("TDS @ {tds_rate_bp} bp on s194t_amount_credited_{h}."),
            Vec::new(),
        )?;
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
        // The TDS's own side, apart from the TDS, is partners' capitals alone -- the walk's own test
        // for adding it back gross: it is theirs. Anything else on that side (a bank, a payable)
        // and it is not judged whose. A net-nil TDS (deducted and reversed, or moved between TDS
        // ledgers) has no side, so it is theirs when it touches a partner's capital.
        let partners_tds = |v: &Voucher| {
            let tds: i128 = -v
                .lines
                .iter()
                .filter(|l| tds_ledgers.contains(&l.ledger))
                .map(|l| i128::from(l.amount_paise))
                .sum::<i128>();
            if tds == 0 {
                return v
                    .lines
                    .iter()
                    .any(|l| all_capitals.contains(&l.ledger) && l.amount_paise != 0);
            }
            let side: BTreeSet<&str> = v
                .lines
                .iter()
                .filter(|l| {
                    i128::from(l.amount_paise).signum() * tds.signum() < 0
                        && !tds_ledgers.contains(&l.ledger)
                })
                .map(|l| l.ledger.as_str())
                .collect();
            !side.is_empty() && side.iter().all(|l| all_capitals.contains(*l))
        };
        let mut seen: BTreeMap<String, &Voucher> = BTreeMap::new();
        let mut unattributed: BTreeMap<String, &Voucher> = BTreeMap::new();
        if !tds_ledgers.is_empty() {
            // Only a non-zero line touches a ledger, as the walk reads it: a zero capital line
            // never makes a shared ledger's TDS that partner's.
            for &v in &pop {
                if v.lines
                    .iter()
                    .any(|l| own.contains(l.ledger.as_str()) && l.amount_paise != 0)
                    && has_tds(v)
                {
                    seen.insert(v.guid.clone(), v);
                }
            }
            // On a shared ledger, TDS booked against partners' capitals alone is theirs, never
            // "whose is not judged" for another.
            for &v in &pop {
                if !seen.contains_key(&v.guid)
                    && has_tds(v)
                    && v.lines.iter().any(|l| {
                        ledgers.contains(l.ledger.as_str())
                            && shared_ledgers.contains(l.ledger.as_str())
                            && l.amount_paise != 0
                    })
                    && !partners_tds(v)
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
on a voucher whose TDS is not booked against partners' capitals alone)."
            ),
            Vec::new(),
        )?;

        let over = subject_paise > i128::from(tds_limit_paise);
        if over || base_uncertain {
            let mut limits = vec![
                "Books only: TAN registration, challans filed and any Form 26A \
route are not visible from vouchers."
                    .to_string(),
            ];
            limits.extend(shared_tds_note(&w));
            if no_interest_ledger {
                limits.push(
                    "No interest ledger is configured for this partner, so interest credited to its \
capital is not in this base, which may therefore be understated."
                        .to_string(),
                );
            }
            let mut odd = w.nonplain_vouchers.clone();
            odd.extend(w.off_capital_vouchers.clone());
            if !odd.is_empty() {
                limits.push(format!(
                    "{} voucher(s) touching this partner are not read exactly ({}): this base may \
be wrong in either direction.",
                    odd.len(),
                    labels(&odd)
                ));
            }
            if !over {
                limits.push(format!(
                    "This base ({}) is under the s.194T threshold ({}) as read, but it may be wrong \
(above), so whether it crosses the threshold is not computed.",
                    rupees(subject_paise),
                    rupees(i128::from(tds_limit_paise))
                ));
            }
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
            // With the switch off a plain partner's base is read, but not confirmed: say which.
            let thr = if over {
                "over the s.194T threshold"
            } else if read_fails
                || !w.off_capital_vouchers.is_empty()
                || !w.unread_credit_vouchers.is_empty()
            {
                "whose s.194T base is not read exactly"
            } else {
                "whose s.194T base is not confirmed: s.40(b) is not computed"
            };
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
            let not_judged = format!(
                "Payments/credits to a partner {thr}: whether TDS was deducted is not judged"
            );
            let title = if tds_ledgers.is_empty() {
                limits.push(
                    "The client's statutory dues classify no ledger as TDS payable, so TDS on the \
partner's vouchers cannot be seen."
                        .to_string(),
                );
                not_judged
            } else if !seen.is_empty() {
                limits.push(format!(
                    "TDS of {} (net) is seen on {} voucher(s) touching this partner: {}.",
                    rupees(tds_seen_paise),
                    seen.len(),
                    first12(&by_date(&seen))
                ));
                format!(
                    "Payments/credits to a partner {thr}, with TDS seen on its vouchers: which \
credits it covers, and its deposit, are the CA's to determine"
                )
            } else if !unattributed.is_empty() {
                not_judged
            } else {
                format!("Payments/credits to a partner {thr} with no TDS ledger line seen")
            };
            if !tds_ledgers.is_empty() && !unattributed.is_empty() {
                let l = by_date(&unattributed);
                limits.push(format!(
                    "TDS lines are seen on {} voucher(s) on an interest or remuneration ledger \
shared by partners whose TDS is not booked against partners' capitals alone: {}. Which partner's \
TDS they are is not judged.",
                    l.len(),
                    first12(&l)
                ));
            }
            let mut ask_client = Vec::new();
            if base_uncertain {
                ask_client.push(
                    "The partner's interest and remuneration for the year, and the TDS deducted on \
each."
                        .to_string(),
                );
            }
            ask_client.push(
                "Confirm the firm's TAN and whether TDS under s.194T was deposited (this is \
not visible from the books shown here if it went through a ledger not captured in \
touched_vouchers)."
                    .to_string(),
            );
            ask_client.push(
                "If not deducted, confirm whether Form 26A relief is available (partner \
returned the income, tax paid)."
                    .to_string(),
            );
            let mut evidence = touched.clone();
            evidence.extend(seen.clone());
            evidence.extend(unattributed.clone());
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
                ask_client,
            });
        }
    }

    // A total over some partners reads as "no disallowance" beside a bare figure: it is withheld
    // whenever a partner is not computed or none is configured, and the reason stands in its place.
    if !partners.is_empty() && not_computed_count == 0 {
        r.fig(
            "s40b_excess_total",
            int(excess_total)?,
            Unit::Paise,
            "Sum of s40b_excess_<partner> across all partners.",
            Vec::new(),
        )?;
    } else {
        r.fig(
            "s40b_excess_total_not_computed",
            Value::Text(if partners.is_empty() {
                "not computed: no partner is configured for this firm".to_string()
            } else {
                format!("not computed: s.40(b) is not computed for {not_computed_count} partner(s)")
            }),
            Unit::Text,
            "The sum of the s.40(b) excess across the firm's partners is not stated: it would leave \
out the partners whose excess is not computed (their s40b_not_computed findings), or no partner is \
configured (see that question). Where a partner's excess is computed, it is that partner's own \
figure.",
            Vec::new(),
        )?;
    }
    r.fig(
        "s194t_interest_credited_total",
        int(credited_total)?,
        Unit::Paise,
        "Sum of interest_credited_<partner> across all partners -- cross-check this against a \
shared interest_ledger's own TB movement when every partner uses the same ledger name.",
        Vec::new(),
    )?;

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

    /// An invented ledger with no GUID (its stable tag is the hash of its name).
    fn ledger(name: &str) -> crate::book::Ledger {
        crate::book::Ledger {
            name: name.to_string(),
            parent: "Capital Account".to_string(),
            chain: vec!["Capital Account".to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            guid: String::new(),
            masterid: None,
        }
    }

    /// The reference selftest's ledgers (`CAPITAL_A`, `CAPITAL_B`, the interest, remuneration,
    /// cash and TDS ledgers, and `UNCLASSIFIED`).
    const LEDGERS: [&str; 7] = [
        "Partner A",
        "Partner B",
        "Interest to Partners",
        "Remuneration to Partners",
        "Cash",
        "TDS Payable",
        "TDS on Partners 194T",
    ];

    /// The selftest's `go` book: partner A opens with Rs 10,00,000 of capital (a credit), B with
    /// none.
    fn book_of(vouchers: Vec<Voucher>) -> Book {
        let row = |opening_paise| crate::book::TbRow {
            opening_paise,
            debit_paise: 0,
            credit_paise: 0,
            closing_paise: 0,
        };
        Book {
            company_name: "Invented".to_string(),
            company_guid: "invented".to_string(),
            ledgers: LEDGERS
                .iter()
                .map(|n| ((*n).to_string(), ledger(n)))
                .collect(),
            vouchers,
            tb: BTreeMap::from([
                ("Partner A".to_string(), row(-100_000_000)),
                ("Partner B".to_string(), row(0)),
            ]),
            ..Default::default()
        }
    }

    /// A regular journal with no number (its label uses the GUID), on `date` (YYYYMMDD).
    fn voucher_on(guid: &str, date: &str, lines: &[(&str, i64)]) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse(date).unwrap(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            status: VoucherStatus::Regular,
            lines: lines
                .iter()
                .map(|(ledger, amount_paise)| LedgerLine {
                    ledger: (*ledger).to_string(),
                    amount_paise: *amount_paise,
                })
                .collect(),
            ..Default::default()
        }
    }

    fn voucher(guid: &str, lines: &[(&str, i64)]) -> Voucher {
        voucher_on(guid, "20260331", lines)
    }

    const ONE: &str = "[partner_a]\ncapital_ledgers = [\"Partner A\"]\n\
interest_ledger = \"Interest to Partners\"\n";
    const TWO: &str = "[partner_a]\ncapital_ledgers = [\"Partner A\"]\n\
interest_ledger = \"Interest to Partners\"\n\
[partner_b]\ncapital_ledgers = [\"Partner B\"]\n\
interest_ledger = \"Interest to Partners\"\n";
    const DEED: &str = "[deed]\ninterest_rate_bp = 1000\n";

    /// The selftest's `go`: a firm under the vendored rules, the given partners and deed, "TDS
    /// Payable" classified as TDS payable unless `tds` says otherwise.
    fn go_with(
        vouchers: Vec<Voucher>,
        config: &str,
        tds: &[&str],
        excess_computed: bool,
    ) -> Result<TestResult> {
        let tds: BTreeSet<String> = tds.iter().map(|s| (*s).to_string()).collect();
        run_with(
            &book_of(vouchers),
            &Rules::vendored().unwrap(),
            &year(),
            "firm",
            &cfg(config),
            &tds,
            excess_computed,
        )
    }

    fn go(vouchers: Vec<Voucher>, config: &str) -> TestResult {
        go_with(vouchers, config, &["TDS Payable"], S40B_EXCESS_COMPUTED).unwrap()
    }

    fn fig<'r>(r: &'r TestResult, name: &str) -> &'r crate::findings::Figure {
        let id = format!("{TEST_ID}.{name}");
        r.figures
            .iter()
            .find(|f| f.id == id)
            .unwrap_or_else(|| panic!("no figure {id}"))
    }

    fn has_fig(r: &TestResult, name: &str) -> bool {
        let id = format!("{TEST_ID}.{name}");
        r.figures.iter().any(|f| f.id == id)
    }

    fn found<'r>(r: &'r TestResult, prefix: &str) -> Vec<&'r Finding> {
        let prefix = format!("{TEST_ID}/{prefix}");
        r.findings
            .iter()
            .filter(|f| f.id.starts_with(&prefix))
            .collect()
    }

    fn config_refusal(result: Result<TestResult>) -> String {
        match result {
            Err(AuditError::Config(m)) => m,
            other => panic!("expected a Config refusal, got {other:?}"),
        }
    }

    const LEAD: &str = "This test computes the s.40(b) excess for no partner: its reading of a \
partner's interest from the books can miss or misread some bookings (another partner's interest or \
remuneration ledger crediting this partner's capital, or a ledger configured in two roles), so no \
excess it computed would be safe to list in clause 21(c). The CA computes the excess from the deed, \
the capital accounts and the interest credited. What this test could see is named below, and the \
interest credited and allowable-interest figures are working figures only.";
    const CLOSING: &str = "No s.40(b) excess is stated for this partner, and none is listed in \
clause 21(c). Its interest credited and allowable-interest figures are marked not usable for \
s.40(b).";
    const PLAIN_ONLY: &str = "This test reads only that plain shape. Correct bookings of other \
shapes are not computed here either: a year-end journal crediting several partners at once, a \
reversal or correction, interest on per-partner ledgers in one journal, one TDS line for several \
partners, TDS booked through the capital, or a round-off. The CA computes the excess from the \
vouchers named.";
    const ASK_RATE: &str = "The s.40(b) computation for this partner: the deed's authorised \
interest rate, the capital balances through the year, and the interest credited.";
    const ASK_VOUCHERS: &str = "The partner's interest for the year, with the interest and \
remuneration in each voucher named here and the TDS on each.";
    const ASK_BASE: &str =
        "The partner's interest and remuneration for the year, and the TDS deducted on each.";

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
        // Typing reads the list as written; `check_partners_config` refuses a name listed twice
        // (a_capital_ledger_list_the_reference_cannot_read_is_refused).
        let (partners, deed) = partners_config(&cfg(
            "[a]\ncapital_ledgers = [\"A\", \"A\"]\n[deed]\ninterest_rate_bp = 1000\n",
        ))
        .unwrap();
        assert_eq!(partners["a"].capital_ledgers, ["A", "A"]);
        assert_eq!(
            deed,
            Some(Deed {
                interest_rate_bp: Some(1000),
                no_interest_authorised: false
            })
        );
    }

    #[test]
    fn two_partner_keys_sharing_a_figure_tag_are_refused_not_a_panic() {
        // SHA-1 of "p30395" and of "p89343" both begin 47ff8a3d.
        assert_eq!(hash8("p30395"), hash8("p89343"));
        let mut book = Book {
            vouchers: Vec::new(),
            ..unreadable_book()
        };
        for name in ["A Capital", "B Capital"] {
            book.ledgers.insert(name.to_string(), ledger(name));
        }
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

    // ---- the configuration the reference refuses (check_partners_config) ----

    const NOINT: &str = "[partner_a]\ncapital_ledgers = [\"Partner A\"]\n\
remuneration_ledger = \"Remuneration to Partners\"\n";

    #[test]
    fn the_no_interest_flag_is_refused_in_any_form_but_a_boolean_or_beside_any_rate() {
        for (deed, why) in [
            (
                "no_interest_authorised = \"yes\"",
                "[partners.deed].no_interest_authorised must be true or false; got ",
            ),
            (
                "no_interest_authorised = 1",
                "[partners.deed].no_interest_authorised must be true or false; got ",
            ),
            (
                "no_interest_authorised = true\ninterest_rate_bp = 0",
                "[partners.deed] sets no_interest_authorised and an interest_rate_bp; the deed \
either authorises no interest or authorises a rate",
            ),
            (
                "no_interest_authorised = true\ninterest_rate_bp = 1000",
                "[partners.deed] sets no_interest_authorised and an interest_rate_bp; the deed \
either authorises no interest or authorises a rate",
            ),
            // The rate is checked first, as the reference checks it.
            (
                "no_interest_authorised = 1\ninterest_rate_bp = -100",
                "[partners.deed].interest_rate_bp must be a whole number of basis points, 0 or \
more; got ",
            ),
        ] {
            let config = format!("{NOINT}[deed]\n{deed}\n");
            let m = config_refusal(go_with(Vec::new(), &config, &["TDS Payable"], false));
            // The value refused is quoted as TOML writes it; the text before it is the reference's.
            assert!(m.starts_with(&format!("{TEST_ID}: {why}")), "{deed}: {m}");
        }
        // A rate that is not a whole number of basis points, 0 or more, is refused too.
        for rate in ["12.5", "true", "\"1200\""] {
            let config = format!("{NOINT}[deed]\ninterest_rate_bp = {rate}\n");
            let m = config_refusal(go_with(Vec::new(), &config, &["TDS Payable"], false));
            assert!(
                m.starts_with(&format!(
                    "{TEST_ID}: [partners.deed].interest_rate_bp must be a whole number"
                )),
                "{m}"
            );
        }
        // A deed that is not a table is refused.
        let m = config_refusal(go_with(
            Vec::new(),
            &format!("deed = 1000\n{NOINT}"),
            &["TDS Payable"],
            false,
        ));
        assert!(
            m.starts_with(&format!("{TEST_ID}: [partners.deed] must be a table; got ")),
            "{m}"
        );
    }

    #[test]
    fn a_ledger_configured_in_two_roles_is_refused() {
        let a = "[partner_a]\ncapital_ledgers = [\"Partner A\"]\n";
        let b = "[partner_b]\ncapital_ledgers = [\"Partner B\"]\n\
interest_ledger = \"Interest to Partners\"\n";
        let role = |role: &str, led: &str| {
            format!(
                "[partners.partner_a].{role} '{led}' is a partner's capital ledger or a TDS ledger"
            )
        };
        for (config, why) in [
            (
                format!(
                    "{a}interest_ledger = \"Interest to Partners\"\n\
remuneration_ledger = \"Interest to Partners\"\n"
                ),
                "[partners.partner_a] names 'Interest to Partners' as both its interest and its \
remuneration ledger"
                    .to_string(),
            ),
            (
                format!("{a}interest_ledger = \"Partner A\"\n"),
                role("interest_ledger", "Partner A"),
            ),
            (
                format!("{a}interest_ledger = \"Partner B\"\n{b}"),
                role("interest_ledger", "Partner B"),
            ),
            (
                format!(
                    "{a}interest_ledger = \"Interest to Partners\"\n\
remuneration_ledger = \"Partner A\"\n"
                ),
                role("remuneration_ledger", "Partner A"),
            ),
            (
                format!("{a}interest_ledger = \"TDS Payable\"\n"),
                role("interest_ledger", "TDS Payable"),
            ),
            (
                format!(
                    "{a}interest_ledger = \"Interest to Partners\"\n\
remuneration_ledger = \"TDS Payable\"\n"
                ),
                role("remuneration_ledger", "TDS Payable"),
            ),
        ] {
            let m = config_refusal(go_with(Vec::new(), &config, &["TDS Payable"], false));
            assert_eq!(m, format!("{TEST_ID}: {why}"), "{config}");
        }
        // The control: the TDS ledger is refused only because it is classified as TDS payable.
        let config = format!("{a}interest_ledger = \"TDS Payable\"\n");
        assert!(go_with(Vec::new(), &config, &[], false).is_ok());
    }

    #[test]
    fn a_capital_ledger_list_or_ledger_the_reference_cannot_read_is_refused() {
        let list = |caps: &str| {
            format!(
                "[partner_a]\ncapital_ledgers = {caps}\ninterest_ledger = \"Interest to Partners\"\n"
            )
        };
        let shape = |got: &str| {
            format!(
                "[partners.partner_a].capital_ledgers must be a non-empty list of distinct ledger \
names; got {got}"
            )
        };
        for (config, why) in [
            (
                list("[\"Partner A\", \"Partner A\"]"),
                shape("['Partner A', 'Partner A']"),
            ),
            (list("[]"), shape("[]")),
            (list("[\"\"]"), shape("['']")),
            (
                format!(
                    "{}{}",
                    list("[\"Partner A\"]"),
                    list("[\"Partner A\"]").replace("partner_a", "partner_b")
                ),
                "[partners.partner_b].capital_ledgers names 'Partner A', which is also partner \
'partner_a''s capital ledger"
                    .to_string(),
            ),
            (
                list("[\"Partner Z\"]"),
                "[partners.partner_a] names ledger 'Partner Z', which is not in the books"
                    .to_string(),
            ),
            (
                "[partner_a]\ncapital_ledgers = [\"Partner A\"]\ninterest_ledger = \"\"\n"
                    .to_string(),
                "[partners.partner_a] names ledger '', which is not in the books".to_string(),
            ),
        ] {
            let m = config_refusal(go_with(Vec::new(), &config, &["TDS Payable"], false));
            assert_eq!(m, format!("{TEST_ID}: {why}"), "{config}");
        }
    }

    // ---- the closing rule: the excess is computed for no partner ----

    #[test]
    fn the_switch_is_off_in_run() {
        // `run` itself, as production calls it: a plain partner is asked, never computed.
        let book = book_of(vec![voucher(
            "p1",
            &[
                ("Interest to Partners", 50_000_000),
                ("Partner A", -50_000_000),
            ],
        )]);
        let tds = BTreeSet::from(["TDS Payable".to_string()]);
        let rules = Rules::vendored().unwrap();
        let r = run(
            &book,
            &rules,
            &year(),
            "firm",
            &cfg(&format!("{ONE}{DEED}")),
            &tds,
        )
        .unwrap();
        assert_eq!(found(&r, "s40b_not_computed/").len(), 1);
        assert!(found(&r, "s40b_excess/").is_empty());
    }

    #[test]
    fn a_plain_partner_with_an_excess_is_asked_not_computed_and_the_total_withheld() {
        let plain = || {
            vec![voucher(
                "p1",
                &[
                    ("Interest to Partners", 50_000_000),
                    ("Partner A", -50_000_000),
                ],
            )]
        };
        let r = go(plain(), &format!("{ONE}{DEED}"));
        let h = hash8("partner_a");
        let f = found(&r, "s40b_not_computed/");
        assert_eq!(f.len(), 1);
        assert_eq!(
            f[0].title,
            format!(
                "s.40(b) not computed for partner (tag {h}): this test computes the excess for no \
partner; the CA computes it"
            )
        );
        assert_eq!(f[0].limits, [LEAD, CLOSING]);
        assert!(f[0].evidence.is_empty());
        assert_eq!(f[0].ask_client, [ASK_RATE]);
        assert!(!has_fig(&r, &format!("s40b_excess_{h}")));
        assert!(found(&r, "s40b_excess/").is_empty());
        assert!(!has_fig(&r, "s40b_excess_total"));
        assert_eq!(
            fig(&r, "s40b_excess_total_not_computed").value,
            Value::Text("not computed: s.40(b) is not computed for 1 partner(s)".to_string())
        );
        assert_eq!(
            fig(&r, &format!("interest_credited_{h}")).value,
            Value::Int(50_000_000)
        );
        for name in ["interest_credited", "allowable_interest"] {
            assert!(fig(&r, &format!("{name}_{h}"))
                .definition
                .contains("Not usable for s.40(b)"));
        }
        assert!(fig(&r, &format!("allowable_interest_{h}")).definition.ends_with(
            " No remuneration ledger is configured for this partner: remuneration credits, if any, \
are read as capital; configure the remuneration ledger."
        ));
        // Over the limit, with the question raised whatever the base.
        let t = found(&r, &format!("s194t/{h}"));
        assert_eq!(
            t[0].title,
            "Payments/credits to a partner over the s.194T threshold with no TDS ledger line seen"
        );
        assert_eq!(t[0].ask_client[0], ASK_BASE);

        // The control: the same book computes the excess with the switch on. Opening capital
        // Rs 10,00,000 all year at 1000 bp is Rs 1,00,000 allowable; Rs 5,00,000 credited.
        let r = go_with(plain(), &format!("{ONE}{DEED}"), &["TDS Payable"], true).unwrap();
        assert!(found(&r, "s40b_not_computed/").is_empty());
        assert_eq!(
            fig(&r, &format!("allowable_interest_{h}")).value,
            Value::Int(10_000_000)
        );
        assert_eq!(
            fig(&r, &format!("s40b_excess_{h}")).value,
            Value::Int(40_000_000)
        );
        assert_eq!(fig(&r, "s40b_excess_total").value, Value::Int(40_000_000));
        assert!(!has_fig(&r, "s40b_excess_total_not_computed"));
        let e = found(&r, "s40b_excess/");
        assert_eq!(e.len(), 1);
        assert_eq!(
            e[0].limits[1],
            "No remuneration ledger is configured for this partner: remuneration credits, if any, \
are read as capital, which would raise the allowable interest and lower this excess; configure the \
remuneration ledger."
        );
    }

    #[test]
    fn at_exactly_the_s194t_limit_the_base_is_not_over_it() {
        // The vendored [s194t] limit is Rs 20,000.
        let credit = |paise: i64| {
            vec![voucher(
                "p1",
                &[("Interest to Partners", paise), ("Partner A", -paise)],
            )]
        };
        let r = go(credit(2_000_000), &format!("{ONE}{DEED}"));
        let h = hash8("partner_a");
        let t = found(&r, &format!("s194t/{h}"));
        assert_eq!(
            t[0].title,
            "Payments/credits to a partner whose s.194T base is not confirmed: s.40(b) is not \
computed with no TDS ledger line seen"
        );
        assert!(t[0].limits.contains(
            &"This base (₹20,000) is under the s.194T threshold (₹20,000) as read, but it may be \
wrong (above), so whether it crosses the threshold is not computed."
                .to_string()
        ));
        assert!(fig(&r, &format!("s194t_amount_credited_{h}"))
            .definition
            .ends_with(
                " It may be wrong in either direction: see this partner's s.194T question."
            ));
        let r = go(credit(2_000_001), &format!("{ONE}{DEED}"));
        assert_eq!(
            found(&r, &format!("s194t/{h}"))[0].title,
            "Payments/credits to a partner over the s.194T threshold with no TDS ledger line seen"
        );
        // With the switch on, a base read exactly and not over the limit is silent.
        let on = |paise| {
            go_with(
                credit(paise),
                &format!("{ONE}{DEED}"),
                &["TDS Payable"],
                true,
            )
        };
        assert!(found(&on(2_000_000).unwrap(), "s194t/").is_empty());
        assert_eq!(found(&on(2_000_001).unwrap(), "s194t/").len(), 1);
    }

    #[test]
    fn a_firm_with_no_partner_configured_is_asked_and_its_total_withheld() {
        let r = go(Vec::new(), DEED);
        let q = found(&r, "partners_not_configured");
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].clauses, ["s.40(b)", "s.194T", "3CD-21(c)"]);
        assert_eq!(
            q[0].title,
            "No partner is configured for this firm: s.40(b) interest and s.194T are not computed"
        );
        assert_eq!(
            q[0].ask_client,
            ["The partners, and for each the capital, interest and remuneration ledgers."]
        );
        assert!(!has_fig(&r, "s40b_excess_total"));
        assert_eq!(
            fig(&r, "s40b_excess_total_not_computed").value,
            Value::Text("not computed: no partner is configured for this firm".to_string())
        );
    }

    #[test]
    fn a_deed_without_its_rate_is_asked_for_the_rate_not_the_deed() {
        let r = go(
            Vec::new(),
            &format!("{ONE}[deed]\nremuneration_authorised = true\n"),
        );
        assert!(found(&r, "deed_missing").is_empty());
        let q = found(&r, "deed_rate_missing");
        assert_eq!(q.len(), 1);
        assert_eq!(
            q[0].title,
            "Partnership deed recorded without its interest rate; s.40(b) interest rate assumed at \
the statutory cap"
        );
        assert_eq!(fig(&r, "interest_rate_bp_used").value, Value::Int(1200));
    }

    // ---- P2-3: one TDS line shared by two partners' interest ----

    #[test]
    fn one_tds_line_for_two_partners_is_named_and_leaves_both_not_computed() {
        let j1 = voucher(
            "j1",
            &[
                ("Interest to Partners", 4_400_000),
                ("Partner A", -1_980_000),
                ("Partner B", -1_980_000),
                ("TDS Payable", -440_000),
            ],
        );
        let r = go(vec![j1], &format!("{TWO}{DEED}"));
        let h = hash8("partner_a");
        // Not added back: the partner's interest is read net of an unknown part of it.
        assert_eq!(
            fig(&r, &format!("interest_credited_{h}")).value,
            Value::Int(1_980_000)
        );
        let f = found(&r, &format!("s40b_not_computed/{h}"));
        assert_eq!(
            f[0].title,
            format!(
                "s.40(b) not computed for partner (tag {h}): its interest credited cannot be read \
exactly; the CA computes it"
            )
        );
        assert_eq!(
            f[0].limits,
            [
                LEAD,
                PLAIN_ONLY,
                "1 interest voucher(s) carry TDS but also credit another party (Journal j1 on \
2026-03-31): the TDS is not divided between them, so the interest counted is net of an unknown part \
of it.",
                CLOSING,
            ]
        );
        assert_eq!(
            f[0].evidence,
            [EvidenceRef::with_label(
                "voucher",
                "j1",
                "Journal j1 on 2026-03-31"
            )]
        );
        assert_eq!(f[0].ask_client, [ASK_RATE, ASK_VOUCHERS]);
        assert_eq!(
            found(&r, &format!("s40b_not_computed/{}", hash8("partner_b"))).len(),
            1
        );
        // Under the limit as read, the s.194T question is still raised.
        let t = found(&r, &format!("s194t/{h}"));
        assert_eq!(
            t[0].title,
            "Payments/credits to a partner whose s.194T base is not read exactly, with TDS seen on \
its vouchers: which credits it covers, and its deposit, are the CA's to determine"
        );
        assert_eq!(
            t[0].limits,
            [
                "Books only: TAN registration, challans filed and any Form 26A route are not \
visible from vouchers.",
                "1 interest or remuneration voucher(s) carry TDS but also credit another party \
(Journal j1 on 2026-03-31): their TDS is not added back, so the interest or remuneration counted may \
be understated by it.",
                "1 voucher(s) touching this partner are not read exactly (Journal j1 on \
2026-03-31): this base may be wrong in either direction.",
                "This base (₹19,800) is under the s.194T threshold (₹20,000) as read, but it may be \
wrong (above), so whether it crosses the threshold is not computed.",
                "TDS of ₹4,400 (net) is seen on 1 voucher(s) touching this partner: Journal j1 on \
2026-03-31.",
            ]
        );
        assert_eq!(t[0].ask_client[0], ASK_BASE);
    }

    // ---- a credit net of a ledger this test does not read ----

    #[test]
    fn a_credit_net_of_an_unclassified_ledger_names_the_ledger_and_its_fix() {
        let i1 = || {
            vec![voucher(
                "i1",
                &[
                    ("Interest to Partners", 9_000_000),
                    ("Partner A", -8_100_000),
                    ("TDS on Partners 194T", -900_000),
                ],
            )]
        };
        let config = format!("{ONE}{DEED}");
        let r = go(i1(), &config);
        let h = hash8("partner_a");
        assert_eq!(
            fig(&r, &format!("interest_credited_{h}")).value,
            Value::Int(8_100_000)
        );
        let f = found(&r, &format!("s40b_not_computed/{h}"));
        assert_eq!(
            f[0].limits,
            [
                LEAD.to_string(),
                PLAIN_ONLY.to_string(),
                format!(
                    "1 interest voucher(s) also credit a ledger this test does not read (Journal i1 \
on 2026-03-31; ledger tag(s) {}): the partner may be credited net of it, so the interest counted may \
be understated. If the ledger is TDS on this partner's interest alone, classify it as TDS payable in \
the client's statutory dues and the interest is counted gross; on a voucher that credits several \
partners the TDS is not divided, and if it is a payable, a round-off or a payment, the CA reads the \
interest from the voucher.",
                    hash8("TDS on Partners 194T")
                ),
                CLOSING.to_string(),
            ]
        );
        assert_eq!(
            f[0].evidence,
            [
                EvidenceRef::with_label("voucher", "i1", "Journal i1 on 2026-03-31"),
                EvidenceRef::new("ledger", "TDS on Partners 194T"),
            ]
        );
        assert_eq!(
            f[0].ask_client,
            [
                ASK_RATE,
                ASK_VOUCHERS,
                "If 'TDS on Partners 194T' carries TDS on this partner's interest, classify it as \
TDS payable in your statutory dues; otherwise, what it is.",
            ]
        );
        let t = found(&r, &format!("s194t/{h}"));
        assert_eq!(
            t[0].limits[1],
            "1 interest or remuneration voucher(s) also credit a ledger this test does not read \
(Journal i1 on 2026-03-31): the partner may be credited net of it, so this base may be understated."
        );
        // Classified as TDS payable, the same voucher is plain and read gross.
        let r = go_with(i1(), &config, &["TDS on Partners 194T"], true).unwrap();
        assert!(found(&r, "s40b_not_computed/").is_empty());
        assert_eq!(
            fig(&r, &format!("interest_credited_{h}")).value,
            Value::Int(9_000_000)
        );
    }

    // ---- the plain-voucher gate ----

    #[test]
    fn an_unbalanced_interest_voucher_is_not_plain_and_is_named() {
        let t7 = voucher(
            "t7",
            &[
                ("Interest to Partners", 5_000_000),
                ("Partner A", -4_500_000),
            ],
        );
        let r = go(vec![t7], &format!("{ONE}{DEED}"));
        let f = found(&r, "s40b_not_computed/");
        assert_eq!(
            f[0].limits,
            [
                LEAD,
                "1 voucher(s) on this partner's capital are not of the plain shape this test reads \
exactly (Journal t7 on 2026-03-31): this partner's own interest or remuneration ledger debited, its \
own capital credited, and TDS on a ledger classified as TDS payable, balanced. Each carries a line \
this test does not read as plain, so the interest counted may be wrong in either direction.",
                PLAIN_ONLY,
                CLOSING,
            ]
        );
        assert_eq!(
            f[0].evidence,
            [EvidenceRef::with_label(
                "voucher",
                "t7",
                "Journal t7 on 2026-03-31"
            )]
        );
    }

    #[test]
    fn another_partners_ledger_on_this_capital_is_named_not_read_as_nothing() {
        let config = format!(
            "{ONE}[partner_b]\ncapital_ledgers = [\"Partner B\"]\n\
interest_ledger = \"Interest to Partners\"\n\
remuneration_ledger = \"Remuneration to Partners\"\n{DEED}"
        );
        let o1 = || {
            vec![voucher(
                "o1",
                &[
                    ("Remuneration to Partners", 6_000_000),
                    ("Partner A", -6_000_000),
                ],
            )]
        };
        let r = go(o1(), &config);
        let (ha, hb) = (hash8("partner_a"), hash8("partner_b"));
        let f = found(&r, &format!("s40b_not_computed/{ha}"));
        assert_eq!(
            f[0].title,
            format!(
                "s.40(b) not computed for partner (tag {ha}): its interest credited cannot be read \
exactly; the CA computes it"
            )
        );
        assert_eq!(
            f[0].evidence,
            [EvidenceRef::with_label(
                "voucher",
                "o1",
                "Journal o1 on 2026-03-31"
            )]
        );
        // Not called partner A's remuneration voucher.
        let b = found(&r, "remuneration_book_profit_required");
        assert!(!b[0]
            .limits
            .iter()
            .any(|x| x.contains("remuneration voucher(s) are not of the plain shape")));
        // B's capital is not touched: with the switch on, B alone is computed.
        let r = go_with(o1(), &config, &["TDS Payable"], true).unwrap();
        assert_eq!(found(&r, &format!("s40b_not_computed/{ha}")).len(), 1);
        assert!(found(&r, &format!("s40b_not_computed/{hb}")).is_empty());
    }

    #[test]
    fn a_zero_capital_line_does_not_hide_a_bank_payment() {
        for cap in ["Partner A", "Partner B"] {
            let iz = voucher(
                "iz",
                &[
                    ("Interest to Partners", 7_000_000),
                    ("Cash", -7_000_000),
                    (cap, 0),
                ],
            );
            let r = go(vec![iz], &format!("{TWO}{DEED}"));
            let h = hash8("partner_a");
            let off = found(&r, &format!("off_capital/{h}"));
            assert_eq!(off.len(), 1, "{cap}");
            assert_eq!(
                off[0].evidence,
                [EvidenceRef::with_label(
                    "voucher",
                    "iz",
                    "Journal iz on 2026-03-31"
                )],
                "{cap}"
            );
            let f = found(&r, &format!("s40b_not_computed/{h}"));
            assert!(f[0].evidence.iter().any(|e| e.id == "iz"), "{cap}");
        }
    }

    #[test]
    fn another_partners_own_tds_on_a_shared_ledger_leaves_this_one_no() {
        let a2 = voucher(
            "a2",
            &[
                ("Interest to Partners", 1_200_000),
                ("Partner A", -1_080_000),
                ("TDS Payable", -120_000),
            ],
        );
        let b1 = voucher(
            "b1",
            &[
                ("Interest to Partners", 3_000_000),
                ("Partner B", -3_000_000),
            ],
        );
        let r = go(vec![a2, b1], &format!("{TWO}{DEED}"));
        let hb = hash8("partner_b");
        assert_eq!(
            fig(&r, &format!("s194t_tds_ledger_seen_{hb}")).value,
            Value::Text("no".to_string())
        );
        let t = found(&r, &format!("s194t/{hb}"));
        assert!(
            !t[0].limits.iter().any(|x| x.contains("a2")),
            "{:?}",
            t[0].limits
        );
    }

    // ---- TDS on a ledger shared by partners (queue items 3 and 7, 28-Sep) ----

    /// The selftest's `PartnersBOffCapital.SHARED`: both partners on one interest and one
    /// remuneration ledger.
    const SHARED: &str = "[partner_a]\ncapital_ledgers = [\"Partner A\"]\n\
interest_ledger = \"Interest to Partners\"\n\
remuneration_ledger = \"Remuneration to Partners\"\n\
[partner_b]\ncapital_ledgers = [\"Partner B\"]\n\
interest_ledger = \"Interest to Partners\"\n\
remuneration_ledger = \"Remuneration to Partners\"\n";
    const PARTNER_C: &str = "[partner_c]\ncapital_ledgers = [\"Partner C\"]\n\
interest_ledger = \"Interest to Partners\"\n\
remuneration_ledger = \"Remuneration to Partners\"\n";

    /// [`go`] on a book that also holds a Bank ledger and the `extra` ones.
    fn go_shared(vouchers: Vec<Voucher>, config: &str, extra: &[&str]) -> TestResult {
        let mut book = book_of(vouchers);
        for n in ["Bank"].iter().chain(extra) {
            book.ledgers.insert((*n).to_string(), ledger(n));
        }
        let tds = BTreeSet::from(["TDS Payable".to_string()]);
        run_with(
            &book,
            &Rules::vendored().unwrap(),
            &year(),
            "firm",
            &cfg(config),
            &tds,
            S40B_EXCESS_COMPUTED,
        )
        .unwrap()
    }

    fn tds_seen(r: &TestResult, key: &str) -> String {
        match &fig(r, &format!("s194t_tds_ledger_seen_{}", hash8(key))).value {
            Value::Text(t) => t.clone(),
            other => panic!("not text: {other:?}"),
        }
    }

    const NOT_JUDGED_TAIL: &str = "Which partner's TDS they are is not judged.";

    #[test]
    fn a_zero_capital_line_does_not_decide_whose_shared_tds_it_is() {
        // Only a non-zero line touches a ledger: TDS on a bank payment on a shared ledger is
        // listed for both partners, and a zero line on either capital changes nothing.
        let sb = |zero_on: Option<&str>| {
            let mut lines = vec![
                ("Interest to Partners", 6_000_000),
                ("Bank", -5_400_000),
                ("TDS Payable", -600_000),
            ];
            lines.extend(zero_on.map(|c| (c, 0)));
            voucher("sz", &lines)
        };
        let said = |r: &TestResult, key: &str| {
            let t = found(r, &format!("s194t/{}", hash8(key)));
            assert_eq!(t.len(), 1, "{key}");
            (tds_seen(r, key), t[0].title.clone(), t[0].limits.clone())
        };
        let plain = go_shared(vec![sb(None)], &format!("{SHARED}{DEED}"), &[]);
        for key in ["partner_a", "partner_b"] {
            let (seen, _, limits) = said(&plain, key);
            assert_eq!(seen, "not judged", "{key}");
            assert!(
                limits.iter().any(|x| x.ends_with(NOT_JUDGED_TAIL)),
                "{limits:?}"
            );
        }
        for cap in ["Partner A", "Partner B"] {
            let r = go_shared(vec![sb(Some(cap))], &format!("{SHARED}{DEED}"), &[]);
            for key in ["partner_a", "partner_b"] {
                assert_eq!(said(&r, key), said(&plain, key), "{cap} {key}");
            }
        }
    }

    #[test]
    fn a_zero_line_on_a_shared_ledger_does_not_make_someone_elses_tds_a_partners() {
        // TDS deducted from a supplier, on a voucher whose only line on the shared interest ledger
        // is zero, is no partner's: each says "no", as without that line.
        for zero in [false, true] {
            let mut lines = vec![
                ("Rent", 1_000_000),
                ("Supplier", -900_000),
                ("TDS Payable", -100_000),
            ];
            if zero {
                lines.push(("Interest to Partners", 0));
            }
            let r = go_shared(
                vec![voucher("sp", &lines)],
                &format!("{SHARED}{DEED}"),
                &["Rent", "Supplier"],
            );
            for key in ["partner_a", "partner_b"] {
                assert_eq!(tds_seen(&r, key), "no", "{zero} {key}");
            }
        }
    }

    #[test]
    fn shared_tds_on_a_voucher_crediting_one_partner_and_a_bank_is_listed_for_the_other() {
        // Queue item 7: the TDS's side carries A's capital and a bank, so it is not exactly A's;
        // B, who shares the ledger, lists it and is "not judged".
        let s1 = voucher(
            "s1",
            &[
                ("Interest to Partners", 12_000_000),
                ("Partner A", -5_400_000),
                ("Bank", -5_400_000),
                ("TDS Payable", -1_200_000),
            ],
        );
        let r = go_shared(vec![s1], &format!("{SHARED}{DEED}"), &[]);
        let hb = hash8("partner_b");
        assert_eq!(tds_seen(&r, "partner_b"), "not judged");
        let t = found(&r, &format!("s194t/{hb}"));
        assert!(
            t[0].limits.contains(
                &"TDS lines are seen on 1 voucher(s) on an interest or remuneration ledger shared \
by partners whose TDS is not booked against partners' capitals alone: Journal s1 on 2026-03-31. \
Which partner's TDS they are is not judged."
                    .to_string()
            ),
            "{:?}",
            t[0].limits
        );
        assert!(t[0].evidence.iter().any(|e| e.id == "s1"));
        assert_eq!(tds_seen(&r, "partner_a"), "yes");
        // The definition states the rule.
        assert!(fig(&r, &format!("s194t_tds_ledger_seen_{hb}"))
            .definition
            .ends_with(
                "(\"not judged\" when none is classified, or when the only such lines are on a \
ledger shared by partners, on a voucher whose TDS is not booked against partners' capitals alone)."
            ));
    }

    #[test]
    fn shared_tds_seen_on_a_partners_own_capital_is_not_also_listed_as_unattributed() {
        // Kills PR-09 (`!seen.contains_key(&v.guid)` -> `true`): s1 touches A's capital and
        // carries TDS, so it is in A's `seen`; it is also on the shared interest ledger with a bank
        // on the TDS's side (not partners' TDS), so only the `seen` guard keeps it out of A's
        // `unattributed`. The reference lists it for A as seen, never as "not judged".
        let s1 = voucher(
            "s1",
            &[
                ("Interest to Partners", 12_000_000),
                ("Partner A", -5_400_000),
                ("Bank", -5_400_000),
                ("TDS Payable", -1_200_000),
            ],
        );
        let r = go_shared(vec![s1], &format!("{SHARED}{DEED}"), &[]);
        let ha = hash8("partner_a");
        assert_eq!(tds_seen(&r, "partner_a"), "yes");
        let t = found(&r, &format!("s194t/{ha}"));
        assert_eq!(t.len(), 1);
        assert_eq!(
            t[0].title,
            "Payments/credits to a partner over the s.194T threshold, with TDS seen on its \
vouchers: which credits it covers, and its deposit, are the CA's to determine"
        );
        assert_eq!(
            t[0].limits,
            [
                "Books only: TAN registration, challans filed and any Form 26A route are not \
visible from vouchers.",
                "1 interest or remuneration voucher(s) also credit a ledger this test does not read \
(Journal s1 on 2026-03-31): the partner may be credited net of it, so this base may be understated.",
                "1 interest or remuneration voucher(s) carry TDS but also credit another party \
(Journal s1 on 2026-03-31): their TDS is not added back, so the interest or remuneration counted may \
be understated by it.",
                "1 voucher(s) touching this partner are not read exactly (Journal s1 on \
2026-03-31): this base may be wrong in either direction.",
                "TDS of ₹12,000 (net) is seen on 1 voucher(s) touching this partner: Journal s1 on \
2026-03-31.",
            ]
        );
        assert_eq!(
            t[0].evidence,
            [EvidenceRef::with_label(
                "voucher",
                "s1",
                "Journal s1 on 2026-03-31"
            )]
        );
    }

    #[test]
    fn shared_tds_on_a_remuneration_voucher_crediting_two_partners_is_cited() {
        // Kills PR-12 (`remuneration_evidence.extend(shared_rem.clone())` dropped): r2's TDS side
        // carries both partners' capitals, so for each it is a shared-TDS remuneration voucher.
        // It is non-plain, but `other_rem` leaves shared-TDS vouchers out, it is not an interest
        // voucher, and another partner's capital is no unread ledger: only `shared_rem` cites it.
        let r2 = voucher(
            "r2",
            &[
                ("Remuneration to Partners", 10_000_000),
                ("Partner A", -4_500_000),
                ("Partner B", -4_500_000),
                ("TDS Payable", -1_000_000),
            ],
        );
        let r = go(vec![r2], &format!("{SHARED}{DEED}"));
        let (ha, hb) = (hash8("partner_a"), hash8("partner_b"));
        for h in [&ha, &hb] {
            assert_eq!(
                fig(&r, &format!("remuneration_credited_{h}")).value,
                Value::Int(4_500_000)
            );
        }
        let b = found(&r, "remuneration_book_profit_required");
        assert_eq!(b.len(), 1);
        assert_eq!(
            b[0].evidence,
            [EvidenceRef::with_label(
                "voucher",
                "r2",
                "Journal r2 on 2026-03-31"
            )]
        );
        let note = |h: &str| {
            format!(
                "Partner (tag {h}): 1 remuneration voucher(s) carry TDS but also credit another \
party (Journal r2 on 2026-03-31): the TDS is not divided between them, so remuneration_credited_{h} \
is net of an unknown part of it."
            )
        };
        assert_eq!(b[0].limits[2..], [note(&ha), note(&hb)]);
    }

    #[test]
    fn shared_tds_exactly_one_partners_stays_that_partners() {
        let a1 = voucher(
            "a1",
            &[
                ("Interest to Partners", 6_000_000),
                ("Partner A", -5_400_000),
                ("TDS Payable", -600_000),
            ],
        );
        let r = go_shared(vec![a1], &format!("{SHARED}{DEED}"), &[]);
        assert_eq!(tds_seen(&r, "partner_b"), "no");
        assert_eq!(tds_seen(&r, "partner_a"), "yes");
    }

    #[test]
    fn tds_credited_against_nothing_else_is_no_partners() {
        let t0 = voucher(
            "t0",
            &[("Interest to Partners", 600_000), ("TDS Payable", -600_000)],
        );
        let r = go_shared(vec![t0], &format!("{SHARED}{DEED}"), &[]);
        for key in ["partner_a", "partner_b"] {
            assert_eq!(tds_seen(&r, key), "not judged", "{key}");
        }
    }

    #[test]
    fn a_net_nil_tds_on_one_partners_voucher_is_not_listed_for_the_other() {
        // No TDS side: it stays with the partner whose capital it touches.
        let z1 = voucher(
            "z1",
            &[
                ("Interest to Partners", 6_000_000),
                ("Partner A", -6_000_000),
                ("TDS Payable", 600_000),
                ("TDS Payable", -600_000),
            ],
        );
        let r = go_shared(vec![z1], &format!("{SHARED}{DEED}"), &[]);
        assert_eq!(tds_seen(&r, "partner_b"), "no");
    }

    #[test]
    fn a_net_nil_tds_on_a_bank_payment_is_listed_for_both() {
        // A zero line on a capital does not count as touching it.
        let zb = voucher(
            "zb",
            &[
                ("Interest to Partners", 6_000_000),
                ("Bank", -6_000_000),
                ("TDS Payable", 600_000),
                ("TDS Payable", -600_000),
                ("Partner A", 0),
            ],
        );
        let r = go_shared(vec![zb], &format!("{SHARED}{DEED}"), &[]);
        for key in ["partner_a", "partner_b"] {
            assert_eq!(tds_seen(&r, key), "not judged", "{key}");
        }
    }

    #[test]
    fn a_reversal_of_one_partners_voucher_is_not_listed_for_the_other() {
        // Signed: the TDS debited, its side is A's capital debited, not the interest ledger.
        let rv = voucher(
            "rv",
            &[
                ("Interest to Partners", -6_000_000),
                ("Partner A", 5_400_000),
                ("TDS Payable", 600_000),
            ],
        );
        let r = go_shared(vec![rv], &format!("{SHARED}{DEED}"), &[]);
        assert_eq!(tds_seen(&r, "partner_b"), "no");
    }

    #[test]
    fn a_joint_journal_is_not_listed_for_a_partner_it_does_not_credit() {
        let j1 = voucher(
            "j1",
            &[
                ("Interest to Partners", 10_000_000),
                ("Partner A", -4_500_000),
                ("Partner B", -4_500_000),
                ("TDS Payable", -1_000_000),
            ],
        );
        let r = go_shared(
            vec![j1],
            &format!("{SHARED}{PARTNER_C}{DEED}"),
            &["Partner C"],
        );
        assert_eq!(tds_seen(&r, "partner_c"), "no");
        for key in ["partner_a", "partner_b"] {
            assert_eq!(tds_seen(&r, key), "yes", "{key}");
        }
    }

    // ---- item 7's off-capital rule, with no TDS on the voucher (queue item 10, 28-Sep) ----

    #[test]
    fn a_shared_interest_voucher_crediting_one_partner_and_a_bank_is_listed_for_the_other() {
        // Shared interest 1,20,000 credited 60,000 to A's capital and paid 60,000 by bank, no
        // TDS. The bank half may be B's interest: B lists the voucher and is not computed.
        // Before, a non-zero line on any partner's capital left it out of B's off-capital check,
        // so none of B's findings named it: a silent miss for B.
        let s2 = voucher(
            "s2",
            &[
                ("Interest to Partners", 12_000_000),
                ("Partner A", -6_000_000),
                ("Bank", -6_000_000),
            ],
        );
        let r = go_shared(vec![s2], &format!("{SHARED}{DEED}"), &[]);
        let hb = hash8("partner_b");
        let off = found(&r, &format!("off_capital/{hb}"));
        assert_eq!(off.len(), 1);
        assert!(off[0].evidence.iter().any(|e| e.id == "s2"));
        assert!(
            off[0]
                .limits
                .iter()
                .any(|x| x.contains("shared by partners")),
            "{:?}",
            off[0].limits
        );
        assert!(
            off[0]
                .limits
                .iter()
                .any(|x| x.contains("not booked against partners' capitals alone")),
            "{:?}",
            off[0].limits
        );
        let nc = found(&r, &format!("s40b_not_computed/{hb}"));
        assert_eq!(nc.len(), 1);
        assert!(nc[0].evidence.iter().any(|e| e.id == "s2"));
        // A's own capital is on it: A reads it, not as off-capital.
        assert!(found(&r, &format!("off_capital/{}", hash8("partner_a"))).is_empty());
    }

    #[test]
    fn own_interest_paid_partly_through_another_partners_capital_and_a_bank_is_listed() {
        let own_b = "[partner_a]\ncapital_ledgers = [\"Partner A\"]\n\
[partner_b]\ncapital_ledgers = [\"Partner B\"]\ninterest_ledger = \"Interest to B\"\n";
        let o2 = voucher(
            "o2",
            &[
                ("Interest to B", 5_000_000),
                ("Partner A", -2_000_000),
                ("Bank", -3_000_000),
            ],
        );
        let r = go_shared(vec![o2], own_b, &["Interest to B"]);
        let off = found(&r, &format!("off_capital/{}", hash8("partner_b")));
        assert_eq!(off.len(), 1);
        assert!(off[0].evidence.iter().any(|e| e.id == "o2"));
        assert!(
            !off[0]
                .limits
                .iter()
                .any(|x| x.contains("shared by partners")),
            "{:?}",
            off[0].limits
        );
    }

    #[test]
    fn a_shared_interest_voucher_against_one_partners_capital_and_tds_is_not_listed_for_the_other()
    {
        // The TDS is set aside: the rest of that side is A's capital alone, so it is A's (V2 on
        // #820, P2-2).
        let a2 = voucher(
            "a2",
            &[
                ("Interest to Partners", 6_000_000),
                ("Partner A", -5_400_000),
                ("TDS Payable", -600_000),
            ],
        );
        let r = go_shared(vec![a2], &format!("{SHARED}{DEED}"), &[]);
        assert!(found(&r, &format!("off_capital/{}", hash8("partner_b"))).is_empty());
    }

    #[test]
    fn a_joint_journal_without_tds_is_not_off_capital_for_a_partner_it_does_not_credit() {
        let j2 = voucher(
            "j2",
            &[
                ("Interest to Partners", 9_000_000),
                ("Partner A", -4_500_000),
                ("Partner C", -4_500_000),
            ],
        );
        let r = go_shared(
            vec![j2],
            &format!("{SHARED}{PARTNER_C}{DEED}"),
            &["Partner C"],
        );
        assert!(found(&r, &format!("off_capital/{}", hash8("partner_b"))).is_empty());
    }

    #[test]
    fn a_reversal_through_one_partners_capital_is_not_listed_for_the_other() {
        // A's capital debited against the shared ledger credited: the side opposite the interest
        // is A's capital alone, so it is A's reversal.
        let r2 = voucher(
            "r2",
            &[
                ("Partner A", 3_000_000),
                ("Interest to Partners", -3_000_000),
            ],
        );
        let r = go_shared(vec![r2], &format!("{SHARED}{DEED}"), &[]);
        assert!(found(&r, &format!("off_capital/{}", hash8("partner_b"))).is_empty());
    }

    #[test]
    fn a_shared_interest_voucher_whose_other_side_is_only_tds_beside_a_capital_is_listed() {
        // Interest against TDS alone, with A's capital debited on the same side as the interest:
        // nothing but TDS is on the interest's other side, so it names no one and is listed for B
        // (an empty side is no partner's).
        let t2 = voucher(
            "t2",
            &[
                ("Interest to Partners", 600_000),
                ("Partner A", 100_000),
                ("TDS Payable", -700_000),
            ],
        );
        let r = go_shared(vec![t2], &format!("{SHARED}{DEED}"), &[]);
        assert!(!found(&r, &format!("off_capital/{}", hash8("partner_b"))).is_empty());
    }

    #[test]
    fn shared_lines_netting_to_nil_keep_the_rule_before() {
        // A reclass on the shared ledger netting to nil leaves no side: a capital touched leaves
        // it out, as before.
        let n2 = voucher(
            "n2",
            &[
                ("Interest to Partners", 500_000),
                ("Interest to Partners", -500_000),
                ("Partner A", 100_000),
                ("Bank", -100_000),
            ],
        );
        let r = go_shared(vec![n2], &format!("{SHARED}{DEED}"), &[]);
        assert!(found(&r, &format!("off_capital/{}", hash8("partner_b"))).is_empty());
    }

    #[test]
    fn a_zero_line_on_a_capital_touches_no_capital_so_the_voucher_is_listed_for_the_other_partner()
    {
        // Shared interest booked and reversed (nets to nil) beside a ZERO line on A's capital.
        // Only a non-zero line touches a ledger (the reference's walk), so no capital is touched
        // and the voucher is off-capital for B, though its lines net to nil.
        let z2 = voucher(
            "z2",
            &[
                ("Interest to Partners", 500_000),
                ("Interest to Partners", -500_000),
                ("Partner A", 0),
            ],
        );
        let r = go_shared(vec![z2], &format!("{SHARED}{DEED}"), &[]);
        let off = found(&r, &format!("off_capital/{}", hash8("partner_b")));
        assert_eq!(off.len(), 1);
        assert!(off[0].evidence.iter().any(|e| e.id == "z2"));
    }

    #[test]
    fn a_one_paisa_line_on_a_capital_still_touches_it_and_leaves_the_voucher_out_for_the_other_partner(
    ) {
        // The same shape with A's capital carrying a single paisa (balanced by the bank): a
        // non-zero line, whatever its size, touches the capital.
        let z3 = voucher(
            "z3",
            &[
                ("Interest to Partners", 500_000),
                ("Interest to Partners", -500_000),
                ("Partner A", 1),
                ("Bank", -1),
            ],
        );
        let r = go_shared(vec![z3], &format!("{SHARED}{DEED}"), &[]);
        assert!(found(&r, &format!("off_capital/{}", hash8("partner_b"))).is_empty());
    }

    #[test]
    fn a_shared_remuneration_voucher_crediting_one_partner_and_a_bank_is_listed_for_the_other() {
        let m2 = voucher(
            "m2",
            &[
                ("Remuneration to Partners", 10_000_000),
                ("Partner A", -5_000_000),
                ("Bank", -5_000_000),
            ],
        );
        let r = go_shared(vec![m2], &format!("{SHARED}{DEED}"), &[]);
        let off = found(&r, &format!("off_capital/{}", hash8("partner_b")));
        assert_eq!(off.len(), 1);
        assert!(off[0].evidence.iter().any(|e| e.id == "m2"));
    }

    #[test]
    fn remuneration_net_of_an_unread_ledger_is_said_on_the_remuneration_finding() {
        let config = format!("{ONE}remuneration_ledger = \"Remuneration to Partners\"\n{DEED}");
        let r1 = voucher(
            "r1",
            &[
                ("Remuneration to Partners", 2_000_000),
                ("Partner A", -1_800_000),
                ("TDS on Partners 194T", -200_000),
            ],
        );
        let r = go(vec![r1], &config);
        let h = hash8("partner_a");
        let b = found(&r, "remuneration_book_profit_required");
        assert_eq!(
            b[0].limits[2..],
            [format!(
                "Partner (tag {h}): 1 remuneration voucher(s) also credit a ledger this test does \
not read (Journal r1 on 2026-03-31): remuneration_credited_{h} may be net of it. If that ledger is \
TDS on this partner's remuneration alone, classify it as TDS payable in the client's statutory dues."
            )]
        );
        assert_eq!(
            b[0].evidence,
            [EvidenceRef::with_label(
                "voucher",
                "r1",
                "Journal r1 on 2026-03-31"
            )]
        );
    }

    #[test]
    fn remuneration_left_inside_an_unsplit_voucher_is_said() {
        let config = format!("{ONE}remuneration_ledger = \"Remuneration to Partners\"\n{DEED}");
        let m1 = voucher(
            "m1",
            &[
                ("Interest to Partners", 400_000),
                ("Remuneration to Partners", 2_000_000),
                ("Partner A", -2_400_000),
                ("Partner A", 100_000),
                ("Cash", -100_000),
            ],
        );
        let r = go(vec![m1], &config);
        let h = hash8("partner_a");
        let rem = fig(&r, &format!("remuneration_credited_{h}"));
        assert_eq!(rem.value, Value::Int(0));
        assert!(rem.definition.ends_with(
            " A voucher carrying both the interest and the remuneration ledger that is not split \
is counted as interest, not here."
        ));
        let b = found(&r, "remuneration_book_profit_required");
        assert_eq!(
            b[0].limits[2..],
            [format!(
                "Partner (tag {h}): 1 voucher(s) carrying both the interest and the remuneration \
ledger are not split (Journal m1 on 2026-03-31): their credit is counted as interest, so \
remuneration_credited_{h} leaves out the remuneration in them."
            )]
        );
    }

    // ---- the owner's 4a: a deed that authorises no interest on capital ----

    #[test]
    fn a_no_interest_deed_states_the_disallowance_and_asks_nothing_without_an_interest_ledger() {
        let books = vec![
            voucher_on(
                "k1",
                "20250601",
                &[("Cash", 5_000_000), ("Partner A", -5_000_000)],
            ),
            voucher(
                "r1",
                &[
                    ("Remuneration to Partners", 12_000_000),
                    ("Partner A", -12_000_000),
                ],
            ),
        ];
        let r = go(
            books,
            &format!("{NOINT}[deed]\nno_interest_authorised = true\n"),
        );
        let h = hash8("partner_a");
        let f = found(&r, &format!("s40b_not_computed/{h}"));
        assert_eq!(f.len(), 1);
        assert_eq!(
            f[0].title,
            format!(
                "s.40(b) not computed for partner (tag {h}): the deed authorises no interest on \
capital, so any interest is disallowed in full"
            )
        );
        assert_eq!(
            f[0].limits,
            [
                LEAD,
                "The deed authorises no interest on capital (as recorded in the configuration), so \
s.40(b) allows none: interest paid or credited to this partner, through any ledger (its \
remuneration ledger included), is disallowed in full. This test does not search the books for such \
interest, and states no disallowed amount.",
                CLOSING,
            ]
        );
        assert!(f[0].ask_client.is_empty());
        assert_eq!(f[0].evidence, [EvidenceRef::new("ledger", "Partner A")]);
        assert_eq!(fig(&r, "interest_rate_bp_used").value, Value::Int(0));
        assert_eq!(
            fig(&r, "interest_rate_bp_used").definition,
            "Rate used for s.40(b) allowable interest: 0, as the deed authorises no interest on \
capital (recorded in the configuration)."
        );
        assert!(found(&r, "deed_rate_missing").is_empty());
        assert!(found(&r, "deed_missing").is_empty());
        assert!(!has_fig(&r, &format!("s40b_excess_{h}")));
        assert!(!has_fig(&r, "s40b_excess_total"));
        let t = found(&r, &format!("s194t/{h}"));
        assert!(t[0].limits.contains(
            &"No interest ledger is configured for this partner, so interest credited to its \
capital is not in this base, which may therefore be understated."
                .to_string()
        ));
    }

    #[test]
    fn a_no_interest_deed_with_an_interest_ledger_asks_for_the_interest_not_the_rate() {
        let i1 = voucher(
            "i1",
            &[
                ("Interest to Partners", 6_000_000),
                ("Partner A", -6_000_000),
            ],
        );
        let r = go(
            vec![i1],
            &format!("{ONE}[deed]\nno_interest_authorised = true\n"),
        );
        let h = hash8("partner_a");
        let f = found(&r, &format!("s40b_not_computed/{h}"));
        assert_eq!(
            f[0].title,
            format!(
                "s.40(b) not computed for partner (tag {h}): this test computes the excess for no \
partner; the CA computes it"
            )
        );
        assert_eq!(
            f[0].limits[1],
            "The deed authorises no interest on capital (as recorded in the configuration), so \
s.40(b) allows none: interest paid or credited to this partner, through any ledger (its \
remuneration ledger included), is disallowed in full. This test does not search the books for such \
interest, beyond reading its configured interest ledger as a working figure (marked not usable), \
and states no disallowed amount."
        );
        assert_eq!(
            f[0].ask_client,
            ["The s.40(b) computation for this partner: the interest credited (the deed authorises \
none, so all of it is disallowed)."]
        );
        assert_eq!(
            fig(&r, &format!("interest_credited_{h}")).value,
            Value::Int(6_000_000)
        );
        assert_eq!(
            fig(&r, &format!("allowable_interest_{h}")).value,
            Value::Int(0)
        );
    }

    #[test]
    fn a_no_interest_flag_set_false_is_the_same_as_not_recorded() {
        let r = go(
            Vec::new(),
            &format!("{NOINT}[deed]\nno_interest_authorised = false\n"),
        );
        assert_eq!(found(&r, "deed_rate_missing").len(), 1);
        assert_eq!(fig(&r, "interest_rate_bp_used").value, Value::Int(1200));
        let f = found(&r, "s40b_not_computed/");
        assert_eq!(
            f[0].title,
            format!(
                "s.40(b) not computed for partner (tag {}): its interest credited cannot be read \
exactly; the CA computes it",
                hash8("partner_a")
            )
        );
        assert_eq!(
            f[0].limits[1],
            "No interest ledger is configured for this partner, so interest credited to its capital \
cannot be told from capital introduced: the capital walk reads every credit to it as capital, \
except remuneration on a configured remuneration ledger, and no interest credited is counted. If \
the deed authorises no interest on capital, the configuration can record that ([partners.deed] \
no_interest_authorised)."
        );
        assert_eq!(
            f[0].ask_client,
            [ASK_RATE, "The ledger that carries this partner's interest."]
        );
    }

    #[test]
    fn a_no_interest_deed_partner_is_not_computed_with_the_switch_on_either() {
        // Queue item 2 (28-Sep): under the flag a partner is never computed, whatever the switch.
        // With the switch on, a plain interest voucher gave a computed excess (rate 0) whose limit
        // said interest "was authorised for the whole year" and asked for the deed's rate.
        let i1 = voucher(
            "i1",
            &[
                ("Interest to Partners", 6_000_000),
                ("Partner A", -6_000_000),
            ],
        );
        let r = go_with(
            vec![i1],
            &format!("{ONE}[deed]\nno_interest_authorised = true\n"),
            &["TDS Payable"],
            true,
        )
        .unwrap();
        let h = hash8("partner_a");
        assert!(found(&r, "s40b_excess/").is_empty());
        assert!(!has_fig(&r, &format!("s40b_excess_{h}")));
        assert!(!has_fig(&r, "s40b_excess_total"));
        assert_eq!(
            fig(&r, "s40b_excess_total_not_computed").value,
            Value::Text("not computed: s.40(b) is not computed for 1 partner(s)".to_string())
        );
        let f = found(&r, &format!("s40b_not_computed/{h}"));
        assert_eq!(f.len(), 1);
        assert_eq!(
            f[0].title,
            format!(
                "s.40(b) not computed for partner (tag {h}): the deed authorises no interest on \
capital, so any interest is disallowed in full"
            )
        );
        assert_eq!(
            f[0].limits,
            [
                "The deed authorises no interest on capital (as recorded in the configuration), so \
s.40(b) allows none: interest paid or credited to this partner, through any ledger (its \
remuneration ledger included), is disallowed in full. This test does not search the books for such \
interest, beyond reading its configured interest ledger as a working figure (marked not usable), \
and states no disallowed amount.",
                CLOSING,
            ]
        );
        assert_eq!(
            f[0].ask_client,
            ["The s.40(b) computation for this partner: the interest credited (the deed authorises \
none, so all of it is disallowed)."]
        );
        for g in &r.findings {
            assert!(
                !g.limits
                    .iter()
                    .any(|x| x.contains("authorised for the whole year")),
                "{}",
                g.id
            );
            assert!(
                !g.ask_client
                    .join(" ")
                    .contains("the deed's authorised interest rate"),
                "{}",
                g.id
            );
        }
    }

    #[test]
    fn with_the_switch_on_a_rate_deed_partner_read_inexactly_is_not_asked_for_the_computation() {
        // The other half of the question's condition: with the switch on, a partner whose deed
        // authorises a rate and whose interest cannot be read is asked about the vouchers named,
        // not for the s.40(b) computation.
        let x1 = voucher(
            "x1",
            &[
                ("Interest to Partners", 1_000_000),
                ("Partner A", -800_000),
                ("Cash", -200_000),
            ],
        );
        let r = go_with(
            vec![x1],
            &format!("{ONE}[deed]\ninterest_rate_bp = 1200\n"),
            &["TDS Payable"],
            true,
        )
        .unwrap();
        let f = found(&r, "s40b_not_computed/");
        assert_eq!(f.len(), 1);
        assert!(f[0].evidence.iter().any(|e| e.id == "x1"));
        assert_eq!(
            f[0].ask_client,
            [
                ASK_VOUCHERS,
                "If 'Cash' carries TDS on this partner's interest, classify it as TDS payable in \
your statutory dues; otherwise, what it is.",
            ]
        );
    }

    // ---- production wiring: the TDS ledgers reach the module (P2-4) ----

    #[test]
    fn the_statutory_dues_tds_ledgers_reach_the_module_through_the_engagement() {
        let engagement = |extra: &str| {
            crate::Engagement::from_toml(
                &format!(
                    "[client]\nlabel = \"Test\"\nassessment_year = \"2026-27\"\n\
entity_type = \"firm\"\n\
[period]\nstart = \"2025-04-01\"\nend = \"2026-03-31\"\n\
[snapshot]\nformat = \"tally-read-v1\"\npath = \"unused\"\n\
[roles]\ncash_groups = []\nbank_groups = []\n\
[partners.partner_a]\ncapital_ledgers = [\"Partner A\"]\n\
interest_ledger = \"Interest to Partners\"\n\
[partners.deed]\ninterest_rate_bp = 1000\n{extra}"
                ),
                std::path::Path::new("."),
            )
            .unwrap()
        };
        let book = book_of(vec![voucher(
            "i1",
            &[
                ("Interest to Partners", 9_000_000),
                ("Partner A", -8_100_000),
                ("TDS Payable", -900_000),
            ],
        )]);
        let rules = Rules::vendored().unwrap();
        let h = hash8("partner_a");
        let value = |dump: &serde_json::Value, name: &str| {
            let id = format!("{TEST_ID}.{name}");
            dump["figures"]
                .as_array()
                .unwrap()
                .iter()
                .find(|f| f["id"] == id)
                .unwrap_or_else(|| panic!("no figure {id}"))["value"]
                .clone()
        };
        let classified =
            engagement("[statutory_dues.nature_by_ledger]\n\"TDS Payable\" = \"tds_payable\"\n");
        let dump = crate::partners_40b_194t_on(&classified, &book, &rules).unwrap();
        // Read gross of the TDS on the ledger [statutory_dues] classifies, and seen.
        assert_eq!(value(&dump, &format!("interest_credited_{h}")), 9_000_000);
        assert_eq!(value(&dump, &format!("s194t_tds_ledger_seen_{h}")), "yes");
        // The control: unclassified, the same voucher is read net and the TDS is not judged.
        let dump = crate::partners_40b_194t_on(&engagement(""), &book, &rules).unwrap();
        assert_eq!(value(&dump, &format!("interest_credited_{h}")), 8_100_000);
        assert_eq!(
            value(&dump, &format!("s194t_tds_ledger_seen_{h}")),
            "not judged"
        );
    }

    #[test]
    fn a_malformed_partner_configuration_is_refused_only_where_the_test_applies() {
        // The reference checks the configuration after the applicability gate: a company never
        // reads it, a firm refuses it before reading any voucher.
        let bad = "[partner_a]\ncapital_ledgers = []\n";
        let rules = Rules::vendored().unwrap();
        let r = run(
            &unreadable_book(),
            &rules,
            &year(),
            "company",
            &cfg(bad),
            &BTreeSet::new(),
        )
        .unwrap();
        assert_eq!(r.figures.len(), 1);
        config_refusal(run(
            &unreadable_book(),
            &rules,
            &year(),
            "firm",
            &cfg(bad),
            &BTreeSet::new(),
        ));
    }

    #[test]
    fn with_the_switch_on_a_deed_without_its_rate_is_judgement_required() {
        // Behind S40B_EXCESS_COMPUTED: the statutory cap stands in for the deed's rate, and the
        // excess finding says which of the two is missing (the reference's limits and confidence).
        let h = hash8("partner_a");
        // Rs 5,00,000 of interest credited, over the allowable at any rate the rules carry.
        let plain = || {
            vec![voucher(
                "p1",
                &[
                    ("Interest to Partners", 50_000_000),
                    ("Partner A", -50_000_000),
                ],
            )]
        };
        let excess = |config: &str| {
            let r = go_with(plain(), config, &["TDS Payable"], true).unwrap();
            let e = found(&r, "s40b_excess/");
            assert_eq!(e.len(), 1);
            (e[0].confidence, e[0].limits[0].clone(), e[0].id.clone())
        };
        let (c, l, id) = excess(ONE);
        assert_eq!(id, format!("{TEST_ID}/s40b_excess/{h}"));
        assert_eq!(c, Confidence::JudgementRequired);
        assert_eq!(
            l,
            "No deed was available; the excess shown uses the statutory cap as the assumed \
authorised rate -- confirm the deed's actual rate and terms before relying on this figure."
        );
        let (c, l, _) = excess(&format!("{ONE}[deed]\n"));
        assert_eq!(c, Confidence::JudgementRequired);
        assert_eq!(
            l,
            "The deed's interest rate is not recorded; the excess shown uses the statutory cap as \
the assumed authorised rate -- confirm the deed's actual rate and terms before relying on this \
figure."
        );
        let (c, l, _) = excess(&format!("{ONE}{DEED}"));
        assert_eq!(c, Confidence::Computed);
        assert_eq!(
            l,
            "Allowable interest here assumes the capital base is exactly the capital ledger(s) \
supplied and that interest was authorised for the whole year; confirm both against the deed."
        );
    }

    #[test]
    fn the_total_is_withheld_when_one_partner_is_computed_and_another_is_not() {
        // Ported from the reference's PlainVoucherGate (queue item 10, 28-Sep): B's voucher is odd
        // by a set-off on B's own capital (two lines on "Partner B"), so the side opposite the
        // interest is partners' capitals alone and A is not affected (a cash line there would list
        // it for A too, as off-capital).
        let plain = voucher(
            "p1",
            &[
                ("Interest to Partners", 50_000_000),
                ("Partner A", -50_000_000),
            ],
        );
        let odd = voucher(
            "p2",
            &[
                ("Interest to Partners", 5_000_000),
                ("Partner B", -6_000_000),
                ("Partner B", 1_000_000),
            ],
        );
        let r = go_with(
            vec![plain, odd],
            &format!("{TWO}{DEED}"),
            &["TDS Payable"],
            true,
        )
        .unwrap();
        let ha = hash8("partner_a");
        let hb = hash8("partner_b");
        assert!(found(&r, &format!("s40b_not_computed/{ha}")).is_empty());
        assert!(!found(&r, &format!("s40b_not_computed/{hb}")).is_empty());
        assert!(has_fig(&r, &format!("s40b_excess_{ha}")));
        assert!(!has_fig(&r, "s40b_excess_total"));
        assert_eq!(
            fig(&r, "s40b_excess_total_not_computed").value,
            Value::Text("not computed: s.40(b) is not computed for 1 partner(s)".to_string())
        );
        // Implied by the Python: A's own capital never appears off-capital here, since p2's side
        // opposite A's interest ledger is B's capital alone.
        assert!(found(&r, &format!("off_capital/{ha}")).is_empty());
    }

    #[test]
    fn a_side_test_on_lines_of_extreme_amounts_does_not_overflow() {
        // Only the sign of a line and of the net it is compared with decides the side; their
        // product must never be formed. Four lines of i64::MAX net to 2^65 - 4, and a line of
        // that size times it exceeds i128::MAX. The overflow shows as a panic in a debug build (the
        // test profile), which is how this test fails without the fix.
        let big = i64::MAX;
        let own_side = voucher(
            "x1",
            &[
                ("Interest to Partners", big),
                ("Interest to Partners", big),
                ("Interest to Partners", big),
                ("Interest to Partners", big),
                ("Partner B", i64::MIN),
            ],
        );
        let tds_side = voucher(
            "x2",
            &[
                ("TDS Payable", big),
                ("TDS Payable", big),
                ("TDS Payable", big),
                ("TDS Payable", big),
                ("Interest to Partners", big),
            ],
        );
        // Partner A's walk (both sites) finishes; partner B's pass then meets a credit of 2^63
        // (the negated i64::MIN), which no longer fits i64, and refuses it with a typed error.
        let refusal = config_refusal(go_with(
            vec![own_side, tds_side],
            &format!("{TWO}{DEED}"),
            &["TDS Payable"],
            S40B_EXCESS_COMPUTED,
        ));
        assert!(refusal.contains("overflowed i64 paise"), "{refusal}");
    }
}
