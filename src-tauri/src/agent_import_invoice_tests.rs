use super::*;
use crate::agent::agent_import::tests::{pilot_lab_guid, sentinel_uuid};
use crate::agent::agent_import::VoucherType;
use serde_json::json;

const RAJ: &str = "Rajasthan";
/// The invoice date of the fixtures here, as the build normalizes it.
const AS_OF: &str = "20260310";
/// The date of the lab's August vouchers.
const AS_OF_AUG: &str = "20260802";
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

/// An 18 percent invoice: 10,000.00 taxable, 900.00 and 900.00 tax.
fn voucher() -> ImportVoucher {
    ImportVoucher {
        bridge_txn_id: "t1".to_string(),
        date: bridge_tally_core::TallyDate::parse("20260310").expect("a YYYYMMDD date"),
        voucher_type: VoucherType::Sales,
        narration: Some("Invoice 278".to_string()),
        reference: None,
        voucher_number: Some("278".to_string()),
        invoice: Some(detail()),
        entries: vec![
            entry("Customer A", "11800.00", EntrySide::Dr),
            entry("Sales", "10000.00", EntrySide::Cr),
            entry("Output CGST", "900.00", EntrySide::Cr),
            entry("Output SGST", "900.00", EntrySide::Cr),
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
                    group_names: groups.iter().map(|g| g.to_string()).collect(),
                    duty_head: *head,
                    gstin: GstinEvidence::NotReported,
                    registration_type: None,
                    bill_wise: None,
                    rates: None,
                },
            )
        })
        .collect()
}

/// A sales ledger's rate rows as the W7 listing returned them: one dated,
/// ledger-specified, taxable row, one all-states block, the three heads with
/// their rates and no cess.
fn sales_rates(cgst: &str, igst: &str) -> wire::LedgerRateRow {
    let head = |name: &str, rate: Option<&str>, valuation: &str| wire::HeadRate {
        head: name.to_string(),
        valuation: Some(valuation.to_string()),
        rate: rate.map(str::to_string),
    };
    wire::LedgerRateRow {
        gst_rows: vec![wire::GstRow {
            applicable_from: Some("20250401".to_string()),
            taxability: Some("Taxable".to_string()),
            source: Some("Specify Details Here".to_string()),
            unmeasured: false,
            empty: false,
            states: vec![wire::StateRates {
                state: Some("Any".to_string()),
                heads: vec![
                    head("CGST", Some(cgst), "Based on Value"),
                    head("SGST/UTGST", Some(cgst), "Based on Value"),
                    head("IGST", Some(igst), "Based on Value"),
                    head("Cess", None, "Not Applicable"),
                    head("State Cess", None, "Based on Value"),
                ],
            }],
        }],
        rate_of_tax_calculation: Some("0".to_string()),
        rounding_method: None,
        rounding_limit: Some("0".to_string()),
        hsn: Ok(None),
        supply: wire::SupplyType::NotJudged,
    }
}

/// A tax ledger as the on-screen ledgers of the lab read: its own rate, no rounding.
fn tax_rates(half: &str) -> wire::LedgerRateRow {
    wire::LedgerRateRow {
        gst_rows: Vec::new(),
        rate_of_tax_calculation: Some(half.to_string()),
        rounding_method: Some("Not Applicable".to_string()),
        rounding_limit: Some("0".to_string()),
        hsn: Ok(None),
        supply: wire::SupplyType::NotJudged,
    }
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
    // The 18 percent of the fixture invoice: 9 and 9, IGST 18.
    facts.get_mut("Sales").unwrap().rates = Some(sales_rates("9", "18"));
    for tax in ["Output CGST", "Output SGST"] {
        facts.get_mut(tax).unwrap().rates = Some(tax_rates("9"));
    }
    // The round-off ledger's empty GST row, as the lab's read.
    facts.get_mut("Round Off").unwrap().rates =
        lab_rates(&["BRIDGE Round Off"]).remove("BRIDGE Round Off");
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
    for bad in ["INV 0042", "INV.0042", "INV_0042", &"1".repeat(17)] {
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
    let roles = classify_sales_invoice(&voucher(), &good_facts(), RAJ, AS_OF).unwrap();
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
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
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
        let got = codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF));
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
    assert!(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF).is_ok());
}

#[test]
fn arithmetic_must_close_and_each_head_must_be_the_per_line_tax() {
    let refused = |v: &ImportVoucher| codes(classify_sales_invoice(v, &good_facts(), RAJ, AS_OF));
    let mut v = voucher();
    v.entries[0].amount = "11801.00".to_string();
    assert_eq!(refused(&v), vec!["invoice_party_amount_does_not_close"]);
    // One head off by a rupee: that head is named, with the figure Tally expects.
    let mut v = voucher();
    v.entries[2].amount = "1000.00".to_string();
    let got = classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF).unwrap_err();
    assert_eq!(
        got,
        vec![refuse_value(
            "invoice_tax_head_not_expected",
            "CGST 1000.00 expected 900.00"
        )]
    );
    // 9 percent a head of 10.00 is 0.90: 0.01 is no tolerance.
    let mut v = voucher();
    v.entries[0].amount = "10.02".to_string();
    v.entries[1].amount = "10.00".to_string();
    v.entries[2].amount = "0.01".to_string();
    v.entries[3].amount = "0.01".to_string();
    assert_eq!(
        refused(&v),
        vec!["invoice_tax_head_not_expected"; 2],
        "both heads"
    );
    // A paisa off, either way, either head: refused (the lab flagged 0.01).
    for (cgst, state) in [
        ("900.01", "900.00"),
        ("900.00", "899.99"),
        ("900.01", "900.01"),
    ] {
        let mut v = voucher();
        v.entries[2].amount = cgst.to_string();
        v.entries[3].amount = state.to_string();
        assert!(
            codes(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF))
                .contains(&"invoice_tax_head_not_expected"),
            "{cgst} / {state}"
        );
    }
    // 9 percent a head of 10,001.00 is 900.09, a whole paisa: both heads are the figure.
    let mut v = voucher();
    v.entries[1].amount = "10001.00".to_string();
    v.entries[2].amount = "900.09".to_string();
    v.entries[3].amount = "900.09".to_string();
    v.entries[0].amount = "11801.18".to_string();
    assert!(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF).is_ok());
    // The figure is worked out on the ledger's rate, not on the tax typed: the
    // ledger says 18 percent, so heads of a 5 percent invoice are refused.
    let mut v = voucher();
    v.entries[2].amount = "250.00".to_string();
    v.entries[3].amount = "250.00".to_string();
    v.entries[0].amount = "10500.00".to_string();
    assert_eq!(refused(&v), vec!["invoice_tax_head_not_expected"; 2]);
}

#[test]
fn a_line_whose_tax_is_not_a_whole_paisa_at_another_rate_than_two_and_a_half_percent_is_refused() {
    // 0.50 at 9 percent a head is 0.045, a tie; 100.10 is 9.009, no tie: the
    // lab measured the rounding of inexact figures at 2.5 percent only.
    for (line, party) in [("0.50", "0.80"), ("100.10", "100.40")] {
        let mut v = voucher();
        v.entries[0].amount = party.to_string();
        v.entries[1].amount = line.to_string();
        v.entries[2].amount = "0.15".to_string();
        v.entries[3].amount = "0.15".to_string();
        assert_eq!(
            codes(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF)),
            vec!["invoice_tax_rounding_unmeasured"],
            "{line}"
        );
    }
    // A line whose tax is whole is admitted at that rate.
    let mut v = voucher();
    v.entries[0].amount = "118.00".to_string();
    v.entries[1].amount = "100.00".to_string();
    v.entries[2].amount = "9.00".to_string();
    v.entries[3].amount = "9.00".to_string();
    assert!(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF).is_ok());
}

#[test]
fn a_round_off_is_taken_by_side_and_bounded_under_a_rupee() {
    // Customer owes 11,800.40: a credit round off of 0.40.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11800.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
    assert_eq!(validate_invoice_voucher(&v), Ok(()));
    assert_eq!(
        classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF)
            .unwrap()
            .round_off,
        Some(4)
    );
    // Owes 11,799.60: a debit round off of 0.40.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11799.60".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Dr));
    assert!(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF).is_ok());
    // Wrong direction does not close.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11800.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Dr));
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF)),
        vec!["invoice_party_amount_does_not_close"]
    );
    // A round off of a rupee or more is refused.
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11801.00".to_string();
    v.entries.push(entry("Round Off", "1.00", EntrySide::Cr));
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF)),
        vec!["invoice_round_off_too_large"]
    );
    // The round off ledger must sit under Indirect Expenses or Incomes.
    let mut facts = good_facts();
    facts.get_mut("Round Off").unwrap().reserved_groups = vec!["Current Assets".into()];
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11800.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
    assert!(codes(classify_sales_invoice(&v, &facts, RAJ, AS_OF))
        .contains(&"invoice_round_off_ledger_group"));
}

/// The lab's rate listing (10 Oct 2026, the `pilot-lab` fixtures), read as the
/// build reads it, for the ledgers named.
fn lab_rates(named: &[&str]) -> BTreeMap<String, wire::LedgerRateRow> {
    wire::parse_ledger_rates(
        &lab_rates_answer(),
        "6b43e498-430c-4d5c-bfef-d32e2ab93c85",
        named,
    )
    .unwrap()
}

