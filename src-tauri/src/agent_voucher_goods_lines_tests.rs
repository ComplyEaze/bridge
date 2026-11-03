//! Goods lines on a voucher's ledger entries, on the live capture of one day of a synthetic
//! book whose Purchase splits one line over two godowns and two batches, and on the
//! committed captures of a Sales and a Purchase item invoice. Each refusal case changes one
//! element of the first capture; none is authored from nothing.
use super::*;

const SHAPE_LAB: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const STOCK_LAB: &str = "1edb9b05-8d35-4c0c-9959-dce655730463";
const FIELD: &str = "goods_lines";
const UNREADABLE_FIELD: &str = "goods_lines_unreadable";

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn captured() -> String {
    utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_goods_lines_two_godowns_shape_lab_live.utf16le.xml"
    ))
}

fn changed(old: &str, new: &str) -> String {
    let xml = captured();
    assert_eq!(xml.matches(old).count(), 1, "one place to change: {old}");
    xml.replacen(old, new, 1)
}

fn parse(xml: &str) -> Vec<Value> {
    parse_agent_rows(xml, SHAPE_LAB).expect("the read still succeeds")
}

/// Where the capture's one goods line sits: (voucher, entry).
fn goods_entry(rows: &[Value]) -> (usize, usize) {
    let mut found = Vec::new();
    for (v, row) in rows.iter().enumerate() {
        for (e, entry) in row["amounts"].as_array().unwrap().iter().enumerate() {
            if entry.get(FIELD).is_some() || entry.get(UNREADABLE_FIELD).is_some() {
                found.push((v, e));
            }
        }
    }
    assert_eq!(found.len(), 1, "one entry carries goods: {found:?}");
    found[0]
}

/// A field Tally sent with a value.
fn value(text: &str) -> Value {
    json!({"state": "value", "value": text})
}

fn batch(godown: &str, batch: &str, quantity: &str, amount: &str) -> Value {
    json!({
        "godown": value(godown),
        "batch": value(batch),
        "actual_quantity": value(quantity),
        "billed_quantity": value(quantity),
        "amount": value(amount),
    })
}

/// The read with one change must differ from the unchanged read only in the goods keys of
/// the one entry that carries them.
fn assert_only_the_goods_differ(rows: &[Value]) {
    let mut base = parse(&captured());
    let mut rows = rows.to_vec();
    for rows in [&mut base, &mut rows] {
        for row in rows.iter_mut() {
            for entry in row["amounts"].as_array_mut().unwrap() {
                let entry = entry.as_object_mut().unwrap();
                entry.remove(FIELD);
                entry.remove(UNREADABLE_FIELD);
            }
        }
    }
    assert_eq!(rows, base);
}

fn assert_unreadable(xml: &str, reason: &str) {
    let rows = parse(xml);
    let (v, e) = goods_entry(&rows);
    let entry = &rows[v]["amounts"][e];
    assert_eq!(entry[UNREADABLE_FIELD], reason, "{entry}");
    assert!(
        entry.get(FIELD).is_none(),
        "no partial list beside a reason: {entry}"
    );
    assert_only_the_goods_differ(&rows);
}

#[test]
fn the_captured_line_and_its_two_godown_split_are_read_as_tally_sent_them() {
    let rows = parse(&captured());
    let (v, e) = goods_entry(&rows);
    assert_eq!(rows[v]["voucher_type"], "Purchase");
    assert_eq!(
        rows[v]["amounts"][e][FIELD],
        json!([{
            "stock_item": "Hydrochloric Acid 33pc",
            "actual_quantity": value("666.667 Kgs"),
            "billed_quantity": value("666.667 Kgs"),
            "rate": value("30.00/Kgs"),
            "amount": value("-20000.00"),
            "batches": [
                batch("Main Location", "H-2504-A", "300.000 Kgs", "-9000.00"),
                batch("Factory Floor", "H-2504-B", "366.667 Kgs", "-11000.00"),
            ],
        }])
    );
    // The Payment of the same day has no goods and no key.
    let payment = rows
        .iter()
        .find(|row| row["voucher_type"] == "Payment")
        .unwrap();
    for entry in payment["amounts"].as_array().unwrap() {
        assert!(entry.get(FIELD).is_none() && entry.get(UNREADABLE_FIELD).is_none());
    }
}

