//! The text the one-ledger dialog shows.
//!
//! One ledger `Create`, two lines for the master, under the pinned company the
//! voucher preview names. It is built from the typed item and the verified
//! company, never from caller text, and it is refused rather than cut when it
//! does not fit: a dialog that hides part of what will be created is not a
//! review. The limits and the unsafe-character checks are the voucher
//! preview's own.
use super::post::{
    has_unreviewable_format_character, has_unsafe_review_layout_character, BATCH_REVIEW_MAX_BYTES,
    BATCH_REVIEW_MAX_CHARS, BATCH_REVIEW_MAX_LINES, BATCH_REVIEW_MAX_LINE_CHARS,
};
use crate::tally::ledger_create_item::{AdmittedLedgerKind, LedgerCreateItem};
use crate::tally::VerifiedCompanyIdentity;

/// The last line of the preview: what the person must not do while the post
/// runs.
pub(super) const MASTER_REVIEW_WARNING: &str =
    "Do not create or rename ledgers in Tally until this is done.";

/// Why a one-ledger preview was refused. Nothing was shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MasterReviewError {
    /// A line break or other control character in text the dialog would show.
    LayoutText,
    /// A format or default-ignorable character that would hide a difference
    /// between two exact names.
    FormatText,
    /// The text is over a limit of lines, characters, characters a line, or
    /// bytes.
    TooLarge,
}

impl MasterReviewError {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::LayoutText => "import_review_layout_text",
            Self::FormatText => "import_review_format_text",
            Self::TooLarge => "master_review_too_large",
        }
    }
}

/// Whether `lines` fit the dialog: the batch preview's limits, held in one
/// place.
fn fits(lines: &[String]) -> bool {
    let text = lines.join("\n");
    lines.len() <= BATCH_REVIEW_MAX_LINES
        && text.chars().count() <= BATCH_REVIEW_MAX_CHARS
        && text.len() <= BATCH_REVIEW_MAX_BYTES
        && lines
            .iter()
            .all(|line| line.chars().count() <= BATCH_REVIEW_MAX_LINE_CHARS)
}

/// The preview of creating `item` in `company`, reached at `origin`.
///
/// Refused with a typed error when any text the dialog would show holds a
/// layout or format character, or the text does not fit; never cut.
pub(super) fn master_review_text(
    item: &LedgerCreateItem,
    company: &VerifiedCompanyIdentity,
    origin: &str,
) -> Result<String, MasterReviewError> {
    let lines = review_lines(item, company, origin, item.name().as_str())?;
    if fits(&lines) {
        Ok(lines.join("\n"))
    } else {
        Err(MasterReviewError::TooLarge)
    }
}

/// The same preview with the ledger's name masked, for a tool result under
/// `mask_parties`: the dialog shows the name whole, a result never does.
///
/// Refused exactly when the dialog's own text is: the fit is judged on the
/// text with the whole name, never on the masked text, so a batch is never
/// prepared under masking that the dialog could not then show.
pub(super) fn masked_review_text(
    item: &LedgerCreateItem,
    company: &VerifiedCompanyIdentity,
    origin: &str,
) -> Result<String, MasterReviewError> {
    master_review_text(item, company, origin)?;
    let masked = super::super::mask(item.name().as_str());
    Ok(review_lines(item, company, origin, &masked)?.join("\n"))
}

/// The preview's lines with the ledger shown as `name`; the layout and format
/// checks read the ledger's real name whatever is shown.
fn review_lines(
    item: &LedgerCreateItem,
    company: &VerifiedCompanyIdentity,
    origin: &str,
    name: &str,
) -> Result<Vec<String>, MasterReviewError> {
    let shown = [
        company.display_name(),
        company.company_guid(),
        company.company_number(),
        company.books_from_yyyymmdd(),
        origin,
        item.name().as_str(),
    ];
    if shown
        .iter()
        .copied()
        .any(has_unsafe_review_layout_character)
    {
        return Err(MasterReviewError::LayoutText);
    }
    if shown.iter().copied().any(has_unreviewable_format_character) {
        return Err(MasterReviewError::FormatText);
    }
    let quoted = |text: &str| serde_json::to_string(text).expect("string serialization");
    let (gst, bill_wise) = match item.kind() {
        AdmittedLedgerKind::Party {
            gstin, bill_wise, ..
        } => (
            gstin
                .as_ref()
                .map_or_else(|| "no GST".to_string(), |g| format!("GSTIN {}", g.as_str())),
            *bill_wise,
        ),
        AdmittedLedgerKind::Suspense => ("no GST".to_string(), false),
        AdmittedLedgerKind::GstDuty { head } => {
            (format!("duty head {}", head.head().tally_spelling()), false)
        }
    };
    Ok(vec![
        format!("Create ONE ledger in {}", quoted(company.display_name())),
        format!("Company GUID: {}", company.company_guid()),
        format!(
            "Company number: {}  Books from: {}",
            company.company_number(),
            // In Tally's day-month-year form, as the approval boxes show it
            // (#2101).
            bridge_tally_core::TallyDate::parse(company.books_from_yyyymmdd()).map_or_else(
                |_| company.books_from_yyyymmdd().to_string(),
                |date| date.shown()
            )
        ),
        format!("Tally: {origin}"),
        String::new(),
        format!("Create ledger {} under {}", quoted(name), item.parent()),
        format!(
            "{gst}, bill-by-bill {}",
            if bill_wise { "Yes" } else { "No" }
        ),
        String::new(),
        MASTER_REVIEW_WARNING.to_string(),
    ])
}

#[cfg(test)]
#[path = "agent_master_review_tests.rs"]
mod tests;
