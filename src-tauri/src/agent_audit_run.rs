//! `audit_test_run`: one admitted tax-audit test on a company's own books.
//!
//! The books are the audit read of [`super::audit_read`] (one consistent,
//! identity-bracketed read, admitted by the tax-audit crate), and the test is
//! the crate's own, run unchanged. What leaves is a projection of the crate's
//! canonical result: figure ids with their values, finding ids with their
//! confidence and clauses, the test's own fixed wording, and counts and codes
//! of the book checks that failed. The crate's evidence references, labels and
//! the subject and detail of a failed check name ledgers, so none of them is
//! returned, and a redaction setting is refused rather than trusted to hide
//! them (it masks only marked values).
use super::audit_read::{read_audit_book, AuditReadFailure, AuditRoles};
use super::voucher_window::{VoucherReadShape, WindowReadLimits};
use super::*;
use bridge_tax_audit::registry::{self, CallerData};

/// The tests this tool runs. A test joins the list only with a real-book
/// parity record at a pinned reference commit, and with a leak test of its own
/// like the one for `cash_44ab` in this module's tests: `project` copies a
/// test's values, clauses and fixed wording whole, so a test whose ids or text
/// are built from the book would leak through it. `cash_44ab`: a real-book
/// parity check passed on 21 Oct 2026 at reference pin 6168aeaa (crate tree
/// 53938eb8), on one book with no cash payments: that side was checked at zero.
pub(super) const ADMITTED: &[&str] = &["cash_44ab"];

mod admitted {
    use super::ADMITTED;

    /// A test in [`ADMITTED`]: the only kind of name `run_test` takes. Its field
    /// is private to this module, so outside it only [`AdmittedTest::parse`]
    /// makes one, and no other ported test can be run through the tool.
    #[derive(Clone, Copy)]
    pub(super) struct AdmittedTest(&'static str);

    impl AdmittedTest {
        pub(super) fn parse(name: &str) -> Option<Self> {
            ADMITTED
                .iter()
                .copied()
                .find(|admitted| *admitted == name)
                .map(Self)
        }

        pub(super) fn name(self) -> &'static str {
            self.0
        }
    }
}
use admitted::AdmittedTest;

/// Refused rather than cut short: a result past these is not one a caller
/// can read whole.
const MAX_FIGURES: usize = 64;
const MAX_FINDINGS: usize = 32;
const MAX_GROUPS: usize = 20;
const MAX_GROUP_CHARS: usize = 128;

const REDACTION_UNSUPPORTED: &str = "audit_redaction_unsupported";
const RESULT_TOO_LARGE: &str = "audit_result_too_large";
const RESULT_NOT_PROJECTABLE: &str = "audit_result_not_projectable";
const AUDIT_TEST_REFUSED: &str = "audit_test_refused";
const PERIOD_OUTSIDE_RULES: &str = "audit_period_outside_ay_2026_27";
/// A group named as both cash and bank would count its ledgers twice over.
const GROUPS_OVERLAP: &str = "audit_groups_overlap";
/// The result's state: completed with every book check holding, or completed
/// with at least one failed. A failed check can mean a figure leaves something
/// out (a ledger with no master or no group chain, MAP-0 and MAP-1, counts as
/// neither cash nor bank), so the state says so rather than `completed` alone.
const STATE_COMPLETED: &str = "completed";
const STATE_CHECKS_FAILED: &str = "completed_with_failed_book_checks";

/// The financial year the vendored rules are for (assessment year 2026-27):
/// the engine refuses any other year's rules, and a period outside this one
/// would silently be judged by this year's thresholds.
const RULES_YEAR_FROM: &str = "20250401";
const RULES_YEAR_TO: &str = "20260331";