#[test]
fn the_committed_sales_and_purchase_invoices_carry_their_lines() {
    let sale = parse_agent_rows(
        &utf16(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/sales-day/register_window_taxed_sales_day_live.utf16le.xml"
        )),
        STOCK_LAB,
    )
    .unwrap();
    let (v, e) = goods_entry(&sale);
    assert_eq!(sale[v]["voucher_type"], "Sales");
    assert_eq!(
        sale[v]["amounts"][e][FIELD],
        json!([{
            "stock_item": "Lab Item 11",
            "actual_quantity": value("1 U."),
            "billed_quantity": value("1 U."),
            "rate": value("100.00/U."),
            "amount": value("100.00"),
            "batches": [batch("Main Location", "Primary Batch", "1 U.", "100.00")],
        }])
    );
    let purchase = parse_agent_rows(
        &utf16(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-reference-date-year-rows.utf16le.xml"
        )),
        SHAPE_LAB,
    )
    .unwrap();
    let (v, e) = goods_entry(&purchase);
    assert_eq!(purchase[v]["voucher_type"], "Purchase");
    let line = &purchase[v]["amounts"][e][FIELD][0];
    assert_eq!(line["stock_item"], "Soda Ash Light");
    assert_eq!(line["actual_quantity"], value("100.000 Kgs"));
    assert_eq!(line["amount"], value("-2500.00"));
}

#[test]
fn the_request_is_the_one_the_vouchers_read_sends() {
    let request = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_goods_lines_two_godowns_shape_lab_request.utf16le.xml"
    ));
    let day = bridge_tally_core::TallyDate::parse("20250424".to_string()).unwrap();
    assert_eq!(
        request.trim_start_matches('\u{feff}'),
        render_agent_vouchers("BRIDGE SHAPE LAB", &day, &day, None).unwrap()
    );
}

#[test]
fn the_stock_movement_requests_are_the_ones_the_vouchers_read_sends() {
    let requests: [(&[u8], &str); 4] = [
        (
            include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_stock_journal_vouchers_shape_lab_request.utf16le.xml"
            ),
            "20260511",
        ),
        (
            include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_delivery_note_vouchers_shape_lab_request.utf16le.xml"
            ),
            "20260512",
        ),
        (
            include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_receipt_note_vouchers_shape_lab_request.utf16le.xml"
            ),
            "20260513",
        ),
        (
            include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_credit_note_vouchers_shape_lab_request.utf16le.xml"
            ),
            "20260514",
        ),
    ];
    for (request, day) in requests {
        let request = utf16(request);
        let day = bridge_tally_core::TallyDate::parse(day.to_string()).unwrap();
        assert_eq!(
            request.trim_start_matches('\u{feff}'),
            render_agent_vouchers("BRIDGE SHAPE LAB", &day, &day, None).unwrap()
        );
    }
}

#[test]
fn an_empty_field_is_empty_and_a_missing_one_is_absent() {
    let rows = parse(&changed("<RATE>30.00/Kgs</RATE>", "<RATE/>"));
    let (v, e) = goods_entry(&rows);
    assert_eq!(
        rows[v]["amounts"][e][FIELD][0]["rate"],
        json!({"state": "empty"})
    );
    let rows = parse(&changed("<BILLEDQTY> 666.667 Kgs</BILLEDQTY>", ""));
    let (v, e) = goods_entry(&rows);
    assert_eq!(
        rows[v]["amounts"][e][FIELD][0]["billed_quantity"],
        json!({"state": "absent"})
    );
    assert_only_the_goods_differ(&rows);
}

#[test]
fn a_line_with_no_amount_is_listed_and_its_split_is_not_checked() {
    // The line's own amount (the first one inside the line), made empty: nothing to add
    // the split up to, so the line is listed with an empty amount and both rows.
    let xml = captured();
    let line = xml.find("<INVENTORYALLOCATIONS.LIST>").unwrap();
    let at = line + xml[line..].find("<AMOUNT>-20000.00</AMOUNT>").unwrap();
    let xml = format!(
        "{}<AMOUNT></AMOUNT>{}",
        &xml[..at],
        &xml[at + "<AMOUNT>-20000.00</AMOUNT>".len()..]
    );
    let rows = parse(&xml);
    let (v, e) = goods_entry(&rows);
    let line = &rows[v]["amounts"][e][FIELD][0];
    assert_eq!(line["amount"], json!({"state": "empty"}));
    assert_eq!(line["batches"].as_array().unwrap().len(), 2);
    assert_only_the_goods_differ(&rows);
}

