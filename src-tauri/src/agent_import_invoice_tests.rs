use super::*;
use crate::agent::agent_import::VoucherType;
use serde_json::json;

const RAJ: &str = "Rajasthan";
// Check digits computed independently of the code under test; the first is a
// public worked example.
const GSTIN_MH: &str = concat!("27AAPFU", "0939F1ZV");
const GSTIN_RJ: &str = concat!("08ABCDE", "1234F1Z0");

fn entry(ledger: &str, amount: &str, side: EntrySide) -> ImportEntry {
    ImportEntry {
        ledger: ledger.to_string(),
        amount: amount.to_string(),
        side,
    }
}

fn detail() -> InvoiceDetail {
    InvoiceDetail {
        voucher_type_name: "Sales".to_string(),
        place_of_supply: RAJ.to_string(),
        round_off_ledger: None,
        observed: None,
    }
}

/// A 12 percent invoice: 10,000.00 taxable, 600.00 and 600.00 tax.
fn voucher() -> ImportVoucher {
    ImportVoucher {
        bridge_txn_id: "t1".to_string(),
        date: "2026-03-10".to_string(),
        voucher_type: VoucherType::Sales,
        narration: Some("Invoice 278".to_string()),
        reference: None,
        voucher_number: Some("278".to_string()),
        invoice: Some(detail()),
        entries: vec![
            entry("Customer A", "11200.00", EntrySide::Dr),
            entry("Sales", "10000.00", EntrySide::Cr),
            entry("Output CGST", "600.00", EntrySide::Cr),
            entry("Output SGST", "600.00", EntrySide::Cr),
        ],
    }
}

fn facts_for(names: &[(&str, &[&str], DutyHead)]) -> BTreeMap<String, LedgerFacts> {
    names
        .iter()
        .map(|(name, groups, head)| {
            (
                name.to_string(),
                LedgerFacts {
                    reserved_groups: groups.iter().map(|g| g.to_string()).collect(),
                    duty_head: *head,
                    gstin: GstinEvidence::NotReported,
                    registration_type: None,
                    bill_wise: None,
                },
            )
        })
        .collect()
}

fn good_facts() -> BTreeMap<String, LedgerFacts> {
    let mut facts = facts_for(&[
        ("Customer A", &["Sundry Debtors", "Current Assets"], DutyHead::NotTax),
        ("Sales", &["Sales Accounts"], DutyHead::NotTax),
        ("Output CGST", &["Duties & Taxes", "Current Liabilities"], DutyHead::Cgst),
        ("Output SGST", &["Duties & Taxes", "Current Liabilities"], DutyHead::State),
        ("Round Off", &["Indirect Expenses"], DutyHead::NotTax),
    ]);
    let party = facts.get_mut("Customer A").unwrap();
    party.gstin = GstinEvidence::InForce(GSTIN_RJ.to_string());
    party.registration_type = Some("Regular".to_string());
    party.bill_wise = Some(true);
    facts
}

fn codes(result: Result<InvoiceRoles, Vec<InvoiceRefusal>>) -> Vec<&'static str> {
    result.unwrap_err().into_iter().map(|r| r.code).collect()
}

#[test]
fn gstin_check_digit_is_the_published_scheme() {
    assert!(gstin_valid(GSTIN_MH));
    assert!(gstin_valid(GSTIN_RJ));
    assert!(!gstin_valid(concat!("27AAPFU", "0939F1ZW")), "wrong check digit");
    assert!(!gstin_valid(concat!("27AAPFU", "0939F1Z")), "too short");
    assert!(!gstin_valid(concat!("27aapfu", "0939f1zv")), "lower case");
    assert!(!gstin_valid(concat!("27AAPFU", "0939F0ZV")), "entity digit 0");
    assert!(!gstin_valid(concat!("27AAPFU", "0939F1XV")), "14th character is Z");
    assert_eq!(gstin_state(GSTIN_MH), Some("Maharashtra"));
    assert_eq!(gstin_state(GSTIN_RJ), Some(RAJ));
}

