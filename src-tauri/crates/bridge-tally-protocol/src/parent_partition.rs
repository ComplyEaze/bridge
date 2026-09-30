//! Splits a book's compliance ledger read into parts by immediate parent
//! group (#679), and proves the parts add up to the catalogue.
//!
//! A `List of Ledgers` read filtered by `$Parent` returns exactly the rows the
//! whole read would return for that parent, byte for byte, and the union of
//! the parts equals the catalogue (`docs/tally/TALLY_PROTOCOL_REFERENCE.md`
//! section 11e: 14 parts against a book of 1,989 ledgers; an `OR` of two
//! parents, one of them carrying `&`, and the reserved root named
//! `&#4; Primary`, on synthetic and client books). Nothing else about the
//! filter is measured, so every bound here is a [`PartitionLimits`] argument
//! chosen by the caller.
//!
//! The rules are types rather than reminders (AGENTS.md P2, P3):
//! - a [`ParentName`] can only be built from text that is safe to place inside
//!   a quoted Tally formula literal, so no request can carry a parent that
//!   ends the literal early;
//! - a [`ParentPartition`] can only be built from a catalogue in which every
//!   ledger has a parent and every part fits the limits, so an over-budget
//!   part is a refusal and never a request;
//! - a [`PartitionCoverage`] is how the rows of the parts are accepted, and
//!   [`PartitionCoverage::finish`] is the only way to learn the parts covered
//!   the catalogue.

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{xml_text::escape_text, TALLY_SANITIZED_ROOT_MARKER};

/// The name of the `SYSTEM` formula a part request adds to the collection.
pub const PARENT_FORMULA_NAME: &str = "BridgeParentPart";

/// The reserved account root as it is written in a request: Tally's own
/// `&#4; Primary`, the form its exports carry (section 1.1(d), section 11e).
const RESERVED_ROOT_REQUEST_LITERAL: &str = "&#4; Primary";

/// Why a partition could not be planned, or why a part's rows did not add up
/// to the catalogue. Every variant is data-free: it names no ledger, no group
/// and no company.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentPartitionError {
    /// A ledger carries no immediate parent, so no `$Parent` part holds it.
    LedgerWithoutParent,
    /// `ledgers` ledgers carry a parent name that cannot be placed in a formula
    /// literal: a blank name, one holding a quote or a control character, or a
    /// reserved-value marker that is not the root. It says how many ledgers,
    /// never which parent or which ledger.
    ParentNameUnsupported { ledgers: u64 },
    /// Two catalogue rows share one GUID, so a row cannot be matched to one
    /// ledger.
    DuplicateLedgerIdentity,
    /// One parent holds more ledgers than a part may. It is not split: no
    /// filter finer than the parent is proven.
    ParentOverBudget { ledgers: u64 },
    /// The parents need more parts than the limit allows.
    TooManyParts { parts: u64 },
    /// A part answered a row whose parent is not one of that part's parents.
    RowOutsideParts,
    /// A part answered a row the catalogue does not hold.
    RowNotInCatalogue,
    /// A part answered a row whose name or parent differs from the
    /// catalogue's row with that GUID.
    RowDiffersFromCatalogue,
    /// Two rows, in one part or two, carry the same GUID.
    RowRepeated,
    /// Catalogue ledgers no part answered.
    RowsMissing,
}

impl ParentPartitionError {
    /// A stable, data-free name for this refusal, safe to return to an agent.
    pub const fn safe_code(self) -> &'static str {
        match self {
            Self::LedgerWithoutParent => "ledger_without_parent",
            Self::ParentNameUnsupported { .. } => "parent_name_unsupported",
            Self::DuplicateLedgerIdentity => "parent_partition_duplicate_ledger_identity",
            Self::ParentOverBudget { .. } => "parent_over_budget",
            Self::TooManyParts { .. } => "parent_partition_too_many_parts",
            Self::RowOutsideParts => "parent_part_row_outside_parents",
            Self::RowNotInCatalogue => "parent_part_row_not_in_catalogue",
            Self::RowDiffersFromCatalogue => "parent_part_row_differs_from_catalogue",
            Self::RowRepeated => "parent_part_row_repeated",
            Self::RowsMissing => "parent_part_rows_missing",
        }
    }
}

