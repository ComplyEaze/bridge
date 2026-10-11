// SPDX-License-Identifier: Apache-2.0

//! The group name index, read from `List of Groups` snapshots captured live
//! from TallyPrime (provenance: `tests/fixtures/agent/shape-lab-group-chain.PROVENANCE.md`
//! for the Shape Lab pair; the other books are the committed group captures of
//! their own lab runs). Every positive case is a capture read as it stands; every
//! negative case is a capture with one element changed in the test, asserting
//! the typed variant that change must produce.
//!
//! Counts are taken by walking the XML structure here, never by counting
//! substrings: the `CMPINFO` block carries a bare `<GROUP>` counter that is
//! not a row.

use bridge_tally_protocol::native_outstandings::{
    parse_group_name_index, parse_native_group_snapshot, GroupNameIndexError, GroupNameRow,
    NativeOutstandingsError,
};
use bridge_tally_protocol::{decode_tally_xml_response_bytes_limited, ExpectedTallyTextEncoding};
use quick_xml::events::Event;
use quick_xml::Reader;

const SHAPE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";

const SHAPE_LAB: &[u8] = include_bytes!("fixtures/agent/native-shape-lab-groups.utf16le.xml");

/// Every committed group capture that carries the request-computed company
/// GUID, with the GUID its request named.
const CAPTURES: [(&str, &[u8], &str); 6] = [
    ("native-shape-lab-groups", SHAPE_LAB, SHAPE_GUID),
    (
        "shape-lab-group-snapshot",
        include_bytes!("fixtures/agent/shape-lab-group-snapshot.utf16le.xml"),
        SHAPE_GUID,
    ),
    (
        "native-ageing-groups",
        include_bytes!("fixtures/agent/native-ageing-groups.utf16le.xml"),
        "eebb9a9f-1679-4468-9e8f-814c729674cb",
    ),
    (
        "native-outstandings-lab-groups",
        include_bytes!("fixtures/agent/native-outstandings-lab-groups.utf16le.xml"),
        "49f1fbda-ee59-4a4b-aacf-b45fe32402d7",
    ),
    (
        "native-party-groups",
        include_bytes!("fixtures/agent/native-party-groups.utf16le.xml"),
        "61c6de69-1748-461c-ad3f-162cb949df9f",
    ),
    (
        "native-register-lab-groups",
        include_bytes!("fixtures/agent/native-register-lab-groups.utf16le.xml"),
        "ae1490be-52c5-4544-9ffc-4b7da85f9797",
    ),
];

fn decode(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the capture decodes")
    .text
}

fn shape_lab() -> String {
    decode(SHAPE_LAB)
}