#[test]
fn structure_admits_the_plain_invoice_and_refuses_each_defect() {
    assert_eq!(validate_invoice_voucher(&voucher()), Ok(()));
    let mutated = |change: &dyn Fn(&mut ImportVoucher)| {
        let mut v = voucher();
        change(&mut v);
        validate_invoice_voucher(&v).unwrap_err()
    };
    assert_eq!(mutated(&|v| v.invoice = None), "invoice_detail_required");
    // What Tally returned is refused on the way in, and required on a saved record.
    let mut supplied = voucher();
    supplied.invoice.as_mut().unwrap().observed = Some(observed());
    assert_eq!(validate_invoice_voucher(&supplied), Ok(()), "a saved record carries it");
    assert_eq!(refuse_supplied_observed(&[supplied]), Err("invoice_observed_not_input".to_string()));
    assert_eq!(refuse_supplied_observed(&[voucher()]), Ok(()));
    assert_eq!(mutated(&|v| v.voucher_number = Some("A\"B".into())), "invoice_number_invalid");
    assert_eq!(mutated(&|v| v.voucher_number = Some(" 278".into())), "invoice_number_invalid");
    assert_eq!(validate_invoice_voucher(&{ let mut v = voucher(); v.voucher_number = Some("INV-26/27-0042".into()); v }), Ok(()));
    // Outside GST rule 46(b): a space, a period, an underscore, over 16 characters.
    for bad in ["INV 0042", "INV.0042", "INV_0042", "12345678901234567"] {
        assert_eq!(mutated(&|v| v.voucher_number = Some(bad.into())), "invoice_number_invalid", "{bad}");
    }
    assert_eq!(mutated(&|v| v.voucher_number = None), "invoice_number_required");
    assert_eq!(mutated(&|v| v.reference = Some("r".into())), "invoice_reference_not_for_sales");
    assert_eq!(
        mutated(&|v| v.invoice.as_mut().unwrap().place_of_supply = "Rajastan".into()),
        "invoice_place_of_supply_unknown"
    );
    assert_eq!(
        mutated(&|v| v.invoice.as_mut().unwrap().voucher_type_name = " Sales".into()),
        "invoice_voucher_type_name_invalid"
    );
    assert_eq!(
        mutated(&|v| v.invoice.as_mut().unwrap().round_off_ledger = Some("Absent".into())),
        "invoice_round_off_ledger_invalid"
    );
    assert_eq!(
        mutated(&|v| v.entries.push(entry("Sales", "1.00", EntrySide::Dr))),
        "invoice_ledger_repeated"
    );
    assert_eq!(
        mutated(&|v| v.entries.truncate(3)),
        "invoice_entry_shape_invalid"
    );
    // A non-invoice voucher must not carry invoice detail.
    let mut journal = voucher();
    journal.voucher_type = VoucherType::Journal;
    assert_eq!(validate_invoice_voucher(&journal), Err("invoice_detail_on_non_invoice".to_string()));
}

#[test]
fn roles_come_from_groups_and_duty_heads_never_from_names() {
    let roles = classify_sales_invoice(&voucher(), &good_facts(), RAJ).unwrap();
    assert_eq!(
        roles,
        InvoiceRoles { party: 0, sales: 1, cgst: 2, state_tax: 3, round_off: None }
    );
    // The same names, but the ledgers say otherwise: the head decides, not "Output SGST".
    let mut facts = good_facts();
    facts.get_mut("Output SGST").unwrap().duty_head = DutyHead::Cgst;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        vec!["invoice_needs_exactly_one_cgst_ledger", "invoice_needs_exactly_one_state_tax_ledger"]
    );
}

#[test]
fn each_master_defect_is_refused_with_its_own_code() {
    let check = |change: &dyn Fn(&mut BTreeMap<String, LedgerFacts>), expect: &str| {
        let mut facts = good_facts();
        change(&mut facts);
        let got = codes(classify_sales_invoice(&voucher(), &facts, RAJ));
        assert!(got.contains(&expect_static(expect)), "{expect}: got {got:?}");
    };
    check(&|f| f.get_mut("Customer A").unwrap().bill_wise = None, "invoice_party_bill_wise_unknown");
    check(&|f| f.get_mut("Customer A").unwrap().gstin = GstinEvidence::NoneInForce, "invoice_party_gstin_missing");
    check(&|f| f.get_mut("Customer A").unwrap().gstin = GstinEvidence::InForce(concat!("08ABCDE", "1234F1Z1").into()), "invoice_party_gstin_invalid");
    check(&|f| f.get_mut("Customer A").unwrap().gstin = GstinEvidence::InForce(GSTIN_MH.into()), "invoice_place_of_supply_not_party_state");
    check(&|f| f.get_mut("Customer A").unwrap().registration_type = Some("Composition".into()), "invoice_party_not_regular");
    check(&|f| f.get_mut("Customer A").unwrap().reserved_groups = vec![], "invoice_ledger_group_unresolved");
    check(&|f| f.get_mut("Sales").unwrap().reserved_groups = vec!["Direct Incomes".into()], "invoice_ledger_role_unknown");
    check(&|f| f.get_mut("Output CGST").unwrap().duty_head = DutyHead::Igst, "invoice_igst_not_supported");
    check(&|f| f.get_mut("Output CGST").unwrap().duty_head = DutyHead::Cess, "invoice_cess_not_supported");
    check(&|f| f.get_mut("Output CGST").unwrap().duty_head = DutyHead::NotTax, "invoice_tax_ledger_head_unknown");
    check(&|f| { f.remove("Sales"); }, "invoice_ledger_not_observed");
}

