//! Captured source replies prove queued import admission without a Tally write.
use super::*;
use crate::tally::approved_import::QueuedAdmission;
use crate::tally::{
    agent_read_request::AgentReadRequest,
    approved_import::{ApprovedImport, ApprovedImportAdmissionError},
};
use bridge_tally_protocol::{
    parse_standard_ledger_catalog_with_identities,
    xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";

fn captured_companies() -> String {
    let bytes = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The captured Currency masters of a book with exactly one (`I₹`).
fn captured_single_currency() -> String {
    let bytes = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn captured_catalogue() -> String {
    let bytes = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn company_plan(xml: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(xml))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn status_plan() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
        .with_framing(ResponseFraming::ContentLength)
}

fn captured_journal() -> String {
    let bytes = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn identity(xml: &str) -> VerifiedCompanyIdentity {
    let companies = parse_companies_from_collection(xml).unwrap();
    let company = companies
        .iter()
        .find(|company| company.guid.as_deref() == Some(GUID))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        company.name.clone(),
        GUID.into(),
        company.company_number.clone().unwrap(),
        company.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

fn approved_import(companies: &str, date: &str) -> ApprovedImport {
    let identity = identity(companies);
    let catalogue = parse_standard_ledger_catalog_with_identities(
        &captured_catalogue(),
        identity.display_name(),
        identity.company_guid(),
    )
    .expect("captured catalog is admitted for the captured company");
    let selected = catalogue
        .names()
        .next()
        .expect("captured catalog has a ledger")
        .to_string();
    let binding = catalogue
        .bind_selected([selected])
        .expect("selected ledger is bound to its observed GUID");
    let company = ValidatedCompanyName::new(identity.display_name().to_string()).unwrap();
    let ledger_catalogue_request = AgentReadRequest::parse(
        ReadOnlyProfile::StandardLedgerCatalogV1 { company: &company }.render(),
    )
    .expect("static catalog read is admitted");
    let currency_request = AgentReadRequest::parse(
        bridge_tally_protocol::native_outstandings::render_company_currency_request(
            identity.display_name(),
        ),
    )
    .expect("currency read is admitted");
    // The all-company marks, in the agent's high-water collection shape: a
    // request no other queue read sends, so a test can tell where it went. The
    // queue sends it as the binding reads begin (#239), last before the POST,
    // and once after (#574).
    let company_marks_request = AgentReadRequest::parse(format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Agent Company High Water\" ISMODIFY=\"No\"><TYPE>Company</TYPE><FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        identity.display_name()
    ))
    .expect("marks read is admitted");
    ApprovedImport::approved_for_test(
        "<ENVELOPE/>".into(),
        bridge_tally_core::TallyDate::parse(date).unwrap(),
        ledger_catalogue_request,
        binding,
        currency_request,
        company_marks_request,
    )
}

/// Each mode probe is status/company, followed by explicit company admission.
/// Each identity-bracketed source read contains two reports and status checks.
fn queued_plans(
    opening_companies: String,
    verification: String,
    catalogue: String,
    final_companies: String,
    post_response: Option<String>,
) -> Vec<ScenarioPlan> {
    let paired = |body: String, companies: &String| {
        vec![
            company_plan(companies.clone()),
            company_plan(body.clone()),
            status_plan(),
            company_plan(body),
            status_plan(),
            company_plan(companies.clone()),
        ]
    };
    // Product/mode observation itself reads the Company collection, followed by
    // the explicit opening company-scope admission.
    let mut plans = vec![
        status_plan(),
        company_plan(opening_companies.clone()),
        company_plan(opening_companies.clone()),
        // The all-company marks as the binding reads begin (#239).
        company_plan(opening_companies.clone()),
    ];
    plans.extend(paired(catalogue, &opening_companies));
    // The Currency masters, re-read in the same brackets (bridge#551).
    plans.extend(paired(captured_single_currency(), &opening_companies));
    plans.extend([
        status_plan(),
        company_plan(final_companies.clone()),
        company_plan(final_companies),
    ]);
    plans.extend(paired(verification.clone(), &opening_companies));
    plans.extend(paired(verification, &opening_companies));
    // The all-company marks, the last Tally request before the POST (#574).
    plans.push(company_plan(opening_companies.clone()));
    if let Some(response) = post_response {
        plans.push(company_plan(response));
    }
    plans
}

fn paired_observation(
    observed: &[tally_protocol_simulator::ObservedRequest],
    responses: &[Vec<u8>],
    index: usize,
) -> RuntimeReadEvidence {
    RuntimeReadEvidence {
        request_sha256: observed[index].request_body_sha256.clone(),
        response_sha256: sha256_hex(&responses[index]),
        bytes: responses[index].len().saturating_mul(2),
    }
}

fn single_observation(
    observed: &[tally_protocol_simulator::ObservedRequest],
    responses: &[Vec<u8>],
    index: usize,
) -> RuntimeReadEvidence {
    RuntimeReadEvidence {
        request_sha256: observed[index].request_body_sha256.clone(),
        response_sha256: sha256_hex(&responses[index]),
        bytes: responses[index].len(),
    }
}

fn expected_queued_evidence(
    observed: &[tally_protocol_simulator::ObservedRequest],
    responses: &[Vec<u8>],
    include_absence_reads: bool,
) -> RuntimeReadEvidence {
    let mut evidence = single_observation(observed, responses, 0)
        .combine(single_observation(observed, responses, 1))
        .combine(single_observation(observed, responses, 2))
        .combine(single_observation(observed, responses, 3));
    evidence = evidence
        .combine(paired_observation(observed, responses, 5))
        .combine(paired_observation(observed, responses, 11));
    let closing = single_observation(observed, responses, 16)
        .combine(single_observation(observed, responses, 17));
    evidence = evidence
        .combine(closing)
        .combine(single_observation(observed, responses, 18));
    if include_absence_reads {
        for index in [20, 26] {
            evidence = evidence.combine(paired_observation(observed, responses, index));
        }
    }
    evidence
}

#[tokio::test]
async fn queued_education_change_refuses_before_final_absence_reads() {
    let companies = captured_companies();
    let education = companies.replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    assert_ne!(
        education, companies,
        "fixture mutation is explicit, not live evidence"
    );
    let mut plans = queued_plans(
        companies.clone(),
        companies.clone(),
        captured_catalogue(),
        education,
        None,
    );
    plans.truncate(19);
    let responses = plans
        .iter()
        .map(ScenarioPlan::response_bytes)
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let dispatched = Arc::new(AtomicBool::new(false));
    let guard = dispatched.clone();
    let result = runtime
        .post_approved_import(
            TallyConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            &identity(&companies),
            approved_import(&companies, "20260915"),
            |_: QueuedAdmission<'_>| Ok(()),
            move || {
                guard.store(true, Ordering::Release);
                Ok(())
            },
        )
        .await;
    let error = result.expect_err("final Education boundary must refuse an unsupported date");
    assert!(error.chain().any(|cause| matches!(
        cause.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::EducationVoucherDateUnsupported)
    )));
    assert!(
        !dispatched.load(Ordering::Acquire),
        "refusal precedes durable intent"
    );
    let observed = simulator.finish().unwrap();
    assert_eq!(
        observed.len(),
        19,
        "final mode refusal precedes absence reads"
    );
    assert_eq!(
        error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence,
        expected_queued_evidence(&observed, &responses, false),
        "initial Licensed, catalogue and final Education observations survive refusal"
    );
}

#[tokio::test]
async fn queued_company_refusal_retains_captured_source_and_final_identity_evidence() {
    let companies = captured_companies();
    let replaced = companies.replacen(
        &format!("<GUID TYPE=\"String\">{GUID}</GUID>"),
        "<GUID TYPE=\"String\">71c6de69-1748-461c-ad3f-162cb949df9f</GUID>",
        1,
    );
    assert_ne!(replaced, companies);
    let mut plans = queued_plans(
        companies.clone(),
        companies.clone(),
        captured_catalogue(),
        replaced,
        None,
    );
    plans.truncate(19);
    let responses = plans
        .iter()
        .map(ScenarioPlan::response_bytes)
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let dispatched = Arc::new(AtomicBool::new(false));
    let guard = dispatched.clone();
    let result = runtime
        .post_approved_import(
            TallyConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            &identity(&companies),
            approved_import(&companies, "20260901"),
            |_: QueuedAdmission<'_>| Ok(()),
            move || {
                guard.store(true, Ordering::Release);
                Ok(())
            },
        )
        .await;
    let error = result.expect_err("replaced final company identity must refuse");
    assert!(error.chain().any(|cause| matches!(
        cause.downcast_ref::<CompanyIdentityBracketError>(),
        Some(CompanyIdentityBracketError::AbsentOrAmbiguous)
    )));
    assert!(
        !dispatched.load(Ordering::Acquire),
        "refusal precedes durable intent"
    );
    let observed = simulator.finish().unwrap();
    assert_eq!(
        observed.len(),
        19,
        "final identity refusal precedes absence reads"
    );
    assert_eq!(
        error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence,
        expected_queued_evidence(&observed, &responses, false),
    );
}

#[tokio::test]
async fn queued_catalogue_rename_refuses_before_intent_or_post() {
    let companies = captured_companies();
    let original_catalogue = captured_catalogue();
    let initial = parse_standard_ledger_catalog_with_identities(
        &original_catalogue,
        identity(&companies).display_name(),
        GUID,
    )
    .expect("captured catalog parses");
    let selected = initial.names().next().unwrap().to_string();
    let renamed = original_catalogue.replacen(
        &format!(r#"NAME="{selected}""#),
        r#"NAME="Bridge queued rename regression""#,
        1,
    );
    assert_ne!(
        renamed, original_catalogue,
        "captured metadata mutation is explicit"
    );
    let plans = queued_plans(
        companies.clone(),
        companies.clone(),
        renamed,
        companies.clone(),
        None,
    );
    let responses = plans
        .iter()
        .map(ScenarioPlan::response_bytes)
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let dispatched = Arc::new(AtomicBool::new(false));
    let guard = dispatched.clone();
    let error = runtime
        .post_approved_import(
            TallyConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            &identity(&companies),
            approved_import(&companies, "20260901"),
            move |queued: QueuedAdmission<'_>| {
                assert!(!queued
                    .ledger_binding
                    .matches(queued.catalogue, identity(&companies).display_name(), GUID)
                    .expect("renamed captured catalog remains structurally valid"));
                Err(ApprovedImportAdmissionError::LedgerIdentityChanged.into())
            },
            move || {
                guard.store(true, Ordering::Release);
                Ok(())
            },
        )
        .await
        .expect_err("queued ledger rename must refuse");
    assert!(error.chain().any(|cause| matches!(
        cause.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::LedgerIdentityChanged)
    )));
    assert!(
        !dispatched.load(Ordering::Acquire),
        "changed master binding precedes intent"
    );
    let observed = simulator.finish().unwrap();
    // 31 admission requests and the marks snapshot (#574); the refusal
    // comes after that last read and before the intent and the POST.
    assert_eq!(
        observed.len(),
        32,
        "catalogue refusal precedes intent and POST"
    );
    assert_eq!(
        error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence,
        expected_queued_evidence(&observed, &responses, true)
            .combine(single_observation(&observed, &responses, 31)),
        "captured queued absence and catalogue evidence survive the master-binding refusal, with the marks snapshot"
    );
}

#[tokio::test]
async fn queued_import_keeps_admission_separate_from_raw_import_wire() {
    let companies = captured_companies();
    // There is no captured Journal-import response fixture. This runtime records
    // raw bytes only, so replay a captured Company body rather than inventing success.
    let plans = queued_plans(
        companies.clone(),
        companies.clone(),
        captured_catalogue(),
        companies.clone(),
        Some(companies.clone()),
    );
    let responses = plans
        .iter()
        .map(ScenarioPlan::response_bytes)
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let dispatched = Arc::new(AtomicBool::new(false));
    let guard = dispatched.clone();
    let dispatch = runtime
        .post_approved_import(
            TallyConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            &identity(&companies),
            approved_import(&companies, "20260901"),
            |_: QueuedAdmission<'_>| Ok(()),
            move || {
                guard.store(true, Ordering::Release);
                Ok(())
            },
        )
        .await
        .expect("captured admission permits a valid date");
    assert!(dispatched.load(Ordering::Acquire));
    let observed = simulator.finish().unwrap();
    // 31 admission requests, the marks snapshot, then the POST. The marks read
    // after the POST is the caller's, once the response is journaled.
    assert_eq!(observed.len(), 33);
    // The binding-time snapshot and the aim snapshot are the marks request,
    // and no other request the queue sends is (#239).
    let marks_at = observed
        .iter()
        .enumerate()
        .filter(|(_, request)| request.request_body_sha256 == observed[3].request_body_sha256)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    assert_eq!(marks_at, [3, 31]);
    assert_eq!(
        dispatch.admission_evidence,
        expected_queued_evidence(&observed, &responses, true)
            .combine(single_observation(&observed, &responses, 31))
    );
    assert_eq!(dispatch.company_marks_before, companies);
    assert_eq!(
        dispatch.response_evidence.request_sha256,
        observed[32].request_body_sha256
    );
    assert_eq!(
        dispatch.response_evidence.response_sha256,
        sha256_hex(&responses[32])
    );
    assert_eq!(dispatch.response_evidence.bytes, responses[32].len());
    assert_ne!(
        dispatch.response_evidence.request_sha256,
        dispatch.admission_evidence.request_sha256
    );
}

#[tokio::test]
async fn queued_import_refuses_attribution_after_final_profile_and_catalogue_reads() {
    let companies = captured_companies();
    let journal = captured_journal();
    let plans = queued_plans(
        companies.clone(),
        journal.clone(),
        captured_catalogue(),
        companies.clone(),
        None,
    );
    let responses = plans
        .iter()
        .map(ScenarioPlan::response_bytes)
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let dispatched = Arc::new(AtomicBool::new(false));
    let guard = dispatched.clone();
    let result = runtime
        .post_approved_import(
            TallyConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            &identity(&companies),
            approved_import(&companies, "20260901"),
            move |queued: QueuedAdmission<'_>| {
                assert_eq!(queued.first, journal.as_str());
                assert_eq!(queued.second, journal.as_str());
                Err(ApprovedImportAdmissionError::PreexistingIdentity.into())
            },
            move || {
                guard.store(true, Ordering::Release);
                Ok(())
            },
        )
        .await;
    let error = result.expect_err("queue recheck must refuse the attributed Journal");
    assert!(error.chain().any(|cause| matches!(
        cause.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::PreexistingIdentity)
    )));
    assert!(
        !dispatched.load(Ordering::Acquire),
        "refusal precedes durable intent"
    );
    let observed = simulator.finish().unwrap();
    // 31 admission requests and the marks snapshot (#574); the refusal
    // comes after that last read and before the intent and the POST.
    assert_eq!(
        observed.len(),
        32,
        "final absence refusal precedes intent and import POST"
    );
    assert_eq!(
        error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence,
        expected_queued_evidence(&observed, &responses, true)
            .combine(single_observation(&observed, &responses, 31)),
        "initial mode/company and all three queued source reads survive refusal, with the marks snapshot"
    );
}

