//! Exact source model for the columnar party and ledger master workbook.
//!
//! This module performs no Tally I/O. Its source is constructed only after
//! paired, company-bracketed reads have established every row and its period.

use std::collections::BTreeSet;

use bridge_tally_core::{ExactDecimal, TallyDate};
use bridge_tally_protocol::{
    PartyLedgerMasterFieldObservation, PartyLedgerMasterFields, TallyNamedMaster,
};
use serde::Serialize;

use crate::tally::OutstandingsCurrencyAssertion;

/// Excel preserves at most 15 significant digits. The workbook refuses a
/// currency precision beyond that documented representation boundary rather
/// than quietly selecting a two-decimal format.
pub(crate) const MAX_RENDERABLE_CURRENCY_DECIMAL_PLACES: u8 = 15;

/// Which date ends the workbook's balance snapshot. Only a date the CA typed
/// can be refused for preceding the books or reported as moved (#903); the
/// host's date keeps the rule it had (#875).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BalanceDateBasis {
    /// The Bridge host's calendar date, the default.
    HostToday(TallyDate),
    /// The date the CA chose on the Outstandings screen.
    Requested(TallyDate),
}

/// What a note says, so the app and the tests read the kind and not the words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceNoteKind {
    /// A voucher is dated after the balance snapshot's end.
    LaterDatedVoucher,
    /// The typed date was moved to the next date Tally accepts as a boundary.
    DateMoved,
    /// The mailing details were asked for, that read failed, and the book was
    /// read again without them (bridge#2214).
    MailingDetailsReadFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceNote {
    pub kind: SourceNoteKind,
    pub text: String,
}

#[derive(Debug, Clone)]
pub(crate) struct PartyLedgerMasterSource {
    pub(crate) company: String,
    pub(crate) company_guid: String,
    /// Money in this workbook may be rendered only after the backend's
    /// existing Tally currency probe established this assertion.
    pub(crate) currency_assertion: OutstandingsCurrencyAssertion,
    /// Tally's observed base-currency display precision. This remains data,
    /// not a renderer default, from the existing currency probe to the XLSX.
    pub(crate) currency_decimal_places: u8,
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
    /// What `to` was chosen from.
    pub(crate) balance_date_basis: BalanceDateBasis,
    /// The extent's last voucher date when the read was pinned. It is after
    /// `to` only when the balance snapshot was ended at the host's today, or at
    /// the start of the books when the host's date is earlier, because the book
    /// holds a later-dated voucher (#875), or at a date the CA typed (#903).
    pub(crate) last_voucher_date: TallyDate,
    pub(crate) rows: Vec<PartyLedgerMasterRow>,
    /// Ordered master/balance/group request body commitments (UTF-16LE on wire).
    pub(crate) request_sha256: String,
    pub(crate) master_response_sha256: String,
    pub(crate) balance_response_sha256: String,
    pub(crate) group_response_sha256: String,
    pub(crate) master_response_bytes: usize,
    pub(crate) balance_response_bytes: usize,
    pub(crate) group_response_bytes: usize,
    /// Native group rows captured in the same company-bracketed read as the
    /// ledger rows. Schedule III classification remains a pure derivation of
    /// this source; it never performs an independent reader call.
    pub(crate) groups: Vec<TallyNamedMaster>,
    /// Ledgers kept in a currency other than the base, left out of `rows`
    /// with their master and balance rows (bridge#551). Empty on a book with
    /// one Currency master, whose base refuses such a ledger instead.
    pub(crate) foreign_currency_ledgers_excluded:
        Vec<bridge_tally_protocol::native_outstandings::ForeignCurrencyLedger>,
    /// Base-currency ledgers left out of `rows` because a balance of theirs
    /// is a currency composite (bridge#551). Empty on a book with one master.
    pub(crate) mixed_currency_ledgers_excluded: Vec<String>,
    /// Whether the census that counted a book whose mark is past its catalogue
    /// was checked against the company's own ledger count (#938); `None` when
    /// no census ran.
    pub(crate) count_cross_check: Option<crate::tally::connection::CountCrossCheck>,
    /// How the read that asked for the mailing details failed, where the book
    /// was then read again without them (bridge#2214); `None` for every other
    /// read. Only that retry sets it.
    pub(crate) mailing_details_read_failed: Option<crate::tally::connection::MailingReadFailure>,
}

impl PartyLedgerMasterSource {
    /// Said in the workbook only when a voucher is dated after the balance
    /// snapshot's end, so a workbook read as "everything in the book" is not.
    /// It claims no count and no cause: a later date may be legitimate
    /// (post-dated entries) or a typing error.
    pub(crate) fn later_dated_vouchers_note(&self) -> Option<String> {
        (self.last_voucher_date > self.to).then(|| match &self.balance_date_basis {
            BalanceDateBasis::HostToday(_) => format!(
                "Vouchers dated after {} (the book's last voucher date is {}) are not included in these balances. Review them in Tally before relying on this workbook. The balances end at the first month boundary (the 1st, 2nd or 31st of a month) on or after this computer's date, or on or after the start of the books if that is later; if this computer's date is wrong, correct it and export again.",
                self.to.as_str(),
                self.last_voucher_date.as_str()
            ),
            BalanceDateBasis::Requested(requested) => format!(
                "Vouchers dated after {} (the book's last voucher date is {}) are not included in these balances. Review them in Tally before relying on this workbook. The balances end at the first month boundary (the 1st, 2nd or 31st of a month) on or after the date chosen for this export, {}.",
                self.to.as_str(),
                self.last_voucher_date.as_str(),
                requested.as_str()
            ),
        })
    }

