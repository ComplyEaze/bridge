//! Captured read replay with mode/date fault injection; no live Education calls.
use super::*;
use crate::reports::party_ledger_master::SourceNoteKind;
use crate::tally::connection::PartyLedgerMasterSourceValidationError;
use crate::tally::ConnectionStatus;
use tally_protocol_simulator::{
    encode, Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
fn companies() -> String {
    decode(include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"))
}
fn extents() -> String {
    include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    ).to_owned()
}
fn company_identity(xml: &str) -> VerifiedCompanyIdentity {
    identity_for_guid(xml, GUID)
}
fn identity_for_guid(xml: &str, guid: &str) -> VerifiedCompanyIdentity {
    let companies = parse_companies_from_collection(xml).unwrap();
    let row = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(guid))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        row.name.clone(),
        crate::tally::test_company_guid(guid),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}
fn xml(text: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(text))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}
fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}
fn education(text: &str) -> String {
    text.replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    )
}
fn assertion(
    extent: &str,
    identity: &VerifiedCompanyIdentity,
) -> PartyLedgerMasterCurrencyAssertion {
    inr_witness_for_tests(extent, identity)
}
fn stale_cache(runtime: &TallyRuntime, config: &TallyConfig) {
    let companies = parse_companies_from_collection(&companies()).unwrap();
    *runtime
        .session(config.clone())
        .unwrap()
        .cached_probe
        .write()
        .unwrap() = Some(CachedProbe {
        review_id: "stale-licensed".into(),
        observed_at_unix_ms: 1,
        freshness_origin_unix_ms: 1,
        reserved: false,
        result: TallyProbeResult {
            connection: ConnectionStatus {
                reachable: true,
                compatible: true,
                server_text: String::new(),
                product: super::super::connection::TallyProduct::Unknown,
                error: None,
            },
            companies,
            profile: bridge_tally_core::CapabilityProfile {
                profile_version: 3,
                product: "TallyPrime".into(),
                release: None,
                license_tier: None,
                mode: Some("Licensed".into()),
                transports: Default::default(),
                features: Default::default(),
                packs: Default::default(),
            },
            passport_snapshot_id: None,
        },
    });
}
fn join(left: &str, right: &str) -> String {
    sha256_hex(format!("{left}:{right}").as_bytes())
}

// These mutations alter profile metadata in captured bodies. They exercise
// admission behavior only; they are not live evidence for another product,
// release, or licence tier.
const REFUSAL_PROFILE_FAULTS: [&str; 2] = ["mode_unknown", "product_unknown"];
const ADMITTED_PROFILE_VARIANTS: [&str; 5] = [
    "none",
    "release_old",
    "release_missing",
    "release_unknown",
    "gold",
];
fn fault_company(fault: &str) -> String {
    let source = companies();
    let release = "<BRIDGERELEASE TYPE=\"String\">7.1</BRIDGERELEASE>";
    let silver = "<SILVER TYPE=\"Logical\">Yes</SILVER>";
    let gold = "<GOLD TYPE=\"Logical\">No</GOLD>";
    let product = "<PRODUCTNAME TYPE=\"String\">TallyPrime</PRODUCTNAME>";
    let changed = match fault {
        "education" => education(&source),
        "erp9" => source.replace(
            product,
            "<PRODUCTNAME TYPE=\"String\">Tally ERP 9</PRODUCTNAME>",
        ),
        "editlog" => source.replace(
            product,
            "<PRODUCTNAME TYPE=\"String\">TallyPrime Edit Log</PRODUCTNAME>",
        ),
        "product_unknown" => source.replace(
            product,
            "<PRODUCTNAME TYPE=\"String\">UnknownProduct</PRODUCTNAME>",
        ),
        "release_old" => source.replace(
            release,
            "<BRIDGERELEASE TYPE=\"String\">7.0</BRIDGERELEASE>",
        ),
        "release_missing" => source.replace(release, ""),
        "release_unknown" => source.replace(
            release,
            "<BRIDGERELEASE TYPE=\"String\">unknown</BRIDGERELEASE>",
        ),
        "gold" => source
            .replace(silver, "<SILVER TYPE=\"Logical\">No</SILVER>")
            .replace(gold, "<GOLD TYPE=\"Logical\">Yes</GOLD>"),
        "tier_ambiguous" => source.replace(gold, "<GOLD TYPE=\"Logical\">Yes</GOLD>"),
        "mode_unknown" => source.replace(silver, "<SILVER TYPE=\"Logical\">No</SILVER>"),
        "none" => return source,
        _ => panic!("unknown profile fault: {fault}"),
    };
    assert_ne!(changed, source, "{fault}");
    changed
}
fn assert_profile_refusal(error: &anyhow::Error, fault: &str) {
    assert_eq!(
        error
            .chain()
            .find_map(|cause| cause.downcast_ref::<OpeningBoundaryObservationError>()),
        Some(&OpeningBoundaryObservationError::Unobserved),
        "{fault}: {error:?}"
    );
}

