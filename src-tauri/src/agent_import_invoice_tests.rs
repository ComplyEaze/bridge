use super::*;
use crate::agent::agent_import::VoucherType;
use serde_json::json;

const RAJ: &str = "Rajasthan";
// Invented GSTINs that cannot be issued: the fourth letter of the PAN part
// names no kind of holder. The check digit of each was computed outside this
// code, by a few lines of Python following the published scheme (over the first
// 14 characters, each valued by its place in 0-9A-Z, the odd places weighted 1
// and the even places 2, add quotient and remainder of each product by 36; the
// digit is the character at (36 - sum mod 36) mod 36). The same script gives
// the digit of the published worked example.
const GSTIN_MH: &str = "27ZZZZZ0000Z1ZQ";
const GSTIN_RJ: &str = "08ZZZZZ0000Z1ZQ";

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
        (
            "Customer A",
            &["Sundry Debtors", "Current Assets"],
            DutyHead::NotTax,
        ),
        ("Sales", &["Sales Accounts"], DutyHead::NotTax),
        (
            "Output CGST",
            &["Duties & Taxes", "Current Liabilities"],
            DutyHead::Cgst,
        ),
        (
            "Output SGST",
            &["Duties & Taxes", "Current Liabilities"],
            DutyHead::State,
        ),
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
    assert!(!gstin_valid("27ZZZZZ0000Z1ZR"), "wrong check digit");
    assert!(!gstin_valid("27ZZZZZ0000Z1Z"), "too short");
    assert!(!gstin_valid("27zzzzz0000z1zq"), "lower case");
    assert!(!gstin_valid("27ZZZZZ0000Z0ZQ"), "entity digit 0");
    assert!(!gstin_valid("27ZZZZZ0000Z1XQ"), "14th character is Z");
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
    assert_eq!(
        validate_invoice_voucher(&supplied),
        Ok(()),
        "a saved record carries it"
    );
    assert_eq!(
        refuse_supplied_observed(&[supplied]),
        Err("invoice_observed_not_input".to_string())
    );
    assert_eq!(refuse_supplied_observed(&[voucher()]), Ok(()));
    assert_eq!(
        mutated(&|v| v.voucher_number = Some("A\"B".into())),
        "invoice_number_invalid"
    );
    assert_eq!(
        mutated(&|v| v.voucher_number = Some(" 278".into())),
        "invoice_number_invalid"
    );
    assert_eq!(
        validate_invoice_voucher(&{
            let mut v = voucher();
            v.voucher_number = Some("INV-26/27-0042".into());
            v
        }),
        Ok(())
    );
    // Outside GST rule 46(b): a space, a period, an underscore, over 16 characters.
    for bad in ["INV 0042", "INV.0042", "INV_0042", "12345678901234567"] {
        assert_eq!(
            mutated(&|v| v.voucher_number = Some(bad.into())),
            "invoice_number_invalid",
            "{bad}"
        );
    }
    assert_eq!(
        mutated(&|v| v.voucher_number = None),
        "invoice_number_required"
    );
    assert_eq!(
        mutated(&|v| v.reference = Some("r".into())),
        "invoice_reference_not_for_sales"
    );
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
    assert_eq!(
        validate_invoice_voucher(&journal),
        Err("invoice_detail_on_non_invoice".to_string())
    );
}

#[test]
fn roles_come_from_groups_and_duty_heads_never_from_names() {
    let roles = classify_sales_invoice(&voucher(), &good_facts(), RAJ).unwrap();
    assert_eq!(
        roles,
        InvoiceRoles {
            party: 0,
            sales: vec![1],
            cgst: 2,
            state_tax: 3,
            round_off: None
        }
    );
    // The same names, but the ledgers say otherwise: the head decides, not "Output SGST".
    let mut facts = good_facts();
    facts.get_mut("Output SGST").unwrap().duty_head = DutyHead::Cgst;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        vec![
            "invoice_needs_exactly_one_cgst_ledger",
            "invoice_needs_exactly_one_state_tax_ledger"
        ]
    );
}

#[test]
fn each_master_defect_is_refused_with_its_own_code() {
    let check = |change: &dyn Fn(&mut BTreeMap<String, LedgerFacts>), expect: &str| {
        let mut facts = good_facts();
        change(&mut facts);
        let got = codes(classify_sales_invoice(&voucher(), &facts, RAJ));
        assert!(
            got.contains(&expect_static(expect)),
            "{expect}: got {got:?}"
        );
    };
    check(
        &|f| f.get_mut("Customer A").unwrap().bill_wise = None,
        "invoice_party_bill_wise_unknown",
    );
    check(
        &|f| f.get_mut("Customer A").unwrap().gstin = GstinEvidence::NoneInForce,
        "invoice_party_gstin_missing",
    );
    check(
        &|f| {
            f.get_mut("Customer A").unwrap().gstin =
                GstinEvidence::InForce("08ZZZZZ0000Z1ZR".into())
        },
        "invoice_party_gstin_invalid",
    );
    check(
        &|f| f.get_mut("Customer A").unwrap().gstin = GstinEvidence::InForce(GSTIN_MH.into()),
        "invoice_place_of_supply_not_party_state",
    );
    check(
        &|f| f.get_mut("Customer A").unwrap().registration_type = Some("Composition".into()),
        "invoice_party_not_regular",
    );
    check(
        &|f| f.get_mut("Customer A").unwrap().reserved_groups = vec![],
        "invoice_ledger_group_unresolved",
    );
    check(
        &|f| f.get_mut("Sales").unwrap().reserved_groups = vec!["Direct Incomes".into()],
        "invoice_ledger_role_unknown",
    );
    check(
        &|f| f.get_mut("Output CGST").unwrap().duty_head = DutyHead::Igst,
        "invoice_igst_not_supported",
    );
    check(
        &|f| f.get_mut("Output CGST").unwrap().duty_head = DutyHead::Cess,
        "invoice_cess_not_supported",
    );
    check(
        &|f| f.get_mut("Output CGST").unwrap().duty_head = DutyHead::NotTax,
        "invoice_tax_ledger_head_unknown",
    );
    check(
        &|f| {
            f.remove("Sales");
        },
        "invoice_ledger_not_observed",
    );
}

fn expect_static(code: &str) -> &'static str {
    // Test-only: the codes under test are string literals in the module.
    Box::leak(code.to_string().into_boxed_str())
}

#[test]
fn a_party_that_is_not_bill_wise_is_admitted() {
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().bill_wise = Some(false);
    assert!(classify_sales_invoice(&voucher(), &facts, RAJ).is_ok());
}