/// The lab's rate listing as text.
fn lab_rates_answer() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The round-off ledger's own GST row is read, and only the shape the lab's
/// round-off ledger carried is admitted: `BRIDGE Round Off`, whose
/// `GSTDETAILS.LIST` holds nothing, and it is the only row. A row that says anything
/// (Taxable, with a rate) might be counted by Tally in the tax, which was not
/// measured; a ledger the listing did not return is not known to carry none.
#[test]
fn a_round_off_ledger_is_admitted_only_with_the_empty_gst_row_the_lab_read() {
    let lab = lab_rates(&["BRIDGE Round Off", "BRIDGE Svc 998313 5%"]);
    let with = |rates: Option<wire::LedgerRateRow>| {
        let mut facts = good_facts();
        facts.get_mut("Round Off").unwrap().rates = rates;
        let mut v = voucher();
        v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
        v.entries[0].amount = "11800.40".to_string();
        v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
        classify_sales_invoice(&v, &facts, RAJ, AS_OF).map(|roles| roles.round_off)
    };
    let measured = lab["BRIDGE Round Off"].clone();
    assert_eq!(with(Some(measured.clone())), Ok(Some(4)));
    // No `GSTDETAILS.LIST` at all is a shape Tally was not seen to send: every
    // one of the lab listing's fourteen ledgers carried exactly one.
    let mut absent = measured.clone();
    absent.gst_rows.clear();
    assert_eq!(
        with(Some(absent)),
        Err(vec![refuse_ledger(
            "invoice_round_off_ledger_gst_row_absent",
            "Round Off"
        )])
    );
    // Two rows, even both empty, are not the one row that was read.
    let mut two = measured;
    two.gst_rows.push(two.gst_rows[0].clone());
    assert_eq!(
        with(Some(two)),
        Err(vec![refuse_ledger(
            "invoice_round_off_ledger_gst_row",
            "Round Off"
        )])
    );
    // A taxable row with a rate: the lab's own 5 percent services ledger's row.
    assert_eq!(
        with(Some(lab["BRIDGE Svc 998313 5%"].clone())),
        Err(vec![refuse_ledger(
            "invoice_round_off_ledger_gst_row",
            "Round Off"
        )])
    );
    assert_eq!(
        with(None),
        Err(vec![refuse_ledger(
            "invoice_round_off_ledger_gst_unknown",
            "Round Off"
        )])
    );
}

/// The round-off ledger as an invoice names it, with `rates` for its row of the
/// rate listing: the round-off line it is admitted on, or the refusals.
fn round_off_with(rates: wire::LedgerRateRow) -> Result<Option<usize>, Vec<InvoiceRefusal>> {
    let mut facts = good_facts();
    facts.get_mut("Round Off").unwrap().rates = Some(rates);
    let mut v = voucher();
    v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".into());
    v.entries[0].amount = "11800.40".to_string();
    v.entries.push(entry("Round Off", "0.40", EntrySide::Cr));
    classify_sales_invoice(&v, &facts, RAJ, AS_OF).map(|roles| roles.round_off)
}

/// A GST row that carries an attribute says something, even with no element
/// and no text: it is not the empty row the lab's round-off ledger read, in
/// either way an element can be written.
#[test]
fn a_round_off_ledgers_gst_row_with_an_attribute_is_not_the_empty_row_the_lab_read() {
    let answer = lab_rates_answer();
    let ledger = answer.find("<LEDGER NAME=\"BRIDGE Round Off\"").unwrap();
    let close = answer[ledger..].find("</LEDGER>").unwrap() + ledger;
    let row = "<GSTDETAILS.LIST>     </GSTDETAILS.LIST>";
    let at = answer[ledger..close].find(row).unwrap() + ledger;
    for attributed in [
        "<GSTDETAILS.LIST TYPE=\"String\">     </GSTDETAILS.LIST>",
        "<GSTDETAILS.LIST TYPE=\"String\"/>",
    ] {
        let mut edited = answer.clone();
        edited.replace_range(at..at + row.len(), attributed);
        let mut rates = wire::parse_ledger_rates(
            &edited,
            "6b43e498-430c-4d5c-bfef-d32e2ab93c85",
            &["BRIDGE Round Off"],
        )
        .unwrap();
        assert_eq!(
            round_off_with(rates.remove("BRIDGE Round Off").unwrap()),
            Err(vec![refuse_ledger(
                "invoice_round_off_ledger_gst_row",
                "Round Off"
            )]),
            "{attributed}"
        );
    }
}

/// The round-off ledger's own rate and rounding fields are read, and only what
/// the lab's round-off ledger carried is admitted: `RATEOFTAXCALCULATION` 0,
/// `ROUNDINGLIMIT` 0 and no `ROUNDINGMETHOD` element (the `pilot-lab` rate
/// listing). An absent field, an empty one and a zero are different answers;
/// no other value of any of the three was measured on a round-off ledger.
#[test]
fn a_round_off_ledger_is_admitted_only_with_the_rate_and_rounding_fields_the_lab_read() {
    let measured = lab_rates(&["BRIDGE Round Off"])
        .remove("BRIDGE Round Off")
        .unwrap();
    assert_eq!(
        (
            measured.rate_of_tax_calculation.as_deref(),
            measured.rounding_limit.as_deref(),
            measured.rounding_method.as_deref()
        ),
        (Some("0"), Some("0"), None)
    );
    assert_eq!(round_off_with(measured.clone()), Ok(Some(4)));
    type Change = fn(&mut wire::LedgerRateRow);
    let changes: [(&str, Change); 9] = [
        ("rate 2.50", |r| {
            r.rate_of_tax_calculation = Some("2.50".into())
        }),
        ("rate 0.00", |r| {
            r.rate_of_tax_calculation = Some("0.00".into())
        }),
        ("rate empty", |r| {
            r.rate_of_tax_calculation = Some(String::new())
        }),
        ("rate absent", |r| r.rate_of_tax_calculation = None),
        ("limit 1", |r| r.rounding_limit = Some("1".into())),
        ("limit empty", |r| r.rounding_limit = Some(String::new())),
        ("limit absent", |r| r.rounding_limit = None),
        ("method empty", |r| r.rounding_method = Some(String::new())),
        ("method Not Applicable", |r| {
            r.rounding_method = Some("Not Applicable".into())
        }),
    ];
    for (label, change) in changes {
        let mut row = measured.clone();
        change(&mut row);
        assert_eq!(
            round_off_with(row),
            Err(vec![refuse_ledger(
                "invoice_round_off_ledger_tax_fields",
                "Round Off"
            )]),
            "{label}"
        );
    }
}

