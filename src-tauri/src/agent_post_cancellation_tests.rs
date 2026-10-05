//! MCP input lifecycle tests; no simulated Tally protocol is needed.
use super::*;
use crate::agent::agent_import::{ledger, ImportLedgerLine};
use std::{
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, ReadBuf};

/// What a withdrawn post answers once it stops. Its answer is replaced by the
/// cancellation's, so what it holds is never read.
fn stand_in_response() -> ToolResponse {
    ToolResponse {
        value: json!({}),
        egress: EgressContext {
            evidence: None,
            tool: "post_import".into(),
            args_sha256: sha256_hex(b"post"),
            company_guid: None,
            request_trail: None,
        },
        recovery_batch_id: None,
    }
}

/// A post that stops when withdrawn, as a real one does before its next queued
/// Tally operation (#725). A withdrawn post is awaited until it stops, so a
/// stand-in that never stopped would hold the call forever.
async fn stand_in(withdrawal: tokio_util::sync::CancellationToken) -> ToolResponse {
    withdrawal.cancelled().await;
    stand_in_response()
}

/// Awaits a call whose post is withdrawn. A withdrawal that no longer stops the
/// post fails the test here instead of hanging the suite.
async fn stops<F: std::future::Future>(call: F) -> F::Output {
    tokio::time::timeout(std::time::Duration::from_secs(5), call)
        .await
        .expect("a withdrawn post stops once its token is cancelled")
}

fn server(path: &Path) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: path.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    })
}

fn saved_batch(server: &Server) -> (ImportLedgerLine, Value) {
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000001", "identity_scheme":"batch_v1",
        "company_guid":"00000000-0000-4000-8000-000000000002",
        "endpoint_origin":"http://127.0.0.1:9",
        "company":{"name":"Synthetic Accounts","guid":"00000000-0000-4000-8000-000000000002","company_number":"100001","books_from":"20260401"},
        "txn_ids":["journal-test"],"date_from":"20260901","date_to":"20260901",
        "sha256":"test", "built_at":"2026-09-07T00:00:00Z", "status":"built",
        "pre_import_mark":{"kind":"company_high_water","value":1,"master_value":1},
        "vouchers":[{"bridge_txn_id":"journal-test","date":"20260901","voucher_type":"Journal",
            "narration":"Synthetic test only","reference":"REF-1","entries":[
                {"ledger":"Expense","amount":"12.50","side":"Dr"},
                {"ledger":"Cash","amount":"12.50","side":"Cr"}]}]
    }))
    .unwrap();
    let args = json!({
        "company_guid":"00000000-0000-4000-8000-000000000002",
        "batch_id":"bridge-00000000-0000-4000-8000-000000000001"
    });
    server.append_import_ledger(&line).unwrap();
    (line, args)
}

#[tokio::test]
async fn contended_cancellation_answers_ping_and_suspends_the_post() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let (_, args) = saved_batch(&server);
    let admission = server.lock_import_admission().unwrap();
    assert_eq!(
        post_dispatch_state(&server, &args),
        PostDispatchState::AdmissionBusy
    );
    let polls = std::cell::Cell::new(0);
    let (client, source) = tokio::io::duplex(4096);
    let (client_read, mut client_write) = tokio::io::split(client);
    let (source_read, mut source_write) = tokio::io::split(source);
    let mut reader = BufReader::new(source_read);
    let (finished, mut completion) = tokio::sync::oneshot::channel();
    let serve = async {
        let result = await_post(
            std::future::poll_fn(|_| {
                polls.set(polls.get() + 1);
                if cancellation.is_cancelled() {
                    Poll::Ready(stand_in_response())
                } else {
                    Poll::<ToolResponse>::Pending
                }
            }),
            PostRequest {
                id: &json!(7),
                args: &args,
                cancellation: &cancellation,
            },
            &server,
            &mut reader,
            &mut Framer::default(),
            &mut std::collections::VecDeque::new(),
            &mut source_write,
        )
        .await;
        finished.send(()).unwrap();
        result
    };
    let client = async {
        client_write.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{\"requestId\":7}}\n{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"ping\"}\n").await.unwrap();
        let mut response = String::new();
        BufReader::new(client_read)
            .read_line(&mut response)
            .await
            .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&response).unwrap(),
            json!({"jsonrpc":"2.0","id":8,"result":{}})
        );
        let polls_after_cancellation = polls.get();
        // While another admission holds the journal, whether the post wrote an
        // intent cannot be read, so it is not polled at all.
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        assert_eq!(polls.get(), polls_after_cancellation);
        drop(admission);
        // Keep stdin active more often than the classifier retry period. This
        // catches a retry sleep that restarts after every incoming frame.
        let mut traffic = tokio::time::interval(std::time::Duration::from_millis(1));
        loop {
            tokio::select! {
                result = &mut completion => { result.unwrap(); break; }
                _ = traffic.tick() => {
                    client_write.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n").await.unwrap();
                }
            }
        }
        // Once it reads as not dispatched, the post is withdrawn (#725): it is
        // polled until it stops, as this stand-in does once withdrawn.
        assert!(polls.get() > polls_after_cancellation);
    };
    let (result, ()) = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        tokio::join!(serve, client)
    })
    .await
    .unwrap();
    assert!(result.unwrap().is_none());
    assert_eq!(
        post_dispatch_state(&server, &args),
        PostDispatchState::NotDispatched
    );
}

