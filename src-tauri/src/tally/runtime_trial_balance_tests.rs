//! Captured native Trial Balance replay with one admission or stability fault at a time.
use super::*;
use tally_protocol_simulator::{
    encode, Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

pub(super) const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";

pub(super) fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

pub(super) fn companies() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ))
}

pub(super) fn extents() -> String {
    include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    )
    .to_string()
}

fn trial_balance() -> String {
    include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_known_lab.xml"
    )
    .to_string()
}

pub(super) fn identity() -> VerifiedCompanyIdentity {
    let companies = parse_companies_from_collection(&companies()).unwrap();
    let row = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(GUID))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        row.name.clone(),
        GUID.into(),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

pub(super) fn xml(text: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(text))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

pub(super) fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

pub(super) fn education(text: &str) -> String {
    text.replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    )
}

pub(super) fn pair(plans: &mut Vec<ScenarioPlan>, response: ScenarioPlan) {
    plans.extend([response.clone(), status(), response, status()]);
}

fn opening_plans(currency: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let extent = xml(extents());
    let mut plans = vec![status(), companies.clone(), companies.clone()];
    pair(&mut plans, extent);
    pair(&mut plans, xml(currency));
    plans
}

fn complete_plans(currency: String, report: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = opening_plans(currency);
    pair(&mut plans, xml(report));
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

pub(super) fn config(simulator: &SequenceSimulator) -> TallyConfig {
    TallyConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    }
}

fn join(left: &str, right: &str) -> String {
    sha256_hex(format!("{left}:{right}").as_bytes())
}

