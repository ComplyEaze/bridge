//! One audit_read part: wire-exact bytes, a single identity-bracketed attempt,
//! a typed failure, and the wire drain after any response Bridge cut off.
use super::*;
use std::time::Duration;
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ScenarioPlan, SequenceSimulator, WireEncoding,
};

fn captured(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

struct Lab {
    companies_xml: String,
    report_xml: String,
    identity: VerifiedCompanyIdentity,
    request: String,
}

fn lab() -> Lab {
    let companies_xml = captured(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-companies.utf16le.xml"
    ));
    let report_xml = captured(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    ));
    let companies = parse_companies_from_collection(&companies_xml).unwrap();
    let company = companies
        .iter()
        .find(|company| company.name == "WR2 Unicode Lab")
        .unwrap();
    let identity = VerifiedCompanyIdentity::from_observed_companies(
        company.name.clone(),
        crate::tally::test_company_guid(&company.guid.clone().unwrap()),
        company.company_number.clone().unwrap(),
        company.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap();
    let request = ReadOnlyProfile::StandardLedgerCatalogV1 {
        company: &bridge_tally_protocol::xml_read_profiles::ValidatedCompanyName::new(
            company.name.clone(),
        )
        .unwrap(),
    }
    .render();
    Lab {
        companies_xml,
        report_xml,
        identity,
        request,
    }
}

fn utf16(xml: &str) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(xml.to_string())).with_encoding(WireEncoding::Utf16Le)
}

fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

fn config(simulator: &SequenceSimulator) -> TallyConfig {
    TallyConfig {
        host: "127.0.0.1".into(),
        port: simulator.address().port(),
    }
}

async fn part(
    runtime: &TallyRuntime,
    simulator: &SequenceSimulator,
    lab: &Lab,
    shape: AuditPartShape,
) -> Result<AuditPart, AuditPartFailure> {
    runtime
        .fetch_audit_part(
            config(simulator),
            &lab.identity,
            super::super::agent_read_request::AgentReadRequest::parse(lab.request.clone()).unwrap(),
            shape,
        )
        .await
}

#[test]
fn the_part_deadline_is_the_ruled_twenty_seconds() {
    // Owner ruling 2 denied raising it; a change here reverses that ruling.
    assert_eq!(AUDIT_PART_DEADLINE, Duration::from_secs(20));
    assert_eq!(
        AUDIT_PART_DEADLINE,
        bridge_tally_transport::DEFAULT_REQUEST_TIMEOUT
    );
}

#[tokio::test]
async fn a_single_part_keeps_the_wire_bytes_between_two_identity_brackets() {
    let lab = lab();
    let report = utf16(&lab.report_xml);
    let wire = report.response_bytes();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        report,
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let runtime = TallyRuntime::default();
    let part = part(&runtime, &simulator, &lab, AuditPartShape::Single)
        .await
        .expect("a clean single part is admitted");
    // UTF-16LE on the wire, so the decoded text cannot stand in for it.
    assert_eq!(part.encoded_body, wire);
    assert_ne!(part.body.as_bytes(), wire.as_slice());
    assert_eq!(part.body, lab.report_xml);
    assert_eq!(part.encoded_sha256, sha256_hex(&wire));
    let observed = simulator.finish().unwrap();
    // company list, the part, company list: nothing else, and once each.
    assert_eq!(observed.len(), 3);
    assert_eq!(
        part.evidence.request_sha256,
        observed[1].request_body_sha256
    );
    assert_eq!(part.evidence.response_sha256, sha256_hex(&wire));
    assert_eq!(part.evidence.bytes, wire.len());
}