fn expect_static(code: &str) -> &'static str {
    // Test-only: the codes under test are string literals in the module.
    Box::leak(code.to_string().into_boxed_str())
}

#[test]
fn a_party_that_is_not_bill_wise_is_admitted_and_an_unknown_flag_is_not() {
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().bill_wise = Some(false);
    assert!(classify_sales_invoice(&voucher(), &facts, RAJ).is_ok());
}

#[test]
fn arithmetic_must_close_and_tax_must_be_a_slab() {
    let mut v = voucher();
    v.entries[0].amount = "11201.00".to_string();
    assert_eq!(codes(classify_sales_invoice(&v, &good_facts(), RAJ)), vec!["invoice_party_amount_does_not_close"]);
    let mut v = voucher();
    v.entries[2].amount = "700.00".to_string();
    assert_eq!(codes(classify_sales_invoice(&v, &good_facts(), RAJ)), vec!["invoice_cgst_and_state_tax_differ"]);
    // A sale of 10.00 carrying 0.02 of tax is not 5 percent (the check is paise, not rupees).
    let mut v = voucher();
    v.entries[0].amount = "10.02".to_string();
    v.entries[1].amount = "10.00".to_string();
    v.entries[2].amount = "0.01".to_string();
    v.entries[3].amount = "0.01".to_string();
    assert_eq!(codes(classify_sales_invoice(&v, &good_facts(), RAJ)), vec!["invoice_tax_matches_no_slab_rate"]);
    // A few paise of per-line rounding is within tolerance: 12 percent of 10,000.03.
    let mut v = voucher();
    v.entries[1].amount = "10000.03".to_string();
    v.entries[0].amount = "11200.03".to_string();
    assert!(classify_sales_invoice(&v, &good_facts(), RAJ).is_ok());
    // 7 percent is no GST slab: 700 + 700.
    let mut v = voucher();
    v.entries[2].amount = "700.00".to_string();
    v.entries[3].amount = "700.00".to_string();
    v.entries[0].amount = "11400.00".to_string();
    assert_eq!(codes(classify_sales_invoice(&v, &good_facts(), RAJ)), vec!["invoice_tax_matches_no_slab_rate"]);
}

#[test]
fn a_round_off_is_taken_by_side_and_bounded_under_a_rupee() {
    // Customer owes 11,200.40: a credit round off of 0.40.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11200.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
    assert_eq!(validate_invoice_voucher(&v), Ok(()));
    assert_eq!(classify_sales_invoice(&v, &good_facts(), RAJ).unwrap().round_off, Some(4));
    // Owes 11,199.60: a debit round off of 0.40.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11199.60".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Dr));
    assert!(classify_sales_invoice(&v, &good_facts(), RAJ).is_ok());
    // Wrong direction does not close.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11200.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Dr));
    assert_eq!(codes(classify_sales_invoice(&v, &good_facts(), RAJ)), vec!["invoice_party_amount_does_not_close"]);
    // A round off of a rupee or more is refused.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11201.00".to_string();
    v.entries.push(entry("Round Off", "1.00", EntrySide::Cr));
    assert_eq!(codes(classify_sales_invoice(&v, &good_facts(), RAJ)), vec!["invoice_round_off_too_large"]);
    // The round off ledger must sit under Indirect Expenses or Incomes.
    let mut facts = good_facts();
    facts.get_mut("Round Off").unwrap().reserved_groups = vec!["Current Assets".into()];
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11200.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
    assert!(codes(classify_sales_invoice(&v, &facts, RAJ)).contains(&"invoice_round_off_ledger_group"));
}

