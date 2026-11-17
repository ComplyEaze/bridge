// SPDX-License-Identifier: Apache-2.0

use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited,
    gst_details::{GstDetailsEntry, GstDetailsObservation},
    hsn_details::{HsnDetailsEntry, HsnDetailsObservation},
    native_masters::{parse_native_masters, NativeMasterKind, NativeMastersError},
    stock_item_candidate::{parse_candidate_stock_items, CandidateStockItem, ScalarObservation},
    ExpectedTallyTextEncoding, TALLY_SANITIZED_ROOT_MARKER,
};

const COMPANY: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";
const SHIPPED_COMPANY: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const BEFORE: &[u8] =
    include_bytes!("fixtures/agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml");
const PLAIN: &[u8] = include_bytes!(
    "fixtures/agent/stock-create-lab/stock-create-lab-items-after-plain-create.utf16le.xml"
);
const MARKED: &[u8] = include_bytes!(
    "fixtures/agent/stock-create-lab/stock-create-lab-items-after-marked-create.utf16le.xml"
);
const ALIAS: &[u8] =
    include_bytes!("fixtures/agent/stock-create-lab/stock-create-lab-items-with-alias.utf16le.xml");
const SHIPPED: &[u8] =
    include_bytes!("fixtures/masters_stock_items_shape_lab_baseline_live.utf16le.xml");
const SHIPPED_TWO_DATES: &[u8] =
    include_bytes!("fixtures/masters_stock_items_shape_lab_two_dates_live.utf16le.xml");

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

fn parsed(bytes: &[u8]) -> Vec<CandidateStockItem> {
    parse_candidate_stock_items(&decode(bytes), COMPANY).expect("the capture parses")
}

fn row<'a>(rows: &'a [CandidateStockItem], name: &str) -> &'a CandidateStockItem {
    rows.iter()
        .find(|row| row.name == name)
        .expect("the named row")
}

fn changed(bytes: &[u8], from: &str, to: &str) -> String {
    let xml = decode(bytes);
    assert_eq!(xml.matches(from).count(), 1, "the change names one place");
    xml.replacen(from, to, 1)
}

fn hsn(item: &CandidateStockItem) -> &[HsnDetailsEntry] {
    match &item.hsn {
        HsnDetailsObservation::Entries { entries } => entries,
        other => panic!("{other:?}"),
    }
}

fn gst(item: &CandidateStockItem) -> &[GstDetailsEntry] {
    match &item.gst {
        GstDetailsObservation::Entries { entries } => entries,
        other => panic!("{other:?}"),
    }
}

fn assert_sibling(item: &CandidateStockItem) {
    assert_eq!(item.parent.as_deref(), Some("SC Goods Group"));
    assert_eq!(item.base_units, ScalarObservation::Text("Nos".into()));
    assert_eq!(
        item.gst_type_of_supply,
        ScalarObservation::Text("Goods".into())
    );
    assert_eq!(
        item.gst_applicable,
        ScalarObservation::Text(format!("{TALLY_SANITIZED_ROOT_MARKER} Applicable"))
    );
    assert_eq!(item.names, ["SC Sibling Goods"]);
    assert_eq!(hsn(item).len(), 1);
    assert_eq!(hsn(item)[0].hsn_code.as_deref(), Some("39239090"));
    assert_eq!(gst(item).len(), 1);
    assert_eq!(gst(item)[0].states.len(), 1);
    assert_eq!(gst(item)[0].states[0].rates.len(), 5);
}

#[test]
fn before_reads_the_siblings_candidate_fields_and_typed_lists() {
    let rows = parsed(BEFORE);
    assert_eq!(rows.len(), 1);
    assert_sibling(row(&rows, "SC Sibling Goods"));
}

