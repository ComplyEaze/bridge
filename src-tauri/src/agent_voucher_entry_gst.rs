//! GST details of one voucher ledger entry (#2027): its rate per duty head, read from
//! `ALLLEDGERENTRIES.LIST/RATEDETAILS.LIST` (`GSTRATEDUTYHEAD`, `GSTRATEVALUATIONTYPE`, `GSTRATE`,
//! `GSTRATEPERUNIT`), its taxability
//! (`GSTOVRDNTAXABILITY`) and its type of supply (`GSTOVRDNTYPEOFSUPPLY`).
//!
//! The vouchers read already asks for these (`ALLLEDGERENTRIES.*`), and captured answers to it
//! carry them: on the synthetic book of
//! `agent/audit-read-pilot-lab/pilot-lab-audit-vouchers.utf16le.xml`, 48 sales and purchase
//! entries each carry three non-zero rates, a taxability and a type of supply, and the party and
//! tax entries carry an empty rate list. Taxability and type of supply are each absent, empty or a
//! value (trimmed); a duty head and its valuation type likewise; a rate and a per-unit rate are
//! each absent, empty, zero or a value with its text as sent ([`GstRate`], the same reading as a
//! ledger's own rates), and those are never merged. A zero is what Tally sent (a captured Purchase
//! with tax charged carries five zero rates), so a zero is not a zero-rated supply, and a
//! valuation type of `Not Applicable` says the head does not apply. On an item invoice the rates
//! sit with the goods line, not on the entry. A rate list with no children is not listed. A rate
//! list nested deeper than the entry (inside a goods line, say) belongs to that line and is not
//! read here.
//!
//! These are the details Tally sent on the entry, not the tax charged: the tax entries' own
//! amounts are that. Like the bank-allocation collector, this never refuses the read: an entry
//! whose details cannot be read whole carries a typed reason and no partial details.
use bridge_tally_protocol::gst_details::{classify_gst_rate, GstRate};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const RATE_LIST: &str = "RATEDETAILS.LIST";
/// Most rate rows kept for one entry. Tally sends one row per duty head (five in the captures);
/// a list past this is reported, not truncated.
const MAX_RATES: usize = 16;

/// The entry's own fields read: each wire name with the name the tool returns it under.
const ENTRY_FIELDS: [(&str, &str); 2] = [
    ("GSTOVRDNTAXABILITY", "taxability"),
    ("GSTOVRDNTYPEOFSUPPLY", "type_of_supply"),
];

const DUTY_HEAD: &str = "GSTRATEDUTYHEAD";
const VALUATION_TYPE: &str = "GSTRATEVALUATIONTYPE";
const RATE: &str = "GSTRATE";
const PER_UNIT_RATE: &str = "GSTRATEPERUNIT";

/// Whether a rate row's child is one this collector reads.
fn rate_field(tag: &str) -> bool {
    matches!(tag, DUTY_HEAD | VALUATION_TYPE | RATE | PER_UNIT_RATE)
}

/// Why an entry's GST details could not be read. The text is the wire value of
/// `gst_details_unreadable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::agent) enum EntryGstProblem {
    /// A field met twice in the entry or in one rate row.
    FieldDuplicated,
    /// A text or entity that cannot be decoded.
    TextUndecodable,
    /// An element inside a field that is read.
    ShapeUnrecognised,
    /// A rate or per-unit rate that is not digits with an optional single `.` fraction.
    RateNotDecimal,
    /// Two rate rows naming the same duty head.
    DutyHeadRepeated,
    /// More rate rows than the read keeps.
    TooMany,
}

impl EntryGstProblem {
    pub(in crate::agent) const fn reason(self) -> &'static str {
        match self {
            Self::FieldDuplicated => "gst_details_field_duplicated",
            Self::TextUndecodable => "gst_details_text_undecodable",
            Self::ShapeUnrecognised => "gst_details_shape_unrecognised",
            Self::RateNotDecimal => "gst_details_rate_not_decimal",
            Self::DutyHeadRepeated => "gst_details_duty_head_repeated",
            Self::TooMany => "gst_details_rates_too_many",
        }
    }
}

/// Fed by the voucher row loop with the path below `ALLLEDGERENTRIES.LIST` (`tail`). One
/// collector serves one entry at a time; `reset` starts the next.
#[derive(Default)]
pub(in crate::agent) struct EntryGstCollector {
    /// The entry's fields met so far, with their text.
    fields: BTreeMap<&'static str, String>,
    open: Option<OpenRate>,
    rates: Vec<Value>,
    /// Duty heads already listed, as sent and trimmed.
    heads: Vec<String>,
    /// Whether any element read here was met: an entry with none has no key.
    met: bool,
    problem: Option<EntryGstProblem>,
}

/// One rate row being read.
#[derive(Default)]
struct OpenRate {
    duty_head: Option<String>,
    valuation_type: Option<String>,
    rate: Option<String>,
    per_unit_rate: Option<String>,
    any_child: bool,
}

impl OpenRate {
    /// The slot a read child fills, if it is one.
    fn slot(&mut self, tag: &str) -> Option<&mut Option<String>> {
        match tag {
            DUTY_HEAD => Some(&mut self.duty_head),
            VALUATION_TYPE => Some(&mut self.valuation_type),
            RATE => Some(&mut self.rate),
            PER_UNIT_RATE => Some(&mut self.per_unit_rate),
            _ => None,
        }
    }
}

