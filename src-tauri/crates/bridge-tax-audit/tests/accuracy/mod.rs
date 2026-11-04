// SPDX-License-Identifier: Apache-2.0
//! Item-level scoring of one canonical dump against a sealed answer key (#738).
//!
//! A key is written by a reader who saw only the book, so it names only what such a reader can
//! see. Findings are named by **family**: the fixed text of a finding id's `format!` in `src/`, up
//! to its first placeholder (`partners_40b_194t/off_capital/`), or the whole id when it has none
//! (`stock/typed_in`). The ids' hash tags, a voucher's GUID and the engine's label prose are not
//! visible to that reader, and nothing here matches on them. For each family the key gives the
//! evidence items the reader expects listed, each with a reason, and a score compares them with
//! the items of all that family's findings in the dump together, counted with repeats: an item the
//! key names twice matches at most two listed copies. What is lost is which finding of a family
//! listed an item. Figures are not scored; parity covers them.
//!
//! Items:
//! - a `ledger` by its name, which is the dump ref's id;
//! - a `voucher` by its type, number and date. Each dump voucher ref is resolved by its GUID
//!   through the Book the run read (`Book::vouchers`), never from its label. A ref with an empty
//!   GUID, or a GUID the Book gives to vouchers that differ in type, number or date, is counted as
//!   unresolved, never matched.
//! - Every other kind is refused in a key until it is ruled on. A dump's refs of those kinds are
//!   counted per kind, as not scored.
//!
//! A key is sealed: it opens only with the sha256 given for it, so a key whose bytes changed after
//! that hash was taken does not open (when the hash was taken is outside what the scorer can see).
//! Names, types and numbers are NFC-normalised on both sides; nothing is trimmed. A score describes
//! one run against one key for one synthetic book, and its three inputs (the key, the dump and the
//! Book) come from that run.
//!
//! The scorer lives under `tests/` because only the crate's tests run it: a module under `src/`
//! would change `src/lib.rs`, which would select every mutation that targets that file (the
//! mutation runner under `parity/`).
//!
//! The key format, in TOML:
//!
//! ```toml
//! test_id = "stale_balances_41_1"
//!
//! [[family]]
//! prefix = "stale_balances_41_1/s41_1/"
//!
//! [[family.item]]
//! kind = "ledger"
//! name = "A ledger name, exactly as the book spells it"
//! reason = "Why the reader expects it listed"
//!
//! [[family.item]]
//! kind = "voucher"
//! voucher_type = "Payment"
//! number = "P/18"                  # as the book shows it; "" for a voucher with no number
//! date = "2025-06-27"
//! reason = "Why the reader expects it listed"
//! ```
//!
//! A family with no `item` is one the reader expects to list nothing.

use std::collections::BTreeMap;

