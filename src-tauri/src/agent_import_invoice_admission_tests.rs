//! One Sales invoice admission, and one read-back, run from the first request
//! to the last against the scripted transport, on one book's answers only:
//! BRIDGE PILOT LAB (the `pilot-lab` set; its PROVENANCE table is beside the
//! fixtures). Every answer is Tally's own bytes, from three days of the book
//! (the 9 Oct set, the company object `O` of 12 Oct and the ledger catalogue of
//! 21 Oct), except the rates read `Q`, which is DERIVED: the lab's 21 Oct answer
//! to the read restricted to the invoice ledgers' parents, cut to the invoice's
//! ledgers (below).
//!
//! - The admission: the company list, the marks, the book extent, the
//!   currencies, the ledger compliance listing, the paired listing, the
//!   groups, the voucher types, the number read and the company's tax units,
//!   from the third round and the invoice round before the post (9 Oct, marks
//!   16 and 223 in both). So is the number read for a number the book uses, from the same day's
//!   second round (marks 15 and 223: between the rounds one voucher of
//!   another number was keyed, and the voucher it finds reads the same
//!   alteration id in both); the lab sent it as the number control's read,
//!   whose request is the same bytes. The company object the HSN/SAC check
//!   reads is the 12 Oct answer (`O`).
//! - The rate listing: the compliance listing with each ledger's GST rate and
//!   rounding, restricted to the invoice's ledgers by GUID (#1331) (`Q`).
//!   DERIVED: the code's request for these six ledgers was never sent to the
//!   lab; the answer served is the lab's 21 Oct 2026 answer to the request
//!   restricted to their parents, cut to the six (`rates_cut_to`, held equal to
//!   the lab's GUID answers of 31 Oct 2026 for five other ledgers). The ledger
//!   catalogue read in the same minute gives the party's bill-wise flag and the
//!   GUIDs. Other derived answers here are counted edits of the same answer: its
//!   ledger rows removed, or cut to another invoice's ledgers.
//! - The read-back: the invoice round's marks after the post (17 and 223) and
//!   its read-back of the posted invoice, before and after the post.
use super::*;
use crate::agent::agent_import::invoice::InvoiceJudge;
use crate::tally::approved_import::{InvoiceAnswers, InvoiceReadPlan};

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// A captured answer, served as Tally serves it.
fn answer(body: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(body))
        .with_encoding(WireEncoding::Utf16LeNoBom)
        .with_framing(ResponseFraming::ContentLength)
}

fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(
        tally_protocol_simulator::ProductStatus::TallyPrime,
    ))
}

/// A server over a simulator that answers `plans` in order, with the company
/// and its verified identity taken from that book's captured company list.
struct Lab {
    simulator: SequenceSimulator,
    server: Server,
    identity: crate::tally::VerifiedCompanyIdentity,
    company: bridge_tally_protocol::TallyCompany,
    _directory: tempfile::TempDir,
}

fn server_over(mut plans: Vec<ScenarioPlan>, company_list: &str, guid: &str) -> Lab {
    // One answer more than is asked for, so a request too many is counted
    // instead of refused.
    plans.push(answer(company_list.to_string()));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(super::super::super::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().into(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: super::super::super::Redaction::None,
        import_enabled: true,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let companies = bridge_tally_protocol::parse_companies_from_collection(company_list).unwrap();
    let company = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(guid))
        .expect("the lab company")
        .clone();
    let identity = crate::tally::VerifiedCompanyIdentity::from_observed_companies(
        company.name.clone(),
        crate::tally::test_company_guid(guid),
        company.company_number.clone().unwrap(),
        company.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap();
    Lab {
        simulator,
        server,
        identity,
        company,
        _directory: directory,
    }
}

/// The requests the simulator received, in order.
fn received(simulator: SequenceSimulator) -> Vec<tally_protocol_simulator::ObservedRequest> {
    simulator.cancel();
    simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .collect()
}

/// How many requests the simulator received.
fn sent(simulator: SequenceSimulator) -> usize {
    received(simulator).len()
}

// ---- BRIDGE PILOT LAB: the admission ----

const PILOT: &str = "BRIDGE PILOT LAB";

/// What one admission sends, in order: e company list, s status probe, m
/// marks, b book extent, c currencies, L the compliance listing, P the paired
/// listing, g groups, Q the listing with each ledger's GST rate and rounding,
/// restricted to the ledgers the invoice names, by GUID (#1331; DERIVED),
/// T voucher types, N the number read, C the company's tax units, O the company
/// object (the HSN/SAC check, once the classification admits);
/// U is the number read for a number the book uses.
///
/// The marks read; the ledger listing (its currency read inside two extents,
/// then its masters inside two more); then the three reads an invoice adds.
const ADMISSION_ORDER: &str = concat!(
    "emsmse",
    "ebsbscscsbsbse",
    "see",
    "bsbsLsLsPsPsgsgsbsbs",
    "ese",
    "eQsQse",
    "eTsTse",
    "eNsNse",
    "eCsCse",
    "eOsOse"
);

/// The lab's request files the tests pin a read to, by sha256: the number
/// read for `BP/26-27/0010` (`r09-number-new-invoice.xml`), the same read for
/// `BP/26-27/0001` (`r10-number-control-k0.xml`) and the company's tax units
/// (`r16-tax-units.xml`, the request the code sends).
const LAB_R09_SHA256: &str = "aa6e7a3ae50bd13d6eff60ffac89bd713f084a49512667edcc6d40fee0ca3985";
const LAB_R10_SHA256: &str = "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6";
const LAB_R16_SHA256: &str = "c763f19d79b1e1736ecd17578fe5f353f9b1c4d781382073286484749f181a38";
/// The rates read restricted to the lab invoice's ledgers by GUID (#1331): the
/// request the code renders, never sent to the lab with these six GUIDs. The
/// lab answered the same request naming five other ledgers on 31 Oct 2026, and
/// a test derives this pin from it (`LAB_INVOICE_GUID_RATES_SHA256`).
const LAB_Q1_SHA256: &str = super::LAB_INVOICE_GUID_RATES_SHA256;

/// The sha256 of each request sent where `order` has `letter`.
fn sent_as<'a>(
    order: &str,
    requests: &'a [tally_protocol_simulator::ObservedRequest],
    letter: char,
) -> Vec<&'a str> {
    order
        .chars()
        .zip(requests)
        .filter(|(sent, _)| *sent == letter)
        .map(|(_, request)| request.request_body_sha256.as_str())
        .collect()
}

fn pilot_company_list() -> String {
    utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-company-extent.utf16le.xml"))
}

fn pilot_plan(letter: char) -> ScenarioPlan {
    let bytes: &[u8] = match letter {
        's' => return status(),
        'e' => return answer(pilot_company_list()),
        'm' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-marks-before-post.utf16le.xml"),
        'b' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-book-extent.utf16le.xml"),
        'c' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-currencies.utf16le.xml"),
        'L' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledgers-compliance.utf16le.xml"),
        'P' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledgers-paired.utf16le.xml"),
        'g' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-groups.utf16le.xml"),
        // DERIVED: see `lab_invoice_rates_hsn`.
        'Q' => super::lab_invoice_rates_hsn(),
        'O' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-company-object.utf16le.xml"),
        'T' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-voucher-types.utf16le.xml"),
        'N' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-new-invoice.utf16le.xml"),
        'C' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-tax-units.utf16le.xml"),
        'M' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-marks-after-post.utf16le.xml"),
        'R' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-status.utf16le.xml"),
        'A' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-before-post.utf16le.xml"),
        'U' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-in-use.utf16le.xml"),
        other => panic!("unknown kind {other}"),
    };
    answer(utf16(bytes))
}

/// The ledger catalogue the build reads, as the pilot lab answered it on
/// 21 Oct 2026 (the file is the W18 book's, byte-equal to that answer and to
/// the one of 31 Oct 2026): the party's bill-wise flag and the GUIDs the rates
/// read names come from here.
fn pilot_catalogue() -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    pilot_catalogue_of(&pilot_catalogue_text())
}

