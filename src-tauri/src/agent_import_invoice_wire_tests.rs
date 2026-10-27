use super::*;
use crate::agent::agent_import::tests::{pilot_lab_guid, sentinel_uuid};

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

fn number(text: &str) -> InvoiceNumber {
    InvoiceNumber::parse(text).expect("an admitted invoice number")
}

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).expect("a calendar date")
}

#[test]
fn the_number_read_is_dated_by_literals_and_leaves_the_class_to_the_reader() {
    let (from, to) = (date("20250401"), date("20260331"));
    let request = render_invoice_number_request("Co", &number("INV/26-27/0042"), (&from, &to));
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
    // GST rule 46(b): 16 characters, letters, digits, hyphen, slash. The
    // renderer takes only a parsed number, so nothing else reaches the literal.
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
        "1".repeat(17).as_str(),
    ] {
        assert_eq!(InvoiceNumber::parse(bad_number), None, "{bad_number:?}");
    }
    assert!(InvoiceNumber::parse(&"1".repeat(16)).is_some());
    assert!(InvoiceNumber::parse("INV/26-27/0042").is_some());
    // A date that is not YYYYMMDD never becomes one either.
    assert!(TallyDate::parse("2025-04-01").is_err());
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
    let guid = &format!("{}-00000193", sentinel_uuid(0x5));
    let (from, to) = (date("20250401"), date("20260331"));
    let type_guid = VoucherTypeGuid::parse(guid).expect("a hex-and-hyphen GUID");
    let request = render_invoice_readback_request("Co", &type_guid, &number("278"), (&from, &to));
    assert!(request.contains(&format!("$Date &gt;= $$Date:\"20250401\" AND $Date &lt;= $$Date:\"20260331\" AND $VoucherNumber = \"278\" AND $GUID:VoucherType:$VoucherTypeName = \"{guid}\"</SYSTEM>")));
    assert!(request.contains("ALLLEDGERENTRIES.*"));
    // Neither a GUID nor a number that could close the literal becomes one.
    for bad_guid in ["g\"", "", "a b", &"a".repeat(65)] {
        assert_eq!(VoucherTypeGuid::parse(bad_guid), None, "{bad_guid:?}");
    }
    assert!(VoucherTypeGuid::parse(&"a".repeat(64)).is_some());
    assert_eq!(InvoiceNumber::parse("A B"), None);
}

/// The pilot lab's tax units as Tally answered the code's own request (see the
/// set's PROVENANCE), decoded.
static TAX_UNITS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-tax-units.utf16le.xml"))
});

/// The pilot lab's GUID with its last character changed: a company that is
/// not the lab.
fn another_guid() -> String {
    let mut guid = pilot_lab_guid();
    let last = guid.pop().unwrap();
    guid.push(if last == '6' { '7' } else { '6' });
    guid
}
const REGISTRATION_ROW: &str =
    "<GSTREGISTRATIONDETAILS.LIST>\r\n      <FROMDATE>20260401</FROMDATE>";

/// The captured answer with `from` replaced by `to`, which must occur once.
fn units_with(from: &str, to: &str) -> String {
    assert_eq!(TAX_UNITS.matches(from).count(), 1, "{from}");
    TAX_UNITS.replacen(from, to, 1)
}

