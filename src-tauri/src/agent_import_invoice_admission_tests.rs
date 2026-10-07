//! One Sales invoice admission run from its first request to its last against
//! the scripted transport.
//!
//! The company list, the marks, the book extent, the currencies, the ledger
//! compliance listing, the paired listing and the groups are Tally's own bytes
//! from one read of the disposable GST lab book (the `register-e2e` set; its
//! PROVENANCE table is beside the fixtures). The empty collection is a captured
//! one. The two answers only an invoice asks for and no capture is committed
//! for, the voucher types and the company's state, are HAND-WRITTEN below:
//! regression doubles that prove the order and the plumbing of the reads, not
//! what Tally answers. The rehearsal's captures replace them.
use super::*;

const LAB_GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";
const LAB: &str = "BRIDGE GST RECON LAB";

/// What one admission sends, in order: e company list, s status probe, m
/// marks, b book extent, c currencies, L the compliance listing, P the paired
/// listing, g groups, T voucher types, N the number read, C the company state.
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

/// HAND-WRITTEN: the predefined Sales type with an Automatic series, and a
/// user type under it with one Manual series (the shape of the lab's answer).
fn voucher_types() -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO><VOUCHERTYPE>0</VOUCHERTYPE></CMPINFO></DESC><DATA><COLLECTION ISMODIFY=\"No\">\
         <VOUCHERTYPE NAME=\"Sales\" RESERVEDNAME=\"Sales\"><GUID>{LAB_GUID}-0000002c</GUID><PARENT>Sales</PARENT>\
         <VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
         <VOUCHERTYPE NAME=\"Sales Manual\" RESERVEDNAME=\"\"><GUID>{LAB_GUID}-00000131</GUID><PARENT>Sales</PARENT>\
         <VOUCHERNUMBERSERIES.LIST><NAME>Default</NAME><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD></VOUCHERNUMBERSERIES.LIST></VOUCHERTYPE>\
         </COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

/// HAND-WRITTEN: one row for the company, chosen by GUID, with its state.
fn company_state() -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <COMPANY NAME=\"{LAB}\"><GUID>{LAB_GUID}</GUID><STATENAME>Rajasthan</STATENAME></COMPANY>\
         </COLLECTION></DATA></BODY></ENVELOPE>"
    )
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
        'T' => voucher_types(),
        'N' => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/native-empty-collection.utf16le.xml")),
        'C' => company_state(),
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

/// HAND-WRITTEN: the number read's answer when one Sales voucher carries the
/// number.
fn number_in_use() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <VOUCHER><VOUCHERNUMBER>TG/25-26/900</VOUCHERNUMBER><BRIDGEVCHISSALES>Yes</BRIDGEVCHISSALES></VOUCHER>\
         </COLLECTION></DATA></BODY></ENVELOPE>"
            .to_string(),
    ))
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
            "voucher_type_guid": format!("{LAB_GUID}-00000131"),
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
/// company's state, in that order. Each case stops where its answer decides
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
            64,
        ),
        (
            flat_field,
            "Sales Manual",
            false,
            ("invoice_party_registration_type_not_reported", flat_field),
            64,
        ),
        // A number in use stops before the company's state is read.
        (
            unregistered,
            "Sales Manual",
            true,
            ("invoice_number_already_used", "TG/25-26/900"),
            58,
        ),
        // A type whose series is not Manual, and a type the book does not
        // have, stop before the number is read.
        (
            unregistered,
            "Sales",
            false,
            ("invoice_voucher_type_numbering_not_manual", "Sales"),
            52,
        ),
        (
            unregistered,
            "Sales Acc",
            false,
            ("invoice_voucher_type_not_found", "Sales Acc"),
            52,
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
/// is a whole admission (64 requests), and what refuses it comes back under
/// its own code: a customer whose registration no longer admits it, or a
/// number taken since the build (58 requests), is never reported as "a master
/// changed".
#[tokio::test]
async fn a_re_read_is_a_whole_admission_and_its_refusal_keeps_its_own_code() {
    let party = "Counter Sales - Unregistered";
    for (number_used, code, requests) in [
        (false, "invoice_party_registration_not_reported", 64),
        (true, "invoice_number_already_used", 58),
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