use bridge_tax_audit::book::Book;
use bridge_tax_audit::canonical::{hex, nfc};
use bridge_tax_audit::read::iso;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// Every finding family `src/` emits: the text of each finding id's `format!("{TEST_ID}/...")`
/// with the test id written out, cut at its first placeholder. One that ends at a placeholder is
/// open and holds every id it begins; one with no placeholder is closed and holds that id alone.
/// A dump finding belongs to the closed family equal to its id, or else to the longest open family
/// it begins with: several formats put their placeholder straight after the test id
/// (`high_value_register/{rid}`), so that family begins its siblings' ids too.
/// `the_families_are_the_format_prefixes_of_src` checks this list against `src/`.
pub const FAMILIES: &[&str] = &[
    "applicability_44ab/applicability",
    "applicability_44ab/presumptive_history",
    "applicability_44ab/profession_44ab_b",
    "bank_reconciliation/not_found",
    "bank_reconciliation/opening_closing_tie",
    "bank_reconciliation/split_settlement",
    "bank_reconciliation/statement_does_not_add_up",
    "bank_reconciliation/statement_refused",
    "book_keeping_quality/contra_direction/{",
    "book_keeping_quality/debtor_writeoff",
    "book_keeping_quality/entry_order/after_last_sale",
    "book_keeping_quality/entry_order/lag_over_30/{",
    "book_keeping_quality/gst_setoff",
    "book_keeping_quality/payment_channel/margin",
    "book_keeping_quality/payment_channel/receipt_match",
    "book_keeping_quality/reissue_narration",
    "book_keeping_quality/stock_untouched",
    "books_examined/clause11",
    "cash_44ab/applicability",
    "cash_book_integrity/expense_credits/{",
    "cash_book_integrity/journal_cash/{",
    "cash_book_integrity/negative_cash/{",
    "cash_book_integrity/opening_difference/all",
    "cash_book_integrity/own_account_as_cash/all",
    "cash_payments_40a3/s269ss269t/{",
    "cash_payments_40a3/s269st/{",
    "cash_payments_40a3/s269st_payment/{",
    "cash_payments_40a3/s40a3/{",
    "clause21a_candidates/{",
    "clause44/breakup",
    "clause44/interest_individual_no_gstin_candidate",
    "clause44/judgement_money_item_candidate",
    "counter_cheques_40a3/s40a3/{",
    "creditor_ageing_43bh/clause22_undetermined",
    "creditor_ageing_43bh/opening_dues_26a_candidate",
    "creditor_ageing_43bh/over45/{",
    "creditor_ageing_43bh/post_year_pending",
    "creditor_ageing_43bh/unknown_classification/{",
    "depreciation/book_vs_act",
    "depreciation/s43_1/{",
    "depreciation/unmapped",
    "entity_269st_gap/{",
    "financial_statements/report_tie",
    "high_value_register/{",
    "high_value_register/ais_sft_not_available",
    "high_value_register/event_occasion_question",
    "high_value_register/journal_transfer/{",
    "high_value_register/s194n_filer_status_unknown",
    "high_value_register/s194n_recipient_type_unknown",
    "high_value_register/single_transaction/{",
    "knock_off_candidates/{",
    "ledger_scrutiny/flags/{",
    "loans_interest/clause31/{",
    "loans_interest/deductor_status",
    "loans_interest/lender_not_configured",
    "loans_interest/loan_not_listed/{",
    "loans_interest/loan_pattern_not_listed/{",
    "loans_interest/not_computed/depends_{",
    "loans_interest/not_computed/interest_first_{",
    "loans_interest/not_computed/refund_or_loan_{",
    "loans_interest/not_computed/s194a_{",
    "loans_interest/not_computed/two_sided_{",
    "loans_interest/per_lender/{",
    "loans_interest/possible_misposted_interest/{",
    "loans_interest/s194a/{",
    "loans_interest/s194a_tds_seen/{",
    "loans_interest/shared_interest/{",
    "narration_payees/{",
    "partners_40b_194t/deed_missing",
    "partners_40b_194t/deed_rate_missing",
    "partners_40b_194t/off_capital/{",
    "partners_40b_194t/partners_not_configured",
    "partners_40b_194t/remuneration_book_profit_required",
    "partners_40b_194t/s194t/{",
    "partners_40b_194t/s40b_excess/{",
    "partners_40b_194t/s40b_not_computed/{",
    "party_monthly/{",
    "party_monthly/no_group/{",
    "party_monthly/opening_balance/{",
    "party_monthly/period_not_fy",
    "party_monthly/tb_difference/{",
    "questionnaire_cl13/{",
    "read_scope/currency_not_read",
    "related_parties_cl23/40a2b_reasonableness/{",
    "related_parties_cl23/clause23/{",
    "related_parties_cl23/not_confirmed",
    "specified_persons_40a2b/{",
    "stale_balances_41_1/recoverability/{",
    "stale_balances_41_1/s41_1/{",
    "statutory_dues_43b/coverage/{",
    "statutory_dues_43b/late/{",
    "statutory_dues_43b/not_visible/{",
    "statutory_dues_43b/opening_paid/{",
    "statutory_dues_43b/opening_paid_late/{",
    "statutory_dues_43b/opening_paid_on_time/{",
    "statutory_dues_43b/opening_still_unpaid/{",
    "statutory_dues_43b/opening_unpaid/{",
    "statutory_dues_43b/unpaid/{",
    "stock/clause_35_quantitative_details",
    "stock/negative_at_close",
    "stock/two_stock_figures",
    "stock/typed_in",
    "stock/went_negative_during_year",
    "tds_payees/{",
    "tds_payees/config_unmatched",
    "tds_payees/deductor_status",
    "tds_payees/not_deposited/{",
    "tds_payees/s194c6/{",
    "tds_payees/short_deduction/{",
    "tds_payees/tds_seen/{",
    "tds_tcs_26as/amount_differs",
    "tds_tcs_26as/books_only",
    "tds_tcs_26as/pan_view_scope_limit",
    "tds_tcs_26as/tcs_capitalisation",
    "tds_tcs_26as/twentysixas_only",
    "twentysixas_receipts/{",
];

/// A family as a key names it: the text before `{` for an open family, the whole id for a closed
/// one.
fn family_name(entry: &str) -> &str {
    entry.strip_suffix('{').unwrap_or(entry)
}