fn pilot_catalogue_text() -> String {
    utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-ledger-catalogue-v2.utf16le.xml"))
}

fn pilot_catalogue_of(text: &str) -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(
        text,
        PILOT,
        &pilot_lab_guid(),
    )
    .unwrap()
}

/// DERIVED, not Tally's bytes: the lab's 10 Oct catalogue with the row of
/// `ledger` put under the group `to`, which stands for the ledger regrouped (or
/// its group renamed) in Tally. Its one `PARENT` element is replaced and the
/// parent must have changed; nothing else differs.
fn pilot_catalogue_with_parent(
    ledger: &str,
    to: &str,
) -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    pilot_catalogue_with_element(ledger, "PARENT", to)
}

/// DERIVED, not Tally's bytes: the lab's catalogue with the row of `ledger`
/// given the GUID `to`, which stands for the ledger deleted and another created
/// under its name and group. Its one `GUID` element is replaced and the GUID
/// must have changed; nothing else differs.
fn pilot_catalogue_with_guid(
    ledger: &str,
    to: &str,
) -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    pilot_catalogue_with_element(ledger, "GUID", to)
}

/// The catalogue with the first `element` of `ledger`'s row set to `to`.
fn pilot_catalogue_with_element(
    ledger: &str,
    element: &str,
    to: &str,
) -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    let text = pilot_catalogue_text();
    let row = format!("<LEDGER NAME=\"{ledger}\"");
    assert_eq!(text.matches(&row).count(), 1, "{ledger}");
    let start = text.find(&row).unwrap();
    let end = start + text[start..].find("</LEDGER>").unwrap();
    let open = start + text[start..end].find(&format!("<{element}")).unwrap();
    let from = open + text[open..end].find('>').unwrap() + 1;
    let to_end = from + text[from..end].find(&format!("</{element}>")).unwrap();
    assert_ne!(&text[from..to_end], to);
    let edited = format!("{}{to}{}", &text[..from], &text[to_end..]);
    pilot_catalogue_of(&edited)
}

fn pilot_lab() -> Lab {
    pilot_lab_over(ADMISSION_ORDER)
}

fn pilot_lab_over(order: &str) -> Lab {
    server_over(
        order.chars().map(pilot_plan).collect(),
        &pilot_company_list(),
        &pilot_lab_guid(),
    )
}

/// The pilot lab's tax units, as Tally answered the code's own request
/// (`pilot-lab-tax-units.utf16le.xml`, decoded).
fn pilot_tax_units() -> String {
    utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-tax-units.utf16le.xml"))
}

/// The captured tax units with each `(from, to, times)` applied, `from` having
/// to occur exactly `times` times: a derived answer is the capture with a
/// stated edit, never a hand-written one.
fn pilot_tax_units_edited(edits: &[(&str, &str, usize)]) -> String {
    let mut units = pilot_tax_units();
    for (from, to, times) in edits {
        assert_eq!(units.matches(from).count(), *times, "{from}");
        units = units.replace(from, to);
    }
    units
}

/// The pilot lab answering every read of `ADMISSION_ORDER` as captured, except
/// the read `kind`, which gets `body`.
fn pilot_lab_replacing(kind: char, body: String) -> Lab {
    server_over(
        ADMISSION_ORDER
            .chars()
            .map(|letter| {
                if letter == kind {
                    answer(body.clone())
                } else {
                    pilot_plan(letter)
                }
            })
            .collect(),
        &pilot_company_list(),
        &pilot_lab_guid(),
    )
}

/// The pilot lab answering as captured, except the tax-unit read, which gets
/// `units`.
fn pilot_lab_with_units(units: String) -> Lab {
    pilot_lab_replacing('C', units)
}

/// The answer to the rates read the code sends for the lab invoice: DERIVED
/// from the lab's own answer (`lab_invoice_rates_hsn`), its six ledgers' rows.
fn pilot_rate_listing() -> String {
    utf16(super::lab_invoice_rates_hsn())
}

/// DERIVED, not Tally's bytes: the captured rate listing with every ledger row
/// removed (the rows are counted, and there must be some), which is a complete
/// answer that lists no ledger.
fn pilot_rate_listing_without_ledgers() -> String {
    let mut listing = pilot_rate_listing();
    let mut removed = 0;
    while let Some(start) = listing.find("<LEDGER NAME=\"") {
        let end = start + listing[start..].find("</LEDGER>").unwrap() + "</LEDGER>".len();
        listing.replace_range(start..end, "");
        removed += 1;
    }
    assert!(removed > 0, "the capture lists no ledger");
    assert!(!listing.contains("<LEDGER "), "a ledger row is left");
    listing
}

/// The invoice the pilot lab's book took (`BP/26-27/0010`), to `party` under
/// the voucher type `filed`, as a caller sends it.
fn pilot_invoice(party: &str, filed: &str) -> ImportVoucher {
    serde_json::from_value(json!({
        "bridge_txn_id":"t1", "date":"20260801", "voucher_type":"Sales", "voucher_number":"BP/26-27/0010",
        "invoice":{"voucher_type_name":filed,"place_of_supply":"Rajasthan","round_off_ledger":"BRIDGE Round Off"},
        "entries":[
            {"ledger":party,"amount":"1051.00","side":"Dr"},
            {"ledger":"BRIDGE Svc 998313 5%","amount":"600.40","side":"Cr"},
            {"ledger":"BRIDGE Svc 998314 5%","amount":"400.40","side":"Cr"},
            {"ledger":"BRIDGE CGST 2.5%","amount":"25.02","side":"Cr"},
            {"ledger":"BRIDGE SGST 2.5%","amount":"25.02","side":"Cr"},
            {"ledger":"BRIDGE Round Off","amount":"0.16","side":"Cr"}
        ]
    }))
    .unwrap()
}

