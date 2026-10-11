//! One native report operation shared by desktop and MCP callers.
use super::*;
use bridge_tally_protocol::native_cash_flow::{
    parse_native_cash_flow, render_native_cash_flow_request, NativeCashFlow, WholeMonthWindow,
    WholeMonthWindowError,
};
use bridge_tally_protocol::native_statement_reports::{
    parse_native_statement, render_native_statement_request, NativeStatement, NativeStatementKind,
};
use bridge_tally_protocol::native_trial_balance::{
    parse_native_trial_balance, parse_native_trial_balance_with_currency,
    render_native_trial_balance_request, render_native_trial_balance_request_with_currency,
    NativeTrialBalance,
};
use bridge_tally_protocol::TallyNamedMaster;

/// A completed observation, not a reusable admission for a later write.
#[derive(Debug, Clone, Serialize)]
pub struct TrialBalanceRead {
    pub company_guid: String,
    pub company_name: String,
    pub from: TallyDate,
    pub to: TallyDate,
    pub currency: CompanyCurrency,
    pub report: NativeTrialBalance,
    pub totals: crate::reports::trial_balance::TrialBalanceTotals,
    pub read_at: String,
    pub evidence: RuntimeReadEvidence,
    /// Which ledgers `report` and `totals` cover. Only a caller that asks for
    /// [`TrialBalanceCurrencyScope::BaseCurrencyLedgersOnly`] can receive a
    /// partial one: the MCP read and, since bridge#709, the desktop screen and
    /// its workbook, which show the ledgers left out.
    pub ledger_scope: TrialBalanceLedgerScope,
}

/// Whether a caller can present a Trial Balance that covers only part of the
/// book. A caller that cannot show the ledgers left out must never receive
/// one, so the partial read is opt-in: [`TallyRuntime::fetch_trial_balance`]
/// keeps the single-INR refusal for every other reader (bridge#709).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TrialBalanceCurrencyScope {
    /// One Currency master only; several refuse (`company_base_currency_undetermined`).
    SingleCurrency,
    /// Several masters admitted through the identified INR base: the plain
    /// base-currency ledgers are read, the rest set aside by name.
    BaseCurrencyLedgersOnly,
}

impl TrialBalanceRead {
    /// The currency this read's amounts are in, as a label and its display
    /// precision: for a several-currency book the base Tally identified, never
    /// `currency`, which there describes the first master read (bridge#709).
    pub(crate) fn amount_currency(&self) -> (&str, u8) {
        match &self.ledger_scope {
            TrialBalanceLedgerScope::AllLedgers => (
                self.currency.mailing_name.as_str(),
                self.currency.decimal_places,
            ),
            TrialBalanceLedgerScope::BaseCurrencyLedgersOnly {
                base_name,
                decimal_places,
                ..
            } => (base_name.as_str(), *decimal_places),
        }
    }
}

/// What every presentation of a several-currency book's Trial Balance says
/// about its totals: the MCP read, the desktop screen and its workbook.
pub(crate) const BASE_CURRENCY_LEDGERS_ONLY_LIMITATION: &str = "Totals cover this book's plain base-currency ledgers only. Ledgers kept in another currency, and base-currency ledgers with a balance Tally shows in another currency, are left out and named, so debit and credit totals are expected to differ.";

/// The first line every surface shows for a several-currency book's partial
/// read, in its own output rather than only in a field (bridge#709): what it
/// covers, and how many ledgers it left out and listed.
pub(crate) fn base_currency_ledgers_only_label(foreign: usize, mixed: usize) -> String {
    let ledgers = |count: usize| if count == 1 { "ledger" } else { "ledgers" };
    format!(
        "Base-currency ledgers only: {foreign} {} kept in another currency and {mixed} base-currency {} with a value Tally shows in another currency are excluded and listed.",
        ledgers(foreign),
        ledgers(mixed),
    )
}