#[tokio::test]
async fn a_paired_part_must_repeat_byte_for_byte() {
    let lab = lab();
    let report = utf16(&lab.report_xml);
    let wire = report.response_bytes();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        report.clone(),
        status(),
        report.clone(),
        status(),
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let runtime = TallyRuntime::default();
    let admitted = part(&runtime, &simulator, &lab, AuditPartShape::Paired)
        .await
        .expect("a stable pair is admitted");
    assert_eq!(admitted.encoded_body, wire);
    assert_eq!(admitted.evidence.bytes, wire.len() * 2);
    assert_eq!(simulator.finish().unwrap().len(), 6);

    let changed = lab.report_xml.replacen("Cash", "Changed Cash", 1);
    assert_ne!(changed, lab.report_xml);
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        report,
        status(),
        utf16(&changed),
    ])
    .unwrap();
    let failure = part(&runtime, &simulator, &lab, AuditPartShape::Paired)
        .await
        .expect_err("a drifted pair is refused");
    assert_eq!(failure.kind, AuditPartFailureKind::PairDrift);
    assert_eq!(failure.kind.code(), "audit_part_pair_drift");
    assert!(failure.kind.retryable(), "someone else was writing");
    // Both completed bodies are accounted for, and no body is released.
    assert!(failure.evidence.expect("pair evidence").bytes > wire.len());
    // Refused on the drift itself, before the trailing health check.
    assert_eq!(simulator.finish().unwrap().len(), 4);
}

#[tokio::test]
async fn an_identity_change_at_the_closing_bracket_discards_a_body_that_arrived() {
    let lab = lab();
    let changed = lab.companies_xml.replace(
        lab.identity.company_guid(),
        "00000000-0000-4000-8000-000000000099",
    );
    assert_ne!(changed, lab.companies_xml);
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml),
        utf16(&changed),
    ])
    .unwrap();
    let failure = part(
        &TallyRuntime::default(),
        &simulator,
        &lab,
        AuditPartShape::Single,
    )
    .await
    .expect_err("the closing bracket refuses");
    assert_eq!(failure.kind, AuditPartFailureKind::IdentityChanged);
    assert_eq!(failure.kind.code(), "audit_part_company_identity_changed");
    assert!(!failure.kind.retryable());
    assert!(
        failure.evidence.is_some(),
        "the part that was read is accounted"
    );
    assert_eq!(simulator.finish().unwrap().len(), 3);
}

/// A cut part owes the drain on the wire record, so the next part probes first
/// and, while Tally does not answer, is refused with nothing more posted.
#[tokio::test]
async fn a_dropped_connection_holds_the_next_part_on_the_wire_drain() {
    let lab = lab();
    let mut plans = dropped(&lab);
    plans.push(status().with_delivery(Delivery::SlowHeaders(Duration::from_millis(600))));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default().with_wire_gate_config(
        WireGateConfig::default().with_drain(bridge_tally_transport::DrainPolicy {
            probe_deadline: Duration::from_millis(300),
            quick_below: Duration::from_millis(100),
            ..bridge_tally_transport::DrainPolicy::MEASURED
        }),
    );
    let failure = part(&runtime, &simulator, &lab, AuditPartShape::Single)
        .await
        .expect_err("the connection was dropped");
    assert_eq!(failure.kind, AuditPartFailureKind::ConnectionDropped);
    assert_eq!(failure.kind.code(), "audit_part_connection_dropped");
    assert!(failure.kind.retryable());
    let refused = part(&runtime, &simulator, &lab, AuditPartShape::Single)
        .await
        .expect_err("owed");
    assert_eq!(
        refused.kind,
        AuditPartFailureKind::NotSent("tally_endpoint_draining")
    );
    assert!(refused.kind.retryable());
    simulator.cancel();
    let posts = simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| request.path == "/")
        .count();
    assert_eq!(posts, 2);
}

#[tokio::test]
async fn an_oversized_part_is_not_retried_as_it_stands() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        ScenarioPlan::new(Fixture::Oversized {
            minimum_bytes: 64 * 1024,
        }),
    ])
    .unwrap();
    let runtime = TallyRuntime::with_transport_policy(bridge_tally_transport::TransportPolicy {
        xml_response_max_bytes: 32 * 1024,
        ..Default::default()
    });
    let failure = part(&runtime, &simulator, &lab, AuditPartShape::Single)
        .await
        .expect_err("over the response limit");
    assert_eq!(failure.kind, AuditPartFailureKind::SizeLimit);
    assert_eq!(
        failure.kind.code(),
        "audit_part_response_size_limit_exceeded"
    );
    // The plan must divide it; sending it again would fail the same way.
    assert!(!failure.kind.retryable());
    assert_eq!(simulator.finish().unwrap().len(), 2);
}

