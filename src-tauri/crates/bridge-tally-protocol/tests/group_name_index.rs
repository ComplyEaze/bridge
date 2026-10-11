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
    NameShape, NativeOutstandingsError,
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

/// The names come from `NAME.LIST`, and the row's `NAME=` attribute must agree
/// with the stored (first) name. No capture shows them differing, so a
/// disagreement is a shape the index refuses rather than resolves.
#[test]
fn names_come_from_the_name_list_and_the_name_attribute_must_agree() {
    let text = shape_lab();
    let row_number = row_of(&text, "Bank OD A/c");
    let attribute = "<GROUP NAME=\"Bank OD A/c\" RESERVEDNAME=\"Bank OD A/c\">";
    // The attribute alone changed.
    let attribute_only = replace_once(
        &text,
        attribute,
        "<GROUP NAME=\"Attribute Only\" RESERVEDNAME=\"Bank OD A/c\">",
    );
    assert_eq!(
        parse(&attribute_only),
        Err(GroupNameIndexError::NameAttributeDisagrees { row: row_number })
    );
    // The stored name alone changed.
    let list_only = replace_once(&text, "<NAME>Bank OD A/c</NAME>", "<NAME>List Only</NAME>");
    assert_eq!(
        parse(&list_only),
        Err(GroupNameIndexError::NameAttributeDisagrees { row: row_number })
    );
    // An alias is not compared with the attribute.
    let alias_changed = replace_once(
        &text,
        "<NAME>Bank OCC A/c</NAME>",
        "<NAME>Other Alias</NAME>",
    );
    let row = parse(&alias_changed).unwrap();
    assert_eq!(
        row.iter()
            .find(|row| row.guid == BANK_OD_GUID)
            .unwrap()
            .names,
        ["Bank OD A/c", "Other Alias"]
    );
    // Both changed together agree, and the name read is the list's.
    let both = replace_once(
        &list_only,
        attribute,
        "<GROUP NAME=\"List Only\" RESERVEDNAME=\"Bank OD A/c\">",
    );
    let index = parse(&both).unwrap();
    let row = index.iter().find(|row| row.guid == BANK_OD_GUID).unwrap();
    assert_eq!(row.names, ["List Only", "Bank OCC A/c"]);
}

#[test]
fn every_name_list_of_every_language_list_is_read_in_document_order() {
    let text = shape_lab();
    // A second NAME.LIST in the same language list, then a second language
    // list.
    let span = language_list_span(&text, "<GROUP NAME=\"Bank OD A/c\"");
    let original = &text[span.clone()];
    let close = "</LANGUAGENAME.LIST>";
    let mut rebuilt = original[..original.len() - close.len()].to_string();
    rebuilt.push_str(
        "<NAME.LIST TYPE=\"String\"><NAME>Second List</NAME><NAME>Second List B</NAME></NAME.LIST>",
    );
    rebuilt.push_str(close);
    rebuilt.push_str("<LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\"><NAME>Other Language</NAME></NAME.LIST><LANGUAGEID> 1081</LANGUAGEID></LANGUAGENAME.LIST>");
    let mut changed = text.clone();
    changed.replace_range(span.clone(), &rebuilt);
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

/// A name the walk would otherwise step over is refused with its row, never
/// dropped: a `NAME.LIST` outside any language list, a `NAME` outside any
/// `NAME.LIST`, and anything but a `NAME` inside a `NAME.LIST`.
#[test]
fn a_name_in_a_shape_no_capture_shows_is_refused_not_dropped() {
    let text = shape_lab();
    let row_number = row_of(&text, "Bank OD A/c");
    let span = language_list_span(&text, "<GROUP NAME=\"Bank OD A/c\"");
    let original = text[span.clone()].to_string();
    let close = "</LANGUAGENAME.LIST>";
    let before_close = original[..original.len() - close.len()].to_string();
    let cases: [(&str, String, NameShape); 7] = [
        (
            "NAME.LIST beside the language list",
            format!("{original}<NAME.LIST TYPE=\"String\"><NAME>Stray Outside</NAME></NAME.LIST>"),
            NameShape::NameListOutsideLanguageList,
        ),
        (
            "NAME.LIST before the language list",
            format!("<NAME.LIST TYPE=\"String\"><NAME>Stray Outside</NAME></NAME.LIST>{original}"),
            NameShape::NameListOutsideLanguageList,
        ),
        (
            "empty NAME.LIST beside the language list",
            format!("{original}<NAME.LIST/>"),
            NameShape::NameListOutsideLanguageList,
        ),
        (
            "NAME directly under the language list",
            format!("{before_close}<NAME>Bare Name</NAME>{close}"),
            NameShape::NameDirectlyUnderLanguageList,
        ),
        (
            "empty NAME directly under the language list",
            format!("{before_close}<NAME/>{close}"),
            NameShape::NameDirectlyUnderLanguageList,
        ),
        (
            "other element inside a NAME.LIST",
            original.replace("</NAME.LIST>", "<ALIAS>Not A Name</ALIAS></NAME.LIST>"),
            NameShape::ElementOtherThanNameInNameList,
        ),
        (
            "empty other element inside a NAME.LIST",
            original.replace("</NAME.LIST>", "<ALIAS/></NAME.LIST>"),
            NameShape::ElementOtherThanNameInNameList,
        ),
    ];
    for (label, replacement, shape) in cases {
        assert_ne!(replacement, original, "{label}");
        let mut changed = text.clone();
        changed.replace_range(span.clone(), &replacement);
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::UnrecognisedNameShape {
                row: row_number,
                shape
            }),
            "{label}"
        );
    }
}