#[test]
fn a_split_that_does_not_add_up_to_the_line_is_unreadable() {
    assert_unreadable(
        &changed("<AMOUNT>-9000.00</AMOUNT>", "<AMOUNT>-9000.01</AMOUNT>"),
        "goods_line_batch_amounts_differ",
    );
}

#[test]
fn an_amount_that_is_not_a_decimal_is_unreadable() {
    assert_unreadable(
        &changed("<AMOUNT>-11000.00</AMOUNT>", "<AMOUNT>-11,000.00</AMOUNT>"),
        "goods_line_amount_unparsable",
    );
}

#[test]
fn a_line_that_names_no_item_is_unreadable() {
    assert_unreadable(
        &changed(
            "<STOCKITEMNAME>Hydrochloric Acid 33pc</STOCKITEMNAME>",
            "<STOCKITEMNAME> </STOCKITEMNAME>",
        ),
        "goods_line_item_missing",
    );
}

#[test]
fn a_field_sent_twice_is_unreadable() {
    assert_unreadable(
        &changed(
            "<RATE>30.00/Kgs</RATE>",
            "<RATE>30.00/Kgs</RATE><RATE>31.00/Kgs</RATE>",
        ),
        "goods_line_field_duplicated",
    );
    assert_unreadable(
        &changed(
            "<BATCHNAME>H-2504-A</BATCHNAME>",
            "<BATCHNAME>H-2504-A</BATCHNAME><BATCHNAME>H-2504-C</BATCHNAME>",
        ),
        "goods_line_field_duplicated",
    );
}

#[test]
fn an_element_inside_a_field_and_a_list_at_another_depth_are_reported() {
    assert_unreadable(
        &changed(
            "<GODOWNNAME>Factory Floor</GODOWNNAME>",
            "<GODOWNNAME>Factory <X>Floor</X></GODOWNNAME>",
        ),
        "goods_line_shape_unrecognised",
    );
    assert_unreadable(
        &changed(
            "<GODOWNNAME>Factory Floor</GODOWNNAME>",
            "<GODOWNNAME>Factory Floor</GODOWNNAME><BATCHALLOCATIONS.LIST><AMOUNT>-1.00</AMOUNT></BATCHALLOCATIONS.LIST>",
        ),
        "goods_line_shape_unrecognised",
    );
}

#[test]
fn more_batch_rows_than_the_read_keeps_are_reported_not_truncated() {
    let xml = captured();
    let start = xml.find("<BATCHALLOCATIONS.LIST>").unwrap();
    let end = xml.find("</BATCHALLOCATIONS.LIST>").unwrap() + "</BATCHALLOCATIONS.LIST>".len();
    // 101 rows of the first one; the sum check never runs, the bound refuses first.
    let many = xml[start..end].repeat(101);
    let xml = format!("{}{}{}", &xml[..start], many, &xml[end..]);
    assert_unreadable(&xml, "goods_lines_too_many");
}

#[test]
fn a_thousand_lines_read_whole_and_one_more_is_reported_not_truncated() {
    let xml = captured();
    let open = "<INVENTORYALLOCATIONS.LIST>";
    let close = "</INVENTORYALLOCATIONS.LIST>";
    let start = xml.find(open).unwrap();
    let end = xml[start..].find(close).unwrap() + start + close.len();
    let with_lines = |count: usize| {
        format!(
            "{}{}{}",
            &xml[..start],
            xml[start..end].repeat(count),
            &xml[end..]
        )
    };
    let rows = parse(&with_lines(1000));
    let (v, e) = goods_entry(&rows);
    assert_eq!(rows[v]["amounts"][e][FIELD].as_array().unwrap().len(), 1000);
    assert_unreadable(&with_lines(1001), "goods_lines_too_many");
}

#[test]
fn an_escaped_or_cdata_name_reads_whole() {
    let rows = parse(&changed(
        "<STOCKITEMNAME>Hydrochloric Acid 33pc</STOCKITEMNAME>",
        "<STOCKITEMNAME>Acid &amp; Co</STOCKITEMNAME>",
    ));
    let (v, e) = goods_entry(&rows);
    assert_eq!(rows[v]["amounts"][e][FIELD][0]["stock_item"], "Acid & Co");
    let rows = parse(&changed(
        "<BATCHNAME>H-2504-B</BATCHNAME>",
        "<BATCHNAME><![CDATA[H-2504-B]]></BATCHNAME>",
    ));
    let (v, e) = goods_entry(&rows);
    assert_eq!(
        rows[v]["amounts"][e][FIELD][0]["batches"][1]["batch"],
        value("H-2504-B")
    );
}

