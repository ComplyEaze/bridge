//! Specified persons under s.40A(2)(b) (Form 3CD clause 23 family): for every amount the
//! related-party test found moving to a confirmed related person, the outside evidence the auditor
//! needs before weighing it, or, for a recorded partner's salary and interest under rules that apply
//! s.40(b), that s.40(b) governs instead. A port of the reference Python implementation's
//! `specified_persons_40a2b` test module, version 1; its contract is the spec pack in
//! `docs/tax-audit/spec-packs/specified_persons_40a2b/`.
//!
//! It never reads the book: every amount and every piece of evidence is a `related_parties_cl23`
//! figure's, and every finding is a question for the auditor or the client.

use std::collections::BTreeMap;

use crate::error::{AuditError, Result};
use crate::findings::{Confidence, Figure, Finding, TestResult, Unit, Value};
use crate::related_parties_cl23::{self, RelatedPerson, NATURES};
use crate::rules::Rules;
use crate::support;
use crate::RelatedPartiesConfig;

pub const TEST_ID: &str = "specified_persons_40a2b";
pub const VERSION: &str = "1";

const PARTNER_LABELS: [&str; 3] = ["partner", "partner in the firm", "working partner"];
const S40B_NATURES: [&str; 2] = ["interest", "salary"];

/// What a nonzero (person, nature) amount is reported as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Treatment {
    /// s.40(b) governs: the rule is open, the person is a recorded partner, the nature is salary
    /// or interest.
    S40bGoverns,
    ComparableRequired,
}

impl Treatment {
    fn kind(self) -> &'static str {
        match self {
            Self::S40bGoverns => "s40b_governs",
            Self::ComparableRequired => "comparable_required",
        }
    }
}

/// The outside figure an amount of each nature is weighed against (spec pack §2.6).
fn comparable(nature: &str) -> &'static str {
    match nature {
        "salary" => "the prevailing salary for a comparable role, qualifications and hours",
        "rent" => "the market rate of rent for comparable property in the same location",
        "interest" => "the rate an unrelated lender charges for a comparable loan",
        "purchases" => {
            "the arm's-length price for comparable goods or services from an unrelated supplier"
        }
        _ => "a comparable arm's-length rate or amount for this nature of transaction",
    }
}

/// Whether the engagement's entity rules apply s.40(b) (spec pack §2.4). Rules with no `[entity]`
/// table refuse, so it is called only where the reference looks the rate up (§8).
fn s40b_open(rules: &Rules, entity_type: &str) -> Result<bool> {
    Ok(rules.s40b_interest_rate_bp(entity_type)? > 0)
}

/// The rule of spec pack §3.3, given whether the entity rules apply s.40(b).
fn treatment(open: bool, person: &RelatedPerson, nature: &str) -> Treatment {
    let partner = PARTNER_LABELS
        .contains(&support::py_lower(support::py_strip(&person.relationship)).as_str());
    if open && partner && S40B_NATURES.contains(&nature) {
        Treatment::S40bGoverns
    } else {
        Treatment::ComparableRequired
    }
}

/// One (person, nature) the table gives a non-empty vocabulary list, with the related-party
/// amount figure for it, if the result has one, in key and vocabulary order.
struct Pair<'a> {
    key: &'a str,
    person: &'a RelatedPerson,
    tag: &'a str,
    nature: &'static str,
    figure_id: String,
    figure: Option<(&'a Figure, i64)>,
}

fn pairs<'a>(
    persons: &'a BTreeMap<String, RelatedPerson>,
    tags: &'a BTreeMap<String, String>,
    related: &'a TestResult,
) -> Vec<Pair<'a>> {
    let mut out = Vec::new();
    for (key, person) in persons {
        let tag = tags[key].as_str();
        for nature in NATURES {
            if person
                .ledgers_by_nature
                .get(nature)
                .is_none_or(Vec::is_empty)
            {
                continue;
            }
            let figure_id = format!("{}.amount_{nature}_{tag}", related_parties_cl23::TEST_ID);
            let figure = related
                .figures
                .iter()
                .find(|f| f.id == figure_id)
                .and_then(|f| match f.value {
                    Value::Int(amount) => Some((f, amount)),
                    _ => None,
                });
            out.push(Pair {
                key,
                person,
                tag,
                nature,
                figure_id,
                figure,
            });
        }
    }
    out
}

fn finding_id(treatment: Treatment, tag: &str, nature: &str) -> String {
    format!("{TEST_ID}/{}/{tag}/{nature}", treatment.kind())
}

