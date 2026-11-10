//! `ledger_movement` on a book with several Currency masters, read from the book's first day only
//! (#683). Which ledgers are set aside, by which rule, and what the result says about them.
//!
//! From the first day a currency composite marks every foreign balance; from a later day a foreign
//! party balance reads as a plain rupee figure that no composite test sees
//! (TALLY_PROTOCOL_REFERENCE §8.2n). So this path is reachable only through a
//! [`SeveralCurrencyFirstDay`], which the runtime builds for no other date, and every rule here is
//! measured on one book: its result is always partial.

use super::super::failure_code::MovementCurrencyCode;
use super::*;
use crate::tally::runtime::{SeveralCurrencyFirstDay, TrialBalanceLedgerScope};
use bridge_tally_protocol::native_outstandings::ForeignCurrencyLedger;
use bridge_tally_protocol::native_trial_balance::NativeTrialBalance;

/// What a several-currency movement says about the ledgers it covers. The Trial Balance's sentence
/// speaks of totals, and the movement returns none.
pub(super) const BASE_CURRENCY_LEDGERS_MOVEMENT_LIMITATION: &str = "Only this book's plain base-currency ledgers are read. Ledgers kept in another currency, and base-currency ledgers with a value Tally shows in another currency, are left out and named, with no figure.";

/// Why a ledger is set aside. A code from a closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SetAsideRule {
    /// Rule 1: the Trial Balance names the ledger as kept in another currency.
    ForeignCurrencyLedger,
    /// Rule 1: a base-currency ledger the Trial Balance shows with a value in another currency.
    MixedCurrencyMovement,
    /// Rule 3: the ledger's opening at the first day is a composite.
    CompositeOpening,
    /// Rule 2: an entry on the ledger in the window is a composite.
    CompositeEntryInWindow,
}

impl SetAsideRule {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::ForeignCurrencyLedger => "foreign_currency_ledger",
            Self::MixedCurrencyMovement => "mixed_currency_movement",
            Self::CompositeOpening => "composite_opening",
            Self::CompositeEntryInWindow => "composite_entry_in_window",
        }
    }
}

/// The ledgers a several-currency movement sets aside, each named once, by the first rule that
/// found it. Built only from a [`SeveralCurrencyFirstDay`].
#[derive(Debug)]
pub(super) struct CurrencySetAside {
    /// Rule 1's ledgers kept in another currency, with their `CURRENCYNAME`, in read order.
    foreign: Vec<ForeignCurrencyLedger>,
    /// Every other set-aside ledger and the rule that named it.
    mixed: BTreeMap<String, SetAsideRule>,
}

impl CurrencySetAside {
    /// Rules 1 and 3, before any voucher is read. `scope` and `kept` are the currency-scoped Trial
    /// Balance of the same window (its set-aside names and its plain rows); `catalogue` is the
    /// movement's own opening export at the first day.
    ///
    /// Every name the Trial Balance sets aside must be in the catalogue, and every catalogue
    /// ledger must be in the Trial Balance: a spelling or listing difference between the two
    /// reports is unmeasured, so either miss refuses (`ledger_currency_scope_unmatched`).
    pub(super) fn before_window(
        first_day: &SeveralCurrencyFirstDay,
        scope: &TrialBalanceLedgerScope,
        kept: &NativeTrialBalance,
        catalogue: &[TallyLedger],
    ) -> Result<Self, MovementCurrencyCode> {
        let TrialBalanceLedgerScope::BaseCurrencyLedgersOnly { foreign, mixed, .. } = scope else {
            // The Trial Balance found one Currency master where the opening read found several:
            // the book changed between the two reads.
            return Err(MovementCurrencyCode::SnapshotDrifted);
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
            return Err(MovementCurrencyCode::ScopeUnmatched);
        }
        let mut set_aside = Self {
            foreign: foreign.clone(),
            mixed: BTreeMap::new(),
        };
        for name in mixed {
            set_aside.add(name, SetAsideRule::MixedCurrencyMovement);
        }
        for name in first_day.composite_openings() {
            set_aside.add(name, SetAsideRule::CompositeOpening);
        }
        Ok(set_aside)
    }

    /// Rule 2: every ledger with a composite entry in the window's vouchers.
    pub(super) fn add_composite_entries(&mut self, vouchers: &[MovementVoucher]) {
        for entry in vouchers.iter().flat_map(|voucher| &voucher.ledger_entries) {
            if entry.amount == MovementAmount::Composite {
                self.add(&entry.ledger_name, SetAsideRule::CompositeEntryInWindow);
            }
        }
    }

    fn add(&mut self, ledger: &str, rule: SetAsideRule) {
        if self.rule_of(ledger).is_none() {
            self.mixed.insert(ledger.to_string(), rule);
        }
    }

    /// The rule that set `ledger` aside, if one did.
    pub(super) fn rule_of(&self, ledger: &str) -> Option<SetAsideRule> {
        if self.foreign.iter().any(|found| found.ledger == ledger) {
            return Some(SetAsideRule::ForeignCurrencyLedger);
        }
        self.mixed.get(ledger).copied()
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
                .keys()
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