/// The ledgers a Trial Balance read covers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TrialBalanceLedgerScope {
    /// Every ledger of a book with one Currency master.
    #[default]
    AllLedgers,
    /// A several-currency book's plain base-currency ledgers. Totals are not
    /// expected to balance and no balanced check is ever made over them.
    BaseCurrencyLedgersOnly {
        /// The identified base master's NAME (the one its ledgers carry).
        base_name: String,
        decimal_places: u8,
        foreign: Vec<bridge_tally_protocol::native_outstandings::ForeignCurrencyLedger>,
        mixed: Vec<String>,
    },
}

/// A Profit and Loss or Balance Sheet read: the Trial Balance it derives from,
/// the group tree that classifies it, and Tally's own Balance Sheet (the gate)
/// and, for a P&L, Tally's own Profit and Loss, all read inside one identity and
/// book-extent bracket (#692).
#[derive(Debug, Clone, Serialize)]
pub struct StatementsRead {
    pub trial_balance: TrialBalanceRead,
    pub derived: crate::reports::statements::DerivedStatements,
}

/// Sealed so that nothing outside it can build a [`SingleInrAdmission`] or a
/// [`SingleCurrencyTrialBalance`]: the rest of this module can obtain the first
/// only from `admit_single_inr`, and the second only from `admitted` with it.
mod single_inr {
    use super::{CompanyCurrencyRead, NativeTrialBalance};

    /// Proof that a currency read found exactly one Currency master, and that it
    /// is INR: `admit_single_inr` is the only way to obtain one, and it succeeds
    /// only where `admit_inr` does. A several-currency book's admission is a
    /// different type and cannot stand in for it (#692; #715 admits its partial
    /// read by another path).
    #[derive(Debug)]
    pub(crate) struct SingleInrAdmission {
        _sealed: (),
    }

    impl CompanyCurrencyRead {
        pub(crate) fn admit_single_inr(self) -> Result<SingleInrAdmission, &'static str> {
            self.admit_inr().map(|_| SingleInrAdmission { _sealed: () })
        }
    }

    /// A Trial Balance whose company passed the single-INR admission: every
    /// ledger of a book with one currency master. It is built only from a
    /// [`SingleInrAdmission`] taken in the same bracket, so a read of a
    /// several-currency book can never become one. The statement derivation
    /// accepts nothing else (#692).
    #[derive(Debug, Clone)]
    pub(crate) struct SingleCurrencyTrialBalance(NativeTrialBalance);

    impl SingleCurrencyTrialBalance {
        pub(super) fn admitted(
            report: NativeTrialBalance,
            _admission: &SingleInrAdmission,
        ) -> Self {
            Self(report)
        }

        pub(crate) fn report(&self) -> &NativeTrialBalance {
            &self.0
        }

        #[cfg(test)]
        pub(crate) fn admitted_for_tests(report: NativeTrialBalance) -> Self {
            Self(report)
        }
    }
}
pub(crate) use single_inr::SingleCurrencyTrialBalance;
use single_inr::SingleInrAdmission;

/// How a Trial Balance read admitted its company's currency: the single-INR
/// admission of every monetary report, or, for a caller that asked for a
/// several-currency book's base-currency ledgers, the identified INR base.
enum TrialBalanceAdmission {
    SingleInr(SingleInrAdmission),
    BaseAmongSeveral(IdentifiedBaseCurrency),
}

/// What a statement read adds to a Trial Balance read.
struct StatementSources {
    trial_balance: SingleCurrencyTrialBalance,
    groups: Vec<TallyNamedMaster>,
    balance_sheet: NativeStatement,
    profit_and_loss: Option<NativeStatement>,
}

/// What a Cash Flow read adds to a Trial Balance read: the group tree that
/// classifies its ledgers and Tally's own Cash Flow for the same window.
struct CashFlowSources {
    trial_balance: SingleCurrencyTrialBalance,
    groups: Vec<TallyNamedMaster>,
    cash_flow: NativeCashFlow,
}