#[test]
fn every_failure_kind_has_a_distinct_stable_code() {
    let kinds = [
        AuditPartFailureKind::Deadline,
        AuditPartFailureKind::SizeLimit,
        AuditPartFailureKind::ConnectionDropped,
        AuditPartFailureKind::Unreachable,
        AuditPartFailureKind::PairDrift,
        AuditPartFailureKind::IdentityChanged,
        AuditPartFailureKind::EducationBoundary,
    ];
    let codes = kinds
        .iter()
        .map(|kind| kind.code())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(codes.len(), kinds.len());
    assert!(codes.iter().all(|code| code.starts_with("audit_part_")));
    // Nothing reached Tally, so it may be tried again.
    assert!(AuditPartFailureKind::Unreachable.retryable());
    assert!(!AuditPartFailureKind::Other("request_size_limit_exceeded").retryable());
}

#[tokio::test]
async fn elapsed_times_the_data_request_and_not_the_brackets() {
    let lab = lab();
    let data = Duration::from_millis(400);
    let bracket = Duration::from_millis(600);
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml).with_delivery(Delivery::SlowHeaders(bracket)),
        utf16(&lab.report_xml).with_delivery(Delivery::SlowHeaders(data)),
        utf16(&lab.companies_xml).with_delivery(Delivery::SlowHeaders(bracket)),
    ])
    .unwrap();
    let admitted = part(
        &TallyRuntime::default(),
        &simulator,
        &lab,
        AuditPartShape::Single,
    )
    .await
    .expect("a slow part inside the deadline is admitted");
    assert!(admitted.elapsed >= data, "{:?}", admitted.elapsed);
    assert!(admitted.elapsed < data + bracket, "{:?}", admitted.elapsed);
    assert_eq!(simulator.finish().unwrap().len(), 3);
}

#[tokio::test]
async fn a_request_that_does_not_name_the_verified_company_is_never_sent() {
    let lab = lab();
    let other = ReadOnlyProfile::StandardLedgerCatalogV1 {
        company: &bridge_tally_protocol::xml_read_profiles::ValidatedCompanyName::new(
            "Entirely Different Company".to_string(),
        )
        .unwrap(),
    }
    .render();
    let unnamed = lab.request.replace(
        &format!(
            "<SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY>",
            lab.identity.display_name()
        ),
        "",
    );
    assert_ne!(unnamed, lab.request, "the fixture names its company");
    let twice = lab.request.replace(
        "<SVCURRENTCOMPANY>",
        "<SVCURRENTCOMPANY>WR2 Unicode Lab</SVCURRENTCOMPANY><SVCURRENTCOMPANY>",
    );
    // One element, right name, wrong place: Tally would not read it there.
    let misplaced = unnamed.replacen(
        "<BODY>",
        &format!(
            "<BODY><NOTE><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></NOTE>",
            lab.identity.display_name()
        ),
        1,
    );
    assert_ne!(misplaced, unnamed);
    let simulator = SequenceSimulator::spawn(vec![utf16(&lab.companies_xml)]).unwrap();
    let runtime = TallyRuntime::default();
    for request in [other, unnamed, twice, misplaced] {
        let failure = runtime
            .fetch_audit_part(
                config(&simulator),
                &lab.identity,
                super::super::agent_read_request::AgentReadRequest::parse(request).unwrap(),
                AuditPartShape::Single,
            )
            .await
            .expect_err("not scoped to the verified company");
        assert_eq!(failure.kind, AuditPartFailureKind::RequestNotCompanyScoped);
        assert!(!failure.kind.retryable());
    }
    assert_eq!(simulator.received(), 0);
    drop(simulator);
}

