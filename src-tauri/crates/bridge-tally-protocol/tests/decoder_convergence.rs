//! The standard ledger catalogue reads a forbidden numeric reference by the same
//! rule as the native collection parsers
//! (`docs/tally/TALLY_PROTOCOL_REFERENCE.md` §1.1(d)).

use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, is_tally_reserved_root,
    parse_standard_ledger_catalog_with_identities, ExpectedTallyTextEncoding,
    TALLY_SANITIZED_ROOT_MARKER,
};

const COMPANY_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";

const LEDGER_CATALOGUE: &[u8] =
    include_bytes!("fixtures/agent/native-ledger-catalogue.utf16le.xml");

fn decode_utf16le(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("captured BOM-less UTF-16LE response decodes")
    .text
}

#[test]
fn the_standard_ledger_catalogue_reads_a_captured_root_parent_as_the_marker() {
    // The captured `List of Ledgers` carries `&#4; Primary` as the parent of
    // `Profit & Loss A/c`. Read as a raw U+0004, the catalogue discarded it as
    // an unsafe display character and reported the parent as unobserved.
    let xml = decode_utf16le(LEDGER_CATALOGUE);
    assert_eq!(
        xml.matches("<PARENT TYPE=\"String\">&#4; Primary</PARENT>")
            .count(),
        1
    );
    let catalogue =
        parse_standard_ledger_catalog_with_identities(&xml, "WR2 Unicode Lab", COMPANY_GUID)
            .expect("the captured catalogue parses");
    let parent = catalogue
        .parents()
        .find(|(name, _)| *name == "Profit & Loss A/c")
        .map(|(_, parent)| parent)
        .expect("the captured catalogue lists the root-parented ledger");
    assert_eq!(
        parent,
        Some(format!("{TALLY_SANITIZED_ROOT_MARKER} Primary").as_str())
    );
    assert!(is_tally_reserved_root(parent.unwrap()));
}
