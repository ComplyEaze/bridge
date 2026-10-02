//! Responses for the local MCP adapter.
use super::*;

#[derive(Clone, Copy)]
enum PageShape {
    Changes,
    Outstandings,
    Rows(&'static str),
}

const CHANGE_AXES: [(&str, &str, &str); 2] = [
    ("vouchers", "next_voucher_alter_id", "voucher_alter_id"),
    ("masters", "next_master_alter_id", "master_alter_id"),
];

/// A base-currency-ledgers-only outstandings result's lists of ledgers left
/// out: those kept in another currency (bridge#551) and base-currency ones
/// with a composite value (bridge#642).
const OUTSTANDINGS_EXCLUDED_AXES: [&str; 2] = [
    "foreign_currency_ledgers_excluded",
    "base_currency_ledgers_mixed_excluded",
];

/// The rows an outstandings result pages by one shared `offset`: open bills
/// and unallocated parties, at the top of the result or, for a
/// base-currency-ledgers-only result, under `base_currency_ledgers`; and that
/// result's excluded ledgers. `None` for any other result.
fn outstandings_axis_widths(result: &Value) -> Option<[usize; 4]> {
    let figures = if result["base_currency_ledgers"].is_object() {
        &result["base_currency_ledgers"]
    } else {
        result
    };
    let [foreign, mixed] = OUTSTANDINGS_EXCLUDED_AXES.map(|key| &result[key]["ledgers"]);
    (figures["open_bills"].is_array()
        || figures["unallocated"]["parties"].is_array()
        || foreign.is_array()
        || mixed.is_array())
    .then(|| {
        [
            &figures["open_bills"],
            &figures["unallocated"]["parties"],
            foreign,
            mixed,
        ]
        .map(|rows| rows.as_array().map_or(0, Vec::len))
    })
}

fn page_shape(response: &Value) -> Option<(PageShape, usize)> {
    let result = &response["result"];
    let change_width = CHANGE_AXES
        .iter()
        .filter_map(|(key, cursor, fallback)| {
            (result[cursor].is_u64() && result[fallback].is_u64())
                .then(|| result[key].as_array().map(Vec::len))
                .flatten()
        })
        .max()
        .unwrap_or(0);
    if change_width > 0 {
        return Some((PageShape::Changes, change_width));
    }
    if let Some(widths) = outstandings_axis_widths(result) {
        return Some((
            PageShape::Outstandings,
            widths.into_iter().max().unwrap_or(0),
        ));
    }
    ["items", "ledgers", "masters"].into_iter().find_map(|key| {
        // `masters` also keys the unpaged results of validate_masters and
        // build_import_xml, which carry no `offset`: only a paged result can be
        // trimmed with a resumable cursor.
        if key == "masters" && !result["offset"].is_u64() {
            return None;
        }
        result[key]
            .as_array()
            .map(|rows| (PageShape::Rows(key), rows.len()))
    })
}

fn retain_page_width(response: &mut Value, shape: PageShape, width: usize) -> Result<(), String> {
    if width == 0 {
        return Err("agent_response_too_large".into());
    }
    let result = &mut response["result"];
    let offset = result["offset"]
        .as_u64()
        .or_else(|| result["base_currency_ledgers"]["offset"].as_u64())
        .unwrap_or(0);
    match shape {
        PageShape::Changes => {
            for (key, cursor, fallback) in CHANGE_AXES {
                if !result[cursor].is_u64() || !result[fallback].is_u64() {
                    continue;
                }
                if let Some(rows) = result[key].as_array_mut().filter(|rows| rows.len() > width) {
                    rows.truncate(width);
                    let next = rows
                        .iter()
                        .filter_map(|row| row["alter_id"].as_u64())
                        .max()
                        .ok_or_else(|| "change_page_cursor_invalid".to_string())?;
                    result[cursor] = json!(next);
                    result["checkpoint_advanceable"] = json!(false);
                }
            }
        }
        PageShape::Outstandings => {
            // Every collection consumes one input offset. Keep their shared
            // prefix width; exhausted shorter axes retain their null cursor.
            // `get_mut`, not indexing: indexing a `Value` mutably inserts a
            // missing key, which would add an empty excluded list to a
            // complete result.
            for key in OUTSTANDINGS_EXCLUDED_AXES {
                let Some(list) = result.get_mut(key) else {
                    continue;
                };
                if let Some(rows) = list["ledgers"]
                    .as_array_mut()
                    .filter(|rows| rows.len() > width)
                {
                    rows.truncate(width);
                    list["next_offset"] = json!(offset + width as u64);
                    list["truncated"] = json!(true);
                }
            }
            let figures = if result["base_currency_ledgers"].is_object() {
                &mut result["base_currency_ledgers"]
            } else {
                result
            };
            if let Some(rows) = figures["open_bills"]
                .as_array_mut()
                .filter(|rows| rows.len() > width)
            {
                rows.truncate(width);
                figures["next_offset"] = json!(offset + width as u64);
            }
            if let Some(rows) = figures["unallocated"]["parties"]
                .as_array_mut()
                .filter(|rows| rows.len() > width)
            {
                rows.truncate(width);
                figures["unallocated"]["next_offset"] = json!(offset + width as u64);
                figures["unallocated"]["truncated"] = json!(true);
            }
        }
        PageShape::Rows(key) => {
            result[key]
                .as_array_mut()
                .expect("observed page rows")
                .truncate(width);
            result["next_offset"] = json!(offset + width as u64);
        }
    }
    response["truncated"] = json!(true);
    // A headline that lists rows says only the rows that are left. Only a page
    // of rows (`Rows`) trims what such a headline describes: the other shapes
    // trim other lists (a partial trial balance's excluded ledgers), and its
    // ledgers are then as many as before.
    if matches!(shape, PageShape::Rows(_)) {
        headline::restate_rows(response, width);
    }
    Ok(())
}

/// Removes every `tally_line_errors` list below `value`, adding its length
/// to the sibling `tally_line_errors_omitted`, and says whether any was
/// there. Tally's LINEERROR text is for reading only, so a result over its
/// cap loses it before anything else, and it is never why a result is paged
/// or refused. A value without the key is left untouched.
pub(super) fn drop_tally_line_error_text(value: &mut Value) -> bool {
    match value {
        Value::Object(fields) => {
            let mut dropped = false;
            if let Some(removed) = fields.remove("tally_line_errors") {
                let omitted = fields
                    .get("tally_line_errors_omitted")
                    .and_then(Value::as_u64)
                    .unwrap_or(0)
                    .saturating_add(removed.as_array().map_or(0, Vec::len) as u64);
                fields.insert("tally_line_errors_omitted".into(), json!(omitted));
                dropped = true;
            }
            for field in fields.values_mut() {
                dropped |= drop_tally_line_error_text(field);
            }
            dropped
        }
        Value::Array(items) => {
            // Every element, never stopping at the first with text.
            let mut dropped = false;
            for item in items {
                dropped |= drop_tally_line_error_text(item);
            }
            dropped
        }
        _ => false,
    }
}

// Measure O(log n) prefix candidates instead of serializing once for each
// discarded row. Each candidate retains at least one row on every active axis.
// The caller measures the actual outer envelope, including duplicated text and
// the wire newline, so escaping and final framing remain part of the byte cap.
fn fit_response(
    response: &mut Value,
    structured_path: &str,
    max_bytes: usize,
    mut encoded_len: impl FnMut(&mut Value) -> usize,
) -> Result<bool, String> {
    if encoded_len(response) <= max_bytes {
        return Ok(false);
    }
    // Tally's LINEERROR text goes first, and only when present: a result it
    // alone pushed over the cap comes back whole, without the text.
    let mut without_text = response.clone();
    if drop_tally_line_error_text(&mut without_text) {
        *response = without_text;
        if encoded_len(response) <= max_bytes {
            return Ok(false);
        }
    }
    let (shape, width) = response
        .pointer(structured_path)
        .and_then(page_shape)
        .ok_or_else(|| "agent_response_too_large".to_string())?;
    let (mut lower, mut upper) = (1, width.saturating_sub(1));
    let original = response.clone();
    let mut best = None;
    while lower <= upper {
        let keep = lower + (upper - lower) / 2;
        let mut candidate = original.clone();
        retain_page_width(
            candidate
                .pointer_mut(structured_path)
                .expect("observed payload"),
            shape,
            keep,
        )?;
        if encoded_len(&mut candidate) <= max_bytes {
            best = Some(candidate);
            lower = keep + 1;
        } else {
            upper = keep - 1;
        }
    }
    *response = best.ok_or_else(|| "agent_response_too_large".to_string())?;
    Ok(true)
}

pub(super) fn enforce_response_byte_cap(
    mut response: Value,
    max_bytes: usize,
) -> Result<(Value, bool, usize), String> {
    let truncated = fit_response(&mut response, "", max_bytes, |value| {
        value.to_string().len()
    })?;
    let rows = response_row_count(&response).unwrap_or_default();
    Ok((response, truncated, rows))
}

#[cfg(test)]
pub(super) fn truncate_response_items(response: &mut Value) -> Result<bool, String> {
    let Some((shape, width)) = page_shape(response).filter(|(_, width)| *width > 0) else {
        return Ok(false);
    };
    retain_page_width(response, shape, width - 1)?;
    Ok(true)
}

pub(super) fn response_row_count(response: &Value) -> Option<usize> {
    let result = &response["result"];
    if let Some(vouchers) = result["vouchers"].as_array() {
        return Some(vouchers.len() + result["masters"].as_array().map_or(0, Vec::len));
    }
    // Receipts count each released outstandings row collection: open bills,
    // unallocated parties and both lists of excluded ledgers. Top parties
    // are a derived ranking summary, not a separately paged row collection,
    // so they are intentionally excluded.
    if let Some(widths) = outstandings_axis_widths(result) {
        return Some(widths.into_iter().sum());
    }
    // Receipt counting does not imply pagination support. Only page_shape
    // determines which arrays can be trimmed with a resumable cursor.
    [
        "items",
        "ledgers",
        "records",
        "companies",
        "masters",
        "loaded_companies",
    ]
    .into_iter()
    .find_map(|key| result[key].as_array().map(Vec::len))
}

pub(super) fn set_mcp_content_json(mcp_response: &mut Value) {
    // MCP 2025-06-18 Tools: preserve the complete payload for clients that only
    // consume TextContent, including the supported 2024-11-05 protocol.
    mcp_response["content"] =
        json!([{"type":"text","text":mcp_response["structuredContent"].to_string()}]);
}

pub(super) fn enforce_mcp_result_byte_cap(
    mcp_response: &mut Value,
    max_bytes: usize,
    _name: &str,
    _fallback_rows: usize,
) -> Result<(), String> {
    fit_response(mcp_response, "/structuredContent", max_bytes, |value| {
        set_mcp_content_json(value);
        value.to_string().len()
    })
    .map(|_| ())
}

pub(super) fn enforce_jsonrpc_response_byte_cap(
    response: &mut Value,
    max_bytes: usize,
) -> Result<(), String> {
    fit_response(response, "/result/structuredContent", max_bytes, |value| {
        if value["result"]["structuredContent"].is_object() {
            set_mcp_content_json(&mut value["result"]);
        }
        value.to_string().len() + 1
    })
    .map(|_| ())
}

#[cfg(test)]
#[path = "agent_response_tests.rs"]
mod tests;