    /// Said in the workbook when the date the CA typed is not a date Tally
    /// accepts as the end of a balance period, so "as of 15 August" is not
    /// read as balances to 15 August.
    pub(crate) fn date_moved_note(&self) -> Option<String> {
        match &self.balance_date_basis {
            BalanceDateBasis::Requested(requested) if requested != &self.to => Some(format!(
                "The date chosen for this export, {}, is not a date Tally accepts as the end of a balance period. The balances end at the next date it accepts, {} (the 1st, 2nd or 31st of a month), and include the vouchers dated {}.",
                requested.as_str(),
                self.to.as_str(),
                self.to.as_str()
            )),
            _ => None,
        }
    }

    /// Said in the workbook when the read that asked for the mailing details
    /// failed and the book was read again without them (bridge#2214), in place
    /// of the note that none were observed: the columns are empty because that
    /// read failed, not because Tally returned none.
    pub(crate) fn mailing_details_read_failed_note(&self) -> Option<String> {
        use crate::tally::connection::MailingReadFailure;
        let why = match self.mailing_details_read_failed? {
            MailingReadFailure::ResponseTooLarge { .. } => {
                "Tally's answer to that read passed the 32 MiB limit"
            }
            MailingReadFailure::Truncated => {
                "Tally's answer to that read broke off before it ended"
            }
            MailingReadFailure::ReadFailed => "Tally's answer to that read could not be read",
            // Never read again: such a read fails the export instead.
            MailingReadFailure::Deadline => return None,
        };
        Some(format!("The mailing details were asked for, and {why}, so this workbook was read again without them. The Mailing columns are empty and their status says not observed: that is not the same as a party having no mailing address. Read the address in Tally."))
    }

    /// The notes of this source, the moved date first.
    pub(crate) fn notes(&self) -> Vec<SourceNote> {
        [
            (SourceNoteKind::DateMoved, self.date_moved_note()),
            (
                SourceNoteKind::LaterDatedVoucher,
                self.later_dated_vouchers_note(),
            ),
            (
                SourceNoteKind::MailingDetailsReadFailed,
                self.mailing_details_read_failed_note(),
            ),
        ]
        .into_iter()
        .filter_map(|(kind, text)| text.map(|text| SourceNote { kind, text }))
        .collect()
    }

    /// The notes as the workbook says them, in one string.
    pub(crate) fn notes_text(&self) -> Option<String> {
        let texts = self
            .notes()
            .into_iter()
            .map(|note| note.text)
            .collect::<Vec<_>>();
        (!texts.is_empty()).then(|| texts.join(" "))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PartyLedgerMasterRow {
    pub(crate) name: String,
    pub(crate) parent: PartyLedgerMasterFieldObservation,
    pub(crate) party_gstin: PartyLedgerMasterFieldObservation,
    pub(crate) fields: PartyLedgerMasterFields,
    pub(crate) guid: String,
    pub(crate) master_id: String,
    pub(crate) alter_id: String,
    pub(crate) opening_balance: ExactDecimal,
    /// `None` is an empty Tally `CLOSINGBALANCE`, not a zero balance.
    pub(crate) closing_balance: Option<ExactDecimal>,
}

#[derive(Debug, Clone)]
pub(crate) struct PartyLedgerMasterWorkbook {
    source: PartyLedgerMasterSource,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(crate) enum PartyLedgerMasterError {
    #[error("party/ledger master source omitted company identity")]
    MissingCompanyIdentity,
    #[error("party/ledger master source omitted a row identity")]
    MissingRowIdentity,
    #[error("party/ledger master source repeated a GUID")]
    DuplicateGuid,
    #[error("party/ledger master source repeated a master ID")]
    DuplicateMasterId,
    #[error("party/ledger master source omitted a source response commitment")]
    MissingResponseCommitment,
    #[error("party/ledger master currency precision cannot be rendered safely ({0})")]
    UnrenderableCurrencyPrecision(u8),
}

pub(crate) fn build_party_ledger_master_workbook(
    source: PartyLedgerMasterSource,
) -> Result<PartyLedgerMasterWorkbook, PartyLedgerMasterError> {
    if source.company.trim().is_empty() || source.company_guid.trim().is_empty() {
        return Err(PartyLedgerMasterError::MissingCompanyIdentity);
    }
    if !sha256(&source.master_response_sha256)
        || !sha256(&source.balance_response_sha256)
        || !sha256(&source.group_response_sha256)
    {
        return Err(PartyLedgerMasterError::MissingResponseCommitment);
    }
    if source.currency_decimal_places > MAX_RENDERABLE_CURRENCY_DECIMAL_PLACES {
        return Err(PartyLedgerMasterError::UnrenderableCurrencyPrecision(
            source.currency_decimal_places,
        ));
    }

    let mut guids = BTreeSet::new();
    let mut master_ids = BTreeSet::new();
    for row in &source.rows {
        if row.name.trim().is_empty()
            || row.guid.trim().is_empty()
            || row.master_id.trim().is_empty()
        {
            return Err(PartyLedgerMasterError::MissingRowIdentity);
        }
        if !guids.insert(row.guid.to_ascii_lowercase()) {
            return Err(PartyLedgerMasterError::DuplicateGuid);
        }
        if !master_ids.insert(row.master_id.clone()) {
            return Err(PartyLedgerMasterError::DuplicateMasterId);
        }
    }

    Ok(PartyLedgerMasterWorkbook { source })
}

impl PartyLedgerMasterWorkbook {
    pub(crate) fn source(&self) -> &PartyLedgerMasterSource {
        &self.source
    }
}

fn sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
#[path = "party_ledger_master_tests.rs"]
mod tests;