#[tokio::test]
async fn cancellation_after_intent_keeps_answering_ping_until_post_completes() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let (line, args) = saved_batch(&server);
    let admission = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
        .unwrap();
    drop(admission);
    let (client, source) = tokio::io::duplex(4096);
    let (client_read, mut client_write) = tokio::io::split(client);
    let (source_read, mut source_write) = tokio::io::split(source);
    let mut reader = BufReader::new(source_read);
    let mut pending = std::collections::VecDeque::new();
    let (complete, wait_for_completion) = tokio::sync::oneshot::channel();
    let id = json!(7);
    let mut framer = Framer::default();
    let serve = await_post(
        async move { wait_for_completion.await.unwrap() },
        PostRequest {
            id: &id,
            args: &args,
            cancellation: &cancellation,
        },
        &server,
        &mut reader,
        &mut framer,
        &mut pending,
        &mut source_write,
    );
    let client = async {
        client_write.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{\"requestId\":7}}\n{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"ping\"}\n").await.unwrap();
        let mut response = String::new();
        BufReader::new(client_read)
            .read_line(&mut response)
            .await
            .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&response).unwrap(),
            json!({"jsonrpc":"2.0","id":8,"result":{}})
        );
        assert!(complete
            .send(ToolResponse {
                value: json!({}),
                egress: EgressContext {
                    evidence: None,
                    tool: "post_import".into(),
                    args_sha256: sha256_hex(b"post"),
                    company_guid: None,
                    request_trail: None,
                },
                recovery_batch_id: None
            })
            .is_ok());
    };
    let (outcome, ()) = tokio::time::timeout(std::time::Duration::from_secs(1), async {
        tokio::join!(serve, client)
    })
    .await
    .unwrap();
    assert!(outcome.unwrap().is_some());
}

#[tokio::test]
async fn ping_responds_before_pending_approval_and_keeps_tools_queued() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let (client, source) = tokio::io::duplex(4096);
    let (client_read, mut client_write) = tokio::io::split(client);
    let (source_read, mut source_write) = tokio::io::split(source);
    let mut reader = BufReader::new(source_read);
    let mut pending = std::collections::VecDeque::new();
    let exchange = async {
        let id = json!(7);
        let args = json!({});
        let mut framer = Framer::default();
        let serve = await_post(
            stand_in(cancellation.clone()),
            PostRequest {
                id: &id,
                args: &args,
                cancellation: &cancellation,
            },
            &server,
            &mut reader,
            &mut framer,
            &mut pending,
            &mut source_write,
        );
        let client = async {
            client_write.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"tools/call\",\"params\":{\"name\":\"tally_status\"}}\n{\"jsonrpc\":\"2.0\",\"id\":9,\"method\":\"ping\"}\n").await.unwrap();
            let mut response = String::new();
            BufReader::new(client_read)
                .read_line(&mut response)
                .await
                .unwrap();
            assert_eq!(
                serde_json::from_str::<Value>(&response).unwrap(),
                json!({"jsonrpc":"2.0","id":9,"result":{}})
            );
            client_write.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{\"requestId\":7}}\n").await.unwrap();
        };
        let (result, _) = tokio::join!(serve, client);
        assert!(result.unwrap().is_none());
    };
    tokio::time::timeout(std::time::Duration::from_secs(2), exchange)
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(
        parse_request(pending.pop_front().unwrap().unwrap()).unwrap()["id"],
        8
    );
}

