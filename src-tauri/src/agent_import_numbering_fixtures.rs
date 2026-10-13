//! Voucher-type answers for tests that pass through the voucher-numbering read
//! (bridge#724), made from two committed live captures: the voucher-type
//! masters answer and the series-numbering answer. Each keeps only the four
//! voucher types a native voucher can be. Their AlterIDs are set to 1 to 4, so
//! the test books' master mark of 7 bounds them, and each type's GUID in the
//! numbering answer is set to its GUID in the masters answer, so the two join.
//! The captured book numbers Payment and Receipt Manual; these answers number
//! every type Automatic except those a test names, which read Manual with the
//! duplicate setting the capture printed. Every series keeps the sub-method the
//! capture printed, `Auto Retain`, unless a test sets one with
//! [`with_sub_method`].
use super::post::numbering::admit_numbering;
use crate::agent::responses::{
    enforce_jsonrpc_response_byte_cap, enforce_mcp_result_byte_cap, enforce_response_byte_cap,
};
use crate::tally::runtime::VoucherNumberingRead;
use bridge_tally_protocol::native_voucher_type_numbering::{
    DuplicateSetting, NumberingSubMethod, SeriesNumberingMethod, VoucherTypeNumbering,
};
use serde_json::{json, Value};
use tally_protocol_simulator::{Fixture, ScenarioPlan};

/// The voucher types a native voucher can be, in the order their AlterIDs are
/// set.
pub(in crate::agent) const NUMBERED_TYPES: [&str; 4] = ["Payment", "Receipt", "Journal", "Contra"];

const MASTERS_CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const NUMBERING_CAPTURE_GUID: &str = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
    .trim_start_matches('\u{feff}')
    .to_string()
}

/// The `VOUCHERTYPE` element named `name` in `text`.
fn block<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text
        .find(&format!("<VOUCHERTYPE NAME=\"{name}\""))
        .unwrap_or_else(|| panic!("{name} is captured"));
    let end = start + text[start..].find("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
    &text[start..end]
}

/// `block` with the text of its first `<tag ...>` element set to `value`.
fn set(block: &str, tag: &str, value: &str) -> String {
    let open = block.find(&format!("<{tag} ")).unwrap();
    let text = open + block[open..].find('>').unwrap() + 1;
    let end = text + block[text..].find(&format!("</{tag}>")).unwrap();
    format!("{}{}{}", &block[..text], value, &block[end..])
}

fn text_of<'a>(block: &'a str, tag: &str) -> &'a str {
    let open = block.find(&format!("<{tag} ")).unwrap();
    let text = open + block[open..].find('>').unwrap() + 1;
    &block[text..text + block[text..].find(&format!("</{tag}>")).unwrap()]
}