/// The admission of one invoice to `party` under the voucher type `filed`,
/// and the requests it sent. A refusal comes back as its code and the
/// ledger or value it names; a failed read as `FAILED` and its code.
async fn admit(
    party: &str,
    filed: &str,
) -> (
    Result<(), Vec<(&'static str, String)>>,
    ImportVoucher,
    Vec<tally_protocol_simulator::ObservedRequest>,
) {
    admit_on(pilot_lab(), pilot_invoice(party, filed)).await
}

/// The admission of `voucher` on `lab`, and the requests it sent.
async fn admit_on(
    lab: Lab,
    mut voucher: ImportVoucher,
) -> (
    Result<(), Vec<(&'static str, String)>>,
    ImportVoucher,
    Vec<tally_protocol_simulator::ObservedRequest>,
) {
    let outcome = match lab
        .server
        .admit_sales_invoice(
            &lab.identity,
            &lab.company,
            &mut voucher,
            &pilot_catalogue(),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(invoice::InvoiceAdmission::Refused(refusals)) => Err(refusals
            .into_iter()
            .map(|refusal| {
                (
                    refusal.code,
                    match refusal.detail {
                        invoice::RefusalDetail::Nothing => String::new(),
                        invoice::RefusalDetail::Ledger(name) => name,
                        invoice::RefusalDetail::Value(value) => value,
                    },
                )
            })
            .collect()),
        Err(invoice::InvoiceAdmission::Failed(failure)) => {
            Err(vec![("FAILED", failure.code.clone())])
        }
    };
    (outcome, voucher, received(lab.simulator))
}

/// What the build records of the pilot lab's invoice to its walk-in customer
/// under `BRIDGE Sales`: the type's GUID, an unregistered customer in the
/// company's state, not bill-wise.
fn pilot_observed() -> serde_json::Value {
    json!({
        "voucher_type_guid": format!("{}-000000ce", pilot_lab_guid()),
        "party_state":"Rajasthan", "party_registration_type":"Unregistered/Consumer",
        "party_bill_wise":false, "company_state":"Rajasthan",
        "sales_ledgers":["BRIDGE Svc 998313 5%","BRIDGE Svc 998314 5%"]
    })
}

/// The whole of one admission: 6 requests for the marks, 40 for the ledger
/// listing, and 6 each for the voucher types, the number read, the
/// company's tax units and (once the classification admits) the company object,
/// in that order. Each case stops where its answer decides
/// it, which the request count shows. The invoice the pilot lab's book took,
/// to its walk-in customer under `BRIDGE Sales`, is admitted after every read
/// and records what it observed; a ledger of the book that is no customer,
/// named as the party, is refused by the classification after every read.
#[tokio::test]
async fn an_admission_makes_every_read_in_order_and_refuses_where_its_answer_decides() {
    let walk_in = "BRIDGE Walk-in";
    let goods = "BRIDGE Goods 18%";
    for (party, filed, outcome, requests) in [
        (walk_in, "BRIDGE Sales", Ok(()), 76),
        (
            goods,
            "BRIDGE Sales",
            Err(vec![
                ("invoice_sales_must_be_credit", goods),
                ("invoice_party_missing", ""),
            ]),
            70,
        ),
        // A type whose series is not Manual, and a type the book does not
        // have, stop before the number is read.
        (
            walk_in,
            "Sales",
            Err(vec![("invoice_voucher_type_numbering_not_manual", "Sales")]),
            58,
        ),
        (
            walk_in,
            "Sales Acc",
            Err(vec![("invoice_voucher_type_not_found", "Sales Acc")]),
            58,
        ),
    ] {
        // Naming a sales ledger as the party changes the ledgers the rate read
        // names: the rates answer is the lab's four-parent answer cut to them
        // (DERIVED, see `rates_cut_to`).
        let (seen, voucher, received) = if party == goods {
            let legs = [
                goods,
                "BRIDGE Svc 998313 5%",
                "BRIDGE Svc 998314 5%",
                "BRIDGE CGST 2.5%",
                "BRIDGE SGST 2.5%",
                "BRIDGE Round Off",
            ];
            let answer = super::rates_cut_to(
                &utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-scoped-hsn.utf16le.xml")),
                &legs,
            );
            admit_on(
                pilot_lab_replacing('Q', answer),
                pilot_invoice(party, filed),
            )
            .await
        } else {
            admit(party, filed).await
        };
        let expected = outcome.map_err(|refusals| {
            refusals
                .into_iter()
                .map(|(code, detail)| (code, detail.to_string()))
                .collect::<Vec<_>>()
        });
        assert_eq!(seen, expected, "{party} under {filed}");
        assert_eq!(received.len(), requests, "{party} under {filed}");
        let observed = voucher.invoice.as_ref().unwrap().observed.as_ref();
        if expected.is_ok() {
            assert_eq!(
                serde_json::to_value(observed.unwrap()).unwrap(),
                pilot_observed()
            );
            // Both sends of the number read and of the tax-unit read
            // are the requests the lab sent, byte for byte.
            assert_eq!(
                sent_as(ADMISSION_ORDER, &received, 'N'),
                [LAB_R09_SHA256; 2]
            );
            assert_eq!(
                sent_as(ADMISSION_ORDER, &received, 'C'),
                [LAB_R16_SHA256; 2]
            );
            assert_eq!(sent_as(ADMISSION_ORDER, &received, 'Q'), [LAB_Q1_SHA256; 2]);
        } else {
            assert!(observed.is_none(), "{party} under {filed}");
        }
    }
}

/// The company's registration is the admission's last read: a registration
/// the build cannot issue under is a refusal under its own code, and an answer
/// with no tax unit at all is a failed read, never "no registration". Both
/// come after every other read (70 requests) and record no observation. The
/// answers are the pilot lab's capture with one stated edit each.
#[tokio::test]
async fn a_company_registration_that_cannot_be_issued_under_is_refused_and_an_unread_one_fails() {
    let units = pilot_tax_units();
    let first = units.find("<TAXUNIT NAME=").unwrap();
    let end = units.rfind("</TAXUNIT>").unwrap() + "</TAXUNIT>".len();
    assert_eq!(units.matches("<TAXUNIT NAME=").count(), 2);
    for (answer, outcome) in [
        (
            // The one registration says Composition, not Regular.
            pilot_tax_units_edited(&[(
                "<REGISTRATIONTYPE>Regular",
                "<REGISTRATIONTYPE>Composition",
                1,
            )]),
            ("invoice_company_registration_not_regular", ""),
        ),
        (
            // Both tax units removed: nothing to read.
            format!("{}{}", &units[..first], &units[end..]),
            ("FAILED", "invoice_company_registration_unread"),
        ),
    ] {
        let (seen, voucher, requests) = admit_on(
            pilot_lab_with_units(answer),
            pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales"),
        )
        .await;
        assert_eq!(seen, Err(vec![(outcome.0, outcome.1.to_string())]));
        assert_eq!(requests.len(), 70);
        assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
    }
}

/// A rate listing with no ledger row is a failed read under its own code, never
/// "this ledger has no rate": the company's ledgers were listed just before it.
/// The admission stops at that read (52 requests) and records no observation.
/// The answer is the lab's rate listing with its ledger rows removed.
#[tokio::test]
async fn an_empty_rate_listing_fails_the_admission_at_that_read() {
    let (seen, voucher, requests) = admit_on(
        pilot_lab_replacing('Q', pilot_rate_listing_without_ledgers()),
        pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales"),
    )
    .await;
    assert_eq!(
        seen,
        Err(vec![(
            "FAILED",
            "invoice_ledger_rates_listing_empty".to_string()
        )])
    );
    assert_eq!(requests.len(), 52);
    assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
}

/// The registration's state is the supplier's state that classification
/// compares the place of supply with: a registration in Haryana (its GSTIN of
/// that state, held in both places) refuses an invoice supplied in Rajasthan.
/// The answer is the pilot lab's capture with its GSTIN and state edited.
#[tokio::test]
async fn the_registrations_state_is_the_supplier_state_the_place_of_supply_is_checked_against() {
    let units = pilot_tax_units_edited(&[
        ("08ZZZZZ0000Z1ZQ", "06ZZZZZ0000Z1ZU", 2),
        ("<STATE>Rajasthan</STATE>", "<STATE>Haryana</STATE>", 1),
    ]);
    let (seen, _, requests) = admit_on(
        pilot_lab_with_units(units),
        pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales"),
    )
    .await;
    let refusals = seen.expect_err("not refused");
    assert!(refusals
        .iter()
        .any(|(code, _)| *code == "invoice_place_of_supply_not_company_state"));
    assert_eq!(requests.len(), 70);
}

/// The same registration read as Composition at the re-read a post makes
/// before its approval window is refused under its own code, not as "a
/// master changed": the saved invoice, built when the registration was
/// Regular, meets an answer that is no longer one.
#[tokio::test]
async fn a_re_read_that_finds_the_registration_composition_is_refused_as_not_regular() {
    let lab = pilot_lab_with_units(pilot_tax_units_edited(&[(
        "<REGISTRATIONTYPE>Regular",
        "<REGISTRATIONTYPE>Composition",
        1,
    )]));
    let result = lab
        .server
        .recheck_sales_invoice(
            &lab.identity,
            &lab.company,
            &pilot_saved("BP/26-27/0010", true),
            &pilot_catalogue(),
        )
        .await;
    assert_eq!(
        result.err().map(|failure| failure.code),
        Some("invoice_company_registration_not_regular".to_string())
    );
    assert_eq!(sent(lab.simulator), 70);
}

/// A number the book already uses, on Tally's own answer: the number read for
/// `BP/26-27/0001` finds the book's first invoice (`BRIDGE Sales`, Sales
/// class), and the invoice is refused after that read, before the company's
/// tax units are read (64 requests). Both sends of the read are the request the
/// lab sent, byte for byte (`r10-number-control-k0.xml`).
#[tokio::test]
async fn a_number_the_book_already_uses_is_refused_after_the_number_read() {
    let order = ADMISSION_ORDER.replace('N', "U");
    let mut voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    voucher.voucher_number = Some("BP/26-27/0001".to_string());
    let (seen, voucher, requests) = admit_on(pilot_lab_over(&order), voucher).await;
    assert_eq!(
        seen,
        Err(vec![(
            "invoice_number_already_used",
            "BP/26-27/0001".to_string()
        )])
    );
    assert_eq!(requests.len(), 64);
    assert_eq!(sent_as(&order, &requests, 'U'), [LAB_R10_SHA256; 2]);
    assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
}

/// The re-read a post makes before its approval window, and again after it,
/// is a whole admission (70 requests when it reaches the company's tax units, 76
/// once the company object is read, which the admitted invoice does).
/// The invoice the build admitted, saved with what it observed, passes; saved
/// with an observation the re-read does not make (another type's GUID), it is
/// "a master changed"; and what refuses it comes back under its own code (a
/// type whose series is not Manual, 58 requests; a number the book uses by
/// the re-read, 64 requests), never as "a master changed".
#[tokio::test]
async fn a_re_read_is_a_whole_admission_and_its_refusal_keeps_its_own_code() {
    let mut other_type = pilot_observed();
    other_type["voucher_type_guid"] = json!(format!("{}-0000002c", pilot_lab_guid()));
    let in_use = ADMISSION_ORDER.replace('N', "U");
    for (filed, number, order, observed, outcome, requests) in [
        (
            "BRIDGE Sales",
            "BP/26-27/0010",
            ADMISSION_ORDER,
            pilot_observed(),
            None,
            76,
        ),
        (
            "BRIDGE Sales",
            "BP/26-27/0010",
            ADMISSION_ORDER,
            other_type,
            Some("import_invoice_masters_changed"),
            76,
        ),
        (
            "Sales",
            "BP/26-27/0010",
            ADMISSION_ORDER,
            pilot_observed(),
            Some("invoice_voucher_type_numbering_not_manual"),
            58,
        ),
        (
            "BRIDGE Sales",
            "BP/26-27/0001",
            in_use.as_str(),
            pilot_observed(),
            Some("invoice_number_already_used"),
            64,
        ),
    ] {
        let lab = pilot_lab_over(order);
        let mut saved = pilot_invoice("BRIDGE Walk-in", filed);
        saved.voucher_number = Some(number.to_string());
        saved.invoice.as_mut().unwrap().observed = Some(serde_json::from_value(observed).unwrap());
        let result = lab
            .server
            .recheck_sales_invoice(&lab.identity, &lab.company, &saved, &pilot_catalogue())
            .await;
        assert_eq!(
            result.err().map(|failure| failure.code),
            outcome.map(str::to_string),
            "{filed} {number}"
        );
        assert_eq!(sent(lab.simulator), requests, "{filed} {number}");
    }
}

// ---- BRIDGE PILOT LAB: the read-back ----

/// What a read-back sends, in order: the marks read (6 requests), then the
/// invoice by type and number (6). M is the marks after the post, R the
/// read-back of `BP/26-27/0010` after the post; before it, m is the marks and A
/// the same read-back (a complete envelope with no voucher).
const READBACK_ORDER: &str = "eMsMseeRsRse";
const READBACK_BEFORE_ORDER: &str = "emsmseeAsAse";
/// The read-back of `BP/26-27/0010` with the GSTR-1 status fields and each
/// line's HSN/SAC (`w8-readback-status-single.xml`, the request the code sends;
/// the lab answered it on 10 Oct 2026).
const LAB_R12_SHA256: &str = "352dfca912a53499f058643e61db4febc301f6b53b753de54d7f2aac475db9c1";

/// The posted invoice the pilot lab's book took, as the build saved it.
fn pilot_saved(number: &str, observed: bool) -> ImportVoucher {
    let mut saved = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    saved.voucher_number = Some(number.to_string());
    if observed {
        saved.invoice.as_mut().unwrap().observed =
            Some(serde_json::from_value(pilot_observed()).unwrap());
    }
    saved
}

/// A posted invoice's read-back, on the pilot lab's answers: the marks are
/// read first (6 requests), then the invoice by type and number (6). The
/// read-back's answer before the post (a captured collection with no voucher)
/// is a difference, never a pass, and so is a saved invoice with no
/// observation, which sends nothing.
#[tokio::test]
async fn a_read_back_reads_the_marks_then_the_invoice_and_an_absent_invoice_is_a_difference() {
    for (observed, difference, requests) in [
        (true, "invoice_not_found", 12),
        (false, "invoice_not_observed", 0),
    ] {
        let lab = pilot_lab_over(READBACK_BEFORE_ORDER);
        let saved = pilot_saved("BP/26-27/0010", observed);
        let (differences, alter_id, guid, lines, _) = lab
            .server
            .read_back_sales_invoice(&lab.identity, &lab.company, &saved)
            .await
            .expect("a read-back after a post is never an error");
        assert_eq!(differences, [difference]);
        assert_eq!((alter_id, guid), (None, None));
        assert!(lines.is_empty());
        let requests_sent = received(lab.simulator);
        assert_eq!(requests_sent.len(), requests, "{difference}");
        if observed {
            assert_eq!(
                sent_as(READBACK_BEFORE_ORDER, &requests_sent, 'A'),
                [LAB_R12_SHA256; 2]
            );
        }
    }
}

/// The read-back's success path, through the server and on Tally's own bytes:
/// the invoice the pilot lab's book took, written here as the build saved it,
/// against the answer Tally gave to the read-back request after the post. No
/// difference, the voucher's own alter id and guid come back, and both sends
/// of the read are the request the lab sent (`r12-invoice-readback.xml`, by
/// sha256). The same answer against an invoice of another number (one the
/// book was read not to hold on 12 Oct 2026) differs in the number and in the
/// reference the number gives.
#[tokio::test]
async fn a_read_back_of_a_posted_invoice_finds_no_difference_on_its_captured_answer() {
    for (number, expected) in [
        ("BP/26-27/0010", Vec::<&str>::new()),
        ("BP/26-27/0040", vec!["VOUCHERNUMBER", "REFERENCE"]),
    ] {
        let lab = pilot_lab_over(READBACK_ORDER);
        let (differences, alter_id, guid, _, _) = lab
            .server
            .read_back_sales_invoice(&lab.identity, &lab.company, &pilot_saved(number, true))
            .await
            .expect("a read-back after a post is never an error");
        assert_eq!(differences, expected, "{number}");
        assert_eq!(alter_id.as_deref(), Some("17"), "{number}");
        assert_eq!(
            guid,
            Some(format!("{}-0000000b", pilot_lab_guid())),
            "{number}"
        );
        let requests = received(lab.simulator);
        assert_eq!(requests.len(), 12, "{number}");
        if number == "BP/26-27/0010" {
            assert_eq!(sent_as(READBACK_ORDER, &requests, 'R'), [LAB_R12_SHA256; 2]);
        }
    }
}

// ---- BRIDGE PILOT LAB: the Sales stop ----

/// What the journal holds after the sent invoice's dispatch intent.
#[derive(Debug, Clone, Copy)]
enum Then {
    Nothing,
    Status(&'static str),
    Released,
}

/// The admission when the journal holds the pilot book's first invoice
/// (`BP/26-27/0001`, dated as the book holds it) sent, then `then` appended
/// after it, on a lab answering `order`: its outcome, and the requests sent.
async fn admission_after_a_sent_invoice(
    then: Then,
    order: &str,
) -> (
    Result<(), Vec<(&'static str, String)>>,
    Vec<tally_protocol_simulator::ObservedRequest>,
) {
    let lab = pilot_lab_over(order);
    let mut first = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    first.bridge_txn_id = "t0".to_string();
    first.voucher_number = Some("BP/26-27/0001".to_string());
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":sentinel_batch_id(1),"identity_scheme":"batch_v1",
        "company_guid":pilot_lab_guid(),"endpoint_origin":"http://127.0.0.1:9000",
        "company":null,"txn_ids":["t0"],"date_from":"20260801","date_to":"20260801",
        "sha256":"a".repeat(64),"built_at":"2026-08-01T00:00:00Z","status":"built",
        "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":7},
        "vouchers":[first]
    }))
    .unwrap();
    lab.server.append_import_ledger(&line).unwrap();
    {
        let _lock = lab.server.lock_import_admission().unwrap();
        lab.server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
                &line,
                "c".repeat(64),
                uuid::Uuid::new_v4(),
            ))
            .unwrap();
        let record = match then {
            Then::Nothing => None,
            Then::Status(status) => Some(
                serde_json::from_value::<ledger::StatusRecord>(json!({
                    "record_type":"verification_status","batch_id":line.batch_id,
                    "batch_sha256":line.sha256,"status":status
                }))
                .unwrap(),
            ),
            Then::Released => Some(ledger::StatusRecord::stop_release(
                &line,
                ledger::ReleaseSeen::NotEstablished,
            )),
        };
        if let Some(record) = record {
            lab.server
                .append_import_record_while_admitted(&record)
                .unwrap();
        }
    }
    let (outcome, _, requests) =
        admit_on(lab, pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales")).await;
    (outcome, requests)
}

