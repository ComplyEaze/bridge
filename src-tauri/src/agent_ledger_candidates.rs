//! Candidates for a ledger name the book does not hold, or holds more than
//! once, so the assistant can ask the user to choose instead of guessing.
//!
//! The resolution itself is `resolve_ledger_name`'s: an exact spelling, or one
//! ledger equal to it in ASCII case and spaces (#1076). This module only says
//! what to show beside a refusal. Nothing here picks a
//! ledger, scores one, or turns a candidate into the resolution (ADR 0016: a
//! near-miss is reported, never resolved). The names come from the catalogue the caller
//! has already read, so no request is added.
use super::*;
use bridge_tally_core::master_binding::{
    self, BindingStatus, Candidates as Found, MasterCatalog, MasterClass, SourceEntity,
    MAX_CANDIDATES_PER_ENTITY,
};
use std::collections::BTreeSet;

/// What a candidate list means. The first four are the core's own states;
/// the last two are this call's: the redaction setting hid the names, or the
/// search could not run. An empty list is never "no such ledger" unless it is
/// `None`, and even that says only that nothing resembles the name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Listing {
    None,
    Listed,
    Truncated,
    Withheld,
    NamesMasked,
    Unavailable,
}

impl Listing {
    /// Whether this state carries a list of names (and so needs room for it).
    pub(super) fn has_list(self) -> bool {
        matches!(self, Self::Listed | Self::Truncated)
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Listed => "listed",
            Self::Truncated => "truncated",
            Self::Withheld => "withheld",
            Self::NamesMasked => "names_masked",
            Self::Unavailable => "unavailable",
        }
    }
}

/// What the refusal shows beside its code.
#[derive(Debug)]
pub(super) struct LedgerMiss {
    pub(super) listing: Listing,
    pub(super) found: usize,
    pub(super) found_is_lower_bound: bool,
    /// Why the search found what it did, or could not run: a core reason
    /// code, when the core gave one.
    pub(super) reason: Option<&'static str>,
}

impl LedgerMiss {
    /// The `candidates_listing` word once the byte budget has had its say: a
    /// list cut to fit is `truncated`, whatever the search reported.
    pub(super) fn listing_word(&self, cut_by_budget: bool) -> &'static str {
        if cut_by_budget && self.listing == Listing::Listed {
            Listing::Truncated.as_str()
        } else {
            self.listing.as_str()
        }
    }

    fn state(listing: Listing) -> Self {
        Self {
            listing,
            found: 0,
            found_is_lower_bound: false,
            reason: None,
        }
    }
}

/// Bridge writes `…` only to shorten a masked party name, and an assistant may
/// retype it as `...`. A name carrying either mark is refused unless it is a
/// ledger's exact spelling, whatever the setting is now (it may have been
/// copied while masking was on): its loose key would offer `RARS` for `Ra…rs`.
fn carries_the_mask_mark(requested: &str) -> bool {
    requested.contains('…') || requested.contains("...")
}

/// Under masking, a retyping without the mark is also a masked spelling when
/// its loose key is that of another ledger's masked form (`Ra..rs`, `Ra rs`):
/// refused as masked, with what to do next, rather than as an unknown name.
/// `resolved` is the ledger the request reached, if any; its own masked form
/// is not another ledger's.
fn is_another_ledgers_masked_form(requested: &str, resolved: Option<&str>, names: &[&str]) -> bool {
    let key = ledger_lookup_key(requested);
    !key.is_empty()
        && names
            .iter()
            .any(|name| Some(*name) != resolved && ledger_lookup_key(&mask(name)) == key)
}

/// One catalogue ledger as a request can reach it (#1085): the spelling its
/// voucher rows carry, and its stored name when the catalogue gave a different
/// one (protocol reference 9.4h). The catalogue's GUID is the ledger's identity
/// and stays in the protocol crate: two entries are two ledgers whatever they
/// are called.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CatalogueLedger {
    row: String,
    stored: Option<String>,
}

impl CatalogueLedger {
    pub(super) fn new(row: &str, stored: Option<&str>) -> Self {
        Self {
            row: row.to_string(),
            stored: stored.map(str::to_string),
        }
    }

    /// The spelling this ledger's voucher rows carry.
    pub(super) fn row(&self) -> &str {
        &self.row
    }

