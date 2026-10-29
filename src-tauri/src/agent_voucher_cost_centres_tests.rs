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
fn an_empty_container_is_present_but_not_observed_never_silence() {
    // No capture shows an entry with an empty container; the one on record (the
    // import-verification read, whose fetch does not ask for the fields) stood for a
    // real allocation. So it is not read as "no allocations".
    let xml = captured();
    let first = xml.find("<CATEGORYALLOCATIONS.LIST>").unwrap();
    let close = "</CATEGORYALLOCATIONS.LIST>";
    let last = xml.find(close).unwrap() + close.len();
    for empty in [
        "<CATEGORYALLOCATIONS.LIST>      </CATEGORYALLOCATIONS.LIST>",
        "<CATEGORYALLOCATIONS.LIST/>",
    ] {
        let emptied = format!("{}{}{}", &xml[..first], empty, &xml[last..]);
        assert_unreadable(&emptied, "cost_category_present_but_empty");
    }
}

fn first_category_block(xml: &str) -> (usize, usize) {
    let first = xml.find("<CATEGORYALLOCATIONS.LIST>").unwrap();
    let close = "</CATEGORYALLOCATIONS.LIST>";
    (first, xml.find(close).unwrap() + close.len())
}

#[test]
fn two_categories_on_one_entry_and_one_centre_name_under_two_categories_are_read() {
    let xml = captured();
    let (first, end) = first_category_block(&xml);
    let block = &xml[first..end];
    let second = block.replace("Business Line", "Other Line");
    let doubled = format!("{}{}{}{}", &xml[..end], &second, "", &xml[end..]);
    let rows = parse(&doubled);
    let entry = &rows[0]["amounts"][0];
    assert!(entry.get(UNREADABLE_FIELD).is_none(), "{entry}");
    // The same centre names under another category are not duplicates.
    assert_eq!(
        entry[COSTCENTRE_FIELD],
        json!([
            {"category": "Business Line", "cost_centre": "Trading", "amount": "-600.00"},
            {"category": "Business Line", "cost_centre": "Assembly", "amount": "-400.00"},
            {"category": "Other Line", "cost_centre": "Trading", "amount": "-600.00"},
            {"category": "Other Line", "cost_centre": "Assembly", "amount": "-400.00"},
        ])
    );
}

#[test]
fn a_second_entry_that_carries_allocations_is_read_on_its_own() {
    let xml = captured();
    let (first, end) = first_category_block(&xml);
    let block = xml[first..end]
        .replace("Trading", "Retail")
        .replace("Assembly", "Repair");
    let close = "</ALLLEDGERENTRIES.LIST>";
    let second_close = xml.rfind(close).unwrap();
    let both = format!("{}{}{}", &xml[..second_close], block, &xml[second_close..]);
    let rows = parse(&both);
    assert_eq!(
        rows[0]["amounts"][0][COSTCENTRE_FIELD][0]["cost_centre"],
        "Trading"
    );
    assert_eq!(
        rows[0]["amounts"][1][COSTCENTRE_FIELD],
        json!([
            {"category": "Business Line", "cost_centre": "Retail", "amount": "-600.00"},
            {"category": "Business Line", "cost_centre": "Repair", "amount": "-400.00"},
        ])
    );
    assert!(rows[0]["amounts"][1].get(UNREADABLE_FIELD).is_none());
}

#[test]
fn a_name_that_cannot_be_decoded_is_unreadable_and_a_cdata_name_is_read() {
    assert_unreadable(
        &changed("<NAME>Trading</NAME>", "<NAME>Trad&bogus;ing</NAME>"),
        "cost_centre_text_undecodable",
    );
    let rows = parse(&changed(
        "<NAME>Trading</NAME>",
        "<NAME><![CDATA[Trading]]></NAME>",
    ));
    assert_eq!(
        rows[0]["amounts"][0][COSTCENTRE_FIELD],
        parse(&captured())[0]["amounts"][0][COSTCENTRE_FIELD]
    );
}

#[test]
fn names_are_trimmed_and_the_amount_is_exactly_as_sent() {
    let rows = parse(&changed(
        "<NAME>Trading</NAME>",
        "<NAME>  Trading \r\n</NAME>",
    ));
    assert_eq!(
        rows[0]["amounts"][0][COSTCENTRE_FIELD][0]["cost_centre"],
        "Trading"
    );
    // The amount is not trimmed: a padded one is refused as the entry's own amount
    // would be, so what is carried is exactly what Tally sent.
    assert_unreadable(
        &changed("<AMOUNT>-600.00</AMOUNT>", "<AMOUNT> -600.00</AMOUNT>"),
        "cost_centre_amount_unparsable",
    );
}