#[test]
fn a_line_element_with_no_children_lists_nothing() {
    let xml = captured();
    let start = xml.find("<INVENTORYALLOCATIONS.LIST>").unwrap();
    let end =
        xml.find("</INVENTORYALLOCATIONS.LIST>").unwrap() + "</INVENTORYALLOCATIONS.LIST>".len();
    let xml = format!(
        "{}<INVENTORYALLOCATIONS.LIST>      </INVENTORYALLOCATIONS.LIST>{}",
        &xml[..start],
        &xml[end..]
    );
    for row in parse(&xml) {
        for entry in row["amounts"].as_array().unwrap() {
            assert!(entry.get(FIELD).is_none() && entry.get(UNREADABLE_FIELD).is_none());
        }
    }
}

#[test]
fn under_mask_parties_no_item_godown_or_batch_name_is_left() {
    let rows = parse(&captured());
    let marked: Vec<Value> = rows.into_iter().map(mark_voucher_party_names).collect();
    let masked = redact_value(Value::Array(marked.clone()), Redaction::MaskParties).to_string();
    for name in [
        "Hydrochloric Acid 33pc",
        "Main Location",
        "Factory Floor",
        "H-2504-A",
        "H-2504-B",
    ] {
        assert!(!masked.contains(name), "{name} left in {masked}");
    }
    // Quantities, rates and amounts are not names and stay.
    assert!(masked.contains("666.667 Kgs") && masked.contains("-11000.00"));
    let plain = redact_value(Value::Array(marked), Redaction::None).to_string();
    assert!(plain.contains("Hydrochloric Acid 33pc") && plain.contains("Factory Floor"));
}

#[test]
fn an_answer_shows_goods_lines_only_when_asked() {
    let rows = parse(&captured());
    let (v, e) = goods_entry(&rows);
    let row = rows[v].clone();
    let withheld = present_voucher_goods_lines(row.clone(), GoodsLinesShown::Withheld);
    for entry in withheld["amounts"].as_array().unwrap() {
        assert!(entry.get(FIELD).is_none(), "{entry}");
        assert!(entry["ledger"].is_string(), "{entry}");
    }
    assert_eq!(
        present_voucher_goods_lines(row.clone(), GoodsLinesShown::Shown),
        row
    );
    let mut unreadable = row;
    unreadable["amounts"][e]
        .as_object_mut()
        .unwrap()
        .insert(UNREADABLE_FIELD.to_string(), json!("goods_lines_too_many"));
    let withheld = present_voucher_goods_lines(unreadable, GoodsLinesShown::Withheld);
    assert!(withheld["amounts"][e].get(UNREADABLE_FIELD).is_none());
}

#[test]
fn the_argument_is_a_real_boolean_and_withholds_by_default() {
    assert_eq!(
        GoodsLinesShown::from_args(&json!({})),
        Ok(GoodsLinesShown::Withheld)
    );
    assert_eq!(
        GoodsLinesShown::from_args(&json!({"goods_lines": false})),
        Ok(GoodsLinesShown::Withheld)
    );
    assert_eq!(
        GoodsLinesShown::from_args(&json!({"goods_lines": true})),
        Ok(GoodsLinesShown::Shown)
    );
    for invalid in [json!("true"), json!(1)] {
        assert_eq!(
            GoodsLinesShown::from_args(&json!({"goods_lines": invalid})),
            Err("argument_invalid:goods_lines".to_string())
        );
    }
}

#[test]
fn amounts_of_another_scale_still_add_up_and_an_empty_batch_amount_skips_the_check() {
    // `-9000.0` and `-11000` against `-20000.00`: the same sum written with fewer places.
    let xml = changed("<AMOUNT>-9000.00</AMOUNT>", "<AMOUNT>-9000.0</AMOUNT>").replacen(
        "<AMOUNT>-11000.00</AMOUNT>",
        "<AMOUNT>-11000</AMOUNT>",
        1,
    );
    let rows = parse(&xml);
    let (v, e) = goods_entry(&rows);
    let line = &rows[v]["amounts"][e][FIELD][0];
    assert_eq!(line["batches"][0]["amount"], value("-9000.0"));
    assert_eq!(line["batches"][1]["amount"], value("-11000"));
    // One batch amount empty: nothing to add up, so the line is listed with both rows.
    let rows = parse(&changed("<AMOUNT>-9000.00</AMOUNT>", "<AMOUNT/>"));
    let (v, e) = goods_entry(&rows);
    let line = &rows[v]["amounts"][e][FIELD][0];
    assert_eq!(line["batches"][0]["amount"], json!({"state": "empty"}));
    assert_eq!(line["amount"], value("-20000.00"));
    assert_only_the_goods_differ(&rows);
}