    /// Both spellings the ledger answers to.
    fn spellings(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.row.as_str()).chain(self.stored.as_deref())
    }

    /// The name an answer shows: the stored name when there is one.
    fn display(&self) -> &str {
        self.stored.as_deref().unwrap_or(&self.row)
    }

    fn is_spelled(&self, requested: &str) -> bool {
        self.row == requested || self.stored.as_deref() == Some(requested)
    }
}

/// What to offer when a request cannot be settled between ledgers: their
/// shown names, or their row spellings when two of them show the same name.
fn names_to_choose_from(ledgers: &[&CatalogueLedger]) -> Vec<String> {
    let shown = ledgers.iter().map(|ledger| ledger.display());
    let distinct = shown.clone().collect::<BTreeSet<_>>().len() == ledgers.len();
    let mut names = if distinct {
        shown.map(str::to_string).collect::<Vec<_>>()
    } else {
        ledgers.iter().map(|ledger| ledger.row.clone()).collect()
    };
    names.sort_unstable();
    names
}

/// The `ledger_match` of an answer: the match, and, when the ledger's voucher
/// rows spell it differently from the name shown, that spelling, so a caller
/// can pass it to a tool that keys on it (#1085).
pub(super) fn ledger_match_json(found: &LedgerMatch, row: &str, redaction: Redaction) -> Value {
    let mut json = found.to_json(redaction);
    // Under masking the field would name nothing a caller could pass on.
    if row != found.name() && redaction != Redaction::MaskParties {
        json["ledger_row_spelling"] = redact_value(party_name_value(row.to_string()), redaction);
    }
    json
}

/// Resolves a requested ledger name against the catalogue's ledgers, or
/// refuses as [`resolve_ledger_or_refuse`] does. A spelling that is exactly
/// one ledger's row spelling or stored name reaches that ledger; one that is
/// two ledgers' is ambiguous; any other request is resolved loosely over the
/// shown names. The answer is the match, which shows the stored name, and the
/// spelling the ledger's voucher rows carry, which is what a filter compares.
pub(super) fn resolve_catalogue_ledger_or_refuse(
    ledgers: &[CatalogueLedger],
    requested: &str,
    redaction: Redaction,
) -> Result<(LedgerMatch, String), ToolFailure> {
    let names = ledgers
        .iter()
        .map(CatalogueLedger::display)
        .collect::<Vec<_>>();
    let reached = ledgers
        .iter()
        .filter(|ledger| ledger.is_spelled(requested))
        .collect::<Vec<_>>();
    let resolved = match reached.as_slice() {
        [one] => resolve_ledger_name(names.iter().copied(), one.display())
            .map(|found| (found, one.row.clone())),
        // Not exactly any ledger's spelling: the case-and-spaces fold, over both
        // spellings of every ledger. More than one ledger answering to it, or to a
        // whitespace twin of it, asks, as before the own name was read.
        [] => {
            let key = ledger_spelling_key(requested);
            let spelled = ledgers
                .iter()
                .filter_map(|ledger| {
                    ledger
                        .spellings()
                        .find(|spelling| ledger_spelling_key(spelling) == key)
                        .map(|spelling| (ledger, spelling))
                })
                .collect::<Vec<_>>();
            match spelled.as_slice() {
                [] => Err(LedgerRefusal::NotFound),
                [(one, matched)] => {
                    let twin = ledger_twin_key(matched);
                    let twins = ledgers
                        .iter()
                        .filter(|ledger| ledger.spellings().any(|s| ledger_twin_key(s) == twin))
                        .collect::<Vec<_>>();
                    if twins.len() == 1 {
                        Ok((
                            LedgerMatch::CaseOrSpacing {
                                name: one.display().to_string(),
                            },
                            one.row.clone(),
                        ))
                    } else {
                        Err(LedgerRefusal::Ambiguous(names_to_choose_from(&twins)))
                    }
                }
                many => {
                    let ledgers = many.iter().map(|(ledger, _)| *ledger).collect::<Vec<_>>();
                    Err(LedgerRefusal::Ambiguous(names_to_choose_from(&ledgers)))
                }
            }
        }
        many => Err(LedgerRefusal::SharedSpelling(names_to_choose_from(many))),
    };
    settle_ledger_resolution(resolved, &names, requested, redaction)
}

