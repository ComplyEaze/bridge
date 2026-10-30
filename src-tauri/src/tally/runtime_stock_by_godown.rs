//! One closing-stock-by-godown read (#2017), inside the identity and book-extent
//! bracket the other built-in report reads use.
//!
//! Three reports are each read twice: Tally's own Godown Summary, the plain
//! Stock Summary its total is tied to, and the Stock Summary with the explode
//! flag, the only answer that carries a value per godown below the godown's
//! own line (protocol reference §12a.24). The size is admitted as the stock
//! summary's is: the book's master mark times an assumed worst-case row,
//! against the response budget, a guess rather than a measurement of these
//! reports.
use super::stock_summary::{
    admit_stock_summary_size, check_stock_premise, stock_summary_period, StockSummaryReadError,
};
use super::*;
use bridge_tally_protocol::native_godown_summary::{
    items_by_godown, parse_native_godown_summary, render_native_godown_summary_request,
    render_native_stock_summary_exploded_request, tie_godowns, NativeGodownSummary,
    NativeGodownTie, NativeItemGodowns,
};
use bridge_tally_protocol::native_stock_summary::{
    parse_native_stock_summary_exploded, parse_native_stock_summary_report,
    render_native_stock_summary_request, NativeStockReport, StockSummaryAsOf,
};
use bridge_tally_protocol::outstandings_shared::OutstandingsError;

/// A completed observation of closing stock by godown at a date.
pub(crate) struct StockByGodownRead {
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
    pub(crate) godowns: NativeGodownSummary,
    pub(crate) tie: NativeGodownTie,
    /// The root items the exploded Stock Summary gave godown lines for, or
    /// `None` when that answer was an empty envelope: not an answer of none.
    pub(crate) items: Option<Vec<NativeItemGodowns>>,
    pub(crate) evidence: RuntimeReadEvidence,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum StockByGodownReadError {
    #[error("stock_by_godown_education_unqualified")]
    EducationUnqualified,
    #[error("stock_by_godown_as_of_before_books")]
    AsOfBeforeBooks,
    #[error("stock_by_godown_as_of_in_future")]
    AsOfInFuture,
    #[error("stock_by_godown_period_not_honoured")]
    PeriodNotHonoured,
    /// The master mark, times an assumed worst-case row, is over the response
    /// budget, so no report was requested.
    #[error("stock_by_godown_too_large")]
    TooLarge {
        master_alter_id: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
        limit_master_alter_id: u64,
    },
    /// A report came back and broke what its size was admitted on.
    #[error("stock_by_godown_bound_premise_violated")]
    PremiseViolated(&'static str),
}

impl StockByGodownReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "stock_by_godown_education_unqualified",
            Self::AsOfBeforeBooks => "stock_by_godown_as_of_before_books",
            Self::AsOfInFuture => "stock_by_godown_as_of_in_future",
            Self::PeriodNotHonoured => "stock_by_godown_period_not_honoured",
            Self::TooLarge { .. } => "stock_by_godown_too_large",
            Self::PremiseViolated(_) => "stock_by_godown_bound_premise_violated",
        }
    }
}

/// The stock summary's refusals, under this read's codes.
fn own(error: StockSummaryReadError) -> StockByGodownReadError {
    match error {
        StockSummaryReadError::AsOfBeforeBooks => StockByGodownReadError::AsOfBeforeBooks,
        StockSummaryReadError::AsOfInFuture => StockByGodownReadError::AsOfInFuture,
        StockSummaryReadError::TooLarge {
            master_alter_id,
            estimated_bytes,
            limit_bytes,
            limit_master_alter_id,
        } => StockByGodownReadError::TooLarge {
            master_alter_id,
            estimated_bytes,
            limit_bytes,
            limit_master_alter_id,
        },
        StockSummaryReadError::PremiseViolated(reason) => {
            StockByGodownReadError::PremiseViolated(reason)
        }
        StockSummaryReadError::EducationUnqualified => StockByGodownReadError::EducationUnqualified,
        StockSummaryReadError::PeriodNotHonoured => StockByGodownReadError::PeriodNotHonoured,
        // Raised by the stock summary's flags read, which this read does not send.
        StockSummaryReadError::NotEnabled => {
            StockByGodownReadError::PremiseViolated("stock_by_godown_refusal_unexpected")
        }
    }
}

