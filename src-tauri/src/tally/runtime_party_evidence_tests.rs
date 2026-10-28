//! Refusal replay of captured reports; UTF-8 fault captures are sent in the
//! negotiated UTF-16LE encoding without changing their XML content.
#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::*;
use crate::tally::connection::PartyLedgerMasterSourceValidationError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const MASTER: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-masters.utf16le.xml"
);
const IDENTITY_MISSING_BALANCE: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/native/ledger_snapshot_aarav.xml"
);
const BALANCE: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-balances.utf16le.xml"
);
const IDENTITY_MISSING_GROUP: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/native/group_snapshot_aarav.xml"
);
const EXTENT: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
);
const BILLS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/native/bills_receivable_aarav.xml"
);

fn captured_identity(extent: &str) -> VerifiedCompanyIdentity {
    let companies = bridge_tally_protocol::parse_companies_from_collection(extent).unwrap();
    let company = companies
        .iter()
        .find(|company| company.guid.as_deref() == Some(GUID))
        .expect("captured extent includes the party-master company");
    VerifiedCompanyIdentity::from_observed_companies(
        company.name.clone(),
        crate::tally::test_company_guid(&company.guid.clone().unwrap()),
        company.company_number.clone().unwrap(),
        company.books_from.clone().unwrap(),
        &companies,
    )
    .expect("captured party-master tuple is uniquely verified")
}