/// The index is the strict reader; the snapshot parser keeps reading these
/// rows exactly as it did. A `GUID` with an invalid escape is read around by
/// the snapshot and refused by the index, and a cut inside a language list
/// keeps the snapshot's own error code.
#[test]
fn the_snapshot_parser_keeps_its_results_and_codes_where_the_index_is_stricter() {
    let text = shape_lab();
    let snapshot = parse_native_group_snapshot(&text, SHAPE_GUID).unwrap();

    let bogus = replace_once(&text, BANK_ACCOUNTS_GUID, "<GUID>&bogus;</GUID>");
    assert_eq!(
        parse_native_group_snapshot(&bogus, SHAPE_GUID).as_ref(),
        Ok(&snapshot),
        "the snapshot does not read the GUID"
    );
    assert_eq!(
        parse(&bogus),
        Err(GroupNameIndexError::Response(
            NativeOutstandingsError::InvalidResponse("native_xml_invalid_escape")
        ))
    );

    let span = language_list_span(&text, "<GROUP NAME=\"Bank Accounts\"");
    let cut = &text[..span.start + "<LANGUAGENAME.LIST>".len()];
    assert_eq!(
        parse_native_group_snapshot(cut, SHAPE_GUID),
        Err(NativeOutstandingsError::InvalidResponse(
            "ledger_subtree_unterminated"
        ))
    );
    assert_eq!(
        parse(cut),
        Err(GroupNameIndexError::Response(
            NativeOutstandingsError::InvalidResponse("group_row_unterminated")
        ))
    );

    // Stray name shapes and a row moved out of the collection are the index's
    // refusals only.
    let stray = {
        let mut changed = text.clone();
        changed.insert_str(span.end, "<NAME.LIST><NAME>Stray</NAME></NAME.LIST>");
        changed
    };
    assert_eq!(
        parse_native_group_snapshot(&stray, SHAPE_GUID).as_ref(),
        Ok(&snapshot)
    );
    assert!(parse(&stray).is_err());

    // An empty collection keeps the snapshot's code.
    let empty = without_rows(&text);
    assert_eq!(
        parse_native_group_snapshot(&empty, SHAPE_GUID),
        Err(NativeOutstandingsError::InvalidResponse(
            "group_response_company_guid_missing"
        ))
    );
}

/// The capture with every `GROUP` row removed from the collection.
fn without_rows(text: &str) -> String {
    let start = text.find("<GROUP NAME=").unwrap();
    let end = text.rfind("</GROUP>").unwrap() + "</GROUP>".len();
    format!("{}{}", &text[..start], &text[end..])
}

#[test]
fn a_collection_with_no_rows_is_its_own_refusal() {
    let text = shape_lab();
    let empty = without_rows(&text);
    assert_eq!(structure(&empty).0, 0);
    assert_eq!(parse(&empty), Err(GroupNameIndexError::EmptyCollection));
    // An unrelated empty collection is not a company mismatch either.
    assert_eq!(
        parse_group_name_index(&empty, "someone-else"),
        Err(GroupNameIndexError::EmptyCollection)
    );
}

/// A body cut after a row, whether the first or the last, is unterminated: the
/// rows read so far must not be returned as the whole collection.
#[test]
fn a_body_cut_after_a_complete_row_is_refused_as_unterminated() {
    let text = shape_lab();
    let after_first = text.find("</GROUP>").unwrap() + "</GROUP>".len();
    let after_last = text.rfind("</GROUP>").unwrap() + "</GROUP>".len();
    assert!(after_first < after_last);
    for (label, end) in [("first", after_first), ("last", after_last)] {
        assert_eq!(
            parse(&text[..end]),
            Err(GroupNameIndexError::Response(
                NativeOutstandingsError::InvalidResponse("group_envelope_unterminated")
            )),
            "cut after the {label} row"
        );
    }
}

/// A `GROUP` that is not under `ENVELOPE/BODY/DATA/COLLECTION` would be
/// stepped over by the row walk, leaving fewer rows than Tally sent; the index
/// refuses it. The bare `CMPINFO` counter is the one `GROUP` allowed elsewhere.
#[test]
fn a_group_outside_the_collection_is_refused_not_skipped() {
    let text = shape_lab();
    let start = text.find("<GROUP NAME=\"Bank OD A/c\"").unwrap();
    let end = start + text[start..].find("</GROUP>").unwrap() + "</GROUP>".len();
    let row = text[start..end].to_string();
    let mut without = text.clone();
    without.replace_range(start..end, "");
    let (rows, _) = structure(&text);
    assert_eq!(structure(&without).0, rows - 1);

    let placements: [(&str, &str, String); 4] = [
        (
            "DATA, after the collection",
            "</COLLECTION>",
            format!("</COLLECTION>{row}"),
        ),
        (
            "DESC, after the counters",
            "</CMPINFO>",
            format!("</CMPINFO>{row}"),
        ),
        ("BODY, after DATA", "</DATA>", format!("</DATA>{row}")),
        (
            "self-closing, in DATA",
            "</COLLECTION>",
            "</COLLECTION><GROUP/>".to_string(),
        ),
    ];
    for (label, anchor, replacement) in placements {
        let moved = replace_once(&without, anchor, &replacement);
        assert_eq!(
            parse(&moved),
            Err(GroupNameIndexError::GroupOutsideCollection),
            "{label}"
        );
    }
    // The original, with its CMPINFO counter, is the control.
    assert!(parse(&text).is_ok());
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
        // An empty NAME.LIST.
        "<LANGUAGENAME.LIST><NAME.LIST/></LANGUAGENAME.LIST>",
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
    // Cut on a character boundary at about the middle.
    let middle = (text.len() / 2..=text.len())
        .find(|&at| text.is_char_boundary(at))
        .unwrap();
    let cut = &text[..middle];
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
