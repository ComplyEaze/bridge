use super::*;

/// The shape of the lab's voucher-type collection (measured 6 Oct 2026): a
/// CMPINFO counter named like the rows, then one VOUCHERTYPE per type with its
/// reserved name as an attribute and the series-level numbering inside
/// VOUCHERNUMBERSERIES.LIST. Names and GUIDs are synthetic.
fn types_xml(extra: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO><COMPANY>0</COMPANY><VOUCHERTYPE>0</VOUCHERTYPE></CMPINFO></DESC><DATA><COLLECTION ISMODIFY=\"No\">\
<VOUCHERTYPE NAME=\"Sales\" RESERVEDNAME=\"Sales\"><GUID>5a1e5-01</GUID><PARENT>Sales</PARENT><NUMBERINGMETHOD>Automatic (Manual Override)</NUMBERINGMETHOD>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Sales Acc\" RESERVEDNAME=\"\"><GUID>acc0-02</GUID><PARENT>Sales</PARENT><NUMBERINGMETHOD>None</NUMBERINGMETHOD>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Attendance\" RESERVEDNAME=\"Attendance\"><GUID>a77e-03</GUID><PARENT>Attendance</PARENT>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Sales Auto\" RESERVEDNAME=\"\"><GUID>a070-04</GUID><PARENT>Sales</PARENT>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Automatic (Manual Override)</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Sales Two\" RESERVEDNAME=\"\"><GUID>7770-05</GUID><PARENT>Sales</PARENT>\
<VOUCHERNUMBERSERIES.LIST><NAME>A</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST>\
<VOUCHERNUMBERSERIES.LIST><NAME>B</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Decoy\" RESERVEDNAME=\"\"><GUID>dec0-06</GUID><PARENT>Attendance</PARENT>\
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
            guid: "acc0-02".into(),
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
    assert_eq!(
        resolve("Sales Acc"),
        Ok(ResolvedVoucherType {
            guid: "acc0-02".into(),
            class: "Sales".into()
        })
    );
    assert_eq!(
        resolve("Sales").unwrap().guid,
        "5a1e5-01",
        "the predefined type: its series is Manual though its top level says otherwise"
    );
    assert_eq!(
        resolve("Sales Auto"),
        Err("invoice_voucher_type_numbering_not_manual")
    );
    assert_eq!(
        resolve("Sales Two"),
        Err("invoice_voucher_type_several_series")
    );
    assert_eq!(resolve("Decoy"), Err("invoice_voucher_type_wrong_class"));
    assert_eq!(
        resolve("Attendance"),
        Err("invoice_voucher_type_wrong_class")
    );
    assert_eq!(resolve("Sales Acx"), Err("invoice_voucher_type_not_found"));
    assert_eq!(
        resolve_voucher_type(&types, "Sales Acc", "Purchase"),
        Err("invoice_voucher_type_wrong_class")
    );
}

#[test]
fn a_name_two_types_carry_and_a_broken_chain_are_refused() {
    let twin = "<VOUCHERTYPE NAME=\"Sales Acc\" RESERVEDNAME=\"\"><GUID>7717-07</GUID><PARENT>Sales</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>";
    let types = parse_voucher_types(&types_xml(twin)).unwrap();
    assert_eq!(
        resolve_voucher_type(&types, "Sales Acc", "Sales"),
        Err("invoice_voucher_type_name_ambiguous")
    );
    let orphan = "<VOUCHERTYPE NAME=\"Orphan\" RESERVEDNAME=\"\"><GUID>0f-08</GUID><PARENT>Nowhere</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>";
    let types = parse_voucher_types(&types_xml(orphan)).unwrap();
    assert_eq!(
        resolve_voucher_type(&types, "Orphan", "Sales"),
        Err("invoice_voucher_type_parent_missing")
    );
    let cycle = "<VOUCHERTYPE NAME=\"C1\" RESERVEDNAME=\"\"><GUID>c1-09</GUID><PARENT>C2</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>D</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE><VOUCHERTYPE NAME=\"C2\" RESERVEDNAME=\"\"><GUID>c2-0a</GUID><PARENT>C1</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>D</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>";
    let types = parse_voucher_types(&types_xml(cycle)).unwrap();
    assert_eq!(
        resolve_voucher_type(&types, "C1", "Sales"),
        Err("invoice_voucher_type_chain_cycles")
    );
}