#[test]
fn text_between_a_batch_rows_fields_never_reaches_the_lines_field() {
    // Text after a batch row's BILLEDQTY closes is inside the row, not inside any field: it
    // must not be added to the line's own billed quantity.
    let rows = parse(&changed(
        "<BILLEDQTY> 300.000 Kgs</BILLEDQTY>",
        "<BILLEDQTY> 300.000 Kgs</BILLEDQTY>stray",
    ));
    let (v, e) = goods_entry(&rows);
    let line = &rows[v]["amounts"][e][FIELD][0];
    assert_eq!(line["billed_quantity"], value("666.667 Kgs"));
    assert_eq!(line["batches"][0]["billed_quantity"], value("300.000 Kgs"));
}

#[test]
fn a_register_row_never_carries_goods_lines() {
    let rows = parse(&captured());
    let (v, e) = goods_entry(&rows);
    let entry = rows[v]["amounts"][e].clone();
    assert!(entry.get(FIELD).is_some());
    let mut unreadable = entry.clone();
    unreadable
        .as_object_mut()
        .unwrap()
        .insert(UNREADABLE_FIELD.to_string(), json!("goods_lines_too_many"));
    let row = super::super::register::mark_register_row(
        json!({"taxable_entries": [entry], "other_entries": [unreadable]}),
    );
    for list in ["taxable_entries", "other_entries"] {
        let entry = &row[list][0];
        assert!(
            entry.get(FIELD).is_none() && entry.get(UNREADABLE_FIELD).is_none(),
            "{entry}"
        );
    }
}

// -- #2016: a voucher's own stock lines, on the four captures of the inventory request. Each
// capture holds one voucher and, sent before the class COMPUTEs joined it, no resolved type, so
// `resolved` adds those elements; the live row of the whole request is the PR's evidence. --

const STOCK_JOURNAL: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_stock_journal_inventory_shape_lab_live.utf16le.xml"
);
const DELIVERY_NOTE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_delivery_note_inventory_shape_lab_live.utf16le.xml"
);
const RECEIPT_NOTE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_receipt_note_inventory_shape_lab_live.utf16le.xml"
);
const CREDIT_NOTE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_credit_note_inventory_shape_lab_live.utf16le.xml"
);

/// The capture as the class request answers it: its type's own reserved name and class.
fn resolved(capture: &[u8], reserved: &str, class: Option<ReservedVoucherClass>) -> String {
    let mut elements = format!(
        "<{VOUCHER_TYPE_GUID_TAG}>{SHAPE_LAB}-00000099</{VOUCHER_TYPE_GUID_TAG}>\
         <{VOUCHER_TYPE_RESERVED_NAME_TAG}>{reserved}</{VOUCHER_TYPE_RESERVED_NAME_TAG}>"
    );
    for each in ReservedVoucherClass::ALL {
        let answer = if class == Some(each) { "Yes" } else { "No" };
        elements.push_str(&format!("<{0}>{answer}</{0}>", each.tag()));
    }
    let xml = utf16(capture);
    let at = xml.find("</VOUCHERTYPENAME>").expect("a voucher") + "</VOUCHERTYPENAME>".len();
    format!("{}{elements}{}", &xml[..at], &xml[at..])
}

fn stock_rows(xml: &str) -> Vec<Value> {
    parse_agent_rows_reading_gst(
        xml,
        SHAPE_LAB,
        GstDetailsShown::Withheld,
        EInvoiceFields::NotAsked,
        StockLines::Asked,
    )
    .expect("the read succeeds")
}

fn one_note(capture: &[u8], reserved: &str, class: Option<ReservedVoucherClass>) -> Value {
    let rows = stock_rows(&resolved(capture, reserved, class));
    assert_eq!(rows.len(), 1);
    rows[0].clone()
}

