// SPDX-License-Identifier: Apache-2.0

//! The ledger name index, read from V3 `List of Ledgers` answers captured live
//! from TallyPrime on a synthetic book (provenance:
//! `tests/fixtures/agent/masters-lab.PROVENANCE.md`). Every positive case is a
//! capture read as it stands; every negative case is a capture with one element
//! changed in the test, asserting the typed variant that change must produce.
//!
//! Rows are counted by walking the XML structure here, never by counting
//! substrings: the `CMPINFO` block carries a bare `<LEDGER>` counter that is
//! not a row.

use bridge_tally_protocol::native_outstandings::render_native_group_snapshot_request;
use bridge_tally_protocol::xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, encode_tally_xml_request_utf16le,
    parse_ledger_name_index, ExpectedTallyTextEncoding, LedgerNameRow, StandardLedgerCatalogError,
};
use quick_xml::events::Event;
use quick_xml::Reader;

const COMPANY: &str = "BRIDGE MASTERS LAB";
const GUID: &str = "f107e523-3d6b-4516-bd24-5b1a14498fa6";

const BASELINE: &[u8] =
    include_bytes!("fixtures/agent/masters-lab-ledger-index-baseline.utf16le.xml");
const AFTER_NAME_CASES: &[u8] =
    include_bytes!("fixtures/agent/masters-lab-ledger-index-after-name-cases.utf16le.xml");
const AFTER_FOLD_CASES: &[u8] =
    include_bytes!("fixtures/agent/masters-lab-ledger-index-after-fold-cases.utf16le.xml");

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

fn parsed(bytes: &[u8]) -> Vec<LedgerNameRow> {
    parse_ledger_name_index(&decode(bytes), GUID).expect("the capture parses")
}

fn refusal(xml: &str) -> StandardLedgerCatalogError {
    parse_ledger_name_index(xml, GUID).expect_err("the changed capture is refused")
}

/// The rows under `ENVELOPE/BODY/DATA/COLLECTION`, counted by structure.
fn collection_rows(xml: &str) -> usize {
    let mut reader = Reader::from_str(xml);
    let mut path: Vec<Vec<u8>> = Vec::new();
    let mut rows = 0;
    loop {
        match reader.read_event().expect("the capture is XML") {
            Event::Start(element) => {
                if path
                    == [
                        b"ENVELOPE".to_vec(),
                        b"BODY".to_vec(),
                        b"DATA".to_vec(),
                        b"COLLECTION".to_vec(),
                    ]
                    && element.name().as_ref() == "LEDGER"
                {
                    rows += 1;
                }
                path.push(element.name().as_ref().as_bytes().to_vec());
            }
            Event::End(_) => {
                path.pop();
            }
            Event::Eof => return rows,
            _ => {}
        }
    }
}

fn row<'a>(rows: &'a [LedgerNameRow], name: &str) -> &'a LedgerNameRow {
    let found = rows
        .iter()
        .filter(|row| row.names[0] == name)
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "one row stores {name}");
    found[0]
}

/// Replaces exactly one occurrence of `from` in the decoded capture.
fn changed(bytes: &[u8], from: &str, to: &str) -> String {
    let xml = decode(bytes);
    assert_eq!(xml.matches(from).count(), 1, "the change names one place");
    xml.replacen(from, to, 1)
}

#[test]
fn the_requests_sent_in_the_lab_are_the_ones_bridge_renders() {
    let company = ValidatedCompanyName::new(COMPANY).unwrap();
    assert_eq!(
        encode_tally_xml_request_utf16le(
            &ReadOnlyProfile::StandardLedgerCatalogV3 { company: &company }.render()
        ),
        include_bytes!("fixtures/agent/masters-lab-ledger-index.request.utf16le.xml")
    );
    assert_eq!(
        encode_tally_xml_request_utf16le(&render_native_group_snapshot_request(COMPANY)),
        include_bytes!("fixtures/agent/masters-lab-groups.request.utf16le.xml")
    );
}