/// Resolves a requested ledger name against the catalogue, or refuses with
/// the candidates a user can choose from. `ledger_not_found` and
/// `ledger_ambiguous` keep their codes; the candidates are additive.
pub(super) fn resolve_ledger_or_refuse<'a>(
    ledger_names: impl Iterator<Item = &'a str>,
    requested: &str,
    redaction: Redaction,
) -> Result<LedgerMatch, ToolFailure> {
    let names = ledger_names.collect::<Vec<_>>();
    let resolved = resolve_ledger_name(names.iter().copied(), requested).map(|found| (found, ()));
    settle_ledger_resolution(resolved, &names, requested, redaction).map(|(found, ())| found)
}

fn settle_ledger_resolution<T>(
    resolved: Result<(LedgerMatch, T), LedgerRefusal>,
    names: &[&str],
    requested: &str,
    redaction: Redaction,
) -> Result<(LedgerMatch, T), ToolFailure> {
    // A request that is not byte-exact and carries the mask mark is refused,
    // whatever the setting; under masking, so is one that reads like another
    // ledger's masked form. A ledger spelled exactly as asked is still reached.
    let refusal = match resolved {
        Ok((found @ LedgerMatch::Exact { .. }, kept)) => return Ok((found, kept)),
        Ok((found, _))
            if carries_the_mask_mark(requested)
                || (redaction == Redaction::MaskParties
                    && is_another_ledgers_masked_form(requested, Some(found.name()), names)) =>
        {
            return Err("ledger_name_masked".to_string().into());
        }
        Ok(found) => return Ok(found),
        Err(_)
            if carries_the_mask_mark(requested)
                || (redaction == Redaction::MaskParties
                    && is_another_ledgers_masked_form(requested, None, names)) =>
        {
            return Err("ledger_name_masked".to_string().into())
        }
        Err(refusal) => refusal,
    };
    // Masked: no search is made, and only the state is returned, so that a
    // count never answers "does a ledger start with this?" for a caller that
    // cannot see the names.
    let (miss, items) = match &refusal {
        _ if redaction == Redaction::MaskParties => {
            (LedgerMiss::state(Listing::NamesMasked), Vec::new())
        }
        LedgerRefusal::Ambiguous(set) => collision_set(set, "case_or_spacing_equal"),
        LedgerRefusal::SharedSpelling(set) => collision_set(set, "spelling_of_two_ledgers"),
        LedgerRefusal::NotFound => near_misses(names, requested),
    };
    let mut failure = ToolFailure::from(refusal.code().to_string());
    failure.candidates = Some(Box::new(Candidates {
        requested: None,
        items,
        miss: Some(miss),
    }));
    Err(failure)
}

fn item(name: &str, rule: &str) -> Value {
    json!({"name": party_name_value(name.to_string()), "rule": rule})
}

/// The set the resolver found ambiguous, already sorted: every ledger that
/// differs from the request only in case and whitespace, so the count is exact.
fn collision_set(same: &[String], rule: &str) -> (LedgerMiss, Vec<Value>) {
    let found = same.len();
    let listing = if found > MAX_CANDIDATES_PER_ENTITY {
        Listing::Truncated
    } else {
        Listing::Listed
    };
    let items = same
        .iter()
        .take(MAX_CANDIDATES_PER_ENTITY)
        .map(|name| item(name, rule))
        .collect();
    (
        LedgerMiss {
            found,
            ..LedgerMiss::state(listing)
        },
        items,
    )
}

