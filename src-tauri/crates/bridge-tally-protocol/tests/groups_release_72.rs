//! `List of Groups` on TallyPrime 7.2 against 7.1 (one book, `BRIDGE TIEOUT B`).
//!
//! The two captures hold the same 28 groups with the same values. Inside each row, the request's own
//! computed `BRIDGECOMPANYGUID` follows `PARENTSTRUCTURE` and comes after `PRIMARYGRPPARENT` on 7.1,
//! and before it on 7.2. The group readers take a row's children by name, so the records are the
//! same and only the per-row source hash differs, because it is taken over the row's bytes.

use bridge_tally_protocol::native_outstandings::{
    parse_native_group_snapshot, parse_native_group_snapshot_with_evidence,
};
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_native_group_source_records_with_evidence,
    ExpectedTallyTextEncoding,
};

const RELEASE_71: &[u8] = include_bytes!("fixtures/statement_groups_stock_book_live.utf16le.xml");
const RELEASE_72: &[u8] =
    include_bytes!("fixtures/statement_groups_stock_book_72_live.utf16le.xml");
const TIEOUT_B_GUID: &str = "d64eda55-da7c-4106-a171-3e3cb6cdd331";

fn decode(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("a captured BOM-less UTF-16LE response decodes")
    .text
}

/// For each group row, the names of its direct children, in order.
fn row_children(text: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("<GROUP ") {
        let end = start + rest[start..].find("</GROUP>").expect("a closed row");
        let mut names = Vec::new();
        let mut depth = 0usize;
        for tag in rest[start..end].split('<').skip(1) {
            let name: String = tag
                .chars()
                .take_while(|c| !c.is_whitespace() && *c != '>' && *c != '/')
                .collect();
            if tag.starts_with('/') {
                depth = depth.saturating_sub(1);
            } else if !name.is_empty() {
                if depth == 1 {
                    names.push(name);
                }
                if !tag.split('>').next().unwrap_or("").ends_with('/') {
                    depth += 1;
                }
            }
        }
        rows.push(names);
        rest = &rest[end..];
    }
    rows
}

#[test]
fn the_computed_field_moves_between_the_releases_and_no_other_child_does() {
    let (old, new) = (
        row_children(&decode(RELEASE_71)),
        row_children(&decode(RELEASE_72)),
    );
    assert_eq!(old.len(), 28);
    assert_eq!(new.len(), 28);
    let position = |names: &[String], name: &str| {
        names
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("{name} in {names:?}"))
    };
    for (old_row, new_row) in old.iter().zip(&new) {
        assert!(position(old_row, "PRIMARYGRPPARENT") < position(old_row, "BRIDGECOMPANYGUID"));
        assert!(position(new_row, "BRIDGECOMPANYGUID") < position(new_row, "PRIMARYGRPPARENT"));
        // Apart from that one element, the children are the same, in the same order.
        let without = |names: &[String]| {
            names
                .iter()
                .filter(|name| *name != "BRIDGECOMPANYGUID")
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(without(old_row), without(new_row));
    }
}

/// The text without the lines that hold the computed `BRIDGECOMPANYGUID`, and how many there were.
fn without_computed_field(text: &str) -> (String, usize) {
    let (kept, removed): (Vec<&str>, Vec<&str>) = text
        .lines()
        .partition(|line| !line.contains("BRIDGECOMPANYGUID"));
    (kept.join("\n"), removed.len())
}

#[test]
fn the_two_captures_are_equal_once_the_computed_field_is_removed() {
    let (old, old_removed) = without_computed_field(&decode(RELEASE_71));
    let (new, new_removed) = without_computed_field(&decode(RELEASE_72));
    assert_eq!((old_removed, new_removed), (28, 28));
    assert_eq!(old, new);
}

#[test]
fn both_releases_give_the_same_group_records_and_different_row_hashes() {
    let (old, new) = (decode(RELEASE_71), decode(RELEASE_72));
    assert_eq!(
        parse_native_group_snapshot(&old, TIEOUT_B_GUID).unwrap(),
        parse_native_group_snapshot(&new, TIEOUT_B_GUID).unwrap()
    );
    let old_rows = parse_native_group_snapshot_with_evidence(&old, TIEOUT_B_GUID).unwrap();
    let new_rows = parse_native_group_snapshot_with_evidence(&new, TIEOUT_B_GUID).unwrap();
    assert_eq!(old_rows.len(), 28);
    assert_eq!(new_rows.len(), 28);
    for (old_row, new_row) in old_rows.iter().zip(&new_rows) {
        assert_eq!(old_row.record, new_row.record);
        assert_ne!(old_row.raw_source_sha256, new_row.raw_source_sha256);
    }
}

#[test]
fn the_source_record_reader_gives_the_same_records_for_both_releases() {
    let (old, new) = (decode(RELEASE_71), decode(RELEASE_72));
    let old_rows = parse_native_group_source_records_with_evidence(&old, TIEOUT_B_GUID).unwrap();
    let new_rows = parse_native_group_source_records_with_evidence(&new, TIEOUT_B_GUID).unwrap();
    assert_eq!(old_rows.records.len(), 28);
    assert_eq!(old_rows.records.len(), new_rows.records.len());
    for (old_row, new_row) in old_rows.records.iter().zip(&new_rows.records) {
        assert_eq!(old_row.record, new_row.record);
        assert_eq!(old_row.source_id, new_row.source_id);
        assert_eq!(old_row.alter_id, new_row.alter_id);
        assert_ne!(old_row.raw_source_sha256, new_row.raw_source_sha256);
    }
}
