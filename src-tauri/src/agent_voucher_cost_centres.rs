//! Cost-centre allocations of one voucher ledger entry, read from
//! `ALLLEDGERENTRIES.LIST/CATEGORYALLOCATIONS.LIST/COSTCENTREALLOCATIONS.LIST`.
//!
//! The vouchers read already asks for these (`ALLLEDGERENTRIES.*`) and a live capture
//! shows the shape: a category with its centres, each a `NAME` and a signed `AMOUNT`
//! (protocol reference §8.2g). This collector never refuses the read. A malformed
//! allocation leaves its entry carrying a typed reason and no partial list: a shipped
//! read that works must not start failing, and a half-read list must not pass for the
//! whole one. An empty amount is never zero and a missing name is never invented.
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const CATEGORY_LIST: &str = "CATEGORYALLOCATIONS.LIST";
const CENTRE_LIST: &str = "COSTCENTREALLOCATIONS.LIST";

/// Why an entry's allocations could not be read. The text is the wire value of
/// `cost_centre_allocations_unreadable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::agent) enum CostCentreProblem {
    NameMissing,
    AmountMissing,
    AmountUnparsable,
    CentreDuplicate,
    CategoryMissing,
    CentresMissing,
    FieldDuplicated,
    TextUndecodable,
    CategoryEmpty,
    /// A category name met twice in one entry: the same centre would be returned twice.
    CategoryRepeated,
    /// An allocation list at a depth the reader does not take.
    ShapeUnrecognised,
}

impl CostCentreProblem {
    pub(in crate::agent) const ALL: [Self; 11] = [
        Self::NameMissing,
        Self::AmountMissing,
        Self::AmountUnparsable,
        Self::CentreDuplicate,
        Self::CategoryMissing,
        Self::CentresMissing,
        Self::FieldDuplicated,
        Self::TextUndecodable,
        Self::CategoryEmpty,
        Self::CategoryRepeated,
        Self::ShapeUnrecognised,
    ];

    /// The problem an entry's `cost_centre_allocations_unreadable` names, by its wire value.
    pub(in crate::agent) fn from_reason(text: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|problem| problem.reason() == text)
    }

    pub(in crate::agent) const fn reason(self) -> &'static str {
        match self {
            Self::NameMissing => "cost_centre_name_missing",
            Self::AmountMissing => "cost_centre_amount_missing",
            Self::AmountUnparsable => "cost_centre_amount_unparsable",
            Self::CentreDuplicate => "cost_centre_duplicate",
            Self::CategoryMissing => "cost_category_missing",
            Self::CentresMissing => "cost_category_without_centres",
            Self::FieldDuplicated => "cost_centre_field_duplicated",
            Self::TextUndecodable => "cost_centre_text_undecodable",
            Self::CategoryEmpty => "cost_category_present_but_empty",
            Self::CategoryRepeated => "cost_category_repeated",
            Self::ShapeUnrecognised => "cost_centre_allocation_shape_unrecognised",
        }
    }
}

#[derive(Default)]
struct Draft {
    fields: BTreeMap<String, String>,
}

/// Fed by the voucher row loop with the path below `ALLLEDGERENTRIES.LIST`
/// (`tail`). One collector serves one entry at a time; `reset` starts the next.
#[derive(Default)]
pub(in crate::agent) struct CostCentreCollector {
    category: Option<Draft>,
    centre: Option<Draft>,
    category_centres: usize,
    /// Where this category's rows start in `rows`: the category's own name may follow
    /// its centres, so its rows get the name when the category ends.
    category_rows_from: usize,
    rows: Vec<Value>,
    /// The centre names seen in the current category (lower case: Tally compares names
    /// without case): a name may repeat under another category, never within one.
    seen: BTreeSet<String>,
    /// The category names seen in this entry, in the same form: a category is met once.
    categories: BTreeSet<String>,
    problem: Option<CostCentreProblem>,
}

impl CostCentreCollector {
    pub(in crate::agent) fn reset(&mut self) {
        *self = Self::default();
    }

    fn note(&mut self, problem: CostCentreProblem) {
        self.problem.get_or_insert(problem);
    }

    /// A text or entity inside the allocations that cannot be decoded.
    pub(in crate::agent) fn undecodable(&mut self) {
        self.note(CostCentreProblem::TextUndecodable);
    }

