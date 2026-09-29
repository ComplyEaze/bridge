//! Observe real preparation records while injecting stdout failures.
use super::*;
use crate::agent::agent_protocol::{finish_response, serve_stdio};
use std::{
    io,
    path::{Path, PathBuf},
    pin::Pin,
    task::{Context, Poll},
};

fn server(path: &Path) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: path.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::MaskParties,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

fn records(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

struct Writer {
    bytes: Vec<u8>,
    path: PathBuf,
    limit: Option<usize>,
    flush_fails: bool,
    completion_fails: bool,
}

impl Writer {
    fn new(path: PathBuf) -> Self {
        Self {
            bytes: Vec::new(),
            path,
            limit: None,
            flush_fails: false,
            completion_fails: false,
        }
    }
}

impl AsyncWrite for Writer {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        if self.path.is_file() {
            assert_eq!(
                records(&self.path).last().unwrap()["record_type"],
                "response_prepared",
                "preparation must already be durable before writing output"
            );
        }
        let remaining = self
            .limit
            .map_or(bytes.len(), |limit| limit.saturating_sub(self.bytes.len()));
        if remaining == 0 {
            return Poll::Ready(Err(io::Error::other("injected write failure")));
        }
        let written = remaining.min(bytes.len());
        self.bytes.extend_from_slice(&bytes[..written]);
        Poll::Ready(Ok(written))
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        if self.flush_fails {
            return Poll::Ready(Err(io::Error::other("injected flush failure")));
        }
        if self.completion_fails && self.path.is_file() {
            fs::rename(&self.path, self.path.with_extension("prepared")).unwrap();
            fs::create_dir(&self.path).unwrap();
        }
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[tokio::test]
async fn delivery_failures_leave_preparation_without_claiming_a_completed_write() {
    for (limit, flush_fails, error) in [
        (Some(0), false, "stdio_write_failed"),
        (Some(7), false, "stdio_write_failed"),
        (None, true, "stdio_flush_failed"),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let server = server(directory.path());
        let path = directory.path().join("agent-egress.jsonl");
        let mut writer = Writer::new(path.clone());
        writer.limit = limit;
        writer.flush_fails = flush_fails;
        let tool = server.call_tool_response("voucher_schema", json!({})).await;
        assert_eq!(
            finish_response(
                &server,
                &mut writer,
                json!(1),
                Ok(tool.value),
                Some(tool.egress),
                None,
                true
            )
            .await,
            Err(error.to_string())
        );
        {
            let history = server.evidence.lock().unwrap();
            assert_eq!(history.records.len(), 1);
            // The prepared read result is complete; delivery failure is a
            // separate fact established by the missing completion receipt.
            assert_eq!(history.records[0].state, "complete");
            assert_eq!(history.records[0].reason_code, None);
        }
        let events = records(&path);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0]["record_type"], "response_prepared");
        assert!(events[0]["bytes_written"].is_null());
        assert!(events[0]["bytes_returned"].is_null());
        uuid::Uuid::parse_str(events[0]["receipt_id"].as_str().unwrap()).unwrap();
        if let Some(limit) = limit {
            assert_eq!(writer.bytes.len(), limit);
        } else {
            assert_eq!(
                writer.bytes.len(),
                events[0]["bytes_prepared"].as_u64().unwrap() as usize
            );
        }
    }
}

#[tokio::test]
async fn delivery_success_links_completion_to_the_exact_prepared_response() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let path = directory.path().join("agent-egress.jsonl");
    let mut writer = Writer::new(path.clone());
    let tool = server.call_tool_response("voucher_schema", json!({})).await;
    finish_response(
        &server,
        &mut writer,
        json!(1),
        Ok(tool.value),
        Some(tool.egress),
        None,
        true,
    )
    .await
    .unwrap();
    let events = records(&path);
    assert_eq!(events.len(), 2);
    assert_eq!(events[0]["record_type"], "response_prepared");
    assert_eq!(events[1]["record_type"], "stdio_write_completed");
    assert_eq!(events[1]["receipt_id"], events[0]["receipt_id"]);
    for event in &events {
        assert_eq!(event["response_sha256"], sha256_hex(&writer.bytes));
    }
    assert_eq!(events[0]["bytes_prepared"], writer.bytes.len());
    assert_eq!(events[1]["bytes_written"], writer.bytes.len());
    assert!(events[1]["fields_prepared"].is_null());
}