#[tokio::test]
async fn financial_reads_refuse_unqualified_profiles_before_reports_despite_stale_cache() {
    for party in [false, true] {
        for cached in [false, true] {
            for fault in REFUSAL_PROFILE_FAULTS {
                let plans = vec![status(), xml(fault_company(fault))];
                let response_bytes = plans
                    .iter()
                    .map(ScenarioPlan::response_bytes)
                    .collect::<Vec<_>>();
                let simulator = SequenceSimulator::spawn(plans).unwrap();
                let config = TallyConfig {
                    host: simulator.address().ip().to_string(),
                    port: simulator.address().port(),
                };
                let identity = company_identity(&companies());
                let runtime = TallyRuntime::default();
                if cached {
                    stale_cache(&runtime, &config);
                }
                let error = if party {
                    runtime
                        .fetch_party_ledger_master_source_with_evidence(
                            config,
                            &identity,
                            assertion(&extents(), &identity),
                            BalanceDateBasis::HostToday(TallyDate::parse("20990101").unwrap()),
                            crate::tally::connection::ComplianceRead::Standard,
                        )
                        .await
                        .unwrap_err()
                } else {
                    runtime
                        .fetch_agent_outstandings_with_evidence(
                            config,
                            &identity,
                            TallyDate::parse("20260902").unwrap(),
                            assertion(&extents(), &identity),
                            OutstandingsAgeingAnchor::DueDate,
                        )
                        .await
                        .unwrap_err()
                };
                assert_profile_refusal(&error, fault);
                let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
                let observed = simulator.finish().unwrap();
                assert_eq!(
                    observed.len(),
                    2,
                    "{fault}: refusal must precede financial reports"
                );
                assert_eq!(
                    evidence.request_sha256,
                    join(
                        &observed[0].request_body_sha256,
                        &observed[1].request_body_sha256
                    )
                );
                assert_eq!(
                    evidence.response_sha256,
                    join(
                        &sha256_hex(&response_bytes[0]),
                        &sha256_hex(&response_bytes[1])
                    )
                );
                assert_eq!(
                    evidence.bytes,
                    response_bytes[0].len() + response_bytes[1].len()
                );
            }
        }
    }
}

fn pair(plans: &mut Vec<ScenarioPlan>, response: ScenarioPlan) {
    plans.extend([response.clone(), status(), response, status()]);
}
fn compliance_plans(profile_fault: &str) -> Vec<ScenarioPlan> {
    let company = xml(fault_company(profile_fault));
    let extent = xml(extents());
    let mut plans = vec![status(), company.clone(), company.clone()];
    pair(&mut plans, extent.clone());
    for bytes in [
        include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-masters.utf16le.xml").as_slice(),
        include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-balances.utf16le.xml").as_slice(),
        include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-groups.utf16le.xml").as_slice(),
    ] { pair(&mut plans,xml(decode(bytes))); }
    pair(&mut plans, extent);
    plans.extend([company.clone(), status(), company]);
    plans
}

#[tokio::test]
async fn compliance_source_requires_closing_mode_and_preserves_source_commitments() {
    for fault in ADMITTED_PROFILE_VARIANTS {
        let plans = compliance_plans(fault);
        let responses = plans
            .iter()
            .map(|plan| encode(&plan.fixture.body(), plan.encoding))
            .collect::<Vec<_>>();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let identity = company_identity(&companies());
        let result = TallyRuntime::default()
            .fetch_party_ledger_master_source_with_evidence(
                TallyConfig {
                    host: simulator.address().ip().to_string(),
                    port: simulator.address().port(),
                },
                &identity,
                assertion(&extents(), &identity),
                BalanceDateBasis::HostToday(TallyDate::parse("20990101").unwrap()),
                crate::tally::connection::ComplianceRead::Standard,
            )
            .await;
        let (source, evidence) = result.unwrap();
        assert_eq!(source.rows.len(), 9);
        let observed = simulator
            .finish()
            .unwrap_or_else(|error| panic!("{fault}: {error}"));
        assert_eq!(observed.len(), 26);
        let req = |i: usize| observed[i].request_body_sha256.clone();
        let res = |i: usize| sha256_hex(&responses[i]);
        let triple =
            |a: String, b: String, c: String| sha256_hex(format!("{a}:{b}:{c}").as_bytes());
        assert_eq!(
            evidence.request_sha256,
            join(
                &join(&join(&req(0), &req(1)), &triple(req(7), req(11), req(15))),
                &join(&req(24), &req(25))
            )
        );
        assert_eq!(
            evidence.response_sha256,
            join(
                &join(&join(&res(0), &res(1)), &triple(res(7), res(11), res(15))),
                &join(&res(24), &res(25))
            )
        );
        assert_eq!(
            evidence.bytes,
            [0, 1, 7, 9, 11, 13, 15, 17, 24, 25]
                .iter()
                .map(|i| responses[*i].len())
                .sum::<usize>()
        );
    }
}