#[test]
fn a_malformed_answer_and_an_unreadable_series_are_errors_not_empty_lists() {
    assert_eq!(
        parse_voucher_types("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA>"),
        Err("invoice_read_malformed")
    );
    let no_guid = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHERTYPE NAME=\"X\" RESERVEDNAME=\"\"><PARENT>Sales</PARENT></VOUCHERTYPE></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(
        parse_voucher_types(no_guid),
        Err("invoice_voucher_type_row_without_guid")
    );
    let half = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHERTYPE NAME=\"X\" RESERVEDNAME=\"\"><GUID>0a-0b</GUID><PARENT>Sales</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(
        parse_voucher_types(half),
        Err("invoice_voucher_type_series_unreadable")
    );
}

#[test]
fn control_character_references_do_not_break_the_read() {
    // Tally writes &#4; for its reserved values; it is not legal XML.
    let xml = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHERTYPE NAME=\"T\" RESERVEDNAME=\"\"><GUID>0a-0b</GUID><PARENT>Sales</PARENT><TAXUNITNAME>&#4; Any</TAXUNITNAME><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE></COLLECTION></DATA></ENVELOPE>";
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
    // "No rows" needs the collection itself: a success envelope without one is
    // another shape, and the duplicate-number check must not read it as "not in use".
    for no_collection in [
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA></DATA></ENVELOPE>",
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><VOUCHER/></DATA></BODY></ENVELOPE>",
    ] {
        assert_eq!(count_vouchers(no_collection), Err("invoice_read_collection_absent"), "{no_collection}");
        assert_eq!(count_sales_vouchers(no_collection), Err("invoice_read_collection_absent"));
    }
    // A row written as an empty element is a row with no fields: it is never
    // "no rows", and the class count refuses it.
    let empty_row = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION><VOUCHER/></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(count_vouchers(empty_row), Ok(1));
    assert_eq!(
        count_sales_vouchers(empty_row),
        Err("invoice_number_class_unread")
    );
    // A collection written as an empty element is still a collection.
    let self_closed =
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION/></DATA></ENVELOPE>";
    assert_eq!(count_vouchers(self_closed), Ok(0));
}

#[test]
fn a_type_whose_guid_cannot_go_into_the_read_back_formula_is_refused_at_the_build() {
    let odd = "<VOUCHERTYPE NAME=\"Sales Odd\" RESERVEDNAME=\"\"><GUID>not \"hex\"</GUID><PARENT>Sales</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>";
    let types = parse_voucher_types(&types_xml(odd)).unwrap();
    assert_eq!(
        resolve_voucher_type(&types, "Sales Odd", "Sales"),
        Err("invoice_voucher_type_guid_unusable")
    );
    // Another type's odd GUID does not stop the named one.
    assert!(resolve_voucher_type(&types, "Sales Acc", "Sales").is_ok());
}

#[test]
fn the_number_read_is_dated_by_literals_and_leaves_the_class_to_the_reader() {
    let request =
        render_invoice_number_request("Co", "INV/26-27/0042", ("20250401", "20260331")).unwrap();
    assert!(
        request.contains("$VoucherNumber = \"INV/26-27/0042\"</SYSTEM>"),
        "the number is the only literal beside the dates"
    );
    // SVFROMDATE and SVTODATE do not limit a collection: the window is a formula too.
    assert!(request.contains("$Date &gt;= $$Date:\"20250401\" AND $Date &lt;= $$Date:\"20260331\""));
    assert!(request.contains("<COMPUTE>BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName</COMPUTE>"));
    assert!(
        !request.contains("AND $$IsSales"),
        "the class is a compute, not an unmeasured filter"
    );
    assert!(request.contains("<TYPE>Voucher</TYPE>") && !request.contains("IMPORTDATA"));
    // GST rule 46(b): 16 characters, letters, digits, hyphen, slash.
    for bad_number in [
        "A\"B",
        "A\\B",
        "",
        " A",
        "A B",
        "A_B",
        "A.B",
        "A$B",
        "x OR y; \"",
        "\u{e9}",
        "12345678901234567",
    ] {
        assert!(
            render_invoice_number_request("Co", bad_number, ("20250401", "20260331")).is_none(),
            "{bad_number:?}"
        );
    }
    assert!(
        render_invoice_number_request("Co", "1234567890123456", ("20250401", "20260331")).is_some()
    );
    assert!(render_invoice_number_request("Co", "1", ("2025-04-01", "20260331")).is_none());
}