/// `(GROUP rows under the collection, NAME elements under a row's
/// LANGUAGENAME.LIST/NAME.LIST)`, counted by walking the element path.
fn structure(text: &str) -> (usize, usize) {
    let mut reader = Reader::from_str(text);
    let mut path: Vec<String> = Vec::new();
    let (mut rows, mut names) = (0, 0);
    let collection = ["ENVELOPE", "BODY", "DATA", "COLLECTION"];
    let name_list = [
        "ENVELOPE",
        "BODY",
        "DATA",
        "COLLECTION",
        "GROUP",
        "LANGUAGENAME.LIST",
        "NAME.LIST",
    ];
    loop {
        match reader.read_event().expect("the capture is well formed") {
            Event::Start(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if path == collection && name == "GROUP" {
                    rows += 1;
                }
                if path == name_list && name == "NAME" {
                    names += 1;
                }
                path.push(name);
            }
            // A self-closing element is counted but has no matching `End`.
            Event::Empty(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if path == name_list && name == "NAME" {
                    names += 1;
                }
            }
            Event::End(_) => {
                path.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    (rows, names)
}

fn parse(text: &str) -> Result<Vec<GroupNameRow>, GroupNameIndexError> {
    parse_group_name_index(text, SHAPE_GUID)
}

/// Replaces the one occurrence of `from`, refusing to run a mutation whose
/// target is not unique.
fn replace_once(text: &str, from: &str, to: &str) -> String {
    assert_eq!(
        text.matches(from).count(),
        1,
        "the mutation target must be unique: {from}"
    );
    text.replacen(from, to, 1)
}

/// The byte span of the first `LANGUAGENAME.LIST` of the row that opens with
/// `row_open`, found by position so the capture's line endings do not matter.
fn language_list_span(text: &str, row_open: &str) -> std::ops::Range<usize> {
    let row = text.find(row_open).expect("the capture holds the row");
    let start = row + text[row..].find("<LANGUAGENAME.LIST>").unwrap();
    let close = "</LANGUAGENAME.LIST>";
    let end = start + text[start..].find(close).unwrap() + close.len();
    start..end
}

/// The 1-based document position of the row whose stored name is `name`.
fn row_of(text: &str, name: &str) -> usize {
    parse_native_group_snapshot(text, SHAPE_GUID)
        .expect("the capture parses")
        .iter()
        .position(|group| group.name == name)
        .map(|index| index + 1)
        .expect("the capture holds the row")
}

const BANK_ACCOUNTS_GUID: &str =
    "<GUID TYPE=\"String\">3a6bd6e1-b835-4bff-89dd-8a6af138c346-00000016</GUID>";
const BANK_ACCOUNTS_ALTER_ID: &str = "<ALTERID TYPE=\"Number\"> 23</ALTERID>";
const BANK_OD_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346-0000000b";

#[test]
fn every_capture_yields_one_index_row_per_group_and_every_name() {
    for (label, bytes, guid) in CAPTURES {
        let text = decode(bytes);
        let (rows, names) = structure(&text);
        let index =
            parse_group_name_index(&text, guid).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert!(rows > 0, "{label}");
        assert_eq!(index.len(), rows, "{label}: one row per GROUP");
        assert_eq!(
            index.iter().map(|row| row.names.len()).sum::<usize>(),
            names,
            "{label}: every NAME of every NAME.LIST"
        );
        // The rows are the existing snapshot's rows, in the same order, and
        // the stored name is the first name.
        let snapshot = parse_native_group_snapshot(&text, guid).unwrap();
        assert_eq!(snapshot.len(), index.len(), "{label}");
        for (row, group) in index.iter().zip(&snapshot) {
            assert_eq!(row.names[0], group.name, "{label}");
            assert_eq!(
                row.parent.as_deref(),
                group.parent.returned_text().filter(|text| !text.is_empty()),
                "{label}: {}",
                group.name
            );
        }
        // Identity is present and unique on every row.
        let mut guids: Vec<&str> = index.iter().map(|row| row.guid.as_str()).collect();
        guids.sort_unstable();
        guids.dedup();
        assert_eq!(guids.len(), index.len(), "{label}");
        assert!(index.iter().all(|row| !row.guid.is_empty()), "{label}");
    }
}

#[test]
fn the_shape_lab_alias_row_keeps_its_stored_name_first_and_its_alias_after() {
    let text = shape_lab();
    let index = parse(&text).unwrap();
    let row = index
        .iter()
        .find(|row| row.guid == BANK_OD_GUID)
        .expect("Bank OD A/c is in the capture");
    assert_eq!(row.names, ["Bank OD A/c", "Bank OCC A/c"]);
    assert_eq!(row.alter_id, 12);
    assert_eq!(row.parent.as_deref(), Some("Loans (Liability)"));
    // The plain row beside it carries one name and its own numbers.
    let bank = index
        .iter()
        .find(|row| row.guid.ends_with("-00000016"))
        .unwrap();
    assert_eq!(bank.names, ["Bank Accounts"]);
    assert_eq!(bank.alter_id, 23);
    assert_eq!(bank.parent.as_deref(), Some("Current Assets"));
    // The six alias rows of this book, read from the capture with a second
    // XML library: each stored name, then its alias.
    let aliased: Vec<(&[String], u64)> = index
        .iter()
        .filter(|row| row.names.len() > 1)
        .map(|row| (row.names.as_slice(), row.alter_id))
        .collect();
    let expected: [([&str; 2], u64); 6] = [
        (["Bank OD A/c", "Bank OCC A/c"], 12),
        (["Direct Expenses", "Expenses (Direct)"], 27),
        (["Direct Incomes", "Income (Direct)"], 26),
        (["Indirect Expenses", "Expenses (Indirect)"], 29),
        (["Indirect Incomes", "Income (Indirect)"], 28),
        (["Reserves & Surplus", "Retained Earnings"], 11),
    ];
    assert_eq!(aliased.len(), expected.len());
    for (row, (names, alter_id)) in aliased.iter().zip(expected) {
        assert_eq!(row.0, names);
        assert_eq!(row.1, alter_id);
    }
}

/// The `NAME` attribute can differ from the stored name; the index reads the
/// names only from `NAME.LIST`.
#[test]
fn names_come_from_the_name_list_never_the_name_attribute() {
    let text = shape_lab();
    let changed = replace_once(
        &text,
        "<GROUP NAME=\"Bank OD A/c\" RESERVEDNAME=\"Bank OD A/c\">",
        "<GROUP NAME=\"Attribute Only\" RESERVEDNAME=\"Bank OD A/c\">",
    );
    let index = parse(&changed).unwrap();
    let row = index.iter().find(|row| row.guid == BANK_OD_GUID).unwrap();
    assert_eq!(row.names, ["Bank OD A/c", "Bank OCC A/c"]);
    assert!(index
        .iter()
        .all(|row| !row.names.iter().any(|name| name == "Attribute Only")));
}

#[test]
fn every_name_list_of_every_language_list_is_read_in_document_order() {
    let text = shape_lab();
    // A second NAME.LIST in the same language list, then a second language
    // list, then a stray NAME.LIST outside any language list.
    let span = language_list_span(&text, "<GROUP NAME=\"Bank OD A/c\"");
    let original = &text[span.clone()];
    let close = "</LANGUAGENAME.LIST>";
    let mut rebuilt = original[..original.len() - close.len()].to_string();
    rebuilt.push_str(
        "<NAME.LIST TYPE=\"String\"><NAME>Second List</NAME><NAME>Second List B</NAME></NAME.LIST>",
    );
    rebuilt.push_str(close);
    rebuilt.push_str("<LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\"><NAME>Other Language</NAME></NAME.LIST><LANGUAGEID> 1081</LANGUAGEID></LANGUAGENAME.LIST>");
    rebuilt.push_str("<NAME.LIST TYPE=\"String\"><NAME>Stray Outside</NAME></NAME.LIST>");
    let mut changed = text.clone();
    changed.replace_range(span, &rebuilt);
    let index = parse(&changed).unwrap();
    let row = index.iter().find(|row| row.guid == BANK_OD_GUID).unwrap();
    assert_eq!(
        row.names,
        [
            "Bank OD A/c",
            "Bank OCC A/c",
            "Second List",
            "Second List B",
            "Other Language"
        ]
    );
    let (rows, names) = structure(&changed);
    assert_eq!(index.len(), rows);
    assert_eq!(
        index.iter().map(|row| row.names.len()).sum::<usize>(),
        names
    );
}

#[test]
fn a_missing_or_empty_guid_is_refused_with_its_row() {
    let text = shape_lab();
    let first = row_of(&text, "Bank Accounts");
    for replacement in [
        "",
        "<GUID TYPE=\"String\"></GUID>",
        "<GUID TYPE=\"String\"> </GUID>",
        "<GUID/>",
    ] {
        let changed = replace_once(&text, BANK_ACCOUNTS_GUID, replacement);
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::GuidMissing { row: first }),
            "{replacement:?}"
        );
    }
}

#[test]
fn a_guid_repeated_inside_one_row_is_refused() {
    let text = shape_lab();
    let changed = replace_once(
        &text,
        BANK_ACCOUNTS_GUID,
        &format!("{BANK_ACCOUNTS_GUID}{BANK_ACCOUNTS_GUID}"),
    );
    assert_eq!(
        parse(&changed),
        Err(GroupNameIndexError::GuidRepeatedInRow {
            row: row_of(&text, "Bank Accounts")
        })
    );
}

#[test]
fn a_guid_shared_by_two_rows_is_refused_naming_both() {
    let text = shape_lab();
    let first = row_of(&text, "Bank Accounts");
    let second = row_of(&text, "Bank OD A/c");
    assert!(first < second);
    let changed = replace_once(
        &text,
        &format!("<GUID TYPE=\"String\">{BANK_OD_GUID}</GUID>"),
        BANK_ACCOUNTS_GUID,
    );
    assert_eq!(
        parse(&changed),
        Err(GroupNameIndexError::DuplicateGuid {
            first_row: first,
            row: second
        })
    );
    // A GUID that differs only by case is the same GUID.
    let cased = replace_once(
        &text,
        &format!("<GUID TYPE=\"String\">{BANK_OD_GUID}</GUID>"),
        &BANK_ACCOUNTS_GUID
            .to_ascii_uppercase()
            .replace("GUID TYPE=\"STRING\"", "GUID TYPE=\"String\""),
    );
    assert_eq!(
        parse(&cased),
        Err(GroupNameIndexError::DuplicateGuid {
            first_row: first,
            row: second
        })
    );
}

#[test]
fn a_whole_row_repeated_is_refused_as_a_duplicate_guid() {
    let text = shape_lab();
    let start = text.find("<GROUP NAME=\"Bank OD A/c\"").unwrap();
    let end = start + text[start..].find("</GROUP>").unwrap() + "</GROUP>".len();
    let mut repeated = text.clone();
    repeated.insert_str(end, &text[start..end]);
    let (rows, _) = structure(&repeated);
    assert_eq!(rows, structure(&text).0 + 1);
    let original = row_of(&text, "Bank OD A/c");
    assert_eq!(
        parse(&repeated),
        Err(GroupNameIndexError::DuplicateGuid {
            first_row: original,
            row: original + 1
        })
    );
}

#[test]
fn a_missing_or_empty_alter_id_is_refused_with_its_row() {
    let text = shape_lab();
    let first = row_of(&text, "Bank Accounts");
    for replacement in [
        "",
        "<ALTERID TYPE=\"Number\"></ALTERID>",
        "<ALTERID TYPE=\"Number\"> </ALTERID>",
        "<ALTERID/>",
    ] {
        let changed = replace_once(&text, BANK_ACCOUNTS_ALTER_ID, replacement);
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::AlterIdMissing { row: first }),
            "{replacement:?}"
        );
    }
}

