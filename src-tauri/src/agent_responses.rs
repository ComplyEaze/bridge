//! Responses for the local MCP adapter.
use super::agent_import::{check_not_run_section, counts_only_section, trim_section};
use super::*;

#[derive(Clone, Copy)]
enum PageShape {
    Outstandings,
    Rows(&'static str),
    /// A list of the newest records first, with no offset to resume from (the egress log, #2125):
    /// trimmed to its newest rows, the count left out said in `omitted_for_size`.
    Newest(&'static str),
}

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
    if let Some(widths) = outstandings_axis_widths(result) {
        return Some((
            PageShape::Outstandings,
            widths.into_iter().max().unwrap_or(0),
        ));
    }
    // The egress log names its order: its records are the newest receipts, and a trimmed answer
    // keeps the newest. No other result carries `order`, so no other `records` list is trimmed.
    if result["order"] == "newest_first" {
        if let Some(rows) = result["records"].as_array() {
            return Some((PageShape::Newest("records"), rows.len()));
        }
    }
    ["items", "ledgers", "masters"].into_iter().find_map(|key| {
        // `masters` also keys the unpaged results of validate_masters and
        // build_import_xml, `items` the unpaged result of gst_uncertain_vouchers and
        // `ledgers` the unpaged negative_ledgers answer; none carries an `offset`:
        // only a paged result can be trimmed with a resumable cursor.
        if !result["offset"].is_u64() {
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
                if let Some(shown) = figures.get_mut("open_bills_shown") {
                    *shown = json!(width);
                }
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
        PageShape::Newest(key) => {
            let rows = result[key].as_array_mut().expect("observed page rows");
            let cut = rows.len().saturating_sub(width) as u64;
            rows.truncate(width);
            // Each cap check may cut again: what is left out adds up.
            let before = result["omitted_for_size"].as_u64().unwrap_or(0);
            result["omitted_for_size"] = json!(before + cut);
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

/// The computed `transfers` section of a result, if it has one.
fn computed_section(response: &Value, structured_path: &str) -> Option<Value> {
    response
        .pointer(&format!("{structured_path}/result/transfers"))
        .filter(|section| section["state"] == "computed")
        .cloned()
}

/// Put `original` into the result as wide as the real encoded response allows,
/// largest first: as many open rows as fit (the rest counted in `open_omitted`),
/// then the counts alone, then a note that the check is to be run again.
/// Whether the response fits.
fn fit_section(
    response: &mut Value,
    structured_path: &str,
    max_bytes: usize,
    encoded_len: &mut impl FnMut(&mut Value) -> usize,
    original: &Value,
) -> bool {
    let pointer = format!("{structured_path}/result/transfers");
    // a statement of only skipped rows has no batch to verify
    let recorded = response.pointer(&format!("{structured_path}/result/state"))
        == Some(&json!("skips_recorded"));
    let mut place = |response: &mut Value, section: Value| {
        if let Some(slot) = response.pointer_mut(&pointer) {
            *slot = section;
        }
        encoded_len(response) <= max_bytes
    };
    let rows = original["open"].as_array().map_or(0, Vec::len);
    let (mut lower, mut upper) = (0, rows);
    let mut best = None;
    while lower <= upper {
        let keep = lower + (upper - lower) / 2;
        if place(response, trim_section(original, keep)) {
            best = Some(keep);
            lower = keep + 1;
        } else if keep == 0 {
            break;
        } else {
            upper = keep - 1;
        }
    }
    match best {
        Some(keep) => place(response, trim_section(original, keep)),
        None => {
            place(response, counts_only_section(original))
                || place(response, check_not_run_section(recorded))
        }
    }
}

/// The widest page, of at least one row, that fits the response as it is now.
fn trim_page(
    original: &Value,
    structured_path: &str,
    shape: PageShape,
    width: usize,
    max_bytes: usize,
    encoded_len: &mut impl FnMut(&mut Value) -> usize,
) -> Result<Option<Value>, String> {
    let (mut lower, mut upper) = (1, width.saturating_sub(1));
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
    Ok(best)
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
    // The transfers section is the least essential part of a result, so it gives
    // way before any row of a page does, in this order:
    //   1. the page at full width, with the widest section that fits beside it
    //      (open rows, then the counts, then a note that the check is to be run
    //      again): the page is not cut if any of them fits;
    //   2. otherwise the page trimmed, with the shorter of the note and the
    //      counts beside it, as wide as it goes;
    //   3. then the section widened again into what the trimmed page leaves.
    // A result with no computed section takes only the page step, as it always
    // did. Everything here returns true when anything was cut, the section's
    // rows included, but the tool path discards that flag. The answer says what
    // was narrowed in-band (`open_omitted`, a `not_established` note); only the
    // drop of the whole section from a result with no page sets the envelope's
    // `truncated`.
    let section = computed_section(response, structured_path);
    if let Some(section) = &section {
        if fit_section(
            response,
            structured_path,
            max_bytes,
            &mut encoded_len,
            section,
        ) {
            return Ok(true);
        }
    }
    let Some((shape, width)) = response.pointer(structured_path).and_then(page_shape) else {
        // A result with no page (a written build) has nothing to trim. If it
        // fits without its section, whatever the section says, it is answered
        // without it and the envelope is marked `truncated`: a build whose file
        // and journal record are written is never turned into an error by the
        // section.
        let dropped = response
            .pointer_mut(&format!("{structured_path}/result"))
            .and_then(Value::as_object_mut)
            .is_some_and(|result| result.remove("transfers").is_some());
        if dropped {
            if let Some(envelope) = response
                .pointer_mut(structured_path)
                .and_then(Value::as_object_mut)
            {
                envelope.insert("truncated".into(), json!(true));
            }
            if encoded_len(response) <= max_bytes {
                return Ok(true);
            }
        }
        return Err("agent_response_too_large".to_string());
    };
    let mut original = response.clone();
    if let Some(section) = &section {
        // A result with a page is a batch's or a verification's, never a
        // statement of only skipped rows, so the note is the batch's. The page
        // is trimmed beside the shorter of the note and the counts.
        let note = check_not_run_section(false);
        let counts = counts_only_section(section);
        let placeholder = if counts.to_string().len() < note.to_string().len() {
            counts
        } else {
            note
        };
        if let Some(slot) = original.pointer_mut(&format!("{structured_path}/result/transfers")) {
            *slot = placeholder;
        }
    }
    let trimmed = trim_page(
        &original,
        structured_path,
        shape,
        width,
        max_bytes,
        &mut encoded_len,
    )?;
    *response = trimmed.ok_or_else(|| "agent_response_too_large".to_string())?;
    if let Some(section) = &section {
        fit_section(
            response,
            structured_path,
            max_bytes,
            &mut encoded_len,
            section,
        );
    }
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
        "buckets",
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