fn observed() -> InvoiceObserved {
    InvoiceObserved {
        voucher_type_guid: "type-guid".to_string(),
        party_gstin: Some(GSTIN_RJ.to_string()),
        party_state: RAJ.to_string(),
        party_registration_type: "Regular".to_string(),
        party_bill_wise: true,
        company_state: RAJ.to_string(),
        sales_ledgers: vec!["Sales".to_string()],
        hsn_warnings: Vec::new(),
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
    let id = sentinel_uuid(0x1a);
    let xml =
        render_sales_invoice_xml(&v, id, "20260310", "<NARRATION>Invoice 278</NARRATION>").unwrap();
    let expected = "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"@ID@\" VCHTYPE=\"Sales\" ACTION=\"Create\" OBJVIEW=\"Invoice Voucher View\"><DATE>20260310</DATE><EFFECTIVEDATE>20260310</EFFECTIVEDATE><REFERENCEDATE>20260310</REFERENCEDATE><REFERENCE>278</REFERENCE><VOUCHERTYPENAME>Sales</VOUCHERTYPENAME><VOUCHERNUMBER>278</VOUCHERNUMBER><PARTYLEDGERNAME>Customer A</PARTYLEDGERNAME><PARTYNAME>Customer A</PARTYNAME><BASICBASEPARTYNAME>Customer A</BASICBASEPARTYNAME><PARTYMAILINGNAME>Customer A</PARTYMAILINGNAME><BASICBUYERNAME>Customer A</BASICBUYERNAME><CONSIGNEEMAILINGNAME>Customer A</CONSIGNEEMAILINGNAME><PARTYGSTIN>@GSTIN@</PARTYGSTIN><STATENAME>Rajasthan</STATENAME><CONSIGNEESTATENAME>Rajasthan</CONSIGNEESTATENAME><PLACEOFSUPPLY>Rajasthan</PLACEOFSUPPLY><GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE><VATDEALERTYPE>Regular</VATDEALERTYPE><COUNTRYOFRESIDENCE>India</COUNTRYOFRESIDENCE><CONSIGNEECOUNTRYNAME>India</CONSIGNEECOUNTRYNAME><PERSISTEDVIEW>Invoice Voucher View</PERSISTEDVIEW><VCHENTRYMODE>Accounting Invoice</VCHENTRYMODE><ISINVOICE>Yes</ISINVOICE><NARRATION>Invoice 278</NARRATION><LEDGERENTRIES.LIST><LEDGERNAME>Customer A</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-11800.00</AMOUNT><BILLALLOCATIONS.LIST><NAME>278</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>-11800.00</AMOUNT></BILLALLOCATIONS.LIST></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Sales</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>10000.00</AMOUNT></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Output CGST</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>900.00</AMOUNT></LEDGERENTRIES.LIST><LEDGERENTRIES.LIST><LEDGERNAME>Output SGST</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>900.00</AMOUNT></LEDGERENTRIES.LIST></VOUCHER></TALLYMESSAGE>";
    assert_eq!(
        xml,
        expected
            .replace("@GSTIN@", GSTIN_RJ)
            .replace("@ID@", &id.to_string())
    );
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
    let id = sentinel_uuid(0x1a);
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
        codes(classify_sales_invoice(
            &voucher(),
            &good_facts(),
            "Haryana",
            AS_OF
        )),
        vec!["invoice_place_of_supply_not_company_state"]
    );
    // An unregistered customer: an entry in force that names no GSTIN and
    // says the customer is unregistered.
    let mut facts = good_facts();
    let party = facts.get_mut("Customer A").unwrap();
    party.gstin = GstinEvidence::NoneInForce;
    party.registration_type = Some("Unregistered/Consumer".into());
    party.bill_wise = Some(false);
    assert!(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF).is_ok());
    // The same entry with no type says nothing: not read as unregistered.
    let type_not_reported = vec!["invoice_party_registration_type_not_reported"];
    facts.get_mut("Customer A").unwrap().registration_type = None;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
        type_not_reported
    );
    // No entry in force on the date (the history starts later): refused.
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::NoEntryInForce;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
        vec!["invoice_party_registration_not_in_force"]
    );
    // A GSTIN in the flat field only, or one in force whose entry names no
    // type: the kind of registration is not known, so not read as Regular.
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::FlatFieldOnly;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
        type_not_reported
    );
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::InForce(GSTIN_RJ.to_string());
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
        type_not_reported
    );
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::NoneInForce;
    // Registered on the ledger but no number in force: refused, not guessed unregistered.
    facts.get_mut("Customer A").unwrap().registration_type = Some("Regular".into());
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
        vec!["invoice_party_gstin_missing"]
    );
    // A GSTIN history Bridge cannot settle refuses.
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::Unsettled;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
        vec!["invoice_party_gstin_unsettled"]
    );
    // Silence is not evidence of "unregistered": a ledger Tally told nothing
    // about registration for is refused, never posted as B2C.
    facts.get_mut("Customer A").unwrap().gstin = GstinEvidence::NotReported;
    assert_eq!(
        codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF)),
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
    let id = sentinel_uuid(0x1a);
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
/// renderer, so what is written and what is compared cannot drift apart. Its
/// four GSTR-1 status elements are the lab's, copied as Tally wrote them for an
/// invoice it lists as included (`included_status_elements`), and so are the
/// five HSN/SAC elements each sales line carries (`resolved_hsn_elements`).
fn readback_xml(v: &ImportVoucher) -> String {
    let id = sentinel_uuid(0x1a);
    let mut rendered =
        render_sales_invoice_xml(v, id, "20260310", "<NARRATION>Invoice 278</NARRATION>").unwrap();
    let sales = v
        .invoice
        .as_ref()
        .and_then(|detail| detail.observed.as_ref());
    for ledger in sales.iter().flat_map(|observed| &observed.sales_ledgers) {
        let name = format!("<LEDGERNAME>{}</LEDGERNAME>", xml_escape(ledger));
        assert_eq!(rendered.matches(&name).count(), 1, "{ledger}");
        rendered = rendered.replacen(&name, &format!("{name}{}", resolved_hsn_elements()), 1);
    }
    let voucher = rendered
        .replace("<TALLYMESSAGE xmlns:UDF=\"TallyUDF\">", "")
        .replace("</TALLYMESSAGE>", "")
        .replace("LEDGERENTRIES.LIST", "ALLLEDGERENTRIES.LIST")
        .replace("<DATE>", &format!("<ISCANCELLED>No</ISCANCELLED><ISOPTIONAL>No</ISOPTIONAL><GUID>g-1</GUID><ALTERID> 77</ALTERID>{}<DATE>", included_status_elements()))
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
    assert_eq!(altered("<AMOUNT>900.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Output SGST", "<AMOUNT>901.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Output SGST"), vec!["legs"]);
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
    let no_allocation = good.replace("<BILLALLOCATIONS.LIST><NAME>278</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>-11800.00</AMOUNT></BILLALLOCATIONS.LIST>", "");
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
    let none = |difference: &str| (vec![difference.to_string()], None, None, Vec::new());
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
    let (differences_found, alter_id, guid, _) =
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
    crate::agent::agent_import::verification::mark_invoice_readback(&mut result, "t1", &[], &[]);
    assert_eq!(
        result["vouchers"][0]["status"], "posted_verified",
        "no difference leaves it"
    );
    crate::agent::agent_import::verification::mark_invoice_readback(
        &mut result,
        "t2",
        &["PARTYGSTIN".into()],
        &[],
    );
    assert_eq!(
        result["vouchers"][0]["status"], "posted_verified",
        "another voucher's difference is not this one's"
    );
    crate::agent::agent_import::verification::mark_invoice_readback(
        &mut result,
        "t1",
        &["PARTYGSTIN".into()],
        &[],
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
    for field in 0..8 {
        changes.push(Box::new(move |v| {
            let seen = v.invoice.as_mut().unwrap().observed.as_mut().unwrap();
            match field {
                0 => seen.voucher_type_guid = "other".into(),
                1 => seen.party_gstin = None,
                2 => seen.party_state = "Haryana".into(),
                3 => seen.party_registration_type = "Unregistered/Consumer".into(),
                4 => seen.party_bill_wise = false,
                5 => seen.sales_ledgers.clear(),
                6 => seen.sales_ledgers[0] = "Output CGST".into(),
                _ => seen.company_state = "Haryana".into(),
            }
        }));
    }
    for change in &changes {
        let mut changed = base.clone();
        change(&mut changed);
        assert_ne!(digest(&base), digest(&changed));
    }
    // Each part of an HSN/SAC warning the dialog speaks for is bound too.
    let warning = |code, ledger: &str, level: wire::HsnLevel, characters| wire::HsnWarning {
        code,
        ledger: ledger.into(),
        level,
        characters,
    };
    let listed = |warnings: Vec<wire::HsnWarning>| {
        let mut voucher = base.clone();
        let seen = voucher.invoice.as_mut().unwrap().observed.as_mut().unwrap();
        seen.hsn_warnings = warnings;
        digest(&voucher)
    };
    let warned = |code, level: wire::HsnLevel, characters| {
        listed(vec![warning(code, "Sales", level, characters)])
    };
    let not_six = wire::HsnWarningCode::NotSixDigits;
    let warned_digests = [
        digest(&base),
        warned(not_six, wire::HsnLevel::Ledger, 4),
        warned(wire::HsnWarningCode::SacForGoods, wire::HsnLevel::Ledger, 4),
        warned(not_six, wire::HsnLevel::Company, 4),
        warned(not_six, wire::HsnLevel::Group("Sales Sub".into()), 4),
        warned(not_six, wire::HsnLevel::Group("Sales Top".into()), 4),
        warned(not_six, wire::HsnLevel::Ledger, 8),
        listed(vec![warning(not_six, "Sales B", wire::HsnLevel::Ledger, 4)]),
        listed(vec![
            warning(not_six, "Sales", wire::HsnLevel::Ledger, 4),
            warning(not_six, "Sales B", wire::HsnLevel::Ledger, 4),
        ]),
    ];
    let distinct = warned_digests
        .iter()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(distinct.len(), warned_digests.len());
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
/// pilot lab's captured read-back is compared in the test after it. Names
/// and numbers are synthetic.
#[test]
fn a_hand_authored_export_of_a_registered_bill_wise_invoice_reads_back_clean() {
    let status = included_status_elements();
    let hsn = resolved_hsn_elements();
    let remote_id = sentinel_uuid(0x2b);
    let xml = format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO><COMPANY>0</COMPANY></CMPINFO></DESC><DATA><COLLECTION>\
<VOUCHER REMOTEID=\"{remote_id}\" VCHKEY=\"k\" VCHTYPE=\"Sales\" OBJVIEW=\"Invoice Voucher View\">\
<DATE TYPE=\"Date\">20260310</DATE><EFFECTIVEDATE TYPE=\"Date\">20260310</EFFECTIVEDATE><REFERENCEDATE TYPE=\"Date\">20260310</REFERENCEDATE>\
<GUID TYPE=\"String\">g-1</GUID><PARTYGSTIN TYPE=\"String\">{GSTIN_RJ}</PARTYGSTIN><STATENAME TYPE=\"String\">Rajasthan</STATENAME>\
<PLACEOFSUPPLY TYPE=\"String\">Rajasthan</PLACEOFSUPPLY><VOUCHERTYPENAME TYPE=\"String\">Sales</VOUCHERTYPENAME><PARTYLEDGERNAME TYPE=\"String\">Customer A</PARTYLEDGERNAME>\
<VOUCHERNUMBER TYPE=\"String\">278</VOUCHERNUMBER><REFERENCE TYPE=\"String\">278</REFERENCE><GSTREGISTRATIONTYPE TYPE=\"String\">Regular</GSTREGISTRATIONTYPE>\
<ISINVOICE>Yes</ISINVOICE><ISCANCELLED TYPE=\"Logical\">No</ISCANCELLED><ISOPTIONAL TYPE=\"Logical\">No</ISOPTIONAL><ALTERID TYPE=\"Number\"> 77</ALTERID>{status}\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Customer A</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">Yes</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">-11800.00</AMOUNT>\
<BILLALLOCATIONS.LIST><NAME TYPE=\"String\">278</NAME><BILLTYPE TYPE=\"String\">New Ref</BILLTYPE><AMOUNT TYPE=\"Amount\">-11800.00</AMOUNT></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Sales</LEDGERNAME>{hsn}<ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Output CGST</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">900.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">Output SGST</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">No</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">900.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>\
</VOUCHER></COLLECTION></DATA></BODY></ENVELOPE>"
    );
    let v = observed_voucher();
    assert_eq!(differences(&v, &xml), Vec::<String>::new());
    // An allocation whose amount did not come back is a difference, not an absence.
    let no_amount = xml.replace(
        "<BILLTYPE TYPE=\"String\">New Ref</BILLTYPE><AMOUNT TYPE=\"Amount\">-11800.00</AMOUNT>",
        "<BILLTYPE TYPE=\"String\">New Ref</BILLTYPE>",
    );
    assert_eq!(differences(&v, &no_amount), vec!["bill_allocation"]);
    // A stray allocation name on a leg that should carry none shows too.
    let stray = xml.replace("<AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST></BILLALLOCATIONS.LIST>", "<AMOUNT TYPE=\"Amount\">10000.00</AMOUNT><BILLALLOCATIONS.LIST><NAME TYPE=\"String\">X</NAME></BILLALLOCATIONS.LIST>");
    assert_ne!(stray, xml);
    assert_eq!(differences(&v, &stray), vec!["bill_allocation"]);
}

/// The pilot lab's captured read-back of the invoice its book took (the
/// `pilot-lab` set; its PROVENANCE table is beside the fixtures), decoded and
/// compared with `expected` as a post compares them.
fn pilot_differences(expected: &ImportVoucher, bytes: &[u8], date: &str) -> Vec<String> {
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

/// The invoice the pilot lab's book took (`BRIDGE Sales`, to its walk-in
/// customer, unregistered and not bill-wise), numbered `number`, as the build
/// saved it with what it observed.
fn pilot_posted(number: &str) -> ImportVoucher {
    serde_json::from_value(serde_json::json!({
        "bridge_txn_id":"t1", "date":"20260801", "voucher_type":"Sales", "voucher_number":number,
        "invoice":{"voucher_type_name":"BRIDGE Sales","place_of_supply":"Rajasthan","round_off_ledger":"BRIDGE Round Off",
            "observed":{
                "voucher_type_guid":format!("{}-000000ce", pilot_lab_guid()),
                "party_state":"Rajasthan", "party_registration_type":"Unregistered/Consumer",
                "party_bill_wise":false, "company_state":"Rajasthan",
                "sales_ledgers":["BRIDGE Svc 998313 5%","BRIDGE Svc 998314 5%"]
            }},
        "entries":[
            {"ledger":"BRIDGE Walk-in","amount":"1051.00","side":"Dr"},
            {"ledger":"BRIDGE Svc 998313 5%","amount":"600.40","side":"Cr"},
            {"ledger":"BRIDGE Svc 998314 5%","amount":"400.40","side":"Cr"},
            {"ledger":"BRIDGE CGST 2.5%","amount":"25.02","side":"Cr"},
            {"ledger":"BRIDGE SGST 2.5%","amount":"25.02","side":"Cr"},
            {"ledger":"BRIDGE Round Off","amount":"0.16","side":"Cr"}
        ]
    }))
    .unwrap()
}

/// The invoice the pilot lab's book took (9 Oct 2026), written as the build
/// saved it, against Tally's answer to the read-back request the code sends
/// now (the GSTR-1 status fields and each line's HSN/SAC, answered on 10 Oct
/// 2026): no difference. The same answer against the invoice under another
/// number differs in the number and the reference the number gives. The 9 Oct
/// answer, whose request asked for no status field, reads as the build wrote it
/// in everything else, and says so once: the GSTR-1 status not reported.
#[test]
fn the_pilot_labs_posted_invoice_reads_back_as_planned_on_its_captured_answer() {
    assert_eq!(
        pilot_differences(&pilot_posted("BP/26-27/0010"), STATUS_INCLUDED, "20260801"),
        Vec::<String>::new()
    );
    assert_eq!(
        pilot_differences(&pilot_posted("BP/26-27/0040"), STATUS_INCLUDED, "20260801"),
        vec!["VOUCHERNUMBER", "REFERENCE"]
    );
    assert_eq!(
        pilot_differences(&pilot_posted("BP/26-27/0010"), STATUS_NOT_ASKED, "20260801"),
        vec!["gst_status_not_reported"]
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
        codes(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF)),
        vec!["invoice_amount_invalid"]
    );
    // The sum fits and the tax's multiplication does not: refused, never wrapped.
    let mut v = voucher();
    v.entries[1].amount = "1".to_string() + &"0".repeat(35) + ".00";
    v.entries[2].amount = "1.00".to_string();
    v.entries[3].amount = "1.00".to_string();
    v.entries[0].amount = "1".to_string() + &"0".repeat(34) + "2.00";
    assert_eq!(
        codes(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF)),
        vec!["invoice_amount_invalid"]
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
        vec!["Sales".into()],
        Vec::new(),
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
            sales_ledgers: vec!["Sales".into()],
            hsn_warnings: Vec::new(),
        }
    );
    let unregistered = observe(
        &GstinEvidence::NoneInForce,
        "ab-01".into(),
        false,
        RAJ.into(),
        vec!["Sales".into()],
        Vec::new(),
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
    assert_eq!(first_difference(Some(&recorded), Some(&observed())), None);
    assert_eq!(
        first_difference(Some(&recorded), Some(&moved)),
        Some("party_bill_wise")
    );
    for (nothing_recorded, nothing_now) in [(true, false), (false, true), (true, true)] {
        assert_eq!(
            first_difference(
                (!nothing_recorded).then_some(&recorded),
                (!nothing_now).then_some(&recorded)
            ),
            Some(NOT_RECORDED),
            "nothing recorded or nothing observed is never unchanged"
        );
    }
}