#[test]
fn under_mask_parties_no_raw_cost_centre_or_category_name_is_left() {
    let rows = parse(&captured());
    let marked: Vec<Value> = rows.into_iter().map(mark_voucher_party_names).collect();
    let masked = redact_value(Value::Array(marked.clone()), Redaction::MaskParties).to_string();
    for name in ["Trading", "Assembly", "Business Line"] {
        assert!(!masked.contains(name), "{name} left in {masked}");
    }
    // The amounts are not names and stay.
    assert!(masked.contains("-600.00") && masked.contains("-400.00"));
    // Unmasked, the names are there.
    let plain = redact_value(Value::Array(marked), Redaction::None).to_string();
    assert!(plain.contains("Trading") && plain.contains("Business Line"));
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

#[test]
fn a_category_whose_name_follows_its_centres_is_read_with_its_name() {
    // The capture prints CATEGORY first; Tally's order is not a rule this read relies on.
    let xml = captured();
    let named = "<CATEGORY>Business Line</CATEGORY>";
    assert_eq!(xml.matches(named).count(), 1);
    let close = "</CATEGORYALLOCATIONS.LIST>";
    let moved = xml
        .replacen(named, "", 1)
        .replacen(close, &format!("{named}{close}"), 1);
    let rows = parse(&moved);
    assert_eq!(
        rows[0]["amounts"][0][COSTCENTRE_FIELD],
        parse(&captured())[0]["amounts"][0][COSTCENTRE_FIELD]
    );
    assert!(rows[0]["amounts"][0].get(UNREADABLE_FIELD).is_none());
    // And with the name never printed, the list is withheld with its reason, not sent
    // with an empty category.
    let nameless = xml.replacen(named, "", 1);
    assert_unreadable(&nameless, "cost_category_missing");
}

#[test]
fn two_categories_that_name_themselves_after_their_centres_do_not_collide() {
    let xml = captured();
    let named = "<CATEGORY>Business Line</CATEGORY>";
    let first = xml.find("<CATEGORYALLOCATIONS.LIST>").unwrap();
    let close = "</CATEGORYALLOCATIONS.LIST>";
    let end = xml.find(close).unwrap() + close.len();
    let block = xml[first..end].to_string();
    let after = |name: &str| {
        block.replacen(named, "", 1).replacen(
            close,
            &format!("<CATEGORY>{name}</CATEGORY>{close}"),
            1,
        )
    };
    let doubled = format!(
        "{}{}{}{}",
        &xml[..first],
        after("Line A"),
        after("Line B"),
        &xml[end..]
    );
    let rows = parse(&doubled);
    let entry = &rows[0]["amounts"][0];
    assert!(entry.get(UNREADABLE_FIELD).is_none(), "{entry}");
    let categories: Vec<_> = entry[COSTCENTRE_FIELD]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["category"].as_str().unwrap(),
                row["cost_centre"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        categories,
        [
            ("Line A", "Trading"),
            ("Line A", "Assembly"),
            ("Line B", "Trading"),
            ("Line B", "Assembly"),
        ]
    );
}

#[test]
fn a_category_met_twice_in_one_entry_is_unreadable_not_returned_twice() {
    let xml = captured();
    let (first, end) = first_category_block(&xml);
    let doubled = format!("{}{}{}", &xml[..end], &xml[first..end], &xml[end..]);
    assert_unreadable(&doubled, "cost_category_repeated");
    // The names are compared without case, as Tally compares them.
    let block = xml[first..end].replace("Business Line", "BUSINESS LINE");
    let doubled = format!("{}{}{}", &xml[..end], block, &xml[end..]);
    assert_unreadable(&doubled, "cost_category_repeated");
}

#[test]
fn a_centre_name_repeated_in_another_case_in_one_category_is_unreadable() {
    assert_unreadable(
        &changed("<NAME>Assembly</NAME>", "<NAME>TRADING</NAME>"),
        "cost_centre_duplicate",
    );
}

#[test]
fn an_allocation_list_at_a_depth_the_reader_does_not_take_is_flagged_not_silent() {
    // Centres directly under the entry (no category), and a category nested in a category.
    let xml = captured();
    let (first, end) = first_category_block(&xml);
    let centres_only = "<COSTCENTREALLOCATIONS.LIST><NAME>Trading</NAME><AMOUNT>-5.00</AMOUNT></COSTCENTREALLOCATIONS.LIST>";
    let direct = format!("{}{}{}", &xml[..first], centres_only, &xml[end..]);
    assert_unreadable(&direct, "cost_centre_allocation_shape_unrecognised");
    let block = &xml[first..end];
    let nested = format!(
        "{}{}{}",
        &xml[..end - "</CATEGORYALLOCATIONS.LIST>".len()],
        block,
        &xml[end - "</CATEGORYALLOCATIONS.LIST>".len()..]
    );
    assert_unreadable(&nested, "cost_centre_allocation_shape_unrecognised");
}

/// An item invoice from the live sales-day capture (protocol reference §12a.13): the sales
/// ledger's entry holds the goods line (`INVENTORYALLOCATIONS.LIST`) as a child, and the
/// capture has no `ALLINVENTORYENTRIES.LIST`, which the read does not fetch. No captured
/// item invoice carries cost-centre allocations, so the cost-centre capture's own category
/// block is put at the two places Tally could print one, and what the read does with each
/// is pinned: beside the goods line it is read and returned; inside it, it is flagged.
#[test]
fn on_an_item_invoice_an_allocation_beside_the_goods_line_is_read_and_one_inside_it_is_flagged() {
    const INVOICE_COMPANY_GUID: &str = "1edb9b05-8d35-4c0c-9959-dce655730463";
    const GOODS_LINE: &str = "<INVENTORYALLOCATIONS.LIST>";
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/sales-day/register_window_taxed_sales_day_live.utf16le.xml"
    );
    let invoice = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    assert_eq!(
        invoice.matches(GOODS_LINE).count(),
        1,
        "one goods line to place beside"
    );
    let (from, to) = first_category_block(&captured());
    let block = captured()[from..to].to_string();
    let entries = |xml: &str| {
        let rows = parse_agent_rows(xml, INVOICE_COMPANY_GUID).expect("the read still succeeds");
        assert_eq!(rows.len(), 1);
        rows[0]["amounts"].as_array().unwrap().clone()
    };

    // As captured: four entries, and the sales ledger's carries no allocation key of either kind.
    let plain = entries(&invoice);
    assert_eq!(plain.len(), 4);
    assert_eq!(plain[1]["ledger"], "Lab Sales");
    for entry in &plain {
        assert!(entry.get(COSTCENTRE_FIELD).is_none() && entry.get(UNREADABLE_FIELD).is_none());
    }

    // Beside the goods line, a child of the sales ledger's entry: read and returned like any other.
    let beside = entries(&invoice.replacen(GOODS_LINE, &format!("{block}{GOODS_LINE}"), 1));
    assert_eq!(
        beside[1][COSTCENTRE_FIELD],
        json!([
            {"category": "Business Line", "cost_centre": "Trading", "amount": "-600.00"},
            {"category": "Business Line", "cost_centre": "Assembly", "amount": "-400.00"},
        ])
    );
    assert!(beside[1].get(UNREADABLE_FIELD).is_none());

    // Inside the goods line: not read, and not silent either.
    let inside = entries(&invoice.replacen(GOODS_LINE, &format!("{GOODS_LINE}{block}"), 1));
    assert_eq!(
        inside[1][UNREADABLE_FIELD],
        "cost_centre_allocation_shape_unrecognised"
    );
    assert!(inside[1].get(COSTCENTRE_FIELD).is_none());

    // Either way the other entries are as captured.
    for entries in [&beside, &inside] {
        for at in [0, 2, 3] {
            assert_eq!(entries[at], plain[at], "entry {at}");
        }
    }
}

/// #1231: the cost-centre summary names an entry's reason as its refusal's cause, through the wire
/// value: each reason is distinct and reads back as its own problem, and other text as none.
#[test]
fn each_unreadable_reason_reads_back_as_its_own_problem() {
    let reasons = CostCentreProblem::ALL.map(CostCentreProblem::reason);
    assert_eq!(
        reasons
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        reasons.len()
    );
    for problem in CostCentreProblem::ALL {
        assert_eq!(
            CostCentreProblem::from_reason(problem.reason()),
            Some(problem)
        );
    }
    assert_eq!(CostCentreProblem::from_reason("cost_centre"), None);
    assert_eq!(CostCentreProblem::from_reason(""), None);
}
