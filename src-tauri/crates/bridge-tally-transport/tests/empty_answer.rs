//! #2377: the empty answer (protocol reference §12a.27) over a real socket. The captured head is
//! [`TallyTransportError::EmptyAnswer`] on both readers; a head that misses one of its conditions
//! keeps the typed refusal it had before, so a transport failure never passes as the answer.
//!
//! The byte condition cannot be missed over HTTP: `Content-Length: 0` frames the body at zero,
//! and anything after it is the next response. It is pinned in the crate's unit tests.
#![allow(
    clippy::disallowed_methods,
    reason = "test double: a local socket serving one captured or edited response head"
)]
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::thread::JoinHandle;

use bridge_tally_transport::{TallyEndpointConfig, TallyHttpTransport, TallyTransportError};

/// The head Tally sent for a party it reported nothing for, byte for byte.
const CAPTURED_HEAD: &[u8] = include_bytes!(
    "../../bridge-tally-protocol/tests/fixtures/agent/party-bills/ledger-outstandings-unknown-party.headers.txt"
);

const CONTENT_TYPE_MISSING: TallyTransportError = TallyTransportError::InvalidEncoding {
    code: "response_content_type_unsupported",
};

/// Serves `response` once to the first request, after reading that request whole.
fn serve_once(response: Vec<u8>) -> (TallyEndpointConfig, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        let body_start = loop {
            let read = socket.read(&mut buffer).unwrap();
            assert!(read > 0, "the request ended before its head");
            request.extend_from_slice(&buffer[..read]);
            if let Some(end) = request.windows(4).position(|window| window == b"\r\n\r\n") {
                break end + 4;
            }
        };
        let head = String::from_utf8_lossy(&request[..body_start]).to_ascii_lowercase();
        let declared = head
            .lines()
            .find_map(|line| line.strip_prefix("content-length:"))
            .map_or(0, |length| length.trim().parse::<usize>().unwrap());
        while request.len() < body_start + declared {
            let read = socket.read(&mut buffer).unwrap();
            assert!(read > 0, "the request ended before its declared body");
            request.extend_from_slice(&buffer[..read]);
        }
        socket.write_all(&response).unwrap();
        socket.flush().unwrap();
        let _ = socket.shutdown(Shutdown::Write);
    });
    (
        TallyEndpointConfig {
            host: address.ip().to_string(),
            port: address.port(),
        },
        server,
    )
}

/// The same head through both readers, each against its own socket.
async fn both_readers(response: &[u8]) -> [Result<(), TallyTransportError>; 2] {
    let (endpoint, server) = serve_once(response.to_vec());
    let retained = TallyHttpTransport::new(endpoint)
        .unwrap()
        .post_xml("<ENVELOPE/>".to_string())
        .await
        .map(drop);
    server.join().unwrap();
    let (endpoint, server) = serve_once(response.to_vec());
    let decoded = TallyHttpTransport::new(endpoint)
        .unwrap()
        .post_xml_decoded("<ENVELOPE/>".to_string())
        .await
        .map(drop);
    server.join().unwrap();
    [retained, decoded]
}

fn edited(from: &str, to: &str) -> Vec<u8> {
    let head = std::str::from_utf8(CAPTURED_HEAD).unwrap();
    assert_eq!(head.matches(from).count(), 1, "{from}");
    head.replace(from, to).into_bytes()
}

#[tokio::test]
async fn the_captured_empty_head_is_the_empty_answer() {
    assert!(CAPTURED_HEAD.ends_with(b"CONTENT-LENGTH: 0\r\n\r\n"));
    assert_eq!(
        both_readers(CAPTURED_HEAD).await,
        [
            Err(TallyTransportError::EmptyAnswer),
            Err(TallyTransportError::EmptyAnswer)
        ]
    );
}

#[tokio::test]
async fn an_empty_head_with_another_success_status_keeps_its_refusal() {
    assert_eq!(
        both_readers(&edited("HTTP/1.1 200 OK", "HTTP/1.1 201 Created")).await,
        [Err(CONTENT_TYPE_MISSING), Err(CONTENT_TYPE_MISSING)]
    );
}

#[tokio::test]
async fn an_empty_body_ended_by_the_connection_with_no_length_keeps_its_refusal() {
    let response = edited(
        "CONNECTION: KEEP-ALIVE\r\nCONTENT-LENGTH: 0\r\n",
        "CONNECTION: close\r\n",
    );
    assert_eq!(
        both_readers(&response).await,
        [Err(CONTENT_TYPE_MISSING), Err(CONTENT_TYPE_MISSING)]
    );
}

#[tokio::test]
async fn a_body_with_no_type_keeps_its_refusal() {
    let response = edited("CONTENT-LENGTH: 0\r\n\r\n", "CONTENT-LENGTH: 2\r\n\r\n<A");
    assert_eq!(
        both_readers(&response).await,
        [Err(CONTENT_TYPE_MISSING), Err(CONTENT_TYPE_MISSING)]
    );
}

#[tokio::test]
async fn an_empty_head_with_a_content_type_is_decoded_as_before() {
    let response = edited(
        "CONTENT-LENGTH: 0\r\n",
        "CONTENT-TYPE: text/xml; charset=utf-16\r\nCONTENT-LENGTH: 0\r\n",
    );
    // A typed empty body is decoded as an empty text, as it was before; the parser behind the
    // read decides what an empty text means.
    assert_eq!(both_readers(&response).await, [Ok(()), Ok(())]);
}

#[tokio::test]
async fn an_empty_head_with_a_content_encoding_is_refused_by_its_encoding() {
    let response = edited(
        "CONTENT-LENGTH: 0\r\n",
        "CONTENT-ENCODING: gzip\r\nCONTENT-LENGTH: 0\r\n",
    );
    assert_eq!(
        both_readers(&response).await,
        [
            Err(TallyTransportError::UnsupportedContentEncoding),
            Err(TallyTransportError::UnsupportedContentEncoding)
        ]
    );
}