#[test]
fn only_a_sales_class_row_with_a_readable_class_counts_as_a_duplicate() {
    let wrap = |rows: &str| {
        format!("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION>{rows}</COLLECTION></DATA></ENVELOPE>")
    };
    let row = |class: &str| {
        format!("<VOUCHER><VOUCHERNUMBER>9</VOUCHERNUMBER><BRIDGEVCHISSALES>{class}</BRIDGEVCHISSALES></VOUCHER>")
    };
    assert_eq!(count_sales_vouchers(&wrap("")), Ok(0));
    assert_eq!(count_sales_vouchers(&wrap(&row("Yes"))), Ok(1));
    // The same number under a Purchase-class type is not an invoice number clash.
    assert_eq!(count_sales_vouchers(&wrap(&row("No"))), Ok(0));
    assert_eq!(
        count_sales_vouchers(&wrap(&(row("Yes") + &row("Yes") + &row("No")))),
        Ok(2)
    );
    // A row whose class did not come back is an error, never a No.
    assert_eq!(
        count_sales_vouchers(&wrap("<VOUCHER><VOUCHERNUMBER>9</VOUCHERNUMBER></VOUCHER>")),
        Err("invoice_number_class_unread")
    );
    assert_eq!(
        count_sales_vouchers(&wrap(&row("Maybe"))),
        Err("invoice_number_class_unread")
    );
    // And an answer that is not Tally's success is never "no rows".
    assert_eq!(
        count_sales_vouchers("<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER><DATA><COLLECTION></COLLECTION></DATA></ENVELOPE>"),
        Err("invoice_read_status_not_success")
    );
}

#[test]
fn the_readback_read_is_dated_and_by_type_guid_and_number() {
    let guid = "11111111-2222-4333-8444-555555555555-00000193";
    let request =
        render_invoice_readback_request("Co", guid, "278", ("20250401", "20260331")).unwrap();
    assert!(request.contains("$Date &gt;= $$Date:\"20250401\" AND $Date &lt;= $$Date:\"20260331\" AND $VoucherNumber = \"278\" AND $GUID:VoucherType:$VoucherTypeName = \"11111111-2222-4333-8444-555555555555-00000193\"</SYSTEM>"));
    assert!(request.contains("ALLLEDGERENTRIES.*"));
    assert!(
        render_invoice_readback_request("Co", "g\"", "278", ("20250401", "20260331")).is_none()
    );
    assert!(render_invoice_readback_request("Co", guid, "A B", ("20250401", "20260331")).is_none());
}

#[test]
fn the_company_state_is_taken_from_the_row_with_the_verified_guid() {
    let request = render_company_state_request("Co & Sons");
    assert!(request.contains("<TYPE>Company</TYPE>") && request.contains("Co &amp; Sons"));
    let row = |guid: &str, state: &str| {
        format!("<COMPANY NAME=\"X\"><GUID>{guid}</GUID><STATENAME>{state}</STATENAME></COMPANY>")
    };
    let wrap = |rows: String| {
        format!("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION>{rows}</COLLECTION></DATA></ENVELOPE>")
    };
    // Another loaded company's row comes first and must not be taken.
    let two = wrap(row("other", "Haryana") + &row("G-1", "Rajasthan"));
    assert_eq!(
        parse_company_state(&two, "g-1"),
        Ok("Rajasthan".to_string())
    );
    assert_eq!(
        parse_company_state(&wrap(row("other", "Haryana")), "g-1"),
        Err("invoice_company_row_missing")
    );
    assert_eq!(
        parse_company_state(&wrap(row("g-1", "")), "g-1"),
        Err("invoice_company_state_unreadable")
    );
    assert_eq!(
        parse_company_state(&wrap(row("g-1", "Rajastan")), "g-1"),
        Err("invoice_company_state_unreadable")
    );
    assert_eq!(
        parse_company_state(
            &wrap(row("g-1", "Rajasthan") + &row("G-1", "Rajasthan")),
            "g-1"
        ),
        Err("invoice_company_row_repeated")
    );
}