/// #697: a post whose wire lock is taken is refused at once, without waiting
/// between its aim recheck and its attempt record, before the attempt is
/// recorded, and the import is never sent.
#[tokio::test]
async fn a_post_refused_by_a_busy_wire_lock_records_nothing_and_sends_nothing() {
    use bridge_tally_transport::{WireRefusal, WireRetryPolicy};
    let companies = captured_companies();
    let plans = queued_plans(
        companies.clone(),
        companies.clone(),
        captured_catalogue(),
        companies.clone(),
        None,
    );
    let queue_reads = plans.len();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let wire = WireGateConfig::default().with_retry(
        WireRetryPolicy::new(
            std::time::Duration::from_millis(10),
            std::time::Duration::from_secs(5),
        )
        .unwrap(),
    );
    let runtime = TallyRuntime::default().with_wire_gate_config(wire.clone());
    let config = TallyConfig {
        host: "127.0.0.1".into(),
        port: simulator.address().port(),
    };
    let lock_path = crate::endpoint_wire::wire_lock_path(wire.root(), &config);
    // Another holder takes the wire lock after the queue's last read, between
    // the admission recheck and the attempt's record, and keeps it.
    let other_holder = std::sync::Mutex::new(None);
    let dispatched = Arc::new(AtomicBool::new(false));
    let guard = dispatched.clone();
    let started = std::time::Instant::now();
    let result = runtime
        .post_approved_import(
            config,
            &identity(&companies),
            approved_import(&companies, "20260901"),
            |_: QueuedAdmission<'_>| {
                let file = crate::local_files::file::open_local_file(&lock_path, true)?;
                file.try_lock()
                    .map_err(|_| anyhow::anyhow!("test holder could not take the wire lock"))?;
                *other_holder.lock().unwrap() = Some(file);
                Ok(())
            },
            move || {
                guard.store(true, Ordering::Release);
                Ok(())
            },
        )
        .await;
    let error = result.expect_err("the wire lock was never free");
    // A five-second budget is available and none of it is used: the refusal
    // is one try, not a wait that happens to run out.
    assert!(
        started.elapsed() < std::time::Duration::from_secs(2),
        "the post's wire lock waited"
    );
    assert!(other_holder.lock().unwrap().is_some(), "the recheck ran");
    assert!(error
        .chain()
        .any(|cause| cause.is::<crate::tally::approved_import::PreIntentQueueRefusal>()));
    assert!(error.chain().any(|cause| matches!(
        cause.downcast_ref::<TallyTransportError>(),
        Some(TallyTransportError::WireRefused {
            refusal: WireRefusal::Busy
        })
    )));
    assert!(
        !dispatched.load(Ordering::Acquire),
        "refused before the attempt is recorded"
    );
    // Every queue read, and no import POST.
    assert_eq!(simulator.finish().unwrap().len(), queue_reads);
}

