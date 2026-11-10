//! `ledger_movement` on a book with several Currency masters, read from the book's first day only
//! (#683). Which ledgers are set aside, by which rule, and what the result says about them.
//!
//! From the first day a currency composite marks every foreign balance; from a later day a foreign
//! party balance reads as a plain rupee figure that no composite test sees
//! (TALLY_PROTOCOL_REFERENCE §8.2n). So this path is reachable only through a
//! [`SeveralCurrencyFirstDay`], which the runtime builds for no other date, and every rule here is
//! measured on one book: its result is always partial.
//!
//! The currency-scoped Trial Balance alone names the ledgers set aside. The movement's own reads
//! check it: an opening at the first day that is a composite, or an entry in the window that is
//! one, must be on a ledger the Trial Balance set aside. On the same book a Trial Balance ending
//! on the first day was seen to read every dollar value as its rupee figure alone, once later
//! vouchers were saved and Tally's current date had moved
//! (`FOREXGROUP_FIRST_DAY_CAPTURE_PROVENANCE.md`); a ledger only the Trial Balance can show, such
//! as `Profit & Loss A/c`, would then be read. So a check that fails refuses the whole read.

use super::super::failure_code::MovementCurrencyCode;
use super::*;
use crate::tally::runtime::{SeveralCurrencyFirstDay, TrialBalanceLedgerScope};
use bridge_tally_protocol::native_outstandings::ForeignCurrencyLedger;
use bridge_tally_protocol::native_trial_balance::NativeTrialBalance;

/// What a several-currency movement says about the ledgers it covers. The Trial Balance's sentence
/// speaks of totals, and the movement returns none.
pub(super) const BASE_CURRENCY_LEDGERS_MOVEMENT_LIMITATION: &str = "Only this book's plain base-currency ledgers are read. Ledgers kept in another currency, and base-currency ledgers with a value Tally shows in another currency, are left out and named, with no figure.";

/// Why the Trial Balance set a ledger aside. A code from a closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SetAsideRule {
    /// The Trial Balance names the ledger as kept in another currency.
    ForeignCurrencyLedger,
    /// A base-currency ledger the Trial Balance shows with a value in another currency.
    MixedCurrencyMovement,
}

/// Where a movement read found a composite on a ledger the Trial Balance reads as plain. A code
/// from a closed set, given as the refusal's `set_aside_reason`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CompositeFoundIn {
    /// The ledger's opening at the first day.
    Opening,
    /// An entry on the ledger in the window.
    WindowEntry,
}

impl CompositeFoundIn {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::Opening => "composite_opening",
            Self::WindowEntry => "composite_entry_in_window",
        }
    }
}

impl SetAsideRule {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::ForeignCurrencyLedger => "foreign_currency_ledger",
            Self::MixedCurrencyMovement => "mixed_currency_movement",
        }
    }
}

/// The ledgers a several-currency movement sets aside, as the Trial Balance names them. Built
/// only from a [`SeveralCurrencyFirstDay`].
#[derive(Debug)]
pub(super) struct CurrencySetAside {
    /// The ledgers kept in another currency, with their `CURRENCYNAME`, in read order.
    foreign: Vec<ForeignCurrencyLedger>,
    /// The base-currency ledgers with a value in another currency.
    mixed: std::collections::BTreeSet<String>,
}

