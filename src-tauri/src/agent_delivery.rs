//! Prepared response commitments and confirmed stdio write completion.
//! Completion confirms write_all and flush, never consumption by an MCP client.
use super::*;

#[derive(Serialize)]
struct EgressReceipt<'a> {
    record_type: &'static str,
    receipt_id: String,
    ts: String,
    tool: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name_sha256: Option<String>,
    args_sha256: String,
    company_guid: Option<String>,
    rows_prepared: usize,
    fields_prepared: Vec<String>,
    bytes_prepared: usize,
    enforced_bytes: usize,
    response_sha256: String,
    truncated: bool,
    redaction_preset: &'a str,
    /// The error the response carried, by whitelist (bridge#799).
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
    /// What each Tally send of the call was and how it ended (#918): the last
    /// 32 by kind, size, outcome and time, and a count of the rest.
    #[serde(skip_serializing_if = "Option::is_none")]
    request_trail: Option<Value>,
}

/// The most window parts a receipt keeps: the last ones sent, the rest
/// counted.
const RECEIPT_WINDOW_PARTS: usize = 64;

/// The error a prepared response carried, kept beside its hash so that a
/// failure no client read (it ended after the client's timeout) can still be
/// diagnosed without sending it again (bridge#799). Taken by whitelist, never
/// by copying: the error code and its cause, each only when it is a code, and
/// a voucher window's timings key by key (dates, counts, bytes, milliseconds
/// and the failed request's kind). Never the message or any other field, so
/// the receipt keeps no row value and no text from Tally. `None` without a
/// code.
fn receipt_error(structured: &Value) -> Option<Value> {
    // A tool's error sits under `result`; a response withheld by the byte cap
    // carries its error at the top level.
    let error = match &structured["result"]["error"] {
        Value::Null => &structured["error"],
        error => error,
    };
    let code = receipt_code(&error["code"])?;
    let mut kept = json!({ "code": code });
    if let Some(cause) = receipt_code(&error["cause"]) {
        kept["cause"] = cause;
    }
    if let Some(window) = receipt_window(&error["window"]) {
        kept["window"] = window;
    }
    Some(kept)
}

fn receipt_code(value: &Value) -> Option<Value> {
    value
        .as_str()
        .filter(|text| {
            (1..=64).contains(&text.len())
                && text
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })
        .map(|text| json!(text))
}

fn receipt_date(value: &Value) -> Option<Value> {
    value
        .as_str()
        .filter(|text| {
            (1..=10).contains(&text.len())
                && text
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || byte == b'-')
        })
        .map(|text| json!(text))
}

fn receipt_number(value: &Value) -> Option<Value> {
    value.is_u64().then(|| value.clone())
}

fn receipt_flag(value: &Value) -> Option<Value> {
    value.is_boolean().then(|| value.clone())
}

