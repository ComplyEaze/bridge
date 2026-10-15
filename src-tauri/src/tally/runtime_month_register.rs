//! One month-wise register read (Tally's own Sales or Purchase Register, by
//! report name), inside the identity and extent brackets the company features
//! read uses: the mode probe and identity, the opening extent, the register
//! (paired), the closing extent (which must equal the opening one), then
//! identity and mode again. The rows are Tally's as printed: nothing ties them
//! to the vouchers or to the trial balance, so no currency admission applies.
use super::*;
use bridge_tally_protocol::native_cash_flow::{WholeMonthWindow, WholeMonthWindowError};
use bridge_tally_protocol::native_month_register::{
    parse_native_month_register, render_native_month_register_request, MonthRegisterKind,
    NativeMonthRegister,
};

/// A completed read of one register over whole months.
pub(crate) struct MonthRegisterRead {
    pub(crate) register: NativeMonthRegister,
    pub(crate) evidence: RuntimeReadEvidence,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum MonthRegisterReadError {
    #[error("month_register_education_unqualified")]
    EducationUnqualified,
    #[error("month_register window refused")]
    Window(WholeMonthWindowError),
    #[error("month_register_before_books")]
    BeforeBooks,
}

impl MonthRegisterReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "month_register_education_unqualified",
            Self::Window(WholeMonthWindowError::NotMonthStart) => {
                "month_register_window_not_month_start"
            }
            Self::Window(WholeMonthWindowError::NotMonthEnd) => {
                "month_register_window_not_month_end"
            }
            Self::Window(WholeMonthWindowError::Reversed) => "month_register_window_reversed",
            Self::Window(WholeMonthWindowError::TooManyMonths) => {
                "month_register_window_too_many_months"
            }
            Self::Window(WholeMonthWindowError::UnsupportedBoundary) => {
                "month_register_window_unsupported_boundary"
            }
            Self::BeforeBooks => "month_register_before_books",
        }
    }
}

impl TallyRuntime {
    /// Tally's own `kind` register for the whole months `from` to `to`. A window
    /// that is not whole months is refused before any request; a window that
    /// starts before the books is refused before the register is asked for; a
    /// refusal of how the answer parsed returns at once; a book that moved
    /// between the extents is refused after the answer was read, so the closing
    /// extent decides.
    pub(crate) async fn fetch_month_register(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        kind: MonthRegisterKind,
        from: TallyDate,
        to: TallyDate,
    ) -> anyhow::Result<MonthRegisterRead> {
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
                        return Err(MonthRegisterReadError::EducationUnqualified.into());
                    }
                    let window = WholeMonthWindow::new(profile, from.clone(), to)
                        .map_err(MonthRegisterReadError::Window)?;
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    if from < *extent.books_from() {
                        return Err(MonthRegisterReadError::BeforeBooks.into());
                    }

                    let request = render_native_month_register_request(
                        kind,
                        identity.display_name(),
                        &window,
                    );
                    let (xml, bytes, hash) = client
                        .fetch_native_report_paired(request.clone())
                        .await?
                        .require_stable(PairedReadValidationError::MonthRegister)?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    let register = parse_native_month_register(&xml, &window)?;

                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::MonthRegisterExtent.into());
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    evidence = evidence
                        .clone()
                        .combine(confirm_read_boundary(&client, profile).await?);
                    Ok(MonthRegisterRead {
                        register,
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
