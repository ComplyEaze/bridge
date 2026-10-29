//! One native stock summary read, inside the bracket the masters read uses.
//!
//! The bracket is `fetch_masters_with_extent`'s, written out again rather than
//! shared: the two differ in their refusals (a separate error type and code
//! for each) and in what sits between the extents, and a generic wrapper over
//! an async body is more than this read needs.
use super::*;
use crate::tally::connection::compliance_estimate;
use bridge_tally_protocol::ledger_census::{
    render_stock_group_census_slice_request, render_stock_item_census_slice_request, Census,
    CensusCollection, CensusLimits, CensusPlan, CensusSlice,
};
use bridge_tally_protocol::native_masters::{
    MASTERS_ASSUMED_NAME_CHARS, MASTERS_RESPONSE_BUDGET_BYTES,
};
use bridge_tally_protocol::native_stock_summary::{
    gate_stock_summary, parse_company_inventory_flags, parse_native_stock_items,
    parse_native_stock_summary_report, render_company_inventory_flags_request,
    render_native_stock_summary_request, stock_item_worst_row_bytes, NativeFlag,
    NativeInventoryFlags, NativeStockGate, NativeStockItemCount, StockSummaryAsOf,
};
use bridge_tally_protocol::outstandings_shared::OutstandingsError;
use bridge_tally_protocol::xml_read_profiles::{ValidatedCompanyName, ValidatedDateRange};
use bridge_tally_protocol::{
    parse_stock_group_census_slice, parse_stock_item_census_slice, StandardLedgerCatalogError,
    MAX_STANDARD_LEDGER_IDENTITY_ROWS,
};

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
    /// The mark was past the single read, the census counted the items, and they are more
    /// than one read is admitted for (#1233), so no stock item was requested.
    #[error("stock_summary_too_large")]
    TooManyItems {
        master_alter_id: u64,
        counted_items: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
        /// The most items one read is admitted for: the budget over the worst stock-item row.
        limit_items: u64,
    },
    /// The counted items and stock groups, times an assumed worst report line, are over the
    /// response budget, so the report was not requested (#1233).
    #[error("stock_summary_report_too_large")]
    ReportTooLarge {
        counted_lines: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
    },
    /// Tally's own stock item count is higher than the census counted: the census missed items,
    /// and a read sized from it would be sized too small. Numbers only.
    #[error("stock_summary_item_count_company_higher")]
    ItemCountCompanyHigher { company: u64, census: u64 },
    /// A stock census refused (a slice over its width, a GUID twice, nothing found, more slices
    /// than allowed) or a slice's answer did not read; the census's own code, the cause, names
    /// which. Nothing was sized from it.
    #[error("stock_summary_census_refused")]
    CensusRefused(&'static str),
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
            Self::TooLarge { .. } | Self::TooManyItems { .. } => "stock_summary_too_large",
            Self::ReportTooLarge { .. } => "stock_summary_report_too_large",
            Self::ItemCountCompanyHigher { .. } => "stock_summary_item_count_company_higher",
            Self::CensusRefused(_) => "stock_summary_census_refused",
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

/// The stock censuses' limits (#1233): the ledger census's slice width and slice count, so they
/// cover a mark of 400,000. A slice holds at most its width in rows; a full slice of the ledger
/// census's worst row fits the masters budget, and a stock census row is taken to be no larger
/// (PARTIAL: no stock census answer was captured when this was written).
const STOCK_CENSUS_LIMITS: CensusLimits = CensusLimits {
    slice_width: 4_000,
    max_slices: 100,
};

/// Characters of one top-level line of Tally's Stock Summary report (a `DSPACCNAME` and its
/// `DSPSTKINFO`) that do not grow with its names, numbers included. The captured lines are about
/// 220 characters with names of 9 to 13; 300 is a margin over them, not a measurement. PARTIAL:
/// short names only, until a report with long names is captured (#1233).
const STOCK_REPORT_LINE_FIXED_CHARS: usize = 300;

/// Name-bearing texts of a report line: the display name, and the unit inside the closing
/// quantity and inside the rate. PARTIAL, as above.
const STOCK_REPORT_LINE_NAME_SLOTS: usize = 3;

/// An assumed worst-case report line, in UTF-16 bytes, by the masters formula: 5,208 bytes.
pub(crate) const fn stock_report_worst_line_bytes() -> u64 {
    2 * (STOCK_REPORT_LINE_FIXED_CHARS
        + 6 * MASTERS_ASSUMED_NAME_CHARS * STOCK_REPORT_LINE_NAME_SLOTS) as u64
}

/// What a read past the single-read mark was admitted on, from the stock censuses' counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CountedAdmission {
    pub(crate) items: u64,
    pub(crate) items_estimate: u64,
    pub(crate) report_estimate: u64,
}