type ReceiptField = (&'static str, fn(&Value) -> Option<Value>);

/// Only `fields` of `value`, each only when its check passes.
fn receipt_object(value: &Value, fields: &[ReceiptField]) -> Option<Value> {
    let object = value.as_object()?;
    Some(Value::Object(
        fields
            .iter()
            .filter_map(|(name, keep)| {
                object
                    .get(*name)
                    .and_then(keep)
                    .map(|kept| ((*name).to_string(), kept))
            })
            .collect(),
    ))
}

fn receipt_tally(value: &Value) -> Option<Value> {
    receipt_object(
        value,
        &[("requests", receipt_number), ("ms", receipt_number)],
    )
}

fn receipt_failed(value: &Value) -> Option<Value> {
    receipt_object(value, &[("kind", receipt_code), ("ms", receipt_number)])
}

fn receipt_part(value: &Value) -> Option<Value> {
    receipt_object(
        value,
        &[
            ("from", receipt_date),
            ("to", receipt_date),
            ("after", receipt_number),
            ("through", receipt_number),
            ("served", receipt_flag),
            ("bytes", receipt_number),
            ("rows", receipt_number),
            ("ms", receipt_number),
        ],
    )
}

fn receipt_window(value: &Value) -> Option<Value> {
    let mut window = receipt_object(
        value,
        &[
            ("from", receipt_date),
            ("to", receipt_date),
            ("marks", receipt_tally),
            ("census", receipt_tally),
            ("failed", receipt_failed),
        ],
    )?;
    // The last parts are kept, since the failed one is the last sent; the
    // rest are counted. A response that already counted its parts instead of
    // listing them keeps that count.
    if let Some(parts) = value["parts"].as_array() {
        let omitted = parts.len().saturating_sub(RECEIPT_WINDOW_PARTS);
        window["parts"] = Value::Array(parts[omitted..].iter().filter_map(receipt_part).collect());
        if omitted > 0 {
            window["parts_omitted"] = json!(omitted);
        }
    } else if let Some(omitted) = receipt_number(&value["parts_omitted"]) {
        window["parts_omitted"] = omitted;
    }
    window
        .as_object()
        .is_some_and(|window| !window.is_empty())
        .then_some(window)
}

// Only a successfully persisted preparation can produce a completion token.
pub(super) struct PreparedReceipt {
    receipt_id: String,
    response_sha256: String,
    bytes: usize,
}

impl Server {
    pub(super) fn append_framed_egress(
        &self,
        context: EgressContext,
        response: &Value,
        serialized_response: &str,
    ) -> Result<PreparedReceipt, String> {
        let structured = response
            .get("result")
            .and_then(|result| result.get("structuredContent"));
        let rows_prepared = structured.and_then(response_row_count).unwrap_or_default();
        let truncated = structured
            .and_then(|value| value.get("truncated"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let fields_prepared = structured
            .map(agent_receipt_fields::released_fields)
            .unwrap_or_default();
        let path = self.settings.data_dir.join("agent-egress.jsonl");
        let (tool, tool_name_sha256) = receipt_tool_identity(&context.tool);
        let receipt = EgressReceipt {
            record_type: "response_prepared",
            receipt_id: uuid::Uuid::new_v4().to_string(),
            ts: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            tool,
            tool_name_sha256,
            args_sha256: context.args_sha256,
            company_guid: context
                .company_guid
                .as_deref()
                .and_then(egress::canonical_company_guid),
            rows_prepared,
            fields_prepared,
            bytes_prepared: serialized_response.len(),
            enforced_bytes: self.settings.max_bytes,
            response_sha256: sha256_hex(serialized_response.as_bytes()),
            truncated,
            redaction_preset: self.settings.redaction.label(),
            // A JSON-RPC error (a refused or withdrawn call, a batch's
            // recovery) has no structured content; its message is its code.
            error: structured.and_then(receipt_error).or_else(|| {
                receipt_code(&response["error"]["message"]).map(|code| json!({ "code": code }))
            }),
            request_trail: context.request_trail,
        };
        let line = serde_json::to_string(&receipt)
            .map_err(|_| "egress_record_write_failed".to_string())?;
        append_egress_line(&path, &line)?;
        Ok(PreparedReceipt {
            receipt_id: receipt.receipt_id,
            response_sha256: receipt.response_sha256,
            bytes: receipt.bytes_prepared,
        })
    }

    pub(super) fn append_notification_refusal_egress(
        &self,
        tool: &str,
        args: &Value,
    ) -> Result<(), String> {
        let refusal = "tools_call_notification_forbidden";
        let (tool, tool_name_sha256) = receipt_tool_identity(tool);
        let receipt = EgressReceipt {
            record_type: "notification_refused",
            receipt_id: uuid::Uuid::new_v4().to_string(),
            ts: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            tool,
            tool_name_sha256,
            args_sha256: sha256_json(args),
            company_guid: args
                .get("company_guid")
                .and_then(Value::as_str)
                .and_then(egress::canonical_company_guid),
            rows_prepared: 0,
            fields_prepared: Vec::new(),
            bytes_prepared: 0,
            enforced_bytes: self.settings.max_bytes,
            response_sha256: sha256_hex(refusal.as_bytes()),
            truncated: false,
            redaction_preset: self.settings.redaction.label(),
            error: None,
            request_trail: None,
        };
        let line = serde_json::to_string(&receipt)
            .map_err(|_| "egress_record_write_failed".to_string())?;
        append_egress_line(&self.settings.data_dir.join("agent-egress.jsonl"), &line)
    }

    pub(super) fn append_stdio_write_completed(
        &self,
        prepared: PreparedReceipt,
    ) -> Result<(), String> {
        let record = json!({
            "record_type": "stdio_write_completed",
            "receipt_id": prepared.receipt_id,
            "ts": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "response_sha256": prepared.response_sha256,
            "bytes_written": prepared.bytes,
        });
        append_egress_line(
            &self.settings.data_dir.join("agent-egress.jsonl"),
            &record.to_string(),
        )
    }
}

#[cfg(test)]
#[path = "agent_delivery_tests.rs"]
mod tests;