/// #697: the post's wire lock is held across the attempt's record and
/// released once its one send has been read.
#[tokio::test]
async fn the_wire_lock_is_held_across_the_record_and_released_after_the_post() {
    let companies = captured_companies();
    let plans = queued_plans(
        companies.clone(),
        companies.clone(),
        captured_catalogue(),
        companies.clone(),
        Some(companies.clone()),
    );
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let config = TallyConfig {
        host: "127.0.0.1".into(),
        port: simulator.address().port(),
    };
    let lock_path =
        crate::endpoint_wire::wire_lock_path(runtime.wire_gate_config().root(), &config);
    let held_at_record = Arc::new(AtomicBool::new(false));
    let seen = held_at_record.clone();
    let probe_path = lock_path.clone();
    runtime
        .post_approved_import(
            config,
            &identity(&companies),
            approved_import(&companies, "20260901"),
            |_: QueuedAdmission<'_>| Ok(()),
            move || {
                let probe = crate::local_files::file::open_local_file(&probe_path, true).map_err(
                    |error| {
                        crate::tally::approved_import::BeforeDispatchError::Other(error.to_string())
                    },
                )?;
                seen.store(
                    matches!(probe.try_lock(), Err(std::fs::TryLockError::WouldBlock)),
                    Ordering::Release,
                );
                Ok(())
            },
        )
        .await
        .expect("a free wire lock posts");
    assert!(
        held_at_record.load(Ordering::Acquire),
        "the wire lock is held while the attempt is recorded"
    );
    let after = crate::local_files::file::open_local_file(&lock_path, true).unwrap();
    after
        .try_lock()
        .expect("the lock is released once the one send has been read");
    assert_eq!(simulator.finish().unwrap().len(), 33);
}