/// The family a dump finding id belongs to, or `None` when no family holds it.
pub fn family_of(finding_id: &str) -> Option<&'static str> {
    if let Some(closed) = FAMILIES
        .iter()
        .find(|f| !f.ends_with('{') && **f == finding_id)
    {
        return Some(closed);
    }
    FAMILIES
        .iter()
        .filter(|f| f.ends_with('{') && finding_id.starts_with(family_name(f)))
        .max_by_key(|f| f.len())
        .map(|f| family_name(f))
}

/// The evidence kinds the crate's tests emit (`EvidenceRef::new` and `with_label` across
/// `src/`). A dump's kind outside this list is not scored, so a misspelt kind is refused rather
/// than counted as a miss. A key names only `ledger` and `voucher`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Config,
    DocumentRow,
    ExcludedVoucher,
    Ledger,
    Pan,
    PayeeName,
    Row,
    Rule,
    StockItem,
    Voucher,
}

/// One evidence item, as a reader who saw only the book names it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Ledger {
        name: String,
    },
    Voucher {
        voucher_type: String,
        number: String,
        /// `YYYY-MM-DD`.
        date: String,
    },
}

/// A dump voucher ref the Book cannot turn into an [`Item`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unresolved {
    /// The ref's GUID is empty.
    EmptyGuid,
    /// The Book gives this GUID to vouchers that differ in type, number or date.
    SharedGuid(String),
}

