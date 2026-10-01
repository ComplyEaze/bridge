//! One native stock summary read, inside the bracket the masters read uses.
//!
//! The bracket is `fetch_masters_with_extent`'s, written out again rather than
//! shared: the two differ in their refusals (a separate error type and code
//! for each) and in what sits between the extents, and a generic wrapper over
//! an async body is more than this read needs.
use super::*;
use crate::tally::connection::compliance_estimate;
use bridge_tally_protocol::native_masters::MASTERS_RESPONSE_BUDGET_BYTES;
use bridge_tally_protocol::native_stock_summary::{
    gate_stock_summary, parse_company_inventory_flags, parse_native_stock_items,
    parse_native_stock_summary_report, render_company_inventory_flags_request,
    render_native_stock_summary_request, stock_item_worst_row_bytes, NativeFlag,
    NativeInventoryFlags, NativeStockGate, StockSummaryAsOf,
};
use bridge_tally_protocol::outstandings_shared::OutstandingsError;
use bridge_tally_protocol::xml_read_profiles::{ValidatedCompanyName, ValidatedDateRange};

/// A completed observation of a company's stock at a date: its inventory
/// flags, and its stock items after the sum of the top-level lines of Tally's
/// own Stock Summary has been compared with their closing values.
pub(crate) struct StockSummaryRead {
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
    pub(crate) inventory: NativeInventoryFlags,
    pub(crate) gate: NativeStockGate,
    pub(crate) evidence: RuntimeReadEvidence,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum StockSummaryReadError {
    #[error("stock_summary_education_unqualified")]
    EducationUnqualified,
    /// The company's `ISINVENTORYON` is `No`: no stock item was requested.
    #[error("stock_not_enabled")]
    NotEnabled,
    #[error("stock_summary_as_of_before_books")]
    AsOfBeforeBooks,
    #[error("stock_summary_as_of_in_future")]
    AsOfInFuture,
    #[error("stock_summary_period_not_honoured")]
    PeriodNotHonoured,
    /// The master mark, times an assumed worst-case stock-item row, is over the
    /// response budget, so no stock item was requested.
    #[error("stock_summary_too_large")]
    TooLarge {
        master_alter_id: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
        /// The largest mark admitted: the budget over the worst stock-item row.
        limit_master_alter_id: u64,
    },
    /// The items came back and broke what their size was admitted on (the
    /// master mark bounds the rows; the response fits the estimate).
    #[error("stock_summary_bound_premise_violated")]
    PremiseViolated(&'static str),
}

impl StockSummaryReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "stock_summary_education_unqualified",
            Self::NotEnabled => "stock_not_enabled",
            Self::AsOfBeforeBooks => "stock_summary_as_of_before_books",
            Self::AsOfInFuture => "stock_summary_as_of_in_future",
            Self::PeriodNotHonoured => "stock_summary_period_not_honoured",
            Self::TooLarge { .. } => "stock_summary_too_large",
            Self::PremiseViolated(_) => "stock_summary_bound_premise_violated",
        }
    }
}

/// The financial year's first day (1 April) containing `date`.
fn financial_year_start(date: &TallyDate) -> Option<TallyDate> {
    let text = date.as_str();
    let year: u32 = text[0..4].parse().ok()?;
    let month: u32 = text[4..6].parse().ok()?;
    let year = if month >= 4 {
        year
    } else {
        year.checked_sub(1)?
    };
    TallyDate::parse(format!("{year:04}0401")).ok()
}

/// The period a stock summary is read over: from the start of the financial
/// year containing `as_of`, or the book's start if that is later, to `as_of`.
/// `as_of` must not precede the book's start or follow the host's today. The
/// period type and its endpoint admission are the Trial Balance's.
pub(crate) fn stock_summary_period(
    profile: DateBoundaryProfile,
    as_of: &StockSummaryAsOf,
    books_from: &TallyDate,
    today: &TallyDate,
) -> Result<NativeLedgerSnapshotPeriod, StockSummaryReadError> {
    let as_of = as_of.date();
    if as_of < books_from {
        return Err(StockSummaryReadError::AsOfBeforeBooks);
    }
    if as_of > today {
        return Err(StockSummaryReadError::AsOfInFuture);
    }
    let start = financial_year_start(as_of).ok_or(StockSummaryReadError::PeriodNotHonoured)?;
    NativeLedgerSnapshotPeriod::new(profile, start.max(books_from.clone()), as_of.clone())
        .map_err(|_| StockSummaryReadError::PeriodNotHonoured)
}

fn budget_bytes() -> u64 {
    u64::try_from(MASTERS_RESPONSE_BUDGET_BYTES).unwrap_or(u64::MAX)
}

/// Admits a whole read only when the master mark, times an assumed worst-case
/// stock-item row, fits the response budget (an estimate exactly at the budget
/// fits). Returns the estimate. Sent before any stock-item request.
pub(crate) fn admit_stock_summary_size(master_alter_id: u64) -> Result<u64, StockSummaryReadError> {
    let row_bytes = u64::try_from(stock_item_worst_row_bytes()).unwrap_or(u64::MAX);
    let estimate = compliance_estimate(master_alter_id, row_bytes, budget_bytes());
    if !estimate.fits {
        return Err(StockSummaryReadError::TooLarge {
            master_alter_id,
            estimated_bytes: estimate.estimated_bytes,
            limit_bytes: budget_bytes(),
            limit_master_alter_id: budget_bytes().checked_div(row_bytes).unwrap_or(0),
        });
    }
    Ok(estimate.estimated_bytes)
}