#[tokio::test]
async fn an_error_envelope_is_rejected_whole() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16("<ENVELOPE><LINEERROR>Could not find company</LINEERROR></ENVELOPE>"),
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml),
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let runtime = TallyRuntime::default();
    let failure = part(&runtime, &simulator, &lab, AuditPartShape::Single)
        .await
        .expect_err("an error envelope is not a part");
    assert_eq!(failure.kind, AuditPartFailureKind::ResponseRejected);
    assert_eq!(failure.kind.code(), "audit_part_response_rejected");
    assert!(!failure.kind.retryable());
    assert!(failure.evidence.is_some(), "the response read is accounted");
    // Read to the end, so nothing is left running: the next part is sent.
    part(&runtime, &simulator, &lab, AuditPartShape::Single)
        .await
        .expect("the next part is sent");
    assert_eq!(simulator.finish().unwrap().len(), 5);
}

/// The header of a company Object export carries the product release fields
/// (measured on a live TallyPrime 7.1 Silver, item 697-S0A): the collection
/// status check refuses it, so the Object request is checked with exactly those
/// five fields allowed and every other check of the envelope kept. A collection
/// answer with the same header stays refused.
fn object_answer(status: &str, release: &str, extra: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>{status}</STATUS>{release}{extra}</HEADER>\
         <BODY><DATA/></BODY></ENVELOPE>"
    )
}

const RELEASE_HEADER: &str = "<PRODMAJORVER>1</PRODMAJORVER><PRODMINORVER>1</PRODMINORVER>\
    <PRODMAJORREL>7</PRODMAJORREL><PRODMINORREL>1</PRODMINORREL><PRODTYPE>5</PRODTYPE>";

async fn object_part(answer: &str) -> Result<AuditPart, AuditPartFailure> {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(answer),
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let object = super::super::agent_read_request::AgentReadRequest::company_object(&lab.identity);
    assert!(object.is_company_object());
    let result = TallyRuntime::default()
        .fetch_audit_part(
            config(&simulator),
            &lab.identity,
            object,
            AuditPartShape::Single,
        )
        .await;
    drop(simulator);
    result
}

#[tokio::test]
async fn the_company_object_answer_is_admitted_with_its_release_header() {
    object_part(&object_answer("1", RELEASE_HEADER, ""))
        .await
        .expect("the object export is admitted");
    // The same answer without the release fields is a plain envelope and passes too.
    object_part(&object_answer("1", "", ""))
        .await
        .expect("a plain envelope is admitted");
}

#[tokio::test]
async fn the_company_object_answer_keeps_every_other_envelope_check() {
    let refused = |answer: String, why: &'static str| async move {
        let failure = object_part(&answer).await.expect_err(why);
        assert_eq!(
            failure.kind,
            AuditPartFailureKind::ResponseRejected,
            "{why}"
        );
    };
    refused(object_answer("0", RELEASE_HEADER, ""), "a failed status").await;
    refused(
        object_answer("1", RELEASE_HEADER, "<EXTRA>1</EXTRA>"),
        "another header field",
    )
    .await;
    refused(
        object_answer("1", RELEASE_HEADER, "<PRODTYPE>5</PRODTYPE>"),
        "a repeated release field",
    )
    .await;
    refused(
        object_answer("1", "<PRODTYPE>x</PRODTYPE>", ""),
        "a release field that is not a number",
    )
    .await;
    refused(
        object_answer("1", RELEASE_HEADER, "").replace("<DATA/>", "<DATA/></ENVELOPE><ENVELOPE>"),
        "a second root",
    )
    .await;
}

