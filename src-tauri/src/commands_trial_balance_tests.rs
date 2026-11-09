use super::*;
use bridge_tally_protocol::native_trial_balance::NativeTrialBalanceError;

#[test]
fn education_refusal_explains_the_report_qualification_limit() {
    let mapped = read_error(TrialBalanceReadError::EducationUnqualified.into());
    assert_eq!(mapped.code, "trial_balance_education_unqualified");
    assert!(mapped.message.contains("Education mode"));
    assert!(mapped.remediation.contains("Licensed TallyPrime"));
    assert!(!mapped.remediation.contains("day 1"));
}

#[test]
fn ordered_period_keeps_the_actionable_desktop_error() {
    let error = TrialBalancePeriod::new(
        TallyDate::parse("20260902").unwrap(),
        TallyDate::parse("20260401").unwrap(),
    )
    .unwrap_err();
    let mapped = read_error(error.into());
    assert_eq!(mapped.code, "trial_balance_period_invalid");
    assert!(mapped.message.contains("start date"));
    assert!(mapped.remediation.contains("valid date range"));
}

#[test]
fn native_trial_balance_errors_have_distinct_safe_desktop_remediation() {
    for (source, code, message_fragment, remediation_fragment) in [
        (
            NativeTrialBalanceError::TallyReportedFailure,
            "trial_balance_tally_rejected",
            "Tally rejected",
            "selected company and date range in Tally",
        ),
        (
            NativeTrialBalanceError::InvalidAmount,
            "trial_balance_amount_invalid",
            "amount ComplyEaze Bridge could not represent safely",
            "affected ledger amount in Tally",
        ),
        (
            NativeTrialBalanceError::InvalidResponse("trial_balance_xml_malformed"),
            "trial_balance_source_invalid",
            "response could not be represented safely",
            "Keep the selected company quiet",
        ),
    ] {
        let mapped = read_error(anyhow::Error::new(source));
        assert_eq!(mapped.code, code);
        assert!(mapped.message.contains(message_fragment));
        assert!(mapped.remediation.contains(remediation_fragment));
        assert!(!mapped.message.contains("trial_balance_xml_malformed"));
        assert!(!mapped.remediation.contains("trial_balance_xml_malformed"));
    }
}

#[test]
fn parent_list_request_refuses_unknown_fields_and_maps_expired_captures() {
    let request: TrialBalanceCaptureParentListRequest =
        serde_json::from_str(r#"{"export_id":"opaque","search":"debtor"}"#).unwrap();
    assert_eq!(request.export_id, "opaque");
    assert_eq!(request.search, "debtor");
    assert!(
        serde_json::from_str::<TrialBalanceCaptureParentListRequest>(
            r#"{"export_id":"opaque","search":"debtor","parent":"unexpected"}"#
        )
        .is_err()
    );

    let expired = parent_list_capture_error(
        crate::reports::trial_balance_store::TrialBalanceExportStoreError::InvalidOrExpired,
    );
    assert_eq!(expired.code, "trial_balance_capture_expired");
    assert!(expired.remediation.contains("Refresh the report"));
}

/// bridge#709: the desktop response carries the scope statement exactly when
/// the read covers a several-currency book's base-currency ledgers only.
#[test]
fn the_scope_statement_accompanies_only_a_base_ledgers_only_read() {
    use crate::reports::trial_balance_xlsx::tests::{captured_read, several_currency_read};
    assert_eq!(scope_limitation(&captured_read()), None);
    assert_eq!(
        scope_limitation(&several_currency_read()),
        Some(crate::tally::runtime::BASE_CURRENCY_LEDGERS_ONLY_LIMITATION)
    );
}

/// The whole desktop response for the non-integrated #2068 capture, as the webview receives it:
/// the read with its rows, the export handle and the limitation, pinned by length and SHA-256
/// (recorded from this response when the pin was added). The handle is fixed here; the store
/// makes its own.
#[test]
fn a_keyed_closing_stock_ledger_response_is_pinned_whole() {
    use sha2::Digest;
    let [(report, company), _] = crate::reports::trial_balance::tests::stock_ledger_captures();
    let mut read = crate::reports::trial_balance_xlsx::tests::captured_read();
    read.company_guid = company.into();
    read.currency.decimal_places = 2;
    read.totals = crate::reports::trial_balance::observed_totals(&report).unwrap();
    read.report = report;
    let response = trial_balance_response(read, "export-1".into());
    assert!(response.movements_differ_limitation.is_some());
    let bytes = serde_json::to_vec(&response).unwrap();
    let hex: String = sha2::Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(
        (bytes.len(), hex.as_str()),
        (
            3_630,
            "50e2aaa7aa9988f3bda4743c8daca16886eda30550552ef84d4877bb578bd3a1"
        )
    );
}
