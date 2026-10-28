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

/// Committed group captures that carry the request-computed company GUID,
/// each with the GUID its request named.
const CAPTURES: [(&str, &[u8], &str); 5] = [
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
    // A difference of case alone is a difference.
    let cased = replace_once(
        &text,
        attribute,
        "<GROUP NAME=\"bank od a/c\" RESERVEDNAME=\"Bank OD A/c\">",
    );
    assert_eq!(
        parse(&cased),
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
    let cases: [(&str, String, NameShape); 8] = [
        (
            "two shapes in one row: the first in document order is named",
            format!(
                "<NAME.LIST/>{before_close}<NAME>Bare Name</NAME><NAME.LIST><ALIAS/></NAME.LIST>{close}"
            ),
            NameShape::NameListOutsideLanguageList,
        ),
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
    assert_eq!(
        parse(&stray),
        Err(GroupNameIndexError::UnrecognisedNameShape {
            row: row_of(&text, "Bank Accounts"),
            shape: NameShape::NameListOutsideLanguageList
        })
    );

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

/// Inserts `inserted` just before the closing tag of the row that opens with
/// `row_open`.
fn insert_in_row(text: &str, row_open: &str, inserted: &str) -> String {
    let start = text.find(row_open).expect("the capture holds the row");
    let close = start + text[start..].find("</GROUP>").unwrap();
    let mut changed = text.to_string();
    changed.insert_str(close, inserted);
    changed
}

/// A `GROUP` directly inside a row, self-closing or not, is a row no walk
/// reads; the index refuses it rather than stepping over it.
#[test]
fn a_group_directly_inside_a_row_is_refused() {
    let text = shape_lab();
    let snapshot = parse_native_group_snapshot(&text, SHAPE_GUID).unwrap();
    let row_open = "<GROUP NAME=\"Bank OD A/c\"";
    let row_number = row_of(&text, "Bank OD A/c");
    for inserted in [
        "<GROUP NAME=\"Nested\"/>",
        "<GROUP NAME=\"Nested\"></GROUP>",
        "<GROUP NAME=\"Nested\">text</GROUP>",
    ] {
        let changed = insert_in_row(&text, row_open, inserted);
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::HiddenInUnknownElement { row: row_number }),
            "{inserted}"
        );
        assert_eq!(
            parse_native_group_snapshot(&changed, SHAPE_GUID).as_ref(),
            Ok(&snapshot),
            "{inserted}"
        );
    }
}

/// A `NAME` that is a direct child of `GROUP`, bare or self-closing, holds a
/// name no `NAME.LIST` carries; the index refuses it instead of skipping it.
/// A second `GUID`, in either form, and a self-closing language list are the
/// shapes the existing arms already decide.
#[test]
fn a_name_directly_under_a_row_is_refused_and_the_other_direct_children_are_decided() {
    let text = shape_lab();
    let row_open = "<GROUP NAME=\"Bank OD A/c\"";
    let row_number = row_of(&text, "Bank OD A/c");
    for inserted in [
        "<NAME>Direct</NAME>",
        "<NAME></NAME>",
        "<NAME/>",
        "<NAME><Y/></NAME>",
    ] {
        assert_eq!(
            parse(&insert_in_row(&text, row_open, inserted)),
            Err(GroupNameIndexError::UnrecognisedNameShape {
                row: row_number,
                shape: NameShape::NameDirectlyUnderRow
            }),
            "{inserted}"
        );
    }
    for inserted in ["<GUID>second</GUID>", "<GUID/>"] {
        assert_eq!(
            parse(&insert_in_row(&text, row_open, inserted)),
            Err(GroupNameIndexError::GuidRepeatedInRow { row: row_number }),
            "{inserted}"
        );
    }
    for inserted in ["<NAME.LIST/>", "<NAME.LIST><NAME>Direct</NAME></NAME.LIST>"] {
        assert_eq!(
            parse(&insert_in_row(&text, row_open, inserted)),
            Err(GroupNameIndexError::UnrecognisedNameShape {
                row: row_number,
                shape: NameShape::NameListOutsideLanguageList
            }),
            "{inserted}"
        );
    }
    // A self-closing language list beside the real one adds no name.
    assert_eq!(
        parse(&insert_in_row(&text, row_open, "<LANGUAGENAME.LIST/>")),
        parse(&text)
    );
    // A second, real language list adds its names.
    let second = insert_in_row(
        &text,
        row_open,
        "<LANGUAGENAME.LIST><NAME.LIST><NAME>Second Language</NAME></NAME.LIST></LANGUAGENAME.LIST>",
    );
    let index = parse(&second).unwrap();
    let row = index.iter().find(|row| row.guid == BANK_OD_GUID).unwrap();
    assert_eq!(
        row.names,
        ["Bank OD A/c", "Bank OCC A/c", "Second Language"]
    );
}

/// The children the captures show as leaves are stepped over unread, so one
/// that holds an element is refused: a name could be nested in it.
#[test]
fn a_captured_leaf_child_that_holds_elements_is_refused() {
    let text = shape_lab();
    let row_open = "<GROUP NAME=\"Bank OD A/c\"";
    let row_number = row_of(&text, "Bank OD A/c");
    for inserted in [
        "<RESERVEDNAME><NAME>Hidden</NAME></RESERVEDNAME>",
        "<MASTERID><Y/></MASTERID>",
        "<PARENTSTRUCTURE>text<Y>1</Y></PARENTSTRUCTURE>",
    ] {
        assert_eq!(
            parse(&insert_in_row(&text, row_open, inserted)),
            Err(GroupNameIndexError::CapturedChildNotLeaf { row: row_number }),
            "{inserted}"
        );
    }
    // The same child in the language list.
    let span = language_list_span(&text, row_open);
    let at = span.start + text[span.clone()].find("<LANGUAGEID>").unwrap();
    let mut changed = text.clone();
    changed.insert_str(at + "<LANGUAGEID>".len(), "<Y/>");
    assert_eq!(
        parse(&changed),
        Err(GroupNameIndexError::CapturedChildNotLeaf { row: row_number })
    );
    // The snapshot readers do not mind.
    assert!(parse_native_group_snapshot(&changed, SHAPE_GUID).is_ok());
}

/// The parser rewrites a forbidden numeric character reference to U+FFFD, so
/// a name holding one is not the name Tally wrote.
#[test]
fn a_name_holding_a_forbidden_character_reference_is_refused() {
    let text = shape_lab();
    let alias_row = row_of(&text, "Bank OD A/c");
    let alias = replace_once(
        &text,
        "<NAME>Bank OCC A/c</NAME>",
        "<NAME>Bank &#4;OCC A/c</NAME>",
    );
    assert_eq!(
        parse(&alias),
        Err(GroupNameIndexError::NameNotRepresentable { row: alias_row })
    );
    let stored = replace_once(
        &text,
        "<NAME>Bank Accounts</NAME>",
        "<NAME>Bank &#4;Accounts</NAME>",
    );
    assert_eq!(
        parse(&stored),
        Err(GroupNameIndexError::NameNotRepresentable {
            row: row_of(&text, "Bank Accounts")
        })
    );
    // A legal reference is read as its character.
    let legal = replace_once(
        &text,
        "<NAME>Bank OCC A/c</NAME>",
        "<NAME>Bank &#65;OCC</NAME>",
    );
    let index = parse(&legal).unwrap();
    let row = index.iter().find(|row| row.guid == BANK_OD_GUID).unwrap();
    assert_eq!(row.names[1], "Bank AOCC");
}

/// The children the six captures show under a row and under its language list
/// are stepped over or read; any other child is searched for a name, an
/// identity or a row hidden inside it, and refused if one is there. The
/// snapshot parser reads none of these rows differently.
#[test]
fn a_name_or_identity_hidden_inside_an_unknown_element_is_refused() {
    let text = shape_lab();
    let snapshot = parse_native_group_snapshot(&text, SHAPE_GUID).unwrap();
    let row_number = row_of(&text, "Bank OD A/c");
    let row_open = "<GROUP NAME=\"Bank OD A/c\"";
    let row_start = text.find(row_open).unwrap();
    let row_close = row_start + text[row_start..].find("</GROUP>").unwrap();
    let span = language_list_span(&text, row_open);
    let language_list = text[span.clone()].to_string();
    let list_start = language_list.find("<NAME.LIST").unwrap();
    let list_end = language_list.find("</NAME.LIST>").unwrap() + "</NAME.LIST>".len();
    let list_close = language_list.rfind("</LANGUAGENAME.LIST>").unwrap();

    let in_row = |inserted: &str| {
        let mut changed = text.clone();
        changed.insert_str(row_close, inserted);
        changed
    };
    let in_language_list = |replacement: String| {
        let mut changed = text.clone();
        changed.replace_range(span.clone(), &replacement);
        changed
    };
    let hidden: [(&str, String); 11] = [
        (
            "the language list inside a wrapper",
            in_language_list(format!("<X>{language_list}</X>")),
        ),
        (
            "the name list inside a wrapper in the language list",
            in_language_list(format!(
                "{}<X>{}</X>{}",
                &language_list[..list_start],
                &language_list[list_start..list_end],
                &language_list[list_end..]
            )),
        ),
        (
            "a GUID inside a wrapper in the language list",
            in_language_list(format!(
                "{}<X><GUID>g</GUID></X>{}",
                &language_list[..list_close],
                &language_list[list_close..]
            )),
        ),
        (
            "a nested GUID",
            in_row("<X><GUID>3a6bd6e1-b835-4bff-89dd-8a6af138c346-ffffffff</GUID></X>"),
        ),
        (
            "a nested GROUP",
            in_row("<X><GROUP NAME=\"Hidden\"><PARENT>p</PARENT></GROUP></X>"),
        ),
        (
            "an empty language list inside a wrapper",
            in_row("<X><LANGUAGENAME.LIST/></X>"),
        ),
        (
            "a hidden NAME followed by a clean unknown element",
            in_row("<X><NAME/></X><Y></Y>"),
        ),
        (
            "a NAME after a sibling element closed",
            in_row("<X><Y></Y><NAME>Hidden</NAME></X>"),
        ),
        ("a nested NAME", in_row("<X><NAME>Hidden</NAME></X>")),
        (
            "a deeply nested empty NAME.LIST",
            in_row("<X><Y><Z><NAME.LIST/></Z></Y></X>"),
        ),
        ("a nested self-closing NAME", in_row("<X><NAME/></X>")),
    ];
    for (label, changed) in hidden {
        assert_eq!(
            parse(&changed),
            Err(GroupNameIndexError::HiddenInUnknownElement { row: row_number }),
            "{label}"
        );
    }

    // The same bytes read by the snapshot parser are not read differently.
    for changed in [
        in_row("<X><GUID>g</GUID></X>"),
        in_row("<X><GROUP NAME=\"Hidden\"><GUID>h</GUID></GROUP></X>"),
        in_language_list(format!("<X>{language_list}</X>")),
    ] {
        assert_eq!(
            parse_native_group_snapshot(&changed, SHAPE_GUID).as_ref(),
            Ok(&snapshot)
        );
    }

    // An unknown element with nothing hidden in it is not refused.
    let control = parse(&text).unwrap();
    assert_eq!(parse(&in_row("<X><Y>text</Y></X>")), Ok(control.clone()));
    assert_eq!(
        parse(&in_language_list(format!(
            "{}<FLAG><Y>1</Y></FLAG>{}",
            &language_list[..list_close],
            &language_list[list_close..]
        ))),
        Ok(control)
    );

    // Cut inside the unknown element, the search cannot finish. The snapshot
    // parser keeps its own code for the same bytes.
    let cut = format!("{}<X>", &text[..row_close]);
    assert_eq!(
        parse_native_group_snapshot(&cut, SHAPE_GUID),
        Err(NativeOutstandingsError::InvalidResponse(
            "ledger_subtree_unterminated"
        ))
    );
    assert_eq!(
        parse(&cut),
        Err(GroupNameIndexError::Response(
            NativeOutstandingsError::InvalidResponse("group_row_unterminated")
        ))
    );
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
    let other = format!("{}999", &SHAPE_GUID[..33]);
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
        parse_group_name_index(&text, &other),
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