#[tokio::test]
async fn party_source_ends_its_balance_snapshot_at_the_hosts_today_when_a_voucher_is_later_dated() {
    // #875. The captured extent's last voucher date for this company is 20260907
    // and its books begin 20260401. The clock is injected, never read.
    for (today, to, later_dated) in [
        ("20260815", "20260831", true),
        // A host clock before the books begin: a one-day snapshot from the
        // start of the books, with the later-dated note.
        ("20260301", "20260401", true),
        ("20260907", "20261001", false),
        ("20990101", "20261001", false),
    ] {
        let plans = compliance_plans(ADMITTED_PROFILE_VARIANTS[0]);
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let identity = company_identity(&companies());
        let (source, _) = TallyRuntime::default()
            .fetch_party_ledger_master_source_with_evidence(
                TallyConfig {
                    host: simulator.address().ip().to_string(),
                    port: simulator.address().port(),
                },
                &identity,
                assertion(&extents(), &identity),
                BalanceDateBasis::HostToday(TallyDate::parse(today).unwrap()),
                crate::tally::connection::ComplianceRead::Standard,
            )
            .await
            .unwrap();
        simulator.finish().unwrap();
        assert_eq!(source.to.as_str(), to, "today {today}");
        assert_eq!(source.last_voucher_date.as_str(), "20260907");
        assert_eq!(
            source.later_dated_vouchers_note().is_some(),
            later_dated,
            "today {today}"
        );
    }
}

/// One party-master read over the captured book (books from 20260401, last
/// voucher 20260907), its balances ending by `basis`. The requests the book saw
/// come back with the source, or the refusal.
async fn party_source_with_basis(
    basis: BalanceDateBasis,
    plans: Vec<ScenarioPlan>,
) -> (
    anyhow::Result<PartyLedgerMasterSource>,
    Vec<tally_protocol_simulator::ObservedRequest>,
) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let identity = company_identity(&companies());
    let result = TallyRuntime::default()
        .fetch_party_ledger_master_source_with_evidence(
            TallyConfig {
                host: simulator.address().ip().to_string(),
                port: simulator.address().port(),
            },
            &identity,
            assertion(&extents(), &identity),
            basis,
            crate::tally::connection::ComplianceRead::Standard,
        )
        .await
        .map(|(source, _)| source);
    (result, simulator.finish().unwrap())
}

fn note_kinds(source: &PartyLedgerMasterSource) -> Vec<SourceNoteKind> {
    source.notes().iter().map(|note| note.kind).collect()
}

#[tokio::test]
async fn a_chosen_date_before_the_books_is_refused_before_any_ledger_request() {
    // The books begin 20260401. The refusal comes from the opening extent: the
    // book saw the profile and identity reads and the extent pair, and nothing
    // after them (the plans hold no more, so a further request would fail).
    let mut plans = compliance_plans(ADMITTED_PROFILE_VARIANTS[0]);
    plans.truncate(7);
    let (result, observed) = party_source_with_basis(
        BalanceDateBasis::Requested(TallyDate::parse("20260331").unwrap()),
        plans,
    )
    .await;
    let error = result.unwrap_err();
    assert!(
        error.chain().any(|cause| matches!(
            cause.downcast_ref::<PartyLedgerMasterSourceValidationError>(),
            Some(PartyLedgerMasterSourceValidationError::AsOfPrecedesBooksFrom)
        )),
        "unexpected refusal: {error:?}"
    );
    assert_eq!(observed.len(), 7);
}

/// The hash of the balance request the book should have seen for a snapshot
/// from the start of the books (20260401) to `to`, rendered here from the
/// request builder and encoded as the wire carries it (UTF-16LE with a BOM).
/// The date is the only thing in the request that depends on the basis.
fn balance_request_sha256(company: &str, to: &str) -> String {
    let period = bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20260401").unwrap(),
        TallyDate::parse(to).unwrap(),
    )
    .unwrap();
    let request = bridge_tally_protocol::native_outstandings::render_native_ledger_snapshot_request(
        company, &period,
    );
    let mut encoded = vec![0xff, 0xfe];
    encoded.extend(request.encode_utf16().flat_map(u16::to_le_bytes));
    sha256_hex(&encoded)
}

