//! One Sales invoice admission run from its first request to its last against
//! the scripted transport.
//!
//! The company list, the marks, the book extent, the currencies, the ledger
//! compliance listing, the paired listing and the groups are Tally's own bytes
//! from one read of the disposable GST lab book (the `register-e2e` set; its
//! PROVENANCE table is beside the fixtures). The voucher types and the number
//! read's answers are Tally's own bytes from the Sales rehearsal on the same
//! book (the `sales-rehearsal` set, 7 Oct 2026). Two things are not this
//! book's bytes and say so: the company's tax units (another synthetic book's
//! capture, RE-LABELLED below) and the bill-wise catalogue row (HAND-WRITTEN). The ledger listing predates the rehearsal's two customers,
//! so no case here is admitted.
use super::*;
use crate::agent::agent_import::invoice::InvoiceJudge;
use crate::tally::approved_import::{InvoiceAnswers, InvoiceReadPlan};

const LAB_GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";
const LAB: &str = "BRIDGE GST RECON LAB";

/// What one admission sends, in order: e company list, s status probe, m
/// marks, b book extent, c currencies, L the compliance listing, P the paired
/// listing, g groups, Q the listing with each ledger's GST rate and rounding,
/// T voucher types, N the number read, C the company's tax units.
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
    "eCsCse"
);

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn company_list() -> String {
    utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-extent.utf16le.xml"))
}

/// One of the rehearsal's captured answers (the `sales-rehearsal` set).
fn rehearsal(bytes: &[u8]) -> String {
    utf16(bytes)
}

/// The book's voucher types as Tally answered on 7 Oct 2026: the predefined
/// Sales type with an Automatic series, and the type keyed for the rehearsal,
/// `Sales Manual`, with one Manual series.
fn voucher_types() -> String {
    rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-voucher-types.utf16le.xml"))
}

/// RE-LABELLED: the pilot lab's captured tax units (one Regular registration
/// in Rajasthan), every GUID moved to this book's company and the
/// registration's date moved a year back to cover this book's invoices; this
/// book's own were not captured.
fn company_registration() -> String {
    include_str!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-tax-units-typed-request.utf8.xml")
        .replace("6b43e498-430c-4d5c-bfef-d32e2ab93c85", LAB_GUID)
        .replace("<FROMDATE>20260401</FROMDATE>", "<FROMDATE>20250401</FROMDATE>")
}

fn plan(letter: char) -> ScenarioPlan {
    let body = match letter {
        's' => {
            return ScenarioPlan::new(Fixture::ProductStatus(
                tally_protocol_simulator::ProductStatus::TallyPrime,
            ))
        }
        'e' => company_list(),
        'm' => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-marks.utf16le.xml")),
        'b' => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-book-extent.utf16le.xml")),
        'c' => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-currencies.utf16le.xml")),
        'L' => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-ledgers-compliance.utf16le.xml")),
        'P' => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-ledgers-paired.utf16le.xml")),
        'g' => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-groups.utf16le.xml")),
        // HAND-WRITTEN, not Tally's bytes: this book's rates were never read, so
        // the listing is an empty collection. No case here reaches the tax
        // arithmetic (the party of each is refused first); the lab's own rate
        // listing is replayed in the pilot-lab tests.
        'Q' => "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION></COLLECTION></DATA></BODY></ENVELOPE>".to_string(),
        'T' => voucher_types(),
        'N' => rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-absent.utf16le.xml")),
        'C' => company_registration(),
        other => panic!("unknown kind {other}"),
    };
    ScenarioPlan::new(Fixture::SyntheticXml(body))
        .with_encoding(WireEncoding::Utf16LeNoBom)
        .with_framing(ResponseFraming::ContentLength)
}