/// The lines a report holds, for the premise that the mark bounds them. The
/// exploded report's godown sub-lines are not counted: one item has one per
/// godown, which the mark does not bound. Its size is still checked.
fn report_lines(report: &NativeStockReport) -> usize {
    match report {
        NativeStockReport::Lines { lines, .. } => lines.len(),
        NativeStockReport::Empty => 0,
    }
}

impl TallyRuntime {
    /// Closing stock by godown at `as_of`, with the same queue, mode and
    /// identity brackets as the other report reads: the book extent before and
    /// after must be equal and each report's two paired reads identical, or the
    /// read refuses. Each report is one paired read (two requests), never
    /// retried. A refusal of what an answer broke is held until the closing
    /// extent is read, so a book that moved mid-read is reported as moved.
    pub(crate) async fn fetch_stock_by_godown(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        as_of: StockSummaryAsOf,
        today: TallyDate,
    ) -> anyhow::Result<StockByGodownRead> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, move |client| {
            let identity = identity.clone();
            let as_of = as_of.clone();
            let today = today.clone();
            async move {
                let mut evidence = RuntimeReadEvidence::empty();
                let result = async {
                    let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                    evidence = mode_evidence;
                    if profile == DateBoundaryProfile::EducationRestricted {
                        return Err(StockByGodownReadError::EducationUnqualified.into());
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    let period = stock_summary_period(profile, &as_of, extent.books_from(), &today)
                        .map_err(own)?;
                    let mark = extent
                        .master_alter_id_high_water()
                        .ok_or(OutstandingsError::MasterWitnessAbsent)?
                        .get();
                    let estimate = admit_stock_summary_size(mark).map_err(own)?;
                    let company = identity.display_name();

                    let mut answers = Vec::with_capacity(3);
                    for (request, drift) in [
                        (
                            render_native_godown_summary_request(company, &period),
                            PairedReadValidationError::NativeGodownSummary,
                        ),
                        (
                            render_native_stock_summary_request(company, &period),
                            PairedReadValidationError::NativeStatement,
                        ),
                        (
                            render_native_stock_summary_exploded_request(company, &period),
                            PairedReadValidationError::NativeStockSummaryExploded,
                        ),
                    ] {
                        let (xml, bytes, hash) = client
                            .fetch_native_report_paired(request.clone())
                            .await?
                            .require_stable(drift)?;
                        evidence = evidence
                            .clone()
                            .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                        answers.push((xml, bytes));
                    }

                    // Held until the closing extent is read.
                    let parsed =
                        (|| {
                            let godowns = parse_native_godown_summary(&answers[0].0)?;
                            let plain = parse_native_stock_summary_report(&answers[1].0)?;
                            let exploded = parse_native_stock_summary_exploded(&answers[2].0)?;
                            let items = exploded
                                .as_deref()
                                .map(|lines| items_by_godown(lines, &plain))
                                .transpose()?;
                            let exploded_lines = exploded.as_ref().map_or(0, Vec::len);
                            Ok::<_, bridge_tally_protocol::native_stock_summary::NativeStockError>(
                                (godowns, plain, items, exploded_lines),
                            )
                        })();

                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::StockByGodownExtent.into());
                    }
                    let (godowns, plain, items, exploded_lines) = parsed?;
                    let godown_lines = match &godowns {
                        NativeGodownSummary::Lines { lines, .. } => lines.len(),
                        NativeGodownSummary::Empty => 0,
                    };
                    for (rows, (_, bytes)) in [godown_lines, report_lines(&plain), exploded_lines]
                        .into_iter()
                        .zip(&answers)
                    {
                        check_stock_premise(rows, mark, *bytes, estimate).map_err(own)?;
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    evidence = evidence
                        .clone()
                        .combine(confirm_read_boundary(&client, profile).await?);
                    Ok(StockByGodownRead {
                        from: period.from().clone(),
                        to: period.to().clone(),
                        tie: tie_godowns(&godowns, &plain),
                        godowns,
                        items,
                        evidence: evidence.clone(),
                    })
                }
                .await;
                result.map_err(|error| with_read_evidence(error, evidence))
            }
        })
        .await
    }
}

#[cfg(test)]
#[path = "runtime_stock_by_godown_tests.rs"]
mod tests;