impl CountedAdmission {
    /// The items read may not hold more rows than the census counted.
    pub(crate) fn check_items(&self, rows: usize) -> Result<(), StockSummaryReadError> {
        if u64::try_from(rows).unwrap_or(u64::MAX) > self.items {
            return Err(StockSummaryReadError::PremiseViolated(
                "stock_rows_exceed_census_count",
            ));
        }
        Ok(())
    }

    /// The report may not be larger than it was admitted at.
    pub(crate) fn check_report(&self, received_bytes: usize) -> Result<(), StockSummaryReadError> {
        if u64::try_from(received_bytes).unwrap_or(u64::MAX) > self.report_estimate {
            return Err(StockSummaryReadError::PremiseViolated(
                "stock_report_over_admitted_bytes",
            ));
        }
        Ok(())
    }
}

/// Admits the whole stock-item read, and sizes the report, on the censuses' counts (#1233), for a
/// book whose mark is past what the mark alone admits. The censuses are the bound; Tally's own
/// item count only refuses, when it is higher than the census's. A count above the mark cannot
/// be: every master holds its own AlterID at or under it, so such a count refuses rather than
/// admits. Sent before any stock-item request.
pub(crate) fn admit_counted_stock(
    mark: u64,
    items: u64,
    groups: u64,
    tally_items: NativeStockItemCount,
) -> Result<CountedAdmission, StockSummaryReadError> {
    let lines = items.saturating_add(groups);
    if lines > mark {
        return Err(StockSummaryReadError::PremiseViolated(
            "stock_census_exceeds_master_mark",
        ));
    }
    if let NativeStockItemCount::Reported(company) = tally_items {
        if company > items {
            return Err(StockSummaryReadError::ItemCountCompanyHigher {
                company,
                census: items,
            });
        }
    }
    let row_bytes = u64::try_from(stock_item_worst_row_bytes()).unwrap_or(u64::MAX);
    let items_estimate = compliance_estimate(items, row_bytes, budget_bytes());
    if !items_estimate.fits {
        return Err(StockSummaryReadError::TooManyItems {
            master_alter_id: mark,
            counted_items: items,
            estimated_bytes: items_estimate.estimated_bytes,
            limit_bytes: budget_bytes(),
            limit_items: budget_bytes().checked_div(row_bytes).unwrap_or(0),
        });
    }
    let report_estimate =
        compliance_estimate(lines, stock_report_worst_line_bytes(), budget_bytes());
    if !report_estimate.fits {
        return Err(StockSummaryReadError::ReportTooLarge {
            counted_lines: lines,
            estimated_bytes: report_estimate.estimated_bytes,
            limit_bytes: budget_bytes(),
        });
    }
    Ok(CountedAdmission {
        items,
        items_estimate: items_estimate.estimated_bytes,
        report_estimate: report_estimate.estimated_bytes,
    })
}

/// Counts THE COMPANY's rows of one master collection by AlterID span over `(0, mark]` (#1233),
/// one slice at a time, each read once. Each slice's answer is added to `evidence`. A census that
/// found no row refuses, unless `none_is_a_count`: a book may hold no stock group, while an empty
/// item census cannot be told from a company that is closed or absent. The extent read after the
/// census, or after the whole stock read, is what shows the book did not move meanwhile.
async fn count_by_span<C: CensusCollection + Copy>(
    client: &TallyClient,
    identity: &VerifiedCompanyIdentity,
    mark: u64,
    render: fn(&str, &CensusSlice<C>) -> String,
    parse: fn(&str, &str, u64) -> Result<Vec<String>, StandardLedgerCatalogError>,
    none_is_a_count: bool,
    evidence: &mut RuntimeReadEvidence,
) -> anyhow::Result<u64> {
    let refused = |error: bridge_tally_protocol::ledger_census::CensusError<C>| {
        StockSummaryReadError::CensusRefused(error.safe_code())
    };
    let plan = CensusPlan::<C>::new(mark, STOCK_CENSUS_LIMITS).map_err(refused)?;
    let mut census = Census::new(plan);
    while let Some(slice) = census.next_slice() {
        let request = render(identity.display_name(), &slice);
        let (body, bytes, sha256) = client.fetch_census_slice(request.clone()).await?;
        *evidence = evidence
            .clone()
            .combine(RuntimeReadEvidence::single(&request, sha256, bytes));
        // The parser only bounds what it will hold, as the ledger census's does; the slice's
        // width is enforced once, by the census, under its own code.
        let guids = parse(
            &body,
            identity.company_guid(),
            MAX_STANDARD_LEDGER_IDENTITY_ROWS as u64,
        )
        .map_err(|_| StockSummaryReadError::CensusRefused("stock_census_slice_invalid"))?;
        census.accept(guids).map_err(refused)?;
    }
    match census.finish() {
        Ok(count) => Ok(count.get()),
        Err(bridge_tally_protocol::ledger_census::CensusError::CensusEmpty) if none_is_a_count => {
            Ok(0)
        }
        Err(error) => Err(refused(error).into()),
    }
}