fn observed() -> InvoiceObserved {
    InvoiceObserved {
        voucher_type_guid: "type-guid".to_string(),
        party_gstin: Some(GSTIN_RJ.to_string()),
        party_state: RAJ.to_string(),
        party_registration_type: "Regular".to_string(),
        party_bill_wise: true,
        company_state: RAJ.to_string(),
    }
}

/// The invoice-view shape Tally accepted on the client book for Sales (38
/// hand-imported vouchers, party leg first with its New Ref, credit legs
/// after), carrying the GST header the accepted Purchase shape carries.
#[test]
fn the_rendered_sales_invoice_is_the_accepted_shape() {
    let mut v = voucher();
    v.invoice.as_mut().unwrap().observed = Some(observed());
    let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let xml = render_sales_invoice_xml(&v, id, "20260310", "<NARRATION>Invoice 278</NARRATION>").unwrap();
    let expected = "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"11111111-1111-1111-1111-111111111111\" VCHTYPE=\"Sales\" ACTION=\"Create\" OBJVIEW=\"Invoice Voucher View\"><DATE>20260310</DATE><EFFECTIVEDATE>20260310</EFFECTIVEDATE><REFERENCEDATE>20260310</REFERENCEDATE><REFERENCE>278</REFERENCE><VOUCHERTYPENAME>Sales</VOUCHERTYPENAME><VOUCHERNUMBER>278</VOUCHERNUMBER><PARTYLEDGERNAME>Customer A</PARTYLEDGERNAME><PARTYNAME>Customer A</PARTYNAME><BASICBASEPARTYNAME>Customer A</BASICBASEPARTYNAME><PARTYGSTIN>@GSTIN@</PARTYGSTIN><STATENAME>Rajasthan</STATENAME><PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY><GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE><COUNTRYOFRESIDENCE>India</COUNTRYOFRESIDENCE><PERSISTEDVIEW>Invoice Voucher View</PERSISTEDVIEW><VCHENTRYMODE>Accounting Invoice</VCHENTRYMODE><ISINVOICE>Yes</ISINVOICE><NARRATION>Invoice 278</NARRATION><LEDGERENTRIES.LIST><LEDGERNAME>Customer A</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-11200.00</AMOUNT><BILLALLOCATIONS.LIST><NAME>278</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>-11200.00</AMOUNT></BILLALLOCATIONS.LIST></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Sales</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>10000.00</AMOUNT></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Output CGST</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>600.00</AMOUNT></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Output SGST</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>600.00</AMOUNT></LEDGERENTRIES.LIST></VOUCHER></TALLYMESSAGE>";
    assert_eq!(xml, expected.replace("@GSTIN@", GSTIN_RJ));
    assert!(!xml.contains("ALLLEDGERENTRIES"), "an invoice view silently drops ALLLEDGERENTRIES");
    // A party that is not bill-wise gets no New Ref; and an unobserved voucher does not render.
    let mut not_bill_wise = v.clone();
    not_bill_wise.invoice.as_mut().unwrap().observed.as_mut().unwrap().party_bill_wise = false;
    assert!(!render_sales_invoice_xml(&not_bill_wise, id, "20260310", "").unwrap().contains("BILLALLOCATIONS"));
    assert!(render_sales_invoice_xml(&voucher(), id, "20260310", "").is_none());
}

#[test]
fn names_with_markup_characters_are_escaped_in_the_rendered_xml() {
    let mut v = voucher();
    v.entries[0].ledger = "A & B \"Co\" <x>".to_string();
    v.invoice.as_mut().unwrap().observed = Some(observed());
    let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let xml = render_sales_invoice_xml(&v, id, "20260310", "").unwrap();
    assert!(!xml.contains("A & B"));
    assert!(xml.contains("A &amp; B"));
    assert!(!xml.contains("<x>"));
}