#[test]
fn every_row_of_the_baseline_is_read_with_its_names_and_alter_id() {
    let xml = decode(BASELINE);
    let rows = parsed(BASELINE);
    assert_eq!(rows.len(), collection_rows(&xml));
    assert_eq!(rows.len(), 8);
    assert!(rows.iter().all(|row| row.row_spelling.is_none()));
    let alias = row(&rows, "MLab Alias Target");
    assert_eq!(
        alias.names,
        ["MLab Alias Target", "MLab Alias Name", "MLab Alias Two"]
    );
    assert_eq!(alias.guid, format!("{GUID}-000000cf"));
    assert_eq!(alias.alter_id, 211);
    assert_eq!(row(&rows, "MLab Dash-Target").names, ["MLab Dash-Target"]);
}

/// After the near-name Creates, four ledgers store the incoming spelling while
/// their row keeps the old one; both stay names of the one ledger.
#[test]
fn a_create_folded_onto_a_ledger_leaves_its_old_spelling_as_the_row_spelling() {
    let xml = decode(AFTER_NAME_CASES);
    let rows = parsed(AFTER_NAME_CASES);
    assert_eq!(rows.len(), collection_rows(&xml));
    assert_eq!(rows.len(), 10);
    for (stored, old) in [
        ("MLAB CASE TARGET", "MLab Case Target"),
        ("MLab  Space Target", "MLab Space Target"),
        ("MLab Dash Target", "MLab Dash-Target"),
        ("MLab Slash Target", "MLab Slash/Target"),
    ] {
        let ledger = row(&rows, stored);
        assert_eq!(ledger.names, [stored]);
        assert_eq!(ledger.row_spelling.as_deref(), Some(old));
    }
    let with_spelling = rows.iter().filter(|row| row.row_spelling.is_some()).count();
    assert_eq!(with_spelling, 4);
    // The ledger created with a group's name is an ordinary ledger row.
    assert!(row(&rows, "MLab Group Target").row_spelling.is_none());
}

#[test]
fn another_companys_guid_is_refused() {
    // Another synthetic lab company's GUID (BRIDGE TIEOUT B, in the merge gate's registry).
    assert_eq!(
        parse_ledger_name_index(&decode(BASELINE), "d64eda55-da7c-4106-a171-3e3cb6cdd331"),
        Err(StandardLedgerCatalogError::CompanyIdentityMismatch)
    );
}

#[test]
fn a_row_without_a_usable_alter_id_is_refused() {
    let one = "<ALTERID TYPE=\"Number\"> 211</ALTERID>";
    for to in [
        "",
        "<ALTERID TYPE=\"Number\"></ALTERID>",
        "<ALTERID TYPE=\"Number\"/>",
        "<ALTERID TYPE=\"Number\">-211</ALTERID>",
        "<ALTERID TYPE=\"Number\"> 211</ALTERID><ALTERID TYPE=\"Number\"> 211</ALTERID>",
    ] {
        assert_eq!(
            refusal(&changed(BASELINE, one, to)),
            StandardLedgerCatalogError::AlterIdInvalid,
            "{to}"
        );
    }
}

#[test]
fn an_empty_or_blank_name_or_a_second_name_list_is_refused_not_skipped() {
    let alias = "<NAME>MLab Alias Two</NAME>";
    for to in ["<NAME></NAME>", "<NAME/>", "<NAME> </NAME>"] {
        assert_eq!(
            refusal(&changed(BASELINE, alias, to)),
            StandardLedgerCatalogError::LedgerNameUnusable,
            "{to}"
        );
    }
    // A second NAME.LIST in one LANGUAGENAME.LIST.
    let list_end = "<NAME>MLab Alias Two</NAME>";
    assert_eq!(
        refusal(&changed(
            BASELINE,
            list_end,
            "<NAME>MLab Alias Two</NAME></NAME.LIST><NAME.LIST TYPE=\"String\"><NAME>MLab Alias Three</NAME>"
        )),
        StandardLedgerCatalogError::LedgerNameUnusable
    );
}

#[test]
fn anything_but_names_inside_a_name_list_is_refused() {
    assert_eq!(
        refusal(&changed(
            BASELINE,
            "<NAME>MLab Alias Two</NAME>",
            "<NAME>MLab Alias Two</NAME><REMOTEALTNAME>x</REMOTEALTNAME>"
        )),
        StandardLedgerCatalogError::MalformedResponse
    );
}