#[tokio::test]
async fn delivery_completion_failure_stops_before_another_tool_dispatch() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let evidence = server.evidence.clone();
    let path = directory.path().join("agent-egress.jsonl");
    let mut writer = Writer::new(path.clone());
    writer.completion_fails = true;
    let input = [
        json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"voucher_schema"}}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"voucher_schema"}}),
    ].iter().map(|value| format!("{value}\n")).collect::<String>();
    assert_eq!(
        serve_stdio(
            server,
            tokio::io::BufReader::new(input.as_bytes()),
            &mut writer
        )
        .await,
        Err("egress_record_write_failed".into())
    );
    assert_eq!(
        evidence.lock().unwrap().records.len(),
        1,
        "second tool was never dispatched"
    );
    assert_eq!(records(&path.with_extension("prepared")).len(), 1);
    let replies = String::from_utf8(writer.bytes)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(replies.len(), 2);
    assert_eq!(replies[1]["id"], 1);
}

#[tokio::test]
async fn delivery_preparation_failure_releases_no_tool_response() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let path = directory.path().join("agent-egress.jsonl");
    fs::create_dir(&path).unwrap();
    let mut writer = Writer::new(path);
    let tool = server.call_tool_response("voucher_schema", json!({})).await;
    assert_eq!(
        finish_response(
            &server,
            &mut writer,
            json!(1),
            Ok(tool.value),
            Some(tool.egress),
            None,
            true
        )
        .await,
        Err("egress_record_write_failed".into())
    );
    assert!(writer.bytes.is_empty());
}

/// The receipt a response holding `structured` gets when it is prepared after
/// the client has gone, so nothing of it can be written. No journal line may
/// hold the text "Private", which every case puts only where it must be dropped.
async fn late_receipt(structured: Value) -> Value {
    late_receipt_for(Ok(json!({
        "content": [{"type": "text", "text": "refused"}],
        "isError": true,
        "structuredContent": structured,
    })))
    .await
}

/// [`late_receipt`] for a whole call result, a JSON-RPC error included.
async fn late_receipt_for(result: Result<Value, String>) -> Value {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let path = directory.path().join("agent-egress.jsonl");
    let mut writer = Writer::new(path.clone());
    writer.limit = Some(0);
    let egress = EgressContext {
        evidence: None,
        tool: "vouchers".into(),
        args_sha256: sha256_hex(b"vouchers"),
        company_guid: None,
    };
    assert_eq!(
        finish_response(
            &server,
            &mut writer,
            json!(1),
            result,
            Some(egress),
            None,
            true
        )
        .await,
        Err("stdio_write_failed".to_string())
    );
    let line = fs::read_to_string(&path).unwrap();
    assert!(!line.contains("Private"), "{line}");
    let events = records(&path);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["record_type"], "response_prepared");
    events[0].clone()
}