/// A company with an invoice sent and not verified posted is refused before
/// any read of Tally, naming the batch. Once that batch reads verified, the
/// admission makes every read as before and one more: the verified invoice
/// is the number control, so after the number read the same read is sent for
/// it (the lab's `r10-number-control-k0.xml`, answered with the book's first
/// invoice), and the invoice is admitted. Once a person released it instead,
/// no invoice of the company stands as sent: the admission makes every read
/// as before, sends no control read, and the invoice is admitted.
#[tokio::test]
async fn an_admission_for_a_stopped_company_is_refused_before_any_read() {
    let stopped = Err(vec![("invoice_company_stopped", sentinel_batch_id(1))]);
    for then in [Then::Nothing, Then::Status("verification_incomplete")] {
        let (outcome, requests) = admission_after_a_sent_invoice(then, ADMISSION_ORDER).await;
        assert_eq!(outcome, stopped, "{then:?}");
        assert_eq!(requests.len(), 0, "{then:?}");
    }
    let (outcome, requests) = admission_after_a_sent_invoice(Then::Released, ADMISSION_ORDER).await;
    assert_eq!(outcome, Ok(()));
    assert_eq!(requests.len(), ADMISSION_ORDER.len());
    // No control read: the number read is followed by the company's tax units.
    assert_eq!(
        sent_as(ADMISSION_ORDER, &requests, 'N'),
        [LAB_R09_SHA256; 2]
    );
    assert_eq!(
        sent_as(ADMISSION_ORDER, &requests, 'C'),
        [LAB_R16_SHA256; 2]
    );
    let controlled = ADMISSION_ORDER.replace("eNsNse", "eNsNseeUsUse");
    let (outcome, requests) =
        admission_after_a_sent_invoice(Then::Status("posted_verified"), &controlled).await;
    assert_eq!(outcome, Ok(()));
    assert_eq!(requests.len(), controlled.len());
    assert_eq!(sent_as(&controlled, &requests, 'N'), [LAB_R09_SHA256; 2]);
    assert_eq!(sent_as(&controlled, &requests, 'U'), [LAB_R10_SHA256; 2]);
    assert_eq!(sent_as(&controlled, &requests, 'C'), [LAB_R16_SHA256; 2]);
}