#[test]
fn arithmetic_must_close_and_tax_must_be_a_slab() {
    let mut v = voucher();
    v.entries[0].amount = "11201.00".to_string();
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_party_amount_does_not_close"]
    );
    let mut v = voucher();
    v.entries[2].amount = "700.00".to_string();
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_cgst_and_state_tax_differ"]
    );
    // A sale of 10.00 carrying 0.02 of tax is not 5 percent (the check is paise, not rupees).
    let mut v = voucher();
    v.entries[0].amount = "10.02".to_string();
    v.entries[1].amount = "10.00".to_string();
    v.entries[2].amount = "0.01".to_string();
    v.entries[3].amount = "0.01".to_string();
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_tax_matches_no_slab_rate"]
    );
    // A few paise of per-line rounding is within tolerance: 12 percent of 10,000.03.
    let mut v = voucher();
    v.entries[1].amount = "10000.03".to_string();
    v.entries[0].amount = "11200.03".to_string();
    assert!(classify_sales_invoice(&v, &good_facts(), RAJ).is_ok());
    // An odd paisa between the heads (a bill whose total tax is odd): either
    // head may carry it, and the party total follows the legs.
    for (cgst, state) in [("4.69", "4.68"), ("4.68", "4.69")] {
        let mut v = voucher();
        v.entries[0].amount = "196.87".to_string();
        v.entries[1].amount = "187.50".to_string();
        v.entries[2].amount = cgst.to_string();
        v.entries[3].amount = state.to_string();
        assert!(
            classify_sales_invoice(&v, &good_facts(), RAJ).is_ok(),
            "{cgst} / {state}"
        );
    }
    // Two paise between the heads is refused, though each is within the slab.
    let mut v = voucher();
    v.entries[0].amount = "196.88".to_string();
    v.entries[1].amount = "187.50".to_string();
    v.entries[2].amount = "4.70".to_string();
    v.entries[3].amount = "4.68".to_string();
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_cgst_and_state_tax_differ"]
    );
    // EACH head is held to the slab: 5 percent of 10,000.00 is 250.00 a head;
    // 250.05 is within five paise, 250.06 is not, and the heads differ by one.
    let mut v = voucher();
    v.entries[0].amount = "10500.11".to_string();
    v.entries[2].amount = "250.05".to_string();
    v.entries[3].amount = "250.06".to_string();
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_tax_matches_no_slab_rate"]
    );
    v.entries[2].amount = "250.06".to_string();
    v.entries[3].amount = "250.05".to_string();
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_tax_matches_no_slab_rate"]
    );
    v.entries[0].amount = "10500.09".to_string();
    v.entries[2].amount = "250.05".to_string();
    v.entries[3].amount = "250.04".to_string();
    assert!(classify_sales_invoice(&v, &good_facts(), RAJ).is_ok());
    // 7 percent is no GST slab: 700 + 700.
    let mut v = voucher();
    v.entries[2].amount = "700.00".to_string();
    v.entries[3].amount = "700.00".to_string();
    v.entries[0].amount = "11400.00".to_string();
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_tax_matches_no_slab_rate"]
    );
}

#[test]
fn a_round_off_is_taken_by_side_and_bounded_under_a_rupee() {
    // Customer owes 11,200.40: a credit round off of 0.40.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11200.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
    assert_eq!(validate_invoice_voucher(&v), Ok(()));
    assert_eq!(
        classify_sales_invoice(&v, &good_facts(), RAJ)
            .unwrap()
            .round_off,
        Some(4)
    );
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
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_party_amount_does_not_close"]
    );
    // A round off of a rupee or more is refused.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11201.00".to_string();
    v.entries.push(entry("Round Off", "1.00", EntrySide::Cr));
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_round_off_too_large"]
    );
    // The round off ledger must sit under Indirect Expenses or Incomes.
    let mut facts = good_facts();
    facts.get_mut("Round Off").unwrap().reserved_groups = vec!["Current Assets".into()];
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11200.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
    assert!(
        codes(classify_sales_invoice(&v, &facts, RAJ)).contains(&"invoice_round_off_ledger_group")
    );
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

/// The bytes the renderer writes, pinned. The rehearsal of 7 Oct 2026 posted
/// two invoices the renderer wrote in this shape and Tally took both (section
/// 9.16). Their import files are not committed, so the bytes pinned here are
/// the renderer's own, with invented names.
#[test]
fn the_rendered_sales_invoice_is_pinned_byte_for_byte() {
    let mut v = voucher();
    v.invoice.as_mut().unwrap().observed = Some(observed());
    let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let xml =
        render_sales_invoice_xml(&v, id, "20260310", "<NARRATION>Invoice 278</NARRATION>").unwrap();
    let expected = "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"11111111-1111-1111-1111-111111111111\" VCHTYPE=\"Sales\" ACTION=\"Create\" OBJVIEW=\"Invoice Voucher View\"><DATE>20260310</DATE><EFFECTIVEDATE>20260310</EFFECTIVEDATE><REFERENCEDATE>20260310</REFERENCEDATE><REFERENCE>278</REFERENCE><VOUCHERTYPENAME>Sales</VOUCHERTYPENAME><VOUCHERNUMBER>278</VOUCHERNUMBER><PARTYLEDGERNAME>Customer A</PARTYLEDGERNAME><PARTYNAME>Customer A</PARTYNAME><BASICBASEPARTYNAME>Customer A</BASICBASEPARTYNAME><PARTYMAILINGNAME>Customer A</PARTYMAILINGNAME><BASICBUYERNAME>Customer A</BASICBUYERNAME><CONSIGNEEMAILINGNAME>Customer A</CONSIGNEEMAILINGNAME><PARTYGSTIN>@GSTIN@</PARTYGSTIN><STATENAME>Rajasthan</STATENAME><CONSIGNEESTATENAME>Rajasthan</CONSIGNEESTATENAME><PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY><GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE><VATDEALERTYPE>Regular</VATDEALERTYPE><COUNTRYOFRESIDENCE>India</COUNTRYOFRESIDENCE><CONSIGNEECOUNTRYNAME>India</CONSIGNEECOUNTRYNAME><PERSISTEDVIEW>Invoice Voucher View</PERSISTEDVIEW><VCHENTRYMODE>Accounting Invoice</VCHENTRYMODE><ISINVOICE>Yes</ISINVOICE><NARRATION>Invoice 278</NARRATION><LEDGERENTRIES.LIST><LEDGERNAME>Customer A</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-11200.00</AMOUNT><BILLALLOCATIONS.LIST><NAME>278</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>-11200.00</AMOUNT></BILLALLOCATIONS.LIST></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Sales</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>10000.00</AMOUNT></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Output CGST</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>600.00</AMOUNT></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Output SGST</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>600.00</AMOUNT></LEDGERENTRIES.LIST></VOUCHER></TALLYMESSAGE>";
    assert_eq!(xml, expected.replace("@GSTIN@", GSTIN_RJ));
    assert!(
        !xml.contains("ALLLEDGERENTRIES"),
        "an invoice view silently drops ALLLEDGERENTRIES"
    );
    // A party that is not bill-wise gets no New Ref; and an unobserved voucher does not render.
    let mut not_bill_wise = v.clone();
    not_bill_wise
        .invoice
        .as_mut()
        .unwrap()
        .observed
        .as_mut()
        .unwrap()
        .party_bill_wise = false;
    assert!(
        !render_sales_invoice_xml(&not_bill_wise, id, "20260310", "")
            .unwrap()
            .contains("BILLALLOCATIONS")
    );
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
    // A state name with an ampersand (three in the list) is escaped where it is
    // written, as the voucher's state and as the consignee's.
    let mut seen = observed();
    seen.party_state = "Jammu & Kashmir".to_string();
    let mut in_jk = v.clone();
    in_jk.invoice.as_mut().unwrap().observed = Some(seen);
    let xml = render_sales_invoice_xml(&in_jk, id, "20260310", "").unwrap();
    assert!(!xml.contains("Jammu & Kashmir"));
    assert!(xml.contains("<CONSIGNEESTATENAME>Jammu &amp; Kashmir</CONSIGNEESTATENAME>"));
    // The type's name is also written as an attribute: a quote in it stays inside.
    v.invoice.as_mut().unwrap().voucher_type_name = "Sales \"A\"".to_string();
    let xml = render_sales_invoice_xml(&v, id, "20260310", "").unwrap();
    assert!(
        xml.contains("VCHTYPE=\"Sales &quot;A&quot;\" ACTION="),
        "{xml}"
    );
}

