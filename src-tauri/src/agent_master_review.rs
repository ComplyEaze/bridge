//! The text the master approval dialogs show.
//!
//! For ledgers: one request of ledger `Create`s, two lines for each master in
//! the order they are sent, under the pinned company the voucher preview
//! names. For a stock item: one `Create` with each copied field and its
//! source. Values are shown JSON-quoted, so a backslash shows doubled. It is
//! built from the typed items and the verified company, never from caller
//! text, and it is refused rather than cut when it does not fit: a dialog
//! that hides part of what will be created is not a review. The limits and the
//! unsafe-character checks are the voucher preview's own.
use super::super::failure_code::{create_render_refusal_code, StockItemCreateCode};
use super::post::{
    has_unreviewable_format_character, has_unsafe_review_layout_character, BATCH_REVIEW_MAX_BYTES,
    BATCH_REVIEW_MAX_CHARS, BATCH_REVIEW_MAX_LINES, BATCH_REVIEW_MAX_LINE_CHARS,
};
use crate::tally::agent_read_request::AgentReadRequest;
use crate::tally::approved_import::{ApprovedImport, StockItemDialogRefusal};
use crate::tally::ledger_create_item::{AdmittedLedgerKind, LedgerCreateBatch, LedgerCreateItem};
use crate::tally::stock_item_create::{
    create_text_refusal, CreateRenderRefusal, MarkedConstant, PreparedStockItem, StockItemDraft,
    GOODS, SPECIFY_DETAILS_HERE, TAXABLE,
};
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

/// The preview of creating `batch` in `company`, reached at `origin`.
///
/// Refused with a typed error when any text the dialog would show holds a
/// layout or format character, or the text does not fit; never cut.
pub(super) fn master_review_text(
    batch: &LedgerCreateBatch,
    company: &VerifiedCompanyIdentity,
    origin: &str,
) -> Result<String, MasterReviewError> {
    let lines = review_lines(batch, company, origin, |item| {
        item.name().as_str().to_string()
    })?;
    if fits(&lines) {
        Ok(lines.join("\n"))
    } else {
        Err(MasterReviewError::TooLarge)
    }
}

/// The same preview with each ledger's name masked, for a tool result under
/// `mask_parties`: the dialog shows the names whole, a result never does.
///
/// Refused exactly when the dialog's own text is: the fit is judged on the
/// text with the whole names, never on the masked text, so a batch is never
/// prepared under masking that the dialog could not then show.
pub(super) fn masked_review_text(
    batch: &LedgerCreateBatch,
    company: &VerifiedCompanyIdentity,
    origin: &str,
) -> Result<String, MasterReviewError> {
    master_review_text(batch, company, origin)?;
    Ok(review_lines(batch, company, origin, |item| {
        super::super::mask(item.name().as_str())
    })?
    .join("\n"))
}

/// The preview's lines with each ledger shown as `name` gives it; the layout
/// and format checks read every ledger's real name whatever is shown. One
/// ledger keeps the words its box was measured with; a batch names its count
/// and lists each ledger in the order it is sent.
fn review_lines(
    batch: &LedgerCreateBatch,
    company: &VerifiedCompanyIdentity,
    origin: &str,
    name: impl Fn(&LedgerCreateItem) -> String,
) -> Result<Vec<String>, MasterReviewError> {
    let shown = [
        company.display_name(),
        company.company_guid(),
        company.company_number(),
        company.books_from_yyyymmdd(),
        origin,
    ]
    .into_iter()
    .chain(batch.items().iter().map(|item| item.name().as_str()))
    .collect::<Vec<_>>();
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
    let count = match batch.len() {
        1 => "ONE ledger".to_string(),
        count => format!("{count} ledgers"),
    };
    let mut lines = vec![
        format!("Create {count} in {}", quoted(company.display_name())),
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
    ];
    for item in batch.items() {
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
        lines.push(format!(
            "Create ledger {} under {}",
            quoted(&name(item)),
            item.parent()
        ));
        lines.push(format!(
            "{gst}, bill-by-bill {}",
            if bill_wise { "Yes" } else { "No" }
        ));
    }
    lines.push(String::new());
    lines.push(MASTER_REVIEW_WARNING.to_string());
    Ok(lines)
}

/// Why a stock-item preview was refused. Nothing was shown. Refusal codes for
/// these come with the dialog that shows the preview.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum StockItemReviewError {
    /// The shared review checks refused the text.
    Review(MasterReviewError),
    /// The Create renderer would refuse this draft, so it is not shown for
    /// approval.
    NotRenderable(CreateRenderRefusal),
}

impl StockItemReviewError {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn code(&self) -> StockItemCreateCode {
        match self {
            Self::Review(MasterReviewError::LayoutText) => StockItemCreateCode::ReviewLayoutText,
            Self::Review(MasterReviewError::FormatText) => StockItemCreateCode::ReviewFormatText,
            Self::Review(MasterReviewError::TooLarge) => StockItemCreateCode::ReviewTooLarge,
            Self::NotRenderable(error) => create_render_refusal_code(error),
        }
    }
}

