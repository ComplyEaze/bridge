//! The company list keeps every row, and each name exactly as Tally reported
//! it (#2433). In-memory mutations of one retained capture, not new fixtures.
use bridge_tally_protocol::xml_read_profiles::{CompanyNameRefusal, ValidatedCompanyName};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_companies_from_collection,
    ExpectedTallyTextEncoding,
};

const CAPTURE: &[u8] =
    include_bytes!("fixtures/note-days/shape_lab_company_list_response.utf16le.xml");
const NAME: &str = "BRIDGE SHAPE LAB";
const GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";

fn captured() -> String {
    decode_tally_xml_response_bytes_limited(
        CAPTURE,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        CAPTURE.len(),
    )
    .unwrap()
    .text
}

/// The capture with its one row repeated under a second GUID, whose name (in
/// its attribute and element) is `second`.
fn two_rows(second: &str) -> String {
    let xml = captured();
    let start = xml.find("<COMPANY ").unwrap();
    // From the row on: `BODY/DESC/CMPINFO` holds an earlier `<COMPANY>0</COMPANY>` counter.
    let end = start + xml[start..].find("</COMPANY>").unwrap() + "</COMPANY>".len();
    let copy = xml[start..end]
        .replace(NAME, second)
        .replace(GUID, "3a6bd6e1-b835-4bff-89dd-8a6af138c347");
    let mut out = xml.clone();
    out.insert_str(end, &copy);
    out
}

#[test]
fn a_name_a_request_cannot_carry_no_longer_refuses_the_list() {
    let long = "A".repeat(300);
    for (second, held, refusal) in [
        (
            long.as_str(),
            long.clone(),
            Some(CompanyNameRefusal::OverLimit),
        ),
        (
            "&#x9;LAB",
            "\tLAB".to_string(),
            Some(CompanyNameRefusal::ControlCharacter),
        ),
        ("", String::new(), Some(CompanyNameRefusal::Blank)),
        ("  LAB  ", "  LAB  ".to_string(), None),
    ] {
        let companies = parse_companies_from_collection(&two_rows(second)).unwrap();
        assert_eq!(companies.len(), 2, "{second:?}: both rows list");
        assert_eq!(companies[0].name, NAME);
        assert_eq!(ValidatedCompanyName::refusal(&companies[0].name), None);
        assert_eq!(companies[1].name, held, "held byte for byte, never trimmed");
        assert_eq!(ValidatedCompanyName::refusal(&companies[1].name), refusal);
    }
}

#[test]
fn a_row_with_no_name_attribute_is_still_a_shape_error() {
    let xml = captured().replacen(&format!(" NAME=\"{NAME}\""), "", 1);
    let error = parse_companies_from_collection(&xml).unwrap_err();
    assert_eq!(
        error.to_string(),
        "company collection omitted the company name"
    );
}