#[test]
fn the_supply_must_be_made_in_the_company_state_and_an_unregistered_customer_is_admitted() {
    // The shape posted is CGST plus state tax: a supply made elsewhere is refused.
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &good_facts(), "Haryana")),
        vec!["invoice_place_of_supply_not_company_state"]
    );
    // An unregistered customer: no GSTIN in force and no registration type.
    let mut facts = good_facts();
    let party = facts.get_mut("Customer A").unwrap();
    party.gstin = GstinEvidence::NoneInForce;
    party.registration_type = None;
    party.bill_wise = Some(false);
    assert!(classify_sales_invoice(&voucher(), &facts, RAJ).is_ok());
    facts.get_mut("Customer A").unwrap().registration_type = Some("Unregistered/Consumer".into());
    assert!(classify_sales_invoice(&voucher(), &facts, RAJ).is_ok());
    // Registered on the ledger but no number in force: refused, not guessed unregistered.
    facts.get_mut("Customer A").unwrap().registration_type = Some("Regular".into());
    assert_eq!(codes(classify_sales_invoice(&voucher(), &facts, RAJ)), vec!["invoice_party_gstin_missing"]);
    // A GSTIN history Bridge cannot settle refuses.
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::Unsettled;
    assert_eq!(codes(classify_sales_invoice(&voucher(), &facts, RAJ)), vec!["invoice_party_gstin_unsettled"]);
    // Silence is not evidence of "unregistered": a ledger Tally told nothing
    // about registration for is refused, never posted as B2C.
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::NotReported;
    assert_eq!(codes(classify_sales_invoice(&voucher(), &facts, RAJ)), vec!["invoice_party_registration_not_reported"]);
}

#[test]
fn an_unregistered_customer_renders_without_a_gstin_in_the_shape_the_book_keys() {
    let mut v = voucher();
    let mut seen = observed();
    seen.party_gstin = None;
    seen.party_registration_type = "Unregistered/Consumer".to_string();
    seen.party_bill_wise = false;
    v.invoice.as_mut().unwrap().observed = Some(seen);
    let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let xml = render_sales_invoice_xml(&v, id, "20260310", "").unwrap();
    assert!(!xml.contains("PARTYGSTIN"));
    assert!(xml.contains("<GSTREGISTRATIONTYPE>Unregistered/Consumer</GSTREGISTRATIONTYPE>"));
    assert!(xml.contains("<STATENAME>Rajasthan</STATENAME><PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY>"));
    assert!(!xml.contains("BILLALLOCATIONS"));
}

/// The collection answer a posted invoice reads back as: the voucher the build
/// rendered, with its entries under the export's element name and the
/// attributes Tally adds, wrapped as a collection response. Built from the
/// renderer, so what is written and what is compared cannot drift apart.
fn readback_xml(v: &ImportVoucher) -> String {
    let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let rendered = render_sales_invoice_xml(v, id, "20260310", "<NARRATION>Invoice 278</NARRATION>").unwrap();
    let voucher = rendered
        .replace("<TALLYMESSAGE xmlns:UDF=\"TallyUDF\">", "")
        .replace("</TALLYMESSAGE>", "")
        .replace("LEDGERENTRIES.LIST", "ALLLEDGERENTRIES.LIST")
        .replace("<DATE>", "<ISCANCELLED>No</ISCANCELLED><ISOPTIONAL>No</ISOPTIONAL><GUID>g-1</GUID><ALTERID> 77</ALTERID><DATE>")
        // The export also gives every leg an empty allocation container.
        .replace("</ISDEEMEDPOSITIVE>", "</ISDEEMEDPOSITIVE><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST>");
    format!("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION>{voucher}</COLLECTION></DATA></ENVELOPE>")
}

fn observed_voucher() -> ImportVoucher {
    let mut v = voucher();
    v.invoice.as_mut().unwrap().observed = Some(observed());
    v
}

fn differences(v: &ImportVoucher, xml: &str) -> Vec<String> {
    let read = wire::parse_invoice_readback(xml).unwrap().expect("one voucher");
    invoice_readback_differences(v, &read, "20260310")
}

#[test]
fn what_the_renderer_writes_reads_back_clean() {
    let v = observed_voucher();
    assert_eq!(differences(&v, &readback_xml(&v)), Vec::<String>::new());
    // Unregistered, not bill-wise: no GSTIN, no allocation.
    let mut u = observed_voucher();
    let seen = u.invoice.as_mut().unwrap().observed.as_mut().unwrap();
    seen.party_gstin = None;
    seen.party_registration_type = "Unregistered/Consumer".into();
    seen.party_bill_wise = false;
    assert_eq!(differences(&u, &readback_xml(&u)), Vec::<String>::new());
}

