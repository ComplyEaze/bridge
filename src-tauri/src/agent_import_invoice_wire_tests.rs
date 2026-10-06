use super::*;

/// The shape of the lab's voucher-type collection (measured 6 Oct 2026): a
/// CMPINFO counter named like the rows, then one VOUCHERTYPE per type with its
/// reserved name as an attribute and the series-level numbering inside
/// VOUCHERNUMBERSERIES.LIST. Names and GUIDs are synthetic.
fn types_xml(extra: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO><COMPANY>0</COMPANY><VOUCHERTYPE>0</VOUCHERTYPE></CMPINFO></DESC><DATA><COLLECTION ISMODIFY=\"No\">\
<VOUCHERTYPE NAME=\"Sales\" RESERVEDNAME=\"Sales\"><GUID>g-sales</GUID><PARENT>Sales</PARENT><NUMBERINGMETHOD>Automatic (Manual Override)</NUMBERINGMETHOD>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Sales Acc\" RESERVEDNAME=\"\"><GUID>g-acc</GUID><PARENT>Sales</PARENT><NUMBERINGMETHOD>None</NUMBERINGMETHOD>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Attendance\" RESERVEDNAME=\"Attendance\"><GUID>g-att</GUID><PARENT>Attendance</PARENT>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Sales Auto\" RESERVEDNAME=\"\"><GUID>g-auto</GUID><PARENT>Sales</PARENT>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Automatic (Manual Override)</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Sales Two\" RESERVEDNAME=\"\"><GUID>g-two</GUID><PARENT>Sales</PARENT>\
<VOUCHERNUMBERSERIES.LIST><NAME>A</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST>\
<VOUCHERNUMBERSERIES.LIST><NAME>B</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Decoy\" RESERVEDNAME=\"\"><GUID>g-decoy</GUID><PARENT>Attendance</PARENT>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
{extra}</COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

#[test]
fn the_cmpinfo_counter_is_not_a_row_and_each_type_is_read_with_its_series() {
    let types = parse_voucher_types(&types_xml("")).unwrap();
    assert_eq!(types.len(), 6);
    let acc = types.iter().find(|row| row.name == "Sales Acc").unwrap();
    assert_eq!(
        acc,
        &VoucherTypeRow {
            name: "Sales Acc".into(),
            guid: "g-acc".into(),
            reserved_name: String::new(),
            parent: "Sales".into(),
            series: vec![("Default".into(), "Manual".into())],
        }
    );
}

#[test]
fn a_named_type_resolves_by_its_parent_chain_and_its_series_not_by_its_name() {
    let types = parse_voucher_types(&types_xml("")).unwrap();
    let resolve = |name| resolve_voucher_type(&types, name, "Sales");
    assert_eq!(resolve("Sales Acc"), Ok(ResolvedVoucherType { guid: "g-acc".into(), class: "Sales".into() }));
    assert_eq!(resolve("Sales").unwrap().guid, "g-sales", "the predefined type: its series is Manual though its top level says otherwise");
    assert_eq!(resolve("Sales Auto"), Err("invoice_voucher_type_numbering_not_manual"));
    assert_eq!(resolve("Sales Two"), Err("invoice_voucher_type_several_series"));
    assert_eq!(resolve("Decoy"), Err("invoice_voucher_type_wrong_class"));
    assert_eq!(resolve("Attendance"), Err("invoice_voucher_type_wrong_class"));
    assert_eq!(resolve("Sales Acx"), Err("invoice_voucher_type_not_found"));
    assert_eq!(resolve_voucher_type(&types, "Sales Acc", "Purchase"), Err("invoice_voucher_type_wrong_class"));
}

#[test]
fn a_name_two_types_carry_and_a_broken_chain_are_refused() {
    let twin = "<VOUCHERTYPE NAME=\"Sales Acc\" RESERVEDNAME=\"\"><GUID>g-twin</GUID><PARENT>Sales</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>";
    let types = parse_voucher_types(&types_xml(twin)).unwrap();
    assert_eq!(resolve_voucher_type(&types, "Sales Acc", "Sales"), Err("invoice_voucher_type_name_ambiguous"));
    let orphan = "<VOUCHERTYPE NAME=\"Orphan\" RESERVEDNAME=\"\"><GUID>g-o</GUID><PARENT>Nowhere</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>";
    let types = parse_voucher_types(&types_xml(orphan)).unwrap();
    assert_eq!(resolve_voucher_type(&types, "Orphan", "Sales"), Err("invoice_voucher_type_parent_missing"));
    let cycle = "<VOUCHERTYPE NAME=\"C1\" RESERVEDNAME=\"\"><GUID>g1</GUID><PARENT>C2</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>D</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE><VOUCHERTYPE NAME=\"C2\" RESERVEDNAME=\"\"><GUID>g2</GUID><PARENT>C1</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>D</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>";
    let types = parse_voucher_types(&types_xml(cycle)).unwrap();
    assert_eq!(resolve_voucher_type(&types, "C1", "Sales"), Err("invoice_voucher_type_chain_cycles"));
}

#[test]
fn a_malformed_answer_and_an_unreadable_series_are_errors_not_empty_lists() {
    assert_eq!(parse_voucher_types("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA>"), Err("invoice_read_malformed"));
    let no_guid = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHERTYPE NAME=\"X\" RESERVEDNAME=\"\"><PARENT>Sales</PARENT></VOUCHERTYPE></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(parse_voucher_types(no_guid), Err("invoice_voucher_type_row_without_guid"));
    let half = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHERTYPE NAME=\"X\" RESERVEDNAME=\"\"><GUID>g</GUID><PARENT>Sales</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(parse_voucher_types(half), Err("invoice_voucher_type_series_unreadable"));
}

#[test]
fn control_character_references_do_not_break_the_read() {
    // Tally writes &#4; for its reserved values; it is not legal XML.
    let xml = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHERTYPE NAME=\"T\" RESERVEDNAME=\"\"><GUID>g</GUID><PARENT>Sales</PARENT><TAXUNITNAME>&#4; Any</TAXUNITNAME><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(parse_voucher_types(xml).unwrap().len(), 1);
}

#[test]
fn an_answer_that_is_not_tallys_success_is_never_read_as_no_rows() {
    let ok = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(count_vouchers(ok), Ok(0));
    for bad in [
        "<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER><DATA><COLLECTION></COLLECTION></DATA></ENVELOPE>",
        "<ENVELOPE><HEADER></HEADER><DATA><COLLECTION></COLLECTION></DATA></ENVELOPE>",
        "<ENVELOPE><BODY><DATA><COLLECTION></COLLECTION></DATA></BODY></ENVELOPE>",
    ] {
        assert_eq!(count_vouchers(bad), Err("invoice_read_status_not_success"), "{bad}");
    }
    let twice = "<ENVELOPE><HEADER><STATUS>1</STATUS><STATUS>1</STATUS></HEADER><DATA><COLLECTION></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(count_vouchers(twice), Err("invoice_read_status_repeated"));
    let one = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHER><VOUCHERNUMBER>9</VOUCHERNUMBER></VOUCHER></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(count_vouchers(one), Ok(1));
}

#[test]
fn the_number_read_is_class_wide_dated_and_carries_only_closed_alphabet_literals() {
    let request = render_invoice_number_request("Co", "INV/26-27/0042", ("20250401", "20260331")).unwrap();
    assert!(request.contains("$VoucherNumber = \"INV/26-27/0042\" AND $$IsSales:$VoucherTypeName"));
    // SVFROMDATE and SVTODATE do not limit a collection: the window is a formula too.
    assert!(request.contains("$Date &gt;= $$Date:\"20250401\" AND $Date &lt;= $$Date:\"20260331\""));
    assert!(request.contains("<SVFROMDATE TYPE=\"Date\">20250401</SVFROMDATE><SVTODATE TYPE=\"Date\">20260331</SVTODATE>"));
    assert!(request.contains("<TYPE>Voucher</TYPE>") && !request.contains("IMPORTDATA"));
    // GST rule 46(b): 16 characters, letters, digits, hyphen, slash.
    for bad_number in ["A\"B", "A\\B", "", " A", "A B", "A_B", "A.B", "A$B", "x OR y; \"", "\u{e9}", "12345678901234567"] {
        assert!(render_invoice_number_request("Co", bad_number, ("20250401", "20260331")).is_none(), "{bad_number:?}");
    }
    assert!(render_invoice_number_request("Co", "1234567890123456", ("20250401", "20260331")).is_some());
    assert!(render_invoice_number_request("Co", "1", ("2025-04-01", "20260331")).is_none());
}

#[test]
fn the_readback_read_is_dated_and_by_type_guid_and_number() {
    let guid = "9da2ed7d-dc75-413d-9260-29deeef6eb8c-00000193";
    let request = render_invoice_readback_request("Co", guid, "278", ("20250401", "20260331")).unwrap();
    assert!(request.contains("$Date &gt;= $$Date:\"20250401\" AND $Date &lt;= $$Date:\"20260331\" AND $VoucherNumber = \"278\" AND $GUID:VoucherType:$VoucherTypeName = \""));
    assert!(request.contains("ALLLEDGERENTRIES.*"));
    assert!(render_invoice_readback_request("Co", "g\"", "278", ("20250401", "20260331")).is_none());
    assert!(render_invoice_readback_request("Co", guid, "A B", ("20250401", "20260331")).is_none());
}

#[test]
fn the_company_state_is_taken_from_the_row_with_the_verified_guid() {
    let request = render_company_state_request("Co & Sons");
    assert!(request.contains("<TYPE>Company</TYPE>") && request.contains("Co &amp; Sons"));
    let row = |guid: &str, state: &str| format!("<COMPANY NAME=\"X\"><GUID>{guid}</GUID><STATENAME>{state}</STATENAME></COMPANY>");
    let wrap = |rows: String| format!("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION>{rows}</COLLECTION></DATA></ENVELOPE>");
    // Another loaded company's row comes first and must not be taken.
    let two = wrap(row("other", "Haryana") + &row("G-1", "Rajasthan"));
    assert_eq!(parse_company_state(&two, "g-1"), Ok("Rajasthan".to_string()));
    assert_eq!(parse_company_state(&wrap(row("other", "Haryana")), "g-1"), Err("invoice_company_row_missing"));
    assert_eq!(parse_company_state(&wrap(row("g-1", "")), "g-1"), Err("invoice_company_state_unreadable"));
    assert_eq!(parse_company_state(&wrap(row("g-1", "Rajastan")), "g-1"), Err("invoice_company_state_unreadable"));
    assert_eq!(
        parse_company_state(&wrap(row("g-1", "Rajasthan") + &row("G-1", "Rajasthan")), "g-1"),
        Err("invoice_company_row_repeated")
    );
}
