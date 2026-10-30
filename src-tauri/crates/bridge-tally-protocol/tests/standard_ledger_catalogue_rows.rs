//! Both parsers of Tally's `List of Ledgers` hold a whole large book
//! (bridge#634). They read the same response, which carries every ledger, and
//! a 1,000-row bound refused any book past it: the catalogue for the `vouchers`
//! ledger filter and import validation, and the identity observation for direct
//! company bootstrap.

use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_ledger_name_index,
    parse_standard_ledger_catalog_with_identities, parse_standard_ledger_identity_observation,
    ExpectedTallyTextEncoding, StandardLedgerCatalogError,
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

/// The catalogue an import is bound against offers and binds a ledger by its
/// row spelling, never by its stored name (#1085: slice (b), writing the stored
/// name, is not built). On the captured book the row spelling `ROUND OFF`
/// binds, and its stored name `Round Off` is neither offered nor bound. This
/// pins the binding the build uses, not the build end to end.
#[test]
fn the_catalogue_binds_the_row_spelling_and_not_the_stored_name() {
    let catalog = parse_standard_ledger_catalog_with_identities(
        &decoded_shape_lab_catalogue(),
        SHAPE_LAB_COMPANY_NAME,
        SHAPE_LAB_COMPANY_GUID,
    )
    .expect("the shape lab capture parses");
    assert!(catalog.names().any(|name| name == "ROUND OFF"));
    assert!(!catalog.names().any(|name| name == "Round Off"));
    assert!(catalog.bind_selected(["ROUND OFF".to_string()]).is_ok());
    assert!(catalog.bind_selected(["Round Off".to_string()]).is_err());
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

// ---- Aliases (#1688) ---------------------------------------------------------------------------------
//
// The `ROUND OFF` capture holds no alias, so every aliased state in this section is that row with its list
// edited by these tests: constructed, not captured. They reach shapes no capture holds (an unusable or a
// repeated alias). Tally's own answers with aliases are tested in the section after it (`alias-lab-1688`).

/// The `(carrying ledger, alias)` pairs the catalogue holds for `xml`.
fn shape_lab_aliases(xml: &str) -> Vec<(String, String)> {
    parse_standard_ledger_catalog_with_identities(
        xml,
        SHAPE_LAB_COMPANY_NAME,
        SHAPE_LAB_COMPANY_GUID,
    )
    .expect("the edited shape lab capture still parses")
    .aliases()
    .map(|(owner, alias)| (owner.to_string(), alias.to_string()))
    .collect()
}

fn round_off_aliases(list: &str) -> Vec<String> {
    shape_lab_aliases(&shape_lab_with_round_off_list(list))
        .into_iter()
        .inspect(|(owner, _)| assert_eq!(owner, "ROUND OFF"))
        .map(|(_, alias)| alias)
        .collect()
}

/// A ledger with no alias, as every ledger of the capture, yields none.
#[test]
fn a_capture_without_aliases_has_none() {
    assert_eq!(shape_lab_aliases(&decoded_shape_lab_catalogue()), []);
}

/// Every name after the first is an alias, read verbatim and in the order
/// Tally listed it, once each; a name the ledger already answers to (its row
/// spelling or its own name) is not an alias of it.
#[test]
fn the_names_after_the_first_are_the_ledgers_aliases() {
    let list = format!(
        "{ROUND_OFF_LIST_HEAD}<NAME>Round Off</NAME><NAME>Rounding</NAME><NAME>Paise  Adj</NAME>\
         <NAME>Rounding</NAME><NAME>ROUND OFF</NAME><NAME>Round Off</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    assert_eq!(round_off_aliases(&list), ["Rounding", "Paise  Adj"]);
}

/// A name the walk cannot use is left out and fails no read: blank, self-closing,
/// undecodable, or a forged name.
#[test]
fn an_unusable_alias_is_left_out_and_fails_no_read() {
    let list = format!(
        "{ROUND_OFF_LIST_HEAD}<NAME>Round Off</NAME><NAME>   </NAME><NAME/><NAME>A &bogus; B</NAME>\
         <NAME>Round\u{202e}Off</NAME><NAME>Rounding</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    assert_eq!(round_off_aliases(&list), ["Rounding"]);
    assert_eq!(
        round_off_stored(&list).as_deref(),
        Some("Round Off"),
        "the own name is read as before"
    );
}

/// An unusable first name leaves the ledger on its row spelling and promotes
/// nothing, but the names after it are still its aliases.
#[test]
fn an_unusable_first_name_does_not_stop_the_aliases_after_it() {
    let list = format!("{ROUND_OFF_LIST_HEAD}<NAME/><NAME>Rounding</NAME>{ROUND_OFF_LIST_TAIL}");
    assert_eq!(round_off_stored(&list), None);
    assert_eq!(round_off_aliases(&list), ["Rounding"]);
}

/// A second `NAME.LIST` of the first list is "after the first name" too; a
/// second `LANGUAGENAME.LIST` is not read (never measured to carry aliases).
#[test]
fn only_the_first_language_list_carries_aliases() {
    let list = format!(
        "<LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\"><NAME>Round Off</NAME><NAME>Rounding</NAME></NAME.LIST>\
         <NAME.LIST TYPE=\"String\"><NAME>Second Of First</NAME></NAME.LIST><LANGUAGEID>0</LANGUAGEID></LANGUAGENAME.LIST>\
         {ROUND_OFF_LIST_HEAD}<NAME>Second List</NAME><NAME>Second List Alias</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    assert_eq!(round_off_aliases(&list), ["Rounding", "Second Of First"]);
}

/// A name outside `NAME.LIST` is no alias.
#[test]
fn a_name_outside_the_name_list_is_no_alias() {
    let list = "<LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\"><NAME>Round Off</NAME></NAME.LIST>\
                <NAME>Loose</NAME><LANGUAGEID><NAME>Nested</NAME></LANGUAGEID></LANGUAGENAME.LIST>";
    assert_eq!(round_off_aliases(list), Vec::<String>::new());
}

/// An alias is not a ledger name: one equal, apart from case, to another
/// ledger's name does not make the catalogue's names collide
/// (`DuplicateIdentity`), and it is not counted among the names.
#[test]
fn an_alias_equal_apart_from_case_to_another_ledger_is_not_a_duplicate_identity() {
    let xml = decoded_shape_lab_catalogue();
    let catalogue = parse_standard_ledger_catalog_with_identities(
        &xml,
        SHAPE_LAB_COMPANY_NAME,
        SHAPE_LAB_COMPANY_GUID,
    )
    .expect("parses");
    let other = catalogue
        .names()
        .find(|name| *name != "ROUND OFF" && name.to_uppercase() != *name)
        .expect("another ledger with lower case letters")
        .to_string();
    let alias = other.to_uppercase();
    let list = format!(
        "{ROUND_OFF_LIST_HEAD}<NAME>Round Off</NAME><NAME>{alias}</NAME>{ROUND_OFF_LIST_TAIL}"
    );
    let edited = shape_lab_with_round_off_list(&list);
    let parsed = parse_standard_ledger_catalog_with_identities(
        &edited,
        SHAPE_LAB_COMPANY_NAME,
        SHAPE_LAB_COMPANY_GUID,
    )
    .expect("an alias is not an identity, so no duplicate");
    assert_eq!(parsed.names().count(), 43);
    assert_eq!(
        parsed.aliases().collect::<Vec<_>>(),
        [("ROUND OFF", alias.as_str())]
    );
}

// ---- The V2 catalogue on captures of a live Tally (#1234) ------------------------------------------
//
// Four synthetic books answered Bridge's own `StandardLedgerCatalogV2` request on 6 Oct 2026: V1's rows, each with
// the ledger's `ISBILLWISEON`. Each provenance file records the request that was sent and the response's hash.

use bridge_tally_protocol::{
    encode_tally_xml_request_utf16le, parse_standard_ledger_catalog_v2_with_identities,
    xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName},
    BillWiseFlag,
};
use sha2::{Digest, Sha256};

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

struct V2Capture {
    company: &'static str,
    guid: &'static str,
    xml: &'static [u8],
    provenance: &'static str,
    ledgers: usize,
    bill_wise: usize,
}

const V2_CAPTURES: [V2Capture; 4] = [
    V2Capture {
        company: "BRIDGE OUTSTANDINGS LAB",
        guid: "49f1fbda-ee59-4a4b-aacf-b45fe32402d7",
        xml: include_bytes!(
            "fixtures/agent/native-outstandings-detail-ledger-catalogue-v2.utf16le.xml"
        ),
        provenance: include_str!(
            "fixtures/agent/native-outstandings-detail-ledger-catalogue-v2.json"
        ),
        ledgers: 17,
        bill_wise: 10,
    },
    V2Capture {
        company: "BRIDGE AMEND LAB",
        guid: "17a10910-773c-42c6-bd66-7bba9a392536",
        xml: include_bytes!("fixtures/agent/d3-amend-lab-ledger-catalogue-v2.utf16le.xml"),
        provenance: include_str!("fixtures/agent/d3-amend-lab-ledger-catalogue-v2.json"),
        ledgers: 19,
        bill_wise: 4,
    },
    V2Capture {
        company: "BRIDGE SHAPE LAB",
        guid: "3a6bd6e1-b835-4bff-89dd-8a6af138c346",
        xml: include_bytes!("fixtures/agent/native-shape-lab-ledger-catalogue-v2.utf16le.xml"),
        provenance: include_str!("fixtures/agent/native-shape-lab-ledger-catalogue-v2.json"),
        ledgers: 44,
        bill_wise: 18,
    },
    V2Capture {
        company: "WR2 Unicode Lab",
        guid: "61c6de69-1748-461c-ad3f-162cb949df9f",
        xml: include_bytes!("fixtures/agent/native-ledger-catalogue-v2.utf16le.xml"),
        provenance: include_str!("fixtures/agent/native-ledger-catalogue-v2.json"),
        ledgers: 9,
        bill_wise: 2,
    },
];

fn decoded(bytes: &[u8]) -> String {
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
fn every_ledger_of_each_v2_capture_carries_a_flag() {
    for capture in &V2_CAPTURES {
        let catalogue = parse_standard_ledger_catalog_v2_with_identities(
            &decoded(capture.xml),
            capture.company,
            capture.guid,
        )
        .unwrap_or_else(|error| panic!("{}: {error}", capture.company));
        let flags = catalogue
            .bill_wise_flags()
            .map(|(_, _, flag)| flag)
            .collect::<Vec<_>>();
        assert_eq!(flags.len(), capture.ledgers, "{}", capture.company);
        assert_eq!(
            flags
                .iter()
                .filter(|flag| **flag == BillWiseFlag::On)
                .count(),
            capture.bill_wise,
            "{}",
            capture.company
        );
        // The same body is not a V1 answer: the V1 parser refuses the new field.
        assert_eq!(
            parse_standard_ledger_catalog_with_identities(
                &decoded(capture.xml),
                capture.company,
                capture.guid
            )
            .unwrap_err(),
            StandardLedgerCatalogError::MalformedResponse
        );
    }
}

#[test]
fn each_v2_capture_is_the_answer_to_the_request_the_profile_renders() {
    for capture in &V2_CAPTURES {
        let provenance: serde_json::Value = serde_json::from_str(capture.provenance).unwrap();
        let company = ValidatedCompanyName::new(capture.company).unwrap();
        let request = ReadOnlyProfile::StandardLedgerCatalogV2 { company: &company }.render();
        assert_eq!(
            provenance["source_request_sha256"].as_str().unwrap(),
            sha256(&encode_tally_xml_request_utf16le(&request)),
            "{}",
            capture.company
        );
        assert_eq!(
            provenance["source_response_sha256"].as_str().unwrap(),
            sha256(capture.xml),
            "{}",
            capture.company
        );
        assert_eq!(
            provenance["fixture_sha256"],
            provenance["source_response_sha256"]
        );
        assert_eq!(provenance["transformation"], serde_json::json!([]));
    }
}

/// A ledger whose name ends in a carriage return and line feed (the case the
/// separate outstandings snapshot could not read) comes back in the V2
/// catalogue as an escaped attribute, with a flag like any other. One of the
/// four is bill-wise (`Yes`), the case the approval's ledger name must carry.
#[test]
fn ledger_names_ending_in_crlf_carry_a_flag_in_the_v2_catalogue() {
    let mut crlf = Vec::new();
    for capture in [&V2_CAPTURES[1], &V2_CAPTURES[2]] {
        let catalogue = parse_standard_ledger_catalog_v2_with_identities(
            &decoded(capture.xml),
            capture.company,
            capture.guid,
        )
        .unwrap();
        crlf.extend(
            catalogue
                .bill_wise_flags()
                .filter(|(name, _, _)| name.ends_with("\r\n"))
                .map(|(name, _, flag)| (name.to_string(), flag)),
        );
    }
    assert_eq!(
        crlf,
        [
            ("LF708 A\r\n".to_string(), BillWiseFlag::Off),
            ("LF708 B\r\n".to_string(), BillWiseFlag::Off),
            ("lf708 c\r\n".to_string(), BillWiseFlag::Off),
            ("CRLF Supplier\r\n".to_string(), BillWiseFlag::On),
        ]
    );
}

// ---- Aliases as Tally returned them (#1688) ----------------------------------------------------------
//
// Tally's own answers to the three ledger-list requests on a book whose ledgers were given aliases on
// Tally's screen and by import (`fixtures/agent/alias-lab-1688/PROVENANCE.md`).

const ALIAS_LAB_COMPANY_NAME: &str = "BRIDGE ALIAS LAB";
const ALIAS_LAB_COMPANY_GUID: &str = "021bda0c-b1b1-44c8-8279-5212ce87c9c7";
const ALIAS_LAB_SCREEN_V2: &[u8] =
    include_bytes!("fixtures/agent/alias-lab-1688/k3-cv2.utf16le.xml");
const ALIAS_LAB_V1: &[u8] = include_bytes!("fixtures/agent/alias-lab-1688/i1-cat.utf16le.xml");
const ALIAS_LAB_V2: &[u8] = include_bytes!("fixtures/agent/alias-lab-1688/i1-cv2.utf16le.xml");
const ALIAS_LAB_V3: &[u8] = include_bytes!("fixtures/agent/alias-lab-1688/i1-idx.utf16le.xml");

/// `é` is one code point, U+00E9, as Tally returned it.
const CAFE_ALIAS: &str = "Lab Caf\u{e9} Sales";
/// The Devanagari alias, by its code points as the import sent them.
const SCRIPT_ALIAS: &str = "\u{932}\u{948}\u{92c} \u{916}\u{93e}\u{924}\u{93e}";

/// The aliases keyed on Tally's screen, each beside the ledger that carries it, in keyed order.
const SCREEN_ALIASES: [(&str, &str); 4] = [
    ("Lab Other Ledger", "Lab Other First"),
    ("Lab Other Ledger", "Lab Other Second"),
    ("Lab Plain Ledger", "Lab Plain Alias"),
    ("Lab Sales", CAFE_ALIAS),
];

fn alias_lab(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("captured BOM-less UTF-16LE response decodes")
    .text
}

/// `(carrying ledger, alias)` pairs, grouped by ledger and kept in Tally's order within each.
fn by_ledger<'a>(pairs: impl Iterator<Item = (&'a str, &'a str)>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = pairs
        .map(|(owner, alias)| (owner.to_string(), alias.to_string()))
        .collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

fn expected(extra: &[(&str, &str)]) -> Vec<(String, String)> {
    by_ledger(SCREEN_ALIASES.iter().chain(extra).copied())
}

/// The screen's aliases, read by the import catalogue (V2), each after its ledger's own name, in
/// keyed order, byte for byte.
#[test]
fn aliases_keyed_on_tallys_screen_are_read_as_tally_listed_them() {
    let v2 = parse_standard_ledger_catalog_v2_with_identities(
        &alias_lab(ALIAS_LAB_SCREEN_V2),
        ALIAS_LAB_COMPANY_NAME,
        ALIAS_LAB_COMPANY_GUID,
    )
    .expect("the captured import catalogue parses");
    assert_eq!(by_ledger(v2.catalog().aliases()), expected(&[]));
}

/// After two ledgers were created by import with an alias each, V1 and V2 read all six, the
/// Devanagari one by its exact code points.
#[test]
fn imported_aliases_are_read_by_both_catalogues_with_their_code_points() {
    let six = expected(&[
        ("Lab Alias Script", SCRIPT_ALIAS),
        ("Lab Alias Shape", "Lab Shape Alias"),
    ]);
    let v1 = parse_standard_ledger_catalog_with_identities(
        &alias_lab(ALIAS_LAB_V1),
        ALIAS_LAB_COMPANY_NAME,
        ALIAS_LAB_COMPANY_GUID,
    )
    .expect("the captured standard catalogue parses");
    assert_eq!(by_ledger(v1.aliases()), six);
    let v2 = parse_standard_ledger_catalog_v2_with_identities(
        &alias_lab(ALIAS_LAB_V2),
        ALIAS_LAB_COMPANY_NAME,
        ALIAS_LAB_COMPANY_GUID,
    )
    .expect("the captured import catalogue parses");
    assert_eq!(by_ledger(v2.catalog().aliases()), six);
    assert_eq!(CAFE_ALIAS.chars().filter(|c| *c == '\u{e9}').count(), 1);
}

/// The name index (V3) holds every alias among its ledger's names, after the own name.
#[test]
fn the_name_index_holds_each_alias_after_its_ledgers_own_name() {
    let rows = parse_ledger_name_index(&alias_lab(ALIAS_LAB_V3), ALIAS_LAB_COMPANY_GUID)
        .expect("the captured name index parses");
    assert_eq!(rows.len(), 7);
    for (owner, aliases) in [
        (
            "Lab Other Ledger",
            &["Lab Other First", "Lab Other Second"][..],
        ),
        ("Lab Plain Ledger", &["Lab Plain Alias"][..]),
        ("Lab Sales", &[CAFE_ALIAS][..]),
        ("Lab Alias Shape", &["Lab Shape Alias"][..]),
        ("Lab Alias Script", &[SCRIPT_ALIAS][..]),
    ] {
        let row = rows
            .iter()
            .find(|row| row.names.first().map(String::as_str) == Some(owner))
            .unwrap_or_else(|| panic!("no row named {owner}"));
        assert_eq!(&row.names[1..], aliases, "{owner}");
    }
}
