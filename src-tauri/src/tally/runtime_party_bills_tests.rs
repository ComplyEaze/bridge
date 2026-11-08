use super::*;
use bridge_tally_transport::TallyTransportError;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

/// Tally answers a party name it does not match with HTTP 200 and no bytes, and
/// no `Content-Type` (`lab/2377-l-answers`, twice). The transport refuses that
/// answer by its missing content type before any parser sees it, so the party's
/// read fails with that typed refusal and never reaches a tie as zero bills.
#[tokio::test]
async fn an_empty_answer_without_a_content_type_is_a_transport_refusal() {
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
    assert_eq!(
        error
            .chain()
            .find_map(|cause| cause.downcast_ref::<TallyTransportError>()),
        Some(&TallyTransportError::InvalidEncoding {
            code: "response_content_type_unsupported",
        }),
        "{error:#}"
    );
}
