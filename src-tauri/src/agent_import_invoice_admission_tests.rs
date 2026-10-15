//! One Sales invoice admission, and one read-back, run from the first request
//! to the last against the scripted transport, on one book's answers only:
//! BRIDGE PILOT LAB (the `pilot-lab` set, captured on 9 Oct 2026; its
//! PROVENANCE table is beside the fixtures). Every answer is Tally's own bytes.
//!
//! - The admission: the company list, the marks, the book extent, the
//!   currencies, the ledger compliance listing, the paired listing, the
//!   groups, the voucher types, the number read, the company's state and the
//!   ledger catalogue the party's bill-wise flag comes from, from the third
//!   round and the invoice round before the post (marks 16 and 223 in both).
//!   So is the number read for a number the book uses, from the same day's
//!   second round (marks 15 and 223: between the rounds one voucher of
//!   another number was keyed, and the voucher it finds reads the same
//!   alteration id in both); the lab sent it as the number control's read,
//!   whose request is the same bytes.
//! - The read-back: the invoice round's marks after the post (17 and 223) and
//!   its read-back of the posted invoice, before and after the post.
use super::*;

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
        writes_enabled: false,
        batch_post_enabled: false,
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
/// listing, g groups, T voucher types, N the number read, C the company state;
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
    "eTsTse",
    "eNsNse",
    "eCsCse"
);

/// The lab's request files the tests pin a read to, by sha256: the number
/// read for `BP/26-27/0010` (`r09-number-new-invoice.xml`), the same read for
/// `BP/26-27/0001` (`r10-number-control-k0.xml`) and the company's state
/// (`r11-company-state.xml`).
const LAB_R09_SHA256: &str = "aa6e7a3ae50bd13d6eff60ffac89bd713f084a49512667edcc6d40fee0ca3985";
const LAB_R10_SHA256: &str = "90d188e7089852c411516ef36617dfc3a157df76cfc1787b1985804833ff31e6";
const LAB_R11_SHA256: &str = "40f91b3fdb131279b413910c60f86a3b56989d8646def9bbe6b1fbdc97b6a68b";

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
        'T' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-voucher-types.utf16le.xml"),
        'N' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-new-invoice.utf16le.xml"),
        'C' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-company-state.utf16le.xml"),
        'M' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-marks-after-post.utf16le.xml"),
        'R' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-after-post.utf16le.xml"),
        'A' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-readback-before-post.utf16le.xml"),
        'U' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-number-in-use.utf16le.xml"),
        other => panic!("unknown kind {other}"),
    };
    answer(utf16(bytes))
}

/// The ledger catalogue the build reads, as the pilot lab answered it: the
/// party's bill-wise flag comes from here.
fn pilot_catalogue() -> bridge_tally_protocol::StandardLedgerCatalogV2 {
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(
        &utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-catalogue-v2.utf16le.xml")),
        PILOT,
        &pilot_lab_guid(),
    )
    .unwrap()
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
        "party_bill_wise":false, "company_state":"Rajasthan"
    })
}

/// The whole of one admission: 6 requests for the marks, 40 for the ledger
/// listing, and 6 each for the voucher types, the number read and the
/// company's state, in that order. Each case stops where its answer decides
/// it, which the request count shows. The invoice the pilot lab's book took,
/// to its walk-in customer under `BRIDGE Sales`, is admitted after every read
/// and records what it observed; a ledger of the book that is no customer,
/// named as the party, is refused by the classification after every read.
#[tokio::test]
async fn an_admission_makes_every_read_in_order_and_refuses_where_its_answer_decides() {
    let walk_in = "BRIDGE Walk-in";
    let goods = "BRIDGE Goods 18%";
    for (party, filed, outcome, requests) in [
        (walk_in, "BRIDGE Sales", Ok(()), 64),
        (
            goods,
            "BRIDGE Sales",
            Err(vec![
                ("invoice_sales_must_be_credit", goods),
                ("invoice_party_missing", ""),
            ]),
            64,
        ),
        // A type whose series is not Manual, and a type the book does not
        // have, stop before the number is read.
        (
            walk_in,
            "Sales",
            Err(vec![("invoice_voucher_type_numbering_not_manual", "Sales")]),
            52,
        ),
        (
            walk_in,
            "Sales Acc",
            Err(vec![("invoice_voucher_type_not_found", "Sales Acc")]),
            52,
        ),
    ] {
        let (seen, voucher, received) = admit(party, filed).await;
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
            // Both sends of the number read and of the company's state read
            // are the requests the lab sent, byte for byte.
            assert_eq!(
                sent_as(ADMISSION_ORDER, &received, 'N'),
                [LAB_R09_SHA256; 2]
            );
            assert_eq!(
                sent_as(ADMISSION_ORDER, &received, 'C'),
                [LAB_R11_SHA256; 2]
            );
        } else {
            assert!(observed.is_none(), "{party} under {filed}");
        }
    }
}

