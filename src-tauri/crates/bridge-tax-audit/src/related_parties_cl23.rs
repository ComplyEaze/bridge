//! Related persons and Form 3CD clause 23 (s.40A(2)(b)): what moved on the ledgers the client
//! confirmed against each related person, nature by nature, and for a nature the client says runs a
//! payable balance, the payable read off the Trial Balance. A port of the reference Python
//! implementation's `related_parties_cl23` test module, version 1; its contract is the spec pack in
//! `docs/tax-audit/spec-packs/related_parties_cl23/`.
//!
//! Only the client's `[related_parties]` table names a person, a relationship or a ledger: nothing
//! is inferred from a ledger, group or party name, and whether an amount is reasonable under
//! s.40A(2)(b) is a judgement finding, never assessed here.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::{Book, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::rules::Rules;
use crate::support;
use crate::RelatedPartiesConfig;

pub const TEST_ID: &str = "related_parties_cl23";
pub const VERSION: &str = "1";

/// The fixed nature vocabulary, in the order the test processes it. Any other name, in
/// `ledgers_by_nature` or `payable_natures`, is ignored.
pub const NATURES: [&str; 5] = ["salary", "rent", "interest", "purchases", "other"];

/// The module check's tolerance: Re 1, fired only when exceeded.
const TOLERANCE_PAISE: i64 = 100;

const CLAUSES: [&str; 2] = ["3CD-23", "s.40A(2)(b)"];

/// One related person as the client confirmed them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RelatedPerson {
    /// As written, never normalised; `""` when absent.
    pub relationship: String,
    /// Nature name to its ledger names, as written (bound by `binding` before the test).
    pub ledgers_by_nature: BTreeMap<String, Vec<String>>,
    pub payable_natures: Vec<String>,
}

fn shape_error(key: &str, field: &str, expected: &str) -> AuditError {
    AuditError::refused(
        "RELATED-table-shape",
        format!("{TEST_ID}: related_parties.{key:?}.{field} is present but is not {expected}"),
    )
}

fn texts(value: &toml::Value, key: &str, field: &str) -> Result<Vec<String>> {
    let expected = "a list of text";
    value
        .as_array()
        .ok_or_else(|| shape_error(key, field, expected))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| shape_error(key, field, expected))
        })
        .collect()
}

/// The `[related_parties]` table, typed. A field that is present with the wrong type is refused,
/// naming the person key and the field; an absent field keeps its default, and an empty person
/// is a person. The reference leaves both uncovered (spec pack §2.2); this is a deliberate,
/// stated divergence on input no golden covers.
pub fn related_persons(cfg: &RelatedPartiesConfig) -> Result<BTreeMap<String, RelatedPerson>> {
    let mut persons = BTreeMap::new();
    for (key, value) in &cfg.persons {
        let table = value
            .as_table()
            .ok_or_else(|| shape_error(key, "(value)", "a table"))?;
        let relationship = match table.get("relationship") {
            None => String::new(),
            Some(v) => v
                .as_str()
                .ok_or_else(|| shape_error(key, "relationship", "text"))?
                .to_string(),
        };
        let mut ledgers_by_nature = BTreeMap::new();
        if let Some(v) = table.get("ledgers_by_nature") {
            let natures = v
                .as_table()
                .ok_or_else(|| shape_error(key, "ledgers_by_nature", "a table"))?;
            for (nature, names) in natures {
                let field = format!("ledgers_by_nature.{nature}");
                ledgers_by_nature.insert(nature.clone(), texts(names, key, &field)?);
            }
        }
        let payable_natures = match table.get("payable_natures") {
            None => Vec::new(),
            Some(v) => texts(v, key, "payable_natures")?,
        };
        persons.insert(
            key.clone(),
            RelatedPerson {
                relationship,
                ledgers_by_nature,
                payable_natures,
            },
        );
    }
    Ok(persons)
}