/// Why a key does not open.
#[derive(Debug, PartialEq, Eq)]
pub enum KeyError {
    /// The key's bytes do not hash to the sealed sha256.
    SealMismatch {
        sealed: String,
        actual: String,
    },
    NotUtf8,
    /// Not the key format: a missing or unknown field, an unknown kind, or a value of the wrong
    /// type.
    Parse(String),
    /// A required text is empty or only whitespace. Names are never trimmed otherwise.
    Blank(&'static str),
    NoFamilies,
    /// The prefix is not one of [`FAMILIES`].
    UnknownFamily(String),
    /// The family is another test's.
    FamilyOfAnotherTest(String),
    DuplicateFamily(String),
    /// A kind a key may not name yet: only `ledger` and `voucher` are ruled on.
    KindRefused(Kind),
    /// An item's fields do not fit its kind: a ledger with voucher fields, a voucher missing one,
    /// or a date not written `YYYY-MM-DD`.
    ItemFields(String),
}

/// Why a dump is not scored against a key.
#[derive(Debug, PartialEq, Eq)]
pub enum ScoreError {
    TestIdDiffers {
        key: String,
        dump: String,
    },
    /// The dump is not a canonical dump: a missing field, an unknown kind, a repeated finding, or
    /// a value of the wrong type.
    DumpInvalid(String),
    /// A dump finding belongs to no family.
    FindingInNoFamily(String),
    /// A dump finding's family is one the key does not name, so the key cannot judge its items.
    FamilyNotInKey(String),
    /// A voucher ref's GUID is not in the Book, so the dump and the Book are not of one run.
    VoucherNotInBook(String),
    /// The key expects nothing and the dump lists nothing: refused, never a perfect score.
    Empty,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawKey {
    test_id: String,
    #[serde(default)]
    family: Vec<RawFamily>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFamily {
    prefix: String,
    #[serde(default)]
    item: Vec<RawItem>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawItem {
    kind: Kind,
    name: Option<String>,
    voucher_type: Option<String>,
    number: Option<String>,
    date: Option<String>,
    reason: String,
}

/// A key that opened against its seal. Only [`SealedKey::open`] makes one.
#[derive(Debug)]
pub struct SealedKey {
    test_id: String,
    /// family -> item -> the reason for each time the key names it.
    families: BTreeMap<String, BTreeMap<Item, Vec<String>>>,
}

fn non_blank(text: &str, field: &'static str) -> Result<(), KeyError> {
    if text.trim().is_empty() {
        return Err(KeyError::Blank(field));
    }
    Ok(())
}

fn is_iso_date(text: &str) -> bool {
    let b = text.as_bytes();
    b.len() == 10
        && b.iter().enumerate().all(|(i, c)| {
            if i == 4 || i == 7 {
                *c == b'-'
            } else {
                c.is_ascii_digit()
            }
        })
}

fn key_item(raw: RawItem) -> Result<(Item, String), KeyError> {
    non_blank(&raw.reason, "item.reason")?;
    let fields = |what: &str| KeyError::ItemFields(what.to_string());
    let item = match raw.kind {
        Kind::Ledger => {
            if raw.voucher_type.is_some() || raw.number.is_some() || raw.date.is_some() {
                return Err(fields("a ledger has a name only"));
            }
            let name = raw.name.ok_or_else(|| fields("a ledger needs a name"))?;
            non_blank(&name, "item.name")?;
            Item::Ledger { name: nfc(&name) }
        }
        Kind::Voucher => {
            if raw.name.is_some() {
                return Err(fields("a voucher has no name"));
            }
            let (Some(voucher_type), Some(number), Some(date)) =
                (raw.voucher_type, raw.number, raw.date)
            else {
                return Err(fields("a voucher needs voucher_type, number and date"));
            };
            non_blank(&voucher_type, "item.voucher_type")?;
            if !is_iso_date(&date) {
                return Err(fields("a voucher's date is written YYYY-MM-DD"));
            }
            Item::Voucher {
                voucher_type: nfc(&voucher_type),
                number: nfc(&number),
                date,
            }
        }
        other => return Err(KeyError::KindRefused(other)),
    };
    Ok((item, raw.reason))
}

impl SealedKey {
    /// Open a key from its bytes and the sha256 (lowercase hex) sealed for it.
    pub fn open(bytes: &[u8], sealed_sha256: &str) -> Result<Self, KeyError> {
        let actual = hex(&Sha256::digest(bytes));
        if actual != sealed_sha256 {
            return Err(KeyError::SealMismatch {
                sealed: sealed_sha256.to_string(),
                actual,
            });
        }
        let text = std::str::from_utf8(bytes).map_err(|_| KeyError::NotUtf8)?;
        let raw: RawKey = toml::from_str(text).map_err(|e| KeyError::Parse(e.to_string()))?;
        non_blank(&raw.test_id, "test_id")?;
        if raw.family.is_empty() {
            return Err(KeyError::NoFamilies);
        }
        let mut families = BTreeMap::new();
        for family in raw.family {
            if !FAMILIES.iter().any(|f| family_name(f) == family.prefix) {
                return Err(KeyError::UnknownFamily(family.prefix));
            }
            if !family.prefix.starts_with(&format!("{}/", raw.test_id)) {
                return Err(KeyError::FamilyOfAnotherTest(family.prefix));
            }
            let mut items: BTreeMap<Item, Vec<String>> = BTreeMap::new();
            for raw_item in family.item {
                let (item, reason) = key_item(raw_item)?;
                items.entry(item).or_default().push(reason);
            }
            if families.insert(family.prefix.clone(), items).is_some() {
                return Err(KeyError::DuplicateFamily(family.prefix));
            }
        }
        Ok(Self {
            test_id: raw.test_id,
            families,
        })
    }
}

/// An expected item the dump did not list, with the key's reason for expecting it.
#[derive(Debug, PartialEq, Eq)]
pub struct Miss {
    pub item: Item,
    pub reason: String,
}

/// One family's comparison, each count with repeats, over all that family's findings in the dump.
/// Recall is `matched / expected`, precision `matched / listed`; each is undefined when its
/// denominator is 0. `listed` counts ledger and resolved voucher refs only.
#[derive(Debug, PartialEq, Eq)]
pub struct FamilyScore {
    pub family: String,
    pub expected: usize,
    pub listed: usize,
    pub matched: usize,
    pub missed: Vec<Miss>,
    /// Each listed copy the key does not expect.
    pub extra: Vec<Item>,
    /// Each voucher ref the Book could not resolve.
    pub unresolved: Vec<Unresolved>,
    /// Refs of a kind no key may name yet, by kind.
    pub unscored: BTreeMap<Kind, usize>,
}

/// Every family the key names, in name order.
#[derive(Debug, PartialEq, Eq)]
pub struct Score {
    pub test_id: String,
    pub families: Vec<FamilyScore>,
}

#[derive(Default)]
struct Listed {
    items: BTreeMap<Item, usize>,
    unresolved: Vec<Unresolved>,
    unscored: BTreeMap<Kind, usize>,
}

fn field<'a>(doc: &'a Value, key: &str, at: &str) -> Result<&'a Value, ScoreError> {
    doc.get(key)
        .ok_or_else(|| ScoreError::DumpInvalid(format!("{at}: no {key}")))
}

fn string<'a>(doc: &'a Value, key: &str, at: &str) -> Result<&'a str, ScoreError> {
    field(doc, key, at)?
        .as_str()
        .ok_or_else(|| ScoreError::DumpInvalid(format!("{at}: {key} is not a string")))
}

fn array<'a>(doc: &'a Value, key: &str, at: &str) -> Result<&'a [Value], ScoreError> {
    field(doc, key, at)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| ScoreError::DumpInvalid(format!("{at}: {key} is not a list")))
}