#[tokio::test]
async fn a_chosen_date_ends_the_balances_there_even_past_the_last_voucher() {
    // The host-date path clamps to the last voucher date (20260907), so a host
    // clock of 20261231 ends the balances at 20261001; a chosen 20261231 ends
    // them at 20261231. Each request is the one for its own end date, and the
    // dates give three different requests.
    let company = company_identity(&companies()).display_name().to_owned();
    let mut seen = std::collections::BTreeSet::new();
    for (requested, to, kinds) in [
        (
            "20260401",
            "20260401",
            vec![SourceNoteKind::LaterDatedVoucher],
        ),
        (
            "20260731",
            "20260731",
            vec![SourceNoteKind::LaterDatedVoucher],
        ),
        ("20261231", "20261231", vec![]),
    ] {
        let (result, observed) = party_source_with_basis(
            BalanceDateBasis::Requested(TallyDate::parse(requested).unwrap()),
            compliance_plans(ADMITTED_PROFILE_VARIANTS[0]),
        )
        .await;
        let source = result.unwrap();
        assert_eq!(source.to.as_str(), to, "chosen {requested}");
        assert_eq!(note_kinds(&source), kinds, "chosen {requested}");
        assert_eq!(
            observed[11].request_body_sha256,
            balance_request_sha256(&company, to),
            "the request carries the end date {to}"
        );
        seen.insert(observed[11].request_body_sha256.clone());
    }
    assert_eq!(seen.len(), 3, "distinct dates give distinct requests");
    let (host, host_observed) = party_source_with_basis(
        BalanceDateBasis::HostToday(TallyDate::parse("20261231").unwrap()),
        compliance_plans(ADMITTED_PROFILE_VARIANTS[0]),
    )
    .await;
    assert_eq!(host.unwrap().to.as_str(), "20261001");
    assert_eq!(
        host_observed[11].request_body_sha256,
        balance_request_sha256(&company, "20261001")
    );
}

#[tokio::test]
async fn a_chosen_date_that_is_not_a_boundary_moves_to_the_next_one_and_says_so() {
    // 20260815 is not a day Tally accepts as a period end; the next is 20260831.
    // The request names 20260831 whether that or 20260815 was typed.
    let balance_request = |observed: &[tally_protocol_simulator::ObservedRequest]| {
        observed[11].request_body_sha256.clone()
    };
    let (moved, moved_observed) = party_source_with_basis(
        BalanceDateBasis::Requested(TallyDate::parse("20260815").unwrap()),
        compliance_plans(ADMITTED_PROFILE_VARIANTS[0]),
    )
    .await;
    let (exact, exact_observed) = party_source_with_basis(
        BalanceDateBasis::Requested(TallyDate::parse("20260831").unwrap()),
        compliance_plans(ADMITTED_PROFILE_VARIANTS[0]),
    )
    .await;
    let (moved, exact) = (moved.unwrap(), exact.unwrap());
    assert_eq!(moved.to.as_str(), "20260831");
    assert_eq!(
        balance_request(&moved_observed),
        balance_request(&exact_observed)
    );
    assert_eq!(
        balance_request(&moved_observed),
        balance_request_sha256(company_identity(&companies()).display_name(), "20260831"),
        "the request carries the end date the typed one moved to, not the typed one"
    );
    assert_eq!(
        note_kinds(&moved),
        vec![SourceNoteKind::DateMoved, SourceNoteKind::LaterDatedVoucher]
    );
    assert_eq!(note_kinds(&exact), vec![SourceNoteKind::LaterDatedVoucher]);
    // Past the last voucher the move is the only note.
    let (past, _) = party_source_with_basis(
        BalanceDateBasis::Requested(TallyDate::parse("20261215").unwrap()),
        compliance_plans(ADMITTED_PROFILE_VARIANTS[0]),
    )
    .await;
    let past = past.unwrap();
    assert_eq!(past.to.as_str(), "20261231");
    assert_eq!(note_kinds(&past), vec![SourceNoteKind::DateMoved]);
}

#[tokio::test]
async fn a_month_end_the_gateway_refuses_moves_to_the_first_of_the_next_month() {
    // Only the 1st, 2nd and 31st are accepted as a period end, so the last day
    // of a 30-day month or of February moves to the 1st, and that day's vouchers
    // are in the balances: the note says so.
    for (requested, to) in [
        ("20260630", "20260701"),
        ("20260930", "20261001"),
        ("20261130", "20261201"),
        ("20270228", "20270301"),
    ] {
        let (result, observed) = party_source_with_basis(
            BalanceDateBasis::Requested(TallyDate::parse(requested).unwrap()),
            compliance_plans(ADMITTED_PROFILE_VARIANTS[0]),
        )
        .await;
        let source = result.unwrap();
        assert_eq!(source.to.as_str(), to, "chosen {requested}");
        assert!(
            note_kinds(&source).contains(&SourceNoteKind::DateMoved),
            "chosen {requested}"
        );
        let moved = source.date_moved_note().unwrap();
        assert!(
            moved.contains(&format!("include the vouchers dated {to}")),
            "{moved}"
        );
        assert_eq!(
            observed[11].request_body_sha256,
            balance_request_sha256(company_identity(&companies()).display_name(), to),
            "chosen {requested}"
        );
    }
}

/// The balance request of the host-date path, taken from master before #903
/// changed anything: with no date typed, the request must not change.
const HOST_DATE_BALANCE_REQUEST_SHA256: &str =
    "8dbb484997cb491660348481f5aa1376291855fa94d8303c2151b1c895d5fbcd";