/// The control read (`r10-number-control-k0.xml`) answered with no voucher:
/// the pilot lab's empty collection, which reads the same whatever the request
/// (`pilot-lab/PROVENANCE.md`). The build is refused before the company's
/// tax units are read, and the refusal names the verified batch that holds the
/// control invoice, the batch `verify_import` is to be run on.
#[tokio::test]
async fn a_control_the_read_does_not_find_refuses_the_build_naming_its_batch() {
    let missing = ADMISSION_ORDER.replace("eNsNse", "eNsNseeNsNse");
    let (outcome, requests) =
        admission_after_a_sent_invoice(Then::Status("posted_verified"), &missing).await;
    assert_eq!(
        outcome,
        Err(vec![(
            "invoice_number_control_missing",
            sentinel_batch_id(1)
        )])
    );
    assert_eq!(
        sent_as(&missing, &requests, 'N'),
        [
            LAB_R09_SHA256,
            LAB_R09_SHA256,
            LAB_R10_SHA256,
            LAB_R10_SHA256
        ]
    );
    assert!(sent_as(&missing, &requests, 'C').is_empty());
}

// ---- #1331: the rate read is restricted to the invoice's ledgers ----

/// The rate read of an admission names the invoice's ledgers by GUID (#1331):
/// both sends of it (the read and its repeat) are that request, and no other
/// request of the admission is.
#[tokio::test]
async fn the_rate_read_is_restricted_to_the_invoices_ledgers() {
    let (seen, _, requests) = admit("BRIDGE Walk-in", "BRIDGE Sales").await;
    assert_eq!(seen, Ok(()));
    assert_eq!(sent_as(ADMISSION_ORDER, &requests, 'Q'), [LAB_Q1_SHA256; 2]);
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.request_body_sha256 == LAB_Q1_SHA256)
            .count(),
        2
    );
}

/// An answer to the rate read that is not the scope's rows is a refusal under
/// its own code, and nothing more is read after it (the admission stops at the
/// first pair of that read). SYNTHETIC answer: the lab's own answer to the
/// unfiltered rates read of the same book (`pilot-lab-ledger-rates-whole-10oct`,
/// 15 rows, four more than the scope's 11) served for the scoped request; it
/// stands for a filter that did not hold, and is not Tally's answer to one.
#[tokio::test]
async fn a_rate_answer_that_is_not_the_scopes_rows_is_refused_and_ends_the_admission() {
    let (seen, voucher, requests) = admit_on(
        pilot_lab_replacing('Q', utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-whole-10oct.utf16le.xml"))),
        pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales"),
    )
    .await;
    assert_eq!(
        seen,
        Err(vec![(
            "invoice_ledger_rates_rows_differ",
            "ledger_scope_row_count_differs".to_string()
        )])
    );
    // Through the rate read's second send and its closing bracket: no voucher
    // type, number or registration read follows.
    assert_eq!(requests.len(), ADMISSION_ORDER.rfind('Q').unwrap() + 3);
    assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
}

/// An invoice naming a ledger the catalogue does not hold is refused before
/// the first request, because its scope cannot be planned.
#[tokio::test]
async fn an_invoice_naming_a_ledger_the_catalogue_lacks_is_refused_before_any_request() {
    let (seen, voucher, requests) = admit("BRIDGE No Such Ledger", "BRIDGE Sales").await;
    assert_eq!(
        seen,
        Err(vec![(
            "invoice_ledger_not_observed",
            "BRIDGE No Such Ledger".to_string()
        )])
    );
    assert_eq!(requests.len(), 0);
    assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
}

// ---- #1337: what the endpoint queue is handed, and when it refuses ----

fn plan_of(voucher: &ImportVoucher, control: &ledger::NumberControl) -> InvoiceReadPlan {
    plan_of_in(voucher, control, &pilot_catalogue())
}

fn plan_of_in(
    voucher: &ImportVoucher,
    control: &ledger::NumberControl,
    catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
) -> InvoiceReadPlan {
    let identity = crate::tally::VerifiedCompanyIdentity::test_fixture(PILOT, pilot_lab_guid());
    InvoiceJudge {
        identity: &identity,
        voucher,
        catalogue,
        control,
    }
    .plan(today())
    .unwrap()
}

/// The day the plans here are made on.
fn today() -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse("20261010".to_string()).unwrap()
}

fn known_control() -> ledger::NumberControl {
    ledger::NumberControl::Known {
        batch_id: "bridge-00000000-0000-4000-8000-000000000001".to_string(),
        number: "BP/26-27/0009".to_string(),
        date: "20260801".to_string(),
    }
}