/// A voucher ref's GUID as the Book's voucher, by type, number and date.
fn resolve(book: &Book, guid: &str) -> Result<Result<Item, Unresolved>, ScoreError> {
    if guid.is_empty() {
        return Ok(Err(Unresolved::EmptyGuid));
    }
    let mut seen: Vec<Item> = book
        .vouchers
        .iter()
        .filter(|v| v.guid == guid)
        .map(|v| Item::Voucher {
            voucher_type: nfc(&v.vtype),
            number: nfc(&v.number),
            date: iso(&v.date),
        })
        .collect();
    seen.sort();
    seen.dedup();
    match seen.len() {
        0 => Err(ScoreError::VoucherNotInBook(guid.to_string())),
        1 => Ok(Ok(seen.remove(0))),
        _ => Ok(Err(Unresolved::SharedGuid(guid.to_string()))),
    }
}

/// Each family's listed items from a canonical dump, pooled over its findings.
fn listed_items(dump: &Value, book: &Book) -> Result<BTreeMap<&'static str, Listed>, ScoreError> {
    let mut listed: BTreeMap<&'static str, Listed> = BTreeMap::new();
    let mut seen = std::collections::BTreeSet::new();
    for finding in array(dump, "findings", "dump")? {
        let id = string(finding, "id", "finding")?;
        if !seen.insert(id.to_string()) {
            return Err(ScoreError::DumpInvalid(format!("finding {id} repeats")));
        }
        let family = family_of(id).ok_or_else(|| ScoreError::FindingInNoFamily(id.to_string()))?;
        let into = listed.entry(family).or_default();
        for evidence in array(finding, "evidence", id)? {
            let kind = string(evidence, "kind", id)?;
            let kind = serde_json::from_value::<Kind>(Value::from(kind))
                .map_err(|_| ScoreError::DumpInvalid(format!("{id}: unknown kind {kind}")))?;
            let ref_id = string(evidence, "id", id)?;
            string(evidence, "label", id)?;
            let item = match kind {
                Kind::Ledger => Item::Ledger {
                    name: ref_id.to_string(),
                },
                Kind::Voucher => match resolve(book, ref_id)? {
                    Ok(item) => item,
                    Err(unresolved) => {
                        into.unresolved.push(unresolved);
                        continue;
                    }
                },
                other => {
                    *into.unscored.entry(other).or_default() += 1;
                    continue;
                }
            };
            *into.items.entry(item).or_default() += 1;
        }
    }
    Ok(listed)
}

/// Score one canonical dump (`canonical::canonical_test_result`'s form) against a sealed key,
/// resolving its voucher refs through `book`, the Book the same run read. A family the key names
/// and the dump lacks lists nothing.
pub fn score(key: &SealedKey, dump: &Value, book: &Book) -> Result<Score, ScoreError> {
    let dump_test = string(dump, "test_id", "dump")?;
    if dump_test != key.test_id {
        return Err(ScoreError::TestIdDiffers {
            key: key.test_id.clone(),
            dump: dump_test.to_string(),
        });
    }
    let mut listed = listed_items(dump, book)?;
    if let Some(family) = listed.keys().find(|f| !key.families.contains_key(**f)) {
        return Err(ScoreError::FamilyNotInKey((*family).to_string()));
    }
    let mut families = Vec::new();
    for (family, expected) in &key.families {
        let listed = listed.remove(family.as_str()).unwrap_or_default();
        let mut missed = Vec::new();
        let mut matched = 0;
        for (item, reasons) in expected {
            let found = listed
                .items
                .get(item)
                .copied()
                .unwrap_or(0)
                .min(reasons.len());
            matched += found;
            missed.extend(reasons[found..].iter().map(|reason| Miss {
                item: item.clone(),
                reason: reason.clone(),
            }));
        }
        let mut extra = Vec::new();
        for (item, &count) in &listed.items {
            let wanted = expected.get(item).map_or(0, Vec::len);
            extra.extend(std::iter::repeat_n(
                item.clone(),
                count.saturating_sub(wanted),
            ));
        }
        families.push(FamilyScore {
            family: family.clone(),
            expected: expected.values().map(Vec::len).sum(),
            listed: listed.items.values().sum(),
            matched,
            missed,
            extra,
            unresolved: listed.unresolved,
            unscored: listed.unscored,
        });
    }
    if families.iter().all(|f| f.expected == 0 && f.listed == 0) {
        return Err(ScoreError::Empty);
    }
    Ok(Score {
        test_id: key.test_id.clone(),
        families,
    })
}
