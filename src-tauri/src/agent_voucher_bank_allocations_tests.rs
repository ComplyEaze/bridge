//! Bank allocations on a voucher's ledger entries, on the live capture of five hand-keyed
//! Contras (protocol reference §8.2h). The other cases change one element of that capture;
//! none is authored from nothing.
use super::*;

const COMPANY_GUID: &str = "8fb5f4db-478d-4084-b1e5-5b8f5d3fef60";
const FIELD: &str = "bank_allocations";
const UNREADABLE_FIELD: &str = "bank_allocations_unreadable";

fn captured() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_contra_hand_keyed_bankref_lab_live.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn parse(xml: &str) -> Vec<Value> {
    parse_agent_rows(xml, COMPANY_GUID).expect("the read still succeeds")
}

/// The row of the Contra with this voucher number.
fn voucher<'a>(rows: &'a [Value], number: &str) -> &'a Value {
    rows.iter()
        .find(|row| row["voucher_number"] == number)
        .unwrap_or_else(|| panic!("the capture holds voucher {number}"))
}

/// A field Tally sent with a value.
fn value(text: &str) -> Value {
    json!({"state": "value", "value": text})
}

/// A field Tally sent empty.
fn empty() -> Value {
    json!({"state": "empty"})
}

/// One listed allocation, by its five fields.
fn allocation(
    bank_date: Value,
    instrument_number: Value,
    instrument_date: Value,
    transaction_type: Value,
    status: Value,
) -> Value {
    json!({
        "bank_date": bank_date,
        "instrument_number": instrument_number,
        "instrument_date": instrument_date,
        "transaction_type": transaction_type,
        "status": status,
    })
}

#[test]
fn the_captured_contras_carry_each_field_as_tally_sent_it() {
    let rows = parse(&captured());
    assert_eq!(rows.len(), 5);
    // C2: a reference keyed only in the bank allocation, on both entries. Nothing was
    // reconciled: the bank date is empty, which says nothing more than that.
    for side in 0..2 {
        assert_eq!(
            voucher(&rows, "2")["amounts"][side][FIELD],
            json!([allocation(
                empty(),
                value("222222222222"),
                value("20260403"),
                value("Same Bank Transfer"),
                value("No"),
            )])
        );
    }
    // C5: a cheque number.
    assert_eq!(
        voucher(&rows, "5")["amounts"][0][FIELD],
        json!([allocation(
            empty(),
            value("123456"),
            value("20260405"),
            value("Cheque"),
            value("No"),
        )])
    );
    // C1: no number was keyed, only the type Tally offered on every bank entry.
    assert_eq!(
        voucher(&rows, "1")["amounts"][1][FIELD],
        json!([allocation(
            empty(),
            empty(),
            value("20260403"),
            value("Cheque/DD"),
            value("No"),
        )])
    );
    // Every bank entry of the capture carries one allocation, and none is unreadable.
    for row in &rows {
        for entry in row["amounts"].as_array().unwrap() {
            assert_eq!(entry[FIELD].as_array().map(Vec::len), Some(1), "{entry}");
            assert!(entry.get(UNREADABLE_FIELD).is_none(), "{entry}");
        }
    }
}

#[test]
fn the_existing_fields_of_an_entry_are_as_they_were() {
    let rows = parse(&captured());
    let entry = &voucher(&rows, "2")["amounts"][0];
    assert_eq!(entry["ledger"], "TLab Bank A");
    assert_eq!(entry["amount"], "50000.00");
    assert_eq!(entry["bill_allocations"], json!([]));
}

/// The capture with voucher 2's first allocation changed by `edit`.
fn with_first_allocation_of_voucher_2(edit: impl Fn(&str) -> String) -> Vec<Value> {
    let xml = captured();
    let at = xml.find("<INSTRUMENTNUMBER>222222222222").unwrap();
    let start = xml[..at].rfind("<BANKALLOCATIONS.LIST>").unwrap();
    let end = xml[start..].find("</BANKALLOCATIONS.LIST>").unwrap()
        + start
        + "</BANKALLOCATIONS.LIST>".len();
    let block = &xml[start..end];
    let edited = edit(block);
    assert_ne!(edited, block, "the capture was changed");
    parse(&format!("{}{edited}{}", &xml[..start], &xml[end..]))
}

fn first_entry_of_voucher_2(rows: &[Value]) -> &Value {
    &voucher(rows, "2")["amounts"][0]
}

