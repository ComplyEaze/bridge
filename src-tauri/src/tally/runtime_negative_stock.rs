//! One native Negative Stock read, inside the identity and book-extent bracket
//! the other built-in report reads use (#1358).
//!
//! The report is Tally's own Negative Stock list, returned as it is: the rule by
//! which Tally lists an item is Tally's and is not derived here, so this read
//! compares the list with no other source. It is bound only by the company, mode
//! and extent checks around it, and sized the way the stock summary is: the
//! book's master mark times an assumed worst-case row, against the response
//! budget. That ceiling is a guess, not a measurement of this report.
use super::stock_summary::{admit_stock_summary_size, check_stock_premise, StockSummaryReadError};
use super::*;
use bridge_tally_protocol::native_negative_stock::{
    parse_native_negative_stock, render_native_negative_stock_request, NativeNegativeStockError,
    NativeNegativeStockListing,
};
use bridge_tally_protocol::outstandings_shared::OutstandingsError;

/// What Tally answered. An empty envelope is its own case, never an empty
/// list: it cannot be told from a report that was not rendered (§12a.11), so it
/// says nothing about whether an item is negative.
pub(crate) enum NegativeStockAnswer {
    Listed(NativeNegativeStockListing),
    EmptyEnvelope,
}

/// A completed observation of Tally's Negative Stock for a window.
pub(crate) struct NegativeStockRead {
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
    pub(crate) answer: NegativeStockAnswer,
    pub(crate) evidence: RuntimeReadEvidence,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum NegativeStockReadError {
    #[error("negative_stock_education_unqualified")]
    EducationUnqualified,
    #[error("negative_stock_before_books")]
    BeforeBooks,
    #[error("negative_stock_period_not_honoured")]
    Period(bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriodError),
    /// The master mark, times an assumed worst-case row, is over the response
    /// budget, so no report was requested.
    #[error("negative_stock_too_large")]
    TooLarge {
        master_alter_id: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
        limit_master_alter_id: u64,
    },
    /// The report came back and broke what its size was admitted on.
    #[error("negative_stock_bound_premise_violated")]
    PremiseViolated(&'static str),
}

impl NegativeStockReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "negative_stock_education_unqualified",
            Self::BeforeBooks => "negative_stock_before_books",
            Self::Period(_) => "negative_stock_period_not_honoured",
            Self::TooLarge { .. } => "negative_stock_too_large",
            Self::PremiseViolated(_) => "negative_stock_bound_premise_violated",
        }
    }
}

/// The stock summary's admission, with a refusal of this read's own.
fn admit_size(master_alter_id: u64) -> Result<u64, NegativeStockReadError> {
    admit_stock_summary_size(master_alter_id).map_err(|error| match error {
        StockSummaryReadError::TooLarge {
            master_alter_id,
            estimated_bytes,
            limit_bytes,
            limit_master_alter_id,
        } => NegativeStockReadError::TooLarge {
            master_alter_id,
            estimated_bytes,
            limit_bytes,
            limit_master_alter_id,
        },
        _ => NegativeStockReadError::PremiseViolated("negative_stock_size_not_admitted"),
    })
}

fn check_premise(
    rows: usize,
    master_alter_id: u64,
    received_bytes: usize,
    estimate_bytes: u64,
) -> Result<(), NegativeStockReadError> {
    check_stock_premise(rows, master_alter_id, received_bytes, estimate_bytes).map_err(|error| {
        match error {
            StockSummaryReadError::PremiseViolated(reason) => {
                NegativeStockReadError::PremiseViolated(reason)
            }
            _ => NegativeStockReadError::PremiseViolated("negative_stock_premise_not_checked"),
        }
    })
}

impl TallyRuntime {
    /// Tally's Negative Stock for `from` to `to`, with the same queue, mode and
    /// identity brackets as the other report reads: the book extent before and
    /// after must be equal and the report's two paired reads identical, or the
    /// read refuses. The report is one paired read (two requests), never retried.
    pub(crate) async fn fetch_negative_stock(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        from: TallyDate,
        to: TallyDate,
    ) -> anyhow::Result<NegativeStockRead> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, move |client| {
            let identity = identity.clone();
            let from = from.clone();
            let to = to.clone();
            async move {
                let mut evidence = RuntimeReadEvidence::empty();
                let result = async {
                    let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                    evidence = mode_evidence;
                    if profile == DateBoundaryProfile::EducationRestricted {
                        return Err(NegativeStockReadError::EducationUnqualified.into());
                    }
                    let period = NativeLedgerSnapshotPeriod::new(profile, from.clone(), to.clone())
                        .map_err(NegativeStockReadError::Period)?;
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    if from < *extent.books_from() {
                        return Err(NegativeStockReadError::BeforeBooks.into());
                    }
                    let mark = extent
                        .master_alter_id_high_water()
                        .ok_or(OutstandingsError::MasterWitnessAbsent)?
                        .get();
                    let estimate = admit_size(mark)?;

                    let request =
                        render_native_negative_stock_request(identity.display_name(), &period);
                    let (xml, bytes, hash) = client
                        .fetch_native_report_paired(request.clone())
                        .await?
                        .require_stable(PairedReadValidationError::NativeNegativeStock)?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    // Held until the closing extent is read: a book that moved
                    // mid-read is reported as moved, not as what its answer broke.
                    let answer = match parse_native_negative_stock(&xml) {
                        Ok(list) => Ok(NegativeStockAnswer::Listed(list)),
                        Err(NativeNegativeStockError::EmptyEnvelope) => {
                            Ok(NegativeStockAnswer::EmptyEnvelope)
                        }
                        Err(error) => Err(error),
                    };

                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::NegativeStockExtent.into());
                    }
                    let answer = answer?;
                    let rows = match &answer {
                        NegativeStockAnswer::Listed(list) => list.items.len(),
                        NegativeStockAnswer::EmptyEnvelope => 0,
                    };
                    check_premise(rows, mark, bytes, estimate)?;
                    bracket_verified_company_identity(&client, &identity).await?;
                    evidence = evidence
                        .clone()
                        .combine(confirm_read_boundary(&client, profile).await?);
                    Ok(NegativeStockRead {
                        from,
                        to,
                        answer,
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
#[path = "runtime_negative_stock_tests.rs"]
mod tests;