impl CurrencySetAside {
    /// Before any voucher is read. `scope` and `kept` are the currency-scoped Trial Balance of the
    /// same window (its set-aside names and its plain rows); `catalogue` is the movement's own
    /// opening export at the first day, whose composite openings are checked
    /// (`check_composite_openings`).
    ///
    /// Every name the Trial Balance sets aside must be in the catalogue, and every catalogue
    /// ledger must be in the Trial Balance: a spelling or listing difference between the two
    /// reports is unmeasured, so either miss refuses (`ledger_currency_scope_unmatched`).
    pub(super) fn before_window(
        first_day: &SeveralCurrencyFirstDay,
        scope: &TrialBalanceLedgerScope,
        kept: &NativeTrialBalance,
        catalogue: &[TallyLedger],
    ) -> Result<Self, ToolFailure> {
        let TrialBalanceLedgerScope::BaseCurrencyLedgersOnly { foreign, mixed, .. } = scope else {
            // The Trial Balance found one Currency master where the opening read found several:
            // the book changed between the two reads.
            return Err(MovementCurrencyCode::SnapshotDrifted.into());
        };
        let catalogue_names = catalogue
            .iter()
            .map(|ledger| ledger.name.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        let trial_balance_names = kept
            .rows
            .iter()
            .map(|row| row.name.as_str())
            .chain(foreign.iter().map(|ledger| ledger.ledger.as_str()))
            .chain(mixed.iter().map(String::as_str))
            .collect::<std::collections::BTreeSet<_>>();
        if foreign
            .iter()
            .map(|ledger| ledger.ledger.as_str())
            .chain(mixed.iter().map(String::as_str))
            .any(|name| !catalogue_names.contains(name))
            || catalogue_names
                .iter()
                .any(|name| !trial_balance_names.contains(name))
        {
            return Err(MovementCurrencyCode::ScopeUnmatched.into());
        }
        let set_aside = Self {
            foreign: foreign.clone(),
            mixed: mixed.iter().cloned().collect(),
        };
        set_aside.check_composite_openings(first_day)?;
        Ok(set_aside)
    }

    /// Every composite opening at the first day must be on a ledger the Trial Balance set aside.
    fn check_composite_openings(
        &self,
        first_day: &SeveralCurrencyFirstDay,
    ) -> Result<(), ToolFailure> {
        self.require_set_aside(
            first_day.composite_openings().iter().map(String::as_str),
            CompositeFoundIn::Opening,
        )
    }

    /// After the window is read: every ledger with a composite entry in it must be one the Trial
    /// Balance set aside.
    pub(super) fn check_composite_entries(
        &self,
        vouchers: &[MovementVoucher],
    ) -> Result<(), ToolFailure> {
        self.require_set_aside(
            vouchers
                .iter()
                .flat_map(|voucher| &voucher.ledger_entries)
                .filter(|entry| entry.amount == MovementAmount::Composite)
                .map(|entry| entry.ledger_name.as_str()),
            CompositeFoundIn::WindowEntry,
        )
    }

    /// A composite on a ledger the Trial Balance reads as plain means the Trial Balance did not
    /// show every value in another currency, so it cannot be trusted to name every ledger to leave
    /// out: refused (`ledger_currency_scope_inconclusive`), naming the first such ledger and where
    /// its composite was, with no figure.
    fn require_set_aside<'a>(
        &self,
        mut ledgers: impl Iterator<Item = &'a str>,
        found_in: CompositeFoundIn,
    ) -> Result<(), ToolFailure> {
        match ledgers.find(|ledger| self.rule_of(ledger).is_none()) {
            Some(ledger) => Err(ToolFailure::from(MovementCurrencyCode::ScopeInconclusive)
                .with_set_aside(ledger, found_in.code())),
            None => Ok(()),
        }
    }

    /// Why the Trial Balance set `ledger` aside, if it did.
    pub(super) fn rule_of(&self, ledger: &str) -> Option<SetAsideRule> {
        if self.foreign.iter().any(|found| found.ledger == ledger) {
            return Some(SetAsideRule::ForeignCurrencyLedger);
        }
        self.mixed
            .contains(ledger)
            .then_some(SetAsideRule::MixedCurrencyMovement)
    }

    /// A named ledger that is set aside refuses, naming it and the rule, with no figure (P7: an
    /// empty list could read as "no movement").
    pub(super) fn refuse_named(&self, selected: Option<&str>) -> Result<(), ToolFailure> {
        match selected.and_then(|ledger| self.rule_of(ledger).map(|rule| (ledger, rule))) {
            Some((ledger, rule)) => Err(ToolFailure::from(MovementCurrencyCode::SetAside)
                .with_set_aside(ledger, rule.code())),
            None => Ok(()),
        }
    }

    /// The result's fields, in `trial_balance`'s words (`agent_trial_balance.rs`).
    pub(super) fn frame(&self, result: &mut serde_json::Map<String, Value>, redaction: Redaction) {
        let named = super::super::trial_balance::EXCLUDED_TRIAL_BALANCE_LEDGERS_NAMED;
        let foreign = json!({
            "count": self.foreign.len(),
            "ledgers": self
                .foreign
                .iter()
                .take(named)
                .map(|ledger| json!({
                    "ledger": party_name(ledger.ledger.clone()),
                    "currency": ledger.currency,
                }))
                .collect::<Vec<_>>(),
        });
        let mixed = json!({
            "count": self.mixed.len(),
            "reason": "mixed_currency_movement",
            "ledgers": self
                .mixed
                .iter()
                .take(named)
                .map(|ledger| party_name(ledger.clone()))
                .collect::<Vec<_>>(),
        });
        let fields = json!({
            "ledgers_scope": "base_currency_ledgers_only",
            "foreign_currency_ledgers_excluded": redact_value(foreign, redaction),
            "base_currency_ledgers_mixed_excluded": redact_value(mixed, redaction),
            "scope_limitation": BASE_CURRENCY_LEDGERS_MOVEMENT_LIMITATION,
            "scope_label": crate::tally::runtime::base_currency_ledgers_only_label(
                self.foreign.len(),
                self.mixed.len(),
            ),
        });
        if let Value::Object(fields) = fields {
            result.extend(fields);
        }
    }
}

#[cfg(test)]
#[path = "agent_movement_currency_tests.rs"]
mod tests;