#[tokio::test]
async fn party_source_host_date_balance_request_is_byte_for_byte_unchanged() {
    let plans = compliance_plans(ADMITTED_PROFILE_VARIANTS[0]);
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let identity = company_identity(&companies());
    TallyRuntime::default()
        .fetch_party_ledger_master_source_with_evidence(
            TallyConfig {
                host: simulator.address().ip().to_string(),
                port: simulator.address().port(),
            },
            &identity,
            assertion(&extents(), &identity),
            BalanceDateBasis::HostToday(TallyDate::parse("20260815").unwrap()),
            crate::tally::connection::ComplianceRead::Standard,
        )
        .await
        .unwrap();
    let observed = simulator.finish().unwrap();
    assert_eq!(
        observed[11].request_body_sha256, HOST_DATE_BALANCE_REQUEST_SHA256,
        "the balance request is the third pair's first request"
    );
}

#[tokio::test]
async fn outstandings_requires_closing_mode_and_retains_sources_on_refusal() {
    for fault in std::iter::once("amount").chain(ADMITTED_PROFILE_VARIANTS) {
        let profile_fault = if fault == "amount" { "none" } else { fault };
        let company = xml(fault_company(profile_fault));
        let extent = xml(extents());
        let mut plans = vec![status(), company.clone(), company.clone()];
        pair(&mut plans, extent.clone());
        for (index, bytes) in [
            include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-receivable.utf16le.xml").as_slice(),
            include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-groups.utf16le.xml").as_slice(),
            include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-payable.utf16le.xml").as_slice(),
            include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-ageing-ledgers.utf16le.xml").as_slice(),
        ].into_iter().enumerate() {
            let mut source = decode(bytes);
            if fault == "amount" && index == 0 {
                let start = source.find("<BILLCL>").unwrap() + "<BILLCL>".len();
                let end = start + source[start..].find("</BILLCL>").unwrap();
                source.replace_range(start..end, "not-a-number");
            }
            pair(&mut plans, xml(source));
        }
        pair(&mut plans, extent);
        plans.extend([company, status(), xml(fault_company(profile_fault))]);
        let responses = plans
            .iter()
            .map(|plan| encode(&plan.fixture.body(), plan.encoding))
            .collect::<Vec<_>>();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let identity = identity_for_guid(&companies(), "eebb9a9f-1679-4468-9e8f-814c729674cb");
        let result = TallyRuntime::default()
            .fetch_agent_outstandings_with_evidence(
                TallyConfig {
                    host: simulator.address().ip().to_string(),
                    port: simulator.address().port(),
                },
                &identity,
                TallyDate::parse("20260801").unwrap(),
                assertion(&extents(), &identity),
                OutstandingsAgeingAnchor::DueDate,
            )
            .await;
        let evidence = if ADMITTED_PROFILE_VARIANTS.contains(&fault) {
            let (result, evidence) = result.unwrap();
            assert!(matches!(result, OutstandingsLoadResult::Complete { .. }));
            evidence
        } else {
            let error = result.unwrap_err();
            if fault != "amount" {
                assert_profile_refusal(&error, fault);
            } else {
                // The amount is the first bill's: the refusal names its row,
                // and the runtime names the report (bridge#1096).
                assert_eq!(
                    error
                        .chain()
                        .find_map(|cause| cause.downcast_ref::<NativeOutstandingsError>()),
                    Some(&NativeOutstandingsError::BillRow {
                        report: Some("receivable"),
                        row: 1,
                        cause: Box::new(NativeOutstandingsError::InvalidAmount),
                    })
                );
            }
            error
                .downcast_ref::<RuntimeReadFailure>()
                .unwrap()
                .evidence
                .clone()
        };
        let observed = simulator.finish().unwrap();
        assert_eq!(observed.len(), 30);
        let req = |i: usize| observed[i].request_body_sha256.clone();
        let res = |i: usize| sha256_hex(&responses[i]);
        let request = [7, 11, 15, 19]
            .iter()
            .fold(join(&req(0), &req(1)), |prior, i| join(&prior, &req(*i)));
        let response = [7, 11, 15, 19]
            .iter()
            .fold(join(&res(0), &res(1)), |prior, i| join(&prior, &res(*i)));
        assert_eq!(
            evidence.request_sha256,
            join(&request, &join(&req(28), &req(29)))
        );
        assert_eq!(
            evidence.response_sha256,
            join(&response, &join(&res(28), &res(29)))
        );
        assert_eq!(
            evidence.bytes,
            [0, 1, 7, 9, 11, 13, 15, 17, 19, 21, 28, 29]
                .iter()
                .map(|i| responses[*i].len())
                .sum::<usize>()
        );
    }
}