/// Each field of the observation has its own name, and when several moved the
/// first in the observation's order is the one named (#1337).
#[test]
fn the_first_field_that_moved_is_the_one_named() {
    type Move = fn(&mut InvoiceObserved);
    let moves: [(&str, Move); 8] = [
        ("voucher_type", |o| o.voucher_type_guid.push('x')),
        ("party_gstin", |o| o.party_gstin = Some("X".into())),
        ("party_state", |o| o.party_state.push('x')),
        ("party_registration", |o| {
            o.party_registration_type.push('x')
        }),
        ("party_bill_wise", |o| {
            o.party_bill_wise = !o.party_bill_wise
        }),
        ("company_state", |o| o.company_state.push('x')),
        ("sales_ledgers", |o| o.sales_ledgers.push("Other".into())),
        ("hsn_warnings", |o| {
            o.hsn_warnings.push(wire::HsnWarning {
                code: wire::HsnWarningCode::NotSixDigits,
                ledger: "Sales".into(),
                level: wire::HsnLevel::Ledger,
                characters: 4,
            })
        }),
    ];
    let recorded = observed();
    for (name, moved) in &moves {
        let mut now = observed();
        moved(&mut now);
        assert_eq!(
            first_difference(Some(&recorded), Some(&now)),
            Some(*name),
            "{name}"
        );
    }
    let mut everything = observed();
    for (_, moved) in &moves {
        moved(&mut everything);
    }
    assert_eq!(
        first_difference(Some(&recorded), Some(&everything)),
        Some("voucher_type")
    );
}

#[test]
fn each_leg_must_sit_on_its_own_side_and_each_role_be_filled_once() {
    let flipped = |index: usize| {
        let mut v = voucher();
        v.entries[index].side = match v.entries[index].side {
            EntrySide::Dr => EntrySide::Cr,
            EntrySide::Cr => EntrySide::Dr,
        };
        codes(classify_sales_invoice(&v, &good_facts(), RAJ, AS_OF))
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
        codes(classify_sales_invoice(&v, &facts, RAJ, AS_OF))
    };
    assert!(with("Customer B", &["Sundry Debtors"]).contains(&"invoice_more_than_one_party"));
    // A second Sales Accounts ledger is a second sales leg: the listing gave
    // this one no rate, so it is refused on that (a bill that splits its
    // taxable value between two ledgers with rates is admitted, below).
    assert_eq!(
        with("Sales B", &["Sales Accounts"]),
        vec!["invoice_sales_ledger_rate_unknown"]
    );
    let mut facts = good_facts();
    facts.get_mut("Customer A").unwrap().reserved_groups = vec!["Sales Accounts".into()];
    let got = codes(classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF));
    assert!(got.contains(&"invoice_party_missing"), "{got:?}");
}

#[test]
fn each_rate_is_admitted_on_the_ledgers_own_rate_and_no_other() {
    // Taxable 10,000.00: the sales ledger says r percent, each head is half.
    let at = |ledger_half: &str, ledger_igst: &str, leg: &str, total: &str| {
        let mut facts = good_facts();
        facts.get_mut("Sales").unwrap().rates = Some(sales_rates(ledger_half, ledger_igst));
        for tax in ["Output CGST", "Output SGST"] {
            facts.get_mut(tax).unwrap().rates = Some(tax_rates(ledger_half));
        }
        let mut v = voucher();
        v.entries[2].amount = leg.to_string();
        v.entries[3].amount = leg.to_string();
        v.entries[0].amount = total.to_string();
        classify_sales_invoice(&v, &facts, RAJ, AS_OF).map_err(|refusals| refusals[0].code)
    };
    for (half, igst, leg, total) in [
        ("2.5", "5", "250.00", "10500.00"),
        ("9", "18", "900.00", "11800.00"),
    ] {
        assert!(at(half, igst, leg, total).is_ok(), "{half}");
    }
    // The slabs the lab did not measure are refused, the invoice and the
    // ledger agreeing.
    for (half, igst, leg, total) in [
        ("6", "12", "600.00", "11200.00"),
        ("14", "28", "1400.00", "12800.00"),
        ("20", "40", "2000.00", "14000.00"),
    ] {
        assert_eq!(
            at(half, igst, leg, total),
            Err("invoice_sales_ledger_rate_not_measured"),
            "{half}"
        );
    }
    // The ledger's rate decides, not the tax typed: heads that fit another slab
    // (250.00 is 5 percent) are refused where the ledger says 18 percent, and
    // the reverse.
    assert_eq!(
        at("9", "18", "250.00", "10500.00"),
        Err("invoice_tax_head_not_expected")
    );
    assert_eq!(
        at("2.5", "5", "900.00", "11800.00"),
        Err("invoice_tax_head_not_expected")
    );
    // A paisa off the figure is refused at every rate.
    assert_eq!(
        at("9", "18", "900.01", "11800.02"),
        Err("invoice_tax_head_not_expected")
    );
}