#[test]
fn the_supply_must_be_made_in_the_company_state_and_an_unregistered_customer_is_admitted() {
    // The shape posted is CGST plus state tax: a supply made elsewhere is refused.
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &good_facts(), "Haryana")),
        vec!["invoice_place_of_supply_not_company_state"]
    );
    // An unregistered customer: an entry in force that names no GSTIN and
    // says the customer is unregistered.
    let mut facts = good_facts();
    let party = facts.get_mut("Customer A").unwrap();
    party.gstin = GstinEvidence::NoneInForce;
    party.registration_type = Some("Unregistered/Consumer".into());
    party.bill_wise = Some(false);
    assert!(classify_sales_invoice(&voucher(), &facts, RAJ).is_ok());
    // The same entry with no type says nothing: not read as unregistered.
    let type_not_reported = vec!["invoice_party_registration_type_not_reported"];
    facts.get_mut("Customer A").unwrap().registration_type = None;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        type_not_reported
    );
    // No entry in force on the date (the history starts later): refused.
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::NoEntryInForce;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        vec!["invoice_party_registration_not_in_force"]
    );
    // A GSTIN in the flat field only, or one in force whose entry names no
    // type: the kind of registration is not known, so not read as Regular.
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::FlatFieldOnly;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        type_not_reported
    );
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::InForce(GSTIN_RJ.to_string());
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        type_not_reported
    );
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::NoneInForce;
    // Registered on the ledger but no number in force: refused, not guessed unregistered.
    facts.get_mut("Customer A").unwrap().registration_type = Some("Regular".into());
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        vec!["invoice_party_gstin_missing"]
    );
    // A GSTIN history Bridge cannot settle refuses.
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::Unsettled;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        vec!["invoice_party_gstin_unsettled"]
    );
    // Silence is not evidence of "unregistered": a ledger Tally told nothing
    // about registration for is refused, never posted as B2C.
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::NotReported;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ)),
        vec!["invoice_party_registration_not_reported"]
    );
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
    // The buyer and consignee block as the keyed unregistered invoice read it:
    // the dealer type is Regular there too.
    assert!(xml.contains(
        "<STATENAME>Rajasthan</STATENAME><CONSIGNEESTATENAME>Rajasthan</CONSIGNEESTATENAME><PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY><GSTREGISTRATIONTYPE>Unregistered/Consumer</GSTREGISTRATIONTYPE><VATDEALERTYPE>Regular</VATDEALERTYPE>"
    ));
    assert!(!xml.contains("BILLALLOCATIONS"));
}