#[test]
fn retained_evidence_does_not_change_runtime_error_classification() {
    let error = with_read_evidence(
        TallyRuntimeReadError::ApplicationResponseRejected.into(),
        RuntimeReadEvidence::empty(),
    );
    assert_eq!(classify_failure(&error), ReadFailureClass::Application);
    let error = with_read_evidence(
        TallyTransportError::ConnectionFailed.into(),
        RuntimeReadEvidence::empty(),
    );
    assert_eq!(classify_failure(&error), ReadFailureClass::Connection);
}

#[tokio::test]
async fn rejected_currency_retains_its_completed_pair_before_any_master_read() {
    let captured = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    for malformed in [false, true] {
        let faulty = if malformed {
            captured.replace(
                "<DECIMALPLACES TYPE=\"Number\"> 2</DECIMALPLACES>",
                "<DECIMALPLACES TYPE=\"Number\">invalid</DECIMALPLACES>",
            )
        } else {
            captured.replace(
                "<MAILINGNAME TYPE=\"String\">INR</MAILINGNAME>",
                "<MAILINGNAME TYPE=\"String\">USD</MAILINGNAME>",
            )
        };
        assert_ne!(faulty, captured);
        let encoded = encode(&faulty, WireEncoding::Utf16Le);
        let company = xml(companies());
        let extent = xml(extents());
        let mut plans = vec![company.clone()];
        pair(&mut plans, extent.clone());
        pair(&mut plans, xml(faulty));
        // A response that does not parse is refused as soon as its pair is
        // read, so the closing extent and identity reads are not sent; a
        // parsed currency is admitted or refused only after them.
        if !malformed {
            pair(&mut plans, extent);
            plans.push(company);
        }
        let total = plans.len();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let error = TallyRuntime::default()
            .fetch_agent_party_ledger_masters_with_evidence(
                TallyConfig {
                    host: simulator.address().ip().to_string(),
                    port: simulator.address().port(),
                },
                &company_identity(&companies()),
                TallyDate::parse("20990101").unwrap(),
                crate::tally::connection::ComplianceRead::Standard,
            )
            .await
            .unwrap_err();
        if malformed {
            assert!(matches!(
                error
                    .chain()
                    .find_map(|cause| cause.downcast_ref::<NativeOutstandingsError>()),
                Some(NativeOutstandingsError::InvalidResponse(
                    "currency_decimal_places_invalid"
                ))
            ));
        } else {
            assert_eq!(error.to_string(), "company_base_currency_not_inr");
        }
        let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
        let observations = simulator.finish().unwrap();
        assert_eq!(observations.len(), total);
        assert_eq!(total, if malformed { 9 } else { 14 });
        assert_eq!(evidence.request_sha256, observations[5].request_body_sha256);
        assert_eq!(evidence.response_sha256, sha256_hex(&encoded));
        assert_eq!(evidence.bytes, 2 * encoded.len());
    }
}

/// The deadline of the reads below, short so a stalled answer cuts in seconds.
const EXPORT_DEADLINE: std::time::Duration = std::time::Duration::from_secs(2);
/// Where the master request sits among the captured book's compliance plans.
const MASTER: usize = 7;

/// The captured master answer, held past [`EXPORT_DEADLINE`] so its read is cut.
fn stalled_master() -> ScenarioPlan {
    compliance_plans(ADMITTED_PROFILE_VARIANTS[0])[MASTER]
        .clone()
        .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
            EXPORT_DEADLINE + std::time::Duration::from_millis(500),
        ))
}

/// The captured master answer, its head declaring a length past the response cap.
fn declared_too_large_master() -> ScenarioPlan {
    compliance_plans(ADMITTED_PROFILE_VARIANTS[0])[MASTER]
        .clone()
        .with_framing(ResponseFraming::DeclaredContentLength {
            bytes: bridge_tally_transport::XML_RESPONSE_MAX_BYTES + 1,
        })
}

/// The captured book's plans with `failing` answering the master request first, then the
/// captured book's master pair and everything after it.
fn plans_after_a_failed_master(failing: ScenarioPlan) -> Vec<ScenarioPlan> {
    let mut plans = compliance_plans(ADMITTED_PROFILE_VARIANTS[0]);
    let rest = plans.split_off(MASTER);
    plans.push(failing);
    plans.extend(rest);
    plans
}

/// The captured book's plans up to the master request, which `failing` answers, and no more.
fn plans_ending_with_a_failed_master(failing: ScenarioPlan) -> Vec<ScenarioPlan> {
    let mut plans = compliance_plans(ADMITTED_PROFILE_VARIANTS[0]);
    plans.truncate(MASTER);
    plans.push(failing);
    plans
}