/// Counts the stock items, then the stock groups, and admits the read on the counts
/// ([`admit_counted_stock`]). The items alone are admitted first, so a book with more items than
/// one read holds is refused before the group census's reads.
async fn admit_on_census(
    client: &TallyClient,
    identity: &VerifiedCompanyIdentity,
    mark: u64,
    tally_items: NativeStockItemCount,
    evidence: &mut RuntimeReadEvidence,
) -> anyhow::Result<CountedAdmission> {
    let items = count_by_span(
        client,
        identity,
        mark,
        render_stock_item_census_slice_request,
        parse_stock_item_census_slice,
        false,
        evidence,
    )
    .await?;
    admit_counted_stock(mark, items, 0, tally_items)?;
    let groups = count_by_span(
        client,
        identity,
        mark,
        render_stock_group_census_slice_request,
        parse_stock_group_census_slice,
        true,
        evidence,
    )
    .await?;
    Ok(admit_counted_stock(mark, items, groups, tally_items)?)
}

impl TallyRuntime {
    /// The company's stock at `as_of`, with the same queue, mode and identity
    /// brackets as the masters read: the extent before and after must be equal,
    /// or the read refuses. In that bracket it reads the company's inventory
    /// flags, the stock items (only when the flags do not say inventory is off
    /// and the master mark admits them, or, past that mark, the counts of a
    /// stock-item and a stock-group census do: `admit_counted_stock`, #1233) and
    /// Tally's own Stock Summary, each paired, and compares the sum of the
    /// report's top-level lines with the items' closing values.
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
                        return Err(StockSummaryReadError::EducationUnqualified.into());
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    let period =
                        stock_summary_period(profile, &as_of, extent.books_from(), &today)?;

                    let request = render_company_inventory_flags_request(
                        identity.display_name(),
                        identity.company_guid_value(),
                    );
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
                    // Past the mark the single read is admitted for, the censuses count the
                    // items and stock groups, and their counts admit the same read (#1233).
                    let (estimate, counted) = match admit_stock_summary_size(mark) {
                        Ok(estimate) => (estimate, None),
                        Err(StockSummaryReadError::TooLarge { .. }) => {
                            match admit_on_census(
                                &client,
                                &identity,
                                mark,
                                company_inventory.item_count,
                                &mut evidence,
                            )
                            .await
                            {
                                Ok(counted) => (counted.items_estimate, Some(counted)),
                                // A refusal of what the census found is held until the extent is
                                // read again: a book that moved during the census (an altered
                                // master changes its AlterID between slices) is reported as
                                // moved, not as what its census broke.
                                Err(error)
                                    if error
                                        .chain()
                                        .any(|cause| cause.is::<StockSummaryReadError>()) =>
                                {
                                    let closing_extent =
                                        client.fetch_company_book_extent(&identity).await?;
                                    if closing_extent != extent {
                                        return Err(
                                            PairedReadValidationError::StockSummaryExtent.into()
                                        );
                                    }
                                    return Err(error);
                                }
                                Err(error) => return Err(error),
                            }
                        }
                        Err(error) => return Err(error.into()),
                    };
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
                    let premise = check_stock_premise(items.rows.len(), mark, bytes, estimate)
                        .and_then(|()| {
                            counted.map_or(Ok(()), |counted| counted.check_items(items.rows.len()))
                        });

                    let request =
                        render_native_stock_summary_request(identity.display_name(), &period);
                    let (xml, bytes, hash) = client
                        .fetch_native_report_paired(request.clone())
                        .await?
                        .require_stable(PairedReadValidationError::NativeStatement)?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    // Held, like the premise, until the closing extent is
                    // read: a book that moved says so before what its rows,
                    // its item count or its report would have refused.
                    let report_premise =
                        counted.map_or(Ok(()), |counted| counted.check_report(bytes));
                    let gate = parse_native_stock_summary_report(&xml).and_then(|report| {
                        gate_stock_summary(items.rows, company_inventory.item_count, &report)
                    });

                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::StockSummaryExtent.into());
                    }
                    premise?;
                    report_premise?;
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
        })
        .await
    }
}