/// The sixteen vouchers of the lab's first two lists (10 Oct 2026): Tally's
/// GSTR-1 included exactly the ones whose heads were the per-line tax. Each
/// is checked here against the code's rule, with the sales lines, the heads
/// and the round off the export read. Single-pair invoices only: 0009 carries
/// two rate pairs and 0015 repeats a ledger, and have their own refusals.
#[test]
fn the_rule_admits_what_tallys_gstr_1_included_and_refuses_what_it_flagged() {
    // (lines, CGST, state tax, round off as signed credit, Tally included it)
    /// A voucher number, its sales lines, its CGST and state tax, its round off, and
    /// whether Tally's GSTR-1 included it.
    type LabVoucher = (
        &'static str,
        &'static [&'static str],
        &'static str,
        &'static str,
        &'static str,
        bool,
    );
    let table: &[LabVoucher] = &[
        ("0001", &["1000.00"], "25.00", "25.00", "0.00", true),
        ("0002", &["1001.00"], "25.03", "25.02", "0.00", false),
        ("0003", &["1001.00"], "25.02", "25.03", "0.00", false),
        ("0004", &["1001.00"], "25.03", "25.03", "-0.01", true),
        ("0005", &["1001.00"], "25.02", "25.02", "0.01", false),
        ("0006", &["1000.00"], "25.01", "25.01", "0.00", false),
        (
            "0007",
            &["600.00", "400.00"],
            "25.00",
            "25.00",
            "0.00",
            true,
        ),
        (
            "0008",
            &["601.00", "400.00"],
            "25.03",
            "25.02",
            "0.00",
            false,
        ),
        (
            "0010",
            &["600.40", "400.40"],
            "25.02",
            "25.02",
            "0.16",
            true,
        ),
        ("0011", &["1001.00"], "25.03", "25.03", "0.00", true),
        ("0012", &["100.10", "100.10"], "5.00", "5.00", "0.00", true),
        ("0013", &["100.20", "100.20"], "5.02", "5.02", "0.00", true),
        ("0014", &["100.60", "100.60"], "5.04", "5.04", "0.00", true),
        ("0016", &["100.20", "100.20"], "5.01", "5.01", "0.00", false),
    ];
    let mut facts = good_facts();
    facts.extend(facts_for(&[(
        "Sales B",
        &["Sales Accounts"],
        DutyHead::NotTax,
    )]));
    for name in ["Sales", "Sales B"] {
        facts.get_mut(name).unwrap().rates = Some(sales_rates("2.5", "5"));
    }
    for tax in ["Output CGST", "Output SGST"] {
        facts.get_mut(tax).unwrap().rates = Some(tax_rates("2.5"));
    }
    let cents = |text: &str| -> i128 {
        let negative = text.starts_with('-');
        let (whole, fraction) = text.trim_start_matches('-').split_once('.').unwrap();
        let value = whole.parse::<i128>().unwrap() * 100 + fraction.parse::<i128>().unwrap();
        if negative {
            -value
        } else {
            value
        }
    };
    let text = |paise: i128| format!("{}.{:02}", paise / 100, paise % 100);
    for (number, lines, cgst, state, round, included) in table {
        let taxable: i128 = lines.iter().map(|line| cents(line)).sum();
        let round = cents(round);
        let party = taxable + cents(cgst) + cents(state) + round;
        let mut v = voucher();
        v.entries.clear();
        v.entries
            .push(entry("Customer A", &text(party), EntrySide::Dr));
        for (name, line) in ["Sales", "Sales B"].iter().zip(lines.iter()) {
            v.entries.push(entry(name, line, EntrySide::Cr));
        }
        v.entries.push(entry("Output CGST", cgst, EntrySide::Cr));
        v.entries.push(entry("Output SGST", state, EntrySide::Cr));
        if round != 0 {
            v.invoice.as_mut().unwrap().round_off_ledger = Some("Round Off".to_string());
            let side = if round > 0 {
                EntrySide::Cr
            } else {
                EntrySide::Dr
            };
            v.entries.push(entry("Round Off", &text(round.abs()), side));
        }
        let outcome = classify_sales_invoice(&v, &facts, RAJ, AS_OF_AUG);
        match (included, outcome) {
            (true, Ok(_)) => {}
            (false, Err(refusals)) => assert!(
                refusals
                    .iter()
                    .all(|refusal| refusal.code == "invoice_tax_head_not_expected"),
                "{number}: {refusals:?}"
            ),
            (included, outcome) => {
                panic!("{number}: Tally included {included}, the rule says {outcome:?}")
            }
        }
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
    let refusals = classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF).unwrap_err();
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
    let year = |date: &str| {
        let (from, to) = financial_year_window(date).unwrap();
        (from.as_str().to_string(), to.as_str().to_string())
    };
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
        for name in ["Sales B", "Sales C"] {
            facts.get_mut(name).unwrap().rates = Some(sales_rates("9", "18"));
        }
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
            .push(entry("Output CGST", "900.00", EntrySide::Cr));
        v.entries
            .push(entry("Output SGST", "900.00", EntrySide::Cr));
        v
    };
    let two = split(&["6000.00", "4000.00"]);
    let roles = classify_sales_invoice(&two, &facts(), RAJ, AS_OF).expect("two sales legs");
    assert_eq!(roles.sales, vec![1, 2]);
    assert!(validate_invoice_voucher(&two).is_ok());
    // Three lines fit the window but not the measured tax: two lines is the
    // most that was measured, so a third is refused until a bill of three is.
    let three = split(&["5000.00", "3000.00", "2000.00"]);
    assert_eq!(
        codes(classify_sales_invoice(&three, &facts(), RAJ, AS_OF)),
        vec!["invoice_too_many_sales_lines"]
    );
    assert!(validate_invoice_voucher(&three).is_ok());
    // The tax is worked out line by line, and the party total is checked on the sum.
    let mut short = split(&["6000.00", "3999.00"]);
    assert_eq!(
        codes(classify_sales_invoice(&short, &facts(), RAJ, AS_OF)),
        vec!["invoice_tax_head_not_expected"; 2]
    );
    // 9 percent of 3,999.00 is 359.91 and of 6,000.00 is 540.00: 899.91 a head.
    short.entries[3].amount = "899.91".into();
    short.entries[4].amount = "899.91".into();
    short.entries[0].amount = "11798.82".into();
    assert!(classify_sales_invoice(&short, &facts(), RAJ, AS_OF).is_ok());
    short.entries[0].amount = "11800.00".into();
    assert_eq!(
        codes(classify_sales_invoice(&short, &facts(), RAJ, AS_OF)),
        vec!["invoice_party_amount_does_not_close"]
    );
    // A sales leg that is a debit is still refused (a discount is netted by the
    // caller, never sent), and a ledger that is no Sales Accounts ledger is no sales leg.
    let mut discount = split(&["6000.00", "4000.00"]);
    discount.entries[2].side = EntrySide::Dr;
    assert!(
        codes(classify_sales_invoice(&discount, &facts(), RAJ, AS_OF))
            .contains(&"invoice_sales_must_be_credit")
    );
    // No Sales Accounts ledger at all is refused under its own code.
    let mut none = voucher();
    none.entries.remove(1);
    assert!(
        codes(classify_sales_invoice(&none, &good_facts(), RAJ, AS_OF))
            .contains(&"invoice_needs_a_sales_ledger")
    );
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
/// no invoice of the company standing as sent; while one stands sent and none
/// stands verified, refused.
#[test]
fn without_a_control_only_an_empty_book_or_no_standing_invoice_is_believed() {
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
        Ok(NumberAbsence::NoStandingInvoice)
    );
    assert_eq!(
        absence_without_control(true, Some(40)).map_err(|refusal| refusal.code),
        Err("invoice_number_control_unavailable")
    );
}

/// Every doubt about a sales ledger's rate is its own refusal, never a guess.
#[test]
fn each_doubt_about_a_sales_ledgers_rate_has_its_own_code() {
    let with = |change: &dyn Fn(&mut wire::LedgerRateRow)| {
        let mut row = sales_rates("2.5", "5");
        change(&mut row);
        sales_rate_milli(&row, AS_OF)
    };
    assert_eq!(with(&|_| {}), Ok(2_500));
    assert_eq!(
        with(&|r| r.gst_rows.clear()),
        Err("invoice_sales_ledger_rate_unknown")
    );
    assert_eq!(
        with(&|r| {
            let again = r.gst_rows[0].clone();
            r.gst_rows.push(again)
        }),
        Err("invoice_sales_ledger_rate_history_unmeasured")
    );
    // A row that starts after the invoice date says nothing of that day.
    assert_eq!(
        with(&|r| r.gst_rows[0].applicable_from = Some("20260401".into())),
        Err("invoice_sales_ledger_rate_not_in_force")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].applicable_from = None),
        Err("invoice_sales_ledger_rate_not_in_force")
    );
    // The date it starts is a calendar day: text that sorts before one is no date.
    assert_eq!(
        with(&|r| r.gst_rows[0].applicable_from = Some("!!!!!!!!".into())),
        Err("invoice_sales_ledger_rate_not_in_force")
    );
    // Eight digits that name no calendar day are no date either (#2009).
    assert_eq!(
        with(&|r| r.gst_rows[0].applicable_from = Some("20251340".into())),
        Err("invoice_sales_ledger_rate_not_in_force")
    );
    // A row that starts on the invoice date is in force on it.
    assert_eq!(
        with(&|r| r.gst_rows[0].applicable_from = Some(AS_OF.into())),
        Ok(2_500)
    );
    // A row with an element or a slab list the lab's rows did not carry.
    assert_eq!(
        with(&|r| r.gst_rows[0].unmeasured = true),
        Err("invoice_sales_ledger_rate_shape_unmeasured")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].taxability = Some("Exempt".into())),
        Err("invoice_sales_ledger_not_taxable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].taxability = None),
        Err("invoice_sales_ledger_not_taxable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].source = Some("As per Company/Group".into())),
        Err("invoice_sales_ledger_rate_not_ledger_specified")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].state = Some("Rajasthan".into())),
        Err("invoice_sales_ledger_rate_state_wise_unmeasured")
    );
    assert_eq!(
        with(&|r| {
            let again = r.gst_rows[0].states[0].clone();
            r.gst_rows[0].states.push(again)
        }),
        Err("invoice_sales_ledger_rate_state_wise_unmeasured")
    );
    // A state head that is not the other, or an IGST that is not their sum.
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[1].rate = Some("3".into())),
        Err("invoice_sales_ledger_rate_heads_unequal")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[2].rate = Some("6".into())),
        Err("invoice_sales_ledger_rate_igst_not_twice_state")
    );
    // A head with no rate, a rate that is not a number, a repeated head.
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[0].rate = None),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[0].rate = Some("2.5x".into())),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[0].rate = Some("2.5555".into())),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[0].rate = Some("0".into())),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[0].valuation = Some("Not Applicable".into())),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    assert_eq!(
        with(&|r| {
            let again = r.gst_rows[0].states[0].heads[0].clone();
            r.gst_rows[0].states[0].heads.push(again)
        }),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[1].head = "CGST".into()),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[0].head = "UTGST".into()),
        Err("invoice_sales_ledger_rate_heads_unreadable")
    );
    // Cess: a rate on it, or a valuation other than Not Applicable, is not supported.
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[3].rate = Some("1".into())),
        Err("invoice_cess_rate_not_supported")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[3].valuation = Some("Based on Value".into())),
        Err("invoice_cess_rate_not_supported")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[4].rate = Some("1".into())),
        Err("invoice_cess_rate_not_supported")
    );
    // A rate GST has no slab for: the ledger and the invoice may agree, and it is refused.
    for (state, igst) in [
        ("1.5", "3"),
        ("12.5", "25"),
        ("0.125", "0.25"),
        ("250", "500"),
    ] {
        let mut row = sales_rates(state, igst);
        row.gst_rows[0].states[0].heads[0].rate = Some(state.into());
        assert_eq!(
            sales_rate_milli(&row, AS_OF),
            Err("invoice_sales_ledger_rate_not_a_slab"),
            "{igst}"
        );
    }
    // State Cess is admitted only as the lab's rows read it: on value, with no rate.
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[4].valuation = Some("Based on Quantity".into())),
        Err("invoice_cess_rate_not_supported")
    );
    assert_eq!(
        with(&|r| r.gst_rows[0].states[0].heads[4].valuation = None),
        Err("invoice_cess_rate_not_supported")
    );
    // Tally writes a rate with a leading space and a decimal tail: " 2.50" reads as 2.5.
    let mut padded = sales_rates("2.50", "5");
    padded.gst_rows[0].states[0].heads[0].rate = Some("2.50".into());
    assert_eq!(sales_rate_milli(&padded, AS_OF), Ok(2_500));
}

