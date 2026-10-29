//! Goods lines of one voucher ledger entry, read from
//! `ALLLEDGERENTRIES.LIST/INVENTORYALLOCATIONS.LIST`, each with its
//! `BATCHALLOCATIONS.LIST` rows.
//!
//! The vouchers read already asks for these (`ALLLEDGERENTRIES.*`). On an invoice-mode
//! Sales or Purchase Tally nests each goods line under the ledger entry it posts to, and
//! each line's godown and batch split under the line (protocol reference §8.2i).
//! Five fields of a line are read (the item, the actual and billed quantity, the rate and
//! the amount) and five of a batch row (the godown, the batch, the two quantities and the
//! amount); every other child is skipped. Each field is reported as absent, empty or a value,
//! the value trimmed and otherwise as Tally sent it: a quantity or a rate keeps its unit
//! (`100.000 Kgs`, `25.00/Kgs`). A voucher whose stock does not sit under a ledger entry
//! (a Stock Journal, whose entry list is empty in this read) has no goods lines here.
//!
//! Like the cost-centre and bank collectors this never refuses the read: a line that cannot
//! be read whole leaves its entry carrying a typed reason and no partial list. A line's batch
//! amounts must add up to its amount when both are given, as they do on every captured line.
use bridge_tally_core::ExactDecimal;
use serde_json::{json, Value};
use std::collections::BTreeMap;

const LINE_LIST: &str = "INVENTORYALLOCATIONS.LIST";
const BATCH_LIST: &str = "BATCHALLOCATIONS.LIST";
/// Most lines kept for one entry, and most batch rows for one line. Bounds chosen here,
/// not measured; a list past either is reported, not truncated.
const MAX_LINES: usize = 1000;
const MAX_BATCHES: usize = 100;

/// Why an entry's goods lines could not be read. The text is the wire value of
/// `goods_lines_unreadable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::agent) enum GoodsLineProblem {
    /// A line with children but no item name.
    ItemMissing,
    /// A line or batch amount that is not a plain decimal.
    AmountUnparsable,
    /// A line whose batch amounts do not add up to its amount.
    BatchAmountsDiffer,
    /// A field met twice inside one line or one batch row.
    FieldDuplicated,
    /// A text or entity that cannot be decoded.
    TextUndecodable,
    /// More lines, or more batch rows on one line, than the read keeps.
    TooMany,
    /// A line or batch list nested anywhere but where the captures show it, or an
    /// element inside a field that is read.
    ShapeUnrecognised,
}

impl GoodsLineProblem {
    pub(in crate::agent) const fn reason(self) -> &'static str {
        match self {
            Self::ItemMissing => "goods_line_item_missing",
            Self::AmountUnparsable => "goods_line_amount_unparsable",
            Self::BatchAmountsDiffer => "goods_line_batch_amounts_differ",
            Self::FieldDuplicated => "goods_line_field_duplicated",
            Self::TextUndecodable => "goods_line_text_undecodable",
            Self::TooMany => "goods_lines_too_many",
            Self::ShapeUnrecognised => "goods_line_shape_unrecognised",
        }
    }
}

/// The fields read from a line: each wire name with the name the tool returns it under.
const LINE_FIELDS: [(&str, &str); 5] = [
    ("STOCKITEMNAME", "stock_item"),
    ("ACTUALQTY", "actual_quantity"),
    ("BILLEDQTY", "billed_quantity"),
    ("RATE", "rate"),
    ("AMOUNT", "amount"),
];

/// The fields read from a batch row, in the same form.
const BATCH_FIELDS: [(&str, &str); 5] = [
    ("GODOWNNAME", "godown"),
    ("BATCHNAME", "batch"),
    ("ACTUALQTY", "actual_quantity"),
    ("BILLEDQTY", "billed_quantity"),
    ("AMOUNT", "amount"),
];

fn wanted(fields: &[(&'static str, &'static str)], tag: &str) -> Option<&'static str> {
    fields
        .iter()
        .find(|(wire, _)| *wire == tag)
        .map(|(_, name)| *name)
}