/// The collection answer a posted invoice reads back as: the voucher the build
/// rendered, with its entries under the export's element name and the
/// attributes Tally adds, wrapped as a collection response. Built from the
/// renderer, so what is written and what is compared cannot drift apart.
fn readback_xml(v: &ImportVoucher) -> String {
    let id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
    let rendered =
        render_sales_invoice_xml(v, id, "20260310", "<NARRATION>Invoice 278</NARRATION>").unwrap();
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
    match wire::parse_invoice_readback(xml).unwrap() {
        wire::Readback::One(read) => invoice_readback_differences(v, &read, "20260310"),
        other => panic!("one voucher expected, got {other:?}"),
    }
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

/// The fields the standard readback also sees are compared here as well: a
/// read-back whose number, date or optional flag differs names that field.
#[test]
fn the_number_the_date_and_the_optional_flag_are_compared_by_name() {
    let v = observed_voucher();
    let good = readback_xml(&v);
    let altered = |from: &str, to: &str| {
        assert!(good.contains(from), "fixture lacks {from}");
        differences(&v, &good.replace(from, to))
    };
    assert_eq!(
        altered(
            "<VOUCHERNUMBER>278</VOUCHERNUMBER>",
            "<VOUCHERNUMBER>279</VOUCHERNUMBER>"
        ),
        vec!["VOUCHERNUMBER"]
    );
    assert_eq!(
        altered("<DATE>20260310</DATE>", "<DATE>20260311</DATE>"),
        vec!["DATE"]
    );
    assert_eq!(
        altered(
            "<ISOPTIONAL>No</ISOPTIONAL>",
            "<ISOPTIONAL>Yes</ISOPTIONAL>"
        ),
        vec!["ISOPTIONAL"]
    );
}

#[test]
fn each_field_the_standard_readback_never_sees_is_compared_by_name() {
    let v = observed_voucher();
    let good = readback_xml(&v);
    let altered = |from: &str, to: &str| {
        assert!(good.contains(from), "fixture lacks {from}");
        differences(&v, &good.replace(from, to))
    };
    assert_eq!(
        altered(&format!("<PARTYGSTIN>{GSTIN_RJ}</PARTYGSTIN>"), ""),
        vec!["PARTYGSTIN"]
    );
    assert_eq!(
        altered(
            "<PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY>",
            "<PLACEOFSUPPLY>Haryana</PLACEOFSUPPLY>"
        ),
        vec!["PLACEOFSUPPLY"]
    );
    assert_eq!(
        altered(
            "<STATENAME>Rajasthan</STATENAME>",
            "<STATENAME>Haryana</STATENAME>"
        ),
        vec!["STATENAME"]
    );
    assert_eq!(
        altered(
            "<GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE>",
            "<GSTREGISTRATIONTYPE>Unregistered/Consumer</GSTREGISTRATIONTYPE>"
        ),
        vec!["GSTREGISTRATIONTYPE"]
    );
    assert_eq!(
        altered("<REFERENCE>278</REFERENCE>", "<REFERENCE>279</REFERENCE>"),
        vec!["REFERENCE"]
    );
    assert_eq!(
        altered(
            "<REFERENCEDATE>20260310</REFERENCEDATE>",
            "<REFERENCEDATE>20260311</REFERENCEDATE>"
        ),
        vec!["REFERENCEDATE"]
    );
    assert_eq!(
        altered(
            "<VOUCHERTYPENAME>Sales</VOUCHERTYPENAME>",
            "<VOUCHERTYPENAME>Sales Acc</VOUCHERTYPENAME>"
        ),
        vec!["VOUCHERTYPENAME"]
    );
    assert_eq!(
        altered("<ISINVOICE>Yes</ISINVOICE>", "<ISINVOICE>No</ISINVOICE>"),
        vec!["ISINVOICE"]
    );
    assert_eq!(
        altered(
            "<ISCANCELLED>No</ISCANCELLED>",
            "<ISCANCELLED>Yes</ISCANCELLED>"
        ),
        vec!["ISCANCELLED"]
    );
    assert_eq!(
        altered(
            "<PARTYLEDGERNAME>Customer A</PARTYLEDGERNAME>",
            "<PARTYLEDGERNAME>Customer B</PARTYLEDGERNAME>"
        ),
        vec!["PARTYLEDGERNAME"]
    );
    // A leg's amount.
    assert_eq!(altered("<AMOUNT>600.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Output SGST", "<AMOUNT>601.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Output SGST"), vec!["legs"]);
    // The New Ref: another name, none at all, a wrong amount.
    assert_eq!(
        altered("<NAME>278</NAME>", "<NAME>279</NAME>"),
        vec!["bill_allocation"]
    );
    assert_eq!(
        altered(
            "<BILLTYPE>New Ref</BILLTYPE>",
            "<BILLTYPE>On Account</BILLTYPE>"
        ),
        vec!["bill_allocation"]
    );
    let no_allocation = good.replace("<BILLALLOCATIONS.LIST><NAME>278</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>-11200.00</AMOUNT></BILLALLOCATIONS.LIST>", "");
    assert_eq!(differences(&v, &no_allocation), vec!["bill_allocation"]);
    // An allocation on a party that is not bill-wise is also a difference.
    let mut not_bill_wise = observed_voucher();
    not_bill_wise
        .invoice
        .as_mut()
        .unwrap()
        .observed
        .as_mut()
        .unwrap()
        .party_bill_wise = false;
    assert_eq!(differences(&not_bill_wise, &good), vec!["bill_allocation"]);
}

#[test]
fn an_absent_repeated_or_unreadable_voucher_is_never_a_pass() {
    let v = observed_voucher();
    let good = readback_xml(&v);
    let empty = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA><COLLECTION></COLLECTION></DATA></ENVELOPE>";
    assert_eq!(
        wire::parse_invoice_readback(empty),
        Ok(wire::Readback::Absent)
    );
    let none = |difference: &str| (vec![difference.to_string()], None, None);
    assert_eq!(
        readback_outcome(wire::parse_invoice_readback(empty), &v, "20260310"),
        none("invoice_not_found")
    );
    // A success envelope with no collection in it is not an empty collection.
    let no_collection = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><DATA></DATA></ENVELOPE>";
    assert_eq!(
        wire::parse_invoice_readback(no_collection),
        Err("invoice_read_collection_absent")
    );
    assert_eq!(
        readback_outcome(wire::parse_invoice_readback(no_collection), &v, "20260310"),
        none("invoice_readback_unreadable")
    );
    let twice = good.replace(
        "</COLLECTION>",
        &format!(
            "{}</COLLECTION>",
            good.split("<COLLECTION>")
                .nth(1)
                .unwrap()
                .split("</COLLECTION>")
                .next()
                .unwrap()
        ),
    );
    assert_eq!(
        wire::parse_invoice_readback(&twice),
        Ok(wire::Readback::Several)
    );
    assert_eq!(
        readback_outcome(wire::parse_invoice_readback(&twice), &v, "20260310"),
        none("invoice_number_not_unique")
    );
    // The one voucher: its differences (none), and its AlterID and GUID as read.
    let (differences_found, alter_id, guid) =
        readback_outcome(wire::parse_invoice_readback(&good), &v, "20260310");
    assert_eq!(differences_found, Vec::<String>::new());
    assert_eq!(
        (alter_id.as_deref(), guid.as_deref()),
        (Some("77"), Some("g-1"))
    );
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
    assert_eq!(
        result["vouchers"][0]["status"], "posted_verified",
        "no difference leaves it"
    );
    crate::agent::agent_import::verification::mark_invoice_readback(
        &mut result,
        "t2",
        &["PARTYGSTIN".into()],
    );
    assert_eq!(
        result["vouchers"][0]["status"], "posted_verified",
        "another voucher's difference is not this one's"
    );
    crate::agent::agent_import::verification::mark_invoice_readback(
        &mut result,
        "t1",
        &["PARTYGSTIN".into()],
    );
    assert_eq!(result["vouchers"][0]["status"], "posted_divergent");
    assert_eq!(
        result["vouchers"][0]["diffs"][0]["invoice_fields"][0],
        "PARTYGSTIN"
    );
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
    assert!(
        !named.contains("Customer A"),
        "the New Ref party carries an allocation, not On Account"
    );
    assert!(
        named.contains("Sales") && named.contains("Output CGST") && named.contains("Output SGST")
    );
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
    // A voucher with no invoice detail still binds its voucher number. The golden
    // pin of how a plain voucher is encoded lives in agent_import_bill_wise_tests.rs
    // (the_party_digest_of_a_known_batch_is_pinned); this checks only the number.
    let mut plain = voucher();
    plain.voucher_type = VoucherType::Journal;
    plain.invoice = None;
    let first = digest(&plain);
    plain.voucher_number = Some("1".into());
    assert_ne!(
        first,
        digest(&plain),
        "the digest still binds the voucher number"
    );
}

/// HAND-AUTHORED, not a capture. The layout (tags, order, an empty allocation
/// container on every leg, TYPE attributes, leading spaces in numbers) follows
/// a hand-keyed invoice read back on the lab; the party GSTIN and the New Ref
/// allocation were on no invoice read when it was written, and were assumed. It checks the
/// comparison against a layout this module's renderer did not produce. The
/// rehearsal's captured read-backs are compared in the test after it. Names
/// and numbers are synthetic.
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
    let no_amount = xml.replace(
        "<BILLTYPE TYPE=\"String\">New Ref</BILLTYPE><AMOUNT TYPE=\"Amount\">-11200.00</AMOUNT>",
        "<BILLTYPE TYPE=\"String\">New Ref</BILLTYPE>",
    );
    assert_eq!(differences(&v, &no_amount), vec!["bill_allocation"]);
    // A stray allocation name on a leg that should carry none shows too.
    let stray = xml.replace("<LEDGERNAME TYPE=\"String\">Sales</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST>", "<LEDGERNAME TYPE=\"String\">Sales</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST><NAME TYPE=\"String\">X</NAME></BILLALLOCATIONS.LIST>");
    assert_eq!(differences(&v, &stray), vec!["bill_allocation"]);
}