/// Only the rates the lab measured on this path are admitted: 5 percent (2.5
/// a head) and 18 percent (9 a head, on one exact bill, voucher 0009 of
/// §9.17). Every other GST slab is refused under its own code, apart from a
/// rate that is no slab, until a lab measurement opens it.
#[test]
fn only_the_measured_rates_are_admitted_and_each_other_slab_is_refused_as_unmeasured() {
    let rate = |half: &str, igst: &str| sales_rate_milli(&sales_rates(half, igst), AS_OF);
    assert_eq!(rate("2.5", "5"), Ok(2_500));
    assert_eq!(rate("9", "18"), Ok(9_000));
    for (half, igst) in [("6", "12"), ("14", "28"), ("20", "40")] {
        assert_eq!(
            rate(half, igst),
            Err("invoice_sales_ledger_rate_not_measured"),
            "{igst}"
        );
    }
    assert_eq!(
        rate("1.5", "3"),
        Err("invoice_sales_ledger_rate_not_a_slab")
    );
}

#[test]
fn a_tax_ledger_must_carry_the_rate_and_no_rounding() {
    let check = |change: &dyn Fn(&mut wire::LedgerRateRow)| {
        let mut row = tax_rates("2.50");
        change(&mut row);
        tax_ledger_refusal(&row, 2_500)
    };
    assert_eq!(check(&|_| {}), None);
    assert_eq!(
        check(&|r| r.rate_of_tax_calculation = Some("0".into())),
        Some("invoice_tax_ledger_rate_mismatch")
    );
    assert_eq!(
        check(&|r| r.rate_of_tax_calculation = None),
        Some("invoice_tax_ledger_rate_mismatch")
    );
    assert_eq!(
        check(&|r| r.rate_of_tax_calculation = Some("9".into())),
        Some("invoice_tax_ledger_rate_mismatch")
    );
    assert_eq!(
        check(&|r| r.rounding_method = None),
        Some("invoice_tax_ledger_rounding_unsupported")
    );
    assert_eq!(
        check(&|r| r.rounding_method = Some("Upward".into())),
        Some("invoice_tax_ledger_rounding_unsupported")
    );
    assert_eq!(
        check(&|r| r.rounding_limit = Some("1".into())),
        Some("invoice_tax_ledger_rounding_unsupported")
    );
    assert_eq!(
        check(&|r| r.rounding_limit = None),
        Some("invoice_tax_ledger_rounding_unsupported")
    );
}

/// The refusals of an invoice's tax that come from its ledgers, through the
/// whole classification: each names the ledger it concerns.
#[test]
fn the_tax_check_refuses_on_the_ledgers_doubts_and_names_them() {
    let refused = |change: &dyn Fn(&mut BTreeMap<String, LedgerFacts>)| {
        let mut facts = good_facts();
        change(&mut facts);
        classify_sales_invoice(&voucher(), &facts, RAJ, AS_OF).unwrap_err()
    };
    // The listing returned no row for the sales ledger.
    assert_eq!(
        refused(&|f| f.get_mut("Sales").unwrap().rates = None),
        vec![refuse_ledger("invoice_sales_ledger_rate_unknown", "Sales")]
    );
    // A listing that holds rows, but none of the sales ledger: the lab's own,
    // read for this invoice's ledgers, which it does not carry.
    assert_eq!(
        refused(&|f| {
            let listed = lab_rates(&["Sales"]);
            f.get_mut("Sales").unwrap().rates = listed.get("Sales").cloned();
        }),
        vec![refuse_ledger("invoice_sales_ledger_rate_unknown", "Sales")]
    );
    // A tax ledger that rounds, or whose own rate is not the sales ledger's.
    assert_eq!(
        refused(&|f| f
            .get_mut("Output SGST")
            .unwrap()
            .rates
            .as_mut()
            .unwrap()
            .rounding_limit = Some("1".into())),
        vec![refuse_ledger(
            "invoice_tax_ledger_rounding_unsupported",
            "Output SGST"
        )]
    );
    // A tax ledger the listing did not return: its rate is not known, which
    // is not a rate that differs.
    assert_eq!(
        refused(&|f| f.get_mut("Output CGST").unwrap().rates = None),
        vec![refuse_ledger(
            "invoice_tax_ledger_rate_unknown",
            "Output CGST"
        )]
    );
    assert_eq!(
        refused(&|f| f.get_mut("Output SGST").unwrap().rates = None),
        vec![refuse_ledger(
            "invoice_tax_ledger_rate_unknown",
            "Output SGST"
        )]
    );
    // A tax ledger that was returned with another rate.
    assert_eq!(
        refused(&|f| f.get_mut("Output CGST").unwrap().rates = Some(tax_rates("2.5"))),
        vec![refuse_ledger(
            "invoice_tax_ledger_rate_mismatch",
            "Output CGST"
        )]
    );
    // Two sales ledgers at two rates: one tax pair cannot carry both.
    let mut two = voucher();
    two.entries.truncate(1);
    two.entries.push(entry("Sales", "6000.00", EntrySide::Cr));
    two.entries.push(entry("Sales B", "4000.00", EntrySide::Cr));
    two.entries
        .push(entry("Output CGST", "900.00", EntrySide::Cr));
    two.entries
        .push(entry("Output SGST", "900.00", EntrySide::Cr));
    let mut facts = good_facts();
    facts.extend(facts_for(&[(
        "Sales B",
        &["Sales Accounts"],
        DutyHead::NotTax,
    )]));
    facts.get_mut("Sales B").unwrap().rates = Some(sales_rates("2.5", "5"));
    assert_eq!(
        classify_sales_invoice(&two, &facts, RAJ, AS_OF).unwrap_err(),
        vec![refuse("invoice_sales_ledgers_rates_differ")]
    );
}

// ---- Tally's GSTR-1 status of a posted invoice (10 Oct 2026, the pilot lab) ----

/// The lab's read-back of `BP/26-27/0010` with the four status fields, the
/// request the code renders: Tally lists it as included, not accepted as it
/// stands.
const STATUS_INCLUDED: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-status.utf16le.xml"
);
/// The same request for `BP/26-27/0021` after its sales ledger's HSN/SAC was
/// set, the voucher untouched: uncertain.
const STATUS_UNCERTAIN: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-status-uncertain.utf16le.xml"
);
/// The same request for `BP/26-27/0021` once the voucher was saved again with
/// no change: included.
const STATUS_RESAVED: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-status-resaved.utf16le.xml"
);
/// The read-back of `BP/26-27/0010` the invoice round took (9 Oct 2026), whose
/// request asked for no status field.
const STATUS_NOT_ASKED: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-after-post.utf16le.xml"
);
/// The same field list as the read-back for three numbers at once:
/// `BP/26-27/0002` uncertain, `0010` included, `0016` accepted as it stands.
const STATUS_THREE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-three-vouchers.utf16le.xml"
);
/// The GST status read's narrow window: seven invoices and four Receipts.
const STATUS_WINDOW: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-status-window.utf16le.xml"
);
const STATUS_TAGS: [&str; 4] = [
    "VCHGSTSTATUSISINCLUDED",
    "VCHGSTSTATUSISUNCERTAIN",
    "VCHGSTSTATUSISEXCLUDED",
    "ISGSTOVERRIDDEN",
];

fn utf16le(bytes: &[u8]) -> String {
    assert_eq!(bytes.len() % 2, 0, "UTF-16LE is whole code units");
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The one element `tag` of `xml`, from its open tag to its close tag.
fn status_element<'a>(xml: &'a str, tag: &str) -> &'a str {
    let open = format!("<{tag}");
    let starts = xml
        .match_indices(&open)
        .map(|(at, _)| at)
        .filter(|at| matches!(xml.as_bytes()[at + open.len()], b'>' | b' '))
        .collect::<Vec<_>>();
    assert_eq!(starts.len(), 1, "one {tag}");
    let close = format!("</{tag}>");
    let end = starts[0] + xml[starts[0]..].find(&close).unwrap() + close.len();
    &xml[starts[0]..end]
}

/// The four status elements of the lab's read-back of an included invoice, as
/// Tally wrote them.
fn included_status_elements() -> String {
    let xml = utf16le(STATUS_INCLUDED);
    STATUS_TAGS
        .iter()
        .map(|tag| status_element(&xml, tag))
        .collect()
}

/// `xml` with the text of its one `tag` element set to `value`.
fn with_flag(xml: &str, tag: &str, value: &str) -> String {
    let element = status_element(xml, tag);
    let text_at = element.find('>').unwrap() + 1;
    let text_end = element.rfind("</").unwrap();
    let edited = format!("{}{value}{}", &element[..text_at], &element[text_end..]);
    xml.replacen(element, &edited, 1)
}

/// `xml` without its one `tag` element.
fn without(xml: &str, tag: &str) -> String {
    let element = status_element(xml, tag).to_string();
    xml.replacen(&element, "", 1)
}

fn codes_of(fields: &BTreeMap<String, String>) -> Vec<&'static str> {
    gst_status_differences(fields)
        .into_iter()
        .map(GstStatusDifference::code)
        .collect()
}

/// The status codes of the one voucher a read-back answer holds.
fn status_codes(xml: &str) -> Vec<&'static str> {
    match wire::parse_invoice_readback(xml).unwrap() {
        wire::Readback::One(read) => codes_of(&read.fields),
        other => panic!("one voucher expected, got {other:?}"),
    }
}

/// Each voucher's number and status codes, each voucher's fields read as the
/// read-back reads its one voucher's.
fn status_codes_by_number(xml: &str) -> Vec<(String, Vec<&'static str>)> {
    wire::rows(xml, "VOUCHER")
        .unwrap()
        .iter()
        .map(|row| {
            let fields = wire::readback_fields(row).unwrap();
            (fields["VOUCHERNUMBER"].clone(), codes_of(&fields))
        })
        .collect()
}