#[test]
fn a_non_numeric_alter_id_is_refused_with_its_row() {
    let text = shape_lab();
    let first = row_of(&text, "Bank Accounts");
    for value in [
        "x23",
        "-23",
        "+23",
        "2.3",
        "2 3",
        "NaN",
        "99999999999999999999999",
    ] {
        let changed = replace_once(
            &text,
            BANK_ACCOUNTS_ALTER_ID,
            &format!("<ALTERID TYPE=\"Number\"> {value}</ALTERID>"),
        );
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::AlterIdNotNumeric { row: first }),
            "{value:?}"
        );
    }
    // The captured value, spaced as Tally writes it, is the control.
    assert!(parse(&text).is_ok());
}

#[test]
fn an_alter_id_repeated_inside_one_row_is_refused() {
    let text = shape_lab();
    let changed = replace_once(
        &text,
        BANK_ACCOUNTS_ALTER_ID,
        &format!("{BANK_ACCOUNTS_ALTER_ID}{BANK_ACCOUNTS_ALTER_ID}"),
    );
    assert_eq!(
        parse(&changed),
        Err(GroupNameIndexError::AlterIdRepeatedInRow {
            row: row_of(&text, "Bank Accounts")
        })
    );
}

#[test]
fn a_row_with_no_names_is_refused() {
    let text = shape_lab();
    let first = row_of(&text, "Bank Accounts");
    let span = language_list_span(&text, "<GROUP NAME=\"Bank Accounts\"");
    for replacement in [
        // No language list at all.
        "",
        // A language list with no NAME.LIST.
        "<LANGUAGENAME.LIST><LANGUAGEID> 1033</LANGUAGEID></LANGUAGENAME.LIST>",
        // A NAME.LIST with no NAME.
        "<LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\"></NAME.LIST></LANGUAGENAME.LIST>",
        // A NAME.LIST outside any language list does not count.
        "<NAME.LIST TYPE=\"String\"><NAME>Bank Accounts</NAME></NAME.LIST>",
    ] {
        let mut changed = text.clone();
        changed.replace_range(span.clone(), replacement);
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::NoNames { row: first }),
            "{replacement:?}"
        );
    }
}