enum ExtraSources {
    Statement(StatementSources),
    CashFlow(CashFlowSources),
}

/// What a Trial Balance read is asked to read inside its bracket besides the
/// Trial Balance itself.
#[derive(Clone, Copy)]
enum TrialBalanceExtras {
    Nothing,
    Statement(NativeStatementKind),
    /// Tally's Cash Flow over whole months. The window is built inside the
    /// bracket, once the mode's date boundary profile is known.
    CashFlow,
}

/// A Cash Flow read: the Trial Balance it is checked against, Tally's own
/// Cash Flow, the window it covers, and the check of the one against the other,
/// all read inside one identity and book-extent bracket (#1232).
#[derive(Debug, Clone)]
pub(crate) struct CashFlowRead {
    pub(crate) trial_balance: TrialBalanceRead,
    pub(crate) cash_flow: NativeCashFlow,
    pub(crate) check: crate::reports::cash_flow::CashFlowCheck,
}

/// An ordered caller-selected range. Profile-specific boundary admission stays
/// inside the identity-bracketed runtime read.
#[derive(Debug, Clone)]
pub struct TrialBalancePeriod {
    from: TallyDate,
    to: TallyDate,
}

impl TrialBalancePeriod {
    pub(crate) fn new(from: TallyDate, to: TallyDate) -> Result<Self, TrialBalanceReadError> {
        if from > to {
            return Err(TrialBalanceReadError::Period(
                bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriodError::InvalidRange,
            ));
        }
        Ok(Self { from, to })
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum TrialBalanceReadError {
    #[error("trial_balance_education_unqualified")]
    EducationUnqualified,
    #[error("trial_balance_before_books")]
    BeforeBooks,
    #[error("trial_balance_period_not_honoured")]
    Period(bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriodError),
    /// The Cash Flow's window is not whole months inside the limit (#1232).
    #[error("cash_flow_window_refused")]
    CashFlowWindow(WholeMonthWindowError),
    #[error("{0}")]
    Currency(&'static str),
}

impl TrialBalanceReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "trial_balance_education_unqualified",
            Self::BeforeBooks => "trial_balance_before_books",
            Self::Period(_) => "trial_balance_period_not_honoured",
            Self::CashFlowWindow(error) => error.code(),
            Self::Currency(code) => code,
        }
    }
}

impl TallyRuntime {
    /// Native ledger-wise Trial Balance, with the same queue and read/write
    /// barrier as other monetary reads. Paired responses and stable book extent
    /// detect observed drift; they do not establish an atomic Tally snapshot.
    pub async fn fetch_trial_balance(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        period: TrialBalancePeriod,
    ) -> anyhow::Result<TrialBalanceRead> {
        self.fetch_trial_balance_with_extent(
            config,
            identity,
            period,
            TrialBalanceCurrencyScope::SingleCurrency,
        )
        .await
        .map(|(read, _)| read)
    }

    /// As [`Self::fetch_trial_balance`], also returning the book extent the
    /// read was pinned under: its opening and closing extents were equal, or
    /// the read refused. A caller can tell later whether the book has moved
    /// since (#630).
    pub(crate) async fn fetch_trial_balance_with_extent(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        period: TrialBalancePeriod,
        scope: TrialBalanceCurrencyScope,
    ) -> anyhow::Result<(TrialBalanceRead, CompanyBookExtent)> {
        self.fetch_trial_balance_sources(
            config,
            identity,
            period,
            scope,
            TrialBalanceExtras::Nothing,
        )
        .await
        .map(|(read, _, extent)| (read, extent))
    }