/// The queue sends the plan it was handed and judges its answers as the answers
/// of this invoice's admission, so a plan that is another invoice's is a wiring
/// fault, refused before any answer is judged.
#[test]
fn a_plan_that_is_not_the_invoices_does_not_fit_it() {
    use crate::tally::approved_import::ApprovedImportAdmissionError::AdmissionInconsistent;
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let plan = plan_of(&voucher, &ledger::NumberControl::NoneStanding);
    assert_eq!(invoice::fit_plan(&plan, &plan), Ok(()));
    // Another number and another financial year are each another invoice's
    // plan; the voucher type is judged, not requested.
    let mut other_number = voucher.clone();
    other_number.voucher_number = Some("BP/26-27/0011".to_string());
    let mut other_year = voucher.clone();
    other_year.date = bridge_tally_core::TallyDate::parse("20270510".to_string()).unwrap();
    for other in [other_number, other_year] {
        let carried = plan_of(&other, &ledger::NumberControl::NoneStanding);
        assert_eq!(
            invoice::fit_plan(&plan, &carried),
            Err(AdmissionInconsistent),
            "{other:?}"
        );
    }
    // The company and the ledger count's request are each held to the invoice
    // too.
    let mut other_company = plan.clone();
    other_company.company_name = "ANOTHER LAB".to_string();
    let mut other_count = plan.clone();
    other_count.ledger_count = plan.rates.clone();
    for carried in [other_company, other_count] {
        assert_eq!(
            invoice::fit_plan(&plan, &carried),
            Err(AdmissionInconsistent),
            "{carried:?}"
        );
    }
    // The day the listing is read as of is held too, but a plan made on another
    // day is a clock that moved, not a fault: it is named as such.
    let mut later = plan.clone();
    later.listing_as_of = bridge_tally_core::TallyDate::parse("20261011".to_string()).unwrap();
    assert_eq!(
        invoice::fit_plan(&plan, &later),
        Err(
            crate::tally::approved_import::ApprovedImportAdmissionError::InvoiceMastersChanged {
                field: "listing_date"
            }
        )
    );
}

/// The journal's control moved between the plan and the queue (another invoice
/// verified, or the control released): a change, named, not a wiring fault.
#[test]
fn a_number_control_that_moved_since_the_plan_is_a_change_not_a_fault() {
    use crate::tally::approved_import::ApprovedImportAdmissionError::InvoiceMastersChanged;
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let with_control = plan_of(&voucher, &known_control());
    let without = plan_of(&voucher, &ledger::NumberControl::NoneStanding);
    assert!(with_control.number_control.is_some() && without.number_control.is_none());
    for (wanted, carried) in [(&with_control, &without), (&without, &with_control)] {
        assert_eq!(
            invoice::fit_plan(wanted, carried),
            Err(InvoiceMastersChanged {
                field: "number_control"
            })
        );
    }
}

/// The rates request names the invoice's ledgers by GUID (#1331), so a named
/// ledger regrouped, or its group renamed, between the plan's catalogue and the
/// queue's leaves the request as it was; the plan's record of each named
/// ledger's parent changes, and that is a master that changed, named as such,
/// not a wiring fault. Every other request of the two plans is equal. The
/// catalogues differ by a counted edit (DERIVED).
#[test]
fn a_ledger_regrouped_since_the_plan_is_a_change_not_a_fault() {
    use crate::tally::approved_import::ApprovedImportAdmissionError::InvoiceMastersChanged;
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let control = ledger::NumberControl::NoneStanding;
    let planned = plan_of(&voucher, &control);
    for (ledger, to) in [
        // Moved into another group.
        ("BRIDGE Walk-in", "Cash-in-Hand"),
        // Its group renamed.
        ("BRIDGE Walk-in", "Sundry Debtors (old)"),
        // A sales ledger moved to a group of its own.
        ("BRIDGE Svc 998313 5%", "Services Sold"),
        // A sales ledger moved to a group another named ledger is under.
        ("BRIDGE Svc 998313 5%", "Indirect Expenses"),
    ] {
        let later = plan_of_in(&voucher, &control, &pilot_catalogue_with_parent(ledger, to));
        assert_eq!(planned.rates, later.rates, "{ledger} under {to}");
        assert_ne!(
            planned.ledger_parents, later.ledger_parents,
            "{ledger} under {to}"
        );
        assert_eq!(planned.number, later.number);
        assert_eq!(planned.voucher_types, later.voucher_types);
        assert_eq!(planned.registration, later.registration);
        assert_eq!(planned.ledger_count, later.ledger_count);
        for (wanted, carried) in [(&later, &planned), (&planned, &later)] {
            assert_eq!(
                invoice::fit_plan(wanted, carried),
                Err(InvoiceMastersChanged {
                    field: "ledger_scope"
                }),
                "{ledger} under {to}"
            );
        }
    }
    // A ledger the invoice does not name, regrouped, changes nothing the plan
    // carries.
    let unnamed = plan_of_in(
        &voucher,
        &control,
        &pilot_catalogue_with_parent("BRIDGE Svc 998315 5%", "Indirect Expenses"),
    );
    assert_eq!(planned.rates, unnamed.rates);
    assert_eq!(planned.ledger_parents, unnamed.ledger_parents);
    assert_eq!(invoice::fit_plan(&planned, &unnamed), Ok(()));
}

/// A named ledger replaced under its name between the plan's catalogue and
/// the queue's (deleted, and another created with the same name and group):
/// its GUID, and so the rates request and the GUIDs the parents are recorded
/// under, differ, while every request the invoice alone decides and every
/// group are equal. That is a master
/// that changed, named `ledger_scope`, not a wiring fault. The catalogues
/// differ by a counted edit (DERIVED).
#[test]
fn a_ledger_replaced_under_its_name_since_the_plan_is_a_change_not_a_fault() {
    use crate::tally::approved_import::ApprovedImportAdmissionError::InvoiceMastersChanged;
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let control = ledger::NumberControl::NoneStanding;
    let planned = plan_of(&voucher, &control);
    let replaced = plan_of_in(
        &voucher,
        &control,
        &pilot_catalogue_with_guid(
            "BRIDGE Walk-in",
            "6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ff",
        ),
    );
    assert_ne!(planned.rates, replaced.rates);
    // The parent the plan records is held by GUID, so that record differs too,
    // though the groups the ledgers sit under did not.
    assert_ne!(planned.ledger_parents, replaced.ledger_parents);
    let parents = |plan: &InvoiceReadPlan| {
        plan.ledger_parents
            .iter()
            .map(|(_, parent)| parent.clone())
            .collect::<std::collections::BTreeSet<_>>()
    };
    assert_eq!(parents(&planned), parents(&replaced));
    assert_eq!(planned.company_name, replaced.company_name);
    assert_eq!(planned.number, replaced.number);
    assert_eq!(planned.voucher_types, replaced.voucher_types);
    assert_eq!(planned.registration, replaced.registration);
    assert_eq!(planned.ledger_count, replaced.ledger_count);
    for (wanted, carried) in [(&replaced, &planned), (&planned, &replaced)] {
        assert_eq!(
            invoice::fit_plan(wanted, carried),
            Err(InvoiceMastersChanged {
                field: "ledger_scope"
            })
        );
    }
}

/// An invoice and its reads go to the queue together or not at all.
#[test]
fn an_invoice_without_reads_and_reads_without_an_invoice_are_refused_unseen() {
    use crate::tally::approved_import::{
        ApprovedImportAdmissionError::AdmissionInconsistent, QueuedInvoice,
    };
    let invoice_voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let journal: ImportVoucher = serde_json::from_value(json!({
        "bridge_txn_id":"j1", "date":"20260801", "voucher_type":"Journal",
        "entries":[
            {"ledger":"BRIDGE Walk-in","amount":"10.00","side":"Dr"},
            {"ledger":"BRIDGE Svc 998313 5%","amount":"10.00","side":"Cr"}
        ]
    }))
    .unwrap();
    let control = known_control();
    let plan = plan_of(&invoice_voucher, &control);
    let answers = InvoiceAnswers::default();
    let queued = || {
        Some(QueuedInvoice {
            plan: &plan,
            answers: &answers,
        })
    };
    let vouchers = [invoice_voucher.clone()];
    let journals = [journal.clone()];
    // A batch with no invoice carries none of it.
    assert!(matches!(
        invoice::queued_invoice_for(&journals, None, None),
        Ok(None)
    ));
    // An invoice with no reads, with reads and no control, or with a control
    // and no reads.
    for (invoice, control) in [(None, None), (queued(), None), (None, Some(&control))] {
        assert_eq!(
            invoice::queued_invoice_for(&vouchers, invoice, control).err(),
            Some(AdmissionInconsistent)
        );
    }
    // Reads and a control with no invoice in the batch, or with two vouchers.
    for batch in [&journals[..], &[invoice_voucher.clone(), journal][..]] {
        assert_eq!(
            invoice::queued_invoice_for(batch, queued(), Some(&control)).err(),
            Some(AdmissionInconsistent)
        );
    }
    // The whole set is admitted.
    assert!(matches!(
        invoice::queued_invoice_for(&vouchers, queued(), Some(&control)),
        Ok(Some(_))
    ));
}