/// The complete preview of one stock-item draft, with every copied field shown
/// and the exact row name or alias used to select its sibling named as source.
/// Every shown value first passes the layout and format checks; then the draft
/// must pass the renderer's own text rule, `create_text_refusal`.
#[allow(dead_code)]
pub(super) fn stock_item_review_text(
    draft: &StockItemDraft<'_>,
    origin: &str,
) -> Result<String, StockItemReviewError> {
    let sibling = draft.sibling().sibling();
    let company = sibling.catalogue().company();
    let source = sibling.source_name().as_str();
    let gst = draft.gst();
    let hsn = draft.hsn();
    let applicability = MarkedConstant::Applicable.render();
    let state = MarkedConstant::Any.render();
    let mut shown = vec![
        company.display_name(),
        company.company_guid(),
        company.company_number(),
        company.books_from_yyyymmdd(),
        origin,
        draft.name().as_str(),
        source,
        draft.parent(),
        draft.unit(),
        applicability,
        GOODS,
        gst.applicable_from(),
        TAXABLE,
        SPECIFY_DETAILS_HERE,
        state,
        hsn.applicable_from(),
        hsn.code(),
        hsn.description(),
    ];
    for rate in gst.rates() {
        shown.push(rate.duty_head());
        shown.push(rate.valuation().render());
        if let Some(value) = rate.rate() {
            shown.push(value);
        }
    }
    if shown
        .iter()
        .copied()
        .any(has_unsafe_review_layout_character)
    {
        return Err(StockItemReviewError::Review(MasterReviewError::LayoutText));
    }
    if shown.iter().copied().any(has_unreviewable_format_character) {
        return Err(StockItemReviewError::Review(MasterReviewError::FormatText));
    }
    create_text_refusal(draft.writes()).map_err(StockItemReviewError::NotRenderable)?;

    let quoted = |text: &str| serde_json::to_string(text).expect("string serialization");
    let mut lines = vec![
        format!(
            "Create ONE stock item in {}",
            quoted(company.display_name())
        ),
        format!("Company GUID: {}", company.company_guid()),
        format!(
            "Company number: {}  Books from: {}",
            company.company_number(),
            bridge_tally_core::TallyDate::parse(company.books_from_yyyymmdd()).map_or_else(
                |_| company.books_from_yyyymmdd().to_string(),
                |date| date.shown()
            )
        ),
        format!("Tally: {origin}"),
        String::new(),
        format!("Create stock item {}", quoted(draft.name().as_str())),
        format!(
            "All six copied fields come from sibling stock item {}.",
            quoted(source)
        ),
        format!(
            "Parent (stock group): {} — source: {}",
            quoted(draft.parent()),
            quoted(source)
        ),
        format!(
            "Base unit: {} — source: {}",
            quoted(draft.unit()),
            quoted(source)
        ),
        format!(
            "GST applicability: {} — source: {}",
            quoted(applicability),
            quoted(source)
        ),
        format!(
            "Type of supply: {} — source: {}",
            quoted(GOODS),
            quoted(source)
        ),
        format!("GST details entry 1 — source: {}", quoted(source)),
        format!("  Applicable from: {}", quoted(gst.applicable_from())),
        format!("  Taxability: {}", quoted(TAXABLE)),
        format!("  Source of GST details: {}", quoted(SPECIFY_DETAILS_HERE)),
        format!("  State row 1: {}", quoted(state)),
    ];
    for (index, rate) in gst.rates().iter().enumerate() {
        let value = rate.rate().map_or_else(|| "absent".to_string(), &quoted);
        lines.push(format!(
            "  Rate row {}: {}; valuation: {}; rate: {value}",
            index + 1,
            quoted(rate.duty_head()),
            quoted(rate.valuation().render())
        ));
    }
    lines.extend([
        "  GST slab rates: empty".to_string(),
        format!("HSN details entry 1 — source: {}", quoted(source)),
        format!("  Applicable from: {}", quoted(hsn.applicable_from())),
        format!("  HSN code: {}", quoted(hsn.code())),
        format!("  HSN description: {}", quoted(hsn.description())),
        format!("  Source of HSN details: {}", quoted(SPECIFY_DETAILS_HERE)),
        String::new(),
        "Do not create or rename stock items in Tally until this is done.".to_string(),
    ]);
    if fits(&lines) {
        Ok(lines.join("\n"))
    } else {
        Err(StockItemReviewError::Review(MasterReviewError::TooLarge))
    }
}

/// Why a stock-item draft was not confirmed. Nothing was created.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum StockItemConfirmError {
    /// The preview could not be built, so no one was asked.
    Review(StockItemReviewError),
    /// The dialog approved nothing.
    Dialog(StockItemDialogRefusal),
}

/// Shows the preview of `draft` in the stock-item dialog and, only on the
/// person's approval, returns the approval of the Create made from it. The
/// preview is built here, from the draft itself, so the window shows what this
/// draft creates.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) async fn confirm_stock_item_draft(
    draft: StockItemDraft<'_>,
    origin: &str,
    company_marks_request: AgentReadRequest,
    batch_id: String,
) -> Result<ApprovedImport, StockItemConfirmError> {
    let preview = stock_item_review_text(&draft, origin).map_err(StockItemConfirmError::Review)?;
    let prepared = PreparedStockItem::from_draft(draft).map_err(|refusal| {
        StockItemConfirmError::Review(StockItemReviewError::NotRenderable(refusal))
    })?;
    ApprovedImport::confirm_stock_item(
        prepared,
        preview,
        company_marks_request,
        super::master_post::stock_company_marks,
        batch_id,
    )
    .await
    .map_err(StockItemConfirmError::Dialog)
}

#[cfg(test)]
#[path = "agent_master_review_tests.rs"]
mod tests;