#[tokio::test]
async fn trial_balance_refuses_education_before_identity_or_report_dispatch() {
    let simulator = SequenceSimulator::spawn(vec![status(), xml(education(&companies()))]).unwrap();

    let error = TallyRuntime::default()
        .fetch_trial_balance(
            config(&simulator),
            &identity(),
            TrialBalancePeriod::new(
                TallyDate::parse("20260401").unwrap(),
                TallyDate::parse("20260902").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();

    assert!(matches!(
        error
            .chain()
            .find_map(|cause| cause.downcast_ref::<super::trial_balance::TrialBalanceReadError>()),
        Some(super::trial_balance::TrialBalanceReadError::EducationUnqualified)
    ));
    assert_eq!(simulator.finish().unwrap().len(), 2);
}

#[tokio::test]
async fn trial_balance_refuses_before_books_before_currency_or_report_dispatch() {
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    pair(&mut plans, xml(extents()));
    let simulator = SequenceSimulator::spawn(plans).unwrap();

    let error = TallyRuntime::default()
        .fetch_trial_balance(
            config(&simulator),
            &identity(),
            TrialBalancePeriod::new(
                TallyDate::parse("20250101").unwrap(),
                TallyDate::parse("20260902").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();

    assert!(matches!(
        error
            .chain()
            .find_map(|cause| cause.downcast_ref::<super::trial_balance::TrialBalanceReadError>()),
        Some(super::trial_balance::TrialBalanceReadError::BeforeBooks)
    ));
    assert_eq!(simulator.finish().unwrap().len(), 7);
}

#[tokio::test]
async fn trial_balance_rejects_non_inr_before_trial_balance_dispatch() {
    let captured = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let foreign = captured.replace(
        "<MAILINGNAME TYPE=\"String\">INR</MAILINGNAME>",
        "<MAILINGNAME TYPE=\"String\">USD</MAILINGNAME>",
    );
    assert_ne!(foreign, captured);
    let responses = opening_plans(foreign.clone())
        .iter()
        .map(|plan| encode(&plan.fixture.body(), plan.encoding))
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(opening_plans(foreign)).unwrap();

    let error = TallyRuntime::default()
        .fetch_trial_balance(
            config(&simulator),
            &identity(),
            TrialBalancePeriod::new(
                TallyDate::parse("20260401").unwrap(),
                TallyDate::parse("20260902").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();

    assert!(matches!(
        error
            .chain()
            .find_map(|cause| cause.downcast_ref::<super::trial_balance::TrialBalanceReadError>()),
        Some(super::trial_balance::TrialBalanceReadError::Currency(
            "company_base_currency_not_inr"
        ))
    ));
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    let observed = simulator.finish().unwrap();
    assert_eq!(
        observed.len(),
        11,
        "currency refusal must precede Trial Balance"
    );
    assert_eq!(
        evidence.request_sha256,
        join(
            &join(
                &observed[0].request_body_sha256,
                &observed[1].request_body_sha256
            ),
            &observed[7].request_body_sha256,
        )
    );
    assert_eq!(
        evidence.response_sha256,
        join(
            &join(&sha256_hex(&responses[0]), &sha256_hex(&responses[1])),
            &sha256_hex(&responses[7])
        )
    );
    assert_eq!(
        evidence.bytes,
        responses[0].len() + responses[1].len() + responses[7].len() * 2
    );
}

#[tokio::test]
async fn trial_balance_replays_captured_native_report_through_all_runtime_brackets() {
    let currency = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let simulator = SequenceSimulator::spawn(complete_plans(currency, trial_balance())).unwrap();

    let read = TallyRuntime::default()
        .fetch_trial_balance(
            config(&simulator),
            &identity(),
            TrialBalancePeriod::new(
                TallyDate::parse("20260401").unwrap(),
                TallyDate::parse("20260902").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(read.company_guid, GUID);
    assert_eq!(read.currency.mailing_name, "INR");
    assert!(!read.report.rows.is_empty());
    assert_eq!(simulator.finish().unwrap().len(), 22);
}

#[tokio::test]
async fn trial_balance_rejects_report_or_book_drift_and_retains_completed_source() {
    let currency = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let captured_report = trial_balance();
    let changed_extent = extents().replace(
        "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>",
        "<ALTMSTID TYPE=\"Number\"> 225</ALTMSTID>",
    );
    assert_ne!(changed_extent, extents());

    for fault in ["report", "extent"] {
        let mut plans = opening_plans(currency.clone());
        if fault == "report" {
            plans.extend([
                xml(captured_report.clone()),
                status(),
                xml(format!("{captured_report}\n")),
                status(),
            ]);
        } else {
            pair(&mut plans, xml(captured_report.clone()));
        }
        if fault == "extent" {
            pair(&mut plans, xml(changed_extent.clone()));
        }
        let responses = plans
            .iter()
            .map(|plan| encode(&plan.fixture.body(), plan.encoding))
            .collect::<Vec<_>>();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let error = TallyRuntime::default()
            .fetch_trial_balance(
                config(&simulator),
                &identity(),
                TrialBalancePeriod::new(
                    TallyDate::parse("20260401").unwrap(),
                    TallyDate::parse("20260902").unwrap(),
                )
                .unwrap(),
            )
            .await
            .unwrap_err();

        if fault == "report" {
            assert!(matches!(
                error
                    .chain()
                    .find_map(|cause| cause.downcast_ref::<PairedReadValidationError>()),
                Some(PairedReadValidationError::NativeLedgerCollection)
            ));
        } else {
            assert!(matches!(
                error
                    .chain()
                    .find_map(|cause| cause.downcast_ref::<PairedReadValidationError>()),
                Some(PairedReadValidationError::NativeLedgerExtent)
            ));
        }
        let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
        let observed = simulator.finish().unwrap();
        assert_eq!(observed.len(), if fault == "report" { 15 } else { 19 });
        let report_index = 11;
        let source_requests = join(
            &join(
                &observed[0].request_body_sha256,
                &observed[1].request_body_sha256,
            ),
            &observed[7].request_body_sha256,
        );
        let report_request = if fault == "report" {
            join(
                &observed[report_index].request_body_sha256,
                &observed[report_index + 2].request_body_sha256,
            )
        } else {
            observed[report_index].request_body_sha256.clone()
        };
        let expected_requests = join(&source_requests, &report_request);
        assert_eq!(evidence.request_sha256, expected_requests);
        let source_response = join(
            &join(&sha256_hex(&responses[0]), &sha256_hex(&responses[1])),
            &sha256_hex(&responses[7]),
        );
        let report_response = if fault == "report" {
            join(
                &sha256_hex(&responses[report_index]),
                &sha256_hex(&responses[report_index + 2]),
            )
        } else {
            sha256_hex(&responses[report_index])
        };
        let expected_response = join(&source_response, &report_response);
        assert_eq!(evidence.response_sha256, expected_response);
        assert_eq!(
            evidence.bytes,
            responses[0].len()
                + responses[1].len()
                + responses[7].len() * 2
                + responses[report_index].len() * if fault == "report" { 1 } else { 2 }
                + if fault == "report" {
                    responses[report_index + 2].len()
                } else {
                    0
                }
        );
    }
}

/// bridge#551 with #692: a statement is derived only from a single-currency
/// Trial Balance, so a several-currency book's Profit and Loss or Balance Sheet
/// read refuses after its currency read and sends nothing more: no base
/// identification, no Trial Balance, no group tree and no statement. A read
/// that asked for the base-currency ledgers here would send those.
#[tokio::test]
async fn a_statement_read_refuses_a_several_currency_book_after_its_currency_read() {
    let currency = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
    ));
    for kind in [
        bridge_tally_protocol::native_statement_reports::NativeStatementKind::BalanceSheet,
        bridge_tally_protocol::native_statement_reports::NativeStatementKind::ProfitAndLoss,
    ] {
        let plans = opening_plans(currency.clone());
        let total = plans.len();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let error = TallyRuntime::default()
            .fetch_statements(
                config(&simulator),
                &identity(),
                TrialBalancePeriod::new(
                    TallyDate::parse("20260401").unwrap(),
                    TallyDate::parse("20260902").unwrap(),
                )
                .unwrap(),
                kind,
            )
            .await
            .unwrap_err();
        assert!(matches!(
            error.chain().find_map(
                |cause| cause.downcast_ref::<super::trial_balance::TrialBalanceReadError>()
            ),
            Some(super::trial_balance::TrialBalanceReadError::Currency(
                "company_base_currency_undetermined"
            ))
        ));
        assert_eq!(simulator.finish().unwrap().len(), total);
    }
}

/// bridge#709: the desktop's read, which opts in to the base-currency
/// ledgers, reads a several-currency book's plain base-currency ledgers
/// through the base Tally identifies, sets the rest aside by name, and
/// presents its amounts in that base, not in the first master read. Captured
/// on one moment of one book (FOREX_601D_CAPTURE_PROVENANCE).
#[tokio::test]
async fn the_desktop_trial_balance_reads_a_several_currency_books_base_ledgers() {
    const FOREX: &str = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
    let forex = |bytes: &[u8]| xml(decode(bytes));
    let extent = forex(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
    ));
    let listing = xml(companies());
    let mut plans = vec![status(), listing.clone(), listing.clone()];
    pair(&mut plans, extent.clone());
    for source in [
        forex(include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
        )),
        forex(include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/currency_originalname_forex_live.utf16le.xml"
        )),
        forex(include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/company_currencyname_live.utf16le.xml"
        )),
        forex(include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/trial_balance_currency_forex_live.utf16le.xml"
        )),
    ] {
        pair(&mut plans, source);
    }
    pair(&mut plans, extent);
    plans.extend([listing.clone(), status(), listing]);
    let total = plans.len();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let observed = parse_companies_from_collection(&companies()).unwrap();
    let row = observed
        .iter()
        .find(|row| row.guid.as_deref() == Some(FOREX))
        .unwrap();
    let identity = VerifiedCompanyIdentity::from_observed_companies(
        row.name.clone(),
        FOREX.into(),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &observed,
    )
    .unwrap();
    let read = crate::commands::trial_balance::read_desktop_trial_balance(
        &TallyRuntime::default(),
        config(&simulator),
        &identity,
        TrialBalancePeriod::new(
            TallyDate::parse("20250401").unwrap(),
            TallyDate::parse("20260915").unwrap(),
        )
        .unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(simulator.finish().unwrap().len(), total);
    let TrialBalanceLedgerScope::BaseCurrencyLedgersOnly {
        base_name,
        foreign,
        mixed,
        ..
    } = &read.ledger_scope
    else {
        panic!("a several-currency book's read covers its base ledgers only");
    };
    assert_eq!(base_name, "I\u{20b9}");
    assert_eq!(foreign.len(), 3);
    assert!(foreign.iter().all(|ledger| ledger.currency == "$"));
    assert_eq!(mixed, &["FX Party 01", "FX Sales", "Profit & Loss A/c"]);
    assert_eq!(
        read.report
            .rows
            .iter()
            .map(|row| row.name.as_str())
            .collect::<Vec<_>>(),
        ["BRIDGE INR DEBTOR A", "Cash", "FX Party 02", "FX Party 03"]
    );
    assert_eq!(read.amount_currency().0, "I\u{20b9}");
    assert_ne!(read.currency.currency_count, 1);
}

/// bridge#551, #709: the default read, which every caller that cannot show
/// the ledgers left out uses (for example a statement derived from the Trial
/// Balance), still refuses a several-currency book after its currency read,
/// and sends nothing more. Only the MCP and desktop Trial Balance opt in.
#[tokio::test]
async fn the_default_trial_balance_read_still_refuses_a_several_currency_book() {
    let currency = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
    ));
    let plans = opening_plans(currency);
    let total = plans.len();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let error = TallyRuntime::default()
        .fetch_trial_balance(
            config(&simulator),
            &identity(),
            TrialBalancePeriod::new(
                TallyDate::parse("20260401").unwrap(),
                TallyDate::parse("20260902").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(
        error
            .chain()
            .find_map(|cause| cause.downcast_ref::<super::trial_balance::TrialBalanceReadError>()),
        Some(super::trial_balance::TrialBalanceReadError::Currency(
            "company_base_currency_undetermined"
        ))
    ));
    assert_eq!(simulator.finish().unwrap().len(), total);
}

const FOREX: &str = "b14e9b2d-8a63-4779-804d-25d59eb787eb";

fn forex_identity() -> VerifiedCompanyIdentity {
    let companies = parse_companies_from_collection(&companies()).unwrap();
    let row = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(FOREX))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        row.name.clone(),
        FOREX.into(),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

/// bridge#551: the opt-in read of a several-currency book's base-currency
/// ledgers refuses when Tally does not identify an INR base. Labelled edits of
/// the captured Company collection: its `CURRENCYNAME` naming the `$` master
/// identifies a base that is not INR, and one naming no master (`€`)
/// identifies none. Each refuses with its own typed error once the base is
/// identified, and sends no Trial Balance request.
#[tokio::test]
async fn the_base_ledgers_trial_balance_refuses_a_base_that_is_not_inr_or_not_identified() {
    let company = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/company_currencyname_live.utf16le.xml"
    ));
    let rupee = "<CURRENCYNAME TYPE=\"String\">\u{20b9}</CURRENCYNAME>";
    // The first rupee CURRENCYNAME is the FOREX company's own row.
    let first = company.find(rupee).unwrap();
    assert!(company.find("BRIDGE CORPUS FOREX").unwrap() < first);
    assert!(first < company.find("BRIDGE SHAPE LAB").unwrap());
    let naming = |value: &str| {
        company.replacen(
            rupee,
            &format!("<CURRENCYNAME TYPE=\"String\">{value}</CURRENCYNAME>"),
            1,
        )
    };
    for (value, code) in [
        ("$", "company_base_currency_not_inr"),
        ("\u{20ac}", "company_base_currency_undetermined"),
    ] {
        let companies = xml(companies());
        let mut plans = vec![status(), companies.clone(), companies];
        pair(
            &mut plans,
            xml(decode(include_bytes!(
                "../../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
            ))),
        );
        for currency in [
            decode(include_bytes!(
                "../../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
            )),
            decode(include_bytes!(
                "../../crates/bridge-tally-protocol/tests/fixtures/currency_originalname_forex_live.utf16le.xml"
            )),
            naming(value),
        ] {
            pair(&mut plans, xml(currency));
        }
        let total = plans.len();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let error = TallyRuntime::default()
            .fetch_trial_balance_with_extent(
                config(&simulator),
                &forex_identity(),
                TrialBalancePeriod::new(
                    TallyDate::parse("20260401").unwrap(),
                    TallyDate::parse("20260902").unwrap(),
                )
                .unwrap(),
                TrialBalanceCurrencyScope::BaseCurrencyLedgersOnly,
            )
            .await
            .unwrap_err();
        assert!(
            matches!(
                error.chain().find_map(
                    |cause| cause.downcast_ref::<super::trial_balance::TrialBalanceReadError>()
                ),
                Some(super::trial_balance::TrialBalanceReadError::Currency(found)) if *found == code
            ),
            "{value}: {error:#}"
        );
        assert_eq!(simulator.finish().unwrap().len(), total, "{value}");
    }
}
