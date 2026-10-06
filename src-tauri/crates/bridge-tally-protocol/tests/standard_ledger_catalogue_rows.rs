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

/// A live capture of a licensed Silver 7.1 company (`BRIDGE SHAPE LAB`), 43
/// ledgers, taken before #1085. One row's `NAME` attribute is `ROUND OFF` and
/// its stored name `Round Off`.
const SHAPE_LAB_COMPANY_NAME: &str = "BRIDGE SHAPE LAB";
const SHAPE_LAB_COMPANY_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const SHAPE_LAB_LEDGER_CATALOGUE: &[u8] =
    include_bytes!("fixtures/agent/native-shape-lab-ledger-catalogue.utf16le.xml");

fn decoded_shape_lab_catalogue() -> String {
    decode_tally_xml_response_bytes_limited(
        SHAPE_LAB_LEDGER_CATALOGUE,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        SHAPE_LAB_LEDGER_CATALOGUE.len(),
    )
    .expect("captured BOM-less UTF-16LE response decodes")
    .text
}

/// The `(row spelling, stored name)` pairs the catalogue holds for `xml`.
fn shape_lab_spellings(xml: &str) -> Vec<(String, Option<String>)> {
    parse_standard_ledger_catalog_with_identities(
        xml,
        SHAPE_LAB_COMPANY_NAME,
        SHAPE_LAB_COMPANY_GUID,
    )
    .expect("the shape lab capture parses")
    .spellings()
    .map(|(row, stored)| (row.to_string(), stored.map(str::to_string)))
    .collect()
}

/// A ledger whose name arrives in the row's attribute and in its own list
/// under two spellings carries both; every other ledger of the capture, whose
/// two spellings are one (several hold escaped markup and line breaks), carries
/// its row spelling alone.
#[test]
fn a_ledger_whose_stored_name_differs_carries_both_spellings() {
    let spellings = shape_lab_spellings(&decoded_shape_lab_catalogue());
    assert_eq!(spellings.len(), 43);
    let differing = spellings
        .iter()
        .filter(|(_, stored)| stored.is_some())
        .collect::<Vec<_>>();
    assert_eq!(
        differing,
        [&("ROUND OFF".to_string(), Some("Round Off".to_string()))]
    );
}

/// Edits of the captured `ROUND OFF` row, to its list only. The capture holds
/// one name there; these are the shapes the parser must still decide.
fn shape_lab_with_round_off_list(list: &str) -> String {
    let xml = decoded_shape_lab_catalogue();
    let start = xml.find("<LEDGER NAME=\"ROUND OFF\"").unwrap();
    let from = start + xml[start..].find("<LANGUAGENAME.LIST>").unwrap();
    let to = start + xml[start..].find("</LEDGER>").unwrap();
    format!("{}{list}{}", &xml[..from], &xml[to..])
}

fn round_off_stored(list: &str) -> Option<String> {
    shape_lab_spellings(&shape_lab_with_round_off_list(list))
        .into_iter()
        .find(|(row, _)| row == "ROUND OFF")
        .expect("the row is still there")
        .1
}

const ROUND_OFF_LIST_HEAD: &str = "<LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\">";
const ROUND_OFF_LIST_TAIL: &str = "</NAME.LIST><LANGUAGEID>0</LANGUAGEID></LANGUAGENAME.LIST>";

/// Only the first name is the ledger's own; the others are aliases and never
/// identity, whatever they say.
#[test]
fn a_later_name_in_the_list_is_an_alias_and_never_the_stored_name() {
    let list = format!(
        "{ROUND_OFF_LIST_HEAD}<NAME>Round Off</NAME><NAME>Rounding</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    assert_eq!(round_off_stored(&list).as_deref(), Some("Round Off"));
}

/// An empty or unusable first name leaves the ledger on its row spelling:
/// the alias after it is not promoted, and the read is not refused.
#[test]
fn an_unusable_first_name_leaves_the_row_spelling_and_does_not_promote_an_alias() {
    let blank =
        format!("{ROUND_OFF_LIST_HEAD}<NAME>   </NAME><NAME>Rounding</NAME>{ROUND_OFF_LIST_TAIL}");
    assert_eq!(round_off_stored(&blank), None);
    let bidi = format!(
        "{ROUND_OFF_LIST_HEAD}<NAME>Round\u{202e}Off</NAME><NAME>Rounding</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    assert_eq!(round_off_stored(&bidi), None);
}

/// A row with no list is known by its row spelling, and a second list adds
/// nothing: only the first can name the ledger.
#[test]
fn a_row_without_a_list_has_no_stored_name_and_a_second_list_names_nothing() {
    assert_eq!(round_off_stored(""), None);
    let list = format!(
        "{ROUND_OFF_LIST_HEAD}<NAME>   </NAME>{ROUND_OFF_LIST_TAIL}\
         {ROUND_OFF_LIST_HEAD}<NAME>Second List</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    assert_eq!(round_off_stored(&list), None);
}

/// A name outside `NAME.LIST` is not the ledger's own.
#[test]
fn a_name_outside_the_name_list_is_not_the_stored_name() {
    let list =
        "<LANGUAGENAME.LIST><NAME>Round Off</NAME><LANGUAGEID>0</LANGUAGEID></LANGUAGENAME.LIST>";
    assert_eq!(round_off_stored(list), None);
}

/// A first name Tally wrote with an entity this parser cannot decode is unusable,
/// not fatal: the catalogue still parses (the skip it replaced never decoded the
/// list), the ledger keeps its row spelling, and the alias after it is not promoted.
#[test]
fn an_undecodable_first_name_leaves_the_row_spelling_and_fails_no_read() {
    let list = format!(
        "{ROUND_OFF_LIST_HEAD}<NAME>A &bogus; B</NAME><NAME>Rounding</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    assert_eq!(round_off_stored(&list), None);
}

/// A self-closing first name is still the first name: the alias after it is not
/// promoted to the ledger's own name.
#[test]
fn a_self_closing_first_name_does_not_let_an_alias_in() {
    let list = format!("{ROUND_OFF_LIST_HEAD}<NAME/><NAME>Rounding</NAME>{ROUND_OFF_LIST_TAIL}");
    assert_eq!(round_off_stored(&list), None);
}

/// Only a `NAME` of a `NAME.LIST` is a name: one nested in another child of the
/// list is not the ledger's own.
#[test]
fn a_name_nested_in_another_child_of_the_list_is_not_the_stored_name() {
    let list =
        "<LANGUAGENAME.LIST><LANGUAGEID><NAME>Round Off</NAME></LANGUAGEID></LANGUAGENAME.LIST>";
    assert_eq!(round_off_stored(list), None);
}
