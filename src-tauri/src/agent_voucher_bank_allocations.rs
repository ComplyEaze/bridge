//! Bank allocations of one voucher ledger entry, read from
//! `ALLLEDGERENTRIES.LIST/BANKALLOCATIONS.LIST`.
//!
//! The vouchers read already asks for these (`ALLLEDGERENTRIES.*`). A hand-keyed Contra
//! keeps a transfer reference either in its narration or as the `INSTRUMENTNUMBER` of
//! the bank allocation on each of its two entries, beside a `TRANSACTIONTYPE` (protocol
//! reference §8.2h). Five fields are read: the bank date (`BANKERSDATE`), the instrument
//! number and date, the transaction type and `STATUS`; every other child, the names and
//! account numbers among them, is skipped. Each field is reported as absent, empty or a
//! value, so an empty bank date is never read as "not reconciled" and an absent one never as
//! empty: no reconciliation verdict is drawn from them. An allocation element with no
//! children (most bank entries' answers carry one) is not listed. Like the cost-centre
//! collector this never refuses the read: an allocation that cannot be read whole leaves its
//! entry carrying a typed reason and no partial list.
use serde_json::{json, Value};
use std::collections::BTreeMap;

const ALLOCATION_LIST: &str = "BANKALLOCATIONS.LIST";
/// Most allocations kept for one entry. A voucher entry is one bank movement; a list
/// past this is reported, not truncated.
const MAX_ALLOCATIONS: usize = 8;

/// Why an entry's bank allocations could not be read. The text is the wire value of
/// `bank_allocations_unreadable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::agent) enum BankAllocationProblem {
    /// A field met twice inside one allocation.
    FieldDuplicated,
    /// A text or entity that cannot be decoded.
    TextUndecodable,
    /// More allocations than the read keeps.
    TooMany,
    /// An allocation list nested anywhere but directly under the entry.
    ShapeUnrecognised,
    /// A bank or instrument date that is not a `YYYYMMDD` date.
    DateUnparsable,
}

impl BankAllocationProblem {
    pub(in crate::agent) const fn reason(self) -> &'static str {
        match self {
            Self::FieldDuplicated => "bank_allocation_field_duplicated",
            Self::TextUndecodable => "bank_allocation_text_undecodable",
            Self::TooMany => "bank_allocations_too_many",
            Self::ShapeUnrecognised => "bank_allocation_shape_unrecognised",
            Self::DateUnparsable => "bank_allocation_date_unparsable",
        }
    }
}

/// Fed by the voucher row loop with the path below `ALLLEDGERENTRIES.LIST`
/// (`tail`). One collector serves one entry at a time; `reset` starts the next.
#[derive(Default)]
pub(in crate::agent) struct BankAllocationCollector {
    open: Option<OpenAllocation>,
    rows: Vec<Value>,
    problem: Option<BankAllocationProblem>,
}

/// One allocation being read: the fields met so far, and whether it had any child at all.
#[derive(Default)]
struct OpenAllocation {
    fields: BTreeMap<&'static str, String>,
    any_child: bool,
}

/// The five fields read: each wire name with the name the tool returns it under.
const FIELDS: [(&str, &str); 5] = [
    ("BANKERSDATE", "bank_date"),
    ("INSTRUMENTNUMBER", "instrument_number"),
    ("INSTRUMENTDATE", "instrument_date"),
    ("TRANSACTIONTYPE", "transaction_type"),
    ("STATUS", "status"),
];

/// The two of them that are dates, admitted only as a `YYYYMMDD` date.
const DATE_FIELDS: [&str; 2] = ["bank_date", "instrument_date"];

/// The returned name of a field that is read.
fn wanted(tag: &str) -> Option<&'static str> {
    FIELDS
        .iter()
        .find(|(wire, _)| *wire == tag)
        .map(|(_, name)| *name)
}

impl BankAllocationCollector {
    pub(in crate::agent) fn reset(&mut self) {
        *self = Self::default();
    }

    fn note(&mut self, problem: BankAllocationProblem) {
        self.problem.get_or_insert(problem);
    }

    /// A text or entity inside an allocation that cannot be decoded.
    pub(in crate::agent) fn undecodable(&mut self) {
        self.note(BankAllocationProblem::TextUndecodable);
    }

    /// `tail` is the path of the PARENT of the element that starts.
    pub(in crate::agent) fn start(&mut self, tail: &[String], tag: &str) {
        match (tail, tag) {
            ([], ALLOCATION_LIST) => {
                self.open = Some(OpenAllocation::default());
            }
            (tail, ALLOCATION_LIST) if !tail.is_empty() => {
                self.note(BankAllocationProblem::ShapeUnrecognised)
            }
            // An element inside a field that is read: its text would be cut in two.
            ([c, field], _) if c == ALLOCATION_LIST && wanted(field).is_some() => {
                self.note(BankAllocationProblem::ShapeUnrecognised)
            }
            ([c], field) if c == ALLOCATION_LIST => {
                let Some(open) = self.open.as_mut() else {
                    return;
                };
                open.any_child = true;
                if let Some(name) = wanted(field) {
                    if open.fields.insert(name, String::new()).is_some() {
                        self.note(BankAllocationProblem::FieldDuplicated);
                    }
                }
            }
            _ => {}
        }
    }

    /// `tail` includes the element whose text arrives.
    pub(in crate::agent) fn text(&mut self, tail: &[String], tag: &str, text: &str) {
        if !matches!(tail, [c, _] if c == ALLOCATION_LIST) {
            return;
        }
        if let (Some(open), Some(name)) = (self.open.as_mut(), wanted(tag)) {
            if let Some(value) = open.fields.get_mut(name) {
                value.push_str(text);
            }
        }
    }

    /// `tail` includes the element that ends.
    pub(in crate::agent) fn end(&mut self, tail: &[String]) {
        if !matches!(tail, [c] if c == ALLOCATION_LIST) {
            return;
        }
        let Some(open) = self.open.take() else {
            return;
        };
        // An allocation element with no children says nothing: it is not listed.
        if !open.any_child {
            return;
        }
        let mut row = serde_json::Map::new();
        for (_, name) in FIELDS {
            let state = match open.fields.get(name).map(|value| value.trim()) {
                None => json!({"state": "absent"}),
                Some("") => json!({"state": "empty"}),
                Some(value) => {
                    if DATE_FIELDS.contains(&name)
                        && bridge_tally_core::TallyDate::parse(value.to_string()).is_err()
                    {
                        self.note(BankAllocationProblem::DateUnparsable);
                    }
                    json!({"state": "value", "value": value})
                }
            };
            row.insert(name.to_string(), state);
        }
        if self.rows.len() == MAX_ALLOCATIONS {
            self.note(BankAllocationProblem::TooMany);
        } else {
            self.rows.push(Value::Object(row));
        }
    }

    /// The entry's allocations, or why they could not be read. An entry with none
    /// yields an empty list.
    pub(in crate::agent) fn finish(&mut self) -> Result<Vec<Value>, BankAllocationProblem> {
        match self.problem {
            Some(problem) => Err(problem),
            None => Ok(std::mem::take(&mut self.rows)),
        }
    }
}