fn registration(xml: &str, as_of: &str) -> Result<CompanyRegistration, RegistrationOutcome> {
    parse_company_registration(xml, &pilot_lab_guid(), as_of)
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
    assert_eq!(registration(&TAX_UNITS, "20260801"), read);
    // A row dated on the invoice date applies; a day earlier nothing does.
    assert_eq!(registration(&TAX_UNITS, "20260401"), read);
    assert_eq!(
        refused_as(&TAX_UNITS, "20260331"),
        "invoice_company_registration_not_yet_in_force"
    );
    // The company's GUID is matched without regard to case.
    assert_eq!(
        parse_company_registration(
            &TAX_UNITS,
            &pilot_lab_guid().to_ascii_uppercase(),
            "20260801"
        ),
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
            units_with(
                &format!("{}-000000cd", pilot_lab_guid()),
                &format!("{}-000000cd", another_guid()),
            ),
            "invoice_company_registration_unbound",
        ),
        (
            units_with(
                &format!("{}-00000064", pilot_lab_guid()),
                &format!("{}-00000064", another_guid()),
            ),
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
    assert_eq!(
        TAX_UNITS[registered..]
            .matches("<GSTOLDREGNUMBER/>")
            .count(),
        1
    );
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
            &units_with(&format!("<GUID>{}-000000cd</GUID>", pilot_lab_guid()), ""),
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

/// One of the pilot lab's captured answers (see the PROVENANCE table beside
/// the fixtures): Tally's own bytes, decoded.
fn captured(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The voucher types as the pilot lab's book answered on 9 Oct 2026 (the
/// `pilot-lab` set): `BRIDGE
/// Sales`, a Sales-class type with one Manual series, resolves; the predefined Sales type (an Automatic series)
/// does not; a type the book lacks is not found; and two raw control
/// characters in `BRIDGE Sales`'s class name do not stop the read.
#[test]
fn the_pilot_labs_voucher_types_resolve_its_manual_type_and_refuse_the_automatic_one() {
    let xml = captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-voucher-types.utf16le.xml"));
    assert_eq!(xml.matches('\u{5}').count(), 2);
    let types = parse_voucher_types(&xml).unwrap();
    assert_eq!(types.len(), 25);
    assert_eq!(
        resolve_voucher_type(&types, "BRIDGE Sales", "Sales"),
        Ok(ResolvedVoucherType {
            guid: format!("{}-000000ce", pilot_lab_guid()),
            class: "Sales".into(),
        })
    );
    let manual = types.iter().find(|row| row.name == "BRIDGE Sales").unwrap();
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
        resolve_voucher_type(&types, "BRIDGE Sales", "Purchase"),
        Err("invoice_voucher_type_wrong_class")
    );
    assert_eq!(
        resolve_voucher_type(&types, "Sales Acc", "Sales"),
        Err("invoice_voucher_type_not_found")
    );
}

/// The duplicate-number read's captured answers on the pilot lab's book: a
/// number its first invoice carries, and a number nothing carries (a
/// collection with no row, which is an answer). The first answer's header
/// counts (`<VOUCHER>30</VOUCHER>` among them) are not rows. Every row in
/// both answers is of the Sales class, so these bytes cannot tell the
/// Sales-class count from a count of every row:
/// `only_a_sales_class_row_with_a_readable_class_counts_as_a_duplicate` is the
/// test that does.
#[test]
fn the_captured_number_answers_count_their_rows_not_their_header() {
    for (name, xml, vouchers, sales) in [
        (
            "number-in-use",
            captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-in-use.utf16le.xml")),
            1,
            1,
        ),
        (
            "number-new-invoice",
            captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-new-invoice.utf16le.xml")),
            0,
            0,
        ),
    ] {
        assert_eq!(count_vouchers(&xml), Ok(vouchers), "{name}");
        assert_eq!(count_sales_vouchers(&xml), Ok(sales), "{name}");
    }
}

/// The control beside the number read, on Tally's own answers: it passes only
/// on a row with the known number, its date and the Sales class. The pilot
/// lab's answer holds its first invoice, numbered BP/26-27/0001 on 1 Aug 2026.
#[test]
fn the_control_row_is_the_known_invoice_with_its_date_and_class() {
    let known = captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-in-use.utf16le.xml"));
    let absent = captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-new-invoice.utf16le.xml"));
    assert_eq!(
        control_row_found(&known, "BP/26-27/0001", "20260801"),
        Ok(true)
    );
    // A read that returns no row is never a control.
    assert_eq!(
        control_row_found(&absent, "BP/26-27/0001", "20260801"),
        Ok(false)
    );
    // The number on another date; another number.
    assert_eq!(
        control_row_found(&known, "BP/26-27/0001", "20260802"),
        Ok(false)
    );
    assert_eq!(
        control_row_found(&known, "BP/26-27/0002", "20260801"),
        Ok(false)
    );
    // A row of the number that is not Sales class is not the control (the
    // captured answer with one edit: no captured answer of this book holds
    // such a row).
    assert_eq!(
        control_row_found(
            &known.replace(
                "<BRIDGEVCHISSALES TYPE=\"Logical\">Yes</BRIDGEVCHISSALES>",
                "<BRIDGEVCHISSALES TYPE=\"Logical\">No</BRIDGEVCHISSALES>"
            ),
            "BP/26-27/0001",
            "20260801"
        ),
        Ok(false)
    );
    // An answer with no collection is an error, as for the read it controls.
    assert_eq!(
        control_row_found(
            "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA></DATA></ENVELOPE>",
            "BP/26-27/0001",
            "20260801"
        ),
        Err("invoice_read_collection_absent")
    );
}

/// The read-back of the invoice ComplyEaze Bridge posted on the pilot lab's
/// book, as Tally answered after the post: every field the comparison reads
/// comes back, and the walk-in customer's invoice carries no GSTIN and no bill
/// allocation.
#[test]
fn the_captured_read_back_carries_every_field_of_a_posted_invoice() {
    let field = |read: &ReadInvoice, key: &str| read.fields.get(key).cloned();
    let Ok(Readback::One(posted)) =
        parse_invoice_readback(&captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-after-post.utf16le.xml")))
    else {
        panic!("one voucher");
    };
    let posted_guid = format!("{}-0000000b", pilot_lab_guid());
    for (key, value) in [
        ("DATE", "20260801"),
        ("VOUCHERNUMBER", "BP/26-27/0010"),
        ("VOUCHERTYPENAME", "BRIDGE Sales"),
        ("REFERENCE", "BP/26-27/0010"),
        ("REFERENCEDATE", "20260801"),
        ("PARTYLEDGERNAME", "BRIDGE Walk-in"),
        ("STATENAME", "Rajasthan"),
        ("PLACEOFSUPPLY", "Rajasthan"),
        ("GSTREGISTRATIONTYPE", "Unregistered/Consumer"),
        ("ISINVOICE", "Yes"),
        ("ISCANCELLED", "No"),
        ("ISOPTIONAL", "No"),
        ("GUID", posted_guid.as_str()),
        ("ALTERID", "17"),
    ] {
        assert_eq!(field(&posted, key).as_deref(), Some(value), "{key}");
    }
    assert_eq!(field(&posted, "PARTYGSTIN"), None);
    let legs = posted
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
            ("BRIDGE Walk-in", "-1051.00", 0),
            ("BRIDGE Svc 998313 5%", "600.40", 0),
            ("BRIDGE Svc 998314 5%", "400.40", 0),
            ("BRIDGE CGST 2.5%", "25.02", 0),
            ("BRIDGE SGST 2.5%", "25.02", 0),
            ("BRIDGE Round Off", "0.16", 0),
        ]
    );
    // The same read before the post: a collection with no voucher.
    assert!(matches!(
        parse_invoice_readback(&captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-before-post.utf16le.xml"))),
        Ok(Readback::Absent)
    ));
}

const RATES_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates.utf16le.xml");

/// The request the code renders is, character for character, the lab
/// request set W16's (the file starts with a byte order mark): the request
/// the lab answered on 10 Oct 2026 with the HSN/SAC check's three fields
/// added to its field list and nothing else changed (answered on 12 Oct 2026).
#[test]
fn the_ledger_rates_request_is_the_answered_one_with_the_hsn_fields() {
    let sent = captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-hsn.request.utf16le.xml"));
    let (from, to) = (date("20260401"), date("20260801"));
    let rendered = render_ledger_rates_request("BRIDGE PILOT LAB", (&from, &to));
    assert_eq!(sent.trim_start_matches('\u{feff}'), rendered);
    let answered = captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates.request.utf16le.xml"));
    assert_eq!(
        answered.trim_start_matches('\u{feff}').replace(
            "ROUNDINGLIMIT</FETCH>",
            "ROUNDINGLIMIT, HSNDETAILS.LIST, GSTTYPEOFSUPPLY, GSTAPPLICABLE</FETCH>"
        ),
        rendered
    );
    // A window that is not a YYYYMMDD calendar date never becomes one, so it
    // cannot reach the request.
    for bad in ["2026-04-01", "2026040", "2026040a", "", "20260230"] {
        assert!(TallyDate::parse(bad).is_err(), "{bad:?}");
    }
    // A company name is escaped into the request.
    assert!(render_ledger_rates_request("A & B", (&from, &to))
        .contains("<SVCURRENTCOMPANY>A &amp; B</SVCURRENTCOMPANY>"));
}

/// One part naming exactly these parents, planned the way a scope plans them
/// (one catalogue row per parent is enough to name it).
fn part_of(parents: &[&str]) -> bridge_tally_protocol::parent_partition::ParentPart {
    use bridge_tally_protocol::parent_partition::{
        ParentObservation, ParentPartition, PartitionLimits,
    };
    let names = parents
        .iter()
        .enumerate()
        .map(|(index, parent)| (format!("ledger {index}"), format!("guid-{index}"), *parent))
        .collect::<Vec<_>>();
    let plan = ParentPartition::plan(
        names.iter().map(|(name, guid, parent)| {
            (
                name.as_str(),
                guid.as_str(),
                ParentObservation::Named(parent),
            )
        }),
        PartitionLimits {
            max_ledgers_per_part: 4_266,
            max_parents_per_part: 200,
            max_parts: 12,
            max_complement_formula_bytes: 262_144,
        },
    )
    .unwrap();
    assert_eq!(plan.parts().len(), 1);
    plan.parts()[0].clone()
}

/// The scoped rates request is the unfiltered one with the parent formula
/// added and nothing else changed (#1331): same fetch list, same window, the
/// `FILTERS` element after the computed company GUID, as the compliance
/// listing's parts carry it (§11e).
#[test]
fn a_scoped_rates_request_adds_only_the_parent_formula_and_its_filter() {
    let (from, to) = (date("20260401"), date("20260801"));
    let window = (&from, &to);
    let whole = render_ledger_rates_request("BRIDGE PILOT LAB", window);
    let part = part_of(&[
        "Sundry Debtors",
        "Sales Accounts",
        "Duties & Taxes",
        "Indirect Expenses",
    ]);
    let scoped = render_ledger_rates_request_for_parents("BRIDGE PILOT LAB", window, &part);
    let formula = concat!(
        "<SYSTEM TYPE=\"Formulae\" NAME=\"BridgeParentPart\">",
        "$Parent = \"Duties &amp; Taxes\" OR $Parent = \"Indirect Expenses\" OR ",
        "$Parent = \"Sales Accounts\" OR $Parent = \"Sundry Debtors\"</SYSTEM>"
    );
    assert!(scoped.contains(formula), "{scoped}");
    let undone = scoped
        .replace(formula, "")
        .replace("<FILTERS>BridgeParentPart</FILTERS>", "");
    assert_eq!(undone, whole);
}

/// The lab's answer, read at the edge: each sales ledger's one dated row with
/// its all-states block, the tax ledgers' own rate and rounding, and what an
/// absent field is (an element that was not returned, never an empty string).
#[test]
fn the_captured_rate_listing_is_read_with_every_field_in_its_place() {
    let named = [
        "BRIDGE Svc 998313 5%",
        "BRIDGE CGST 2.5%",
        "BRIDGE Output CGST XML",
    ];
    let rates = parse_ledger_rates(&captured(RATES_ANSWER), &pilot_lab_guid(), &named).unwrap();
    assert_eq!(rates.len(), 3, "only the ledgers asked for are read");
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
    assert!(!row.empty);
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
    assert!(empty.empty, "it holds no element and no text");
}

#[test]
fn a_rate_listing_that_is_not_this_companys_or_not_whole_is_refused() {
    let xml = captured(RATES_ANSWER);
    let named = ["BRIDGE Svc 998313 5%"];
    // Another company's answer.
    assert_eq!(
        parse_ledger_rates(&xml, "00000000-0000-4000-8000-000000000001", &named),
        Err("invoice_ledger_rates_company_mismatch")
    );
    // One row of another company, the last one: every row is bound, not the first.
    let last = xml.rfind(&pilot_lab_guid()).unwrap();
    let mut foreign = xml.clone();
    foreign.replace_range(
        last..last + pilot_lab_guid().len(),
        "00000000-0000-4000-8000-000000000001",
    );
    assert_eq!(
        parse_ledger_rates(&foreign, &pilot_lab_guid(), &named),
        Err("invoice_ledger_rates_company_mismatch")
    );
    // A ledger asked for, named twice.
    let first = xml.find("<LEDGER NAME=\"Cash\"").unwrap();
    let end = xml[first..].find("</LEDGER>").unwrap() + first + "</LEDGER>".len();
    let twice = format!("{}{}{}", &xml[..end], &xml[first..end], &xml[end..]);
    assert_eq!(
        parse_ledger_rates(&twice, &pilot_lab_guid(), &["Cash"]),
        Err("invoice_ledger_rates_name_repeated")
    );
    // The same duplicate in a ledger the invoice does not name is not read.
    assert!(parse_ledger_rates(&twice, &pilot_lab_guid(), &named).is_ok());
    // Tally's error answer, a truncated one, and one with no collection.
    assert_eq!(
        parse_ledger_rates(
            &xml.replace("<STATUS>1</STATUS>", "<STATUS>0</STATUS>"),
            &pilot_lab_guid(),
            &named
        ),
        Err("invoice_read_status_not_success")
    );
    assert_eq!(
        parse_ledger_rates(&xml[..xml.len() / 2], &pilot_lab_guid(), &named),
        Err("invoice_read_malformed")
    );
    assert_eq!(
        parse_ledger_rates(
            "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY></BODY></ENVELOPE>",
            &pilot_lab_guid(),
            &named
        ),
        Err("invoice_read_collection_absent")
    );
}

/// A listing with no ledger row is a read that went wrong, never "no ledger
/// has a rate": a company always has ledgers (the invoice's own were read in
/// the compliance listing before it). One that holds rows, but not the ledger
/// asked for, reads as no row for that ledger.
#[test]
fn an_empty_rate_listing_is_its_own_failure_and_a_listing_without_the_ledger_is_not() {
    for empty in [
        "<COLLECTION></COLLECTION>",
        "<COLLECTION> </COLLECTION>",
        "<COLLECTION/>",
    ] {
        let xml = format!(
            "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA>{empty}</DATA></BODY></ENVELOPE>"
        );
        assert_eq!(
            parse_ledger_rates(&xml, &pilot_lab_guid(), &["L"]),
            Err("invoice_ledger_rates_listing_empty"),
            "{empty}"
        );
    }
    let other = parse_ledger_rates(&ledger_answer("Other", ""), &pilot_lab_guid(), &["L"]).unwrap();
    assert!(other.is_empty());
}

/// A ledger row of the listing with no name is a read that went wrong, refused
/// once the row is known to be this company's, whatever the invoice names. The
/// lab's listing has none: its fourteen rows are named and read whole (the
/// nameless `LEDGER` element in it is a count inside `CMPINFO`, not a row).
#[test]
fn a_nameless_ledger_row_fails_the_rate_read_and_the_captured_listing_has_none() {
    let nameless = ledger_answer("L", "").replace("<LEDGER NAME=\"L\">", "<LEDGER>");
    assert_eq!(
        parse_ledger_rates(&nameless, &pilot_lab_guid(), &["L"]),
        Err("invoice_ledger_rates_row_unnamed")
    );
    let beside = ledger_answer("L", "").replace(
        "</COLLECTION>",
        &format!(
            "<LEDGER><BRIDGECOMPANYGUID>{guid}</BRIDGECOMPANYGUID></LEDGER></COLLECTION>",
            guid = pilot_lab_guid()
        ),
    );
    assert_eq!(
        parse_ledger_rates(&beside, &pilot_lab_guid(), &["L"]),
        Err("invoice_ledger_rates_row_unnamed")
    );
    // Another company's nameless row is refused as another company's.
    assert_eq!(
        parse_ledger_rates(&nameless, "00000000-0000-4000-8000-000000000001", &["L"]),
        Err("invoice_ledger_rates_company_mismatch")
    );
    let named = [
        "BRIDGE CGST 2.5%",
        "BRIDGE CGST 9%",
        "BRIDGE Goods 18%",
        "BRIDGE Output CGST XML",
        "BRIDGE Purchase Svc 5%",
        "BRIDGE Round Off",
        "BRIDGE SGST 2.5%",
        "BRIDGE SGST 9%",
        "BRIDGE Supplier RJ",
        "BRIDGE Svc 998313 5%",
        "BRIDGE Svc 998314 5%",
        "BRIDGE Walk-in",
        "Cash",
        "Profit & Loss A/c",
    ];
    let rates = parse_ledger_rates(&captured(RATES_ANSWER), &pilot_lab_guid(), &named).unwrap();
    assert_eq!(rates.keys().map(String::as_str).collect::<Vec<_>>(), named);
}

/// One ledger row around `inner`, bound to the lab company.
fn ledger_answer(name: &str, inner: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <LEDGER NAME=\"{name}\"><BRIDGECOMPANYGUID>{guid}</BRIDGECOMPANYGUID>{inner}</LEDGER>\
         </COLLECTION></DATA></BODY></ENVELOPE>",
        guid = pilot_lab_guid()
    )
}

/// What a row of the listing says that the lab's rows did not, and what a
/// ledger the invoice does not name can say without failing it.
#[test]
fn a_row_the_lab_did_not_measure_is_marked_and_an_unrelated_row_cannot_fail_the_read() {
    let row = |extra_row: &str, extra_state: &str, extra_head: &str, slabs: &str| {
        format!(
            "<GSTDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><TAXABILITY>Taxable</TAXABILITY>\
             <SRCOFGSTDETAILS>Specify Details Here</SRCOFGSTDETAILS>{extra_row}<STATEWISEDETAILS.LIST>\
             <STATENAME>Any</STATENAME>{extra_state}<RATEDETAILS.LIST><GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD>\
             <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>2.50</GSTRATE>{extra_head}\
             </RATEDETAILS.LIST>{slabs}</STATEWISEDETAILS.LIST></GSTDETAILS.LIST>"
        )
    };
    let unmeasured = |inner: String| {
        let rates =
            parse_ledger_rates(&ledger_answer("L", &inner), &pilot_lab_guid(), &["L"]).unwrap();
        rates["L"].gst_rows[0].unmeasured
    };
    assert!(!unmeasured(row(
        "",
        "",
        "",
        "<GSTSLABRATES.LIST></GSTSLABRATES.LIST>"
    )));
    assert!(unmeasured(row(
        "<ISREVERSECHARGEAPPLICABLE>Yes</ISREVERSECHARGEAPPLICABLE>",
        "",
        "",
        ""
    )));
    assert!(unmeasured(row("", "<ISMRPBASED>Yes</ISMRPBASED>", "", "")));
    assert!(unmeasured(row(
        "",
        "",
        "<GSTRATEPERUNIT>3</GSTRATEPERUNIT>",
        ""
    )));
    assert!(unmeasured(row(
        "",
        "",
        "",
        "<GSTSLABRATES.LIST><SLABFROM>1000</SLABFROM></GSTSLABRATES.LIST>"
    )));
    assert!(unmeasured(row(
        "",
        "",
        "",
        "<GSTSLABRATES.LIST>1000</GSTSLABRATES.LIST>"
    )));
    // An element that is present and empty is not an absent one.
    let present = parse_ledger_rates(
        &ledger_answer(
            "L",
            "<ROUNDINGMETHOD></ROUNDINGMETHOD><ROUNDINGLIMIT>0</ROUNDINGLIMIT>",
        ),
        &pilot_lab_guid(),
        &["L"],
    )
    .unwrap();
    assert_eq!(present["L"].rounding_method.as_deref(), Some(""));
    assert_eq!(present["L"].rate_of_tax_calculation, None);
    // A ledger name with an entity decodes like the other reads' names.
    let named = parse_ledger_rates(
        &ledger_answer("A &amp; B", ""),
        &pilot_lab_guid(),
        &["A & B"],
    )
    .unwrap();
    assert!(named.contains_key("A & B"));
    // A row of a ledger nobody named may be malformed in every way the parse
    // refuses for a named one: it is not read.
    let broken = format!(
        "<LEDGER NAME=\"Other\"><BRIDGECOMPANYGUID>{guid}</BRIDGECOMPANYGUID>\
         <GSTDETAILS.LIST><APPLICABLEFROM>1</APPLICABLEFROM><APPLICABLEFROM>2</APPLICABLEFROM>\
         <STATEWISEDETAILS.LIST><RATEDETAILS.LIST></RATEDETAILS.LIST></STATEWISEDETAILS.LIST>\
         </GSTDETAILS.LIST></LEDGER><LEDGER/>",
        guid = pilot_lab_guid()
    );
    let answer = ledger_answer("L", "").replace("</COLLECTION>", &format!("{broken}</COLLECTION>"));
    let wanted = parse_ledger_rates(&answer, &pilot_lab_guid(), &["L"]);
    assert_eq!(
        wanted,
        Err("invoice_ledger_rates_company_mismatch"),
        "a bare <LEDGER/> carries no company"
    );
    let answer = ledger_answer("L", "").replace(
        "</COLLECTION>",
        &format!("{}</COLLECTION>", broken.replace("<LEDGER/>", "")),
    );
    assert!(parse_ledger_rates(&answer, &pilot_lab_guid(), &["L"]).is_ok());
    // The same row, asked for, is refused for what is wrong with it (a head
    // with no name; an element twice is refused under its own code).
    assert_eq!(
        parse_ledger_rates(&answer, &pilot_lab_guid(), &["Other"]),
        Err("invoice_ledger_rates_head_unnamed")
    );
    let twice = ledger_answer(
        "L",
        "<GSTDETAILS.LIST><APPLICABLEFROM>1</APPLICABLEFROM><APPLICABLEFROM>2</APPLICABLEFROM></GSTDETAILS.LIST>",
    );
    assert_eq!(
        parse_ledger_rates(&twice, &pilot_lab_guid(), &["L"]),
        Err("invoice_read_field_repeated")
    );
    let no_name = ledger_answer(
        "L",
        "<GSTDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><STATEWISEDETAILS.LIST>\
         <RATEDETAILS.LIST><GSTRATE>5</GSTRATE></RATEDETAILS.LIST></STATEWISEDETAILS.LIST></GSTDETAILS.LIST>",
    );
    assert_eq!(
        parse_ledger_rates(&no_name, &pilot_lab_guid(), &["L"]),
        Err("invoice_ledger_rates_head_unnamed")
    );
}

/// The read-back request with the four GSTR-1 status fields is, byte for byte,
/// the one the lab answered for `BP/26-27/0010` on 10 Oct 2026 (the file begins
/// with the byte order mark the request is sent with); for `BP/26-27/0021` it
/// has the SHA-256 of the request file the lab sent for that number.
#[test]
fn the_status_read_back_request_is_the_one_the_lab_answered() {
    let type_guid =
        VoucherTypeGuid::parse("6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce").unwrap();
    let (from, to) = (date("20260401"), date("20270331"));
    let sent = |text: &str| {
        let rendered = render_invoice_readback_request(
            "BRIDGE PILOT LAB",
            &type_guid,
            &number(text),
            (&from, &to),
        );
        [0xFF, 0xFE]
            .into_iter()
            .chain(rendered.encode_utf16().flat_map(u16::to_le_bytes))
            .collect::<Vec<u8>>()
    };
    let answered: &[u8] = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-status.request.utf16le.xml"
    );
    assert_eq!(sent("BP/26-27/0010"), answered);
    assert_eq!(
        crate::agent::sha256_hex(&sent("BP/26-27/0021")),
        "b5167673a63dba8bb7e96791e3b6bc2746c60517ea06886c40d47b52ea76aca5"
    );
}

/// The group request is W16's: the group listing the compliance read sends
/// (answered on 10 Oct 2026) with `HSNDETAILS.LIST` added and nothing else
/// changed (answered on 12 Oct 2026).
#[test]
fn the_group_hsn_request_is_the_answered_listing_with_the_hsn_rows() {
    let sent = captured(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-groups-hsn.request.utf16le.xml"));
    let rendered = render_group_hsn_request("BRIDGE PILOT LAB");
    assert_eq!(sent.trim_start_matches('\u{feff}'), rendered);
    assert_eq!(
        bridge_tally_protocol::native_outstandings::render_native_group_snapshot_request(
            "BRIDGE PILOT LAB"
        )
        .replace(
            "RESERVEDNAME</FETCH>",
            "RESERVEDNAME, HSNDETAILS.LIST</FETCH>"
        ),
        rendered
    );
    assert!(render_group_hsn_request("A & B")
        .contains("<SVCURRENTCOMPANY>A &amp; B</SVCURRENTCOMPANY>"));
}