/// `text` with its `VOUCHERTYPE` elements replaced by `rows`.
fn with_rows(text: &str, rows: &str) -> String {
    let start = text.find("<VOUCHERTYPE ").unwrap();
    let end = text.rfind("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
    format!("{}{}{}", &text[..start], rows, &text[end..])
}

/// The sub-method element every captured series prints.
const CAPTURED_SUB_METHOD: &str = "<NUMBERINGSUBMETHOD>Auto Retain</NUMBERINGSUBMETHOD>";

/// `plans` with the series of the voucher type named `voucher_type` printing
/// `element` in place of its captured sub-method element, in both answers of
/// the numbering pair. One edit of the captured answer; nothing else changes.
pub(in crate::agent) fn with_sub_method(
    mut plans: Vec<ScenarioPlan>,
    voucher_type: &str,
    element: &str,
) -> Vec<ScenarioPlan> {
    let mut changed = 0;
    for plan in &mut plans {
        let Fixture::SyntheticXml(body) = &plan.fixture else {
            continue;
        };
        if !body.contains("<VOUCHERNUMBERSERIES.LIST>") {
            continue;
        }
        let row = block(body, voucher_type);
        assert_eq!(
            row.matches(CAPTURED_SUB_METHOD).count(),
            1,
            "{voucher_type}"
        );
        let edited = row.replacen(CAPTURED_SUB_METHOD, element, 1);
        assert_eq!(body.matches(row).count(), 1, "{voucher_type}");
        let body = body.replacen(row, &edited, 1);
        plan.fixture = Fixture::SyntheticXml(body);
        changed += 1;
    }
    assert_eq!(changed, 2, "the numbering pair");
    plans
}

/// `structured`, a tool's answer to a numbering refusal, naming all four
/// voucher types in place of those it names (the refused types under `at`,
/// and the message where the answer carries one), with the readings that
/// make the message longest (one set to renumber, three with a sub-method not
/// recognised); then held to `max_bytes` by the three caps an answer passes on
/// its way out. The framed answer, or the cap's refusal.
pub(in crate::agent) fn four_types_through_the_caps(
    mut structured: Value,
    at: &str,
    max_bytes: usize,
) -> Result<Value, String> {
    let automatic = |sub_method| VoucherTypeNumbering::Observed {
        method: SeriesNumberingMethod::Automatic(sub_method),
        duplicates: DuplicateSetting::NotPrevented,
    };
    let read = VoucherNumberingRead::for_test(
        "",
        0,
        NUMBERED_TYPES
            .iter()
            .enumerate()
            .map(|(at, name)| {
                let sub_method = if at == 0 {
                    NumberingSubMethod::AutoRenumber
                } else {
                    NumberingSubMethod::Unrecognised(String::new())
                };
                ((*name).to_string(), automatic(sub_method))
            })
            .collect(),
    );
    let refusal = admit_numbering(read, NUMBERED_TYPES).expect_err("every type is refused");
    let named = structured.pointer_mut(at).expect(at);
    named["refused_types"] = refusal.refused_types();
    if named.get("message").is_some() {
        named["message"] = json!(refusal.message());
    }
    let (structured, _, rows) = enforce_response_byte_cap(structured, max_bytes)?;
    let mut result = json!({
        "content": [{"type":"text", "text": ""}],
        "structuredContent": structured,
        "isError": true,
    });
    enforce_mcp_result_byte_cap(&mut result, max_bytes, "", rows)?;
    let mut framed = json!({"jsonrpc":"2.0", "id":1, "result":result});
    enforce_jsonrpc_response_byte_cap(&mut framed, max_bytes)?;
    Ok(framed)
}

/// The masters answer and the numbering answer of `company_guid`'s four voucher
/// types, the types in `manual` numbered Manual and the rest Automatic.
pub(in crate::agent) fn numbering_answers(company_guid: &str, manual: &[&str]) -> (String, String) {
    let masters = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/masters_voucher_types_shape_lab_live.utf16le.xml"
    ))
    .replace(MASTERS_CAPTURE_GUID, company_guid);
    let numbering = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_type_numbering_series_live.utf16le.xml"
    ))
    .replace(NUMBERING_CAPTURE_GUID, company_guid);
    let mut master_rows = String::new();
    let mut numbering_rows = String::new();
    for (at, name) in NUMBERED_TYPES.iter().enumerate() {
        let alter_id = format!(" {}", at + 1);
        let master = set(block(&masters, name), "ALTERID", &alter_id);
        let guid = text_of(&master, "GUID").to_string();
        master_rows.push_str(&master);
        let series = set(
            &set(block(&numbering, name), "GUID", &guid),
            "ALTERID",
            &alter_id,
        );
        let (from, to) = if manual.contains(name) {
            (
                "<NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
                "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
            )
        } else {
            (
                "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
                "<NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
            )
        };
        numbering_rows.push_str(&series.replace(from, to));
    }
    (
        with_rows(&masters, &master_rows),
        with_rows(&numbering, &numbering_rows),
    )
}