/// One of the Sales rehearsal's captured read-backs (the `sales-rehearsal`
/// set; its PROVENANCE table is beside the fixtures), decoded and compared
/// with `expected` as a post compares them. In the registered customer's two
/// read-backs the GSTIN is a substituted token (that table says so), so its
/// comparison there shows the element is read, not what Tally stored.
fn rehearsal_differences(expected: &ImportVoucher, bytes: &[u8], date: &str) -> Vec<String> {
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    match wire::parse_invoice_readback(&xml).unwrap() {
        wire::Readback::One(read) => invoice_readback_differences(expected, &read, date),
        other => panic!("one voucher expected, got {other:?}"),
    }
}

/// An invoice of the rehearsal, written here from its build plan and from what
/// the build recorded as observed: the keyed `Sales Manual` type, the lab
/// company's customers and ledgers. The narration is left out; the comparison
/// does not read it.
fn rehearsal_invoice(
    (number, date): (&str, &str),
    party: &str,
    registered_bill_wise: bool,
    credits: &[(&str, &str)],
    total: &str,
) -> ImportVoucher {
    let round_off = credits
        .iter()
        .find(|(ledger, _)| *ledger == "Round Off")
        .map(|(ledger, _)| ledger.to_string());
    let mut entries = vec![entry(party, total, EntrySide::Dr)];
    entries.extend(
        credits
            .iter()
            .map(|(ledger, amount)| entry(ledger, amount, EntrySide::Cr)),
    );
    ImportVoucher {
        bridge_txn_id: "rehearsal".to_string(),
        date: date.to_string(),
        voucher_type: VoucherType::Sales,
        narration: None,
        reference: None,
        voucher_number: Some(number.to_string()),
        invoice: Some(InvoiceDetail {
            voucher_type_name: "Sales Manual".to_string(),
            place_of_supply: RAJ.to_string(),
            round_off_ledger: round_off,
            observed: Some(InvoiceObserved {
                voucher_type_guid: "ae1490be-52c5-4544-9ffc-4b7da85f9797-00000106".to_string(),
                party_gstin: registered_bill_wise.then(|| GSTIN_RJ.to_string()),
                party_state: RAJ.to_string(),
                party_registration_type: if registered_bill_wise {
                    "Regular"
                } else {
                    "Unregistered/Consumer"
                }
                .to_string(),
                party_bill_wise: registered_bill_wise,
                company_state: RAJ.to_string(),
            }),
        }),
        entries,
    }
}

/// The two invoices the rehearsal posted (7 Oct 2026, the synthetic lab
/// company), each written from its build plan, against Tally's answer to the
/// read-back request sent raw after the post (not the binary's own read of
/// it): no difference. The invoice keyed by hand to the registered customer
/// differs from what a build would write only by the reference and its date,
/// which a keyed invoice of that book does not carry.
#[test]
fn the_rehearsals_posted_invoices_read_back_as_planned_on_the_captured_answers() {
    let registered = rehearsal_invoice(
        ("TG/25-26/002", "2026-03-11"),
        "TG Buyer Regular RJ",
        true,
        &[
            ("Sales - Goods", "1234.50"),
            ("Output CGST", "111.11"),
            ("Output SGST", "111.11"),
            ("Round Off", "0.28"),
        ],
        "1457.00",
    );
    assert_eq!(
        rehearsal_differences(
            &registered,
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-readback-posted-registered.utf16le.xml"),
            "20260311",
        ),
        Vec::<String>::new()
    );
    let ten_thousand_at_18_percent: &[(&str, &str)] = &[
        ("Sales - Goods", "10000.00"),
        ("Output CGST", "900.00"),
        ("Output SGST", "900.00"),
    ];
    let unregistered = rehearsal_invoice(
        ("TG/25-26/003", "2026-03-11"),
        "TG Buyer Unregistered RJ",
        false,
        ten_thousand_at_18_percent,
        "11800.00",
    );
    assert_eq!(
        rehearsal_differences(
            &unregistered,
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-readback-posted-unregistered.utf16le.xml"),
            "20260311",
        ),
        Vec::<String>::new()
    );
    // Each read-back is that invoice's own: against the other invoice it
    // differs in every field the two do not share.
    assert_eq!(
        rehearsal_differences(
            &unregistered,
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-readback-posted-registered.utf16le.xml"),
            "20260311",
        ),
        vec![
            "VOUCHERNUMBER",
            "REFERENCE",
            "PARTYLEDGERNAME",
            "PARTYGSTIN",
            "GSTREGISTRATIONTYPE",
            "legs",
            "bill_allocation"
        ]
    );
    let keyed = rehearsal_invoice(
        ("TG/25-26/001", "2026-03-10"),
        "TG Buyer Regular RJ",
        true,
        ten_thousand_at_18_percent,
        "11800.00",
    );
    assert_eq!(
        rehearsal_differences(
            &keyed,
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-readback-keyed-registered.utf16le.xml"),
            "20260310",
        ),
        vec!["REFERENCE", "REFERENCEDATE"]
    );
}

#[test]
fn an_amount_too_large_to_add_or_multiply_is_refused_instead_of_overflowing() {
    // The three credits cannot be added: refused at the sum.
    let mut v = voucher();
    let huge = "9".repeat(36) + ".00";
    for entry in &mut v.entries {
        entry.amount = huge.clone();
    }
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_amount_invalid"]
    );
    // The sum fits and the slab check's multiplication does not: no rate matches.
    let mut v = voucher();
    v.entries[1].amount = "1".to_string() + &"0".repeat(35) + ".00";
    v.entries[2].amount = "1.00".to_string();
    v.entries[3].amount = "1.00".to_string();
    v.entries[0].amount = "1".to_string() + &"0".repeat(34) + "2.00";
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ)),
        vec!["invoice_tax_matches_no_slab_rate"]
    );
}

/// The two invoice-number refusals name GST rule 46(b) and what it allows, so
/// a person can renumber, and tell the assistant never to alter the number.
#[test]
fn an_invoice_number_refusal_names_rule_46b_and_what_it_allows() {
    let invalid = refuse_value("invoice_number_invalid", "INV 0042").to_json();
    assert_eq!(invalid["code"], "invoice_number_invalid");
    assert_eq!(invalid["detail"], "INV 0042");
    assert_eq!(
        invalid["remediation"],
        "An invoice number must follow GST rule 46(b): at most 16 characters, made only of \
         letters, digits, the hyphen (-) and the slash (/), and unique in the financial year. \
         This number is longer than that or has a space or another character the rule does not \
         allow, so ComplyEaze Bridge refuses it. Do not shorten or change the number yourself: \
         tell the user the rule and ask which invoice number to use."
    );
    let used = refuse_value("invoice_number_already_used", "278").to_json();
    assert_eq!(
        used["remediation"],
        "GST rule 46(b) needs an invoice number to be unique in the financial year (1 April to \
         31 March), and the read of that year's Sales vouchers found this number already in use, \
         so ComplyEaze Bridge refuses it. The voucher that has the number may be this same invoice, \
         already in the book: open it in Tally first, and if an earlier batch for this invoice \
         was sent, run verify_import on it. Only if it is another invoice, tell the user the \
         number is in use and ask which invoice number to use. Do not pick another number \
         yourself."
    );
    // The structural refusal reaches the caller as a failure code, which takes
    // the same text.
    assert_eq!(
        crate::agent::refusal_remediation("invoice_number_invalid"),
        invalid["remediation"].as_str()
    );
    // A code with no next step of its own carries none, never filler.
    let plain = refuse("invoice_party_missing").to_json();
    assert_eq!(
        plain,
        json!({"code": "invoice_party_missing", "detail": ""})
    );
}

