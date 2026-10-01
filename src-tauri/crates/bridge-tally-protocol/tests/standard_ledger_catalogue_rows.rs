//! Both parsers of Tally's `List of Ledgers` hold a whole large book
//! (bridge#634). They read the same response, which carries every ledger, and
//! a 1,000-row bound refused any book past it: the catalogue for the `vouchers`
//! ledger filter and import validation, and the identity observation for direct
//! company bootstrap.

use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_standard_ledger_catalog_with_identities,
    parse_standard_ledger_identity_observation, ExpectedTallyTextEncoding,
    StandardLedgerCatalogError,
};

const COMPANY_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const LEDGER_CATALOGUE: &[u8] =
    include_bytes!("fixtures/agent/native-ledger-catalogue.utf16le.xml");

/// A synthetic lab company named `BRIDGE ESCAPE & LAB` (GUID
/// `21463ec7-c236-44fb-a06d-20f0ad8a6df3`), captured 28 Sep 2026 15:28 IST on
/// licensed TallyPrime 7.1 Silver via Bridge's own `StandardLedgerCatalogV1`
/// request, with the company name sent XML-escaped as master already renders
/// it (bridge#832; see `escape-lab-ledger-catalogue.json`).
const ESCAPE_LAB_COMPANY_NAME: &str = "BRIDGE ESCAPE & LAB";
const ESCAPE_LAB_COMPANY_GUID: &str = "21463ec7-c236-44fb-a06d-20f0ad8a6df3";
const ESCAPE_LAB_LEDGER_CATALOGUE: &[u8] =
    include_bytes!("fixtures/agent/escape-lab-ledger-catalogue.utf16le.xml");

#[test]
fn a_list_of_ledgers_past_a_thousand_rows_parses_whole() {
    let captured = decode_tally_xml_response_bytes_limited(
        LEDGER_CATALOGUE,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        LEDGER_CATALOGUE.len(),
    )
    .expect("captured BOM-less UTF-16LE response decodes")
    .text;
    // Each added row is the capture's first row under its own name and GUID.
    let start = captured.find("<LEDGER NAME=").unwrap();
    let end = start + captured[start..].find("</LEDGER>").unwrap() + "</LEDGER>".len();
    let template = &captured[start..end];
    assert!(template.contains("Bridge Nested Debtor WR4") && template.contains("-000000d5<"));
    let rows = (0..1_000)
        .map(|index| {
            template
                .replace(
                    "Bridge Nested Debtor WR4",
                    &format!("Bulk Ledger {index:04}"),
                )
                .replace("-000000d5<", &format!("-b{index:07x}<"))
        })
        .collect::<String>();
    let close = captured.rfind("</COLLECTION>").unwrap();
    let xml = format!("{}{rows}{}", &captured[..close], &captured[close..]);
    let captured_rows = captured.matches("<LEDGER NAME=").count();
    assert_eq!(xml.matches("<LEDGER NAME=").count(), captured_rows + 1_000);

    let catalogue =
        parse_standard_ledger_catalog_with_identities(&xml, "WR2 Unicode Lab", COMPANY_GUID)
            .expect("a book past a thousand ledgers is one catalogue");
    assert_eq!(catalogue.names().count(), captured_rows + 1_000);
    let observed = parse_standard_ledger_identity_observation(&xml, "WR2 Unicode Lab")
        .expect("a book past a thousand ledgers confirms its company");
    assert_eq!(observed.ledger_count, (captured_rows + 1_000) as u64);
    assert!(observed.company_guid.eq_ignore_ascii_case(COMPANY_GUID));
}

fn decoded_escape_lab_catalogue() -> String {
    decode_tally_xml_response_bytes_limited(
        ESCAPE_LAB_LEDGER_CATALOGUE,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        ESCAPE_LAB_LEDGER_CATALOGUE.len(),
    )
    .expect("captured BOM-less UTF-16LE response decodes")
    .text
}

/// A live capture of a company name that needed XML escaping is accepted for
/// its own (escaped) company name and GUID, and returns exactly the two
/// ledgers it carries (bridge#832).
#[test]
fn a_capture_of_a_company_name_needing_escaping_is_accepted_for_its_own_name() {
    let xml = decoded_escape_lab_catalogue();
    let catalogue = parse_standard_ledger_catalog_with_identities(
        &xml,
        ESCAPE_LAB_COMPANY_NAME,
        ESCAPE_LAB_COMPANY_GUID,
    )
    .expect("the escaped capture is accepted for its own escaped company name");
    let mut names = catalogue.names().collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(names, ["Cash", "Profit & Loss A/c"]);
}

/// The same bytes, unchanged, are refused for a different expected company
/// name -- the negative control: acceptance above is specific to this
/// capture's own name, not a parser that ignores the identity check.
#[test]
fn the_same_capture_is_refused_for_a_different_expected_company_name() {
    let xml = decoded_escape_lab_catalogue();
    let error = parse_standard_ledger_catalog_with_identities(
        &xml,
        "BRIDGE SOME OTHER LAB",
        ESCAPE_LAB_COMPANY_GUID,
    )
    .expect_err("a different expected company name must be refused");
    assert_eq!(error, StandardLedgerCatalogError::CompanyIdentityMismatch);
}
