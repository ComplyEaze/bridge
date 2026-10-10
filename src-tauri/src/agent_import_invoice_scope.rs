//! The ledgers a Sales invoice names, and the one parent-group filter its
//! ledger-rate read is restricted to (#1331).
//!
//! The rate read used to ask for every ledger of the book at each of an
//! invoice's three checks. It now asks for the ledgers under the parents of the
//! ledgers the invoice names, and believes the answer only when its rows are
//! exactly the ones the catalogue holds under those parents.
//!
//! The rules are a type, not reminders (AGENTS.md P2, P3): an
//! [`InvoiceLedgerScope`] can only be made by [`InvoiceLedgerScope::from_catalogue`],
//! from the catalogue the check has just read, so no request names a parent
//! the catalogue does not, and its only way to accept an answer is
//! [`InvoiceLedgerScope::prove`].
//!
//! Matching is exact on the catalogue's row spelling, as the import's own
//! binding is (no case fold, no alias, no stored name). After a ledger is
//! resolved everything is by GUID.

use super::wire::ListedLedger;
use super::{refuse_ledger, refuse_value, InvoiceRefusal};
use bridge_tally_protocol::parent_partition::{
    ParentName, ParentObservation, ParentPart, ParentPartition, ParentPartitionError,
    PartitionCoverage,
};
use bridge_tally_protocol::StandardLedgerCatalog;
use std::collections::BTreeSet;

/// The parents of the ledgers one invoice names, planned as one part, and the
/// proof that an answer holds exactly the catalogue's ledgers under them.
pub(super) struct InvoiceLedgerScope {
    part: ParentPart,
    coverage: PartitionCoverage,
}

impl InvoiceLedgerScope {
    /// Resolves every name in `named` to one catalogue ledger and plans a read
    /// of the ledgers under their parents. Refused before any request:
    /// - `invoice_ledger_not_observed`: the catalogue holds no ledger so spelled;
    /// - `invoice_ledger_parent_unnameable`: the ledger has no parent, or one
    ///   that cannot be written inside a filter;
    /// - `invoice_scope_parent_folds`: another parent of the book differs from
    ///   one of these only in letter case. Tally's `$Parent` folds case
    ///   (reference 11e), so a filter naming one would return both;
    /// - `invoice_ledger_scope_too_large`: the ledgers under these parents do
    ///   not fit one read.
    pub(super) fn from_catalogue(
        catalogue: &StandardLedgerCatalog,
        named: &[&str],
    ) -> Result<Self, InvoiceRefusal> {
        let rows = catalogue.identified_parents().collect::<Vec<_>>();
        let mut parents = BTreeSet::new();
        for name in named.iter().copied().collect::<BTreeSet<_>>() {
            // The catalogue's parse refuses a repeated spelling, so a name
            // resolves to one ledger or to none.
            let Some((_, _, parent)) = rows.iter().find(|(row, _, _)| *row == name) else {
                return Err(refuse_ledger("invoice_ledger_not_observed", name));
            };
            match parent {
                ParentObservation::Named(text) if ParentName::parse(text).is_ok() => {
                    parents.insert(*text);
                }
                _ => return Err(refuse_ledger("invoice_ledger_parent_unnameable", name)),
            }
        }
        for (_, _, parent) in &rows {
            let ParentObservation::Named(text) = parent else {
                continue;
            };
            if parents.contains(text) {
                continue;
            }
            if let Some(scoped) = parents
                .iter()
                .find(|scoped| scoped.to_lowercase() == text.to_lowercase())
            {
                return Err(refuse_value("invoice_scope_parent_folds", *scoped));
            }
        }
        let planned = ParentPartition::plan(
            rows.iter()
                .filter(|(_, _, parent)| {
                    matches!(parent, ParentObservation::Named(text) if parents.contains(text))
                })
                .copied(),
            crate::tally::connection::parent_partition_limits(),
        )
        .map_err(|error| match error {
            ParentPartitionError::ParentOverBudget { .. }
            | ParentPartitionError::TooManyParts { .. } => {
                refuse_value("invoice_ledger_scope_too_large", error.safe_code())
            }
            other => refuse_value("invoice_ledger_scope_unplannable", other.safe_code()),
        })?;
        let [part] = planned.parts() else {
            return Err(refuse_value(
                "invoice_ledger_scope_too_large",
                format!("{} parts", planned.parts().len()),
            ));
        };
        Ok(Self {
            part: part.clone(),
            coverage: planned.coverage(),
        })
    }

    /// The part the rate request is restricted to.
    pub(super) fn part(&self) -> &ParentPart {
        &self.part
    }

    /// Whether `listed`, the rows an answer to the scoped request holds, are
    /// exactly the catalogue's ledgers under the scope's parents: the count
    /// first, then each row by GUID, name and parent, none twice, none left
    /// out. Anything else is `invoice_ledger_rates_rows_differ`, naming which
    /// way (`parent_part_row_count_differs` and its siblings): a filter Tally
    /// did not apply, a ledger that moved, or a read that lost a row.
    pub(super) fn prove(&self, listed: &[ListedLedger]) -> Result<(), InvoiceRefusal> {
        let differs = |error: ParentPartitionError| {
            refuse_value("invoice_ledger_rates_rows_differ", error.safe_code())
        };
        self.part.check_row_count(listed.len()).map_err(differs)?;
        let mut coverage = self.coverage.clone();
        for row in listed {
            let guid = row.guid.as_deref().ok_or_else(|| {
                refuse_value("invoice_ledger_rates_rows_differ", "row_without_guid")
            })?;
            coverage
                .accept(0, guid, &row.name, row.parent.as_deref())
                .map_err(differs)?;
        }
        coverage.finish().map_err(differs)
    }
}

#[cfg(test)]
#[path = "agent_import_invoice_scope_tests.rs"]
mod tests;
