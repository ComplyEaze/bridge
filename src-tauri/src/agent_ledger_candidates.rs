//! Candidates for a ledger name the book does not hold, or holds more than
//! once, so the assistant can ask the user to choose instead of guessing.
//!
//! The resolution itself stays `resolve_ledger_name`'s, unchanged: a lone
//! ledger whose lookup key equals the request's is still read, as before. This
//! module only says what to show beside a refusal. Nothing here picks a
//! ledger, scores one, or turns a candidate into the resolution (ADR 0016: a
//! near-miss is reported, never resolved). The names come from the catalogue the caller
//! has already read, so no request is added.
use super::*;
use bridge_tally_core::master_binding::{
    self, BindingStatus, Candidates as Found, MasterCatalog, MasterClass, SourceEntity,
    MAX_CANDIDATES_PER_ENTITY,
};

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
/// retype it as `...`. The lookup key ignores everything but letters and
/// digits, so `Ra…rs` finds a ledger named `RARS`: a name carrying either mark
/// is never looked up by its key, whatever the setting is now (it may have
/// been copied while masking was on).
fn carries_the_mask_mark(requested: &str) -> bool {
    requested.contains('…') || requested.contains("...")
}

/// Under masking, a retyping without the mark is also a masked spelling when
/// its key is the key of another ledger's masked form (`Ra..rs`, `Ra rs`).
fn is_another_ledgers_masked_form(requested: &str, resolved: &str, names: &[&str]) -> bool {
    let key = ledger_lookup_key(requested);
    names
        .iter()
        .any(|name| *name != resolved && ledger_lookup_key(&mask(name)) == key)
}

/// Resolves a requested ledger name against the catalogue, or refuses with
/// the candidates a user can choose from. `ledger_not_found` and
/// `ledger_ambiguous` keep their codes; the candidates are additive.
pub(super) fn resolve_ledger_or_refuse<'a>(
    ledger_names: impl Iterator<Item = &'a str>,
    requested: &str,
    redaction: Redaction,
) -> Result<String, ToolFailure> {
    let names = ledger_names.collect::<Vec<_>>();
    let resolved = resolve_ledger_name(names.iter().copied(), requested);
    // A request that is not byte-exact and carries the mask mark is refused,
    // whatever the setting; under masking, so is one that reads like another
    // ledger's masked form. A ledger spelled exactly as asked is still reached.
    match &resolved {
        Ok(name) if name == requested => return Ok(name.clone()),
        Ok(name)
            if carries_the_mask_mark(requested)
                || (redaction == Redaction::MaskParties
                    && is_another_ledgers_masked_form(requested, name, &names)) =>
        {
            return Err("ledger_name_masked".to_string().into())
        }
        Err(_) if carries_the_mask_mark(requested) => {
            return Err("ledger_name_masked".to_string().into())
        }
        _ => {}
    }
    let code = match resolved {
        Ok(name) => return Ok(name),
        Err(code) => code,
    };
    // Masked: no search is made, and only the state is returned, so that a
    // count never answers "does a ledger start with this?" for a caller that
    // cannot see the names.
    let (miss, items) = if redaction == Redaction::MaskParties {
        (LedgerMiss::state(Listing::NamesMasked), Vec::new())
    } else if code == "ledger_ambiguous" {
        collision_set(&names, requested)
    } else {
        near_misses(&names, requested)
    };
    let mut failure = ToolFailure::from(code);
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

/// Every ledger whose lookup key equals the request's, sorted: the very set
/// the resolver found ambiguous, so the count is exact.
fn collision_set(names: &[&str], requested: &str) -> (LedgerMiss, Vec<Value>) {
    let key = ledger_lookup_key(requested);
    let mut same = names
        .iter()
        .filter(|name| ledger_lookup_key(name) == key)
        .copied()
        .collect::<Vec<_>>();
    same.sort_unstable();
    let found = same.len();
    let listing = if found > MAX_CANDIDATES_PER_ENTITY {
        Listing::Truncated
    } else {
        Listing::Listed
    };
    let items = same
        .into_iter()
        .take(MAX_CANDIDATES_PER_ENTITY)
        .map(|name| item(name, "lookup_key_equal"))
        .collect();
    (
        LedgerMiss {
            found,
            ..LedgerMiss::state(listing)
        },
        items,
    )
}

/// The near-misses the shared binding rules find for a name that matched no
/// ledger. A search that could not run is `unavailable`, never `none`: the
/// binding rules refuse a requested name holding a control character, a blank
/// one or one over their length bound, and a catalogue over their bounds or
/// holding a name they refuse.
fn near_misses(names: &[&str], requested: &str) -> (LedgerMiss, Vec<Value>) {
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
            vec![item(catalog_name, "identifier_match")],
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
                    item(&candidate.catalog_name, &rule)
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