#[test]
fn a_stock_journal_gives_both_sides_once_with_each_ones_own_godown() {
    let row = one_note(STOCK_JOURNAL, "Stock Journal", None);
    let lines = row["stock_lines"].as_array().unwrap();
    // Two lines, not four: the journal's in and out lists repeat them without godowns.
    assert_eq!(lines.len(), 2, "{row}");
    assert_eq!(lines[0]["direction"], "in");
    assert_eq!(lines[1]["direction"], "out");
    assert_eq!(lines[0]["stock_item"], "Hydrochloric Acid 33pc");
    assert_eq!(lines[0]["actual_quantity"], value("10.000 Kgs"));
    assert_eq!(lines[0]["batches"][0]["godown"], value("Factory Floor"));
    assert_eq!(lines[0]["batches"][0]["batch"], value("H-2504-A"));
    assert_eq!(lines[1]["batches"][0]["godown"], value("Main Location"));
    // The in side carries no value in this capture: not zero, and not borrowed from the out side.
    assert_eq!(lines[0]["amount"], json!({"state": "empty"}));
    assert_eq!(lines[0]["rate"], json!({"state": "empty"}));
    assert_eq!(lines[1]["amount"], value("300.00"));
    assert!(lines
        .iter()
        .all(|line| line.get("deemed_positive").is_none()));
    assert_eq!(row["amounts"], json!([]));
}

#[test]
fn each_note_says_which_way_its_goods_moved() {
    for (capture, reserved, class, direction, amount, godown) in [
        (
            DELIVERY_NOTE,
            "Delivery Note",
            None,
            "out",
            "30.00",
            "Main Location",
        ),
        (
            RECEIPT_NOTE,
            "Receipt Note",
            None,
            "in",
            "-30.00",
            "Factory Floor",
        ),
        (
            CREDIT_NOTE,
            "Credit Note",
            Some(ReservedVoucherClass::CreditNote),
            "in",
            "-30.00",
            "Main Location",
        ),
    ] {
        let row = one_note(capture, reserved, class);
        let lines = row["stock_lines"].as_array().unwrap();
        assert_eq!(lines.len(), 1, "{row}");
        assert_eq!(lines[0]["direction"], direction, "{row}");
        assert_eq!(lines[0]["amount"], value(amount), "{row}");
        assert_eq!(lines[0]["batches"][0]["godown"], value(godown), "{row}");
    }
}

#[test]
fn a_kind_no_capture_covers_is_named_and_its_lines_are_not_read() {
    for (reserved, class) in [
        ("Sales", Some(ReservedVoucherClass::Sales)),
        ("Purchase", Some(ReservedVoucherClass::Purchase)),
        ("Debit Note", Some(ReservedVoucherClass::DebitNote)),
        ("Physical Stock", None),
        // A type the user made has no reserved name, and no measured class says Delivery Note.
        ("", None),
    ] {
        let row = one_note(DELIVERY_NOTE, reserved, class);
        assert_eq!(row["stock_lines_not_read"], true, "{row}");
        assert!(row.get("stock_lines").is_none(), "{row}");
    }
    // A child of Credit Note answers its class, but no capture holds one.
    let child = one_note(CREDIT_NOTE, "", Some(ReservedVoucherClass::CreditNote));
    assert_eq!(child["stock_lines_not_read"], true, "{child}");
    assert!(child.get("stock_lines").is_none(), "{child}");
}

/// The capture with its stock lines cut out, as the Stock Journal answered a childless list on
/// the shipped read.
fn without_lines(capture: &[u8], reserved: &str, class: Option<ReservedVoucherClass>) -> String {
    let xml = resolved(capture, reserved, class);
    let from = xml.find("<ALLINVENTORYENTRIES.LIST>").expect("a line");
    let end = "</ALLINVENTORYENTRIES.LIST>";
    let to = xml.rfind(end).expect("a line") + end.len();
    format!("{}{}", &xml[..from], &xml[to..])
}

#[test]
fn a_voucher_that_only_moves_stock_and_answers_no_line_is_unreadable() {
    for (capture, reserved) in [
        (STOCK_JOURNAL, "Stock Journal"),
        (DELIVERY_NOTE, "Delivery Note"),
        (RECEIPT_NOTE, "Receipt Note"),
    ] {
        let row = stock_rows(&without_lines(capture, reserved, None)).remove(0);
        assert_eq!(row["stock_lines_unreadable"], "stock_line_missing", "{row}");
        assert!(row.get("stock_lines").is_none(), "{row}");
    }
}