/// `related` is `related_parties_cl23`'s result on the same book with the same table.
pub fn run(
    rules: &Rules,
    entity_type: &str,
    cfg: &RelatedPartiesConfig,
    related: &TestResult,
) -> Result<TestResult> {
    let persons = related_parties_cl23::related_persons(cfg)?;
    let tags = related_parties_cl23::person_tags(&persons)?;
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.fig(
        "applicable",
        Value::Text(if persons.is_empty() { "no" } else { "yes" }.into()),
        Unit::Text,
        "Whether any related person has been confirmed by the client for this engagement (the \
SAME [related_parties] config related_parties_cl23 reads). 'no' means none were confirmed and this \
test reports nothing further.",
        Vec::new(),
    )?;
    if persons.is_empty() {
        return Ok(r);
    }
    // Once for a confirmed table, before any person's natures are read (spec pack §8), so rules
    // with no `[entity]` table refuse whatever the amounts.
    let open = s40b_open(rules, entity_type)?;
    for pair in pairs(&persons, &tags, related) {
        let Some((figure, amount)) = pair.figure else {
            return Err(AuditError::refused(
                "SPECIFIED-missing-figure",
                format!(
                    "{TEST_ID}: the related_parties_cl23 result has no amount figure {} for \
related person {:?}; run both tests on the same [related_parties] table",
                    pair.figure_id, pair.key
                ),
            ));
        };
        if amount == 0 {
            continue;
        }
        let (tag, nature) = (pair.tag, pair.nature);
        let treatment = treatment(open, pair.person, nature);
        let (clauses, title, limits, ask_client) = match treatment {
            Treatment::ComparableRequired => (
                ["3CD-23", "s.40A(2)(b)"],
                format!(
                    "A comparable is needed to test the '{nature}' amount paid to related person \
(tag {tag}) under s.40A(2)(b)"
                ),
                "s.40A(2)(b) disallows only the part of this amount that lies above a market \
comparable; the books show only the amount that moved, never a market rate or price, so this test \
takes no view on how any part of it compares to the comparable."
                    .to_string(),
                vec![
                    format!(
                        "Obtain {} and record it as the basis for this amount.",
                        comparable(nature)
                    ),
                    "Confirm the business need for the transaction.".to_string(),
                ],
            ),
            Treatment::S40bGoverns => (
                ["s.40A(2)(b)", "s.40(b)"],
                format!(
                    "s.40A(2)(b) does not apply to this amount for related person (tag {tag}) -- \
s.40(b) governs instead (confirm)"
                ),
                format!(
                    "Related person (tag {tag}) is recorded with relationship {}, a client \
confirmation of partner status, and this engagement's rules apply s.40(b) to a partner's interest \
and remuneration; on that basis s.40A(2)(b) does not additionally apply to the same amount. This \
rests on the client's own confirmation, not a reading of the deed, so it is stated here for \
confirmation, not as a settled fact.",
                    as_written(&pair.person.relationship)
                ),
                vec![
                    "Confirm the partnership deed treats this amount as partner interest or \
remuneration under s.40(b), not a separate related-party payment under s.40A(2)(b)."
                        .to_string(),
                ],
            ),
        };
        r.findings.push(Finding {
            id: finding_id(treatment, tag, nature),
            clauses: clauses.map(str::to_string).to_vec(),
            title,
            facts: vec![("amount".into(), figure.id.clone())],
            evidence: figure.evidence.clone(),
            confidence: Confidence::JudgementRequired,
            limits: vec![limits],
            ask_client,
        });
    }
    Ok(r)
}

/// The relationship as the limits text shows it (spec pack §3.4): split on Python's whitespace,
/// joined with single spaces and single-quoted, with no escaping.
fn as_written(relationship: &str) -> String {
    format!("'{}'", support::py_split(relationship).join(" "))
}

