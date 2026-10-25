//! What a test produces (the reference Python implementation's findings module): figures, each
//! a number a CA may see with its unit and one-line definition, and findings that cite figures
//! by id.

use crate::error::{AuditError, Result};

/// A figure's value. Money is paise and ratios are basis points, both integers; a ratio with
/// a zero denominator is `Undefined`, which is not zero. There is no float variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Undefined,
    Text(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Paise,
    BasisPoints,
    Count,
    Days,
    Text,
}

impl Unit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Paise => "paise",
            Self::BasisPoints => "bp",
            Self::Count => "count",
            Self::Days => "days",
            Self::Text => "text",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    Computed,
    Indicative,
    NeedsDocument,
    JudgementRequired,
}

impl Confidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Computed => "computed",
            Self::Indicative => "indicative",
            Self::NeedsDocument => "needs_document",
            Self::JudgementRequired => "judgement_required",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRef {
    pub kind: String,
    pub id: String,
    pub label: String,
}

impl EvidenceRef {
    pub fn new(kind: &str, id: &str) -> Self {
        Self {
            kind: kind.to_string(),
            id: id.to_string(),
            label: String::new(),
        }
    }

    /// An evidence ref carrying a label: the reference engine's canonical serialiser compares
    /// this field, so a port must reproduce a fixed label format exactly (e.g. a voucher's
    /// `"{vtype} {number} on {date}"`), never free prose that merely conveys the same meaning.
    pub fn with_label(kind: &str, id: &str, label: &str) -> Self {
        Self {
            kind: kind.to_string(),
            id: id.to_string(),
            label: label.to_string(),
        }
    }

    pub fn key(&self) -> String {
        format!("{}:{}", self.kind, self.id)
    }
}

#[derive(Debug, Clone)]
pub struct Figure {
    pub id: String,
    pub value: Value,
    pub unit: Unit,
    pub definition: String,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub id: String,
    pub clauses: Vec<String>,
    pub title: String,
    /// name -> figure id, never a literal number.
    pub facts: Vec<(String, String)>,
    pub evidence: Vec<EvidenceRef>,
    pub confidence: Confidence,
    pub limits: Vec<String>,
    pub ask_client: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_id: String,
    pub test_version: String,
    pub rules_version: String,
    pub figures: Vec<Figure>,
    pub findings: Vec<Finding>,
    pub population_note: String,
}

impl TestResult {
    pub fn new(test_id: &str, test_version: &str, rules_version: &str) -> Self {
        Self {
            test_id: test_id.to_string(),
            test_version: test_version.to_string(),
            rules_version: rules_version.to_string(),
            figures: Vec::new(),
            findings: Vec::new(),
            population_note: String::new(),
        }
    }

    /// Add a figure named `<test_id>.<name>` and return its id.
    ///
    /// # Errors
    ///
    /// `DuplicateFigureId` when the id is already present, as the reference engine's `fig` raises.
    /// Some names are built from configuration (a configured ledger's tag), so a configuration
    /// shape can repeat one without any bug in the test (#644).
    pub fn fig(
        &mut self,
        name: &str,
        value: Value,
        unit: Unit,
        definition: &str,
        evidence: Vec<EvidenceRef>,
    ) -> Result<String> {
        let id = format!("{}.{name}", self.test_id);
        if self.figures.iter().any(|f| f.id == id) {
            return Err(AuditError::DuplicateFigureId(id));
        }
        self.figures.push(Figure {
            id: id.clone(),
            value,
            unit,
            definition: definition.to_string(),
            evidence,
        });
        Ok(id)
    }
}

/// A ratio in basis points, half-up (5.47% is 547); `Undefined` when the denominator is zero.
/// Integer arithmetic throughout, widened to i128 and floored exactly as Python's `//` floors,
/// so a large book cannot overflow and a negative operand rounds the way the reference does.
/// `None` only if the ratio itself does not fit an i64.
pub fn pct_bp(num_paise: i64, den_paise: i64) -> Option<Value> {
    if den_paise == 0 {
        return Some(Value::Undefined);
    }
    let (num, den) = (i128::from(num_paise), i128::from(den_paise));
    let (a, b) = (num * 10_000 * 2 + den, 2 * den);
    let q = a / b;
    let floored = if a % b != 0 && ((a < 0) != (b < 0)) {
        q - 1
    } else {
        q
    };
    i64::try_from(floored).ok().map(Value::Int)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_repeated_figure_id_is_refused_with_its_typed_error() {
        let mut r = TestResult::new("t", "1", "r");
        let add = |r: &mut TestResult, name: &str, v: i64| {
            r.fig(name, Value::Int(v), Unit::Count, "d", Vec::new())
        };
        assert_eq!(add(&mut r, "a", 1).unwrap(), "t.a");
        assert_eq!(add(&mut r, "b", 2).unwrap(), "t.b");
        let err = add(&mut r, "a", 3).unwrap_err();
        assert!(
            matches!(&err, AuditError::DuplicateFigureId(id) if id == "t.a"),
            "{err}"
        );
        // The refused figure is not kept: the first `a` stands, and nothing was appended.
        let kept: Vec<_> = r
            .figures
            .iter()
            .map(|f| (f.id.as_str(), &f.value))
            .collect();
        assert_eq!(kept, [("t.a", &Value::Int(1)), ("t.b", &Value::Int(2))]);
    }
}