/// The fields of one line or batch row met so far, and whether it had any child at all.
#[derive(Default)]
struct Draft {
    fields: BTreeMap<&'static str, String>,
    any_child: bool,
}

impl Draft {
    /// `name` is met: false when it was met before.
    fn open_field(&mut self, name: &'static str) -> bool {
        self.fields.insert(name, String::new()).is_none()
    }

    /// Each field of `fields` as `absent`, `empty` or `value`.
    fn states(&self, fields: &[(&str, &'static str)]) -> serde_json::Map<String, Value> {
        let mut row = serde_json::Map::new();
        for (_, name) in fields {
            let state = match self.fields.get(name).map(|value| value.trim()) {
                None => json!({"state": "absent"}),
                Some("") => json!({"state": "empty"}),
                Some(value) => json!({"state": "value", "value": value}),
            };
            row.insert((*name).to_string(), state);
        }
        row
    }

    /// The amount, when given: `Err` when it is not a plain decimal.
    fn amount(&self) -> Result<Option<ExactDecimal>, GoodsLineProblem> {
        match self.fields.get("amount").map(|value| value.trim()) {
            None | Some("") => Ok(None),
            Some(value) => ExactDecimal::parse(value.to_string())
                .map(Some)
                .map_err(|_| GoodsLineProblem::AmountUnparsable),
        }
    }
}

/// Fed by the voucher row loop with the path below `ALLLEDGERENTRIES.LIST`
/// (`tail`). One collector serves one entry at a time; `reset` starts the next.
#[derive(Default)]
pub(in crate::agent) struct GoodsLineCollector {
    line: Option<Draft>,
    batch: Option<Draft>,
    batches: Vec<Draft>,
    rows: Vec<Value>,
    problem: Option<GoodsLineProblem>,
}

impl GoodsLineCollector {
    pub(in crate::agent) fn reset(&mut self) {
        *self = Self::default();
    }

    fn note(&mut self, problem: GoodsLineProblem) {
        self.problem.get_or_insert(problem);
    }

    /// A text or entity inside a line that cannot be decoded.
    pub(in crate::agent) fn undecodable(&mut self) {
        self.note(GoodsLineProblem::TextUndecodable);
    }

    /// `tail` is the path of the PARENT of the element that starts.
    pub(in crate::agent) fn start(&mut self, tail: &[String], tag: &str) {
        match (tail, tag) {
            ([], LINE_LIST) => {
                self.line = Some(Draft::default());
                self.batches.clear();
            }
            (_, LINE_LIST) => self.note(GoodsLineProblem::ShapeUnrecognised),
            ([c], BATCH_LIST) if c == LINE_LIST => {
                if let Some(line) = self.line.as_mut() {
                    line.any_child = true;
                }
                self.batch = Some(Draft::default());
            }
            (_, BATCH_LIST) => self.note(GoodsLineProblem::ShapeUnrecognised),
            // An element inside a field that is read: its text would be cut in two.
            ([c, field], _) if c == LINE_LIST && wanted(&LINE_FIELDS, field).is_some() => {
                self.note(GoodsLineProblem::ShapeUnrecognised)
            }
            ([c, k, field], _)
                if c == LINE_LIST && k == BATCH_LIST && wanted(&BATCH_FIELDS, field).is_some() =>
            {
                self.note(GoodsLineProblem::ShapeUnrecognised)
            }
            ([c], field) if c == LINE_LIST => {
                let Some(line) = self.line.as_mut() else {
                    return;
                };
                line.any_child = true;
                if let Some(name) = wanted(&LINE_FIELDS, field) {
                    if !line.open_field(name) {
                        self.note(GoodsLineProblem::FieldDuplicated);
                    }
                }
            }
            ([c, k], field) if c == LINE_LIST && k == BATCH_LIST => {
                let Some(batch) = self.batch.as_mut() else {
                    return;
                };
                batch.any_child = true;
                if let Some(name) = wanted(&BATCH_FIELDS, field) {
                    if !batch.open_field(name) {
                        self.note(GoodsLineProblem::FieldDuplicated);
                    }
                }
            }
            _ => {}
        }
    }

    /// `tail` includes the element whose text arrives.
    pub(in crate::agent) fn text(&mut self, tail: &[String], tag: &str, text: &str) {
        // `tag` is the last element that started; text between a batch row's children
        // arrives with the row as the innermost element and must not reach the line's field
        // of that name.
        if tail.last().is_none_or(|last| last != tag) {
            return;
        }
        let (draft, fields) = match tail {
            [c, _] if c == LINE_LIST => (self.line.as_mut(), &LINE_FIELDS),
            [c, k, _] if c == LINE_LIST && k == BATCH_LIST => (self.batch.as_mut(), &BATCH_FIELDS),
            _ => return,
        };
        if let (Some(draft), Some(name)) = (draft, wanted(fields, tag)) {
            if let Some(value) = draft.fields.get_mut(name) {
                value.push_str(text);
            }
        }
    }

    /// `tail` includes the element that ends.
    pub(in crate::agent) fn end(&mut self, tail: &[String]) {
        match tail {
            [c, k] if c == LINE_LIST && k == BATCH_LIST => self.finish_batch(),
            [c] if c == LINE_LIST => self.finish_line(),
            _ => {}
        }
    }

    fn finish_batch(&mut self) {
        let Some(batch) = self.batch.take() else {
            return;
        };
        // A batch element with no children says nothing: it is not listed.
        if !batch.any_child {
            return;
        }
        if self.batches.len() == MAX_BATCHES {
            self.note(GoodsLineProblem::TooMany);
        } else {
            self.batches.push(batch);
        }
    }

    fn finish_line(&mut self) {
        let Some(line) = self.line.take() else {
            return;
        };
        let batches = std::mem::take(&mut self.batches);
        // A line element with no children says nothing: it is not listed.
        if !line.any_child {
            return;
        }
        if line
            .fields
            .get("stock_item")
            .is_none_or(|value| value.trim().is_empty())
        {
            self.note(GoodsLineProblem::ItemMissing);
            return;
        }
        let amount = match line.amount() {
            Ok(amount) => amount,
            Err(problem) => return self.note(problem),
        };
        let mut batch_amounts = Vec::with_capacity(batches.len());
        for batch in &batches {
            match batch.amount() {
                Ok(amount) => batch_amounts.push(amount),
                Err(problem) => return self.note(problem),
            }
        }
        // The split is checked only when the line and every batch row carry an amount.
        if let (Some(amount), false) = (amount.as_ref(), batches.is_empty()) {
            if let Some(amounts) = batch_amounts.iter().cloned().collect::<Option<Vec<_>>>() {
                let agrees = amounts
                    .into_iter()
                    .try_fold(ExactDecimal::zero(), |sum, next| sum.checked_add(&next))
                    .and_then(|total| amount.checked_subtract(&total))
                    .is_ok_and(|difference| difference.is_zero());
                if !agrees {
                    return self.note(GoodsLineProblem::BatchAmountsDiffer);
                }
            }
        }
        let mut row = line.states(&LINE_FIELDS);
        row.insert(
            "stock_item".to_string(),
            json!(line.fields["stock_item"].trim()),
        );
        if !batches.is_empty() {
            row.insert(
                "batches".to_string(),
                Value::Array(
                    batches
                        .iter()
                        .map(|batch| Value::Object(batch.states(&BATCH_FIELDS)))
                        .collect(),
                ),
            );
        }
        if self.rows.len() == MAX_LINES {
            self.note(GoodsLineProblem::TooMany);
        } else {
            self.rows.push(Value::Object(row));
        }
    }

    /// The entry's goods lines, or why they could not be read. An entry with none
    /// yields an empty list.
    pub(in crate::agent) fn finish(&mut self) -> Result<Vec<Value>, GoodsLineProblem> {
        match self.problem {
            Some(problem) => Err(problem),
            None => Ok(std::mem::take(&mut self.rows)),
        }
    }
}