fn entry_field(tag: &str) -> Option<&'static str> {
    ENTRY_FIELDS
        .iter()
        .find(|(wire, _)| *wire == tag)
        .map(|(_, name)| *name)
}

/// A field as absent, empty or a value, the value trimmed.
fn field_state(value: Option<&String>) -> Value {
    match value.map(|value| value.trim()) {
        None => json!({"state": "absent"}),
        Some("") => json!({"state": "empty"}),
        Some(value) => json!({"state": "value", "value": value}),
    }
}

impl EntryGstCollector {
    pub(in crate::agent) fn reset(&mut self) {
        *self = Self::default();
    }

    fn note(&mut self, problem: EntryGstProblem) {
        self.problem.get_or_insert(problem);
    }

    /// A text or entity inside a field read here that cannot be decoded.
    pub(in crate::agent) fn undecodable(&mut self, tail: &[String]) {
        let inside = match tail {
            [field] => entry_field(field).is_some(),
            [list, field] => list == RATE_LIST && rate_field(field),
            _ => false,
        };
        if inside {
            self.note(EntryGstProblem::TextUndecodable);
        }
    }

    /// `tail` is the path of the PARENT of the element that starts.
    pub(in crate::agent) fn start(&mut self, tail: &[String], tag: &str) {
        match tail {
            [] if tag == RATE_LIST => {
                self.met = true;
                self.open = Some(OpenRate::default());
            }
            [] => {
                if let Some(name) = entry_field(tag) {
                    self.met = true;
                    if self.fields.insert(name, String::new()).is_some() {
                        self.note(EntryGstProblem::FieldDuplicated);
                    }
                }
            }
            // An element inside a field that is read: its text would be cut in two.
            [field] if entry_field(field).is_some() => {
                self.note(EntryGstProblem::ShapeUnrecognised)
            }
            [list, field] if list == RATE_LIST && rate_field(field) => {
                self.note(EntryGstProblem::ShapeUnrecognised)
            }
            [list] if list == RATE_LIST => {
                let Some(open) = self.open.as_mut() else {
                    return;
                };
                open.any_child = true;
                let Some(slot) = open.slot(tag) else {
                    return;
                };
                if slot.replace(String::new()).is_some() {
                    self.note(EntryGstProblem::FieldDuplicated);
                }
            }
            _ => {}
        }
    }

    /// `tail` includes the element whose text arrives.
    pub(in crate::agent) fn text(&mut self, tail: &[String], tag: &str, text: &str) {
        match tail {
            [field] if field == tag => {
                if let Some(value) = entry_field(tag).and_then(|name| self.fields.get_mut(name)) {
                    value.push_str(text);
                }
            }
            [list, field] if list == RATE_LIST && field == tag => {
                if let Some(value) = self
                    .open
                    .as_mut()
                    .and_then(|open| open.slot(tag))
                    .and_then(Option::as_mut)
                {
                    value.push_str(text);
                }
            }
            _ => {}
        }
    }

    /// `tail` includes the element that ends.
    pub(in crate::agent) fn end(&mut self, tail: &[String]) {
        if !matches!(tail, [list] if list == RATE_LIST) {
            return;
        }
        let Some(open) = self.open.take() else {
            return;
        };
        // A rate list with no children says nothing: it is not listed.
        if !open.any_child {
            return;
        }
        let (rate, per_unit_rate): (GstRate, GstRate) = match (
            classify_gst_rate(open.rate),
            classify_gst_rate(open.per_unit_rate),
        ) {
            (Ok(rate), Ok(per_unit_rate)) => (rate, per_unit_rate),
            _ => {
                self.note(EntryGstProblem::RateNotDecimal);
                return;
            }
        };
        if let Some(head) = open.duty_head.as_deref().map(str::trim) {
            if !head.is_empty() {
                if self.heads.iter().any(|seen| seen == head) {
                    self.note(EntryGstProblem::DutyHeadRepeated);
                }
                self.heads.push(head.to_string());
            }
        }
        if self.rates.len() == MAX_RATES {
            self.note(EntryGstProblem::TooMany);
            return;
        }
        self.rates.push(json!({
            "duty_head": field_state(open.duty_head.as_ref()),
            "valuation_type": field_state(open.valuation_type.as_ref()),
            "rate": rate,
            "per_unit_rate": per_unit_rate,
        }));
    }

    /// The entry's details, `None` when the entry carried none of the elements read here, or why
    /// they could not be read.
    pub(in crate::agent) fn finish(&mut self) -> Result<Option<Value>, EntryGstProblem> {
        if let Some(problem) = self.problem {
            return Err(problem);
        }
        if !self.met {
            return Ok(None);
        }
        let mut details = serde_json::Map::new();
        for (_, name) in ENTRY_FIELDS {
            details.insert(name.to_string(), field_state(self.fields.get(name)));
        }
        details.insert(
            "rates".into(),
            Value::Array(std::mem::take(&mut self.rates)),
        );
        Ok(Some(Value::Object(details)))
    }
}