#[test]
fn an_empty_name_is_refused_with_its_row_whether_stored_or_alias() {
    let text = shape_lab();
    let first = row_of(&text, "Bank Accounts");
    for replacement in ["<NAME></NAME>", "<NAME/>", "<NAME>  </NAME>"] {
        let changed = replace_once(&text, "<NAME>Bank Accounts</NAME>", replacement);
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::EmptyName { row: first }),
            "stored {replacement:?}"
        );
    }
    let alias_row = row_of(&text, "Bank OD A/c");
    for replacement in ["<NAME></NAME>", "<NAME/>"] {
        let changed = replace_once(&text, "<NAME>Bank OCC A/c</NAME>", replacement);
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::EmptyName { row: alias_row }),
            "alias {replacement:?}"
        );
    }
}

#[test]
fn the_response_company_guid_must_be_present_and_the_expected_one() {
    let text = shape_lab();
    let element = format!("<BRIDGECOMPANYGUID TYPE=\"String\">{SHAPE_GUID}</BRIDGECOMPANYGUID>");
    // Absent from the first row.
    let absent = text.replacen(&element, "", 1);
    assert_ne!(absent, text);
    assert_eq!(parse(&absent), Err(GroupNameIndexError::CompanyGuidMissing));
    // Empty on the first row.
    let empty = text.replacen(
        &element,
        "<BRIDGECOMPANYGUID TYPE=\"String\"></BRIDGECOMPANYGUID>",
        1,
    );
    assert_eq!(parse(&empty), Err(GroupNameIndexError::CompanyGuidMissing));
    // A different company on one row only.
    let other = "3a6bd6e1-b835-4bff-89dd-8a6af138c999";
    let swapped = text.replacen(
        &element,
        &format!("<BRIDGECOMPANYGUID TYPE=\"String\">{other}</BRIDGECOMPANYGUID>"),
        1,
    );
    assert_ne!(swapped, text);
    assert_eq!(
        parse(&swapped),
        Err(GroupNameIndexError::CompanyGuidMismatch)
    );
    // The caller expecting another company.
    assert_eq!(
        parse_group_name_index(&text, other),
        Err(GroupNameIndexError::CompanyGuidMismatch)
    );
    // Case alone does not make a different company.
    assert!(parse_group_name_index(&text, &SHAPE_GUID.to_ascii_uppercase()).is_ok());
}

#[test]
fn a_response_that_is_not_a_usable_group_answer_is_a_response_error() {
    let text = shape_lab();
    let cut = &text[..text.len() / 2];
    assert!(matches!(
        parse(cut),
        Err(GroupNameIndexError::Response(
            NativeOutstandingsError::InvalidResponse(_)
        ))
    ));
    let failed = replace_once(&text, "<STATUS>1</STATUS>", "<STATUS>0</STATUS>");
    assert_eq!(
        parse(&failed),
        Err(GroupNameIndexError::Response(
            NativeOutstandingsError::TallyReportedFailure
        ))
    );
    assert!(matches!(parse(""), Err(GroupNameIndexError::Response(_))));
}