#[test]
fn the_book_size_guard_admits_at_each_bound_and_refuses_one_past_it() {
    assert_eq!(voucher_mark_refusal(Some(25_000)), None);
    assert_eq!(
        voucher_mark_refusal(Some(25_001)),
        Some(refuse_value(
            "invoice_book_too_many_vouchers",
            "voucher mark 25001"
        ))
    );
    assert_eq!(
        voucher_mark_refusal(None),
        Some(refuse("invoice_book_size_unknown"))
    );
    assert!(mark_admits(5_000));
    assert!(!mark_admits(5_001));
    assert_eq!(ledger_count_refusal(Some(2_000), 9_000), None);
    assert_eq!(
        ledger_count_refusal(Some(2_001), 9_000),
        Some(refuse_value("invoice_book_too_large", "2001 ledgers"))
    );
    // A count Tally did not give never admits, and is unknown, not large.
    assert_eq!(
        ledger_count_refusal(None, 9_000),
        Some(refuse_value(
            "invoice_book_size_unknown",
            "master mark 9000"
        ))
    );
}

#[test]
fn the_invoice_read_back_must_be_the_voucher_the_batch_was_attributed() {
    let matched = json!({"guid": "ABCD-0001", "alter_id": 77});
    let same = |guid, alter_id| readback_identity_differences(guid, alter_id, &matched);
    assert_eq!(same(Some("abcd-0001"), Some(" 77")), Vec::<String>::new());
    assert_eq!(same(Some("abcd-0002"), Some("77")), vec!["guid"]);
    assert_eq!(same(Some("abcd-0001"), Some("78")), vec!["alter_id"]);
    // A value that did not come back is a difference on either read.
    assert_eq!(same(None, None), vec!["guid", "alter_id"]);
    assert_eq!(same(Some("abcd-0001"), Some("seventy")), vec!["alter_id"]);
    assert_eq!(
        readback_identity_differences(None, None, &json!({})),
        vec!["guid", "alter_id"],
        "absent on both reads is not agreement"
    );
}

/// "No GSTIN in force" from the ledger read is two things. Only an entry in
/// force is evidence; a history that starts after the invoice date is not.
#[test]
fn a_registration_that_starts_after_the_invoice_date_is_not_read_as_unregistered() {
    use bridge_tally_protocol::gst_registration::{GstRegistrationEntry, GstRegistrationHistory};
    let entry = |from: &str, gstin: Option<&str>, kind: Option<&str>| GstRegistrationEntry {
        applicable_from: from.to_string(),
        gstin: gstin.map(str::to_string),
        registration_type: kind.map(str::to_string),
    };
    let history = |entries| GstRegistrationHistory::Entries { entries };
    let on = |flat: Option<&str>, history: &GstRegistrationHistory| {
        gstin_evidence(flat, history, "20260310")
    };
    // Registered from a later date: nothing is known of the invoice date.
    let later = history(vec![entry("20260401", Some(GSTIN_RJ), Some("Regular"))]);
    assert_eq!(on(None, &later), (GstinEvidence::NoEntryInForce, None));
    // An entry in force that names no GSTIN, with the type it names.
    let unregistered = history(vec![entry("20170701", None, Some("Unregistered/Consumer"))]);
    assert_eq!(
        on(None, &unregistered),
        (
            GstinEvidence::NoneInForce,
            Some("Unregistered/Consumer".to_string())
        )
    );
    // A GSTIN in force, with its entry's type.
    let regular = history(vec![entry("20170701", Some(GSTIN_RJ), Some("Regular"))]);
    assert_eq!(
        on(None, &regular),
        (
            GstinEvidence::InForce(GSTIN_RJ.to_string()),
            Some("Regular".to_string())
        )
    );
    // Only the flat field: a GSTIN, and no word on the kind of registration.
    assert_eq!(
        on(Some(GSTIN_RJ), &history(vec![])),
        (GstinEvidence::FlatFieldOnly, None)
    );
    assert_eq!(
        on(Some(GSTIN_RJ), &GstRegistrationHistory::NotObserved),
        (GstinEvidence::FlatFieldOnly, None)
    );
    // Nothing at all, and two sources that disagree.
    assert_eq!(
        on(None, &GstRegistrationHistory::NotObserved),
        (GstinEvidence::NotReported, None)
    );
    assert_eq!(on(Some(GSTIN_MH), &regular).0, GstinEvidence::Unsettled);
}

#[test]
fn what_the_build_records_follows_the_customers_evidence() {
    let registered = observe(
        &GstinEvidence::InForce(GSTIN_MH.to_string()),
        "ab-01".into(),
        true,
        RAJ.into(),
    );
    assert_eq!(
        registered,
        InvoiceObserved {
            voucher_type_guid: "ab-01".into(),
            party_gstin: Some(GSTIN_MH.to_string()),
            party_state: "Maharashtra".into(),
            party_registration_type: "Regular".into(),
            party_bill_wise: true,
            company_state: RAJ.into(),
        }
    );
    let unregistered = observe(
        &GstinEvidence::NoneInForce,
        "ab-01".into(),
        false,
        RAJ.into(),
    );
    assert_eq!(
        (
            unregistered.party_gstin,
            unregistered.party_state.as_str(),
            unregistered.party_registration_type.as_str(),
            unregistered.party_bill_wise
        ),
        (None, RAJ, "Unregistered/Consumer", false)
    );
}

#[test]
fn a_re_read_admits_only_the_observation_the_build_recorded() {
    let recorded = observed();
    let mut moved = observed();
    moved.party_bill_wise = !moved.party_bill_wise;
    assert!(observation_unchanged(Some(&recorded), Some(&observed())));
    assert!(!observation_unchanged(Some(&recorded), Some(&moved)));
    assert!(
        !observation_unchanged(None, Some(&recorded)),
        "nothing recorded is never unchanged"
    );
    assert!(!observation_unchanged(Some(&recorded), None));
    assert!(!observation_unchanged(None, None));
}

