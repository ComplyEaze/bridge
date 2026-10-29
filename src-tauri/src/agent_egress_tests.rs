use super::*;
use std::fs::OpenOptions;

#[test]
fn failed_partial_write_and_sync_restore_the_original_receipts() {
    for partial_write in [true, false] {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("agent-egress.jsonl");
        let original = b"{\"tool\":\"existing\"}\n";
        fs::write(&path, original).unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        file.lock().unwrap();
        let result = append_locked(&mut file, r#"{"tool":"failed"}"#, |file, bytes| {
            file.write_all(if partial_write { &bytes[..7] } else { bytes })?;
            Err(std::io::Error::other("injected append or sync failure"))
        });
        file.unlock().unwrap();
        assert_eq!(result, Err("egress_record_write_failed".into()));
        assert_eq!(fs::read(&path).unwrap(), original);
        append_egress_line(&path, r#"{"tool":"next"}"#).unwrap();
        let rows = read_egress_tail(&path, 2).unwrap();
        assert_eq!(
            rows.records,
            [r#"{"tool":"next"}"#, r#"{"tool":"existing"}"#]
        );
    }
}

#[test]
fn failed_rollback_is_distinct_and_incomplete_log_refuses_future_appends() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    fs::write(&path, b"{}\n").unwrap();
    // A read-only handle makes the real truncation fail on both supported OSes.
    let mut file = File::open(&path).unwrap();
    let result = append_locked(&mut file, "{}", |_, _| {
        OpenOptions::new()
            .append(true)
            .open(&path)?
            .write_all(b"{\"partial")?;
        Err(std::io::Error::other("injected write failure"))
    });
    assert_eq!(result, Err("egress_record_rollback_failed".into()));
    let damaged = fs::read(&path).unwrap();
    assert_eq!(
        append_egress_line(&path, "{}"),
        Err("egress_log_incomplete".into())
    );
    assert_eq!(fs::read(&path).unwrap(), damaged);
    assert_eq!(
        read_egress_tail(&path, 20).unwrap_err(),
        "egress_log_incomplete"
    );
}

#[test]
fn receipt_size_limit_preserves_the_file_and_admitted_tail_is_readable() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    append_egress_line(&path, "{}").unwrap();
    let oversized = "x".repeat(MAX_EGRESS_TAIL_BYTES - 1);
    assert_eq!(
        append_egress_line(&path, &oversized),
        Err("egress_record_too_large".into())
    );
    assert_eq!(fs::read(&path).unwrap(), b"{}\n");
    let admitted = format!("\"{}\"", "x".repeat(MAX_EGRESS_TAIL_BYTES - 4));
    append_egress_line(&path, &admitted).unwrap();
    assert_eq!(read_egress_tail(&path, 1).unwrap().records, [admitted]);
}

#[test]
fn company_receipts_only_admit_canonical_uuid_identifiers() {
    assert_eq!(
        canonical_company_guid("{12345678-ABCD-4ABC-8ABC-123456789012}"),
        Some("12345678-abcd-4abc-8abc-123456789012".into())
    );
    assert_eq!(canonical_company_guid("unverified selector"), None);
}

#[cfg(unix)]
#[test]
fn receipt_append_creates_and_repairs_private_permissions() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    append_egress_line(&path, "{}").unwrap();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    append_egress_line(&path, "{}").unwrap();
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
}

