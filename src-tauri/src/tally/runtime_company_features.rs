//! One company-features read, inside the identity and extent brackets the stock
//! summary read uses: the mode probe and identity, the opening extent, the
//! Company collection (paired), the closing extent (which must equal the
//! opening one), then identity and mode again.
use super::*;
use bridge_tally_protocol::native_company_features::{
    parse_company_features, render_company_features_request, ExpectedCompany, NativeCompanyFeatures,
};

/// A completed observation of a company's settings and currency symbol.
pub(crate) struct CompanyFeaturesRead {
    pub(crate) features: NativeCompanyFeatures,
    pub(crate) evidence: RuntimeReadEvidence,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CompanyFeaturesReadError {
    #[error("company_features_education_unqualified")]
    EducationUnqualified,
}

impl CompanyFeaturesReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "company_features_education_unqualified",
        }
    }
}

impl TallyRuntime {
    /// The company's settings and currency symbol, with the same queue, mode and
    /// identity brackets as the stock summary read. A refusal of how the answer
    /// parsed returns at once; a book that moved between the extents is refused
    /// after the answer was read, so the closing extent decides.
    pub(crate) async fn fetch_company_features(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
    ) -> anyhow::Result<CompanyFeaturesRead> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(
            config,
            ReadOperation::MasterExport,
            ReadRetryPolicy::SINGLE_ATTEMPT,
            move |client| {
                let identity = identity.clone();
                async move {
                    let mut evidence = RuntimeReadEvidence::empty();
                    let result = async {
                        let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                        evidence = mode_evidence;
                        if profile == DateBoundaryProfile::EducationRestricted {
                            return Err(CompanyFeaturesReadError::EducationUnqualified.into());
                        }
                        bracket_verified_company_identity(&client, &identity).await?;
                        let extent = client.fetch_company_book_extent(&identity).await?;

                        let request = render_company_features_request(
                            identity.display_name(),
                            identity.company_guid(),
                        )?;
                        let (xml, bytes, hash) = client
                            .fetch_native_report_paired(request.clone())
                            .await?
                            .require_stable(PairedReadValidationError::CompanyFeatures)?;
                        evidence = evidence
                            .clone()
                            .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                        let features = parse_company_features(
                            &xml,
                            &ExpectedCompany {
                                guid: identity.company_guid(),
                                name: identity.display_name(),
                                number: identity.company_number(),
                                books_from: identity.books_from_yyyymmdd(),
                            },
                        )?;

                        let closing_extent = client.fetch_company_book_extent(&identity).await?;
                        if closing_extent != extent {
                            return Err(PairedReadValidationError::CompanyFeaturesExtent.into());
                        }
                        bracket_verified_company_identity(&client, &identity).await?;
                        evidence = evidence
                            .clone()
                            .combine(confirm_read_boundary(&client, profile).await?);
                        Ok(CompanyFeaturesRead {
                            features,
                            evidence: evidence.clone(),
                        })
                    }
                    .await;
                    result.map_err(|error| with_read_evidence(error, evidence))
                }
            },
        )
        .await
    }
}