/// The judge names the answer it needs next, in the order the admission reads,
/// and the queue's marks decide whether a ledger count is one of them.
#[test]
fn the_judge_asks_for_the_marks_first_and_a_ledger_count_only_over_the_gate() {
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let catalogue = pilot_catalogue();
    let control = ledger::NumberControl::NoneStanding;
    let identity = crate::tally::VerifiedCompanyIdentity::test_fixture(PILOT, pilot_lab_guid());
    let judge = InvoiceJudge {
        identity: &identity,
        voucher: &voucher,
        catalogue: &catalogue,
        control: &control,
    };
    let needs = |answers: &InvoiceAnswers| match judge.judge(answers) {
        Err(invoice::Verdict::Need(need)) => need,
        other => panic!("{other:?}"),
    };
    let mut answers = InvoiceAnswers::default();
    assert_eq!(needs(&answers), invoice::Need::Marks);
    answers.marks = Some(pilot_marks(16, 223));
    assert_eq!(needs(&answers), invoice::Need::Listing);
    answers.marks = Some(pilot_marks(16, 5001));
    assert_eq!(needs(&answers), invoice::Need::LedgerCount);
}

/// `admit_queued_invoice` holds the plan the queue sent to the invoice's own
/// before it judges any answer (#1337). The answers here are ones the judge
/// refuses whatever plan stands beside them (the book is over the voucher mark),
/// so a plan that does not fit is refused as a plan, not by the judge, and the
/// call that checks it is pinned: without it each case below would come back as
/// the judge's refusal. It is pure, so nothing is sent.
#[test]
fn the_queue_refuses_a_plan_that_does_not_fit_before_judging_any_answer() {
    use crate::tally::approved_import::{
        ApprovedImportAdmissionError::{
            AdmissionInconsistent, InvoiceMastersChanged, InvoiceRefused,
        },
        QueuedInvoice,
    };
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let catalogue = pilot_catalogue();
    let answers = InvoiceAnswers {
        marks: Some(pilot_marks(25_001, 223)),
        ..InvoiceAnswers::default()
    };
    let identity = crate::tally::VerifiedCompanyIdentity::test_fixture(PILOT, pilot_lab_guid());
    let never = ledger::NumberControl::NoneStanding;
    let known = known_control();
    let admit = |carried: &InvoiceReadPlan, control: &ledger::NumberControl| {
        invoice::admit_queued_invoice(
            &voucher,
            &identity,
            &catalogue,
            control,
            &today(),
            &QueuedInvoice {
                plan: carried,
                answers: &answers,
            },
        )
    };
    // The plan that fits is judged: the judge refuses these answers itself.
    assert_eq!(
        admit(&plan_of(&voucher, &never), &never),
        Err(InvoiceRefused {
            code: "invoice_book_too_many_vouchers".to_string()
        })
    );
    // Another invoice's plan is a wiring fault, whatever the answers say.
    let mut other = voucher.clone();
    other.voucher_number = Some("BP/26-27/0011".to_string());
    assert_eq!(
        admit(&plan_of(&other, &never), &never),
        Err(AdmissionInconsistent)
    );
    // A plan made on another day (midnight fell between the last re-read and
    // the queue) is named by its date, not taken for a wiring fault, whatever
    // the answers say.
    let mut yesterdays = plan_of(&voucher, &never);
    yesterdays.listing_as_of = bridge_tally_core::TallyDate::parse("20261009".to_string()).unwrap();
    assert_eq!(
        admit(&yesterdays, &never),
        Err(InvoiceMastersChanged {
            field: "listing_date"
        })
    );
    // A control that moved either way since the plan is a named change.
    for (carried, now) in [(&never, &known), (&known, &never)] {
        assert_eq!(
            admit(&plan_of(&voucher, carried), now),
            Err(InvoiceMastersChanged {
                field: "number_control"
            })
        );
    }
}

/// DERIVED, not Tally's bytes: the pilot lab's captured marks (16 vouchers,
/// 223 masters) with the two counts replaced, each occurring once.
fn pilot_marks(vouchers: u64, masters: u64) -> String {
    pilot_marks_of_book(PILOT, &pilot_lab_guid(), vouchers, masters)
}

/// DERIVED, not Tally's bytes: the pilot lab's captured marks with the company's
/// name and GUID and the two counts replaced, each occurring once. The precheck
/// reads nothing else of the answer.
fn pilot_marks_of_book(name: &str, guid: &str, vouchers: u64, masters: u64) -> String {
    let mut marks = pilot_marks_captured();
    for (from, to) in [
        (
            format!("COMPANY NAME=\"{PILOT}\""),
            format!("COMPANY NAME=\"{name}\""),
        ),
        (pilot_lab_guid(), guid.to_string()),
        (
            "> 16</ALTVCHID>".to_string(),
            format!("> {vouchers}</ALTVCHID>"),
        ),
        (
            "> 223</ALTMSTID>".to_string(),
            format!("> {masters}</ALTMSTID>"),
        ),
    ] {
        assert_eq!(marks.matches(&from).count(), 1, "{from}");
        marks = marks.replace(&from, &to);
    }
    marks
}

/// The captured marks of the invoice round before the post (`m`).
fn pilot_marks_captured() -> String {
    utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-marks-before-post.utf16le.xml"))
}

/// What the queue asks before it sends any read of the invoice (#1337): the
/// currency, then the size limits, on what it has already read. Each outcome is
/// decided here, on the marks and currencies alone, so a refusal needs no read
/// of the plan; a read that cannot be used is told from a refusal by its type.
#[test]
fn the_queue_precheck_judges_the_currency_and_the_limits_before_any_read() {
    use crate::tally::approved_import::{
        ApprovedImportAdmissionError as Refusal, InvoicePrecheck, InvoicePrecheckInput,
    };
    let one_currency = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-currencies.utf16le.xml"
    ));
    let two_currencies = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
    ));
    // Tally's own count of a 4,339-ledger book (the size book's row), with the
    // names the precheck is asked about.
    let size_book_count = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/company-ledger-count.utf16le.xml"
    ));
    let check = |marks: &str, currencies: &str, count: Option<&str>, name: &str, guid: &str| {
        invoice::queued_precheck(&InvoicePrecheckInput {
            marks,
            currencies,
            ledger_count: count,
            company_name: name,
            company_guid: guid,
        })
    };
    let guid = pilot_lab_guid();
    let at_lab = |marks: &str, currencies: &str, count: Option<&str>| {
        check(marks, currencies, count, PILOT, &guid)
    };
    // Admitted: under both marks, one currency master.
    assert_eq!(
        at_lab(&pilot_marks(16, 223), &one_currency, None),
        Ok(InvoicePrecheck::Admitted)
    );
    // Over the voucher mark: refused as the build refuses it, with no count.
    assert_eq!(
        at_lab(&pilot_marks(25_001, 223), &one_currency, None),
        Err(Refusal::InvoiceRefused {
            code: "invoice_book_too_many_vouchers".to_string()
        })
    );
    // Over the master mark: the ledger count decides, so it is asked for.
    assert_eq!(
        at_lab(&pilot_marks(16, 5_001), &one_currency, None),
        Ok(InvoicePrecheck::NeedsLedgerCount)
    );
    // And with Tally's count of a book over the ledger limit, refused.
    // DERIVED: the pilot lab's marks under the size book's name and GUID.
    let size_marks = pilot_marks_of_book(
        "BRIDGE SIZE 2K",
        "f8dab51e-a5d9-49d5-9232-54a16c8b95bb",
        16,
        5_001,
    );
    assert_eq!(
        check(
            &size_marks,
            &one_currency,
            Some(&size_book_count),
            "BRIDGE SIZE 2K",
            "f8dab51e-a5d9-49d5-9232-54a16c8b95bb"
        ),
        Err(Refusal::InvoiceRefused {
            code: "invoice_book_too_large".to_string()
        })
    );
    // Two currency masters: the currency's own typed refusal, ahead of the limits.
    assert!(matches!(
        at_lab(&pilot_marks(25_001, 223), &two_currencies, None),
        Err(Refusal::MultiCurrencyBook { .. })
    ));
    // Marks that cannot be read are a read that could not be used, not a refusal.
    assert!(matches!(
        at_lab("<ENVELOPE/>", &one_currency, None),
        Err(Refusal::InvoiceReadUnusable { .. })
    ));
}