    /// `tail` is the path of the PARENT of the element that starts.
    pub(in crate::agent) fn start(&mut self, tail: &[String], tag: &str) {
        // An allocation list anywhere but where the capture shows it is not read, and the
        // entry says so: no key must never read as "not allocated" when a list was there.
        let recognised = matches!((tail, tag), ([], CATEGORY_LIST))
            || (matches!(tail, [c] if c == CATEGORY_LIST) && tag == CENTRE_LIST);
        if !recognised && (tag == CATEGORY_LIST || tag == CENTRE_LIST) {
            self.note(CostCentreProblem::ShapeUnrecognised);
        }
        match (tail, tag) {
            ([], CATEGORY_LIST) => {
                self.category = Some(Draft::default());
                self.category_centres = 0;
                self.category_rows_from = self.rows.len();
                self.seen.clear();
            }
            ([c], CENTRE_LIST) if c == CATEGORY_LIST && self.category.is_some() => {
                self.centre = Some(Draft::default());
            }
            ([c], field) if c == CATEGORY_LIST => {
                if let Some(draft) = self.category.as_mut() {
                    if draft
                        .fields
                        .insert(field.to_string(), String::new())
                        .is_some()
                    {
                        self.note(CostCentreProblem::FieldDuplicated);
                    }
                }
            }
            ([c, k], field) if c == CATEGORY_LIST && k == CENTRE_LIST => {
                if let Some(draft) = self.centre.as_mut() {
                    if draft
                        .fields
                        .insert(field.to_string(), String::new())
                        .is_some()
                    {
                        self.note(CostCentreProblem::FieldDuplicated);
                    }
                }
            }
            _ => {}
        }
    }

    /// `tail` includes the element whose text arrives.
    pub(in crate::agent) fn text(&mut self, tail: &[String], tag: &str, text: &str) {
        let target = match tail {
            [c, _] if c == CATEGORY_LIST => self.category.as_mut(),
            [c, k, _] if c == CATEGORY_LIST && k == CENTRE_LIST => self.centre.as_mut(),
            _ => None,
        };
        if let Some(draft) = target {
            if let Some(value) = draft.fields.get_mut(tag) {
                value.push_str(text);
            }
        }
    }

    /// `tail` includes the element that ends.
    pub(in crate::agent) fn end(&mut self, tail: &[String]) {
        match tail {
            [c, k] if c == CATEGORY_LIST && k == CENTRE_LIST => self.finish_centre(),
            [c] if c == CATEGORY_LIST => self.finish_category(),
            _ => {}
        }
    }

    fn finish_centre(&mut self) {
        let Some(centre) = self.centre.take() else {
            return;
        };
        self.category_centres += 1;
        let name = centre
            .fields
            .get("NAME")
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let Some(name) = name else {
            self.note(CostCentreProblem::NameMissing);
            return;
        };
        let amount = centre
            .fields
            .get("AMOUNT")
            .filter(|value| !value.trim().is_empty());
        let Some(amount) = amount else {
            self.note(CostCentreProblem::AmountMissing);
            return;
        };
        if bridge_tally_core::ExactDecimal::parse(amount.clone()).is_err() {
            self.note(CostCentreProblem::AmountUnparsable);
            return;
        }
        if !self.seen.insert(name.to_lowercase()) {
            self.note(CostCentreProblem::CentreDuplicate);
            return;
        }
        // The category name is set when the category ends (it can follow its centres).
        self.rows.push(json!({
            "category": "",
            "cost_centre": name,
            "amount": amount,
        }));
    }

    fn finish_category(&mut self) {
        let Some(category) = self.category.take() else {
            return;
        };
        let named = category
            .fields
            .get("CATEGORY")
            .is_some_and(|value| !value.trim().is_empty());
        if self.category_centres == 0 {
            // Neither shape is in a capture: an entry without allocations has no
            // container at all there, and the one empty container on record (the
            // import-verification read, whose fetch does not ask for the fields) stood
            // for a real allocation. So an empty container is not read as "no
            // allocations": the entry says the container was present and not observed.
            self.note(if named {
                CostCentreProblem::CentresMissing
            } else {
                CostCentreProblem::CategoryEmpty
            });
            return;
        }
        if !named {
            self.note(CostCentreProblem::CategoryMissing);
            return;
        }
        let name = category
            .fields
            .get("CATEGORY")
            .map(|value| value.trim().to_string())
            .unwrap_or_default();
        if !self.categories.insert(name.to_lowercase()) {
            self.note(CostCentreProblem::CategoryRepeated);
            return;
        }
        for row in self.rows.iter_mut().skip(self.category_rows_from) {
            row["category"] = json!(name);
        }
    }

    /// The entry's allocations, or why they could not be read. An entry with none
    /// yields an empty list.
    pub(in crate::agent) fn finish(&mut self) -> Result<Vec<Value>, CostCentreProblem> {
        match self.problem {
            Some(problem) => Err(problem),
            None => Ok(std::mem::take(&mut self.rows)),
        }
    }
}