#[test]
fn each_field_the_standard_readback_never_sees_is_compared_by_name() {
    let v = observed_voucher();
    let good = readback_xml(&v);
    let altered = |from: &str, to: &str| {
        assert!(good.contains(from), "fixture lacks {from}");
        differences(&v, &good.replace(from, to))
    };
    assert_eq!(altered(&format!("<PARTYGSTIN>{GSTIN_RJ}</PARTYGSTIN>"), ""), vec!["PARTYGSTIN"]);
    assert_eq!(altered("<PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY>", "<PLACEOFSUPPLY>Haryana</PLACEOFSUPPLY>"), vec!["PLACEOFSUPPLY"]);
    assert_eq!(altered("<STATENAME>Rajasthan</STATENAME>", "<STATENAME>Haryana</STATENAME>"), vec!["STATENAME"]);
    assert_eq!(altered("<GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE>", "<GSTREGISTRATIONTYPE>Unregistered/Consumer</GSTREGISTRATIONTYPE>"), vec!["GSTREGISTRATIONTYPE"]);
    assert_eq!(altered("<REFERENCE>278</REFERENCE>", "<REFERENCE>279</REFERENCE>"), vec!["REFERENCE"]);
    assert_eq!(altered("<REFERENCEDATE>20260310</REFERENCEDATE>", "<REFERENCEDATE>20260311</REFERENCEDATE>"), vec!["REFERENCEDATE"]);
    assert_eq!(altered("<VOUCHERTYPENAME>Sales</VOUCHERTYPENAME>", "<VOUCHERTYPENAME>Sales Acc</VOUCHERTYPENAME>"), vec!["VOUCHERTYPENAME"]);
    assert_eq!(altered("<ISINVOICE>Yes</ISINVOICE>", "<ISINVOICE>No</ISINVOICE>"), vec!["ISINVOICE"]);
    assert_eq!(altered("<ISCANCELLED>No</ISCANCELLED>", "<ISCANCELLED>Yes</ISCANCELLED>"), vec!["ISCANCELLED"]);
    assert_eq!(altered("<PARTYLEDGERNAME>Customer A</PARTYLEDGERNAME>", "<PARTYLEDGERNAME>Customer B</PARTYLEDGERNAME>"), vec!["PARTYLEDGERNAME"]);
    // A leg's amount, its side, and a leg gone.
    assert_eq!(altered("<AMOUNT>600.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Output SGST", "<AMOUNT>601.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Output SGST"), vec!["legs"]);
    // The New Ref: another name, none at all, a wrong amount.
    assert_eq!(altered("<NAME>278</NAME>", "<NAME>279</NAME>"), vec!["bill_allocation"]);
    assert_eq!(altered("<BILLTYPE>New Ref</BILLTYPE>", "<BILLTYPE>On Account</BILLTYPE>"), vec!["bill_allocation"]);
    let no_allocation = good.replace("<BILLALLOCATIONS.LIST><NAME>278</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>-11200.00</AMOUNT></BILLALLOCATIONS.LIST>", "");
    assert_eq!(differences(&v, &no_allocation), vec!["bill_allocation"]);
    // An allocation on a party that is not bill-wise is also a difference.
    let mut not_bill_wise = observed_voucher();
    not_bill_wise.invoice.as_mut().unwrap().observed.as_mut().unwrap().party_bill_wise = false;
    assert_eq!(differences(&not_bill_wise, &good), vec!["bill_allocation"]);
}

#[test]
fn an_absent_repeated_or_unreadable_voucher_is_never_a_pass() {
    let v = observed_voucher();
    let good = readback_xml(&v);
    let empty = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(wire::parse_invoice_readback(empty), Ok(None));
    let twice = good.replace("</COLLECTION>", &format!("{}</COLLECTION>", good.split("<COLLECTION>").nth(1).unwrap().split("</COLLECTION>").next().unwrap()));
    assert_eq!(wire::parse_invoice_readback(&twice), Err("invoice_readback_several_vouchers"));
    // A leg whose side did not come back makes the legs unreadable, not equal.
    let no_side = good.replace("<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>", "");
    assert!(differences(&v, &no_side).contains(&"legs_unreadable".to_string()));
    // The collection is read by the entry list's own name: a voucher that
    // came back with LEDGERENTRIES only has no legs, which is a difference.
    let renamed = good.replace("ALLLEDGERENTRIES.LIST", "LEDGERENTRIES.LIST");
    assert!(differences(&v, &renamed).contains(&"legs".to_string()));
}