/// A synthetic V2 catalogue row for each named ledger (the listing fixtures
/// carry no bill-wise flag; this one is not Tally's bytes).
fn catalogue(ledgers: &[(&str, bool)]) -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    let rows = ledgers
        .iter()
        .enumerate()
        .map(|(index, (name, bill_wise))| {
            format!(
                "<LEDGER NAME=\"{name}\" RESERVEDNAME=\"\"><GUID TYPE=\"String\">{LAB_GUID}-{index:04}</GUID>\
                 <PARENT TYPE=\"String\">Synthetic Group</PARENT><ISBILLWISEON TYPE=\"Logical\">{}</ISBILLWISEON>\
                 <BRIDGECOMPANYGUID TYPE=\"String\">{LAB_GUID}</BRIDGECOMPANYGUID>\
                 <BRIDGECOMPANYNAME TYPE=\"String\">{LAB}</BRIDGECOMPANYNAME></LEDGER>",
                if *bill_wise { "Yes" } else { "No" }
            )
        })
        .collect::<String>();
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(
        &format!("<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>{rows}</COLLECTION></DATA></BODY></ENVELOPE>"),
        LAB,
        LAB_GUID,
    )
    .unwrap()
}

/// The number read's captured answer when one Sales voucher carries the
/// number (the rehearsal's hand-keyed invoice).
fn number_in_use() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(rehearsal(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/sales-rehearsal/sales-rehearsal-number-known.utf16le.xml"))))
        .with_encoding(WireEncoding::Utf16LeNoBom)
        .with_framing(ResponseFraming::ContentLength)
}

/// A server over a simulator that answers `plans` in order, with the lab
/// company and its verified identity taken from the captured company list.
struct Lab {
    simulator: SequenceSimulator,
    server: Server,
    identity: crate::tally::VerifiedCompanyIdentity,
    company: bridge_tally_protocol::TallyCompany,
    _directory: tempfile::TempDir,
}

fn lab(mut plans: Vec<ScenarioPlan>) -> Lab {
    // One answer more than is asked for, so a request too many is counted
    // instead of refused.
    plans.push(plan('e'));
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
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let companies =
        bridge_tally_protocol::parse_companies_from_collection(&company_list()).unwrap();
    let company = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(LAB_GUID))
        .expect("the lab company")
        .clone();
    let identity = crate::tally::VerifiedCompanyIdentity::from_observed_companies(
        company.name.clone(),
        LAB_GUID.into(),
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

/// How many requests the simulator received.
fn sent(simulator: SequenceSimulator) -> usize {
    simulator.cancel();
    simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .count()
}

/// One invoice to `party` under the voucher type `filed`, as a caller sends
/// it or, with `observed`, as a build saved it for an unregistered customer.
fn invoice_to(party: &str, filed: &str, observed: bool) -> ImportVoucher {
    let mut voucher = json!({
        "bridge_txn_id":"t1", "date":"2026-03-10", "voucher_type":"Sales", "voucher_number":"TG/25-26/900",
        "invoice":{"voucher_type_name":filed,"place_of_supply":"Rajasthan"},
        "entries":[
            {"ledger":party,"amount":"11800.00","side":"Dr"},
            {"ledger":"Sales - Goods","amount":"10000.00","side":"Cr"},
            {"ledger":"Output CGST","amount":"900.00","side":"Cr"},
            {"ledger":"Output SGST","amount":"900.00","side":"Cr"}
        ]
    });
    if observed {
        voucher["invoice"]["observed"] = json!({
            "voucher_type_guid": format!("{LAB_GUID}-00000106"),
            "party_state":"Rajasthan", "party_registration_type":"Unregistered/Consumer",
            "party_bill_wise":false, "company_state":"Rajasthan"
        });
    }
    serde_json::from_value(voucher).unwrap()
}

/// The plans of one admission, with the number read answering "in use" when
/// `number_used`.
fn admission_plans(number_used: bool) -> Vec<ScenarioPlan> {
    ADMISSION_ORDER
        .chars()
        .map(|letter| match letter {
            'N' if number_used => number_in_use(),
            letter => plan(letter),
        })
        .collect()
}

/// The admission of one invoice to `party` under the voucher type `filed`,
/// and how many requests it sent. A refusal comes back as its code and the
/// ledger or value it names; a failed read as `FAILED` and its code.
async fn admit(
    party: &str,
    filed: &str,
    number_used: bool,
) -> (
    Result<(), Vec<(&'static str, String)>>,
    ImportVoucher,
    usize,
) {
    let lab = lab(admission_plans(number_used));
    let mut voucher = invoice_to(party, filed, false);
    let outcome = match lab
        .server
        .admit_sales_invoice(
            &lab.identity,
            &lab.company,
            &mut voucher,
            &catalogue(&[(party, false)]),
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
    (outcome, voucher, sent(lab.simulator))
}

/// The whole of one admission: 6 requests for the marks, 40 for the ledger
/// listing, and 6 each for the voucher types, the number read and the
/// company's tax units, in that order. Each case stops where its answer decides
/// it, which the request count shows; none is admitted, because the lab book
/// as captured holds no customer an invoice can go to (its unregistered
/// customers carry no registration entry, and its registered ones hold their
/// GSTIN in the ledger's flat field only).
#[tokio::test]
async fn an_admission_makes_every_read_in_order_and_refuses_where_its_answer_decides() {
    let unregistered = "Counter Sales - Unregistered";
    let flat_field = "Thane Retail Stores";
    for (party, filed, number_used, refusal, requests) in [
        // Every read is made, and the classification refuses the customer.
        (
            unregistered,
            "Sales Manual",
            false,
            ("invoice_party_registration_not_reported", unregistered),
            70,
        ),
        (
            flat_field,
            "Sales Manual",
            false,
            ("invoice_party_registration_type_not_reported", flat_field),
            70,
        ),
        // A number in use stops before the company's tax units are read.
        (
            unregistered,
            "Sales Manual",
            true,
            ("invoice_number_already_used", "TG/25-26/900"),
            64,
        ),
        // A type whose series is not Manual, and a type the book does not
        // have, stop before the number is read.
        (
            unregistered,
            "Sales",
            false,
            ("invoice_voucher_type_numbering_not_manual", "Sales"),
            58,
        ),
        (
            unregistered,
            "Sales Acc",
            false,
            ("invoice_voucher_type_not_found", "Sales Acc"),
            58,
        ),
    ] {
        let (outcome, voucher, sent) = admit(party, filed, number_used).await;
        assert_eq!(
            outcome,
            Err(vec![(refusal.0, refusal.1.to_string())]),
            "{party} under {filed}"
        );
        assert_eq!(sent, requests, "{party} under {filed}");
        assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
    }
}

/// The company's registration is the admission's last read: a registration
/// the build cannot issue under is a refusal under its own code, and an answer
/// with no tax unit at all is a failed read, never "no registration". Both
/// come after every other read (70 requests) and record no observation.
#[tokio::test]
async fn a_company_registration_that_cannot_be_issued_under_is_refused_and_an_unread_one_fails() {
    let units = company_registration();
    let first = units.find("<TAXUNIT NAME=").unwrap();
    let end = units.rfind("</TAXUNIT>").unwrap() + "</TAXUNIT>".len();
    for (answer, outcome) in [
        (
            units.replacen(
                "<REGISTRATIONTYPE>Regular",
                "<REGISTRATIONTYPE>Composition",
                1,
            ),
            ("invoice_company_registration_not_regular", ""),
        ),
        (
            format!("{}{}", &units[..first], &units[end..]),
            ("FAILED", "invoice_company_registration_unread"),
        ),
    ] {
        let lab = lab(ADMISSION_ORDER
            .chars()
            .map(|letter| match letter {
                'C' => ScenarioPlan::new(Fixture::SyntheticXml(answer.clone()))
                    .with_encoding(WireEncoding::Utf16LeNoBom)
                    .with_framing(ResponseFraming::ContentLength),
                letter => plan(letter),
            })
            .collect());
        let party = "Counter Sales - Unregistered";
        let mut voucher = invoice_to(party, "Sales Manual", false);
        let result = lab
            .server
            .admit_sales_invoice(
                &lab.identity,
                &lab.company,
                &mut voucher,
                &catalogue(&[(party, false)]),
            )
            .await;
        let seen = match result {
            Err(invoice::InvoiceAdmission::Refused(refusals)) => refusals
                .into_iter()
                .map(|refusal| {
                    (
                        refusal.code,
                        refusal.detail == invoice::RefusalDetail::Nothing,
                    )
                })
                .collect::<Vec<_>>(),
            Err(invoice::InvoiceAdmission::Failed(failure)) => {
                assert_eq!(failure.code, outcome.1);
                vec![("FAILED", true)]
            }
            Ok(_) => panic!("admitted"),
        };
        assert_eq!(seen, vec![(outcome.0, true)]);
        assert_eq!(sent(lab.simulator), 70);
        assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
    }
}

/// The registration's state is the supplier's state that classification
/// compares the place of supply with: a registration in Haryana (its GSTIN of
/// that state) refuses an invoice supplied in Rajasthan.
#[tokio::test]
async fn the_registrations_state_is_the_supplier_state_the_place_of_supply_is_checked_against() {
    let units = company_registration()
        .replace("08ZZZZZ0000Z1ZQ", "06ZZZZZ0000Z1ZU")
        .replacen("<STATE>Rajasthan</STATE>", "<STATE>Haryana</STATE>", 1);
    let lab = lab(ADMISSION_ORDER
        .chars()
        .map(|letter| match letter {
            'C' => ScenarioPlan::new(Fixture::SyntheticXml(units.clone()))
                .with_encoding(WireEncoding::Utf16LeNoBom)
                .with_framing(ResponseFraming::ContentLength),
            letter => plan(letter),
        })
        .collect());
    let party = "Counter Sales - Unregistered";
    let mut voucher = invoice_to(party, "Sales Manual", false);
    let Err(invoice::InvoiceAdmission::Refused(refusals)) = lab
        .server
        .admit_sales_invoice(
            &lab.identity,
            &lab.company,
            &mut voucher,
            &catalogue(&[(party, false)]),
        )
        .await
    else {
        panic!("not refused");
    };
    assert!(refusals
        .iter()
        .any(|refusal| refusal.code == "invoice_place_of_supply_not_company_state"));
    assert_eq!(sent(lab.simulator), 70);
}

/// A posted invoice's read-back, on the same captured book: the marks are read
/// first (6 requests), then the invoice by type and number (6). An answer with
/// no voucher in it (a captured empty collection) is a difference, never a
/// pass, and so is a saved invoice with no observation, which sends nothing.
#[tokio::test]
async fn a_read_back_reads_the_marks_then_the_invoice_and_an_absent_invoice_is_a_difference() {
    for (observed, difference, requests) in [
        (true, "invoice_not_found", 12),
        (false, "invoice_not_observed", 0),
    ] {
        let lab = lab("emsmseeNsNse".chars().map(plan).collect());
        let saved = invoice_to("Counter Sales - Unregistered", "Sales Manual", observed);
        let (differences, alter_id, guid, _) = lab
            .server
            .read_back_sales_invoice(&lab.identity, &lab.company, &saved)
            .await
            .expect("a read-back after a post is never an error");
        assert_eq!(differences, [difference]);
        assert_eq!((alter_id, guid), (None, None));
        assert_eq!(sent(lab.simulator), requests, "{difference}");
    }
}

/// The re-read a post makes before its approval window, and again after it,
/// is a whole admission (70 requests), and what refuses it comes back under
/// its own code: a customer whose registration no longer admits it, or a
/// number taken since the build (64 requests), is never reported as "a master
/// changed".
#[tokio::test]
async fn a_re_read_is_a_whole_admission_and_its_refusal_keeps_its_own_code() {
    let party = "Counter Sales - Unregistered";
    for (number_used, code, requests) in [
        (false, "invoice_party_registration_not_reported", 70),
        (true, "invoice_number_already_used", 64),
    ] {
        let lab = lab(admission_plans(number_used));
        let saved = invoice_to(party, "Sales Manual", true);
        let failure = lab
            .server
            .recheck_sales_invoice(
                &lab.identity,
                &lab.company,
                &saved,
                &catalogue(&[(party, false)]),
            )
            .await
            .expect_err(code);
        assert_eq!(failure.code, code);
        assert_eq!(sent(lab.simulator), requests, "{code}");
    }
}

/// What one admission does for a company whose journal holds an invoice batch
/// (sent, then `then` appended after it): its refusal code, and how many
/// requests it sent.
async fn admission_after_a_sent_invoice(then: Option<&str>) -> (Option<String>, usize) {
    let party = "Counter Sales - Unregistered";
    let lab = lab(admission_plans(false));
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000001","identity_scheme":"batch_v1",
        "company_guid":LAB_GUID,"endpoint_origin":"http://127.0.0.1:9000",
        "company":null,"txn_ids":["t0"],"date_from":"20260801","date_to":"20260801",
        "sha256":"a".repeat(64),"built_at":"2026-08-01T00:00:00Z","status":"built",
        "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":7},
        "vouchers":[{"bridge_txn_id":"t0","date":"20260801","voucher_type":"Sales",
            "voucher_number":"TG/25-26/899","narration":null,"reference":null,
            "invoice":{"voucher_type_name":"Sales Manual","place_of_supply":"Rajasthan"},
            "entries":[{"ledger":party,"amount":"118.00","side":"Dr"},
                {"ledger":"Sales - Goods","amount":"118.00","side":"Cr"}]}]
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
        if let Some(status) = then {
            lab.server
                .append_import_record_while_admitted(
                    &serde_json::from_value::<ledger::StatusRecord>(json!({
                        "record_type":"verification_status","batch_id":line.batch_id,
                        "batch_sha256":line.sha256,"status":status
                    }))
                    .unwrap(),
                )
                .unwrap();
        }
    }
    let mut voucher = invoice_to(party, "Sales Manual", false);
    let outcome = lab
        .server
        .admit_sales_invoice(
            &lab.identity,
            &lab.company,
            &mut voucher,
            &catalogue(&[(party, false)]),
        )
        .await;
    let stopped = match outcome {
        Err(invoice::InvoiceAdmission::Refused(refusals)) => refusals
            .into_iter()
            .find(|refusal| refusal.code == "invoice_company_stopped")
            .map(|refusal| match refusal.detail {
                invoice::RefusalDetail::Value(batch_id) => batch_id,
                _ => String::new(),
            }),
        _ => None,
    };
    (stopped, sent(lab.simulator))
}

/// A company with an invoice sent and not verified posted is refused before
/// any read of Tally, naming the batch; once that batch reads verified, the
/// admission goes on to every read as before.
#[tokio::test]
async fn an_admission_for_a_stopped_company_is_refused_before_any_read() {
    let batch_id = "bridge-00000000-0000-4000-8000-000000000001".to_string();
    assert_eq!(
        admission_after_a_sent_invoice(None).await,
        (Some(batch_id.clone()), 0)
    );
    assert_eq!(
        admission_after_a_sent_invoice(Some("verification_incomplete")).await,
        (Some(batch_id), 0)
    );
    assert_eq!(
        admission_after_a_sent_invoice(Some("posted_verified")).await,
        (None, 70)
    );
}

// ---- #1337: what the endpoint queue is handed, and when it refuses ----

fn plan_of(voucher: &ImportVoucher, control: &ledger::NumberControl) -> InvoiceReadPlan {
    let catalogue = catalogue(&[("Customer", true)]);
    InvoiceJudge {
        company_name: LAB,
        company_guid: LAB_GUID,
        voucher,
        catalogue: &catalogue,
        control,
    }
    .plan(
        bridge_tally_core::TallyDate::parse("20261010".to_string()).unwrap(),
        false,
    )
    .unwrap()
}

fn known_control() -> ledger::NumberControl {
    ledger::NumberControl::Known {
        batch_id: "bridge-00000000-0000-4000-8000-000000000001".to_string(),
        number: "TG/25-26/899".to_string(),
        date: "20260309".to_string(),
    }
}

/// The queue sends the plan it was handed and judges its answers as the answers
/// of this invoice's admission, so a plan that is another invoice's is a wiring
/// fault, refused before any answer is judged.
#[test]
fn a_plan_that_is_not_the_invoices_does_not_fit_it() {
    use crate::tally::approved_import::ApprovedImportAdmissionError::AdmissionInconsistent;
    let voucher = invoice_to("Customer", "Sales Manual", true);
    let plan = plan_of(&voucher, &ledger::NumberControl::NeverSent);
    assert_eq!(invoice::fit_plan(&plan, &plan), Ok(()));
    // Another number and another financial year are each another invoice's
    // plan; the voucher type is judged, not requested, and the date a listing
    // is read as of is the clock's.
    let mut other_number = voucher.clone();
    other_number.voucher_number = Some("TG/25-26/901".to_string());
    let mut other_year = voucher.clone();
    other_year.date = "2027-05-10".to_string();
    for other in [other_number, other_year] {
        let carried = plan_of(&other, &ledger::NumberControl::NeverSent);
        assert_eq!(
            invoice::fit_plan(&plan, &carried),
            Err(AdmissionInconsistent),
            "{other:?}"
        );
    }
    let mut later = plan.clone();
    later.listing_as_of = bridge_tally_core::TallyDate::parse("20261011".to_string()).unwrap();
    assert_eq!(invoice::fit_plan(&plan, &later), Ok(()));
}

/// The journal's control moved between the plan and the queue (another invoice
/// verified, or the control released): a change, named, not a wiring fault.
#[test]
fn a_number_control_that_moved_since_the_plan_is_a_change_not_a_fault() {
    use crate::tally::approved_import::ApprovedImportAdmissionError::InvoiceMastersChanged;
    let voucher = invoice_to("Customer", "Sales Manual", true);
    let with_control = plan_of(&voucher, &known_control());
    let without = plan_of(&voucher, &ledger::NumberControl::NeverSent);
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

/// An invoice and its reads go to the queue together or not at all.
#[test]
fn an_invoice_without_reads_and_reads_without_an_invoice_are_refused_unseen() {
    use crate::tally::approved_import::{
        ApprovedImportAdmissionError::AdmissionInconsistent, QueuedInvoice,
    };
    let invoice_voucher = invoice_to("Customer", "Sales Manual", true);
    let journal: ImportVoucher = serde_json::from_value(json!({
        "bridge_txn_id":"j1", "date":"2026-03-10", "voucher_type":"Journal",
        "entries":[
            {"ledger":"Customer","amount":"10.00","side":"Dr"},
            {"ledger":"Sales - Goods","amount":"10.00","side":"Cr"}
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
    let voucher = invoice_to("Customer", "Sales Manual", true);
    let catalogue = catalogue(&[("Customer", true)]);
    let control = ledger::NumberControl::NeverSent;
    let judge = InvoiceJudge {
        company_name: LAB,
        company_guid: LAB_GUID,
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
    let marks = |masters: u64| {
        format!(
            "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
             <COMPANY NAME=\"{LAB}\"><GUID TYPE=\"String\">{LAB_GUID}</GUID>\
             <ALTVCHID TYPE=\"Number\"> 16</ALTVCHID><ALTMSTID TYPE=\"Number\"> {masters}</ALTMSTID>\
             </COMPANY></COLLECTION></DATA></BODY></ENVELOPE>"
        )
    };
    answers.marks = Some(marks(223));
    assert_eq!(needs(&answers), invoice::Need::Listing);
    answers.marks = Some(marks(5001));
    assert_eq!(needs(&answers), invoice::Need::LedgerCount);
}