#[tokio::test]
async fn a_collection_answer_with_the_release_header_is_refused() {
    let lab = lab();
    let answer = object_answer("1", RELEASE_HEADER, "");
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(&answer),
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let collection =
        super::super::agent_read_request::AgentReadRequest::parse(lab.request.clone()).unwrap();
    assert!(!collection.is_company_object());
    let failure = TallyRuntime::default()
        .fetch_audit_part(
            config(&simulator),
            &lab.identity,
            collection,
            AuditPartShape::Single,
        )
        .await
        .expect_err("a collection answer with that header is refused");
    assert_eq!(failure.kind, AuditPartFailureKind::ResponseRejected);
    drop(simulator);
}

#[tokio::test]
async fn a_refused_response_head_is_not_retried() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml).with_http_status(500),
    ])
    .unwrap();
    let runtime = TallyRuntime::default();
    let failure = part(&runtime, &simulator, &lab, AuditPartShape::Single)
        .await
        .expect_err("HTTP 500");
    assert_eq!(
        failure.kind,
        AuditPartFailureKind::HeadRejected("http_status_failure")
    );
    assert!(!failure.kind.retryable());
    assert_eq!(simulator.finish().unwrap().len(), 2);
}

fn dropped(lab: &Lab) -> Vec<ScenarioPlan> {
    vec![
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml).with_delivery(Delivery::ResetAfterRequestProcessed {
            delay: Duration::ZERO,
        }),
    ]
}

/// Parts are serialised, not refused, while another part is merely running.
#[tokio::test]
async fn a_part_queued_behind_a_clean_part_is_sent_after_it() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml).with_delivery(Delivery::SlowHeaders(Duration::from_millis(300))),
        utf16(&lab.companies_xml),
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml),
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let runtime = TallyRuntime::default();
    let first = part(&runtime, &simulator, &lab, AuditPartShape::Single);
    let queued = async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        part(&runtime, &simulator, &lab, AuditPartShape::Single).await
    };
    let (first, queued) = tokio::join!(first, queued);
    first.expect("first part");
    queued.expect("the queued part waits and is then sent");
    assert_eq!(simulator.finish().unwrap().len(), 6);
}

/// A tool call withdrawn before the part is queued (#584) sends nothing and
/// owes nothing.
#[tokio::test]
async fn a_withdrawn_call_sends_no_part() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![utf16(&lab.companies_xml)]).unwrap();
    let runtime = TallyRuntime::default();
    let withdrawn = tokio_util::sync::CancellationToken::new();
    withdrawn.cancel();
    let failure = TOOL_CANCELLATION
        .scope(
            withdrawn,
            part(&runtime, &simulator, &lab, AuditPartShape::Single),
        )
        .await
        .expect_err("withdrawn");
    assert_eq!(
        failure.kind,
        AuditPartFailureKind::NotSent("request_cancelled")
    );
    assert!(failure.kind.retryable());
    assert_eq!(simulator.received(), 0);
    drop(simulator);
}

fn education(companies_xml: &str) -> String {
    let flipped = companies_xml.replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    assert_ne!(flipped, companies_xml, "the fixture carries EDUMODE");
    flipped
}

fn dated(request: &str, day: &str) -> String {
    let dated = request.replacen(
        "</STATICVARIABLES>",
        &format!(
            "<SVFROMDATE TYPE=\"Date\">{day}</SVFROMDATE><SVTODATE TYPE=\"Date\">{day}</SVTODATE></STATICVARIABLES>"
        ),
        1,
    );
    assert_ne!(dated, request, "the request carries STATICVARIABLES");
    dated
}

async fn dated_part(
    runtime: &TallyRuntime,
    simulator: &SequenceSimulator,
    lab: &Lab,
    day: &str,
) -> Result<AuditPart, AuditPartFailure> {
    runtime
        .fetch_audit_part(
            config(simulator),
            &lab.identity,
            super::super::agent_read_request::AgentReadRequest::parse(dated(&lab.request, day))
                .unwrap(),
            AuditPartShape::Single,
        )
        .await
}

