//! Search over the rows a `vouchers` window read already holds (#1230).
//!
//! The arguments are parsed once into a [`VoucherSearch`] before the window is read, so a refused
//! search is refused before the window is read. The search then runs on the rows after the window is labelled and
//! after every other selector, exactly as the ledger and type selectors do, so a zero from a
//! `complete` window is a checked zero. No new Tally request is made.
use super::*;

/// The shortest narration term a search accepts, in characters after folding: shorter terms
/// match most of a book and say nothing.
pub(super) const MIN_NARRATION_TERM_CHARS: usize = 3;

/// The longest term of any criterion, in characters. A voucher number, a reference and a
/// narration phrase are short; a longer term is a paste, not a search.
pub(super) const MAX_SEARCH_TERM_CHARS: usize = 256;

/// What a search asks for. Every criterion present must hold (AND). A struct with no
/// criterion is not constructible: [`VoucherSearch::from_args`] returns `None` instead.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct VoucherSearch {
    /// Trimmed, as is the book's number; compared whole, ignoring ASCII case.
    voucher_number: Option<String>,
    /// Trimmed; compared whole, ignoring ASCII case.
    reference: Option<String>,
    /// `narration_search_key` of the term; a substring of the same key of the narration.
    narration: Option<String>,
    /// The unsigned canonical amount, compared by numeric value with the absolute value of
    /// each ledger entry of a voucher.
    amount: Option<bridge_tally_core::ExactDecimal>,
    /// Only vouchers whose narration ends in a suspense tag Bridge writes. Always together
    /// with the `ledger` selector, so "on the suspense ledger" is decided by the ledger and
    /// the tag only classifies those vouchers.
    suspense_tagged: bool,
}

fn search_failure(code: &str) -> ToolFailure {
    ToolFailure::from(code.to_string())
}

/// One criterion's text: absent stays absent, a blank or over-long value is refused.
fn criterion(args: &Value, key: &str) -> Result<Option<String>, ToolFailure> {
    let Some(value) = optional_string(args, key)? else {
        return Ok(None);
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(search_failure("search_criterion_empty"));
    }
    if trimmed.chars().count() > MAX_SEARCH_TERM_CHARS {
        return Err(search_failure("search_criterion_too_long"));
    }
    Ok(Some(trimmed.to_string()))
}

impl VoucherSearch {
    /// The search the arguments ask for, or `None` when they name no criterion.
    ///
    /// Under `drop_narration` redaction a narration phrase is refused: the setting keeps
    /// narrations from the assistant, and a search that answers whether a phrase occurs
    /// would let it read them back one probe at a time.
    pub(super) fn from_args(
        args: &Value,
        redaction: Redaction,
    ) -> Result<Option<Self>, ToolFailure> {
        let voucher_number = criterion(args, "voucher_number")?;
        let reference = criterion(args, "reference")?;
        let narration = match criterion(args, "narration_contains")? {
            Some(_) if redaction == Redaction::DropNarration => {
                return Err(search_failure("search_narration_redacted"));
            }
            Some(term) => {
                let key = bridge_tally_core::text_search::narration_search_key(&term);
                if key.chars().count() < MIN_NARRATION_TERM_CHARS {
                    return Err(search_failure("search_narration_too_short"));
                }
                Some(key)
            }
            None => None,
        };
        let amount = match criterion(args, "amount")? {
            Some(text) => Some(search_amount(&text)?),
            None => None,
        };
        let suspense_tagged = match args.get("suspense_tagged") {
            None => false,
            Some(Value::Bool(true)) => true,
            Some(_) => return Err(search_failure("search_suspense_tagged_invalid")),
        };
        if suspense_tagged {
            // A tag alone cannot say what is still in suspense (a reallocated voucher keeps
            // its tag), and a narration filter under redaction would leak by probing.
            if optional_string(args, "ledger")?.is_none_or(|ledger| ledger.trim().is_empty()) {
                return Err(search_failure("search_suspense_tagged_needs_ledger"));
            }
            if redaction == Redaction::DropNarration {
                return Err(search_failure("search_narration_redacted"));
            }
        }
        if voucher_number.is_none()
            && reference.is_none()
            && narration.is_none()
            && amount.is_none()
            && !suspense_tagged
        {
            return Ok(None);
        }
        Ok(Some(Self {
            voucher_number,
            reference,
            narration,
            amount,
            suspense_tagged,
        }))
    }