/// Each person's tag: the first 8 hex digits of the SHA-1 of the key's UTF-8 bytes as written
/// (no trimming, case folding or normalisation). Two keys sharing a tag would share every figure
/// id, so the table is refused, naming both keys, rather than one person being dropped or merged.
/// The reference stops on the duplicate figure id instead (spec pack §10); this refuses this test
/// alone, as the crate's other config refusals do.
pub fn person_tags(persons: &BTreeMap<String, RelatedPerson>) -> Result<BTreeMap<String, String>> {
    let mut by_tag: BTreeMap<String, &String> = BTreeMap::new();
    let mut tags = BTreeMap::new();
    for key in persons.keys() {
        let tag = support::hash8(key);
        if let Some(first) = by_tag.insert(tag.clone(), key) {
            return Err(AuditError::refused(
                "RELATED-tag-collision",
                format!(
                    "{TEST_ID}: related persons {first:?} and {key:?} share the tag {tag}; \
rename one key in [related_parties]"
                ),
            ));
        }
        tags.insert(key.clone(), tag);
    }
    Ok(tags)
}

fn ledger_refs(set: &BTreeSet<&str>) -> Vec<EvidenceRef> {
    set.iter().map(|n| EvidenceRef::new("ledger", n)).collect()
}

pub fn run(book: &Book, rules: &Rules, cfg: &RelatedPartiesConfig) -> Result<TestResult> {
    let persons = related_persons(cfg)?;
    let tags = person_tags(&persons)?;
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let overflow = || support::overflow(TEST_ID);

    if persons.is_empty() {
        r.fig(
            "applicable",
            Value::Text("no".into()),
            Unit::Text,
            "Whether any related person has been confirmed by the client for this engagement. \
'no' means none were confirmed -- never inferred from a ledger or party name.",
            Vec::new(),
        )?;
        r.findings.push(Finding {
            id: format!("{TEST_ID}/not_confirmed"),
            clauses: CLAUSES.map(str::to_string).to_vec(),
            title: "No related person has been confirmed for this engagement".into(),
            facts: Vec::new(),
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "Who is a related person under s.40A(2)(b) is a fact the books do not record; \
payments to such persons cannot be listed until the client names them."
                    .into(),
            ],
            ask_client: vec![
                "List the relatives of the proprietor or partners, and any concern in which any \
of them has a substantial interest, with every payment made to them in the year."
                    .into(),
            ],
        });
        return Ok(r);
    }

    r.population_note = "Books population (optional, cancelled and post-dated vouchers excluded). \
One entry = one voucher's net Dr+/Cr- line-sum on a person's nature ledger set."
        .into();
    let population = book.population()?;
    r.fig(
        "applicable",
        Value::Text("yes".into()),
        Unit::Text,
        "Whether any related person has been confirmed by the client for this engagement. \
'no' means none were confirmed -- never inferred from a ledger or party name.",
        Vec::new(),
    )?;
    r.fig(
        "related_person_count",
        support::count(TEST_ID, persons.len())?,
        Unit::Count,
        "Related persons confirmed by the client for this engagement.",
        Vec::new(),
    )?;

    for (key, person) in &persons {
        let tag = &tags[key];
        r.fig(
            &format!("relationship_{tag}"),
            Value::Text(person.relationship.clone()),
            Unit::Text,
            &format!(
                "Relationship confirmed by the client for related person (tag {tag}) -- client \
confirmation only, never inferred from a ledger or party name."
            ),
            Vec::new(),
        )?;
        let mut facts: Vec<(String, String)> = Vec::new();
        let mut total = 0i64;
        let mut any_nonzero = false;
        for nature in NATURES {
            let Some(names) = person.ledgers_by_nature.get(nature) else {
                continue;
            };
            let set: BTreeSet<&str> = names.iter().map(String::as_str).collect();
            if set.is_empty() {
                continue;
            }
            let (amount, entries) = walk(&population, &set)?;
            let mut evidence = ledger_refs(&set);
            evidence.extend(entries.values().map(|(_, v)| {
                EvidenceRef::with_label("voucher", &v.guid, &support::voucher_label(v))
            }));
            let id = r.fig(
                &format!("amount_{nature}_{tag}"),
                Value::Int(amount),
                Unit::Paise,
                &format!(
                    "Net Dr+/Cr- population-line movement on related person (tag {tag})'s \
'{nature}' ledger(s), confirmed by the client."
                ),
                evidence,
            )?;
            facts.push((nature.to_string(), id));
            total = total.checked_add(amount).ok_or_else(overflow)?;
            any_nonzero |= amount != 0;

            if !person.payable_natures.iter().any(|p| p == nature) {
                continue;
            }
            let (mut opening, mut closing, mut debit, mut credit) = (0i64, 0i64, 0i64, 0i64);
            for name in &set {
                // A ledger in the set with no Trial Balance row contributes 0.
                if let Some(row) = book.tb.get(*name) {
                    opening = opening
                        .checked_add(row.opening_paise)
                        .ok_or_else(overflow)?;
                    closing = closing
                        .checked_add(row.closing_paise)
                        .ok_or_else(overflow)?;
                    debit = debit.checked_add(row.debit_paise).ok_or_else(overflow)?;
                    credit = credit.checked_add(row.credit_paise).ok_or_else(overflow)?;
                }
            }
            let payable = [
                (
                    "payable_opening",
                    "payable_opening",
                    opening.checked_neg().ok_or_else(overflow)?,
                    format!(
                        "Opening payable (-TB opening_paise) on person (tag {tag})'s '{nature}' \
ledger(s)."
                    ),
                ),
                (
                    "payable_closing",
                    "payable_closing",
                    closing.checked_neg().ok_or_else(overflow)?,
                    format!(
                        "Closing payable (-TB closing_paise) on person (tag {tag})'s '{nature}' \
ledger(s)."
                    ),
                ),
                (
                    "payable_accrued",
                    "accrued",
                    credit,
                    format!(
                        "TB period CREDIT movement (accrual, P&L side) on person (tag {tag})'s \
'{nature}' ledger(s)."
                    ),
                ),
                (
                    "payable_paid",
                    "paid",
                    debit,
                    format!(
                        "TB period DEBIT movement (amount actually paid) on person (tag {tag})'s \
'{nature}' ledger(s)."
                    ),
                ),
            ];
            for (figure, fact, value, definition) in payable {
                let id = r.fig(
                    &format!("{figure}_{nature}_{tag}"),
                    Value::Int(value),
                    Unit::Paise,
                    &definition,
                    ledger_refs(&set),
                )?;
                facts.push((format!("{nature}_{fact}"), id));
            }
        }
        let total_id = r.fig(
            &format!("amount_total_{tag}"),
            Value::Int(total),
            Unit::Paise,
            &format!(
                "Sum of amount_<nature>_{tag} across every nature configured for person (tag \
{tag})."
            ),
            Vec::new(),
        )?;
        facts.push(("total".into(), total_id));

        // A finding needs a nature that moved; the total does not matter (it may net to 0).
        if !any_nonzero {
            continue;
        }
        r.findings.push(Finding {
            id: format!("{TEST_ID}/clause23/{tag}"),
            clauses: CLAUSES.map(str::to_string).to_vec(),
            title: format!(
                "Transactions with a related person (tag {tag}) confirmed by the client, \
reportable under Clause 23"
            ),
            facts: facts.clone(),
            evidence: Vec::new(),
            confidence: Confidence::Computed,
            limits: vec![
                "Figures are the ledger population's own movement on the ledger(s) confirmed \
against this person; a person transacting through a ledger not listed in the client's own \
confirmation would not appear here."
                    .into(),
            ],
            ask_client: Vec::new(),
        });
        r.findings.push(Finding {
            id: format!("{TEST_ID}/40a2b_reasonableness/{tag}"),
            clauses: vec!["s.40A(2)(b)".into()],
            title: format!(
                "s.40A(2)(b) reasonableness of the amount(s) paid to related person (tag {tag}) \
is not assessed here"
            ),
            facts,
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "s.40A(2)(b) disallows only the excess over fair market value / legitimate \
business need, which needs a comparable arm's-length rate or amount that the books do not carry; \
this test reports the amount actually moved, never a view on whether it is excessive."
                    .into(),
            ],
            ask_client: vec![
                "Confirm the basis (market rate, comparable, or documented business need) for \
the amount paid to this related person in each nature shown above."
                    .into(),
            ],
        });
    }
    Ok(r)
}