    /// Tally's `kind` statement derived from the Trial Balance and group tree.
    /// Tally's own Balance Sheet for the same window gates every result, and for
    /// a P&L Tally's own Profit and Loss gates gross and net as well.
    pub(crate) async fn fetch_statements(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        period: TrialBalancePeriod,
        kind: NativeStatementKind,
    ) -> anyhow::Result<StatementsRead> {
        let (trial_balance, sources, _) = self
            .fetch_trial_balance_sources(
                config,
                identity,
                period,
                TrialBalanceCurrencyScope::SingleCurrency,
                TrialBalanceExtras::Statement(kind),
            )
            .await?;
        let Some(ExtraSources::Statement(sources)) = sources else {
            return Err(anyhow::anyhow!("statement_sources_not_read"));
        };
        let derived = crate::reports::statements::derive_statements(
            &sources.trial_balance,
            &sources.groups,
            &sources.balance_sheet,
            sources.profit_and_loss.as_ref(),
        )?;
        Ok(StatementsRead {
            trial_balance,
            derived,
        })
    }

    /// Tally's own Cash Flow for whole months, with the Trial Balance of the
    /// same window and the book's group tree read beside it, and the check of
    /// the Cash Flow's net total against the cash and bank ledgers' movement.
    /// All inside one identity and book-extent bracket; a several-currency book
    /// is refused as for every statement.
    pub(crate) async fn fetch_cash_flow(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        period: TrialBalancePeriod,
    ) -> anyhow::Result<CashFlowRead> {
        let (trial_balance, sources, _) = self
            .fetch_trial_balance_sources(
                config,
                identity,
                period,
                TrialBalanceCurrencyScope::SingleCurrency,
                TrialBalanceExtras::CashFlow,
            )
            .await?;
        let Some(ExtraSources::CashFlow(sources)) = sources else {
            return Err(anyhow::anyhow!("cash_flow_sources_not_read"));
        };
        let check = crate::reports::cash_flow::check_cash_flow(
            &sources.trial_balance,
            &sources.groups,
            &sources.cash_flow,
        )?;
        Ok(CashFlowRead {
            trial_balance,
            cash_flow: sources.cash_flow,
            check,
        })
    }