/// Every state the lab's answers show, each read by the read-back's own parse
/// and the GST status read's readings: included (twice, one after a re-save),
/// uncertain, accepted as it stands and not in the return. Only included and not
/// accepted is clean.
#[test]
fn each_gstr_1_status_the_lab_answered_reads_as_its_own_code() {
    assert_eq!(status_codes(&utf16le(STATUS_INCLUDED)), Vec::<&str>::new());
    assert_eq!(
        status_codes(&utf16le(STATUS_UNCERTAIN)),
        ["gst_status_uncertain"]
    );
    // The ledger's HSN/SAC alone left it uncertain; saving the voucher again
    // (AlterID 34 to 35) made it included.
    assert_eq!(status_codes(&utf16le(STATUS_RESAVED)), Vec::<&str>::new());
    let number_and_alter_id = |bytes: &[u8]| match wire::parse_invoice_readback(&utf16le(bytes)) {
        Ok(wire::Readback::One(read)) => (
            read.fields["VOUCHERNUMBER"].clone(),
            read.fields["ALTERID"].clone(),
        ),
        other => panic!("one voucher expected, got {other:?}"),
    };
    assert_eq!(
        number_and_alter_id(STATUS_UNCERTAIN),
        ("BP/26-27/0021".to_string(), "34".to_string())
    );
    assert_eq!(
        number_and_alter_id(STATUS_RESAVED),
        ("BP/26-27/0021".to_string(), "35".to_string())
    );
    assert_eq!(
        status_codes_by_number(&utf16le(STATUS_THREE)),
        [
            ("BP/26-27/0002".to_string(), vec!["gst_status_uncertain"]),
            ("BP/26-27/0010".to_string(), vec![]),
            ("BP/26-27/0016".to_string(), vec!["gst_status_overridden"]),
        ]
    );
    let window = status_codes_by_number(&utf16le(STATUS_WINDOW));
    let receipts =
        ["1", "2", "3", "4"].map(|number| (number.to_string(), vec!["gst_status_not_in_return"]));
    assert_eq!(window[7..], receipts);
    assert_eq!(
        window[..7]
            .iter()
            .map(|(number, codes)| (number.as_str(), codes.clone()))
            .collect::<Vec<_>>(),
        [
            ("BP/26-27/0012", vec![]),
            ("BP/26-27/0013", vec![]),
            ("BP/26-27/0014", vec![]),
            ("BP/26-27/0015", vec![]),
            ("BP/26-27/0016", vec!["gst_status_overridden"]),
            ("Z1/0001", vec![]),
            ("BP/26-27/0019", vec![]),
        ]
    );
    // The same codes reach the invoice's differences after its own fields.
    let v = observed_voucher();
    let uncertain = utf16le(STATUS_UNCERTAIN);
    let mut flagged = readback_xml(&v);
    for tag in STATUS_TAGS {
        flagged = flagged.replacen(
            status_element(&readback_xml(&v), tag),
            status_element(&uncertain, tag),
            1,
        );
    }
    assert_eq!(differences(&v, &flagged), vec!["gst_status_uncertain"]);
}

/// An answer that carries no status field is one difference, never a pass and
/// never two: the read-back the invoice round took before the fields were asked
/// for says `gst_status_not_reported` once, although both of the status read's
/// readings (the three flags and the override flag) report it.
#[test]
fn status_fields_that_did_not_come_back_are_one_difference_never_a_pass() {
    let not_asked = utf16le(STATUS_NOT_ASKED);
    assert!(STATUS_TAGS.iter().all(|tag| !not_asked.contains(tag)));
    assert_eq!(status_codes(&not_asked), ["gst_status_not_reported"]);
    // DERIVED, not Tally's bytes: edited in one place each (counted), from the
    // included answer: no capture lacks a single field.
    let included = utf16le(STATUS_INCLUDED);
    assert_eq!(
        status_codes(&without(&included, "ISGSTOVERRIDDEN")),
        ["gst_status_not_reported"]
    );
    assert_eq!(
        status_codes(&without(&included, "VCHGSTSTATUSISUNCERTAIN")),
        ["gst_status_unreadable"]
    );
    assert_eq!(
        status_codes(&without(
            &without(&included, "VCHGSTSTATUSISUNCERTAIN"),
            "ISGSTOVERRIDDEN"
        )),
        ["gst_status_unreadable", "gst_status_not_reported"]
    );
    // An empty flag came back as nothing, so it is read as one not reported.
    assert_eq!(
        status_codes(&with_flag(&included, "ISGSTOVERRIDDEN", "")),
        ["gst_status_not_reported"]
    );
}

/// DERIVED, not Tally's bytes: the states no capture shows, each from a
/// captured answer edited in one place (the flag's text, counted): excluded (never observed), uncertain and accepted
/// together, two flags Yes, and a value that is neither Yes nor No.
#[test]
fn a_state_no_capture_shows_is_read_from_a_captured_answer_edited_in_one_place() {
    let included = utf16le(STATUS_INCLUDED);
    let excluded = with_flag(
        &with_flag(&included, "VCHGSTSTATUSISINCLUDED", "No"),
        "VCHGSTSTATUSISEXCLUDED",
        "Yes",
    );
    assert_eq!(status_codes(&excluded), ["gst_status_excluded"]);
    assert_eq!(
        status_codes(&with_flag(
            &utf16le(STATUS_UNCERTAIN),
            "ISGSTOVERRIDDEN",
            "Yes"
        )),
        ["gst_status_uncertain", "gst_status_overridden"]
    );
    assert_eq!(
        status_codes(&with_flag(&included, "VCHGSTSTATUSISUNCERTAIN", "Yes")),
        ["gst_status_unreadable"]
    );
    assert_eq!(
        status_codes(&with_flag(&included, "ISGSTOVERRIDDEN", "Unknown")),
        ["gst_status_unreadable"]
    );
}

// ---- The HSN/SAC Tally used for each sales line (10 Oct 2026, the pilot lab) ----

/// The five elements of a ledger entry that say which HSN/SAC Tally used and
/// where it took it from.
const HSN_TAGS: [&str; 5] = [
    "HSNSOURCETYPE",
    "HSNLEDGERSOURCE",
    "HSNGROUPSOURCE",
    "GSTHSNNAME",
    "GSTHSNINFERAPPLICABILITY",
];

/// The one ledger entry of `xml` on `ledger`, from its open tag to its close tag.
fn leg_of<'a>(xml: &'a str, ledger: &str) -> &'a str {
    let name = format!(">{ledger}</LEDGERNAME>");
    assert_eq!(xml.matches(&name).count(), 1, "one leg on {ledger}");
    let at = xml.find(&name).unwrap();
    let start = xml[..at].rfind("<ALLLEDGERENTRIES.LIST>").unwrap();
    let close = "</ALLLEDGERENTRIES.LIST>";
    &xml[start..at + xml[at..].find(close).unwrap() + close.len()]
}

/// The one `tag` element of `leg`, written empty (`<TAG/>`) or with its text.
fn leg_element<'a>(leg: &'a str, tag: &str) -> &'a str {
    let empty = format!("<{tag}/>");
    let open = format!("<{tag}>");
    match (leg.matches(&empty).count(), leg.matches(&open).count()) {
        (1, 0) => &leg[leg.find(&empty).unwrap()..][..empty.len()],
        (0, 1) => {
            let start = leg.find(&open).unwrap();
            let close = format!("</{tag}>");
            &leg[start..start + leg[start..].find(&close).unwrap() + close.len()]
        }
        other => panic!("one {tag} expected, found {other:?}"),
    }
}

/// `xml` with the `tag` element of the leg on `ledger` replaced by `element`
/// (empty text to remove it), nothing else changed.
fn with_leg_element(xml: &str, ledger: &str, tag: &str, element: &str) -> String {
    let leg = leg_of(xml, ledger);
    let edited = leg.replacen(leg_element(leg, tag), element, 1);
    assert_ne!(edited, leg, "{ledger} {tag}");
    xml.replacen(leg, &edited, 1)
}

/// The five HSN/SAC elements of the first sales line of the lab's read-back of
/// `BP/26-27/0010`, as Tally wrote them: code 998313, from its own ledger.
fn resolved_hsn_elements() -> String {
    let xml = utf16le(STATUS_INCLUDED);
    let leg = leg_of(&xml, SVC_998313);
    HSN_TAGS.iter().map(|tag| leg_element(leg, tag)).collect()
}

/// A Sales invoice of the pilot lab's `BRIDGE Sales` type to its walk-in
/// customer, as a build records it, written from what the lab's read-back says
/// the voucher holds.
fn pilot_invoice(
    (number, date): (&str, &str),
    total: &str,
    credits: &[(&str, &str)],
    sales: &[&str],
) -> ImportVoucher {
    let mut entries = vec![entry("BRIDGE Walk-in", total, EntrySide::Dr)];
    entries.extend(
        credits
            .iter()
            .map(|(ledger, amount)| entry(ledger, amount, EntrySide::Cr)),
    );
    ImportVoucher {
        bridge_txn_id: "pilot".to_string(),
        date: bridge_tally_core::TallyDate::parse(date).expect("a pilot date"),
        voucher_type: VoucherType::Sales,
        narration: None,
        reference: None,
        voucher_number: Some(number.to_string()),
        invoice: Some(InvoiceDetail {
            voucher_type_name: "BRIDGE Sales".to_string(),
            place_of_supply: RAJ.to_string(),
            round_off_ledger: credits
                .iter()
                .find(|(ledger, _)| *ledger == "BRIDGE Round Off")
                .map(|(ledger, _)| ledger.to_string()),
            observed: Some(InvoiceObserved {
                voucher_type_guid: "6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce".to_string(),
                party_gstin: None,
                party_state: RAJ.to_string(),
                party_registration_type: "Unregistered/Consumer".to_string(),
                party_bill_wise: false,
                company_state: RAJ.to_string(),
                sales_ledgers: sales.iter().map(|ledger| ledger.to_string()).collect(),
                hsn_warnings: Vec::new(),
            }),
        }),
        entries,
    }
}

const SVC_998313: &str = "BRIDGE Svc 998313 5%";
const SVC_998314: &str = "BRIDGE Svc 998314 5%";
const SVC_998315: &str = "BRIDGE Svc 998315 5%";

/// `BP/26-27/0010` as the pilot replay builds and posts it.
fn pilot_0010() -> ImportVoucher {
    pilot_invoice(
        ("BP/26-27/0010", "20260801"),
        "1051.00",
        &[
            (SVC_998313, "600.40"),
            (SVC_998314, "400.40"),
            ("BRIDGE CGST 2.5%", "25.02"),
            ("BRIDGE SGST 2.5%", "25.02"),
            ("BRIDGE Round Off", "0.16"),
        ],
        &[SVC_998313, SVC_998314],
    )
}