#[test]
fn a_matched_invoice_whose_fields_differ_stops_being_posted_verified() {
    let mut result = json!({
        "vouchers":[{"bridge_txn_id":"t1","status":"posted_verified","alter_id":77}],
        "counts":{"posted_verified":1,"posted_divergent":0}
    });
    crate::agent::agent_import::verification::mark_invoice_readback(&mut result, "t1", &[]);
    assert_eq!(result["vouchers"][0]["status"], "posted_verified", "no difference leaves it");
    crate::agent::agent_import::verification::mark_invoice_readback(&mut result, "t2", &["PARTYGSTIN".into()]);
    assert_eq!(result["vouchers"][0]["status"], "posted_verified", "another voucher's difference is not this one's");
    crate::agent::agent_import::verification::mark_invoice_readback(&mut result, "t1", &["PARTYGSTIN".into()]);
    assert_eq!(result["vouchers"][0]["status"], "posted_divergent");
    assert_eq!(result["vouchers"][0]["diffs"][0]["invoice_fields"][0], "PARTYGSTIN");
    assert_eq!(result["counts"]["posted_verified"], 0);
    assert_eq!(result["counts"]["posted_divergent"], 1);
}

#[test]
fn a_new_ref_party_is_not_named_to_the_on_account_gate_and_every_other_leg_still_is() {
    use crate::agent::agent_import::bill_wise::named_ledgers;
    let payload = |bill_wise: bool| {
        let mut v = voucher();
        let mut seen = observed();
        seen.party_bill_wise = bill_wise;
        v.invoice.as_mut().unwrap().observed = Some(seen);
        crate::agent::agent_import::ImportPayload {
            company_guid: "g".into(),
            vouchers: vec![v],
            amends_batch_id: None,
        }
    };
    let bill_wise = payload(true);
    let named = named_ledgers(&bill_wise);
    assert!(!named.contains("Customer A"), "the New Ref party carries an allocation, not On Account");
    assert!(named.contains("Sales") && named.contains("Output CGST") && named.contains("Output SGST"));
    // A party that is not bill-wise is named as every ledger is (the gate then finds it Off).
    assert!(named_ledgers(&payload(false)).contains("Customer A"));
    assert_eq!(new_ref_party(&bill_wise.vouchers[0]), Some("Customer A"));
    assert_eq!(new_ref_party(&payload(false).vouchers[0]), None);
    let mut journal = voucher();
    journal.voucher_type = VoucherType::Journal;
    journal.invoice = None;
    assert_eq!(new_ref_party(&journal), None, "no other voucher is exempt");
}

#[test]
fn the_approval_digest_binds_every_invoice_field_and_leaves_other_vouchers_alone() {
    use crate::agent::agent_import::bill_wise::batch_content_digest;
    let base = observed_voucher();
    let digest = |v: &ImportVoucher| batch_content_digest(std::slice::from_ref(v));
    type Change = Box<dyn Fn(&mut ImportVoucher)>;
    let mut changes: Vec<Change> = vec![
        Box::new(|v| v.invoice.as_mut().unwrap().voucher_type_name = "Sales Acc".into()),
        Box::new(|v| v.invoice.as_mut().unwrap().place_of_supply = "Haryana".into()),
        Box::new(|v| v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into())),
        Box::new(|v| v.invoice.as_mut().unwrap().observed = None),
    ];
    for field in 0..6 {
        changes.push(Box::new(move |v| {
            let seen = v.invoice.as_mut().unwrap().observed.as_mut().unwrap();
            match field {
                0 => seen.voucher_type_guid = "other".into(),
                1 => seen.party_gstin = None,
                2 => seen.party_state = "Haryana".into(),
                3 => seen.party_registration_type = "Unregistered/Consumer".into(),
                4 => seen.party_bill_wise = false,
                _ => seen.company_state = "Haryana".into(),
            }
        }));
    }
    for change in &changes {
        let mut changed = base.clone();
        change(&mut changed);
        assert_ne!(digest(&base), digest(&changed));
    }
    // A voucher with no invoice detail hashes as before this field existed.
    let mut plain = voucher();
    plain.voucher_type = VoucherType::Journal;
    plain.invoice = None;
    let again = plain.clone();
    assert_eq!(digest(&plain), digest(&again));
}