#[test]
fn an_empty_field_is_empty_and_a_missing_one_is_absent_never_an_empty_string() {
    let rows = with_first_allocation_of_voucher_2(|block| {
        block.replace(
            "<INSTRUMENTNUMBER>222222222222</INSTRUMENTNUMBER>",
            "<INSTRUMENTNUMBER/>",
        )
    });
    assert_eq!(
        first_entry_of_voucher_2(&rows)[FIELD][0]["instrument_number"],
        empty()
    );
    let rows = with_first_allocation_of_voucher_2(|block| {
        block
            .replace("<BANKERSDATE/>", "")
            .replace("<STATUS>No</STATUS>", "")
    });
    let listed = &first_entry_of_voucher_2(&rows)[FIELD][0];
    assert_eq!(listed["bank_date"], json!({"state": "absent"}));
    assert_eq!(listed["status"], json!({"state": "absent"}));
    assert_eq!(listed["instrument_number"], value("222222222222"));
}

#[test]
fn a_bank_date_is_listed_as_sent_and_says_nothing_more() {
    // Edited capture: a bank date put on an entry nothing reconciled. No reconciled entry has
    // been captured; this pins only that a date is carried as Tally sends it.
    let rows = with_first_allocation_of_voucher_2(|block| {
        block.replace("<BANKERSDATE/>", "<BANKERSDATE>20260406</BANKERSDATE>")
    });
    let listed = &first_entry_of_voucher_2(&rows)[FIELD][0];
    assert_eq!(listed["bank_date"], value("20260406"));
    assert_eq!(listed["status"], value("No"));
    assert!(listed.get("reconciled").is_none(), "{listed}");
}

#[test]
fn a_date_that_is_not_a_date_leaves_a_reason_and_no_partial_list() {
    for (from, to) in [
        ("<BANKERSDATE/>", "<BANKERSDATE>6-Apr-2026</BANKERSDATE>"),
        (
            "<INSTRUMENTDATE>20260403</INSTRUMENTDATE>",
            "<INSTRUMENTDATE>20261332</INSTRUMENTDATE>",
        ),
    ] {
        let rows = with_first_allocation_of_voucher_2(|block| block.replace(from, to));
        let entry = first_entry_of_voucher_2(&rows);
        assert_eq!(
            entry[UNREADABLE_FIELD], "bank_allocation_date_unparsable",
            "{to}"
        );
        assert!(entry.get(FIELD).is_none(), "{entry}");
    }
}

#[test]
fn an_allocation_element_with_no_children_lists_nothing() {
    let rows = with_first_allocation_of_voucher_2(|_| {
        "<BANKALLOCATIONS.LIST>      </BANKALLOCATIONS.LIST>".to_string()
    });
    let entry = first_entry_of_voucher_2(&rows);
    assert!(entry.get(FIELD).is_none(), "{entry}");
    assert!(entry.get(UNREADABLE_FIELD).is_none(), "{entry}");
}

#[test]
fn a_repeated_field_in_one_allocation_leaves_a_reason_and_no_partial_list() {
    let old = "<INSTRUMENTNUMBER>123456</INSTRUMENTNUMBER>";
    let xml = captured();
    assert!(xml.contains(old));
    let xml = xml.replacen(old, &format!("{old}{old}"), 1);
    let rows = parse(&xml);
    let entry = &voucher(&rows, "5")["amounts"][0];
    assert_eq!(entry[UNREADABLE_FIELD], "bank_allocation_field_duplicated");
    assert!(entry.get(FIELD).is_none(), "{entry}");
    // The other entry of the voucher is untouched.
    assert!(voucher(&rows, "5")["amounts"][1]
        .get(UNREADABLE_FIELD)
        .is_none());
}

#[test]
fn more_allocations_than_the_read_keeps_are_reported_not_truncated() {
    let rows = with_first_allocation_of_voucher_2(|block| block.repeat(9));
    let entry = first_entry_of_voucher_2(&rows);
    assert_eq!(entry[UNREADABLE_FIELD], "bank_allocations_too_many");
    assert!(entry.get(FIELD).is_none());
    // Eight are kept and listed.
    let rows = with_first_allocation_of_voucher_2(|block| block.repeat(8));
    assert_eq!(
        first_entry_of_voucher_2(&rows)[FIELD]
            .as_array()
            .unwrap()
            .len(),
        8
    );
}

#[test]
fn an_allocation_list_nested_anywhere_else_is_reported() {
    let xml = captured();
    let nested = xml.replacen(
        "<BANKALLOCATIONS.LIST>",
        "<BANKALLOCATIONS.LIST><BANKALLOCATIONS.LIST></BANKALLOCATIONS.LIST>",
        1,
    );
    assert_ne!(nested, xml);
    let rows = parse(&nested);
    let reasons = rows
        .iter()
        .flat_map(|row| row["amounts"].as_array().unwrap().iter())
        .filter_map(|entry| entry[UNREADABLE_FIELD].as_str())
        .collect::<Vec<_>>();
    assert_eq!(reasons, ["bank_allocation_shape_unrecognised"]);
}