/// One party-master read of the captured book (mark 219, inside the mailing bound, so the
/// export's master request names the mailing details) under [`EXPORT_DEADLINE`] and `wire`,
/// returned with the requests the book saw.
async fn export_read(
    read: crate::tally::connection::ComplianceRead,
    plans: Vec<ScenarioPlan>,
    wire: crate::endpoint_wire::WireGateConfig,
) -> (
    anyhow::Result<PartyLedgerMasterSource>,
    Vec<tally_protocol_simulator::ObservedRequest>,
) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let identity = company_identity(&companies());
    let result = TallyRuntime::with_transport_policy(bridge_tally_transport::TransportPolicy {
        request_timeout: EXPORT_DEADLINE,
        ..Default::default()
    })
    .with_wire_gate_config(wire)
    .fetch_party_ledger_master_source_with_evidence(
        TallyConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        &identity,
        assertion(&extents(), &identity),
        BalanceDateBasis::HostToday(TallyDate::parse("20990101").unwrap()),
        read,
    )
    .await
    .map(|(source, _)| source);
    (result, simulator.finish().unwrap())
}

fn mailing_cause(error: &anyhow::Error) -> Option<crate::tally::connection::MailingReadFailure> {
    error
        .chain()
        .find_map(|cause| {
            cause.downcast_ref::<crate::tally::connection::MailingDetailsReadFailed>()
        })
        .map(crate::tally::connection::MailingDetailsReadFailed::cause)
}

fn transport_errors(error: &anyhow::Error) -> Vec<&bridge_tally_transport::TallyTransportError> {
    error
        .chain()
        .filter_map(|cause| cause.downcast_ref::<bridge_tally_transport::TallyTransportError>())
        .collect()
}

/// bridge#2214: the export's mailing read refused past the response cap that Tally declared, the
/// one case measured live, is sent once more as the standard request, and the source is that
/// read's: every row, the standard requests' commitment, and the failure named. The mailing
/// request was sent once, and nothing before the master pair was sent again. It is read under the
/// measured drain policy: a declared oversize owes no drain, so no `/status` probe comes before
/// the standard read.
#[tokio::test]
async fn an_export_whose_mailing_answer_tally_declared_too_large_is_read_again_without_the_details()
{
    use crate::endpoint_wire::WireGateConfig;
    use crate::tally::connection::{
        ComplianceRead::{Standard, WithMailingDetailsWhereWholeReadFits as Export},
        MailingReadFailure,
    };
    let (standard, standard_seen) = export_read(
        Standard,
        compliance_plans(ADMITTED_PROFILE_VARIANTS[0]),
        WireGateConfig::default(),
    )
    .await;
    let standard = standard.unwrap();
    assert_eq!(standard.mailing_details_read_failed, None);
    {
        let (failing, cause, wire) = (
            declared_too_large_master(),
            MailingReadFailure::ResponseTooLarge {
                declared_by_peer: true,
            },
            WireGateConfig::default().with_drain(bridge_tally_transport::DrainPolicy::MEASURED),
        );
        let (source, seen) = export_read(Export, plans_after_a_failed_master(failing), wire).await;
        let source = source.unwrap_or_else(|error| panic!("{cause:?}: {error:#}"));
        assert_eq!(seen.len(), standard_seen.len() + 1, "{cause:?}");
        let mailing = &seen[MASTER].request_body_sha256;
        assert_ne!(mailing, &standard_seen[MASTER].request_body_sha256);
        assert_eq!(
            seen.iter()
                .filter(|request| &request.request_body_sha256 == mailing)
                .count(),
            1,
            "{cause:?}: the mailing request is sent once"
        );
        let shas = |requests: &[tally_protocol_simulator::ObservedRequest]| {
            requests
                .iter()
                .map(|request| request.request_body_sha256.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(shas(&seen[..MASTER]), shas(&standard_seen[..MASTER]));
        assert_eq!(shas(&seen[MASTER + 1..]), shas(&standard_seen[MASTER..]));
        assert_eq!(source.rows.len(), standard.rows.len());
        assert_eq!(source.request_sha256, standard.request_sha256);
        assert_eq!(source.mailing_details_read_failed, Some(cause));
        assert!(note_kinds(&source).contains(&SourceNoteKind::MailingDetailsReadFailed));
    }
}

/// bridge#2214: a standard read that fails after the mailing read did is reported as itself,
/// with no mailing marker, and is not sent a third time.
#[tokio::test]
async fn a_failed_standard_read_after_a_failed_mailing_read_is_reported_as_itself() {
    use crate::tally::connection::ComplianceRead::WithMailingDetailsWhereWholeReadFits as Export;
    let mut plans = plans_ending_with_a_failed_master(declared_too_large_master());
    plans.push(stalled_master());
    let (result, seen) = export_read(
        Export,
        plans,
        crate::endpoint_wire::WireGateConfig::default(),
    )
    .await;
    let error = result.unwrap_err();
    assert_eq!(mailing_cause(&error), None, "{error:#}");
    assert!(
        matches!(
            transport_errors(&error)[..],
            [bridge_tally_transport::TallyTransportError::RequestTimedOut]
        ),
        "{error:#}"
    );
    assert_eq!(seen.len(), MASTER + 2);
    assert_ne!(
        seen[MASTER].request_body_sha256,
        seen[MASTER + 1].request_body_sha256
    );
}

/// bridge#2214: only the export's own kind is read again, and only after an answer Tally
/// declared past the response cap. A `ledger_masters` call that asked for the mailing details by
/// name, with either failure; a standard or GST-details read that timed out; and an export whose
/// mailing read timed out or whose answer broke off keep their failure, and nothing is sent after
/// it. A deadline is not read again until a cut mailing read has been seen live.
#[tokio::test]
async fn no_other_read_or_failure_is_read_again_without_the_details() {
    use crate::tally::connection::{
        ComplianceRead::{
            Standard, WithGstDetails, WithMailingDetails,
            WithMailingDetailsWhereWholeReadFits as Export,
        },
        MailingReadFailure,
    };
    let master = compliance_plans(ADMITTED_PROFILE_VARIANTS[0])[MASTER].clone();
    let broke_off = master
        .clone()
        .with_framing(ResponseFraming::DeclaredContentLength {
            bytes: master.response_bytes().len() + 2,
        });
    for (read, failing, cause) in [
        (
            WithMailingDetails,
            stalled_master(),
            Some(MailingReadFailure::Deadline),
        ),
        (
            WithMailingDetails,
            declared_too_large_master(),
            Some(MailingReadFailure::ResponseTooLarge {
                declared_by_peer: true,
            }),
        ),
        (Standard, stalled_master(), None),
        (WithGstDetails, stalled_master(), None),
        (Export, stalled_master(), Some(MailingReadFailure::Deadline)),
        (Export, broke_off, Some(MailingReadFailure::Truncated)),
    ] {
        let (result, seen) = export_read(
            read,
            plans_ending_with_a_failed_master(failing),
            crate::endpoint_wire::WireGateConfig::default(),
        )
        .await;
        let error = result.unwrap_err();
        assert_eq!(mailing_cause(&error), cause, "{read:?}: {error:#}");
        assert_eq!(transport_errors(&error).len(), 1, "{read:?}: {error:#}");
        assert_eq!(
            seen.len(),
            MASTER + 1,
            "{read:?}: nothing after the failure"
        );
    }
}

/// bridge#2214: a `/status` check between the mailing pair's posts that times out is not the
/// mailing read's failure: Tally answered the post. The export fails with the check's own error,
/// unmarked, and reads nothing again.
#[tokio::test]
async fn a_slow_status_check_inside_the_mailing_pair_is_not_the_mailing_reads_failure() {
    use crate::tally::connection::ComplianceRead::WithMailingDetailsWhereWholeReadFits as Export;
    let mut plans = compliance_plans(ADMITTED_PROFILE_VARIANTS[0]);
    plans.truncate(MASTER + 2);
    plans[MASTER + 1] = status().with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
        EXPORT_DEADLINE + std::time::Duration::from_millis(500),
    ));
    let (result, seen) = export_read(
        Export,
        plans,
        crate::endpoint_wire::WireGateConfig::default(),
    )
    .await;
    let error = result.unwrap_err();
    assert_eq!(mailing_cause(&error), None, "{error:#}");
    assert!(
        matches!(
            transport_errors(&error)[..],
            [bridge_tally_transport::TallyTransportError::RequestTimedOut]
        ),
        "{error:#}"
    );
    assert_eq!(seen.len(), MASTER + 2);
    assert_eq!(seen[MASTER + 1].path, "/status");
}

