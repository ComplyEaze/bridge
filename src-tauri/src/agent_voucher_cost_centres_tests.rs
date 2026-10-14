//! Cost-centre allocations on a voucher's ledger entries, on the live capture of the
//! request the vouchers read sends. Each refusal case changes one element of that
//! capture; none is authored from nothing.
use super::*;

const COMPANY_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const COSTCENTRE_FIELD: &str = "cost_centre_allocations";
const UNREADABLE_FIELD: &str = "cost_centre_allocations_unreadable";

fn captured() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_cost_centre_allocations_shape_lab_live.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn changed(old: &str, new: &str) -> String {
    let xml = captured();
    assert_eq!(xml.matches(old).count(), 1, "one place to change: {old}");
    xml.replacen(old, new, 1)
}

fn parse(xml: &str) -> Vec<Value> {
    parse_agent_rows(xml, COMPANY_GUID).expect("the read still succeeds")
}

fn without(entry: &Value, key: &str) -> Value {
    let mut copy = entry.clone();
    copy.as_object_mut().unwrap().remove(key);
    copy
}

/// The capture's read with one change must differ from the unchanged read only in how the
/// first entry carries its allocations: the second entry and the voucher stay as they were.
fn assert_only_the_allocations_differ(rows: &[Value]) {
    let base = parse(&captured());
    assert_eq!(rows.len(), base.len());
    assert_eq!(rows[0]["amounts"][1], base[0]["amounts"][1]);
    let mut mutated = rows[0].clone();
    let mut original = base[0].clone();
    for entry in [&mut mutated, &mut original] {
        let first = entry["amounts"][0].as_object_mut().unwrap();
        first.remove(COSTCENTRE_FIELD);
        first.remove(UNREADABLE_FIELD);
    }
    assert_eq!(mutated, original);
}

fn assert_unreadable(xml: &str, reason: &str) {
    let rows = parse(xml);
    let entry = &rows[0]["amounts"][0];
    assert_eq!(entry[UNREADABLE_FIELD], reason);
    assert!(
        entry.get(COSTCENTRE_FIELD).is_none(),
        "no partial list beside a reason: {entry}"
    );
    assert_only_the_allocations_differ(&rows);
}

#[test]
fn captured_cost_centres_are_read_beside_the_entry_that_carries_them() {
    let rows = parse(&captured());
    assert_eq!(rows.len(), 1);
    let entries = &rows[0]["amounts"];
    assert_eq!(entries.as_array().unwrap().len(), 2);
    assert_eq!(entries[0]["amount"], "-1000.00");
    assert_eq!(
        entries[0][COSTCENTRE_FIELD],
        json!([
            {"category": "Business Line", "cost_centre": "Trading", "amount": "-600.00"},
            {"category": "Business Line", "cost_centre": "Assembly", "amount": "-400.00"},
        ])
    );
    assert!(entries[0].get(UNREADABLE_FIELD).is_none());
    // The other entry has no allocation element at all in this capture: no key.
    assert!(entries[1].get(COSTCENTRE_FIELD).is_none());
    assert!(entries[1].get(UNREADABLE_FIELD).is_none());
    // The existing fields of both entries are as they were.
    assert_eq!(entries[0]["bill_allocations"], json!([]));
    assert_eq!(entries[0]["ledger"], "Power Charges");
}

#[test]
fn an_empty_or_missing_amount_is_never_zero() {
    for replacement in ["<AMOUNT/>", "<AMOUNT></AMOUNT>", "<AMOUNT>   </AMOUNT>", ""] {
        assert_unreadable(
            &changed("<AMOUNT>-600.00</AMOUNT>", replacement),
            "cost_centre_amount_missing",
        );
    }
}

#[test]
fn an_amount_that_is_not_a_decimal_is_unreadable_and_a_foreign_composite_is_too() {
    for text in ["abc", "1,000.00", "-600.00.00", "$ 8 = 600.00"] {
        assert_unreadable(
            &changed(
                "<AMOUNT>-600.00</AMOUNT>",
                &format!("<AMOUNT>{text}</AMOUNT>"),
            ),
            "cost_centre_amount_unparsable",
        );
    }
}