    /// Whether an amount is one of the criteria (so a withheld voucher was kept unjudged).
    pub(super) fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    /// The rows that satisfy every criterion, each carrying `matched`: which criteria held
    /// and, for an amount, the entries (by position in `amounts`) that equalled it.
    ///
    /// A withheld voucher has no amounts to compare, so an amount criterion can neither
    /// admit nor exclude it: it stays, to be listed as withheld, and the caller's result is
    /// `partial` for it. Every other criterion is decided on the fields it does carry.
    pub(super) fn apply(&self, rows: Vec<Value>) -> Vec<Value> {
        rows.into_iter()
            .filter_map(|mut row| {
                let matched = self.matched(&row)?;
                row["matched"] = matched;
                Some(row)
            })
            .collect()
    }

    fn matched(&self, row: &Value) -> Option<Value> {
        let mut matched = serde_json::Map::new();
        if let Some(wanted) = &self.voucher_number {
            let number = row["voucher_number"].as_str()?;
            if !number.trim().eq_ignore_ascii_case(wanted) {
                return None;
            }
            matched.insert("voucher_number".to_string(), json!(true));
        }
        if let Some(wanted) = &self.reference {
            let reference = row["reference"].as_str()?;
            if !reference.eq_ignore_ascii_case(wanted) {
                return None;
            }
            matched.insert("reference".to_string(), json!(true));
        }
        if let Some(wanted) = &self.narration {
            let narration = row["narration"].as_str()?;
            if !bridge_tally_core::text_search::narration_search_key(narration).contains(wanted) {
                return None;
            }
            matched.insert("narration".to_string(), json!(true));
        }
        if self.suspense_tagged {
            let narration = row["narration"].as_str()?;
            let ledgers = row["amounts"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|entry| entry["ledger"].as_str());
            let tag = bridge_bank_statement::proposals::suspense_tag(
                without_trailing_marker(narration),
                ledgers,
            )?;
            matched.insert(
                "suspense_tag".to_string(),
                json!(match tag {
                    bridge_bank_statement::proposals::SuspenseTag::PurposeNotConfirmed => {
                        "purpose_not_confirmed"
                    }
                    bridge_bank_statement::proposals::SuspenseTag::Unidentified => "unidentified",
                }),
            );
        }
        if let Some(wanted) = &self.amount {
            // A withheld voucher has no amounts to compare: it passes this criterion unjudged.
            if row.get(WITHHELD_MARKER).is_none() {
                let entries = row["amounts"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .filter(|(_, entry)| {
                        entry["amount"]
                            .as_str()
                            .and_then(|amount| {
                                bridge_tally_core::ExactDecimal::parse(amount.to_string()).ok()
                            })
                            .is_some_and(|amount| amount.magnitude().numeric_eq(wanted))
                    })
                    .map(|(position, _)| position)
                    .collect::<Vec<_>>();
                if entries.is_empty() {
                    return None;
                }
                matched.insert("amount_entries".to_string(), json!(entries));
            }
        }
        Some(Value::Object(matched))
    }
}

/// A narration without trailing whitespace and without the one closing ` [BRIDGE:...]` marker a
/// hand-imported file carries after it, set off by a space (a native post sends none). A
/// narration with two markers, a marker with no space before it, or a marker that is not the
/// last thing in it keeps its marker, so its tag is not at the end and does not match.
fn without_trailing_marker(narration: &str) -> &str {
    let text = narration.trim_end();
    let Some(start) = text
        .rfind(super::agent_import::NARRATION_MARKER_PREFIX)
        .filter(|start| text[..*start].ends_with(char::is_whitespace))
    else {
        return text;
    };
    let closes_last = text.ends_with(']') && text[start..].matches(']').count() == 1;
    if closes_last && super::agent_import::narration_markers(text).count() == 1 {
        text[..start].trim_end()
    } else {
        text
    }
}

/// An amount to search for: a plain unsigned decimal above zero, in canonical form.
fn search_amount(text: &str) -> Result<bridge_tally_core::ExactDecimal, ToolFailure> {
    let invalid = || search_failure("search_amount_invalid");
    if text.starts_with('-') {
        return Err(invalid());
    }
    let parsed = bridge_tally_core::ExactDecimal::parse(text.to_string()).map_err(|_| invalid())?;
    if parsed.is_zero() {
        return Err(invalid());
    }
    bridge_tally_core::ExactDecimal::zero()
        .checked_add(&parsed)
        .map_err(|_| invalid())
}

#[cfg(test)]
#[path = "agent_voucher_search_tests.rs"]
mod tests;