impl Server {
    pub(super) async fn audit_test_run(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        if self.settings.redaction != Redaction::None {
            return Err(REDACTION_UNSUPPORTED.to_string().into());
        }
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        if from > to {
            return Err("invalid_date_range".to_string().into());
        }
        if from.as_str() < RULES_YEAR_FROM || to.as_str() > RULES_YEAR_TO {
            return Err(PERIOD_OUTSIDE_RULES.to_string().into());
        }
        // The tool's schema lists `ADMITTED` as the only values of `test`, so
        // the catalog has refused any other as `argument_invalid:test` before
        // this runs; the parse makes that a type rather than an assumption.
        let test = AdmittedTest::parse(required_string(args, "test")?)
            .ok_or_else(|| "argument_invalid:test".to_string())?;
        let roles = AuditRoles {
            cash_groups: group_list(args, "cash_groups")?,
            bank_groups: group_list(args, "bank_groups")?,
        };
        if roles.cash_groups.iter().any(|cash| {
            roles
                .bank_groups
                .iter()
                .any(|bank| bank.trim().eq_ignore_ascii_case(cash.trim()))
        }) {
            return Err(GROUPS_OVERLAP.to_string().into());
        }
        let (company, identity, prior) = self.verified_company(guid).await?;
        // A lab build with lab writes on reads only the lab's target company,
        // as every lab tool does, so a book is kept only when the run names it.
        #[cfg(feature = "lab-writes")]
        let grant = if lab::env_lab_writes_enabled() {
            Some(
                lab::admit_audit_read_capture(self, identity.company_guid())
                    .map_err(|failure| failure.with_prior_evidence(prior.clone()))?,
            )
        } else {
            None
        };
        let config = self.tally_config();
        let audit = match read_audit_book(
            &self.runtime,
            &config,
            &identity,
            (&from, &to),
            &roles,
            WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard),
        )
        .await
        {
            Ok(audit) => audit,
            Err(failure) => return Err(ToolFailure::from(failure.code).with_prior_evidence(prior)),
        };
        // The read the guards admitted above is kept on this computer, for a
        // real-book parity record; on success the answer is unchanged.
        #[cfg(feature = "lab-writes")]
        if let Some(grant) = grant {
            lab::persist_audit_read(self, grant, &audit.stored).map_err(|code| {
                ToolFailure::from(code)
                    .with_prior_evidence(combine_evidence(prior.clone(), audit.evidence.clone()))
            })?;
        }
        let evidence = combine_evidence(prior, audit.evidence);
        let dump = run_test(test, &audit.engagement, &audit.book).map_err(|failure| {
            ToolFailure::from(failure.code).with_prior_evidence(evidence.clone())
        })?;
        let result = project(&dump)
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

/// The test on the book, as the crate's canonical dump.
fn run_test(
    test: AdmittedTest,
    engagement: &bridge_tax_audit::Engagement,
    book: &bridge_tax_audit::book::Book,
) -> Result<Value, AuditReadFailure> {
    let refused = || AuditReadFailure {
        code: AUDIT_TEST_REFUSED.to_string(),
        retryable: false,
    };
    let ported = registry::find(test.name()).ok_or_else(refused)?;
    let rules = bridge_tax_audit::rules_for(engagement).map_err(|_| refused())?;
    // A refusal by the test carries the crate's detail, which can name a
    // ledger: only its rule code, when it has one, is kept.
    (ported.run_on)(engagement, book, &rules, &CallerData::default()).map_err(|error| match error {
        bridge_tax_audit::AuditError::Refused { code, .. } => AuditReadFailure {
            code: code.to_string(),
            retryable: false,
        },
        _ => refused(),
    })
}

/// One to twenty group names, each nonblank, without control characters, at
/// most 128 characters.
fn group_list(args: &Value, key: &str) -> Result<Vec<String>, String> {
    let invalid = || format!("argument_invalid:{key}");
    let items = args
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(invalid)?;
    if items.is_empty() || items.len() > MAX_GROUPS {
        return Err(invalid());
    }
    items
        .iter()
        .map(|item| {
            item.as_str()
                .filter(|name| {
                    !name.trim().is_empty()
                        && name.chars().count() <= MAX_GROUP_CHARS
                        && !name.chars().any(char::is_control)
                })
                .map(str::to_string)
                .ok_or_else(invalid)
        })
        .collect()
}

/// What leaves of a canonical dump. Every field named here is either a number,
/// a code, or wording fixed in the admitted test's source.
fn project(dump: &Value) -> Result<Value, String> {
    let not_projectable = || RESULT_NOT_PROJECTABLE.to_string();
    let list = |key: &str| dump[key].as_array().ok_or_else(not_projectable);
    let (figures, findings) = (list("figures")?, list("findings")?);
    if figures.len() > MAX_FIGURES || findings.len() > MAX_FINDINGS {
        return Err(RESULT_TOO_LARGE.to_string());
    }
    // Ids, units and values are closed shapes: a number or null for a value,
    // a known unit, and ids made only of lower-case letters, digits and
    // underscores around one separator. Anything else (a text value, an id
    // with a space, a capital or punctuation) could be a name from the book.
    let closed = |text: &Value, separator: char| {
        text.as_str().is_some_and(|id| {
            id.split_once(separator).is_some_and(|(head, tail)| {
                let part = |part: &str| {
                    !part.is_empty()
                        && part
                            .bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
                };
                part(head) && part(tail)
            })
        })
    };
    for figure in figures {
        let value_ok = figure["value"].is_number() || figure["value"].is_null();
        let unit_ok = matches!(
            figure["unit"].as_str(),
            Some("paise" | "bp" | "count" | "days")
        );
        if !(closed(&figure["id"], '.') && value_ok && unit_ok) {
            return Err(not_projectable());
        }
    }
    for finding in findings {
        let facts = finding["facts"].as_array().ok_or_else(not_projectable)?;
        let facts_ok = facts.iter().all(|fact| {
            closed(&fact["figure_id"], '.')
                && fact["name"].as_str().is_some_and(|name| {
                    !name.is_empty()
                        && name
                            .bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
                })
        });
        if !(closed(&finding["id"], '/') && facts_ok) {
            return Err(not_projectable());
        }
    }
    let pick = |source: &Value, keys: &[&str]| -> Value {
        let mut out = serde_json::Map::new();
        for key in keys {
            if let Some(value) = source.get(*key) {
                out.insert((*key).to_string(), value.clone());
            }
        }
        Value::Object(out)
    };
    // The failed book checks, by code and count. Their subject and detail
    // name the ledger or voucher at fault and stay here.
    let mut failed: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    for key in [
        "book_invariant_violations",
        "result_invariant_violations",
        "module_invariant_violations",
    ] {
        for violation in list(key)? {
            let code = violation["invariant"]
                .as_str()
                .ok_or_else(not_projectable)?;
            *failed.entry(code.to_string()).or_default() += 1;
        }
    }
    let state = if failed.is_empty() {
        STATE_COMPLETED
    } else {
        STATE_CHECKS_FAILED
    };
    Ok(json!({
        "state": state,
        "test": pick(dump, &["test_id", "test_version", "rules_version"]),
        "population_note": dump["population_note_text"],
        "figures": figures
            .iter()
            .map(|figure| pick(figure, &["id", "value", "unit", "definition_text"]))
            .collect::<Vec<_>>(),
        "findings": findings
            .iter()
            .map(|finding| pick(finding, &["id", "confidence", "clauses", "facts", "title_text", "limits_text", "ask_client_text"]))
            .collect::<Vec<_>>(),
        "book_checks": {
            "evaluated": dump["book_invariants_evaluated"],
            "failed": failed
                .into_iter()
                .map(|(invariant, count)| json!({"invariant": invariant, "count": count}))
                .collect::<Vec<_>>(),
        },
        "limitations": [
            "Books only: the figures are what the company's own books show for the period, not a tax opinion, and a finding says what a document would be needed to confirm.",
            "The cash and bank groups are the ones the caller named; a ledger outside them is not counted, and whether an overdraft group is a bank leg is a judgement for the user's accountant.",
        ],
    }))
}

#[cfg(test)]
#[path = "agent_audit_run_tests.rs"]
mod tests;