#[test]
fn a_credit_note_may_answer_no_line() {
    let class = Some(ReservedVoucherClass::CreditNote);
    let row = stock_rows(&without_lines(CREDIT_NOTE, "Credit Note", class)).remove(0);
    assert_eq!(row["stock_lines"], json!([]), "{row}");
    assert!(row.get("stock_lines_unreadable").is_none(), "{row}");
}

#[test]
fn a_row_with_no_resolved_type_is_not_read() {
    let rows = stock_rows(&utf16(DELIVERY_NOTE));
    assert_eq!(rows[0]["stock_lines_not_read"], true);
}

#[test]
fn a_read_that_does_not_ask_carries_no_stock_key() {
    let row = parse(&resolved(STOCK_JOURNAL, "Stock Journal", None)).remove(0);
    for key in [
        "stock_lines",
        "stock_lines_not_read",
        "stock_lines_unreadable",
    ] {
        assert!(row.get(key).is_none(), "{row}");
    }
}

#[test]
fn a_line_that_cannot_be_read_gives_its_reason_and_no_lines() {
    let delivery = || resolved(DELIVERY_NOTE, "Delivery Note", None);
    let foreign = "$1.00 @ ? 30.00/$ = ? 30.00";
    for (old, new, reason) in [
        (
            "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>",
            "<ISDEEMEDPOSITIVE>Maybe</ISDEEMEDPOSITIVE>",
            "goods_line_direction_unreadable",
        ),
        (
            "<AMOUNT>30.00</AMOUNT>",
            &format!("<AMOUNT>{foreign}</AMOUNT>"),
            "goods_line_amount_unparsable",
        ),
    ] {
        let xml = delivery().replacen(old, new, 1);
        assert_ne!(xml, delivery(), "{old}");
        let row = stock_rows(&xml).remove(0);
        assert_eq!(row["stock_lines_unreadable"], reason, "{row}");
        assert!(row.get("stock_lines").is_none(), "{row}");
    }
}

#[test]
fn the_stock_line_request_is_the_captured_one_with_the_class_elements() {
    let request = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/stock_movement_stock_journal_inventory_shape_lab_request.utf16le.xml"
    ));
    let day = bridge_tally_core::TallyDate::parse("20260511").unwrap();
    let rendered = super::super::voucher_window::VoucherReadShape::ClassInventoryWildcard
        .render("BRIDGE SHAPE LAB", &day, &day, None)
        .unwrap();
    assert_eq!(
        rendered.replace(&voucher_type_class_computes(), ""),
        request.trim_start_matches('\u{feff}')
    );
    assert!(rendered.contains(&voucher_type_class_computes()));
}

#[test]
fn stock_lines_refuse_what_needs_ledger_entries_and_nothing_else() {
    assert_eq!(StockLines::from_args(&json!({})), Ok(StockLines::NotAsked));
    assert_eq!(
        StockLines::from_args(&json!({"stock_lines": false, "ledger": "Cash"})),
        Ok(StockLines::NotAsked)
    );
    assert_eq!(
        StockLines::from_args(&json!({
            "stock_lines": true, "goods_lines": false, "voucher_type": "Stock Journal",
            "narration_contains": "x", "ledger": null,
        })),
        Ok(StockLines::Asked)
    );
    for (name, given) in [
        ("ledger", json!("Cash")),
        ("amount", json!("30.00")),
        ("suspense_tagged", json!(true)),
        ("summarise_by", json!("ledger")),
        ("bank_allocations", json!(true)),
        ("goods_lines", json!(true)),
        ("include_gst_details", json!(true)),
        ("include_e_invoice", json!(true)),
    ] {
        let mut args = json!({"stock_lines": true});
        args[name] = given;
        assert_eq!(
            StockLines::from_args(&args),
            Err(format!("voucher_stock_lines_excludes_{name}"))
        );
    }
}

#[test]
fn under_mask_parties_no_stock_line_name_is_left() {
    let row = mark_voucher_party_names(one_note(DELIVERY_NOTE, "Delivery Note", None));
    let masked = redact_value(row, Redaction::MaskParties).to_string();
    for name in ["Hydrochloric Acid 33pc", "Main Location", "H-2504-A"] {
        assert!(!masked.contains(name), "{name} left in {masked}");
    }
    assert!(masked.contains("1.000 Kgs") && masked.contains("30.00"));
}
