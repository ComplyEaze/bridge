//! The party bills read against a local socket serving captured answer bytes.
#![allow(
    clippy::disallowed_methods,
    reason = "test double: a local socket serving captured bytes"
)]
use super::*;
use bridge_tally_protocol::native_outstandings::parse_native_ledger_snapshot;
use bridge_tally_transport::TallyTransportError;
use std::collections::VecDeque;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/crates/bridge-tally-protocol/tests/fixtures/agent/party-bills/"
);

/// The head and (empty) body Tally sent for a party it reported nothing for, byte for byte.
const EMPTY_HEAD: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/party-bills/ledger-outstandings-unknown-party.headers.txt"
);

fn fixture_text(name: &str) -> String {
    let bytes = std::fs::read(format!("{FIXTURES}{name}")).unwrap();
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16(&units).unwrap()
}

/// A captured report answer with a head of the shape Tally sends with a body.
fn report_answer(name: &str) -> Vec<u8> {
    let body = std::fs::read(format!("{FIXTURES}{name}")).unwrap();
    let head = format!(
        "HTTP/1.1 200 OK\r\nCONTENT-TYPE: text/xml; charset=utf-16\r\nRESPSTATUS: 1\r\nCONNECTION: KEEP-ALIVE\r\nCONTENT-LENGTH: {}\r\n\r\n",
        body.len()
    );
    [head.as_bytes(), &body].concat()
}

fn status_answer() -> Vec<u8> {
    let body = b"<RESPONSE>TallyPrime Server is Running</RESPONSE>";
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/xml; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    [head.as_bytes(), body].concat()
}

/// One request read to the end of its declared body, or `None` when the client closed the
/// connection before sending one.
async fn next_request(socket: &mut tokio::net::TcpStream) -> Option<Vec<u8>> {
    let mut request = Vec::new();
    let mut chunk = [0_u8; 4096];
    let head_end = loop {
        if let Some(end) = request.windows(4).position(|window| window == b"\r\n\r\n") {
            break end + 4;
        }
        let read = socket.read(&mut chunk).await.unwrap();
        if read == 0 {
            assert!(request.is_empty(), "a request ended before its head");
            return None;
        }
        request.extend_from_slice(&chunk[..read]);
    };
    let head = String::from_utf8_lossy(&request[..head_end]).to_ascii_lowercase();
    let declared = head
        .lines()
        .find_map(|line| line.strip_prefix("content-length:"))
        .map_or(0, |length| length.trim().parse::<usize>().unwrap());
    while request.len() < head_end + declared {
        let read = socket.read(&mut chunk).await.unwrap();
        assert!(read > 0, "a request ended before its declared body");
        request.extend_from_slice(&chunk[..read]);
    }
    Some(request)
}

/// Answers each request in turn with the next `(method, answer)`, after checking its method,
/// keeping a connection open across answers that keep it alive, as Tally's do. Returns how many
/// answers were sent.
fn serve(answers: Vec<(&'static str, Vec<u8>)>) -> (TallyClient, tokio::task::JoinHandle<usize>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let listener = TcpListener::from_std(listener).unwrap();
    let mut answers = VecDeque::from(answers);
    let server = tokio::spawn(async move {
        let mut sent = 0;
        while !answers.is_empty() {
            let (mut socket, _) = listener.accept().await.unwrap();
            while let Some(request) = next_request(&mut socket).await {
                let (method, answer) = answers.pop_front().expect("no request past the plan");
                assert!(
                    request.starts_with(method.as_bytes()),
                    "answer {sent} expected {method}"
                );
                socket.write_all(&answer).await.unwrap();
                socket.flush().await.unwrap();
                sent += 1;
                let closes = String::from_utf8_lossy(&answer)
                    .to_ascii_lowercase()
                    .contains("connection: close");
                if closes || answers.is_empty() {
                    let _ = socket.shutdown().await;
                    break;
                }
            }
        }
        sent
    });
    let client = TallyClient::new(TallyConfig {
        host: address.ip().to_string(),
        port: address.port(),
    })
    .unwrap();
    (client, server)
}

fn outstandings_request() -> String {
    render_ledger_outstandings_request(
        "BRIDGE PILOT LAB",
        &LedgerScopeName::new("BRIDGE Supplier RJ").unwrap(),
        &TallyDate::parse("20260731").unwrap(),
    )
}

/// The pair's own drift, by its typed value.
fn is_party_bills_drift(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<PairedReadValidationError>(),
            Some(PairedReadValidationError::PartyBills)
        )
    })
}