#[test]
fn each_leg_must_sit_on_its_own_side_and_each_role_be_filled_once() {
    let flipped = |index: usize| {
        let mut v = voucher();
        v.entries[index].side = match v.entries[index].side {
            EntrySide::Dr => EntrySide::Cr,
            EntrySide::Cr => EntrySide::Dr,
        };
        codes(classify_sales_invoice(&v, &good_facts(), RAJ))
    };
    assert_eq!(flipped(0), vec!["invoice_party_must_be_debit"]);
    assert_eq!(flipped(1), vec!["invoice_sales_must_be_credit"]);
    assert_eq!(flipped(2), vec!["invoice_tax_must_be_credit"]);
    // Two customers, two sales ledgers, no customer.
    let with = |name: &str, groups: &[&str]| {
        let mut facts = good_facts();
        facts.extend(facts_for(&[(name, groups, DutyHead::NotTax)]));
        let mut v = voucher();
        v.entries.push(entry(name, "1.00", EntrySide::Cr));
        codes(classify_sales_invoice(&v, &facts, RAJ))
    };
    assert!(with("Customer B", &["Sundry Debtors"]).contains(&"invoice_more_than_one_party"));
    // A second Sales Accounts ledger is a second sales leg: here it breaks the
    // slab arithmetic (10,001.00 taxed 600.00 a head), where a bill that splits
    // its taxable value between two ledgers is admitted (below).
    assert_eq!(
        with("Sales B", &["Sales Accounts"]),
        vec!["invoice_tax_matches_no_slab_rate"]
    );
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().reserved_groups = vec!["Sales Accounts".into()];
    let got = codes(classify_sales_invoice(&voucher(), &facts, RAJ));
    assert!(got.contains(&"invoice_party_missing"), "{got:?}");
}

#[test]
fn every_slab_rate_is_admitted_and_no_other() {
    // Taxable 10,000.00: each leg is half the rate.
    let at = |leg: &str, total: &str| {
        let mut v = voucher();
        v.entries[2].amount = leg.to_string();
        v.entries[3].amount = leg.to_string();
        v.entries[0].amount = total.to_string();
        classify_sales_invoice(&v, &good_facts(), RAJ).map_err(|refusals| refusals[0].code)
    };
    for (leg, total) in [
        ("250.00", "10500.00"),
        ("600.00", "11200.00"),
        ("900.00", "11800.00"),
        ("1400.00", "12800.00"),
        ("2000.00", "14000.00"),
    ] {
        assert!(at(leg, total).is_ok(), "{leg}");
    }
    // Five paise a leg off the slab is within tolerance; six is not.
    assert!(at("600.05", "11200.10").is_ok());
    assert!(at("599.95", "11199.90").is_ok());
    assert_eq!(
        at("600.06", "11200.12"),
        Err("invoice_tax_matches_no_slab_rate")
    );
    assert_eq!(
        at("599.94", "11199.88"),
        Err("invoice_tax_matches_no_slab_rate")
    );
    // Between and beyond the slabs: 3, 9, 24 and 50 percent.
    for (leg, total) in [
        ("150.00", "10300.00"),
        ("450.00", "10900.00"),
        ("1200.00", "12400.00"),
        ("2500.00", "15000.00"),
    ] {
        assert_eq!(
            at(leg, total),
            Err("invoice_tax_matches_no_slab_rate"),
            "{leg}"
        );
    }
}

/// A ledger name in a refusal goes out through the party marker, so it is
/// masked when the setting masks parties; a count or a state is not a party.
#[test]
fn refusal_details_follow_the_masking_setting() {
    use crate::agent::{redact_value, Redaction};
    let ledger = refuse_ledger("invoice_party_gstin_unsettled", "Customer A").to_json();
    assert_eq!(
        redact_value(ledger.clone(), Redaction::None)["detail"],
        "Customer A"
    );
    let masked = redact_value(ledger, Redaction::MaskParties);
    assert!(masked["detail"].is_string(), "{masked}");
    assert_ne!(masked["detail"], "Customer A", "{masked}");
    let value = refuse_value("invoice_place_of_supply_not_company_state", "Haryana").to_json();
    assert_eq!(
        redact_value(value, Redaction::MaskParties)["detail"],
        "Haryana"
    );
    // Every refusal that names a ledger carries it as a ledger: a classification
    // with every kind of ledger defect names no ledger outside the marker.
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::Unsettled;
    facts.get_mut("Customer A").unwrap().bill_wise = None;
    facts.get_mut("Sales").unwrap().reserved_groups = vec![];
    facts.get_mut("Output CGST").unwrap().duty_head = DutyHead::Igst;
    facts.remove("Output SGST");
    let refusals = classify_sales_invoice(&voucher(), &facts, RAJ).unwrap_err();
    assert!(refusals.len() >= 5, "{refusals:?}");
    let names = ["Customer A", "Sales", "Output CGST", "Output SGST"];
    for refusal in &refusals {
        if let RefusalDetail::Value(text) = &refusal.detail {
            assert!(
                !names.contains(&text.as_str()),
                "a ledger name outside the marker: {refusal:?}"
            );
        }
        let shown = redact_value(refusal.to_json(), Redaction::MaskParties);
        assert!(
            !names.iter().any(|name| shown["detail"] == *name),
            "a ledger name unmasked: {shown}"
        );
    }
}

#[test]
fn an_invoice_is_built_alone_and_never_amended() {
    use crate::agent::agent_import::refuse_mixed_shapes;
    let mut journal = voucher();
    journal.voucher_type = VoucherType::Journal;
    journal.invoice = None;
    assert_eq!(refuse_mixed_shapes(&[voucher()]), Ok(()));
    assert_eq!(
        refuse_mixed_shapes(&[voucher(), voucher()]),
        Err("invoice_one_per_batch".to_string())
    );
    assert_eq!(
        refuse_mixed_shapes(&[voucher(), journal.clone()]),
        Err("voucher_type_shapes_mixed".to_string())
    );
    assert_eq!(
        refuse_invoice_amendment(&[voucher()]),
        Err("invoice_amendment_not_supported".to_string())
    );
    assert_eq!(refuse_invoice_amendment(&[journal]), Ok(()));
}

#[test]
fn the_financial_year_runs_from_april_to_march() {
    let year = |date: &str| financial_year_window(date).unwrap();
    assert_eq!(
        year("20260331"),
        ("20250401".to_string(), "20260331".to_string())
    );
    assert_eq!(
        year("20260401"),
        ("20260401".to_string(), "20270331".to_string())
    );
    assert_eq!(
        year("20251231"),
        ("20250401".to_string(), "20260331".to_string())
    );
    assert_eq!(financial_year_window("2026"), None);
}

/// The type name an invoice is filed under, which the standard readback
/// compares, is the one the caller named, not the class.
#[test]
fn an_invoice_is_filed_under_the_type_the_caller_named() {
    let mut v = voucher();
    v.invoice.as_mut().unwrap().voucher_type_name = "Sales Manual".into();
    assert_eq!(v.filed_type_name(), "Sales Manual");
    v.voucher_type = VoucherType::Journal;
    assert_eq!(
        v.filed_type_name(),
        "Journal",
        "any other type is filed under its class name"
    );
    // And only an invoice has a New Ref party, whatever detail a voucher carries.
    let mut bill_wise = observed_voucher();
    assert_eq!(new_ref_party(&bill_wise), Some("Customer A"));
    bill_wise.voucher_type = VoucherType::Journal;
    assert_eq!(new_ref_party(&bill_wise), None);
}