    async fn fetch_trial_balance_sources(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        period: TrialBalancePeriod,
        scope: TrialBalanceCurrencyScope,
        extras: TrialBalanceExtras,
    ) -> anyhow::Result<(TrialBalanceRead, Option<ExtraSources>, CompanyBookExtent)> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(
            config,
            ReadOperation::MasterExport,
            ReadRetryPolicy::SINGLE_ATTEMPT,
            move |client| {
                let identity = identity.clone();
                let from = period.from.clone();
                let to = period.to.clone();
                async move {
                    let mut evidence = RuntimeReadEvidence::empty();
                    let result = async {
                        let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                        evidence = mode_evidence;
                        if profile == DateBoundaryProfile::EducationRestricted {
                            return Err(TrialBalanceReadError::EducationUnqualified.into());
                        }
                        let period =
                            NativeLedgerSnapshotPeriod::new(profile, from.clone(), to.clone())
                                .map_err(TrialBalanceReadError::Period)?;
                        // Before anything is sent for the report: a window that
                        // is not whole months costs no request.
                        let cash_flow_window = match extras {
                            TrialBalanceExtras::CashFlow => Some(
                                WholeMonthWindow::new(profile, from.clone(), to.clone())
                                    .map_err(TrialBalanceReadError::CashFlowWindow)?,
                            ),
                            _ => None,
                        };
                        bracket_verified_company_identity(&client, &identity).await?;
                        let extent = client.fetch_company_book_extent(&identity).await?;
                        if from < *extent.books_from() {
                            return Err(TrialBalanceReadError::BeforeBooks.into());
                        }

                        let currency_request =
                            render_company_currency_request(identity.display_name());
                        let (currency_xml, bytes, hash) = client
                            .fetch_native_report_paired(currency_request.clone())
                            .await?
                            .require_stable(PairedReadValidationError::CurrencyMaster)?;
                        evidence = evidence.clone().combine(RuntimeReadEvidence::paired(
                            &currency_request,
                            hash,
                            bytes,
                        ));
                        let currency = parse_company_currency(&currency_xml)?;
                        // A several-currency book is admitted only for a caller
                        // that can show the ledgers left out, through the base
                        // Tally identifies (bridge#551). Every other read keeps
                        // the single-INR admission of existing monetary reports,
                        // and only that admission can build the Trial Balance a
                        // statement is derived from (#692).
                        let admitted = if scope
                            == TrialBalanceCurrencyScope::BaseCurrencyLedgersOnly
                            && currency.currency_count > 1
                        {
                            let masters = parse_currency_master_list(&currency_xml)?;
                            let identified = identify_base_among_several(
                                &client,
                                &identity,
                                &mut evidence,
                                masters,
                            )
                            .await?
                            .ok_or(TrialBalanceReadError::Currency(
                                "company_base_currency_undetermined",
                            ))?;
                            if !identified.is_inr() {
                                return Err(TrialBalanceReadError::Currency(
                                    "company_base_currency_not_inr",
                                )
                                .into());
                            }
                            TrialBalanceAdmission::BaseAmongSeveral(identified)
                        } else {
                            TrialBalanceAdmission::SingleInr(
                                CompanyCurrencyRead {
                                    currency: currency.clone(),
                                    extent: extent.clone(),
                                    evidence: evidence.clone(),
                                }
                                .admit_single_inr()
                                .map_err(TrialBalanceReadError::Currency)?,
                            )
                        };
                        let base = match &admitted {
                            TrialBalanceAdmission::BaseAmongSeveral(identified) => Some(identified),
                            TrialBalanceAdmission::SingleInr(_) => None,
                        };

                        let request = match &base {
                            Some(_) => render_native_trial_balance_request_with_currency(
                                identity.display_name(),
                                &period,
                            ),
                            None => render_native_trial_balance_request(
                                identity.display_name(),
                                &period,
                            ),
                        };
                        let (xml, bytes, hash) = client
                            .fetch_native_report_paired(request.clone())
                            .await?
                            .require_stable(PairedReadValidationError::NativeLedgerCollection)?;
                        evidence = evidence
                            .clone()
                            .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                        let (report, ledger_scope) = match &base {
                            Some(identified) => {
                                let scoped = parse_native_trial_balance_with_currency(
                                    &xml,
                                    identity.company_guid(),
                                    identified.base(),
                                )?;
                                (
                                    scoped.report,
                                    TrialBalanceLedgerScope::BaseCurrencyLedgersOnly {
                                        base_name: identified.base().name().to_string(),
                                        decimal_places: identified.decimal_places(),
                                        foreign: scoped.foreign_currency_ledgers,
                                        mixed: scoped.mixed_currency_ledgers,
                                    },
                                )
                            }
                            None => (
                                parse_native_trial_balance(&xml, identity.company_guid())?,
                                TrialBalanceLedgerScope::AllLedgers,
                            ),
                        };
                        let totals = crate::reports::trial_balance::observed_totals(&report)?;
                        let sources = match extras {
                            TrialBalanceExtras::Nothing => None,
                            TrialBalanceExtras::CashFlow => {
                                let Some(window) = &cash_flow_window else {
                                    return Err(anyhow::anyhow!("cash_flow_window_not_built"));
                                };
                                let request =
                                    render_native_group_snapshot_request(identity.display_name());
                                let (xml, bytes, hash) = client
                                    .fetch_native_report_paired(request.clone())
                                    .await?
                                    .require_stable(PairedReadValidationError::NativeLedgerGroup)?;
                                evidence = evidence
                                    .clone()
                                    .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                                let groups =
                                    parse_native_group_snapshot(&xml, identity.company_guid())?;
                                let request = render_native_cash_flow_request(
                                    identity.display_name(),
                                    window,
                                );
                                let (xml, bytes, hash) = client
                                    .fetch_native_report_paired(request.clone())
                                    .await?
                                    .require_stable(PairedReadValidationError::NativeCashFlow)?;
                                evidence = evidence
                                    .clone()
                                    .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                                let cash_flow = parse_native_cash_flow(&xml, window)?;
                                // A several-currency book was refused above
                                // (`fetch_cash_flow` asks for the single-currency
                                // scope); a partial read never reaches a check.
                                let TrialBalanceAdmission::SingleInr(admission) = &admitted else {
                                    return Err(TrialBalanceReadError::Currency(
                                        "company_base_currency_undetermined",
                                    )
                                    .into());
                                };
                                Some(ExtraSources::CashFlow(CashFlowSources {
                                    trial_balance: SingleCurrencyTrialBalance::admitted(
                                        report.clone(),
                                        admission,
                                    ),
                                    groups,
                                    cash_flow,
                                }))
                            }
                            TrialBalanceExtras::Statement(kind) => {
                                let request =
                                    render_native_group_snapshot_request(identity.display_name());
                                let (xml, bytes, hash) = client
                                    .fetch_native_report_paired(request.clone())
                                    .await?
                                    .require_stable(PairedReadValidationError::NativeLedgerGroup)?;
                                evidence = evidence
                                    .clone()
                                    .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                                let groups =
                                    parse_native_group_snapshot(&xml, identity.company_guid())?;
                                // Tally's own Balance Sheet gates both tools; its own
                                // Profit and Loss is read only for a P&L's report.
                                let request = render_native_statement_request(
                                    NativeStatementKind::BalanceSheet,
                                    identity.display_name(),
                                    &period,
                                );
                                let (xml, bytes, hash) = client
                                    .fetch_native_report_paired(request.clone())
                                    .await?
                                    .require_stable(PairedReadValidationError::NativeStatement)?;
                                evidence = evidence
                                    .clone()
                                    .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                                let balance_sheet = parse_native_statement(
                                    NativeStatementKind::BalanceSheet,
                                    &xml,
                                )?;
                                let profit_and_loss = if kind == NativeStatementKind::ProfitAndLoss
                                {
                                    let request = render_native_statement_request(
                                        kind,
                                        identity.display_name(),
                                        &period,
                                    );
                                    let (xml, bytes, hash) = client
                                        .fetch_native_report_paired(request.clone())
                                        .await?
                                        .require_stable(
                                            PairedReadValidationError::NativeStatement,
                                        )?;
                                    evidence = evidence.clone().combine(
                                        RuntimeReadEvidence::paired(&request, hash, bytes),
                                    );
                                    Some(parse_native_statement(kind, &xml)?)
                                } else {
                                    None
                                };
                                // fetch_statements asks for the single-currency
                                // scope, so a several-currency book was refused
                                // above; a partial read never reaches a statement.
                                let TrialBalanceAdmission::SingleInr(admission) = &admitted else {
                                    return Err(TrialBalanceReadError::Currency(
                                        "company_base_currency_undetermined",
                                    )
                                    .into());
                                };
                                Some(ExtraSources::Statement(StatementSources {
                                    trial_balance: SingleCurrencyTrialBalance::admitted(
                                        report.clone(),
                                        admission,
                                    ),
                                    groups,
                                    balance_sheet,
                                    profit_and_loss,
                                }))
                            }
                        };
                        let closing_extent = client.fetch_company_book_extent(&identity).await?;
                        if closing_extent != extent {
                            return Err(PairedReadValidationError::NativeLedgerExtent.into());
                        }
                        bracket_verified_company_identity(&client, &identity).await?;
                        evidence = evidence
                            .clone()
                            .combine(confirm_read_boundary(&client, profile).await?);
                        Ok((
                            TrialBalanceRead {
                                company_guid: identity.company_guid().to_string(),
                                company_name: identity.display_name().to_string(),
                                from,
                                to,
                                currency,
                                report,
                                totals,
                                read_at: chrono::Utc::now().to_rfc3339(),
                                evidence: evidence.clone(),
                                ledger_scope,
                            },
                            sources,
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