/// Tally answers a party it reports nothing for with HTTP 200, one `Content-Length: 0`, no
/// `Content-Type` and no bytes (protocol reference §12a.27). An ordinary report read does not
/// ask for that answer, so it fails with no second send. Its typed cause is now `EmptyAnswer`,
/// which carries the message and safe code the missing type always had.
#[tokio::test]
async fn an_ordinary_report_read_of_the_empty_answer_fails_as_before() {
    let head = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/party-bills/ledger-outstandings-unknown-party.headers.txt"
    );
    let body = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/party-bills/ledger-outstandings-unknown-party.utf16le.xml"
    );
    assert!(body.is_empty());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let request =
            crate::tally::connection::tests::read_complete_http_request(&mut socket).await;
        assert!(!request.is_empty());
        socket.write_all(head).await.unwrap();
        socket.write_all(body).await.unwrap();
        socket.shutdown().await.unwrap();
    });
    let client = TallyClient::new(TallyConfig {
        host: address.ip().to_string(),
        port: address.port(),
    })
    .unwrap();
    let party = LedgerScopeName::new("Lab Party Q9").unwrap();
    let to = TallyDate::parse("20260331").unwrap();
    let Err(error) = client
        .fetch_native_report_paired(render_ledger_outstandings_request(
            "BRIDGE BILLTRAIL LAB",
            &party,
            &to,
        ))
        .await
    else {
        panic!("an empty answer is not a report");
    };
    server.await.unwrap();
    let transport = error
        .chain()
        .find_map(|cause| cause.downcast_ref::<TallyTransportError>());
    assert_eq!(
        transport,
        Some(&TallyTransportError::EmptyAnswer),
        "{error:#}"
    );
    assert_eq!(
        transport.unwrap().to_string(),
        TallyTransportError::InvalidEncoding {
            code: "response_content_type_unsupported"
        }
        .to_string()
    );
}