/// SPD-1 (spec pack §4), independent of [`run`]: each nonzero pair has exactly the one finding
/// the rule calls for. A pair with no amount figure, or a zero one, is skipped.
pub fn check_invariants(
    rules: &Rules,
    entity_type: &str,
    cfg: &RelatedPartiesConfig,
    related: &TestResult,
    result: &TestResult,
) -> Result<Vec<String>> {
    let persons = related_parties_cl23::related_persons(cfg)?;
    let tags = related_parties_cl23::person_tags(&persons)?;
    let mut out = Vec::new();
    for pair in pairs(&persons, &tags, related) {
        if pair.figure.is_none_or(|(_, amount)| amount == 0) {
            continue;
        }
        // SPD-1 looks the rate up for each pair it checks, as the reference does.
        let governs = treatment(s40b_open(rules, entity_type)?, pair.person, pair.nature)
            == Treatment::S40bGoverns;
        let governed_id = finding_id(Treatment::S40bGoverns, pair.tag, pair.nature);
        let comparable_id = finding_id(Treatment::ComparableRequired, pair.tag, pair.nature);
        let has = |id: &str| result.findings.iter().any(|f| f.id == id);
        let (key, nature) = (
            support::py_repr_str(pair.key),
            support::py_repr_str(pair.nature),
        );
        let (governed, comparable) = (has(&governed_id), has(&comparable_id));
        let detail = if governed && comparable {
            format!(
                "SPD-1: {key} nature {nature} has BOTH {governed_id} and {comparable_id} -- \
exactly one must be emitted"
            )
        } else if governs && !governed {
            let emitted = if comparable {
                format!("a comparable-required finding {comparable_id} was emitted instead")
            } else {
                format!("{governed_id} is missing")
            };
            format!(
                "SPD-1: {key} nature {nature} should be governed by s.40(b) (relationship {}, \
entity rules gate open) but {emitted}",
                support::py_repr_str(&pair.person.relationship)
            )
        } else if !governs && !comparable {
            let emitted = if governed {
                format!("a s.40(b)-governs finding {governed_id} was emitted instead")
            } else {
                format!("{comparable_id} is missing")
            };
            format!("SPD-1: {key} nature {nature} should ask for a comparable but {emitted}")
        } else {
            continue;
        };
        out.push(detail);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::book::Book;
    use crate::Engagement;

    fn table(text: &str) -> RelatedPartiesConfig {
        let persons: toml::Table = text.parse().unwrap();
        RelatedPartiesConfig {
            persons: persons.into_iter().collect(),
        }
    }

    /// A refusal as its whole value, code and detail.
    fn refusal(err: AuditError) -> (&'static str, String) {
        match err {
            AuditError::Refused { code, detail } => (code, detail),
            other => panic!("not a refusal: {other:?}"),
        }
    }

    /// A related-party result holding only `amount_rent_<tag>` = `amount` for `key`.
    fn rent_amount(rules: &Rules, key: &str, amount: i64) -> TestResult {
        let mut r = TestResult::new(related_parties_cl23::TEST_ID, "1", &rules.version);
        let name = format!("amount_rent_{}", support::hash8(key));
        r.fig(&name, Value::Int(amount), Unit::Paise, "", Vec::new())
            .unwrap();
        r
    }

    #[test]
    fn a_missing_related_party_figure_is_refused_naming_it_and_the_check_skips_it() {
        let rules = Rules::vendored().unwrap();
        let cfg = table("[\"Person A\"]\nledgers_by_nature = { rent = [\"Rent\"] }\n");
        let empty = TestResult::new(related_parties_cl23::TEST_ID, "1", &rules.version);
        let err = run(&rules, "individual", &cfg, &empty).unwrap_err();
        let tag = support::hash8("Person A");
        assert_eq!(
            refusal(err),
            (
                "SPECIFIED-missing-figure",
                format!(
                    "specified_persons_40a2b: the related_parties_cl23 result has no amount \
figure related_parties_cl23.amount_rent_{tag} for related person \"Person A\"; run both tests on \
the same [related_parties] table"
                )
            )
        );
        let own = TestResult::new(TEST_ID, VERSION, &rules.version);
        let check = check_invariants(&rules, "individual", &cfg, &empty, &own).unwrap();
        assert_eq!(check, Vec::<String>::new());
    }

    #[test]
    fn a_related_party_refusal_stops_this_test_on_the_runner_path() {
        let text = "[client]\nlabel = \"Test\"\nassessment_year = \"2026-27\"\n\
entity_type = \"firm\"\n[period]\nstart = \"2025-04-01\"\nend = \"2026-03-31\"\n[snapshot]\n\
format = \"tally-read-v1\"\npath = \"unused\"\n[roles]\ncash_groups = []\n\
bank_groups = []\n[related_parties]\n\"Person A\" = \"brother\"\n";
        let e = Engagement::from_toml(text, Path::new(".")).unwrap();
        let (book, rules) = (Book::default(), Rules::vendored().unwrap());
        let want = (
            "RELATED-table-shape",
            "related_parties_cl23: related_parties.\"Person A\".(value) is present but is not a \
table"
                .to_string(),
        );
        let related = crate::related_parties_cl23_on(&e, &book, &rules).unwrap_err();
        assert_eq!(refusal(related), want);
        let this = crate::specified_persons_40a2b_on(&e, &book, &rules).unwrap_err();
        assert_eq!(refusal(this), want);
    }

    #[test]
    fn rules_with_no_entity_table_refuse_any_confirmed_table_and_never_an_empty_one() {
        // The rate is looked up once for a confirmed table, before any nature is read, so such
        // rules refuse whatever the amounts; an empty table never looks (spec pack §8). SPD-1
        // looks it up only for a pair with a nonzero amount.
        let mut rules = Rules::vendored().unwrap();
        rules.entity = None;
        let no_entity =
            |err| matches!(err, AuditError::Config(m) if m == "rules: no [entity] table");
        let empty = RelatedPartiesConfig::default();
        let nothing = TestResult::new(related_parties_cl23::TEST_ID, "1", &rules.version);
        let result = run(&rules, "firm", &empty, &nothing).unwrap();
        assert!(result.findings.is_empty());
        let check = check_invariants(&rules, "firm", &empty, &nothing, &result).unwrap();
        assert_eq!(check, Vec::<String>::new());

        let cfg = table("[\"Person A\"]\nledgers_by_nature = { rent = [\"Rent\"] }\n");
        let zero = rent_amount(&rules, "Person A", 0);
        assert!(no_entity(run(&rules, "firm", &cfg, &zero).unwrap_err()));
        let check = check_invariants(&rules, "firm", &cfg, &zero, &result).unwrap();
        assert_eq!(check, Vec::<String>::new());
        let moved = rent_amount(&rules, "Person A", 1);
        let check = check_invariants(&rules, "firm", &cfg, &moved, &result).unwrap_err();
        assert!(no_entity(check));
    }

    #[test]
    fn spd_1_reports_each_message_whole() {
        let rules = Rules::vendored().unwrap();
        let cfg = table(
            "[\"Person A\"]\nrelationship = \" Partner\"\nledgers_by_nature = { rent = [\"Rent\"], \
salary = [\"Pay\"] }\n",
        );
        let tag = support::hash8("Person A");
        let mut related = rent_amount(&rules, "Person A", 5);
        let salary = format!("amount_salary_{tag}");
        related
            .fig(&salary, Value::Int(-7), Unit::Paise, "", Vec::new())
            .unwrap();
        let result = run(&rules, "firm", &cfg, &related).unwrap();
        let ids: Vec<&str> = result.findings.iter().map(|f| f.id.as_str()).collect();
        let (comparable, governed) = (
            format!("{TEST_ID}/comparable_required/{tag}/rent"),
            format!("{TEST_ID}/s40b_governs/{tag}/salary"),
        );
        // Vocabulary order: salary before rent.
        assert_eq!(ids, [governed.as_str(), comparable.as_str()]);
        let check = |r: &TestResult| check_invariants(&rules, "firm", &cfg, &related, r).unwrap();
        assert_eq!(check(&result), Vec::<String>::new());

        // Each finding dropped.
        let mut tampered = result.clone();
        tampered.findings.clear();
        assert_eq!(
            check(&tampered),
            [
                format!(
                    "SPD-1: 'Person A' nature 'salary' should be governed by s.40(b) \
(relationship ' Partner', entity rules gate open) but {governed} is missing"
                ),
                format!(
                    "SPD-1: 'Person A' nature 'rent' should ask for a comparable but {comparable} \
is missing"
                ),
            ]
        );
        // Each pair given both kinds.
        let mut tampered = result.clone();
        for f in &result.findings {
            let mut twin = f.clone();
            twin.id = if f.id == comparable {
                format!("{TEST_ID}/s40b_governs/{tag}/rent")
            } else {
                format!("{TEST_ID}/comparable_required/{tag}/salary")
            };
            tampered.findings.push(twin);
        }
        assert_eq!(
            check(&tampered),
            [
                format!(
                    "SPD-1: 'Person A' nature 'salary' has BOTH {governed} and \
{TEST_ID}/comparable_required/{tag}/salary -- exactly one must be emitted"
                ),
                format!(
                    "SPD-1: 'Person A' nature 'rent' has BOTH {TEST_ID}/s40b_governs/{tag}/rent \
and {comparable} -- exactly one must be emitted"
                ),
            ]
        );
        // Each pair given the other kind instead.
        let mut tampered = result.clone();
        for f in &mut tampered.findings {
            f.id = if f.id == comparable {
                format!("{TEST_ID}/s40b_governs/{tag}/rent")
            } else {
                format!("{TEST_ID}/comparable_required/{tag}/salary")
            };
        }
        assert_eq!(
            check(&tampered),
            [
                format!(
                    "SPD-1: 'Person A' nature 'salary' should be governed by s.40(b) \
(relationship ' Partner', entity rules gate open) but a comparable-required finding \
{TEST_ID}/comparable_required/{tag}/salary was emitted instead"
                ),
                format!(
                    "SPD-1: 'Person A' nature 'rent' should ask for a comparable but a \
s.40(b)-governs finding {TEST_ID}/s40b_governs/{tag}/rent was emitted instead"
                ),
            ]
        );
    }
}