/// One of the rehearsal's captured answers (see the PROVENANCE table beside
/// the fixtures): Tally's own bytes, decoded. In the two read-backs of the
/// registered customer's invoices one token, the customer's GSTIN, is a
/// substitute named in that table.
fn rehearsal(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The voucher types as the lab book answered on 7 Oct 2026: the type keyed
/// for the rehearsal resolves, the predefined Sales type (an Automatic series)
/// does not, and two raw control characters in a class name do not stop the
/// read.
#[test]
fn the_captured_voucher_types_resolve_the_manual_type_and_refuse_the_automatic_one() {
    let xml = rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-voucher-types.utf16le.xml"));
    assert_eq!(xml.matches('\u{5}').count(), 2);
    let types = parse_voucher_types(&xml).unwrap();
    assert_eq!(types.len(), 25);
    assert_eq!(
        resolve_voucher_type(&types, "Sales Manual", "Sales"),
        Ok(ResolvedVoucherType {
            guid: "ae1490be-52c5-4544-9ffc-4b7da85f9797-00000106".into(),
            class: "Sales".into(),
        })
    );
    let manual = types.iter().find(|row| row.name == "Sales Manual").unwrap();
    assert_eq!(
        (manual.parent.as_str(), manual.reserved_name.as_str()),
        ("Sales", "")
    );
    assert_eq!(
        manual.series,
        [("Default".to_string(), "Manual".to_string())]
    );
    assert_eq!(
        resolve_voucher_type(&types, "Sales", "Sales"),
        Err("invoice_voucher_type_numbering_not_manual")
    );
    assert_eq!(
        resolve_voucher_type(&types, "Sales Manual", "Purchase"),
        Err("invoice_voucher_type_wrong_class")
    );
}

/// The duplicate-number read's three captured answers: a number one invoice
/// carries, a number nothing carries (a collection with no row, which is an
/// answer), and a number a Purchase and a Sales voucher share.
#[test]
fn the_captured_number_answers_count_only_sales_class_vouchers() {
    for (name, vouchers, sales) in [
        ("number-known", 1, 1),
        ("number-absent", 0, 0),
        ("number-shared", 2, 1),
    ] {
        let xml = match name {
            "number-known" => rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-known.utf16le.xml")),
            "number-absent" => rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-absent.utf16le.xml")),
            _ => rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-shared.utf16le.xml")),
        };
        assert_eq!(count_vouchers(&xml), Ok(vouchers), "{name}");
        assert_eq!(count_sales_vouchers(&xml), Ok(sales), "{name}");
    }
}

/// The control beside the number read, on Tally's own answers: it passes only
/// on a row with the known number, its date and the Sales class. The shared
/// answer holds a Sales voucher numbered 12 on 18 Sep 2025 and a Purchase
/// voucher numbered 12 on 22 Jul 2025.
#[test]
fn the_control_row_is_the_known_invoice_with_its_date_and_class() {
    let known = rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-known.utf16le.xml"));
    let absent = rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-absent.utf16le.xml"));
    let shared = rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-shared.utf16le.xml"));
    assert_eq!(
        control_row_found(&known, "TG/25-26/001", "20260310"),
        Ok(true)
    );
    // A read that returns no row is never a control.
    assert_eq!(
        control_row_found(&absent, "TG/25-26/001", "20260310"),
        Ok(false)
    );
    assert_eq!(control_row_found(&shared, "12", "20250918"), Ok(true));
    // The Purchase row has the number and its own date, and is not Sales.
    assert_eq!(control_row_found(&shared, "12", "20250722"), Ok(false));
    // The number on another date; another number.
    assert_eq!(control_row_found(&shared, "12", "20250101"), Ok(false));
    assert_eq!(
        control_row_found(&shared, "TG/25-26/001", "20250918"),
        Ok(false)
    );
    // An answer with no collection is an error, as for the read it controls.
    assert_eq!(
        control_row_found(
            "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA></DATA></ENVELOPE>",
            "12",
            "20250918"
        ),
        Err("invoice_read_collection_absent")
    );
}

/// The read-back of the two invoices ComplyEaze Bridge posted and of one keyed
/// by hand, as Tally answered: every field the comparison reads comes back on
/// a posted invoice; a keyed one in this book carries no reference.
#[test]
fn the_captured_read_backs_carry_every_field_of_a_posted_invoice() {
    let field = |read: &ReadInvoice, key: &str| read.fields.get(key).cloned();
    let Ok(Readback::One(registered)) =
        parse_invoice_readback(&rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-readback-posted-registered.utf16le.xml")))
    else {
        panic!("one voucher");
    };
    for (key, value) in [
        ("DATE", "20260311"),
        ("VOUCHERNUMBER", "TG/25-26/002"),
        ("VOUCHERTYPENAME", "Sales Manual"),
        ("REFERENCE", "TG/25-26/002"),
        ("REFERENCEDATE", "20260311"),
        ("PARTYLEDGERNAME", "TG Buyer Regular RJ"),
        ("PARTYGSTIN", "08ZZZZZ0000Z1ZQ"),
        ("STATENAME", "Rajasthan"),
        ("PLACEOFSUPPLY", "Rajasthan"),
        ("GSTREGISTRATIONTYPE", "Regular"),
        ("ISINVOICE", "Yes"),
        ("ISCANCELLED", "No"),
        ("ISOPTIONAL", "No"),
        ("GUID", "ae1490be-52c5-4544-9ffc-4b7da85f9797-0000003e"),
        ("ALTERID", "64"),
    ] {
        assert_eq!(field(&registered, key).as_deref(), Some(value), "{key}");
    }
    let legs = registered
        .legs
        .iter()
        .map(|leg| {
            (
                leg.ledger.as_str(),
                leg.amount.as_str(),
                leg.allocations.len(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        legs,
        [
            ("TG Buyer Regular RJ", "-1457.00", 1),
            ("Sales - Goods", "1234.50", 0),
            ("Output CGST", "111.11", 0),
            ("Output SGST", "111.11", 0),
            ("Round Off", "0.28", 0),
        ]
    );
    assert_eq!(
        registered.legs[0].allocations,
        [(
            Some("TG/25-26/002".to_string()),
            Some("New Ref".to_string()),
            "-1457.00".to_string()
        )]
    );

    let Ok(Readback::One(unregistered)) =
        parse_invoice_readback(&rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-readback-posted-unregistered.utf16le.xml")))
    else {
        panic!("one voucher");
    };
    assert_eq!(
        field(&unregistered, "VOUCHERNUMBER").as_deref(),
        Some("TG/25-26/003")
    );
    assert_eq!(
        field(&unregistered, "GSTREGISTRATIONTYPE").as_deref(),
        Some("Unregistered/Consumer")
    );
    assert_eq!(field(&unregistered, "PARTYGSTIN"), None);
    assert_eq!(field(&unregistered, "ALTERID").as_deref(), Some("65"));
    assert_eq!(unregistered.legs.len(), 4);
    assert!(unregistered
        .legs
        .iter()
        .all(|leg| leg.allocations.is_empty()));

    let Ok(Readback::One(keyed)) = parse_invoice_readback(&rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-readback-keyed-registered.utf16le.xml")))
    else {
        panic!("one voucher");
    };
    assert_eq!(
        field(&keyed, "VOUCHERNUMBER").as_deref(),
        Some("TG/25-26/001")
    );
    assert_eq!(
        field(&keyed, "PARTYGSTIN").as_deref(),
        Some("08ZZZZZ0000Z1ZQ")
    );
    assert_eq!(
        (field(&keyed, "REFERENCE"), field(&keyed, "REFERENCEDATE")),
        (None, None)
    );
    assert_eq!(keyed.legs.len(), 4);
}