/// `BP/26-27/0021`, three sales lines on three sibling ledgers. Its read-backs
/// carry no reference, so a build's expectation differs there as well.
fn pilot_0021() -> ImportVoucher {
    pilot_invoice(
        ("BP/26-27/0021", "20260805"),
        "315.30",
        &[
            (SVC_998313, "100.10"),
            (SVC_998314, "100.10"),
            (SVC_998315, "100.10"),
            ("BRIDGE CGST 2.5%", "7.50"),
            ("BRIDGE SGST 2.5%", "7.50"),
        ],
        &[SVC_998313, SVC_998314, SVC_998315],
    )
}

/// The differences and the sales-line evidence of a read-back answer, as a
/// post reads them.
fn hsn_outcome(v: &ImportVoucher, xml: &str) -> (Vec<String>, Vec<serde_json::Value>) {
    let date = v.date.as_str().to_string();
    let (differences, _, _, lines) = readback_outcome(wire::parse_invoice_readback(xml), v, &date);
    (differences, lines)
}

/// One sales line's evidence: Tally used `code` from the line's own ledger, or
/// gave no code and no source.
fn line_evidence(ledger: &str, code: Option<&str>) -> serde_json::Value {
    let name = |text: &str| json!(crate::agent::party_name(text));
    json!({
        "ledger": name(ledger),
        "hsn_code": code,
        "hsn_source_type": code.map(|_| "Ledger"),
        "hsn_ledger_source": code.map(|_| name(ledger)),
        "hsn_group_source": null,
        "hsn_resolved": code.is_some(),
    })
}

/// The leg of a parsed read-back on `ledger`.
fn read_leg(xml: &str, ledger: &str) -> wire::ReadLeg {
    match wire::parse_invoice_readback(xml).unwrap() {
        wire::Readback::One(read) => read
            .legs
            .into_iter()
            .find(|leg| leg.ledger == ledger)
            .unwrap(),
        other => panic!("one voucher expected, got {other:?}"),
    }
}

/// #691: `BP/26-27/0021` read while Tally's GSTR-1 listed it as uncertain: its
/// third sales line carries the HSN/SAC elements empty (the applicability alone
/// filled), the other two carry their codes from their own ledgers. It is
/// divergent with `invoice_line_hsn_unresolved`, and the evidence names the
/// line: `BRIDGE Svc 998315 5%`, with no code.
#[test]
fn a_sales_line_tally_filled_no_hsn_for_is_divergent_and_named() {
    let uncertain = utf16le(STATUS_UNCERTAIN);
    let (differences, lines) = hsn_outcome(&pilot_0021(), &uncertain);
    assert_eq!(
        differences,
        [
            "REFERENCE",
            "REFERENCEDATE",
            "invoice_line_hsn_unresolved",
            "gst_status_uncertain"
        ]
    );
    assert_eq!(
        lines,
        [
            line_evidence(SVC_998313, Some("998313")),
            line_evidence(SVC_998314, Some("998314")),
            line_evidence(SVC_998315, None),
        ]
    );
    // What the parse read on that line: present and empty, never absent.
    let empty = wire::EntryText::Empty;
    assert_eq!(
        read_leg(&uncertain, SVC_998315).hsn,
        wire::LegHsn {
            code: empty.clone(),
            source_type: empty.clone(),
            ledger_source: empty.clone(),
            group_source: empty,
            applicability: wire::EntryText::Text("As per Masters/Company".into()),
        }
    );
}

/// The same voucher once it was saved again (AlterID 35), and `BP/26-27/0010`:
/// every sales line names its code and its ledger, and nothing differs for
/// this check. The party, tax and round-off legs carry the elements empty and
/// are not judged.
#[test]
fn every_sales_line_with_a_code_and_a_source_is_clean_and_shown() {
    let (differences, lines) = hsn_outcome(&pilot_0021(), &utf16le(STATUS_RESAVED));
    assert_eq!(differences, ["REFERENCE", "REFERENCEDATE"]);
    assert_eq!(
        lines,
        [
            line_evidence(SVC_998313, Some("998313")),
            line_evidence(SVC_998314, Some("998314")),
            line_evidence(SVC_998315, Some("998315")),
        ]
    );
    let included = utf16le(STATUS_INCLUDED);
    let (differences, lines) = hsn_outcome(&pilot_0010(), &included);
    assert_eq!(differences, Vec::<String>::new());
    assert_eq!(
        lines,
        [
            line_evidence(SVC_998313, Some("998313")),
            line_evidence(SVC_998314, Some("998314")),
        ]
    );
    for ledger in ["BRIDGE Walk-in", "BRIDGE CGST 2.5%", "BRIDGE Round Off"] {
        assert!(!read_leg(&included, ledger).hsn.resolved(), "{ledger}");
    }
}

/// DERIVED, not Tally's bytes: the states no capture shows, each from the
/// re-saved read-back of `BP/26-27/0021` edited in one place (counted) on its third sales line: a source with
/// no code, a code with no source, the five elements not returned at all, and
/// a code behind a control-character reference (cleaned as every other
/// value). Only a code and a source together are clean.
#[test]
fn a_sales_line_missing_its_code_or_its_source_is_unresolved() {
    let resaved = utf16le(STATUS_RESAVED);
    let unresolved = |xml: &str| {
        let (differences, lines) = hsn_outcome(&pilot_0021(), xml);
        assert_eq!(lines[2]["hsn_resolved"], false, "{lines:?}");
        differences
    };
    let expected = ["REFERENCE", "REFERENCEDATE", "invoice_line_hsn_unresolved"];
    let no_code = with_leg_element(&resaved, SVC_998315, "GSTHSNNAME", "<GSTHSNNAME/>");
    assert_eq!(unresolved(&no_code), expected);
    assert_eq!(
        read_leg(&no_code, SVC_998315).hsn.source_type,
        wire::EntryText::Text("Ledger".into())
    );
    let no_source = with_leg_element(&resaved, SVC_998315, "HSNSOURCETYPE", "<HSNSOURCETYPE/>");
    assert_eq!(unresolved(&no_source), expected);
    assert_eq!(
        read_leg(&no_source, SVC_998315).hsn.code,
        wire::EntryText::Text("998315".into())
    );
    let absent = HSN_TAGS.iter().fold(resaved.clone(), |xml, tag| {
        with_leg_element(&xml, SVC_998315, tag, "")
    });
    assert_eq!(unresolved(&absent), expected);
    let gone = wire::EntryText::Absent;
    assert_eq!(
        read_leg(&absent, SVC_998315).hsn,
        wire::LegHsn {
            code: gone.clone(),
            source_type: gone.clone(),
            ledger_source: gone.clone(),
            group_source: gone.clone(),
            applicability: gone,
        }
    );
    // A control-character reference alone is no value; before a code it is
    // not part of it.
    let marked_empty = with_leg_element(
        &resaved,
        SVC_998315,
        "GSTHSNNAME",
        "<GSTHSNNAME>&#4;</GSTHSNNAME>",
    );
    assert_eq!(unresolved(&marked_empty), expected);
    let marked = with_leg_element(
        &resaved,
        SVC_998315,
        "GSTHSNNAME",
        "<GSTHSNNAME>&#4; 998315</GSTHSNNAME>",
    );
    assert_eq!(
        read_leg(&marked, SVC_998315).hsn.code,
        wire::EntryText::Text("998315".into())
    );
    assert_eq!(
        hsn_outcome(&pilot_0021(), &marked).0,
        ["REFERENCE", "REFERENCEDATE"]
    );
    // An element Tally wrote twice on a leg fails the read, never picks one.
    let twice = with_leg_element(
        &resaved,
        SVC_998315,
        "GSTHSNNAME",
        "<GSTHSNNAME>998315</GSTHSNNAME><GSTHSNNAME/>",
    );
    assert_eq!(
        wire::parse_invoice_readback(&twice),
        Err("invoice_read_field_repeated")
    );
}

/// A batch saved before the build recorded its sales lines cannot say which
/// legs to judge: its read-back is `invoice_not_observed`, never a pass.
#[test]
fn an_invoice_whose_sales_lines_were_not_recorded_is_not_observed() {
    let mut v = pilot_0010();
    v.invoice
        .as_mut()
        .unwrap()
        .observed
        .as_mut()
        .unwrap()
        .sales_ledgers
        .clear();
    let (differences, lines) = hsn_outcome(&v, &utf16le(STATUS_INCLUDED));
    assert_eq!(differences, ["invoice_not_observed"]);
    assert_eq!(lines, Vec::<serde_json::Value>::new());
}

/// A sales ledger whose code resolved is warned about and never refused; one
/// whose code did not is refused and carries no warning.
#[test]
fn a_resolved_code_is_warned_about_and_an_unresolved_one_refused() {
    let resolved = |code: &str| wire::HsnStep::Resolved {
        code: code.to_string(),
        level: wire::HsnLevel::Ledger,
    };
    let (refusals, warnings) = hsn_admission(vec![
        ("A", resolved("9983"), wire::SupplyType::Services),
        ("B", resolved("998313"), wire::SupplyType::Services),
        (
            "C",
            wire::HsnStep::Refused(wire::HSN_NOT_MEASURED),
            wire::SupplyType::Services,
        ),
        ("D", wire::HsnStep::NeedsGroups, wire::SupplyType::Services),
    ]);
    assert_eq!(
        refusals,
        [
            refuse_ledger(wire::HSN_NOT_MEASURED, "C"),
            refuse_ledger(wire::HSN_NOT_MEASURED, "D")
        ]
    );
    assert_eq!(
        warnings,
        wire::hsn_warnings(
            "A",
            "9983",
            &wire::HsnLevel::Ledger,
            wire::SupplyType::Services
        )
    );
    assert_eq!(warnings.len(), 1);
}

/// The divergent invoice's next step must not promise that saving the invoice again repairs it:
/// a save changes the voucher's AlterID, and what a verification reads after one is not
/// established (voucher writes reference, section 9.16).
#[test]
fn the_divergent_next_step_does_not_promise_a_resave_repairs_the_invoice() {
    let text = crate::agent::agent_import::verification::INVOICE_DIVERGENT_NEXT_STEP;
    assert!(!text.contains("then open the invoice and save it again"));
    assert!(text.contains("changes the invoice's AlterID"));
    assert!(text.contains("has not been established"));
    assert!(text.ends_with("Do not post it again."));
}