#[test]
fn plain_create_keeps_absent_applicability_and_an_empty_state_row() {
    let rows = parsed(PLAIN);
    let copied = row(&rows, "SC Copied Goods");
    assert_eq!(copied.gst_applicable, ScalarObservation::Absent);
    assert_eq!(copied.base_units, ScalarObservation::Text("Nos".into()));
    assert_eq!(
        copied.gst_type_of_supply,
        ScalarObservation::Text("Goods".into())
    );
    assert_eq!(gst(copied).len(), 1);
    assert_eq!(gst(copied)[0].states.len(), 1);
    assert_eq!(gst(copied)[0].states[0].state_name, None);
    assert!(gst(copied)[0].states[0].rates.is_empty());
    assert_sibling(row(&rows, "SC Sibling Goods"));
}

#[test]
fn marked_create_matches_the_sibling_but_plain_create_does_not() {
    let rows = parsed(MARKED);
    let sibling = row(&rows, "SC Sibling Goods");
    let marked = row(&rows, "SC Copied Marked");
    let plain = row(&rows, "SC Copied Goods");
    assert_eq!(marked.parent, sibling.parent);
    assert_eq!(marked.base_units, sibling.base_units);
    assert_eq!(marked.gst_applicable, sibling.gst_applicable);
    assert_eq!(marked.gst_type_of_supply, sibling.gst_type_of_supply);
    assert_eq!(marked.hsn, sibling.hsn);
    assert_eq!(marked.gst, sibling.gst);
    assert_ne!(marked.name, sibling.name);
    assert_ne!(marked.guid, sibling.guid);
    assert_ne!(plain.gst_applicable, sibling.gst_applicable);
    assert_ne!(plain.gst, sibling.gst);
}

#[test]
fn every_language_name_is_kept_in_document_order() {
    let rows = parsed(ALIAS);
    assert_eq!(
        row(&rows, "SC Sibling Goods").names,
        ["SC Sibling Goods", "SC Sibling Alias"]
    );
    assert_eq!(row(&rows, "SC Copied Goods").names, ["SC Copied Goods"]);
    assert_eq!(row(&rows, "SC Copied Marked").names, ["SC Copied Marked"]);
}

#[test]
fn the_shipped_read_leaves_all_three_scalars_absent() {
    for bytes in [SHIPPED, SHIPPED_TWO_DATES] {
        let text = decode(bytes);
        let native = parse_native_masters(NativeMasterKind::StockItems, &text, SHIPPED_COMPANY)
            .expect("the shipped capture parses");
        let rows = parse_candidate_stock_items(&text, SHIPPED_COMPANY)
            .expect("the candidate read accepts the shipped shape");
        assert_eq!(rows.len(), native.rows.len());
        assert!(rows.iter().all(|row| {
            row.base_units == ScalarObservation::Absent
                && row.gst_applicable == ScalarObservation::Absent
                && row.gst_type_of_supply == ScalarObservation::Absent
        }));
    }
}

#[test]
fn absent_empty_and_whitespace_units_stay_distinct() {
    const UNIT: &str = "<BASEUNITS TYPE=\"String\">Nos</BASEUNITS>";
    let rows = parse_candidate_stock_items(&changed(BEFORE, UNIT, ""), COMPANY).unwrap();
    assert_eq!(rows[0].base_units, ScalarObservation::Absent);
    for empty in [
        "<BASEUNITS TYPE=\"String\"></BASEUNITS>",
        "<BASEUNITS TYPE=\"String\"/>",
        "<BASEUNITS TYPE=\"String\">   </BASEUNITS>",
    ] {
        let rows = parse_candidate_stock_items(&changed(BEFORE, UNIT, empty), COMPANY).unwrap();
        assert_eq!(rows[0].base_units, ScalarObservation::Empty);
    }
}