impl std::fmt::Display for ParentPartitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::LedgerWithoutParent => "a ledger has no immediate parent group",
            Self::ParentNameUnsupported { .. } => "a parent group name cannot be used in a filter",
            Self::DuplicateLedgerIdentity => "the ledger catalogue repeats a ledger GUID",
            Self::ParentOverBudget { .. } => "one parent group holds more ledgers than a part may",
            Self::TooManyParts { .. } => "the parent groups need more parts than allowed",
            Self::RowOutsideParts => "a part returned a ledger outside its parent groups",
            Self::RowNotInCatalogue => "a part returned a ledger the catalogue does not hold",
            Self::RowDiffersFromCatalogue => {
                "a part returned a ledger that differs from the catalogue"
            }
            Self::RowRepeated => "a ledger was returned more than once across the parts",
            Self::RowsMissing => "the parts did not return every catalogue ledger",
        })
    }
}

impl std::error::Error for ParentPartitionError {}

/// What the catalogue observed for one ledger's immediate parent. A parent
/// the catalogue could not carry safely (a control or deceptive display
/// character, a blank or over-long name) is `Unsupported`, not `Absent`: the
/// ledger has a parent, and the filter cannot name it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentObservation<'a> {
    Absent,
    Unsupported,
    Named(&'a str),
}

impl<'a> From<Option<&'a str>> for ParentObservation<'a> {
    fn from(parent: Option<&'a str>) -> Self {
        match parent {
            Some(name) if !name.is_empty() => Self::Named(name),
            _ => Self::Absent,
        }
    }
}

/// An immediate parent group name that is safe to place in a quoted formula
/// literal, held as the catalogue returned it.
///
/// Escaping cannot protect a formula literal (see [`crate::xml_text`]): Tally
/// decodes `&quot;` back to `"` before evaluating, so a quote would end the
/// literal early. A name with a quote is therefore refused, as is any control
/// character. The reserved account root is the one name that carries a
/// U+FFFD marker, and only in its exact form; any other marked value (Tally's
/// other reserved names) is refused rather than guessed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParentName {
    text: String,
}

impl ParentName {
    /// `text` is the parent exactly as the catalogue decoded it.
    pub fn parse(text: &str) -> Result<Self, ParentPartitionError> {
        if text.trim().is_empty() || text.contains('"') || text.chars().any(char::is_control) {
            return Err(ParentPartitionError::ParentNameUnsupported { ledgers: 1 });
        }
        if text.contains('\u{fffd}') && !is_exact_reserved_root(text) {
            return Err(ParentPartitionError::ParentNameUnsupported { ledgers: 1 });
        }
        Ok(Self {
            text: text.to_owned(),
        })
    }

    /// The name as the catalogue returned it, for comparing a part's rows.
    pub fn as_catalogue_text(&self) -> &str {
        &self.text
    }

    /// The name as it is written inside the formula's quotes: the reserved
    /// root as `&#4; Primary`, any other name XML-escaped once.
    fn request_literal(&self) -> String {
        if is_exact_reserved_root(&self.text) {
            RESERVED_ROOT_REQUEST_LITERAL.to_owned()
        } else {
            escape_text(&self.text)
        }
    }
}

fn is_exact_reserved_root(text: &str) -> bool {
    text.strip_prefix(TALLY_SANITIZED_ROOT_MARKER) == Some(" Primary")
}

/// The limits a plan must fit. Passed in, not fixed here, because none is a
/// measured Tally limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartitionLimits {
    /// Most ledgers one part may hold.
    pub max_ledgers_per_part: u64,
    /// Most parents one part's formula may name.
    pub max_parents_per_part: usize,
    /// Most parts a book may need.
    pub max_parts: usize,
}

/// One filtered read: the parents its formula names and the ledgers the
/// catalogue says they hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentPart {
    parents: Vec<ParentName>,
    ledger_count: u64,
}

impl ParentPart {
    pub fn parents(&self) -> &[ParentName] {
        &self.parents
    }

    /// The ledgers the catalogue holds under this part's parents.
    pub fn ledger_count(&self) -> u64 {
        self.ledger_count
    }