/// A number the book already uses, on Tally's own answer: the number read for
/// `BP/26-27/0001` finds the book's first invoice (`BRIDGE Sales`, Sales
/// class), and the invoice is refused after that read, before the company's
/// state is read (58 requests). Both sends of the read are the request the
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
    assert_eq!(requests.len(), 58);
    assert_eq!(sent_as(&order, &requests, 'U'), [LAB_R10_SHA256; 2]);
    assert!(voucher.invoice.as_ref().unwrap().observed.is_none());
}

/// The re-read a post makes before its approval window, and again after it,
/// is a whole admission (64 requests when it reaches the company's state).
/// The invoice the build admitted, saved with what it observed, passes; saved
/// with an observation the re-read does not make (another type's GUID), it is
/// "a master changed"; and what refuses it comes back under its own code (a
/// type whose series is not Manual, 52 requests; a number the book uses by
/// the re-read, 58 requests), never as "a master changed".
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
            64,
        ),
        (
            "BRIDGE Sales",
            "BP/26-27/0010",
            ADMISSION_ORDER,
            other_type,
            Some("import_invoice_masters_changed"),
            64,
        ),
        (
            "Sales",
            "BP/26-27/0010",
            ADMISSION_ORDER,
            pilot_observed(),
            Some("invoice_voucher_type_numbering_not_manual"),
            52,
        ),
        (
            "BRIDGE Sales",
            "BP/26-27/0001",
            in_use.as_str(),
            pilot_observed(),
            Some("invoice_number_already_used"),
            58,
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
const LAB_R12_SHA256: &str = "dc03b35f075e638acf6ca0a1a8503311ee501c5ee6f739e31d4240c4d22bf5f7";

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
        let (differences, alter_id, guid, _) = lab
            .server
            .read_back_sales_invoice(&lab.identity, &lab.company, &saved)
            .await
            .expect("a read-back after a post is never an error");
        assert_eq!(differences, [difference]);
        assert_eq!((alter_id, guid), (None, None));
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
        let (differences, alter_id, guid, _) = lab
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
        "batch_id":"bridge-00000000-0000-4000-8000-000000000001","identity_scheme":"batch_v1",
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
    let stopped = Err(vec![(
        "invoice_company_stopped",
        "bridge-00000000-0000-4000-8000-000000000001".to_string(),
    )]);
    for then in [Then::Nothing, Then::Status("verification_incomplete")] {
        let (outcome, requests) = admission_after_a_sent_invoice(then, ADMISSION_ORDER).await;
        assert_eq!(outcome, stopped, "{then:?}");
        assert_eq!(requests.len(), 0, "{then:?}");
    }
    let (outcome, requests) = admission_after_a_sent_invoice(Then::Released, ADMISSION_ORDER).await;
    assert_eq!(outcome, Ok(()));
    assert_eq!(requests.len(), ADMISSION_ORDER.len());
    // No control read: the number read is followed by the company's state.
    assert_eq!(
        sent_as(ADMISSION_ORDER, &requests, 'N'),
        [LAB_R09_SHA256; 2]
    );
    assert_eq!(
        sent_as(ADMISSION_ORDER, &requests, 'C'),
        [LAB_R11_SHA256; 2]
    );
    let controlled = ADMISSION_ORDER.replace("eNsNse", "eNsNseeUsUse");
    let (outcome, requests) =
        admission_after_a_sent_invoice(Then::Status("posted_verified"), &controlled).await;
    assert_eq!(outcome, Ok(()));
    assert_eq!(requests.len(), controlled.len());
    assert_eq!(sent_as(&controlled, &requests, 'N'), [LAB_R09_SHA256; 2]);
    assert_eq!(sent_as(&controlled, &requests, 'U'), [LAB_R10_SHA256; 2]);
    assert_eq!(sent_as(&controlled, &requests, 'C'), [LAB_R11_SHA256; 2]);
}