/// The premise a read was admitted on, checked against what came back: every
/// object has its own AlterID at or under the mark, so the rows cannot outnumber
/// the mark, and the response (one of the two paired reads, checked after both)
/// cannot exceed the admitted estimate. The rows' own AlterIDs are not checked,
/// as the masters read checks theirs: the stock-item request does not fetch
/// `ALTERID`, and adding it would leave the committed capture's request. A
/// violation is a loud refusal, not a truncated read.
pub(crate) fn check_stock_premise(
    rows: usize,
    master_alter_id: u64,
    received_bytes: usize,
    estimate_bytes: u64,
) -> Result<(), StockSummaryReadError> {
    if u64::try_from(rows).unwrap_or(u64::MAX) > master_alter_id {
        return Err(StockSummaryReadError::PremiseViolated(
            "stock_rows_exceed_master_mark",
        ));
    }
    if u64::try_from(received_bytes).unwrap_or(u64::MAX) > estimate_bytes {
        return Err(StockSummaryReadError::PremiseViolated(
            "stock_response_over_admitted_bytes",
        ));
    }
    Ok(())
}

impl TallyRuntime {
    /// The company's stock at `as_of`, with the same queue, mode and identity
    /// brackets as the masters read: the extent before and after must be equal,
    /// or the read refuses. In that bracket it reads the company's inventory
    /// flags, the stock items (only when the flags do not say inventory is off
    /// and the master mark admits them) and Tally's own Stock Summary, each
    /// paired, and compares the sum of the report's top-level lines with the
    /// items' closing values.
    /// Also returns the extent, so a caller can tell later whether the book has
    /// moved. A refusal of how the answer parsed returns at once; a refusal of
    /// what the mark admitted is held until the closing extent is read, so a
    /// book that moved mid-read is reported as moved.
    pub(crate) async fn fetch_stock_summary_with_extent(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        as_of: StockSummaryAsOf,
        today: TallyDate,
    ) -> anyhow::Result<(StockSummaryRead, CompanyBookExtent)> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(
            config,
            ReadOperation::MasterExport,
            ReadRetryPolicy::SINGLE_ATTEMPT,
            move |client| {
                let identity = identity.clone();
                let as_of = as_of.clone();
                let today = today.clone();
                async move {
                    let mut evidence = RuntimeReadEvidence::empty();
                    let result = async {
                        let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                        evidence = mode_evidence;
                        if profile == DateBoundaryProfile::EducationRestricted {
                            return Err(StockSummaryReadError::EducationUnqualified.into());
                        }
                        bracket_verified_company_identity(&client, &identity).await?;
                        let extent = client.fetch_company_book_extent(&identity).await?;
                        let period =
                            stock_summary_period(profile, &as_of, extent.books_from(), &today)?;

                        let request = render_company_inventory_flags_request(
                            identity.display_name(),
                            identity.company_guid(),
                        )?;
                        let (xml, bytes, hash) = client
                            .fetch_native_report_paired(request.clone())
                            .await?
                            .require_stable(PairedReadValidationError::StockSummaryCollection)?;
                        evidence = evidence
                            .clone()
                            .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                        let company_inventory =
                            parse_company_inventory_flags(&xml, identity.company_guid())?;
                        let inventory = company_inventory.flags;
                        if inventory.inventory_on == NativeFlag::No {
                            return Err(StockSummaryReadError::NotEnabled.into());
                        }

                        let mark = extent
                            .master_alter_id_high_water()
                            .ok_or(OutstandingsError::MasterWitnessAbsent)?
                            .get();
                        let estimate = admit_stock_summary_size(mark)?;
                        let company = ValidatedCompanyName::new(identity.display_name())?;
                        let range =
                            ValidatedDateRange::new(period.from().as_str(), period.to().as_str())?;
                        let request = ReadOnlyProfile::AuditStockItemsV1 {
                            company: &company,
                            period: &range,
                        }
                        .render();
                        let (xml, bytes, hash) = client
                            .fetch_native_report_paired(request.clone())
                            .await?
                            .require_stable(PairedReadValidationError::StockSummaryCollection)?;
                        evidence = evidence
                            .clone()
                            .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                        let items = parse_native_stock_items(&xml, identity.company_guid())?;
                        // Held until the closing extent is read: a book that
                        // moved mid-read is reported as moved, not as whatever
                        // its new rows broke.
                        let premise = check_stock_premise(items.rows.len(), mark, bytes, estimate);

                        let request =
                            render_native_stock_summary_request(identity.display_name(), &period);
                        let (xml, bytes, hash) = client
                            .fetch_native_report_paired(request.clone())
                            .await?
                            .require_stable(PairedReadValidationError::NativeStatement)?;
                        evidence = evidence
                            .clone()
                            .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                        let report = parse_native_stock_summary_report(&xml)?;
                        // Held, like the premise, until the closing extent is
                        // read: a book that moved says so before what its rows
                        // or its item count would have refused.
                        let gate =
                            gate_stock_summary(items.rows, company_inventory.item_count, &report);

                        let closing_extent = client.fetch_company_book_extent(&identity).await?;
                        if closing_extent != extent {
                            return Err(PairedReadValidationError::StockSummaryExtent.into());
                        }
                        premise?;
                        let gate = gate?;
                        bracket_verified_company_identity(&client, &identity).await?;
                        evidence = evidence
                            .clone()
                            .combine(confirm_read_boundary(&client, profile).await?);
                        Ok((
                            StockSummaryRead {
                                from: period.from().clone(),
                                to: period.to().clone(),
                                inventory,
                                gate,
                                evidence: evidence.clone(),
                            },
                            extent,
                        ))
                    }
                    .await;
                    result.map_err(|error| with_read_evidence(error, evidence))
                }
            },
        )
        .await
    }
}