#[tokio::test]
async fn cancellation_before_intent_withdraws_the_post() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let (mut client, source) = tokio::io::duplex(1024);
    let mut reader = BufReader::new(source);
    let mut framer = Framer::default();
    let mut pending = std::collections::VecDeque::new();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let mut output = Vec::new();
    client.write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{\"requestId\":7}}\n").await.unwrap();
    // Withdrawn, the post is awaited until it stops, then answers as cancelled.
    let future = stand_in(cancellation.clone());
    assert!(stops(await_post(
        future,
        PostRequest {
            id: &json!(7),
            args: &json!({}),
            cancellation: &cancellation,
        },
        &server,
        &mut reader,
        &mut framer,
        &mut pending,
        &mut output,
    ))
    .await
    .unwrap()
    .is_none());
}

#[tokio::test]
async fn disconnect_before_intent_withdraws_the_post() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let mut reader = BufReader::new(&b""[..]);
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let result = stops(await_post(
        stand_in(cancellation.clone()),
        PostRequest {
            id: &json!(7),
            args: &json!({}),
            cancellation: &cancellation,
        },
        &server,
        &mut reader,
        &mut Framer::default(),
        &mut std::collections::VecDeque::new(),
        &mut Vec::new(),
    ))
    .await;
    assert_eq!(result.err().as_deref(), Some("stdio_client_disconnected"));
}

#[tokio::test]
async fn interrupted_partial_frame_is_preserved() {
    let (mut client, source) = tokio::io::duplex(1024);
    let mut reader = BufReader::new(source);
    let mut framer = Framer::default();
    client.write_all(b"{\"jsonrpc\":\"2.0\",").await.unwrap();
    tokio::select! {
        biased;
        _ = framer.read(&mut reader, 1024) => panic!("not a complete frame"),
        _ = tokio::task::yield_now() => {}
    }
    client
        .write_all(b"\"id\":8,\"method\":\"ping\"}\n")
        .await
        .unwrap();
    let request = parse_request(
        framer
            .read(&mut reader, 1024)
            .await
            .unwrap()
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(request["id"], 8);
}

#[tokio::test]
async fn queue_overflow_is_refused_in_band_and_waits_for_cancellation() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let input = format!(
        "{}{{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{{\"requestId\":7}}}}\n",
        "{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"tools/list\"}\n".repeat(9),
    );
    let mut reader = BufReader::new(input.as_bytes());
    let mut pending = std::collections::VecDeque::new();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let mut output = Vec::new();
    let result = stops(await_post(
        stand_in(cancellation.clone()),
        PostRequest {
            id: &json!(7),
            args: &json!({}),
            cancellation: &cancellation,
        },
        &server,
        &mut reader,
        &mut Framer::default(),
        &mut pending,
        &mut output,
    ))
    .await;
    assert!(result.unwrap().is_none());
    assert_eq!(pending.len(), 8);
    let refusal: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(refusal["id"], 8);
    assert_eq!(refusal["error"]["code"], -32000);
    assert_eq!(
        refusal["error"]["message"],
        "stdio_pending_requests_exceeded"
    );
}

#[tokio::test]
async fn queue_overflow_refuses_an_oversized_id_without_ending_the_post_wait() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let oversized = "é\"".repeat(100);
    let input = format!(
        "{}{{\"jsonrpc\":\"2.0\",\"id\":{},\"method\":\"ping\"}}\n{{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{{\"requestId\":7}}}}\n",
        "{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"tools/list\"}\n".repeat(8),
        serde_json::to_string(&oversized).unwrap(),
    );
    let mut reader = BufReader::new(input.as_bytes());
    let mut pending = std::collections::VecDeque::new();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server(directory.path());
    server.settings.max_bytes = 256;
    let mut output = Vec::new();
    let result = stops(await_post(
        stand_in(cancellation.clone()),
        PostRequest {
            id: &json!(7),
            args: &json!({}),
            cancellation: &cancellation,
        },
        &server,
        &mut reader,
        &mut Framer::default(),
        &mut pending,
        &mut output,
    ))
    .await;
    assert!(result.unwrap().is_none());
    assert_eq!(pending.len(), 8);
    assert!(output.len() <= 256);
    let refusal: Value = serde_json::from_slice(&output).unwrap();
    assert!(refusal["id"].is_null());
    assert_eq!(refusal["error"]["message"], "request_id_too_large");
}