    /// The formula text: `$Parent = "A" OR $Parent = "B"`.
    pub fn formula(&self) -> String {
        self.parents
            .iter()
            .map(|parent| format!("$Parent = \"{}\"", parent.request_literal()))
            .collect::<Vec<_>>()
            .join(" OR ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpectedRow {
    name: String,
    parent: String,
    part: usize,
}

/// Every catalogue ledger assigned to exactly one part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentPartition {
    parts: Vec<ParentPart>,
    expected: HashMap<String, ExpectedRow>,
}

impl ParentPartition {
    /// Plans the parts for a catalogue's rows, `(name, guid, immediate
    /// parent)`, packing parents largest first into the first part with room.
    /// Deterministic: the same rows give the same parts in any input order.
    pub fn plan<'a>(
        rows: impl IntoIterator<Item = (&'a str, &'a str, ParentObservation<'a>)>,
        limits: PartitionLimits,
    ) -> Result<Self, ParentPartitionError> {
        let mut counts = BTreeMap::<ParentName, u64>::new();
        let mut rows_by_guid = HashMap::<String, (String, ParentName)>::new();
        let mut unsupported = 0u64;
        for (name, guid, parent) in rows {
            let parent = match parent {
                ParentObservation::Absent => return Err(ParentPartitionError::LedgerWithoutParent),
                ParentObservation::Unsupported => {
                    unsupported += 1;
                    continue;
                }
                ParentObservation::Named(text) => match ParentName::parse(text) {
                    Ok(parent) => parent,
                    Err(_) => {
                        unsupported += 1;
                        continue;
                    }
                },
            };
            *counts.entry(parent.clone()).or_default() += 1;
            let previous =
                rows_by_guid.insert(guid.to_ascii_lowercase(), (name.to_owned(), parent));
            if previous.is_some() {
                return Err(ParentPartitionError::DuplicateLedgerIdentity);
            }
        }
        if unsupported > 0 {
            return Err(ParentPartitionError::ParentNameUnsupported {
                ledgers: unsupported,
            });
        }
        let mut ordered = counts.into_iter().collect::<Vec<_>>();
        ordered.sort_by(|(left_name, left), (right_name, right)| {
            right.cmp(left).then_with(|| left_name.cmp(right_name))
        });
        let mut parts = Vec::<ParentPart>::new();
        for (parent, ledgers) in ordered {
            if ledgers > limits.max_ledgers_per_part {
                return Err(ParentPartitionError::ParentOverBudget { ledgers });
            }
            let room = parts.iter_mut().find(|part| {
                part.parents.len() < limits.max_parents_per_part
                    && part.ledger_count + ledgers <= limits.max_ledgers_per_part
            });
            match room {
                Some(part) => {
                    part.parents.push(parent);
                    part.ledger_count += ledgers;
                }
                None => parts.push(ParentPart {
                    parents: vec![parent],
                    ledger_count: ledgers,
                }),
            }
        }
        if parts.len() > limits.max_parts {
            return Err(ParentPartitionError::TooManyParts {
                parts: parts.len() as u64,
            });
        }
        for part in &mut parts {
            part.parents.sort();
        }
        let part_of = parts
            .iter()
            .enumerate()
            .flat_map(|(index, part)| part.parents.iter().map(move |parent| (parent, index)))
            .collect::<HashMap<_, _>>();
        let expected = rows_by_guid
            .into_iter()
            .map(|(guid, (name, parent))| {
                let part = part_of[&parent];
                (
                    guid,
                    ExpectedRow {
                        name,
                        parent: parent.as_catalogue_text().to_owned(),
                        part,
                    },
                )
            })
            .collect();
        Ok(Self { parts, expected })
    }

    pub fn parts(&self) -> &[ParentPart] {
        &self.parts
    }

    /// A fresh tracker that accepts the parts' rows against this plan.
    pub fn coverage(&self) -> PartitionCoverage {
        PartitionCoverage {
            remaining: self.expected.clone(),
            seen: HashSet::new(),
        }
    }
}

/// Accepts the rows each part answered, and says whether together they are
/// the catalogue: no row outside its part, none invented, none changed, none
/// repeated, none missing. GUIDs compare ignoring ASCII case.
#[derive(Debug, Clone)]
pub struct PartitionCoverage {
    remaining: HashMap<String, ExpectedRow>,
    seen: HashSet<String>,
}

impl PartitionCoverage {
    /// `part` is the index into [`ParentPartition::parts`] the row came from;
    /// `parent` is the row's immediate parent as the part's response carried
    /// it, `None` when it named none.
    pub fn accept(
        &mut self,
        part: usize,
        guid: &str,
        name: &str,
        parent: Option<&str>,
    ) -> Result<(), ParentPartitionError> {
        let key = guid.to_ascii_lowercase();
        let Some(expected) = self.remaining.get(&key) else {
            return Err(if self.seen.contains(&key) {
                ParentPartitionError::RowRepeated
            } else {
                ParentPartitionError::RowNotInCatalogue
            });
        };
        if expected.part != part {
            return Err(ParentPartitionError::RowOutsideParts);
        }
        if expected.name != name || Some(expected.parent.as_str()) != parent {
            return Err(ParentPartitionError::RowDiffersFromCatalogue);
        }
        self.remaining.remove(&key);
        self.seen.insert(key);
        Ok(())
    }

    /// Ok only when every catalogue ledger was accepted.
    pub fn finish(self) -> Result<(), ParentPartitionError> {
        if self.remaining.is_empty() {
            Ok(())
        } else {
            Err(ParentPartitionError::RowsMissing)
        }
    }
}

#[cfg(test)]
#[path = "parent_partition_tests.rs"]
mod tests;