/// DERIVED, not Tally's bytes: the lab's scoped rates answer with the one `HSNCODE` of `BRIDGE Svc 998313 5%`
/// cut to its first four digits, which draws `hsn_warning_not_six_digits`.
fn pilot_rate_listing_with_a_four_digit_code() -> String {
    let listing = pilot_rate_listing();
    let from = "<HSNCODE TYPE=\"String\">998313</HSNCODE>";
    assert_eq!(listing.matches(from).count(), 1);
    listing.replace(from, "<HSNCODE TYPE=\"String\">9983</HSNCODE>")
}

/// The compliance listing the build and the queue read, as the pilot lab
/// answered it: read through the code's own reader over the lab's answers.
async fn pilot_listing() -> crate::tally::runtime::PartyLedgerMasterListing {
    let from = ADMISSION_ORDER.find("ebsbs").unwrap();
    let to = ADMISSION_ORDER.find("eQsQse").unwrap();
    let lab = pilot_lab_over(&ADMISSION_ORDER[from..to]);
    lab.server
        .runtime
        .fetch_agent_party_ledger_masters_with_evidence(
            lab.server.tally_config(),
            &lab.identity,
            today(),
            crate::tally::connection::ComplianceRead::Standard,
        )
        .await
        .unwrap()
}

/// The answers the queue would read for the pilot invoice, `rates` being its
/// scoped rates answer.
async fn pilot_queue_answers(rates: String) -> InvoiceAnswers {
    InvoiceAnswers {
        marks: Some(pilot_marks_captured()),
        listing: Some(pilot_listing().await),
        rates: Some(rates),
        voucher_types: Some(pilot_text(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-voucher-types.utf16le.xml"))),
        number: Some(pilot_text(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-new-invoice.utf16le.xml"))),
        registration: Some(pilot_tax_units()),
        company_hsn: Some(pilot_text(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-company-object.utf16le.xml"))),
        ..InvoiceAnswers::default()
    }
}

fn pilot_text(bytes: &[u8]) -> String {
    utf16(bytes)
}

/// The queue's re-read of an invoice with a recorded HSN/SAC warning
/// (#1337, #691): the warning the build recorded, read again, is admitted; a
/// warning that is gone, one that came, and one that moved are each the master
/// that changed, named `hsn_warnings` (and not a wiring fault or a refusal).
/// The answers are the lab's own except the rates answer, which is derived
/// twice (the parent filter, then one code cut to four digits for the warned
/// case).
#[tokio::test]
async fn the_queue_holds_a_warned_invoice_to_the_warning_the_build_recorded() {
    use crate::tally::approved_import::{
        ApprovedImportAdmissionError::InvoiceMastersChanged, QueuedInvoice,
    };
    let identity = crate::tally::VerifiedCompanyIdentity::test_fixture(PILOT, pilot_lab_guid());
    let catalogue = pilot_catalogue();
    let control = ledger::NumberControl::NoneStanding;
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let plan = plan_of(&voucher, &control);
    let warned = pilot_queue_answers(pilot_rate_listing_with_a_four_digit_code()).await;
    let unwarned = pilot_queue_answers(pilot_rate_listing()).await;
    // What the build records of each.
    let observed_of = |answers: &InvoiceAnswers| {
        InvoiceJudge {
            identity: &identity,
            voucher: &voucher,
            catalogue: &catalogue,
            control: &control,
        }
        .judge(answers)
        .unwrap_or_else(|verdict| panic!("{verdict:?}"))
    };
    let (recorded_warned, recorded_unwarned) = (observed_of(&warned), observed_of(&unwarned));
    assert_eq!(recorded_warned.hsn_warnings.len(), 1);
    assert_eq!(
        recorded_warned.hsn_warnings[0].ledger,
        "BRIDGE Svc 998313 5%"
    );
    assert!(recorded_unwarned.hsn_warnings.is_empty());
    let queue = |recorded: &invoice::InvoiceObserved, answers: &InvoiceAnswers| {
        let mut saved = voucher.clone();
        saved.invoice.as_mut().unwrap().observed = Some(recorded.clone());
        invoice::admit_queued_invoice(
            &saved,
            &identity,
            &catalogue,
            &control,
            &today(),
            &QueuedInvoice {
                plan: &plan,
                answers,
            },
        )
    };
    // The recorded warning, read again: admitted; so is no warning twice.
    assert_eq!(queue(&recorded_warned, &warned), Ok(()));
    assert_eq!(queue(&recorded_unwarned, &unwarned), Ok(()));
    // A warning that is gone, and one that came, are changes.
    let changed = Err(InvoiceMastersChanged {
        field: "hsn_warnings",
    });
    assert_eq!(queue(&recorded_warned, &unwarned), changed);
    assert_eq!(queue(&recorded_unwarned, &warned), changed);
    // A warning that moved (the build recorded another length) is a change too.
    let mut moved = recorded_warned.clone();
    moved.hsn_warnings[0].characters += 1;
    assert_eq!(queue(&moved, &warned), changed);
}

/// The two number reads are built only from an `InvoiceNumber`, so a number its
/// TDL literal cannot hold never becomes a request: the invoice's own number is
/// refused as `invoice_number_invalid`, and a journal control whose number is
/// not one is `invoice_number_control_unreadable`. The codes a caller sees are
/// pinned here.
#[test]
fn a_number_the_literal_cannot_hold_is_refused_before_either_number_read() {
    let catalogue = pilot_catalogue();
    let identity = crate::tally::VerifiedCompanyIdentity::test_fixture(PILOT, pilot_lab_guid());
    let none = ledger::NumberControl::NoneStanding;
    let mut voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    voucher.voucher_number = Some("A B".to_string());
    let judge = InvoiceJudge {
        identity: &identity,
        voucher: &voucher,
        catalogue: &catalogue,
        control: &none,
    };
    match judge.request(invoice::Need::Number) {
        Err(invoice::Verdict::Refused(refusals)) => {
            assert_eq!(refusals.len(), 1);
            assert_eq!(refusals[0].code, "invoice_number_invalid");
        }
        other => panic!("{other:?}"),
    }
    let ledger::NumberControl::Known { batch_id, date, .. } = known_control() else {
        panic!("the fixture control is a known one");
    };
    let control = ledger::NumberControl::Known {
        batch_id,
        number: "A\"B".to_string(),
        date,
    };
    let voucher = pilot_invoice("BRIDGE Walk-in", "BRIDGE Sales");
    let judge = InvoiceJudge {
        identity: &identity,
        voucher: &voucher,
        catalogue: &catalogue,
        control: &control,
    };
    match judge.request(invoice::Need::NumberControl) {
        Err(invoice::Verdict::Failed(code)) => {
            assert_eq!(code, "invoice_number_control_unreadable");
        }
        other => panic!("{other:?}"),
    }
}
