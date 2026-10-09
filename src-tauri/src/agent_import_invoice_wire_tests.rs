use super::*;

/// The shape of the lab's voucher-type collection (measured 6 Oct 2026): a
/// CMPINFO counter named like the rows, then one VOUCHERTYPE per type with its
/// reserved name as an attribute and the series-level numbering inside
/// VOUCHERNUMBERSERIES.LIST. Names and GUIDs are synthetic.
fn types_xml(extra: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO><COMPANY>0</COMPANY><VOUCHERTYPE>0</VOUCHERTYPE></CMPINFO></DESC><DATA><COLLECTION ISMODIFY=\"No\">\
<VOUCHERTYPE NAME=\"Sales\" RESERVEDNAME=\"Sales\"><GUID>5a1e5-01</GUID><PARENT>Sales</PARENT><NUMBERINGMETHOD>Automatic (Manual Override)</NUMBERINGMETHOD>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD><PREVENTDUPLICATES>Yes</PREVENTDUPLICATES></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
<VOUCHERTYPE NAME=\"Sales Acc\" RESERVEDNAME=\"\"><GUID>acc0-02</GUID><PARENT>Sales</PARENT><NUMBERINGMETHOD>None</NUMBERINGMETHOD>\
<VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD><PREVENTDUPLICATES>Yes</PREVENTDUPLICATES></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
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
            series: vec![("Default".into(), "Manual".into(), Some("Yes".into()))],
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
    // A Manual series that lets a number repeat, or that does not say, is refused.
    for (guard, name) in [
        ("<PREVENTDUPLICATES>No</PREVENTDUPLICATES>", "Dup No"),
        ("", "Dup Unsaid"),
    ] {
        let row = format!("<VOUCHERTYPE NAME=\"{name}\" RESERVEDNAME=\"\"><GUID>d0-0c</GUID><PARENT>Sales</PARENT><VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>{guard}</VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>");
        let types = parse_voucher_types(&types_xml(&row)).unwrap();
        assert_eq!(
            resolve_voucher_type(&types, name, "Sales"),
            Err("invoice_voucher_type_duplicates_allowed"),
            "{name}"
        );
    }
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

/// The pilot lab's tax units as Tally answered (see the set's PROVENANCE).
const TAX_UNITS: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-tax-units-typed-request.utf8.xml"
);
const PILOT_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";
const REGISTRATION_ROW: &str =
    "<GSTREGISTRATIONDETAILS.LIST>\r\n      <FROMDATE>20260401</FROMDATE>";

/// The captured answer with `from` replaced by `to`, which must occur once.
fn units_with(from: &str, to: &str) -> String {
    assert_eq!(TAX_UNITS.matches(from).count(), 1, "{from}");
    TAX_UNITS.replacen(from, to, 1)
}

fn registration(xml: &str, as_of: &str) -> Result<CompanyRegistration, RegistrationOutcome> {
    parse_company_registration(xml, PILOT_GUID, as_of)
}

fn refused_as(xml: &str, as_of: &str) -> &'static str {
    match registration(xml, as_of) {
        Err(RegistrationOutcome::Refused(code)) => code,
        other => panic!("not refused: {other:?}"),
    }
}

#[test]
fn the_company_registration_in_force_is_read_from_the_captured_tax_units() {
    let request = render_company_registration_request("Co & Sons");
    assert!(request.contains("<TYPE>TaxUnit</TYPE>") && request.contains("Co &amp; Sons"));
    let read = Ok(CompanyRegistration {
        gstin: "08ZZZZZ0000Z1ZQ".into(),
        state: "Rajasthan".into(),
    });
    assert_eq!(registration(TAX_UNITS, "20260801"), read);
    // A row dated on the invoice date applies; a day earlier nothing does.
    assert_eq!(registration(TAX_UNITS, "20260401"), read);
    assert_eq!(
        refused_as(TAX_UNITS, "20260331"),
        "invoice_company_registration_not_yet_in_force"
    );
    // The company's GUID is matched without regard to case.
    assert_eq!(
        parse_company_registration(TAX_UNITS, &PILOT_GUID.to_ascii_uppercase(), "20260801"),
        read
    );
}

