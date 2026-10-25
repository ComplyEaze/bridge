//! Desktop adapters for one completed native Trial Balance observation.
use super::*;
use crate::reports::trial_balance_store::TrialBalanceExportStore;
use crate::reports::trial_balance_xlsx::render_trial_balance_xlsx;
use crate::tally::runtime::{TrialBalancePeriod, TrialBalanceRead, TrialBalanceReadError};
use bridge_tally_protocol::PartyLedgerMasterFieldObservation;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrialBalanceRequest {
    config: TallyConfig,
    selected_company: SelectedCompanyIdentity,
    from: TallyDate,
    to: TallyDate,
}

#[derive(Serialize)]
pub struct TrialBalanceResponse {
    read: TrialBalanceRead,
    export_id: String,
    /// Present only when the read covers a several-currency book's
    /// base-currency ledgers; the screen shows it with the ledgers left out
    /// (bridge#709).
    scope_limitation: Option<&'static str>,
}

fn scope_limitation(read: &TrialBalanceRead) -> Option<&'static str> {
    match read.ledger_scope {
        crate::tally::runtime::TrialBalanceLedgerScope::AllLedgers => None,
        crate::tally::runtime::TrialBalanceLedgerScope::BaseCurrencyLedgersOnly { .. } => {
            Some(crate::tally::runtime::BASE_CURRENCY_LEDGERS_ONLY_LIMITATION)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrialBalanceCaptureParentQueryRequest {
    export_id: String,
    parent: PartyLedgerMasterFieldObservation,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrialBalanceCaptureParentListRequest {
    export_id: String,
    search: String,
}

#[derive(Serialize)]
pub struct TrialBalanceCaptureProvenance {
    company_guid: String,
    company_name: String,
    from: TallyDate,
    to: TallyDate,
    read_at: String,
    request_sha256: String,
    response_sha256: String,
    source_bytes: usize,
    expires_in_seconds: u64,
}

#[derive(Serialize)]
pub struct TrialBalanceCaptureParentQueryResponse {
    query: crate::reports::trial_balance::TrialBalanceParentQuery,
    capture: TrialBalanceCaptureProvenance,
}

fn local_error(code: &'static str, message: &str, remediation: &'static str) -> TallyCommandError {
    tally_command_error(
        code,
        "Trial Balance",
        message,
        "after_change",
        false,
        remediation,
    )
}

fn parent_list_capture_error(
    error: crate::reports::trial_balance_store::TrialBalanceExportStoreError,
) -> TallyCommandError {
    match error {
        crate::reports::trial_balance_store::TrialBalanceExportStoreError::InvalidOrExpired => {
            local_error(
                "trial_balance_capture_expired",
                "This captured Trial Balance is no longer available for a follow-up query.",
                "Refresh the report before selecting a parent again.",
            )
        }
        crate::reports::trial_balance_store::TrialBalanceExportStoreError::ResourceLimit
        | crate::reports::trial_balance_store::TrialBalanceExportStoreError::Unavailable => {
            local_error(
                "trial_balance_capture_unavailable",
                "ComplyEaze Bridge could not access the retained Trial Balance safely.",
                "Refresh the report before selecting a parent again.",
            )
        }
    }
}

fn read_error(error: anyhow::Error) -> TallyCommandError {
    if let Some(reason) = error
        .chain()
        .find_map(|cause| cause.downcast_ref::<TrialBalanceReadError>())
    {
        if matches!(
            reason,
            TrialBalanceReadError::Period(
                bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriodError::InvalidRange
            )
        ) {
            return local_error(
                "trial_balance_period_invalid",
                "The start date must be on or before the end date.",
                "Choose a valid date range and refresh the report.",
            );
        }
        if matches!(reason, TrialBalanceReadError::EducationUnqualified) {
            return local_error(reason.safe_code(), "Native Trial Balance is not yet qualified for Education mode.",
                "This report currently requires observed Licensed TallyPrime. Education support needs further qualification.");
        }
        return local_error(reason.safe_code(), "ComplyEaze Bridge could not admit this Trial Balance period or currency.",
            "Choose dates on or after book start. This report requires an INR base currency that Tally identifies.");
    }
    if let Some(reason) = error.chain().find_map(|cause| {
        cause.downcast_ref::<bridge_tally_protocol::native_trial_balance::NativeTrialBalanceError>()
    }) {
        return match reason {
            bridge_tally_protocol::native_trial_balance::NativeTrialBalanceError::TallyReportedFailure => local_error(
                "trial_balance_tally_rejected",
                "Tally rejected the Trial Balance request.",
                "Confirm the selected company and date range in Tally, then retry the report.",
            ),
            bridge_tally_protocol::native_trial_balance::NativeTrialBalanceError::InvalidAmount => local_error(
                "trial_balance_amount_invalid",
                "Tally returned a Trial Balance amount ComplyEaze Bridge could not represent safely.",
                "Keep the selected company quiet and retry the report. If it persists, review the affected ledger amount in Tally.",
            ),
            bridge_tally_protocol::native_trial_balance::NativeTrialBalanceError::InvalidResponse(_) => local_error(
                "trial_balance_source_invalid",
                "Tally's Trial Balance response could not be represented safely.",
                "Keep the selected company quiet, confirm the date range and retry the read.",
            ),
        };
    }
    tally_runtime_command_error(error)
}

#[cfg(test)]
#[path = "commands_trial_balance_tests.rs"]
mod tests;

/// The desktop's Trial Balance read. The screen and its workbook show the
/// ledgers a several-currency book's read leaves out, so it asks for the
/// base-currency ledgers explicitly (bridge#709); every other caller keeps
/// the default refusal.
pub(crate) async fn read_desktop_trial_balance(
    runtime: &TallyRuntime,
    config: TallyConfig,
    identity: &VerifiedCompanyIdentity,
    period: TrialBalancePeriod,
) -> anyhow::Result<TrialBalanceRead> {
    runtime
        .fetch_trial_balance_with_extent(
            config,
            identity,
            period,
            crate::tally::runtime::TrialBalanceCurrencyScope::BaseCurrencyLedgersOnly,
        )
        .await
        .map(|(read, _)| read)
}

#[tauri::command]
pub async fn fetch_tally_trial_balance(
    request: TrialBalanceRequest,
    runtime: State<'_, TallyRuntime>,
    exports: State<'_, TrialBalanceExportStore>,
) -> Result<TrialBalanceResponse, TallyCommandError> {
    let period = TrialBalancePeriod::new(request.from, request.to)
        .map_err(|error| read_error(error.into()))?;
    exports.clear().map_err(|_| {
        local_error(
            "trial_balance_export_unavailable",
            "ComplyEaze Bridge could not reset the previous export.",
            "Restart ComplyEaze Bridge, then refresh the report.",
        )
    })?;
    let identity =
        verify_observed_company_tuple(&runtime, &request.config, &request.selected_company).await?;
    let read = read_desktop_trial_balance(&runtime, request.config, &identity, period)
        .await
        .map_err(read_error)?;
    let scope_limitation = scope_limitation(&read);
    let export_id = exports.insert(read.clone()).map_err(|_| {
        local_error(
            "trial_balance_export_budget",
            "The captured Trial Balance exceeds the local export budget.",
            "Review the selected company and retry with a smaller supported source.",
        )
    })?;
    Ok(TrialBalanceResponse {
        read,
        export_id,
        scope_limitation,
    })
}

/// The webview sends only an opaque handle; no amounts, rows or Tally request
/// can enter this formatting-only operation.
#[tauri::command]
pub async fn export_tally_trial_balance(
    app: tauri::AppHandle,
    export_id: String,
    exports: State<'_, TrialBalanceExportStore>,
) -> Result<String, TallyCommandError> {
    let read = exports.get(&export_id).map_err(|_| {
        local_error(
            "trial_balance_export_expired",
            "This captured Trial Balance is no longer available for export.",
            "Refresh the report, then export the newly captured result.",
        )
    })?;
    let mut slug = statement_filename_slug(&read.company_name);
    slug.truncate(150);
    let filename = format!("trial-balance-{slug}-{}.xlsx", read.to.as_str());
    let bytes = tauri::async_runtime::spawn_blocking(move || render_trial_balance_xlsx(&read)).await
        .map_err(|_| local_error("trial_balance_export_failed", "ComplyEaze Bridge could not finish the workbook.", "Retry the export."))?
        .map_err(|_| local_error("trial_balance_export_failed", "ComplyEaze Bridge could not represent this workbook safely.", "Review the captured report; amounts that exceed Excel precision cannot be exported as numbers."))?;
    save_report_download_bytes(&app, &filename, &bytes).map_err(|_| {
        local_error(
            "trial_balance_save_failed",
            "ComplyEaze Bridge could not save the workbook to Downloads.",
            "Check Downloads-folder access and retry the export.",
        )
    })
}

/// Derives a bounded parent subset from one already captured report. This
/// command cannot acquire Tally data, accept a company identity, or write.
#[tauri::command]
pub async fn query_tally_trial_balance_capture_parent(
    request: TrialBalanceCaptureParentQueryRequest,
    exports: State<'_, TrialBalanceExportStore>,
) -> Result<TrialBalanceCaptureParentQueryResponse, TallyCommandError> {
    let capture = exports.get_capture(&request.export_id).map_err(|_| {
        local_error(
            "trial_balance_capture_expired",
            "This captured Trial Balance is no longer available for a follow-up query.",
            "Refresh the report before selecting a parent again.",
        )
    })?;
    let read = std::sync::Arc::clone(&capture.read);
    let query = tauri::async_runtime::spawn_blocking(move || {
        crate::reports::trial_balance::query_observed_parent(&read.report, &request.parent)
    })
    .await
    .map_err(|_| {
        local_error(
            "trial_balance_capture_parent_query_failed",
            "ComplyEaze Bridge could not derive rows from the retained Trial Balance.",
            "Select the parent again or refresh the report.",
        )
    })?
    .map_err(|error| match error {
        crate::reports::trial_balance::TrialBalanceParentQueryError::ParentNotInCapture => {
            local_error(
                "trial_balance_capture_parent_unavailable",
                "That exact parent was not returned by the retained capture.",
                "Select a parent returned by this capture or refresh the report.",
            )
        }
        crate::reports::trial_balance::TrialBalanceParentQueryError::TotalsUnavailable => {
            local_error(
                "trial_balance_capture_invalid",
                "The retained Trial Balance cannot be summarized safely.",
                "Refresh the report before selecting a parent again.",
            )
        }
    })?;
    Ok(TrialBalanceCaptureParentQueryResponse {
        query,
        capture: TrialBalanceCaptureProvenance {
            company_guid: capture.read.company_guid.clone(),
            company_name: capture.read.company_name.clone(),
            from: capture.read.from.clone(),
            to: capture.read.to.clone(),
            read_at: capture.read.read_at.clone(),
            request_sha256: capture.read.evidence.request_sha256.clone(),
            response_sha256: capture.read.evidence.response_sha256.clone(),
            source_bytes: capture.read.evidence.bytes,
            expires_in_seconds: capture.expires_in.as_secs(),
        },
    })
}

/// Lists a bounded set of parent observations from one already retained Trial
/// Balance. The opaque handle is resolved locally before an off-thread scan;
/// this command cannot acquire Tally data, accept an identity, or write.
#[tauri::command]
pub async fn list_tally_trial_balance_capture_parents(
    request: TrialBalanceCaptureParentListRequest,
    exports: State<'_, TrialBalanceExportStore>,
) -> Result<crate::reports::trial_balance::TrialBalanceCaptureParentOptions, TallyCommandError> {
    let capture = exports
        .get_capture(&request.export_id)
        .map_err(parent_list_capture_error)?;
    let read = capture.read;
    tauri::async_runtime::spawn_blocking(move || {
        crate::reports::trial_balance::list_observed_capture_parents(&read.report, request.search)
    })
    .await
    .map_err(|_| {
        local_error(
            "trial_balance_capture_parent_list_failed",
            "ComplyEaze Bridge could not scan the retained Trial Balance.",
            "Refresh the report before selecting a parent again.",
        )
    })?
    .map_err(|error| match error {
        crate::reports::trial_balance::TrialBalanceCaptureParentOptionsError::SearchInvalid => {
            local_error(
                "trial_balance_capture_parent_search_invalid",
                "That parent search is not valid for this retained capture.",
                "Use a shorter parent name or clear the search.",
            )
        }
    })
}