#[test]
fn a_missing_or_empty_name_is_unreadable() {
    for replacement in ["<NAME/>", "<NAME></NAME>", "<NAME>  </NAME>", ""] {
        assert_unreadable(
            &changed("<NAME>Trading</NAME>", replacement),
            "cost_centre_name_missing",
        );
    }
}

#[test]
fn a_centre_named_twice_in_one_category_is_unreadable() {
    assert_unreadable(
        &changed("<NAME>Assembly</NAME>", "<NAME>Trading</NAME>"),
        "cost_centre_duplicate",
    );
}

#[test]
fn a_category_without_a_name_is_unreadable() {
    for replacement in ["<CATEGORY/>", "<CATEGORY> </CATEGORY>"] {
        assert_unreadable(
            &changed("<CATEGORY>Business Line</CATEGORY>", replacement),
            "cost_category_missing",
        );
    }
}

#[test]
fn a_field_sent_twice_in_one_centre_is_unreadable() {
    assert_unreadable(
        &changed(
            "<NAME>Trading</NAME>",
            "<NAME>Trading</NAME><NAME>Trading</NAME>",
        ),
        "cost_centre_field_duplicated",
    );
}

#[test]
fn a_category_named_twice_is_unreadable() {
    assert_unreadable(
        &changed(
            "<CATEGORY>Business Line</CATEGORY>",
            "<CATEGORY>Business Line</CATEGORY><CATEGORY>Business Line</CATEGORY>",
        ),
        "cost_centre_field_duplicated",
    );
}

#[test]
fn a_named_category_with_no_centre_is_unreadable_not_dropped() {
    let xml = captured();
    let first = xml.find("<COSTCENTREALLOCATIONS.LIST>").unwrap();
    let close = "</COSTCENTREALLOCATIONS.LIST>";
    let last = xml.rfind(close).unwrap() + close.len();
    let cut = format!("{}{}", &xml[..first], &xml[last..]);
    assert_unreadable(&cut, "cost_category_without_centres");
}

#[test]
fn an_empty_container_is_the_placeholder_and_adds_nothing() {
    let xml = captured();
    let first = xml.find("<CATEGORYALLOCATIONS.LIST>").unwrap();
    let close = "</CATEGORYALLOCATIONS.LIST>";
    let last = xml.find(close).unwrap() + close.len();
    for placeholder in [
        "<CATEGORYALLOCATIONS.LIST>      </CATEGORYALLOCATIONS.LIST>",
        "<CATEGORYALLOCATIONS.LIST/>",
    ] {
        let emptied = format!("{}{}{}", &xml[..first], placeholder, &xml[last..]);
        let rows = parse(&emptied);
        let entry = &rows[0]["amounts"][0];
        assert!(entry.get(COSTCENTRE_FIELD).is_none());
        assert!(entry.get(UNREADABLE_FIELD).is_none());
        assert_only_the_allocations_differ(&rows);
    }
}

#[test]
fn an_escaped_name_reads_as_one_value_and_other_entries_stay_untouched() {
    let rows = parse(&changed(
        "<NAME>Trading</NAME>",
        "<NAME>Trad&amp;ing</NAME>",
    ));
    assert_eq!(
        rows[0]["amounts"][0][COSTCENTRE_FIELD][0]["cost_centre"],
        "Trad&ing"
    );
    assert_eq!(
        without(&rows[0]["amounts"][1], COSTCENTRE_FIELD),
        without(&parse(&captured())[0]["amounts"][1], COSTCENTRE_FIELD)
    );
}

#[test]
fn the_sort_order_fields_and_empty_quantities_are_not_read_as_amounts() {
    // PAYHEADSORTORDER, EMPLOYEESORTORDER, ACTUALQTY and BILLEDQTY sit beside NAME and
    // AMOUNT in the capture; a non-numeric ACTUALQTY must change nothing.
    let xml = captured();
    assert!(xml.contains("<ACTUALQTY/>"));
    let rows = parse(&xml.replacen("<ACTUALQTY/>", "<ACTUALQTY>not a number</ACTUALQTY>", 1));
    assert_eq!(
        rows[0]["amounts"][0][COSTCENTRE_FIELD],
        parse(&captured())[0]["amounts"][0][COSTCENTRE_FIELD]
    );
}