#[test]
fn a_registration_that_is_not_one_regular_unit_in_force_is_refused() {
    let cases = [
        (
            units_with("<REGISTRATIONTYPE>Regular", "<REGISTRATIONTYPE>Composition"),
            "invoice_company_registration_not_regular",
        ),
        (
            units_with("<ISINACTIVE>No", "<ISINACTIVE>Yes"),
            "invoice_company_registration_inactive",
        ),
        (
            units_with("<ISINACTIVE>No</ISINACTIVE>", ""),
            "invoice_company_registration_incomplete",
        ),
        (
            units_with("<STATE>Rajasthan</STATE>", "<STATE>Rajastan</STATE>"),
            "invoice_company_registration_incomplete",
        ),
        (
            units_with("<FROMDATE>20260401", "<FROMDATE>20260231"),
            "invoice_company_registration_incomplete",
        ),
        // The GSTIN's state code is Rajasthan's; the dated row names another.
        (
            units_with("<STATE>Rajasthan</STATE>", "<STATE>Haryana</STATE>"),
            "invoice_company_registration_inconsistent",
        ),
        (
            units_with("TAXREGISTRATION=\"08ZZZZZ0000Z1ZQ\"", "TAXREGISTRATION=\"08ZZZZZ0000Z2ZQ\""),
            "invoice_company_registration_inconsistent",
        ),
        (
            units_with("<USEDFOR>GST</USEDFOR>", "<USEDFOR/>"),
            "invoice_company_registration_inconsistent",
        ),
        // Two dated rows on one day that differ.
        (
            units_with(
                REGISTRATION_ROW,
                &format!("<GSTREGISTRATIONDETAILS.LIST><FROMDATE>20260401</FROMDATE><STATE>Rajasthan</STATE><REGISTRATIONTYPE>Composition</REGISTRATIONTYPE><ISINACTIVE>No</ISINACTIVE></GSTREGISTRATIONDETAILS.LIST>{REGISTRATION_ROW}"),
            ),
            "invoice_company_registration_inconsistent",
        ),
        // Each marker of a GST unit must agree with the others.
        (
            units_with("TAXTYPE=\"GST\"", "TAXTYPE=\"\""),
            "invoice_company_registration_inconsistent",
        ),
        (
            units_with("<GSTREGNUMBER>08ZZZZZ0000Z1ZQ</GSTREGNUMBER>", ""),
            "invoice_company_registration_inconsistent",
        ),
        // The GSTIN must be held twice, alike.
        (
            units_with(" TAXREGISTRATION=\"08ZZZZZ0000Z1ZQ\"", ""),
            "invoice_company_registration_inconsistent",
        ),
        // Another company's unit is never dropped from the count.
        (
            units_with("-00000064</GUID>", "</GUID>"),
            "invoice_company_registration_unbound",
        ),
        (
            units_with("bfef-d32e2ab93c85-000000cd", "bfef-d32e2ab93c86-000000cd"),
            "invoice_company_registration_unbound",
        ),
        (
            units_with("bfef-d32e2ab93c85-00000064", "bfef-d32e2ab93c86-00000064"),
            "invoice_company_registration_unbound",
        ),
    ];
    for (xml, code) in cases {
        assert_eq!(refused_as(&xml, "20260801"), code);
    }
    // A GSTIN that fails its check character, held alike in both places.
    assert_eq!(TAX_UNITS.matches("08ZZZZZ0000Z1ZQ").count(), 2);
    assert_eq!(
        refused_as(
            &TAX_UNITS.replace("08ZZZZZ0000Z1ZQ", "08ZZZZZ0000Z1ZA"),
            "20260801"
        ),
        "invoice_company_registration_inconsistent"
    );
    // The registration's GSTIN was changed: an earlier GSTIN is held.
    let registered = TAX_UNITS
        .find("<TAXUNIT NAME=\"Rajasthan Registration\"")
        .unwrap();
    let old_gstin = format!(
        "{}{}",
        &TAX_UNITS[..registered],
        TAX_UNITS[registered..].replacen(
            "<GSTOLDREGNUMBER/>",
            "<GSTOLDREGNUMBER>08ZZZZZ0000Z2ZQ</GSTOLDREGNUMBER>",
            1
        )
    );
    assert_eq!(
        refused_as(&old_gstin, "20260801"),
        "invoice_company_registration_inconsistent"
    );
    // No GST unit: only the Default Tax Unit is left.
    let end = TAX_UNITS.rfind("</TAXUNIT>").unwrap() + "</TAXUNIT>".len();
    let default_only = format!("{}{}", &TAX_UNITS[..registered], &TAX_UNITS[end..]);
    assert_eq!(
        refused_as(&default_only, "20260801"),
        "invoice_company_registration_absent"
    );
    // A second GST unit in force, bound to the company.
    let unit = &TAX_UNITS[registered..end];
    let second = unit.replace("-000000cd</GUID>", "-000000ce</GUID>");
    let two = format!("{}{second}{}", &TAX_UNITS[..end], &TAX_UNITS[end..]);
    assert_eq!(
        refused_as(&two, "20260801"),
        "invoice_company_registration_ambiguous"
    );
    // A second unit whose row in force is inactive leaves the first alone.
    let inactive = second.replacen("<ISINACTIVE>No", "<ISINACTIVE>Yes", 1);
    let beside = format!("{}{inactive}{}", &TAX_UNITS[..end], &TAX_UNITS[end..]);
    assert_eq!(
        registration(&beside, "20260801").map(|read| read.gstin),
        Ok("08ZZZZZ0000Z1ZQ".to_string())
    );
}