#[test]
fn malformed_candidate_fields_return_typed_errors() {
    const SUPPLY: &str = "<GSTTYPEOFSUPPLY TYPE=\"String\">Goods</GSTTYPEOFSUPPLY>";
    assert_eq!(
        parse_candidate_stock_items(
            &changed(BEFORE, SUPPLY, &(SUPPLY.to_string() + SUPPLY)),
            COMPANY
        ),
        Err(NativeMastersError::Malformed("masters_row_field_repeated"))
    );
    assert_eq!(
        parse_candidate_stock_items(
            &changed(
                BEFORE,
                "<GSTAPPLICABLE TYPE=\"String\">&#4; Applicable</GSTAPPLICABLE>",
                "<GSTAPPLICABLE TYPE=\"String\"><X/></GSTAPPLICABLE>"
            ),
            COMPANY
        ),
        Err(NativeMastersError::Malformed(
            "masters_scalar_not_text_only"
        ))
    );
    assert_eq!(
        parse_candidate_stock_items(
            &changed(BEFORE, "<NAME>SC Sibling Goods</NAME>", "<NAME></NAME>"),
            COMPANY
        ),
        Err(NativeMastersError::Malformed("masters_name_entry_empty"))
    );
    assert_eq!(
        parse_candidate_stock_items(&decode(BEFORE), SHIPPED_COMPANY),
        Err(NativeMastersError::RowGuidForeign)
    );
}

#[test]
fn a_scalar_named_inside_a_list_is_not_read_as_the_rows() {
    const SUPPLY: &str = "<GSTTYPEOFSUPPLY TYPE=\"String\">Goods</GSTTYPEOFSUPPLY>";
    const LIST: &str = "<LANGUAGENAME.LIST>";
    let nested = format!("{LIST}<GSTTYPEOFSUPPLY TYPE=\"String\">Services</GSTTYPEOFSUPPLY>");
    let rows = parse_candidate_stock_items(&changed(BEFORE, LIST, &nested), COMPANY).unwrap();
    assert_eq!(
        rows[0].gst_type_of_supply,
        ScalarObservation::Text("Goods".into())
    );
    let only_nested = changed(BEFORE, SUPPLY, "").replacen(LIST, &nested, 1);
    let rows = parse_candidate_stock_items(&only_nested, COMPANY).unwrap();
    assert_eq!(rows[0].gst_type_of_supply, ScalarObservation::Absent);
    let nested_empty = format!("{LIST}<GSTTYPEOFSUPPLY TYPE=\"String\"/>");
    let rows = parse_candidate_stock_items(&changed(BEFORE, LIST, &nested_empty), COMPANY).unwrap();
    assert_eq!(
        rows[0].gst_type_of_supply,
        ScalarObservation::Text("Goods".into())
    );
}

#[test]
fn a_self_closing_or_blank_language_name_is_refused() {
    const NAME: &str = "<NAME>SC Sibling Goods</NAME>";
    for blank in ["<NAME/>", "<NAME>  </NAME>"] {
        assert_eq!(
            parse_candidate_stock_items(&changed(BEFORE, NAME, blank), COMPANY),
            Err(NativeMastersError::Malformed("masters_name_entry_empty"))
        );
    }
}

#[test]
fn names_from_two_language_lists_are_kept_in_document_order() {
    const END: &str = "</LANGUAGENAME.LIST>";
    let second = format!(
        "{END}<LANGUAGENAME.LIST><NAME.LIST TYPE=\"String\"><NAME>SC Second Language</NAME></NAME.LIST>{END}"
    );
    let rows = parse_candidate_stock_items(&changed(BEFORE, END, &second), COMPANY).unwrap();
    assert_eq!(rows[0].names, ["SC Sibling Goods", "SC Second Language"]);
}

#[test]
fn a_scalar_that_is_not_plain_text_is_refused() {
    const UNIT: &str = "<BASEUNITS TYPE=\"String\">Nos</BASEUNITS>";
    for (bad, error) in [
        (
            "<BASEUNITS TYPE=\"String\"><![CDATA[Nos]]></BASEUNITS>",
            "masters_scalar_not_text_only",
        ),
        (
            "<BASEUNITS TYPE=\"String\">&bogus;</BASEUNITS>",
            "masters_xml_invalid_escape",
        ),
    ] {
        assert_eq!(
            parse_candidate_stock_items(&changed(BEFORE, UNIT, bad), COMPANY),
            Err(NativeMastersError::Malformed(error))
        );
    }
}