/// bridge#799: a response prepared after the client has gone keeps its error
/// in the receipt, so the failure can be read back without sending the read
/// again. Only the whitelisted fields are kept: the code, a code-shaped cause,
/// and the window's dates, counts, bytes, milliseconds and failed kind. Every
/// other field, and any value that is not of its field's shape, is dropped.
#[tokio::test]
async fn a_late_response_keeps_only_its_whitelisted_error_in_the_receipt() {
    let error = json!({
        "code": "vouchers_window_part_failed",
        "cause": "request_deadline",
        "message": "Private message",
        "ledger": "Private Ledger",
        "window": {
            "from": "2026-09-27", "to": "2026-09-27",
            "marks": {"requests": 2, "ms": 40, "company": "Private Company"},
            "census": {"requests": 4, "ms": "Private"},
            "parts": [
                {"from": "2026-09-27", "to": "2026-09-27", "after": 10, "through": 90,
                 "served": true, "bytes": 2048, "rows": 12, "ms": 300,
                 "narration": "Private narration"},
                {"from": "Private", "to": "2026-09-277", "served": "Private", "ms": 78000}
            ],
            "failed": {"kind": "part", "ms": 78000},
            "voucher_number": "Private-7"
        }
    });
    let kept = json!({
        "code": "vouchers_window_part_failed",
        "cause": "request_deadline",
        "window": {
            "from": "2026-09-27", "to": "2026-09-27",
            "marks": {"requests": 2, "ms": 40},
            "census": {"requests": 4},
            "failed": {"kind": "part", "ms": 78000},
            "parts": [
                {"from": "2026-09-27", "to": "2026-09-27", "after": 10, "through": 90,
                 "served": true, "bytes": 2048, "rows": 12, "ms": 300},
                {"ms": 78000}
            ]
        }
    });
    let receipt = late_receipt(json!({"result": {"error": error.clone()}})).await;
    assert_eq!(receipt["error"], kept);

    // A cause or failed kind that is not a code is dropped on its own.
    let mut not_codes = error.clone();
    not_codes["cause"] = json!("PRIVATE_CAUSE");
    not_codes["window"]["failed"]["kind"] = json!("Private Kind");
    let mut expected = kept.clone();
    expected.as_object_mut().unwrap().remove("cause");
    expected["window"]["failed"] = json!({"ms": 78000});
    let receipt = late_receipt(json!({"result": {"error": not_codes}})).await;
    assert_eq!(receipt["error"], expected);

    // Of 65 parts, the last 64 are kept (the failed part is the last sent)
    // and one is counted; 64 are all kept.
    for (count, omitted) in [(65, Some(1)), (64, None)] {
        let mut many = error.clone();
        many["window"]["parts"] = json!((0..count)
            .map(|index| json!({"ms": index}))
            .collect::<Vec<_>>());
        let receipt = late_receipt(json!({"result": {"error": many}})).await;
        let window = &receipt["error"]["window"];
        let parts = window["parts"].as_array().unwrap();
        assert_eq!(parts.len(), 64, "{count}");
        assert_eq!(parts[63], json!({"ms": count - 1}), "{count}");
        assert_eq!(window["parts_omitted"].as_u64(), omitted, "{count}");
    }

    // A response that counted its parts instead of listing them keeps that
    // count; a window with nothing to keep is left out.
    let mut counted = error.clone();
    counted["window"].as_object_mut().unwrap().remove("parts");
    counted["window"]["parts_omitted"] = json!(400);
    let receipt = late_receipt(json!({"result": {"error": counted}})).await;
    assert_eq!(receipt["error"]["window"]["parts_omitted"], 400);
    assert!(receipt["error"]["window"].get("parts").is_none());
    // A count that is not a number is dropped, and a stale count beside a
    // listed set of parts is not carried over.
    let mut counted = error.clone();
    counted["window"].as_object_mut().unwrap().remove("parts");
    counted["window"]["parts_omitted"] = json!("Private");
    let receipt = late_receipt(json!({"result": {"error": counted}})).await;
    assert!(receipt["error"]["window"].get("parts_omitted").is_none());
    let mut stale = error.clone();
    stale["window"]["parts_omitted"] = json!(400);
    let receipt = late_receipt(json!({"result": {"error": stale}})).await;
    assert_eq!(receipt["error"], kept);
    let mut empty = error.clone();
    empty["window"] = json!({"ledger": "Private Ledger"});
    let receipt = late_receipt(json!({"result": {"error": empty}})).await;
    assert!(receipt["error"].get("window").is_none(), "{receipt}");

    // A response withheld by the byte cap carries its error at the top level.
    let receipt = late_receipt(json!({"error": {"code": "agent_response_too_large",
        "message": "Private message"}}))
    .await;
    assert_eq!(
        receipt["error"],
        json!({"code": "agent_response_too_large"})
    );

    // A JSON-RPC error keeps its message, which is its code, and only a code.
    let receipt = late_receipt_for(Err("request_cancelled".to_string())).await;
    assert_eq!(receipt["error"], json!({"code": "request_cancelled"}));
    let receipt = late_receipt_for(Err("Private text".to_string())).await;
    assert!(receipt.get("error").is_none(), "{receipt}");

    // An error whose code is not a code is not kept at all.
    for code in ["Private Code", "PRIVATE_CODE", &"private_".repeat(9)] {
        let mut not_a_code = error.clone();
        not_a_code["code"] = json!(code);
        let receipt = late_receipt(json!({"result": {"error": not_a_code}})).await;
        assert!(receipt.get("error").is_none(), "{code}: {receipt}");
    }
}