/// A voucher as Tally's export returns it, written out by hand from the shape of a
/// real hand-keyed invoice read back on the lab (tags, order, an empty allocation
/// container on every leg, TYPE attributes, leading spaces in numbers), not
/// produced by this module's renderer: the comparison is checked against Tally's
/// own layout, not only against itself. Names and numbers are synthetic.
#[test]
fn a_hand_authored_export_of_a_registered_bill_wise_invoice_reads_back_clean() {
    let xml = format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO><COMPANY>0</COMPANY></CMPINFO></DESC><DATA><COLLECTION>\
<VOUCHER REMOTEID=\"22222222-2222-2222-2222-222222222222\" VCHKEY=\"k\" VCHTYPE=\"Sales\" OBJVIEW=\"Invoice Voucher View\">\
<DATE TYPE=\"Date\">20260310</DATE><EFFECTIVEDATE TYPE=\"Date\">20260310</EFFECTIVEDATE><REFERENCEDATE TYPE=\"Date\">20260310</REFERENCEDATE>\
<GUID TYPE=\"String\">g-1</GUID><PARTYGSTIN TYPE=\"String\">{GSTIN_RJ}</PARTYGSTIN><STATENAME TYPE=\"String\">Rajasthan</STATENAME>\
<PLACEOFSUPPLY TYPE=\"String\">Rajasthan</PLACEOFSUPPLY><VOUCHERTYPENAME TYPE=\"String\">Sales</VOUCHERTYPENAME><PARTYLEDGERNAME TYPE=\"String\">Customer A</PARTYLEDGERNAME>\
<VOUCHERNUMBER TYPE=\"String\">278</VOUCHERNUMBER><REFERENCE TYPE=\"String\">278</REFERENCE><GSTREGISTRATIONTYPE TYPE=\"String\">Regular</GSTREGISTRATIONTYPE>\
<ISINVOICE>Yes</ISINVOICE><ISCANCELLED TYPE=\"Logical\">No</ISCANCELLED><ISOPTIONAL TYPE=\"Logical\">No</ISOPTIONAL><ALTERID TYPE=\"Number\"> 77</ALTERID>\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Customer A</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">Yes</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">-11200.00</AMOUNT>\
<BILLALLOCATIONS.LIST><NAME TYPE=\"String\">278</NAME><BILLTYPE TYPE=\"String\">New Ref</BILLTYPE><AMOUNT TYPE=\"Amount\">-11200.00</AMOUNT></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Sales</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Output CGST</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">600.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Output SGST</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">600.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
</VOUCHER></COLLECTION></DATA></BODY></ENVELOPE>"
    );
    let v = observed_voucher();
    assert_eq!(differences(&v, &xml), Vec::<String>::new());
    // An allocation whose amount did not come back is a difference, not an absence.
    let no_amount = xml.replace("<BILLTYPE TYPE=\"String\">New Ref</BILLTYPE><AMOUNT TYPE=\"Amount\">-11200.00</AMOUNT>", "<BILLTYPE TYPE=\"String\">New Ref</BILLTYPE>");
    assert_eq!(differences(&v, &no_amount), vec!["bill_allocation"]);
    // A stray allocation name on a leg that should carry none shows too.
    let stray = xml.replace("<LEDGERNAME TYPE=\"String\">Sales</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST>", "<LEDGERNAME TYPE=\"String\">Sales</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST><NAME TYPE=\"String\">X</NAME></BILLALLOCATIONS.LIST>");
    assert_eq!(differences(&v, &stray), vec!["bill_allocation"]);
}

#[test]
fn an_amount_too_large_to_multiply_matches_no_rate_instead_of_overflowing() {
    let mut v = voucher();
    let huge = "9".repeat(36) + ".00";
    v.entries[0].amount = huge.clone();
    v.entries[1].amount = huge.clone();
    v.entries[2].amount = huge.clone();
    v.entries[3].amount = huge;
    let result = classify_sales_invoice(&v, &good_facts(), RAJ);
    assert!(result.is_err(), "must refuse, not panic or wrap");
}