/// bridge#581 for audit parts: Education answers a window starting on another
/// day with a well-formed empty collection, so the part is refused before it
/// is sent, and nothing is owed.
#[tokio::test]
async fn an_education_endpoint_refuses_a_part_it_would_serve_empty() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![utf16(&education(&lab.companies_xml))]).unwrap();
    let runtime = TallyRuntime::default();
    let failure = dated_part(&runtime, &simulator, &lab, "20250403")
        .await
        .expect_err("Education does not honour the 3rd");
    assert_eq!(failure.kind, AuditPartFailureKind::EducationBoundary);
    assert_eq!(
        failure.kind.code(),
        "audit_part_window_unsupported_in_education"
    );
    assert!(!failure.kind.retryable());
    // Only the opening bracket was sent.
    assert_eq!(simulator.finish().unwrap().len(), 1);
}

#[tokio::test]
async fn an_education_endpoint_serves_a_part_on_a_day_it_honours() {
    let lab = lab();
    let companies = education(&lab.companies_xml);
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&companies),
        utf16(&lab.report_xml),
        utf16(&companies),
    ])
    .unwrap();
    let admitted = dated_part(&TallyRuntime::default(), &simulator, &lab, "20250401")
        .await
        .expect("the 1st is honoured");
    assert_eq!(
        admitted.boundary_profile,
        DateBoundaryProfile::EducationRestricted
    );
    assert_eq!(simulator.finish().unwrap().len(), 3);
}

/// A licence dropping to Education during the part: which mode answered is
/// unknown, so the part is refused as if it had been sent in Education, with
/// what was read accounted for. The body was read to the end: nothing owed.
#[tokio::test]
async fn education_first_seen_at_the_closing_bracket_refuses_the_part() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml),
        utf16(&education(&lab.companies_xml)),
    ])
    .unwrap();
    let runtime = TallyRuntime::default();
    let failure = dated_part(&runtime, &simulator, &lab, "20250403")
        .await
        .expect_err("closing bracket reports Education");
    assert_eq!(failure.kind, AuditPartFailureKind::EducationBoundary);
    assert!(
        failure.evidence.is_some(),
        "the part that was read is accounted"
    );
    assert_eq!(simulator.finish().unwrap().len(), 3);
}

#[tokio::test]
async fn a_licensed_part_reports_the_ordinary_profile() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml),
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let admitted = dated_part(&TallyRuntime::default(), &simulator, &lab, "20250403")
        .await
        .expect("licensed");
    assert_eq!(admitted.boundary_profile, DateBoundaryProfile::ModeAgnostic);
    assert_eq!(simulator.finish().unwrap().len(), 3);
}

/// Education first reported at the closing bracket on a day it honours: the
/// part is admitted and reports Education, the stricter of the two profiles,
/// so a caller keeps Education in force for the rest of the window.
#[tokio::test]
async fn a_part_reports_education_when_only_the_closing_bracket_saw_it() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&lab.companies_xml),
        utf16(&lab.report_xml),
        utf16(&education(&lab.companies_xml)),
    ])
    .unwrap();
    let admitted = dated_part(&TallyRuntime::default(), &simulator, &lab, "20250401")
        .await
        .expect("the 1st is honoured in either mode");
    assert_eq!(
        admitted.boundary_profile,
        DateBoundaryProfile::EducationRestricted
    );
    assert_eq!(simulator.finish().unwrap().len(), 3);
}

/// The other asymmetric case: Education at the opening bracket, licensed at the
/// closing one, on a day Education honours. Still Education.
#[tokio::test]
async fn a_part_keeps_education_seen_only_at_the_opening_bracket() {
    let lab = lab();
    let simulator = SequenceSimulator::spawn(vec![
        utf16(&education(&lab.companies_xml)),
        utf16(&lab.report_xml),
        utf16(&lab.companies_xml),
    ])
    .unwrap();
    let admitted = dated_part(&TallyRuntime::default(), &simulator, &lab, "20250401")
        .await
        .expect("the 1st is honoured in either mode");
    assert_eq!(
        admitted.boundary_profile,
        DateBoundaryProfile::EducationRestricted
    );
    assert_eq!(simulator.finish().unwrap().len(), 3);
}
