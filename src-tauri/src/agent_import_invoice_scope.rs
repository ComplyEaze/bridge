//! The ledgers a Sales invoice names, and the one filter its ledger-rate read
//! is restricted to (#1331).
//!
//! The rate read used to ask for every ledger of the book at each of an
//! invoice's checks, and then for every ledger under the parents of the named
//! ones. A customer's parent can hold thousands of ledgers, so the read now
//! names the invoice's ledgers themselves, by the GUID the catalogue gave each,
//! and believes the answer only when its rows are exactly those ledgers.
//!
//! The rules are a type, not reminders (AGENTS.md P2, P3): an
//! [`InvoiceLedgerScope`] can only be made by [`InvoiceLedgerScope::from_catalogue`],
//! from the catalogue the check has just read, so no request names a ledger
//! the catalogue does not, and its only way to accept an answer is
//! [`InvoiceLedgerScope::prove`].
//!
//! Matching is exact on the catalogue's row spelling, as the import's own
//! binding is (no case fold, no alias, no stored name). After a ledger is
//! resolved everything is by GUID, whose alphabet is closed, so the filter
//! holds no text the invoice supplied.

use super::wire::ListedLedger;
use super::{refuse_ledger, refuse_value, InvoiceRefusal};
use bridge_tally_protocol::ledger_guid_part::{LedgerGuid, LedgerGuidPart};
use bridge_tally_protocol::parent_partition::ParentObservation;
use bridge_tally_protocol::StandardLedgerCatalog;
use std::collections::{BTreeMap, BTreeSet};

/// One ledger the scope names, as the catalogue gave it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ScopedLedger {
    name: String,
    parent: String,
}

/// The ledgers one invoice names, by GUID, and what the catalogue says each is
/// called and grouped under.
pub(super) struct InvoiceLedgerScope {
    part: LedgerGuidPart,
    /// By lower-case GUID.
    expected: BTreeMap<String, ScopedLedger>,
}

impl InvoiceLedgerScope {
    /// Resolves every name in `named` to one catalogue ledger. Refused before
    /// any request:
    /// - `invoice_ledger_not_observed`: the catalogue holds no ledger so spelled;
    /// - `invoice_ledger_guid_unnameable`: the ledger's GUID is not of the closed
    ///   alphabet a filter takes;
    /// - `invoice_ledger_parent_unnameable`: the ledger has no parent, or one
    ///   the catalogue could not carry, so its row cannot be proved;
    /// - `invoice_ledger_scope_unplannable`: the part cannot be made.
    pub(super) fn from_catalogue(
        catalogue: &StandardLedgerCatalog,
        named: &[&str],
    ) -> Result<Self, InvoiceRefusal> {
        let rows = catalogue.identified_parents().collect::<Vec<_>>();
        let mut guids = Vec::new();
        let mut expected = BTreeMap::new();
        for name in named.iter().copied().collect::<BTreeSet<_>>() {
            // The catalogue's parse refuses a repeated spelling, so a name
            // resolves to one ledger or to none.
            let Some((_, guid, parent)) = rows.iter().find(|(row, _, _)| *row == name) else {
                return Err(refuse_ledger("invoice_ledger_not_observed", name));
            };
            let guid = LedgerGuid::parse(guid)
                .map_err(|_| refuse_ledger("invoice_ledger_guid_unnameable", name))?;
            let ParentObservation::Named(parent) = parent else {
                return Err(refuse_ledger("invoice_ledger_parent_unnameable", name));
            };
            expected.insert(
                guid.as_str().to_ascii_lowercase(),
                ScopedLedger {
                    name: name.to_string(),
                    parent: (*parent).to_string(),
                },
            );
            guids.push(guid);
        }
        let part = LedgerGuidPart::new(guids)
            .map_err(|error| refuse_value("invoice_ledger_scope_unplannable", error.code()))?;
        Ok(Self { part, expected })
    }

    /// The part the rate request is restricted to.
    pub(super) fn part(&self) -> &LedgerGuidPart {
        &self.part
    }

    /// Each scoped ledger's GUID and catalogue parent, in GUID order: what the
    /// plan carries so that a ledger regrouped between the plan and the queue
    /// is told apart, though its GUID, and so its request, is unchanged.
    pub(super) fn parents(&self) -> Vec<(String, String)> {
        self.expected
            .iter()
            .map(|(guid, ledger)| (guid.clone(), ledger.parent.clone()))
            .collect()
    }

    /// Whether `listed`, the rows an answer to the scoped request holds, are
    /// exactly the scope's ledgers: the count first, then each row by GUID,
    /// name and parent, none twice, none left out. Anything else is
    /// `invoice_ledger_rates_rows_differ`, naming which way: a filter Tally did
    /// not apply, a ledger that moved or was renamed, or a read that lost a row.
    pub(super) fn prove(&self, listed: &[ListedLedger]) -> Result<(), InvoiceRefusal> {
        let differs = |cause: &str| refuse_value("invoice_ledger_rates_rows_differ", cause);
        if listed.len() != self.expected.len() {
            return Err(differs("ledger_scope_row_count_differs"));
        }
        let mut seen = BTreeSet::new();
        for row in listed {
            let guid = row
                .guid
                .as_deref()
                .ok_or_else(|| differs("ledger_scope_row_without_guid"))?
                .to_ascii_lowercase();
            let Some(expected) = self.expected.get(&guid) else {
                return Err(differs("ledger_scope_row_not_named"));
            };
            if !seen.insert(guid) {
                return Err(differs("ledger_scope_row_repeated"));
            }
            if row.name != expected.name || row.parent.as_deref() != Some(expected.parent.as_str())
            {
                return Err(differs("ledger_scope_row_differs_from_catalogue"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "agent_import_invoice_scope_tests.rs"]
mod tests;