#[test]
fn a_row_without_a_language_list_is_refused() {
    let xml = decode(BASELINE);
    let start = xml
        .find("<LEDGER NAME=\"MLab Alias Target\"")
        .expect("the row");
    let list = start + xml[start..].find("<LANGUAGENAME.LIST>").unwrap();
    let end =
        list + xml[list..].find("</LANGUAGENAME.LIST>").unwrap() + "</LANGUAGENAME.LIST>".len();
    let removed = format!("{}{}", &xml[..list], &xml[end..]);
    assert_eq!(
        refusal(&removed),
        StandardLedgerCatalogError::LedgerNameUnusable
    );
}

#[test]
fn a_v2_answer_without_alter_ids_is_not_a_name_index() {
    let xml = decode(BASELINE);
    let mut stripped = String::new();
    let mut rest = xml.as_str();
    while let Some(start) = rest.find("<ALTERID") {
        let end = start + rest[start..].find("</ALTERID>").unwrap() + "</ALTERID>".len();
        stripped.push_str(&rest[..start]);
        rest = &rest[end..];
    }
    stripped.push_str(rest);
    assert_eq!(
        refusal(&stripped),
        StandardLedgerCatalogError::AlterIdInvalid
    );
}

/// After a restart the earlier renames carry their new spelling everywhere;
/// the two ledgers a removed space and removed full stops altered since the
/// restart keep their old spelling as the row spelling.
#[test]
fn a_removed_space_or_full_stop_folds_and_a_restart_settles_the_row_spelling() {
    let xml = decode(AFTER_FOLD_CASES);
    let rows = parsed(AFTER_FOLD_CASES);
    assert_eq!(rows.len(), collection_rows(&xml));
    assert_eq!(rows.len(), 19);
    let respelled = rows
        .iter()
        .filter(|row| row.row_spelling.is_some())
        .map(|row| (row.names[0].as_str(), row.row_spelling.as_deref().unwrap()))
        .collect::<Vec<_>>();
    assert_eq!(
        respelled,
        [
            ("MLab FreshParty", "MLab Fresh Party"),
            ("MLab Probe Pvt Ltd", "MLab Probe Pvt. Ltd."),
        ]
    );
    assert!(row(&rows, "MLAB CASE TARGET").row_spelling.is_none());
}

#[test]
fn markup_inside_a_name_is_refused() {
    for to in [
        "<NAME><![CDATA[MLab Alias Two]]></NAME>",
        "<NAME>MLab <B>Alias</B> Two</NAME>",
    ] {
        assert_eq!(
            refusal(&changed(BASELINE, "<NAME>MLab Alias Two</NAME>", to)),
            StandardLedgerCatalogError::LedgerNameUnusable,
            "{to}"
        );
    }
}

/// A reserved ledger renamed by a user keeps its built-in name in
/// `RESERVEDNAME`; the index keeps it as one more name. The capture's own
/// reserved row (Profit & Loss A/c, not renamed) adds nothing.
#[test]
fn a_renamed_reserved_ledgers_built_in_name_is_kept() {
    let rows = parsed(BASELINE);
    assert!(rows.iter().all(|row| row.reserved_name.is_none()));
    let renamed = changed(
        BASELINE,
        "<LEDGER NAME=\"MLab Alias Target\" RESERVEDNAME=\"\">",
        "<LEDGER NAME=\"MLab Alias Target\" RESERVEDNAME=\"Synthetic Reserved\">",
    );
    let rows = parse_ledger_name_index(&renamed, GUID).unwrap();
    assert_eq!(
        row(&rows, "MLab Alias Target").reserved_name.as_deref(),
        Some("Synthetic Reserved")
    );
}

/// A `LEDGER` outside the collection (beside it under `DATA`, or under
/// `DESC`) is a row the walk would step over: it would read as absent and a
/// Create of its name would alter it, so the answer is refused. Edited
/// captures: no answer of this shape has been seen.
#[test]
fn a_ledger_outside_the_collection_refuses_the_answer() {
    for (from, to) in [
        (
            "</COLLECTION>",
            "</COLLECTION><LEDGER NAME=\"Stray\"><NAME>Stray</NAME></LEDGER>",
        ),
        ("</COLLECTION>", "</COLLECTION><LEDGER/>"),
        ("<DESC>", "<DESC><LEDGER NAME=\"Stray\"></LEDGER>"),
    ] {
        assert_eq!(
            refusal(&changed(BASELINE, from, to)),
            StandardLedgerCatalogError::MalformedResponse,
            "{to}"
        );
    }
    // The CMPINFO counter is not a row, and the capture still parses.
    assert!(!parsed(BASELINE).is_empty());
}
