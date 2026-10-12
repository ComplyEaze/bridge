//! Voucher-type answers for tests that pass through the voucher-numbering read
//! (bridge#724), made from two committed live captures: the voucher-type
//! masters answer and the series-numbering answer. Each keeps only the four
//! voucher types a native voucher can be. Their AlterIDs are set to 1 to 4, so
//! the test books' master mark of 7 bounds them, and each type's GUID in the
//! numbering answer is set to its GUID in the masters answer, so the two join.
//! The captured book numbers Payment and Receipt Manual; these answers number
//! every type Automatic except those a test names, which read Manual with the
//! duplicate setting the capture printed.

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