/// The near-misses for a name that matched no ledger: the shared binding
/// rules' candidates, then every ledger equal to it once symbols, accents and
/// spaces are ignored (`lookup_key_equal`). Those are the ledgers the old
/// lookup read without asking (#1076). They are offered whatever the binding
/// rules found, and listed first, so that no spelling that used to resolve
/// meets an empty or cut list without them; they are never chosen. (Under
/// masking nothing is listed, and this is not called.)
fn near_misses(names: &[&str], requested: &str) -> (LedgerMiss, Vec<Value>) {
    let (mut miss, mut listed) = binder_near_misses(names, requested);
    let key = ledger_lookup_key(requested);
    let mut loose = names
        .iter()
        .filter(|name| !key.is_empty() && ledger_lookup_key(name) == key)
        .filter(|name| !listed.iter().any(|(seen, _)| seen == *name))
        .copied()
        .collect::<Vec<_>>();
    loose.sort_unstable();
    loose.dedup();
    let total = loose.len();
    loose.truncate(MAX_CANDIDATES_PER_ENTITY);
    if total > 0 {
        miss.listing = match miss.listing {
            // The binding rules found nothing: the list is these ledgers, and
            // the reason no longer says so.
            Listing::None => {
                miss.reason = None;
                miss.found = total;
                Listing::Listed
            }
            // The rules could not run: these are listed, and the count is a
            // floor, since the rest of the book was not searched.
            Listing::Unavailable => {
                miss.found = total;
                miss.found_is_lower_bound = true;
                Listing::Listed
            }
            // The rules listed everything they found, so these add to it.
            Listing::Listed => {
                miss.found += total;
                Listing::Listed
            }
            // A family was withheld, or the rules' list was already cut: these
            // may already be in their count, so it stays theirs, as a floor.
            Listing::Withheld | Listing::Truncated => {
                miss.found_is_lower_bound = true;
                Listing::Truncated
            }
            other => other,
        };
        if total > loose.len() {
            miss.listing = Listing::Truncated;
        }
        // First, so that neither the cap nor a small response budget, which
        // keeps a list from its front, cuts the ledger the old lookup read.
        let room = MAX_CANDIDATES_PER_ENTITY - loose.len();
        if listed.len() > room {
            listed.truncate(room);
            miss.listing = Listing::Truncated;
        }
        let mut first = loose
            .into_iter()
            .map(|name| (name.to_string(), "lookup_key_equal".to_string()))
            .collect::<Vec<_>>();
        first.append(&mut listed);
        listed = first;
    }
    let items = listed.iter().map(|(name, rule)| item(name, rule)).collect();
    (miss, items)
}

/// The near-misses the shared binding rules find for a name that matched no
/// ledger. A search that could not run is `unavailable`, never `none`: the
/// binding rules refuse a requested name holding a control character, a blank
/// one or one over their length bound, and a catalogue over their bounds or
/// holding a name they refuse.
fn binder_near_misses(names: &[&str], requested: &str) -> (LedgerMiss, Vec<(String, String)>) {
    let unavailable = |reason: &'static str| {
        (
            LedgerMiss {
                reason: Some(reason),
                ..LedgerMiss::state(Listing::Unavailable)
            },
            Vec::new(),
        )
    };
    let catalog = match MasterCatalog::new(MasterClass::Ledger, names.iter().copied()) {
        Ok(catalog) => catalog,
        Err(error) => return unavailable(error.safe_reason_code()),
    };
    let entity = match SourceEntity::new(0, requested) {
        Ok(entity) => entity,
        Err(error) => return unavailable(error.safe_reason_code()),
    };
    let report = match master_binding::bind(&catalog, &[entity]) {
        Ok(report) => report,
        Err(error) => return unavailable(error.safe_reason_code()),
    };
    let Some(binding) = report.entities().first() else {
        return unavailable("master_binding_no_result");
    };
    match &binding.status {
        // An embedded identifier named one ledger. It is shown for the user to
        // confirm and is never the resolution.
        // The core skips its name search on this path, so what else resembles
        // the name was never looked for: one ledger is shown, and the count is
        // a floor.
        BindingStatus::Bound { catalog_name, .. } => (
            LedgerMiss {
                found: 1,
                found_is_lower_bound: true,
                reason: Some("name_search_not_run"),
                ..LedgerMiss::state(Listing::Listed)
            },
            vec![(catalog_name.clone(), "identifier_match".to_string())],
        ),
        BindingStatus::Ambiguous(unresolved) | BindingStatus::Unmatched(unresolved) => {
            let found: &Found = &unresolved.candidates;
            // Exhaustive on the core's enum, so a fifth state is a compile
            // error here and not a silent "listed".
            let listing = match found {
                Found::None => Listing::None,
                Found::Listed { .. } => Listing::Listed,
                Found::Truncated { .. } => Listing::Truncated,
                Found::Withheld { .. } => Listing::Withheld,
            };
            let items = found
                .listed()
                .iter()
                .map(|candidate| {
                    // A rule that cannot be named is shown as such, and the
                    // name is still listed: it is for the user to look at.
                    let rule = serde_json::to_value(candidate.rule)
                        .ok()
                        .and_then(|value| value.as_str().map(str::to_string))
                        .unwrap_or_else(|| "unnamed_rule".to_string());
                    (candidate.catalog_name.clone(), rule)
                })
                .collect();
            (
                LedgerMiss {
                    listing,
                    found: found.found(),
                    found_is_lower_bound: found.count_is_lower_bound(),
                    reason: Some(unresolved.reason.safe_reason_code()),
                },
                items,
            )
        }
    }
}

#[cfg(test)]
#[path = "agent_ledger_candidates_tests.rs"]
mod tests;