/// bridge#2214: only the master request names the mailing details, so a balance pair that times
/// out after the mailing master answered is told as itself and not read again. The captured
/// standard master answers the mailing request here: only that a master answered is used.
#[tokio::test]
async fn a_balance_pair_that_times_out_after_the_mailing_master_is_not_read_again() {
    use crate::tally::connection::ComplianceRead::WithMailingDetailsWhereWholeReadFits as Export;
    const BALANCE: usize = MASTER + 4;
    let mut plans = compliance_plans(ADMITTED_PROFILE_VARIANTS[0]);
    plans.truncate(BALANCE + 1);
    plans[BALANCE] =
        plans[BALANCE]
            .clone()
            .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
                EXPORT_DEADLINE + std::time::Duration::from_millis(500),
            ));
    let (result, seen) = export_read(
        Export,
        plans,
        crate::endpoint_wire::WireGateConfig::default(),
    )
    .await;
    let error = result.unwrap_err();
    assert_eq!(mailing_cause(&error), None, "{error:#}");
    // The balance post's own deadline, not a later read's failure.
    assert!(
        matches!(
            transport_errors(&error)[..],
            [bridge_tally_transport::TallyTransportError::RequestTimedOut]
        ),
        "{error:#}"
    );
    assert_eq!(seen.len(), BALANCE + 1, "nothing after the balance post");
}

#[path = "runtime_outstandings_currency_tests.rs"]
mod outstandings_currency_tests;

#[path = "runtime_failure_evidence_inventory_tests.rs"]
mod failure_evidence_inventory_tests;