/// The party-bills read asks for the empty answer by name: twice is a value, recorded at 0 bytes
/// with the SHA-256 of no input, after both health checks.
#[tokio::test]
async fn the_party_bills_pair_admits_the_empty_answer_twice() {
    let (client, server) = serve(vec![
        ("POST", EMPTY_HEAD.to_vec()),
        ("GET", status_answer()),
        ("POST", EMPTY_HEAD.to_vec()),
        ("GET", status_answer()),
    ]);
    let read = client
        .fetch_party_bills_paired(outstandings_request())
        .await;
    assert_eq!(server.await.unwrap(), 4);
    let (body, bytes, hash) = read.unwrap();
    assert_eq!(body, PartyBillsAnswerBody::Empty);
    assert_eq!(bytes, 0);
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

/// A body answered twice is read as the report read reads it.
#[tokio::test]
async fn the_party_bills_pair_reads_a_body_twice_as_before() {
    let answer = report_answer("ledger-outstandings-p1-20260731.utf16le.xml");
    let (client, server) = serve(vec![
        ("POST", answer.clone()),
        ("GET", status_answer()),
        ("POST", answer),
        ("GET", status_answer()),
    ]);
    let read = client
        .fetch_party_bills_paired(outstandings_request())
        .await;
    assert_eq!(server.await.unwrap(), 4);
    let (body, bytes, _) = read.unwrap();
    let PartyBillsAnswerBody::Rows(text) = body else {
        panic!("a body is rows");
    };
    assert_eq!(
        text,
        fixture_text("ledger-outstandings-p1-20260731.utf16le.xml")
    );
    assert_eq!(bytes, 3926);
}

/// Empty once beside a body, in either order, is the pair's drift, never either answer.
#[tokio::test]
async fn an_empty_answer_beside_a_body_is_the_pairs_drift() {
    let body = report_answer("ledger-outstandings-p1-20260731.utf16le.xml");
    for (first, second) in [
        (EMPTY_HEAD.to_vec(), body.clone()),
        (body.clone(), EMPTY_HEAD.to_vec()),
    ] {
        let (client, server) = serve(vec![
            ("POST", first),
            ("GET", status_answer()),
            ("POST", second),
            ("GET", status_answer()),
        ]);
        let error = client
            .fetch_party_bills_paired(outstandings_request())
            .await
            .unwrap_err();
        assert_eq!(server.await.unwrap(), 4);
        assert!(is_party_bills_drift(&error), "{error:#}");
        assert_eq!(
            error
                .chain()
                .find_map(|cause| cause.downcast_ref::<TallyTransportError>()),
            None,
            "{error:#}"
        );
    }
}

/// An empty head that misses one condition (here its status) is the transport's refusal, never
/// the empty answer and never the pair's drift.
#[tokio::test]
async fn an_empty_head_that_is_not_the_captured_one_stays_a_transport_refusal() {
    let not_captured = std::str::from_utf8(EMPTY_HEAD)
        .unwrap()
        .replace("HTTP/1.1 200 OK", "HTTP/1.1 201 Created")
        .into_bytes();
    let (client, server) = serve(vec![
        ("POST", EMPTY_HEAD.to_vec()),
        ("GET", status_answer()),
        ("POST", not_captured),
    ]);
    let error = client
        .fetch_party_bills_paired(outstandings_request())
        .await
        .unwrap_err();
    assert_eq!(server.await.unwrap(), 3);
    assert!(!is_party_bills_drift(&error), "{error:#}");
    assert_eq!(
        error
            .chain()
            .find_map(|cause| cause.downcast_ref::<TallyTransportError>()),
        Some(&TallyTransportError::InvalidEncoding {
            code: "response_content_type_unsupported"
        }),
        "{error:#}"
    );
}

fn snapshot(name: &str) -> Vec<LedgerSnapshotEntry> {
    parse_native_ledger_snapshot(&fixture_text(name)).unwrap()
}

/// The snapshot decides what the empty answer is, each case on a captured snapshot. Row 2377-NB's
/// bill-wise party before its first entry has an empty closing, so the read refuses as none
/// reported; a bill-wise party with a closing refuses as an answer missing; a name the snapshot
/// does not carry and a ledger that keeps no bills refuse by those codes first.
#[test]
fn the_snapshot_decides_what_the_empty_answer_is() {
    const NB: &str = "ledger-snapshot-supplier-rj-20260731.utf16le.xml";
    const P1: &str = "ledger-snapshot-p1-20260731.utf16le.xml";
    let supplier = snapshot(NB)
        .into_iter()
        .find(|entry| entry.name == "BRIDGE Supplier RJ")
        .unwrap();
    assert!(supplier.bill_wise_on && supplier.closing_balance.is_none());
    assert!(matches!(
        tie_to_snapshot(snapshot(NB), "BRIDGE Supplier RJ", None),
        Err(PartyBillsReadError::NoneReported)
    ));
    assert!(matches!(
        tie_to_snapshot(snapshot(P1), "Lab Party P1", None),
        Err(PartyBillsReadError::AnswerMissing)
    ));
    assert!(matches!(
        tie_to_snapshot(snapshot(P1), "Lab Party Q9", None),
        Err(PartyBillsReadError::NotInSnapshot)
    ));
    for (name, ledger) in [(NB, "BRIDGE Walk-in"), (P1, "Cash")] {
        assert!(matches!(
            tie_to_snapshot(snapshot(name), ledger, None),
            Err(PartyBillsReadError::NotBillWise)
        ));
    }
    assert_eq!(
        PartyBillsReadError::NoneReported.safe_code(),
        "party_bills_none_reported"
    );
    assert_eq!(
        PartyBillsReadError::AnswerMissing.safe_code(),
        "party_bills_answer_missing"
    );
}

/// A body is tied as before, and a name the snapshot does not carry still refuses first.
#[test]
fn a_body_is_tied_to_the_snapshot_as_before() {
    let answer = || {
        Some(parse_ledger_outstandings_answer(
            &fixture_text("ledger-outstandings-p1-20260731.utf16le.xml"),
            &TallyDate::parse("20250401").unwrap(),
            &TallyDate::parse("20260731").unwrap(),
        ))
    };
    let read = tie_to_snapshot(
        snapshot("ledger-snapshot-p1-20260731.utf16le.xml"),
        "Lab Party P1",
        answer(),
    )
    .unwrap();
    assert_eq!((read.bills().len(), read.on_account().len()), (4, 1));
    assert!(matches!(
        tie_to_snapshot(
            snapshot("ledger-snapshot-supplier-rj-20260731.utf16le.xml"),
            "Lab Party P1",
            answer()
        ),
        Err(PartyBillsReadError::NotInSnapshot)
    ));
}