async fn refusal(
    retained: &[&'static [u8]],
    fail_transport: bool,
) -> (anyhow::Error, RuntimeReadEvidence) {
    refusal_with(
        std::str::from_utf8(EXTENT).unwrap(),
        crate::tally::connection::ComplianceRead::Standard,
        retained,
        fail_transport,
    )
    .await
}

/// `refusal` for a book whose extent and read kind are chosen.
async fn refusal_with(
    extent: &str,
    read: crate::tally::connection::ComplianceRead,
    retained: &[&'static [u8]],
    fail_transport: bool,
) -> (anyhow::Error, RuntimeReadEvidence) {
    let identity = captured_identity(extent);
    let parsed_extent = bridge_tally_protocol::outstandings_shared::parse_company_book_extent_v2(
        extent,
        &identity.company_book_extent_expectation().unwrap(),
    )
    .unwrap();
    let assertion = PartyLedgerMasterCurrencyAssertion {
        assertion: OutstandingsCurrencyAssertion::Inr,
        decimal_places: 2,
        base: single_master_base_for_tests(),
        currency_read_extent: parsed_extent,
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let mut bodies = vec![bridge_tally_protocol::encode_tally_xml_request_utf16le(
        extent,
    )];
    bodies.extend(retained.iter().map(|body| {
        if body.get(1) == Some(&0) || body.starts_with(&[0xff, 0xfe]) {
            body.to_vec()
        } else {
            bridge_tally_protocol::encode_tally_xml_request_utf16le(
                std::str::from_utf8(body).unwrap(),
            )
        }
    }));
    if fail_transport {
        bodies.push(Vec::new());
    }
    let server = tokio::spawn(async move {
        let mut evidence = RuntimeReadEvidence::empty();
        for (index, body) in bodies.iter().enumerate() {
            let mut requests = Vec::new();
            for paired in 0..4 {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut headers = Vec::new();
                while !headers.ends_with(b"\r\n\r\n") {
                    headers.push(socket.read_u8().await.unwrap());
                    assert!(headers.len() <= 64 * 1024);
                }
                let headers = String::from_utf8(headers).unwrap();
                let length = headers
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                assert!(length <= 1024 * 1024);
                let mut request = vec![0; length];
                socket.read_exact(&mut request).await.unwrap();
                if fail_transport && index + 1 == bodies.len() {
                    assert_eq!(paired, 0);
                    assert!(headers.starts_with("POST / "));
                    socket.write_all(b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").await.unwrap();
                    socket.shutdown().await.unwrap();
                    return evidence;
                }
                let response_body = if paired % 2 == 0 {
                    assert!(headers.starts_with("POST / "));
                    requests.push(request);
                    body.as_slice()
                } else {
                    assert!(headers.starts_with("GET /status "));
                    b"<RESPONSE>TallyPrime Server is Running</RESPONSE>"
                };
                let encoding = if response_body.starts_with(&[0xff, 0xfe])
                    || response_body.get(1) == Some(&0)
                {
                    "utf-16"
                } else {
                    "utf-8"
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/xml; charset={encoding}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", response_body.len()
                );
                socket.write_all(response.as_bytes()).await.unwrap();
                socket.write_all(response_body).await.unwrap();
                socket.shutdown().await.unwrap();
            }
            assert_eq!(requests[0], requests[1]);
            if index > 0 {
                evidence = evidence.combine(RuntimeReadEvidence {
                    request_sha256: sha256_hex(&requests[0]),
                    response_sha256: sha256_hex(body),
                    bytes: body.len() * 2,
                });
            }
        }
        evidence
    });
    let client = TallyClient::new(TallyConfig {
        host: address.ip().to_string(),
        port: address.port(),
    })
    .unwrap();
    let error = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        client.fetch_party_ledger_master_source(
            &identity,
            DateBoundaryProfile::ModeAgnostic,
            assertion,
            &BalanceDateBasis::HostToday(TallyDate::parse("20990101").unwrap()),
            read,
        ),
    )
    .await
    .unwrap()
    .unwrap_err();
    let expected = tokio::time::timeout(std::time::Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
    (error, expected)
}

fn assert_retained(error: &anyhow::Error, expected: RuntimeReadEvidence) {
    let actual = &error
        .downcast_ref::<RuntimeReadFailure>()
        .expect("completed paired reads survive refusal")
        .evidence;
    assert_eq!(actual.bytes, expected.bytes);
    assert_eq!(actual.request_sha256, expected.request_sha256);
    assert_eq!(actual.response_sha256, expected.response_sha256);
}

#[tokio::test]
async fn party_master_parse_refusal_retains_its_actual_paired_wire_evidence() {
    // An unchanged captured Bills response is deliberately sent to the master
    // parser. This is a wrong-response fault, never positive qualification.
    let (error, expected) = refusal(&[BILLS], false).await;
    assert!(error.chain().any(|cause| matches!(
        cause.downcast_ref::<PartyLedgerMasterSourceValidationError>(),
        Some(PartyLedgerMasterSourceValidationError::MasterResponseInvalid { .. })
    )));
    assert_retained(&error, expected);
}

#[tokio::test]
async fn party_balance_identity_refusal_retains_master_and_balance_wire_evidence() {
    // The captured balance report omits company GUID. It must remain refused,
    // while both the earlier accepted master and rejected balance are audited.
    let (error, expected) = refusal(&[MASTER, IDENTITY_MISSING_BALANCE], false).await;
    assert!(
        error.chain().any(|cause| matches!(
            cause.downcast_ref::<PartyLedgerMasterSourceValidationError>(),
            Some(PartyLedgerMasterSourceValidationError::BalanceCompanyIdentityUnverified)
        )),
        "unexpected typed refusal: {error:?}"
    );
    assert_retained(&error, expected);
}

#[tokio::test]
async fn party_group_identity_refusal_retains_all_completed_reports() {
    // A captured group response lacking computed company identity is refused
    // after the unchanged captured master and balance responses are accepted.
    let (error, expected) = refusal(&[MASTER, BALANCE, IDENTITY_MISSING_GROUP], false).await;
    assert!(
        error.chain().any(|cause| matches!(
            cause.downcast_ref::<PartyLedgerMasterSourceValidationError>(),
            Some(PartyLedgerMasterSourceValidationError::GroupCompanyIdentityUnverified)
        )),
        "unexpected typed refusal: {error:?}"
    );
    assert_retained(&error, expected);
}

#[tokio::test]
async fn party_later_transport_refusal_preserves_completed_master_pair() {
    let (error, expected) = refusal(&[MASTER], true).await;
    assert!(
        error.chain().any(|cause| matches!(
            cause.downcast_ref::<bridge_tally_transport::TallyTransportError>(),
            Some(bridge_tally_transport::TallyTransportError::HttpStatus { status: 503 })
        )),
        "unexpected typed refusal: {error:?}"
    );
    assert_retained(&error, expected);
}

/// The captured extent with the captured company's master-alteration mark set to `mark`.
fn extent_with_master_mark(mark: u64) -> String {
    let extent = std::str::from_utf8(EXTENT).unwrap();
    let from = "<ALTMSTID TYPE=\"Number\"> 219</ALTMSTID>";
    let at = extent.find(GUID).expect("the captured company's extent");
    let start = extent[..at].rfind("<COMPANY ").unwrap();
    let end = at + extent[at..].find("</COMPANY>").unwrap();
    assert_eq!(extent[start..end].matches(from).count(), 1);
    let company = extent[start..end].replace(
        from,
        &format!("<ALTMSTID TYPE=\"Number\"> {mark}</ALTMSTID>"),
    );
    format!("{}{}{}", &extent[..start], company, &extent[end..])
}

/// The party/ledger master export (bridge#1490) sends the request a standard read sends for a book
/// whose master-alteration mark is past the mailing bound, so nothing it read before is sized or
/// refused by the larger estimate; within the bound it sends the mailing request. Compared by the
/// hash of the first ledger request actually sent, through the client.
#[tokio::test]
async fn the_export_sends_the_standard_request_for_a_book_past_the_mailing_bound() {
    use crate::tally::connection::ComplianceRead::{
        Standard, WithMailingDetails, WithMailingDetailsWhereWholeReadFits as Export,
    };
    let sent = |mark: u64, read| async move {
        let (_, evidence) =
            refusal_with(&extent_with_master_mark(mark), read, &[BILLS], false).await;
        evidence.request_sha256
    };
    // A mark in 3,405 to 4,266: the standard read admits the whole book by the mark alone.
    assert_eq!(sent(3_500, Export).await, sent(3_500, Standard).await);
    assert_eq!(sent(4_266, Export).await, sent(4_266, Standard).await);
    // The control: the mailing kind would not take that path (it counts the ledgers first).
    assert_ne!(
        sent(3_500, WithMailingDetails).await,
        sent(3_500, Standard).await
    );
    // Inside the bound the export names the mailing details.
    assert_eq!(sent(219, Export).await, sent(219, WithMailingDetails).await);
    assert_ne!(sent(219, Export).await, sent(219, Standard).await);
}