/// The capture with the instrument number of voucher 2's first entry replaced.
fn with_number_markup(markup: &str) -> Vec<Value> {
    with_first_allocation_of_voucher_2(|block| {
        block.replace("<INSTRUMENTNUMBER>222222222222</INSTRUMENTNUMBER>", markup)
    })
}

#[test]
fn an_entity_in_the_number_is_decoded_and_the_whole_number_is_listed() {
    let rows = with_number_markup("<INSTRUMENTNUMBER>12&amp;34</INSTRUMENTNUMBER>");
    assert_eq!(
        first_entry_of_voucher_2(&rows)[FIELD][0]["instrument_number"],
        value("12&34")
    );
}

#[test]
fn a_cdata_number_is_read_whole() {
    let rows = with_number_markup("<INSTRUMENTNUMBER><![CDATA[90 0<1]]></INSTRUMENTNUMBER>");
    assert_eq!(
        first_entry_of_voucher_2(&rows)[FIELD][0]["instrument_number"],
        value("90 0<1")
    );
}

#[test]
fn a_number_that_cannot_be_decoded_is_unreadable_not_cut() {
    let rows = with_number_markup("<INSTRUMENTNUMBER>90&bogus;02</INSTRUMENTNUMBER>");
    let entry = first_entry_of_voucher_2(&rows);
    assert_eq!(entry[UNREADABLE_FIELD], "bank_allocation_text_undecodable");
    assert!(entry.get(FIELD).is_none(), "{entry}");
}

#[test]
fn an_empty_element_and_a_full_one_are_a_repeated_field_not_a_number() {
    let rows = with_number_markup("<INSTRUMENTNUMBER/><INSTRUMENTNUMBER>9</INSTRUMENTNUMBER>");
    let entry = first_entry_of_voucher_2(&rows);
    assert_eq!(entry[UNREADABLE_FIELD], "bank_allocation_field_duplicated");
    assert!(entry.get(FIELD).is_none(), "{entry}");
}

#[test]
fn an_element_inside_the_number_is_reported_not_listed_in_part() {
    for markup in [
        "<INSTRUMENTNUMBER>2222<X></X>22222222</INSTRUMENTNUMBER>",
        "<INSTRUMENTNUMBER>2222<X/>22222222</INSTRUMENTNUMBER>",
    ] {
        let rows = with_number_markup(markup);
        let entry = first_entry_of_voucher_2(&rows);
        assert_eq!(
            entry[UNREADABLE_FIELD], "bank_allocation_shape_unrecognised",
            "{markup}"
        );
        assert!(entry.get(FIELD).is_none(), "{entry}");
    }
}

#[test]
fn an_answer_shows_bank_allocations_only_when_asked() {
    let rows = parse(&captured());
    let row = voucher(&rows, "2").clone();
    let withheld = present_voucher_bank_allocations(row.clone(), BankAllocationsShown::Withheld);
    for entry in withheld["amounts"].as_array().unwrap() {
        assert!(entry.get(FIELD).is_none(), "{entry}");
        assert!(entry["ledger"].is_string(), "{entry}");
    }
    let shown = present_voucher_bank_allocations(row.clone(), BankAllocationsShown::Shown);
    assert_eq!(shown, row);
    // An unreadable list is withheld the same way.
    let mut unreadable = row;
    unreadable["amounts"][0].as_object_mut().unwrap().insert(
        UNREADABLE_FIELD.to_string(),
        json!("bank_allocations_too_many"),
    );
    let withheld = present_voucher_bank_allocations(unreadable, BankAllocationsShown::Withheld);
    assert!(withheld["amounts"][0].get(UNREADABLE_FIELD).is_none());
}

#[test]
fn the_argument_is_a_real_boolean_and_withholds_by_default() {
    assert_eq!(
        BankAllocationsShown::from_args(&json!({})),
        Ok(BankAllocationsShown::Withheld)
    );
    assert_eq!(
        BankAllocationsShown::from_args(&json!({"bank_allocations": false})),
        Ok(BankAllocationsShown::Withheld)
    );
    assert_eq!(
        BankAllocationsShown::from_args(&json!({"bank_allocations": true})),
        Ok(BankAllocationsShown::Shown)
    );
    for invalid in [json!("true"), json!(1)] {
        assert_eq!(
            BankAllocationsShown::from_args(&json!({"bank_allocations": invalid})),
            Err("argument_invalid:bank_allocations".to_string())
        );
    }
}
