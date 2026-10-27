//! One native Negative Ledgers read, inside the identity and book-extent bracket
//! the other built-in report reads use (#1358).
//!
//! The report is Tally's own Negative Ledgers list (the rule by which Tally lists a
//! ledger is Tally's, is not established and is not derived here), so
//! this read compares it with no other source: it is the report as returned, bound
//! only by the company, mode and extent checks around it. The bracket is the
//! trial balance read's (the period before the identity bracket, the books-from
//! check after the extent), written out again for the same reason as the stock
//! summary's: a separate error type and code, and nothing between the extents
//! but one paired report.
use super::*;
use bridge_tally_protocol::native_negative_ledgers::{
    parse_native_negative_ledgers, render_native_negative_ledgers_request, NativeNegativeLedgers,
};

/// A completed observation of Tally's Negative Ledgers for a window.
pub(crate) struct NegativeLedgersRead {
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
    pub(crate) list: NativeNegativeLedgers,
    pub(crate) evidence: RuntimeReadEvidence,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum NegativeLedgersReadError {
    #[error("negative_ledgers_education_unqualified")]
    EducationUnqualified,
    #[error("negative_ledgers_before_books")]
    BeforeBooks,
    #[error("negative_ledgers_period_not_honoured")]
    Period(bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriodError),
}

impl NegativeLedgersReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "negative_ledgers_education_unqualified",
            Self::BeforeBooks => "negative_ledgers_before_books",
            Self::Period(_) => "negative_ledgers_period_not_honoured",
        }
    }
}

impl TallyRuntime {
    /// Tally's Negative Ledgers for `from` to `to`, with the same queue, mode and
    /// identity brackets as the other report reads: the book extent before and
    /// after must be equal and the report's two paired reads identical, or the
    /// read refuses. The report has no size check: its cost on a large book is
    /// not known, and the report is one paired read (two requests), never retried.
    pub(crate) async fn fetch_negative_ledgers(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        from: TallyDate,
        to: TallyDate,
    ) -> anyhow::Result<NegativeLedgersRead> {
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
                        return Err(NegativeLedgersReadError::EducationUnqualified.into());
                    }
                    let period = NativeLedgerSnapshotPeriod::new(profile, from.clone(), to.clone())
                        .map_err(NegativeLedgersReadError::Period)?;
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    if from < *extent.books_from() {
                        return Err(NegativeLedgersReadError::BeforeBooks.into());
                    }

                    let request =
                        render_native_negative_ledgers_request(identity.display_name(), &period);
                    let (xml, bytes, hash) = client
                        .fetch_native_report_paired(request.clone())
                        .await?
                        .require_stable(PairedReadValidationError::NativeNegativeLedgers)?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    // Held until the closing extent is read: a book that moved
                    // mid-read is reported as moved, not as what its answer broke.
                    let list = parse_native_negative_ledgers(&xml);

                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::NativeLedgerExtent.into());
                    }
                    let list = list?;
                    bracket_verified_company_identity(&client, &identity).await?;
                    evidence = evidence
                        .clone()
                        .combine(confirm_read_boundary(&client, profile).await?);
                    Ok(NegativeLedgersRead {
                        from,
                        to,
                        list,
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