/// A Sales line another build saved can sit in the journal of a build that
/// does not qualify Sales. Its verification does not send the invoice
/// read-back, and does not call the voucher verified.
#[test]
fn a_saved_invoice_of_an_unqualified_type_is_not_read_back_and_not_called_verified() {
    use crate::agent::agent_import::verification::invoice_readback_due;
    use crate::agent::agent_import::LIVE_QUALIFIED_VOUCHER_TYPES;
    let saved = [observed_voucher()];
    let matched = || {
        json!({
            "vouchers":[{"bridge_txn_id":"t1","status":"posted_verified","alter_id":77,"guid":"g-1"}],
            "counts":{"posted_verified":1,"posted_divergent":0}
        })
    };
    // This build: Sales is not qualified.
    let mut result = matched();
    assert!(invoice_readback_due(&mut result, &saved, LIVE_QUALIFIED_VOUCHER_TYPES).is_none());
    assert_eq!(result["vouchers"][0]["status"], "posted_divergent");
    assert_eq!(
        result["vouchers"][0]["diffs"][0]["invoice_fields"],
        json!(["invoice_type_unqualified"])
    );
    assert_eq!(
        result["counts"],
        json!({"posted_verified":0,"posted_divergent":1})
    );
    // A build that qualifies the type: the read-back is due, and nothing is
    // marked before it runs.
    let mut result = matched();
    let (voucher, row) =
        invoice_readback_due(&mut result, &saved, &[VoucherType::Sales]).expect("due");
    assert_eq!(
        (voucher.bridge_txn_id.as_str(), &row["alter_id"]),
        ("t1", &json!(77))
    );
    assert_eq!(result, matched());
    // A voucher the standard readback did not match has nothing due and is
    // left as it was, qualified or not.
    for qualified in [LIVE_QUALIFIED_VOUCHER_TYPES, &[VoucherType::Sales][..]] {
        let unmatched = json!({
            "vouchers":[{"bridge_txn_id":"t1","status":"not_found"}],
            "counts":{"posted_verified":0,"posted_divergent":0}
        });
        let mut result = unmatched.clone();
        assert!(invoice_readback_due(&mut result, &saved, qualified).is_none());
        assert_eq!(result, unmatched);
    }
}

/// Several sales legs on one tax pair: the taxable value is their sum, the
/// slab and the party total are checked on the sum, every leg is a credit to
/// a Sales Accounts ledger, and the window's leg cap is held before any read.
#[test]
fn several_sales_legs_share_one_pair_of_tax_heads() {
    let facts = || {
        let mut facts = good_facts();
        facts.extend(facts_for(&[
            ("Sales B", &["Sales Accounts"], DutyHead::NotTax),
            ("Sales C", &["Sales Accounts"], DutyHead::NotTax),
        ]));
        facts
    };
    // 10,000.00 as 6,000.00 + 4,000.00: the same tax, the same party total.
    let split = |amounts: &[&str]| {
        let mut v = voucher();
        v.entries.truncate(1);
        for (name, amount) in ["Sales", "Sales B", "Sales C"].iter().zip(amounts) {
            v.entries.push(entry(name, amount, EntrySide::Cr));
        }
        v.entries
            .push(entry("Output CGST", "600.00", EntrySide::Cr));
        v.entries
            .push(entry("Output SGST", "600.00", EntrySide::Cr));
        v
    };
    let two = split(&["6000.00", "4000.00"]);
    let roles = classify_sales_invoice(&two, &facts(), RAJ).expect("two sales legs");
    assert_eq!(roles.sales, vec![1, 2]);
    assert!(validate_invoice_voucher(&two).is_ok());
    let three = split(&["5000.00", "3000.00", "2000.00"]);
    assert_eq!(
        classify_sales_invoice(&three, &facts(), RAJ)
            .expect("three sales legs")
            .sales,
        vec![1, 2, 3]
    );
    assert!(validate_invoice_voucher(&three).is_ok());
    // The sum is what the slab and the party total are checked on.
    let mut short = split(&["6000.00", "3999.00"]);
    assert_eq!(
        codes(classify_sales_invoice(&short, &facts(), RAJ)),
        vec!["invoice_tax_matches_no_slab_rate"]
    );
    short.entries[2].amount = "3999.97".into();
    short.entries[0].amount = "11199.97".into();
    assert!(classify_sales_invoice(&short, &facts(), RAJ).is_ok());
    short.entries[0].amount = "11200.00".into();
    assert_eq!(
        codes(classify_sales_invoice(&short, &facts(), RAJ)),
        vec!["invoice_party_amount_does_not_close"]
    );
    // A sales leg that is a debit is still refused (a discount is netted by the
    // caller, never sent), and a ledger that is no Sales Accounts ledger is no sales leg.
    let mut discount = split(&["6000.00", "4000.00"]);
    discount.entries[2].side = EntrySide::Dr;
    assert!(codes(classify_sales_invoice(&discount, &facts(), RAJ))
        .contains(&"invoice_sales_must_be_credit"));
    // No Sales Accounts ledger at all is refused under its own code.
    let mut none = voucher();
    none.entries.remove(1);
    assert!(codes(classify_sales_invoice(&none, &good_facts(), RAJ))
        .contains(&"invoice_needs_a_sales_ledger"));
    // The window's cap on legs: a fourth sales leg, or a round off beside three,
    // is refused before any read.
    let mut four = split(&["4000.00", "3000.00", "2000.00"]);
    four.entries
        .push(entry("Sales D", "1000.00", EntrySide::Cr));
    assert_eq!(
        validate_invoice_voucher(&four),
        Err("invoice_too_many_entries".to_string())
    );
    let mut with_round_off = three.clone();
    with_round_off.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    with_round_off
        .entries
        .push(entry("Round Off", "0.10", EntrySide::Cr));
    assert_eq!(
        validate_invoice_voucher(&with_round_off),
        Err("invoice_too_many_entries".to_string())
    );
    // Two sales legs and a round off are within the cap.
    let mut two_and_round = two.clone();
    two_and_round.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    two_and_round
        .entries
        .push(entry("Round Off", "0.10", EntrySide::Cr));
    assert!(validate_invoice_voucher(&two_and_round).is_ok());
}

/// "Not in use" with no known invoice to read beside it: an empty book, or
/// the company's first invoice sent; once one was sent and none stands
/// verified, refused.
#[test]
fn without_a_control_only_an_empty_book_or_a_first_post_is_believed() {
    assert_eq!(
        absence_without_control(false, Some(0)).map_err(|r| r.code),
        Ok(NumberAbsence::EmptyBook)
    );
    assert_eq!(
        absence_without_control(true, Some(0)).map_err(|r| r.code),
        Ok(NumberAbsence::EmptyBook)
    );
    assert_eq!(
        absence_without_control(false, Some(40)).map_err(|r| r.code),
        Ok(NumberAbsence::FirstPost)
    );
    assert_eq!(
        absence_without_control(true, Some(40)).map_err(|refusal| refusal.code),
        Err("invoice_number_control_unavailable")
    );
}