#[test]
fn the_dated_row_in_force_is_the_latest_on_or_before_the_invoice_date() {
    // Document order is not date order: a later Composition row, written first.
    let later = units_with(
        REGISTRATION_ROW,
        &format!("<GSTREGISTRATIONDETAILS.LIST><FROMDATE>20260701</FROMDATE><STATE>Rajasthan</STATE><REGISTRATIONTYPE>Composition</REGISTRATIONTYPE><ISINACTIVE>No</ISINACTIVE></GSTREGISTRATIONDETAILS.LIST>{REGISTRATION_ROW}"),
    );
    assert_eq!(
        registration(&later, "20260630").map(|read| read.state),
        Ok("Rajasthan".to_string())
    );
    assert_eq!(
        refused_as(&later, "20260701"),
        "invoice_company_registration_not_regular"
    );
}

#[test]
fn a_tax_unit_answer_that_cannot_be_read_is_a_failed_read() {
    let collection = TAX_UNITS.find("<TAXUNIT NAME=\"Default").unwrap();
    let end = TAX_UNITS.rfind("</TAXUNIT>").unwrap() + "</TAXUNIT>".len();
    let none = format!("{}{}", &TAX_UNITS[..collection], &TAX_UNITS[end..]);
    assert_eq!(
        registration(&none, "20260801"),
        Err(RegistrationOutcome::Failed(
            "invoice_company_registration_unread"
        ))
    );
    assert_eq!(
        registration(
            &units_with("<STATUS>1</STATUS>", "<STATUS>0</STATUS>"),
            "20260801"
        ),
        Err(RegistrationOutcome::Failed(
            "invoice_read_status_not_success"
        ))
    );
    assert_eq!(
        registration(
            &units_with(
                "<GUID>6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd</GUID>",
                ""
            ),
            "20260801"
        ),
        Err(RegistrationOutcome::Failed("invoice_tax_unit_without_guid"))
    );
    assert_eq!(
        registration(
            &units_with(
                "<USEDFOR>GST</USEDFOR>",
                "<USEDFOR>GST</USEDFOR><USEDFOR>GST</USEDFOR>"
            ),
            "20260801"
        ),
        Err(RegistrationOutcome::Failed("invoice_read_field_repeated"))
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
        [(
            "Default".to_string(),
            "Manual".to_string(),
            Some("Yes".to_string())
        )]
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

/// The synthetic company's GUID in the lab's answers.
const LAB_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

const RATES_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates.utf16le.xml");

/// The request the lab answered on 10 Oct 2026 is, character for character,
/// the one the code renders (the file starts with a byte order mark).
#[test]
fn the_ledger_rates_request_is_the_one_the_lab_answered() {
    let sent = rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates.request.utf16le.xml"));
    let rendered =
        render_ledger_rates_request("BRIDGE PILOT LAB", ("20260401", "20260801")).unwrap();
    assert_eq!(sent.trim_start_matches('\u{feff}'), rendered);
    // A window that is not eight digits renders nothing.
    for bad in ["2026-04-01", "2026040", "2026040a", ""] {
        assert!(render_ledger_rates_request("X", (bad, "20260801")).is_none());
        assert!(render_ledger_rates_request("X", ("20260401", bad)).is_none());
    }
    // A company name is escaped into the request.
    assert!(
        render_ledger_rates_request("A & B", ("20260401", "20260801"))
            .unwrap()
            .contains("<SVCURRENTCOMPANY>A &amp; B</SVCURRENTCOMPANY>")
    );
}

/// The lab's answer, read at the edge: each sales ledger's one dated row with
/// its all-states block, the tax ledgers' own rate and rounding, and what an
/// absent field is (an element that was not returned, never an empty string).
#[test]
fn the_captured_rate_listing_is_read_with_every_field_in_its_place() {
    let rates = parse_ledger_rates(&rehearsal(RATES_ANSWER), LAB_GUID).unwrap();
    assert_eq!(rates.len(), 14);
    let svc = &rates["BRIDGE Svc 998313 5%"];
    assert_eq!(svc.rate_of_tax_calculation.as_deref(), Some("0"));
    assert_eq!(svc.rounding_method, None, "the element is absent");
    assert_eq!(svc.rounding_limit.as_deref(), Some("0"));
    let [row] = svc.gst_rows.as_slice() else {
        panic!("one dated row: {svc:?}")
    };
    assert_eq!(row.applicable_from.as_deref(), Some("20260401"));
    assert_eq!(row.taxability.as_deref(), Some("Taxable"));
    assert_eq!(row.source.as_deref(), Some("Specify Details Here"));
    let [block] = row.states.as_slice() else {
        panic!("one block: {row:?}")
    };
    assert_eq!(
        block.state.as_deref(),
        Some("Any"),
        "the control character is not part of it"
    );
    let heads = block
        .heads
        .iter()
        .map(|h| (h.head.as_str(), h.valuation.as_deref(), h.rate.as_deref()))
        .collect::<Vec<_>>();
    assert_eq!(
        heads,
        vec![
            ("CGST", Some("Based on Value"), Some("2.50")),
            ("SGST/UTGST", Some("Based on Value"), Some("2.50")),
            ("IGST", Some("Based on Value"), Some("5")),
            ("Cess", Some("Not Applicable"), None),
            ("State Cess", Some("Based on Value"), None),
        ]
    );
    // An on-screen tax ledger: its own rate, and Tally's own spelling of no rounding.
    let tax = &rates["BRIDGE CGST 2.5%"];
    assert_eq!(tax.rate_of_tax_calculation.as_deref(), Some("2.50"));
    assert_eq!(tax.rounding_method.as_deref(), Some("Not Applicable"));
    assert_eq!(tax.rounding_limit.as_deref(), Some("0"));
    assert_eq!(
        tax.gst_rows[0].source.as_deref(),
        Some("As per Company/Group")
    );
    // The ledger made by import has no rounding field and one GST row with no field.
    let imported = &rates["BRIDGE Output CGST XML"];
    assert_eq!(imported.rounding_method, None);
    let [empty] = imported.gst_rows.as_slice() else {
        panic!("one empty row: {imported:?}")
    };
    assert_eq!(
        (&empty.applicable_from, &empty.taxability, &empty.source),
        (&None, &None, &None),
        "a row with no field is not a row with an empty string"
    );
}

#[test]
fn a_rate_listing_that_is_not_this_companys_or_not_whole_is_refused() {
    let xml = rehearsal(RATES_ANSWER);
    // Another company's answer.
    assert_eq!(
        parse_ledger_rates(&xml, "00000000-0000-4000-8000-000000000001"),
        Err("invoice_ledger_rates_company_mismatch")
    );
    // A ledger named twice.
    let first = xml.find("<LEDGER NAME=\"Cash\"").unwrap();
    let end = xml[first..].find("</LEDGER>").unwrap() + first + "</LEDGER>".len();
    let twice = format!("{}{}{}", &xml[..end], &xml[first..end], &xml[end..]);
    assert_eq!(
        parse_ledger_rates(&twice, LAB_GUID),
        Err("invoice_ledger_rates_name_repeated")
    );
    // Tally's error answer, a truncated one, and one with no collection.
    assert_eq!(
        parse_ledger_rates(
            &xml.replace("<STATUS>1</STATUS>", "<STATUS>0</STATUS>"),
            LAB_GUID
        ),
        Err("invoice_read_status_not_success")
    );
    assert_eq!(
        parse_ledger_rates(&xml[..xml.len() / 2], LAB_GUID),
        Err("invoice_read_malformed")
    );
    assert_eq!(
        parse_ledger_rates(
            "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY></BODY></ENVELOPE>",
            LAB_GUID
        ),
        Err("invoice_read_collection_absent")
    );
}