#[tokio::test]
async fn queue_overflow_tool_request_has_a_prepared_and_completed_refusal_receipt() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let input = format!(
        "{}{{\"jsonrpc\":\"2.0\",\"id\":9,\"method\":\"tools/call\",\"params\":{{\"name\":\"voucher_schema\",\"arguments\":{{}}}}}}\n{{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{{\"requestId\":7}}}}\n",
        "{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"tools/list\"}\n".repeat(8),
    );
    let mut reader = BufReader::new(input.as_bytes());
    let mut pending = std::collections::VecDeque::new();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let mut output = Vec::new();
    assert!(stops(await_post(
        stand_in(cancellation.clone()),
        PostRequest {
            id: &json!(7),
            args: &json!({}),
            cancellation: &cancellation,
        },
        &server,
        &mut reader,
        &mut Framer::default(),
        &mut pending,
        &mut output,
    ))
    .await
    .unwrap()
    .is_none());
    let records = fs::read_to_string(directory.path().join("agent-egress.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(records[0]["record_type"], "response_prepared");
    assert_eq!(records[0]["tool"], "voucher_schema");
    assert_eq!(records[1]["record_type"], "stdio_write_completed");
}

struct AlwaysReadable {
    frame: Vec<u8>,
}

impl AsyncRead for AlwaysReadable {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        read: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        read.put_slice(&self.frame);
        Poll::Ready(Ok(()))
    }
}

impl AsyncBufRead for AlwaysReadable {
    fn poll_fill_buf(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<&[u8]>> {
        Poll::Ready(Ok(&self.get_mut().frame))
    }

    fn consume(self: Pin<&mut Self>, _: usize) {}
}

#[tokio::test]
async fn readable_queue_traffic_cannot_starve_the_pending_post() {
    let cancellation = tokio_util::sync::CancellationToken::new();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let mut reader = AlwaysReadable {
        frame: b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n".to_vec(),
    };
    let mut pending = std::collections::VecDeque::new();
    let mut output = Vec::new();
    let completed = tokio::time::timeout(
        std::time::Duration::from_millis(100),
        await_post(
            async {
                tokio::task::yield_now().await;
                ToolResponse {
                    value: json!({}),
                    egress: EgressContext {
                        evidence: None,
                        tool: "post_import".into(),
                        args_sha256: sha256_hex(b"post"),
                        company_guid: None,
                        request_trail: None,
                    },
                    recovery_batch_id: None,
                }
            },
            PostRequest {
                id: &json!(7),
                args: &json!({}),
                cancellation: &cancellation,
            },
            &server,
            &mut reader,
            &mut Framer::default(),
            &mut pending,
            &mut output,
        ),
    )
    .await;
    assert!(completed.unwrap().unwrap().is_some());
}

/// A cancellation before the intent withdraws the post (#725): its token is
/// cancelled, so it starts no further queued operation, and the batch's held
/// approval is revoked, so no intent can be written for it. The same holds
/// when the input ends instead.
#[tokio::test]
async fn a_withdrawn_post_revokes_its_approval_and_cancels_its_operations() {
    for input in [
        &b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{\"requestId\":7}}\n"[..],
        &b""[..],
    ] {
        let directory = tempfile::tempdir().unwrap();
        let server = server(directory.path());
        let (_, args) = saved_batch(&server);
        let batch_id = args["batch_id"].as_str().unwrap().to_string();
        let _held = server.post_approvals.redeeming_for_test(&batch_id);
        let cancellation = tokio_util::sync::CancellationToken::new();
        let _ = stops(await_post(
            stand_in(cancellation.clone()),
            PostRequest {
                id: &json!(7),
                args: &args,
                cancellation: &cancellation,
            },
            &server,
            &mut BufReader::new(input),
            &mut Framer::default(),
            &mut std::collections::VecDeque::new(),
            &mut Vec::new(),
        ))
        .await;
        assert!(cancellation.is_cancelled());
        assert!(!server.post_approvals.holds(&batch_id));
    }
}