#[test]
fn tail_discards_a_partial_unicode_line_before_decoding() {
    let directory = tempfile::tempdir().expect("temporary egress directory");
    let path = directory.path().join("agent-egress.jsonl");
    let body = format!(
        "{{\"tool\":\"{}\"}}\n{{\"tool\":\"new\"}}\n",
        "古".repeat(EGRESS_TAIL_CHUNK_BYTES)
    );
    assert!(!body.is_char_boundary(body.len() - EGRESS_TAIL_CHUNK_BYTES));
    fs::write(&path, body).expect("Unicode receipt followed by recent receipt");
    assert_eq!(
        read_egress_tail(&path, 1)
            .expect("latest complete receipt")
            .records,
        [r#"{"tool":"new"}"#]
    );
}

#[test]
fn locked_append_preserves_existing_receipts_and_adds_a_complete_line() {
    let directory = tempfile::tempdir().expect("temporary egress directory");
    let path = directory.path().join("agent-egress.jsonl");
    fs::write(&path, b"{\"tool\":\"existing\"}\n").expect("existing receipt");
    append_egress_line(&path, r#"{"tool":"new"}"#).expect("locked append");
    assert_eq!(
        fs::read(&path).expect("receipt bytes"),
        b"{\"tool\":\"existing\"}\n{\"tool\":\"new\"}\n"
    );
    assert_eq!(
        read_egress_tail(&path, 2)
            .expect("locked tail read")
            .records,
        [r#"{"tool":"new"}"#, r#"{"tool":"existing"}"#]
    );
}

#[test]
fn egress_tail_distinguishes_complete_logs_from_record_limited_pages() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    let missing = read_egress_tail(&path, 20).unwrap();
    assert!(missing.records.is_empty());
    assert!(!missing.truncated);
    fs::write(&path, b"").unwrap();
    let empty = read_egress_tail(&path, 20).unwrap();
    assert!(empty.records.is_empty());
    assert!(!empty.truncated);
    append_egress_line(&path, r#"{"index":0}"#).unwrap();
    append_egress_line(&path, r#"{"index":1}"#).unwrap();
    for limit in [2, 20] {
        let complete = read_egress_tail(&path, limit).unwrap();
        assert_eq!(complete.records, [r#"{"index":1}"#, r#"{"index":0}"#]);
        assert!(!complete.truncated);
    }
    let bounded = read_egress_tail(&path, 1).unwrap();
    assert_eq!(bounded.records, [r#"{"index":1}"#]);
    assert!(bounded.truncated);
}

#[tokio::test]
async fn egress_log_reports_byte_bounded_tail_even_below_requested_record_count() {
    use super::super::{Redaction, Server, Settings, TallyEndpointConfig};
    use serde_json::{json, Value};
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 100,
        max_bytes: 1_000_000,
        redaction: Redaction::MaskParties,
        import_enabled: false,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    append_egress_line(&path, r#"{"index":0}"#).unwrap();
    let complete = server.call_tool("egress_log", json!({"limit":20})).await;
    assert_eq!(complete["isError"], false);
    assert_eq!(complete["structuredContent"]["truncated"], false);
    assert_eq!(
        complete["structuredContent"]["result"]["records"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    for index in 1..=6 {
        let row = json!({"index": index, "padding": "古".repeat(20_000)}).to_string();
        append_egress_line(&path, &row).unwrap();
    }
    assert!(fs::metadata(&path).unwrap().len() > MAX_EGRESS_TAIL_BYTES as u64);
    let tail = read_egress_tail(&path, 20).unwrap();
    assert!(tail.truncated);
    assert_eq!(
        tail.records.len(),
        4,
        "only four complete rows fit the scan bound"
    );
    for (offset, row) in tail.records.iter().enumerate() {
        let parsed: Value = serde_json::from_str(row).expect("complete UTF-8 JSON receipt");
        assert_eq!(parsed["index"], 6 - offset);
    }
    let bounded = server.call_tool("egress_log", json!({"limit":20})).await;
    assert_eq!(bounded["isError"], false);
    assert_eq!(bounded["structuredContent"]["truncated"], true);
    assert_eq!(
        bounded["structuredContent"]["result"]["records"],
        json!(tail.records)
    );
}

#[tokio::test]
async fn egress_log_rejects_unterminated_and_malformed_receipts_in_band() {
    use super::super::{Redaction, Server, Settings, TallyEndpointConfig};
    use serde_json::{json, Value};
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 100,
        max_bytes: 1_000_000,
        redaction: Redaction::MaskParties,
        import_enabled: false,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    server
        .append_notification_refusal_egress("voucher_schema", &json!({}))
        .unwrap();
    let valid = fs::read(&path).unwrap();
    let receipt: Value = serde_json::from_slice(&valid).unwrap();
    assert_eq!(receipt["tool"], "voucher_schema");
    assert!(receipt["response_sha256"].is_string());
    assert!(receipt["fields_prepared"].is_array());
    let complete = read_egress_tail(&path, 20).unwrap();
    assert_eq!(complete.records.len(), 1);
    assert!(!complete.truncated);
    let mut prefix_with_newline = valid[..valid.len() / 2].to_vec();
    prefix_with_newline.push(b'\n');
    for damaged in [
        valid[..valid.len() - 1].to_vec(),
        valid[..valid.len() / 2].to_vec(),
        prefix_with_newline,
        [b"{\"tool\":\n".as_slice(), valid.as_slice()].concat(),
    ] {
        fs::write(&path, &damaged).unwrap();
        assert_eq!(
            read_egress_tail(&path, 20).unwrap_err(),
            "egress_log_incomplete"
        );
        let response = server.call_tool("egress_log", json!({"limit":20})).await;
        assert_eq!(response["isError"], true);
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"],
            "egress_log_incomplete"
        );
        assert_eq!(
            response["structuredContent"]["evidence"]["state"],
            "partial"
        );
        assert!(response["structuredContent"]["result"]["records"].is_null());
        assert_eq!(
            fs::read(&path).unwrap(),
            damaged,
            "reads never repair damaged receipts"
        );
    }
}

#[test]
fn receipt_append_waits_until_all_shared_readers_release_the_file() {
    use std::sync::mpsc;
    use std::time::Duration;
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    append_egress_line(&path, r#"{"tool":"first"}"#).unwrap();
    let reader = File::open(&path).unwrap();
    let other_reader = File::open(&path).unwrap();
    reader.lock_shared().unwrap();
    other_reader.try_lock_shared().unwrap();
    let contender = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .unwrap();
    assert!(matches!(
        contender.try_lock(),
        Err(std::fs::TryLockError::WouldBlock)
    ));
    let (started, ready) = mpsc::channel();
    let writer_path = path.clone();
    let writer = std::thread::spawn(move || {
        started.send(()).unwrap();
        append_egress_line(&writer_path, r#"{"tool":"second"}"#)
    });
    ready.recv_timeout(Duration::from_secs(2)).unwrap();
    reader.unlock().unwrap();
    std::thread::sleep(Duration::from_millis(1000));
    assert!(
        !writer.is_finished(),
        "remaining reader must exclude the append"
    );
    other_reader.unlock().unwrap();
    writer.join().unwrap().unwrap();
    contender.try_lock().unwrap();
    contender.unlock().unwrap();
    assert_eq!(
        read_egress_tail(&path, 2).unwrap().records,
        [r#"{"tool":"second"}"#, r#"{"tool":"first"}"#]
    );
}

/// Receipts carrying a full trail (about 11 KiB each) are read back by `egress_log`
/// within its byte bound: complete lines, flagged truncated when the bound cut the
/// tail, never an error.
#[tokio::test]
async fn egress_log_reads_back_receipts_that_carry_a_full_trail() {
    use super::super::{Redaction, Server, Settings, TallyEndpointConfig};
    use crate::request_trail::{RequestTrail, TRAIL_LAST};
    use bridge_tally_transport::{SendKind, SendRecord};
    use serde_json::{json, Value};
    let directory = tempfile::tempdir().unwrap();
    let trail = RequestTrail::default();
    for index in 0..(TRAIL_LAST + 5) {
        trail.push(SendRecord {
            kind: SendKind::Post,
            request_bytes: Some(700),
            outcome: "answered",
            response_bytes: Some(1_234),
            held_ms: index as u64,
        });
    }
    let snapshot = trail.snapshot().unwrap();
    let path = directory.path().join("agent-egress.jsonl");
    for index in 0..40 {
        let line =
            json!({"record_type": "response_prepared", "index": index, "request_trail": snapshot})
                .to_string();
        append_egress_line(&path, &line).unwrap();
    }
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 100,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    // The default read (20 receipts) of receipts that each carry a full trail
    // fits the byte cap, and every line is complete.
    let response = server.call_tool("egress_log", json!({})).await;
    assert_eq!(response["isError"], false, "{response}");
    let records = response["structuredContent"]["result"]["records"]
        .as_array()
        .unwrap();
    assert_eq!(records.len(), 20);
    for line in records {
        let parsed: Value = serde_json::from_str(line.as_str().unwrap()).unwrap();
        assert_eq!(parsed["request_trail"]["sent"], TRAIL_LAST + 5);
    }
    // A much larger `limit` over such receipts exceeds the byte cap: the answer keeps the newest
    // whole receipts that fit the cap as the response is really encoded (both copies and the
    // envelope), says so with `truncated` and counts the rest in `omitted_for_size`; it is no
    // longer refused (#2125).
    let many = server.call_tool("egress_log", json!({"limit": 100})).await;
    assert_eq!(many["isError"], false, "{many}");
    assert_eq!(many["structuredContent"]["truncated"], true, "{many}");
    let kept = many["structuredContent"]["result"]["records"]
        .as_array()
        .unwrap();
    let omitted = many["structuredContent"]["result"]["omitted_for_size"]
        .as_u64()
        .unwrap();
    assert!(!kept.is_empty() && kept.len() < 40, "{}", kept.len());
    // The file holds 40 receipts and the scan reaches all of them: what is kept and what is
    // left out account for every receipt.
    assert_eq!(kept.len() as u64 + omitted, 40);
    // Whole lines, newest first.
    for (position, line) in kept.iter().enumerate() {
        let parsed: Value = serde_json::from_str(line.as_str().unwrap()).unwrap();
        assert_eq!(parsed["index"], 39 - position);
    }
    // And the answer is within the cap as it is sent.
    assert!(many.to_string().len() <= 200_000);
}

/// The fixture of the next two tests: `count` receipts of the same length, so that a cap can be
/// set to the byte.
fn equal_receipts(directory: &tempfile::TempDir, count: usize, padding: usize) {
    let path = directory.path().join("agent-egress.jsonl");
    for index in 0..count {
        let line = serde_json::json!({"record_type": "response_prepared", "index": format!("{index:04}"), "padding": "x".repeat(padding)}).to_string();
        append_egress_line(&path, &line).unwrap();
    }
}

fn server_with_cap(directory: &tempfile::TempDir, max_bytes: usize) -> super::super::Server {
    use super::super::{Redaction, Server, Settings, TallyEndpointConfig};
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 256,
        max_bytes,
        redaction: Redaction::None,
        import_enabled: false,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    })
}

/// The boundary, to the byte: a cap equal to the size of the answer for twelve receipts returns
/// all twelve and nothing is marked left out; one byte less returns eleven, marked, and the
/// eleven are within that cap as sent.
#[tokio::test]
async fn egress_log_keeps_the_newest_receipts_that_fit_at_the_exact_boundary() {
    use serde_json::json;
    let directory = tempfile::tempdir().unwrap();
    equal_receipts(&directory, 12, 1_000);
    let whole = server_with_cap(&directory, 1_000_000)
        .call_tool("egress_log", json!({"limit": 12}))
        .await;
    assert_eq!(whole["structuredContent"]["truncated"], false, "{whole}");
    let size = whole.to_string().len();
    let at = server_with_cap(&directory, size)
        .call_tool("egress_log", json!({"limit": 12}))
        .await;
    assert_eq!(at["isError"], false, "{at}");
    assert_eq!(at["structuredContent"]["truncated"], false, "{at}");
    assert!(at["structuredContent"]["result"]
        .get("omitted_for_size")
        .is_none());
    assert_eq!(
        at["structuredContent"]["result"]["records"]
            .as_array()
            .unwrap()
            .len(),
        12
    );
    let under = server_with_cap(&directory, size - 1)
        .call_tool("egress_log", json!({"limit": 12}))
        .await;
    assert_eq!(under["isError"], false, "{under}");
    assert_eq!(under["structuredContent"]["truncated"], true, "{under}");
    let result = &under["structuredContent"]["result"];
    assert_eq!(result["records"].as_array().unwrap().len(), 11);
    assert_eq!(result["omitted_for_size"], 1);
    assert!(under.to_string().len() < size);
    // The newest eleven: the oldest receipt is the one left out.
    let first = result["records"][0].as_str().unwrap();
    assert!(first.contains("\"0011\""), "{first}");
}

/// One receipt that alone is over the cap cannot be cut mid-line, so the call is refused with
/// the existing code, as before (#2125); so is any cap too small for the first newest receipt.
#[tokio::test]
async fn egress_log_still_refuses_when_one_receipt_alone_is_over_the_cap() {
    use serde_json::json;
    let directory = tempfile::tempdir().unwrap();
    equal_receipts(&directory, 3, 60_000);
    let refused = server_with_cap(&directory, 100_000)
        .call_tool("egress_log", json!({"limit": 3}))
        .await;
    assert_eq!(refused["isError"], true, "{refused}");
    assert_eq!(
        refused["structuredContent"]["error"]["code"], "agent_response_too_large",
        "{refused}"
    );
    // The same receipts fit a cap that holds one of them: the newest only, two counted left out.
    let one = server_with_cap(&directory, 300_000)
        .call_tool("egress_log", json!({"limit": 3}))
        .await;
    assert_eq!(one["isError"], false, "{one}");
    let result = &one["structuredContent"]["result"];
    let kept = result["records"].as_array().unwrap().len();
    assert_eq!(
        kept as u64 + result["omitted_for_size"].as_u64().unwrap(),
        3
    );
}

/// The response digest of `egress_log` hashes the receipt lines the call read, not the ones it
/// returned, so on an answer trimmed for size it covers more lines than the answer holds
/// (`omitted_for_size` counts the difference): the digest is of what was read from the log and
/// the README says so (#2125).
#[tokio::test]
async fn egress_log_digest_covers_every_line_read_even_when_the_answer_is_trimmed() {
    use serde_json::json;
    let directory = tempfile::tempdir().unwrap();
    equal_receipts(&directory, 12, 1_000);
    let path = directory.path().join("agent-egress.jsonl");
    let read = read_egress_tail(&path, 12).unwrap();
    assert_eq!(read.records.len(), 12);
    let size = server_with_cap(&directory, 1_000_000)
        .call_tool("egress_log", json!({"limit": 12}))
        .await
        .to_string()
        .len();
    let trimmed = server_with_cap(&directory, size - 1)
        .call_tool("egress_log", json!({"limit": 12}))
        .await;
    let result = &trimmed["structuredContent"]["result"];
    let returned = result["records"].as_array().unwrap().len();
    assert_eq!(returned, 11);
    assert_eq!(result["omitted_for_size"], 1);
    let digest = trimmed["structuredContent"]["evidence"]["response_sha256"]
        .as_str()
        .unwrap();
    // The digest is of all twelve lines read, as the call computes it.
    assert_eq!(digest, super::super::sha256_json(&read.records));
    // And not of the eleven returned.
    let returned_lines = read.records[..returned].to_vec();
    assert_ne!(digest, super::super::sha256_json(&returned_lines));
}