/// A ledger set's entries, by voucher GUID: each voucher's net on the set, and the voucher.
type Entries<'a> = BTreeMap<&'a str, (i64, &'a Voucher)>;

/// The population walk of one ledger set (spec pack §4): each voucher's net on the set, in book
/// order; a voucher netting to zero is no entry; a later voucher sharing a GUID replaces the
/// earlier one's entry, unless it nets to zero. The amount is the sum of the entries' nets.
fn walk<'a>(population: &[&'a Voucher], set: &BTreeSet<&str>) -> Result<(i64, Entries<'a>)> {
    let overflow = || support::overflow(TEST_ID);
    let mut entries: Entries = BTreeMap::new();
    for v in population {
        let mut net = 0i64;
        for line in &v.lines {
            if set.contains(line.ledger.as_str()) {
                net = net.checked_add(line.amount_paise).ok_or_else(overflow)?;
            }
        }
        if net != 0 {
            entries.insert(v.guid.as_str(), (net, v));
        }
    }
    let amount = entries
        .values()
        .try_fold(0i64, |acc, (net, _)| acc.checked_add(*net))
        .ok_or_else(overflow)?;
    Ok((amount, entries))
}

/// SUM-1 and XCL-1 (spec pack §6), independent of [`run`]: both re-read the Trial Balance rows
/// of the ledgers each figure cites, and fire only past the 100-paise tolerance.
pub fn check_invariants(book: &Book, result: &TestResult) -> Result<Vec<String>> {
    let overflow = || support::overflow(TEST_ID);
    let mut out = Vec::new();
    let ledgers_of = |id: &str| -> Option<Vec<&str>> {
        result.figures.iter().find(|f| f.id == id).map(|f| {
            f.evidence
                .iter()
                .filter(|e| e.kind == "ledger")
                .map(|e| e.id.as_str())
                .collect()
        })
    };
    let value_of = |id: &str| {
        result
            .figures
            .iter()
            .find(|f| f.id == id)
            .and_then(|f| match f.value {
                Value::Int(v) => Some(v),
                _ => None,
            })
    };
    let tb_sum = |names: &[&str], column: fn(&crate::book::TbRow) -> i64| -> Result<i64> {
        names.iter().try_fold(0i64, |acc, name| {
            let value = book.tb.get(*name).map_or(0, column);
            acc.checked_add(value).ok_or_else(overflow)
        })
    };
    let amount_prefix = format!("{TEST_ID}.amount_");
    let total_prefix = format!("{TEST_ID}.amount_total_");
    let paid_prefix = format!("{TEST_ID}.payable_paid_");
    for figure in &result.figures {
        let Value::Int(value) = figure.value else {
            continue;
        };
        if figure.id.starts_with(&amount_prefix) && !figure.id.starts_with(&total_prefix) {
            let names = ledgers_of(&figure.id).unwrap_or_default();
            if names.is_empty() {
                continue;
            }
            let movement = tb_sum(&names, |row| row.closing_paise)?
                .checked_sub(tb_sum(&names, |row| row.opening_paise)?)
                .ok_or_else(overflow)?;
            let excess = value
                .checked_abs()
                .and_then(|v| v.checked_sub(movement.checked_abs()?))
                .ok_or_else(overflow)?;
            if excess > TOLERANCE_PAISE {
                out.push(format!(
                    "SUM-1: {} population-walk net ({value}p) exceeds its ledger set's own TB \
net movement ({movement}p) in magnitude; the population walk cannot exceed the TB, which \
includes every voucher (excluded or not)",
                    figure.id
                ));
            }
        }
        let Some(rest) = figure.id.strip_prefix(&paid_prefix) else {
            continue;
        };
        let paid = value;
        let accrued_id = format!("{TEST_ID}.payable_accrued_{rest}");
        let Some(accrued) = value_of(&accrued_id) else {
            out.push(format!("XCL-1: {} has no matching {accrued_id}", figure.id));
            continue;
        };
        let names = ledgers_of(&figure.id).unwrap_or_default();
        if names.is_empty() {
            out.push(format!(
                "XCL-1: {} carries no ledger evidence to verify against the Trial Balance",
                figure.id
            ));
            continue;
        }
        let opening = tb_sum(&names, |row| row.opening_paise)?
            .checked_neg()
            .ok_or_else(overflow)?;
        let closing = tb_sum(&names, |row| row.closing_paise)?
            .checked_neg()
            .ok_or_else(overflow)?;
        for (side, recomputed) in [("opening", opening), ("closing", closing)] {
            let id = format!("{TEST_ID}.payable_{side}_{rest}");
            if let Some(published) = value_of(&id) {
                if published != recomputed {
                    out.push(format!(
                        "XCL-1: {id} = {published}p but the TB itself gives {side} payable \
{recomputed}p for the same ledger set"
                    ));
                }
            }
        }
        let expected = opening
            .checked_add(accrued)
            .and_then(|v| v.checked_sub(closing))
            .ok_or_else(overflow)?;
        let gap = paid
            .checked_sub(expected)
            .and_then(i64::checked_abs)
            .ok_or_else(overflow)?;
        if gap > TOLERANCE_PAISE {
            out.push(format!(
                "XCL-1: {} = {paid}p but opening payable ({opening}p) + accrued ({accrued}p) - \
closing payable ({closing}p) = {expected}p -- the identity paid = opening payable + accrued - \
closing payable does not hold (check for a transposed accrued/paid figure)",
                figure.id
            ));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A refusal's code and whole detail, so a refusal that loses the keys or the field it
    /// names fails the test.
    fn refusal(err: AuditError) -> (&'static str, String) {
        match err {
            AuditError::Refused { code, detail } => (code, detail),
            other => panic!("not a refusal: {other}"),
        }
    }

    fn table(text: &str) -> RelatedPartiesConfig {
        let persons: toml::Table = text.parse().unwrap();
        RelatedPartiesConfig {
            persons: persons.into_iter().collect(),
        }
    }

    #[test]
    fn two_keys_sharing_a_tag_are_refused_and_case_or_form_variants_are_not() {
        // `Person PXD` and `Person ACOW` share the tag 773442d1 (spec pack §10).
        assert_eq!(support::hash8("Person PXD"), "773442d1");
        assert_eq!(support::hash8("Person ACOW"), "773442d1");
        let persons =
            related_persons(&table("\"Person PXD\" = {}\n\"Person ACOW\" = {}\n")).unwrap();
        assert_eq!(
            refusal(person_tags(&persons).unwrap_err()),
            (
                "RELATED-tag-collision",
                "related_parties_cl23: related persons \"Person ACOW\" and \"Person PXD\" share \
the tag 773442d1; rename one key in [related_parties]"
                    .to_string()
            )
        );
        // The tag is of the key as written: case and composed/decomposed forms are persons apart.
        let persons = related_persons(&table(
            "\"person a\" = {}\n\"Person A\" = {}\n\"Ren\u{e9}\" = {}\n\"Rene\u{301}\" = {}\n",
        ))
        .unwrap();
        assert_eq!(person_tags(&persons).unwrap().len(), 4);
    }

    #[test]
    fn a_present_field_of_the_wrong_shape_is_refused_naming_the_person() {
        for (text, field, expected) in [
            ("\"Person A\" = \"brother\"\n", "(value)", "a table"),
            ("[\"Person A\"]\nrelationship = 5\n", "relationship", "text"),
            (
                "[\"Person A\"]\nledgers_by_nature = [\"Rent\"]\n",
                "ledgers_by_nature",
                "a table",
            ),
            (
                "[\"Person A\"]\nledgers_by_nature = { rent = \"Rent\" }\n",
                "ledgers_by_nature.rent",
                "a list of text",
            ),
            (
                "[\"Person A\"]\nledgers_by_nature = { rent = [\"Rent\", 5] }\n",
                "ledgers_by_nature.rent",
                "a list of text",
            ),
            (
                "[\"Person A\"]\nledgers_by_nature = { commission = [true] }\n",
                "ledgers_by_nature.commission",
                "a list of text",
            ),
            (
                "[\"Person A\"]\npayable_natures = \"rent\"\n",
                "payable_natures",
                "a list of text",
            ),
            (
                "[\"Person A\"]\npayable_natures = [1]\n",
                "payable_natures",
                "a list of text",
            ),
        ] {
            assert_eq!(
                refusal(related_persons(&table(text)).unwrap_err()),
                (
                    "RELATED-table-shape",
                    format!(
                        "related_parties_cl23: related_parties.\"Person A\".{field} is present but \
is not {expected}"
                    )
                ),
                "{text}"
            );
        }
    }

    #[test]
    fn an_absent_field_keeps_its_default_and_an_empty_person_is_a_person() {
        let persons = related_persons(&table("\"Person A\" = {}\n")).unwrap();
        assert_eq!(
            persons,
            BTreeMap::from([("Person A".to_string(), RelatedPerson::default())])
        );
        let persons = related_persons(&table(
            "[\"Person B\"]\nrelationship = \"sister\"\nledgers_by_nature = { rent = [\"Rent\"] }\n",
        ))
        .unwrap();
        assert_eq!(
            persons["Person B"],
            RelatedPerson {
                relationship: "sister".into(),
                ledgers_by_nature: BTreeMap::from([("rent".to_string(), vec!["Rent".to_string()])]),
                payable_natures: Vec::new(),
            }
        );
    }

    /// XCL-1's three messages the test's own output cannot reach (spec pack §6), on a hand-built
    /// result: a paid figure with no accrued figure, a paid figure citing no ledger, and a
    /// published opening and closing payable the TB does not give.
    #[test]
    fn xcl_1_reports_what_the_test_itself_never_publishes() {
        let mut book = Book::default();
        book.tb.insert(
            "Salary Payable".into(),
            crate::book::TbRow {
                opening_paise: -3_000_000,
                debit_paise: 11_000_000,
                credit_paise: 12_000_000,
                closing_paise: -4_000_000,
            },
        );
        let rules = Rules::vendored().unwrap();
        let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
        let ledger = || vec![EvidenceRef::new("ledger", "Salary Payable")];
        r.fig(
            "payable_paid_rent_aaaaaaaa",
            Value::Int(10),
            Unit::Paise,
            "d",
            ledger(),
        )
        .unwrap();
        r.fig(
            "payable_paid_other_bbbbbbbb",
            Value::Int(10),
            Unit::Paise,
            "d",
            Vec::new(),
        )
        .unwrap();
        r.fig(
            "payable_accrued_other_bbbbbbbb",
            Value::Int(10),
            Unit::Paise,
            "d",
            Vec::new(),
        )
        .unwrap();
        for (figure, value) in [
            ("payable_opening_salary_cccccccc", 1),
            ("payable_closing_salary_cccccccc", 2),
            ("payable_accrued_salary_cccccccc", 12_000_000),
            ("payable_paid_salary_cccccccc", 11_000_000),
        ] {
            r.fig(figure, Value::Int(value), Unit::Paise, "d", ledger())
                .unwrap();
        }
        assert_eq!(
            check_invariants(&book, &r).unwrap(),
            [
                "XCL-1: related_parties_cl23.payable_paid_rent_aaaaaaaa has no matching \
related_parties_cl23.payable_accrued_rent_aaaaaaaa",
                "XCL-1: related_parties_cl23.payable_paid_other_bbbbbbbb carries no ledger evidence \
to verify against the Trial Balance",
                "XCL-1: related_parties_cl23.payable_opening_salary_cccccccc = 1p but the TB itself \
gives opening payable 3000000p for the same ledger set",
                "XCL-1: related_parties_cl23.payable_closing_salary_cccccccc = 2p but the TB itself \
gives closing payable 4000000p for the same ledger set",
            ]
        );
    }

    /// SUM-1's movement is checked arithmetic: a closing and opening whose difference leaves i64
    /// refuse as every other money sum here does, instead of wrapping or panicking.
    #[test]
    fn sum_1_refuses_a_movement_that_overflows() {
        let mut book = Book::default();
        book.tb.insert(
            "Rent".into(),
            crate::book::TbRow {
                opening_paise: -1,
                debit_paise: 0,
                credit_paise: 0,
                closing_paise: i64::MAX,
            },
        );
        let rules = Rules::vendored().unwrap();
        let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
        let rent = vec![EvidenceRef::new("ledger", "Rent")];
        r.fig(
            "amount_rent_aaaaaaaa",
            Value::Int(5),
            Unit::Paise,
            "d",
            rent,
        )
        .unwrap();
        let err = check_invariants(&book, &r).unwrap_err();
        assert!(
            matches!(&err, AuditError::Config(m) if m == "related_parties_cl23: a total overflowed i64 paise"),
            "{err}"
        );
    }
}