/// #778: the reads before the intent stop for a withdrawn call, but the POST
/// does not. A withdrawal that lands as the intent is recorded, here from the
/// recording callback itself, still sends the POST, so an intent is never left
/// without its send.
#[tokio::test]
async fn a_withdrawal_as_the_intent_is_recorded_still_sends_the_post() {
    let companies = captured_companies();
    let plans = queued_plans(
        companies.clone(),
        companies.clone(),
        captured_catalogue(),
        companies.clone(),
        Some(companies.clone()),
    );
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let withdrawal = CancellationToken::new();
    let withdraw = withdrawal.clone();
    TOOL_CANCELLATION
        .scope(
            withdrawal,
            runtime.post_approved_import(
                TallyConfig {
                    host: "127.0.0.1".into(),
                    port: simulator.address().port(),
                },
                &identity(&companies),
                approved_import(&companies, "20260901"),
                |_: QueuedAdmission<'_>| Ok(()),
                move || {
                    withdraw.cancel();
                    Ok(())
                },
            ),
        )
        .await
        .expect("a post whose intent was recorded is sent");
    // Every queue read, then the POST.
    assert_eq!(simulator.finish().unwrap().len(), 33);
}

/// A pre-intent read refused by the wire gate stays the refusal, with its retry
/// time and "nothing sent"; any other failed read is the typed unconfirmed
/// refusal (#697).
#[test]
fn a_wire_refused_pre_intent_read_stays_the_wire_refusal() {
    use crate::tally::approved_import::ApprovedImportAdmissionError as Unconfirmed;
    use bridge_tally_transport::WireRefusal;
    let refused = anyhow::Error::new(TallyTransportError::WireRefused {
        refusal: WireRefusal::Busy,
    });
    let kept = unconfirmed_unless_wire_refused(refused, Unconfirmed::MastersUnconfirmed);
    assert_eq!(
        crate::endpoint_wire::wire_refusal(&kept),
        Some(WireRefusal::Busy)
    );
    assert!(!kept.chain().any(|cause| cause.is::<Unconfirmed>()));
    // A send the call withdrew before it started stays that withdrawal (#778).
    let withdrawn = unconfirmed_unless_wire_refused(
        anyhow::Error::new(TallyTransportError::SendWithdrawn),
        Unconfirmed::CompanyScopeUnconfirmed,
    );
    assert_eq!(
        withdrawn.downcast_ref::<TallyTransportError>(),
        Some(&TallyTransportError::SendWithdrawn)
    );
    assert!(!withdrawn.chain().any(|cause| cause.is::<Unconfirmed>()));
    let failed = anyhow::Error::new(TallyTransportError::ConnectionFailed);
    let typed = unconfirmed_unless_wire_refused(failed, Unconfirmed::MastersUnconfirmed);
    assert!(typed.chain().any(|cause| cause.is::<Unconfirmed>()));
    assert_eq!(crate::endpoint_wire::wire_refusal(&typed), None);
}
