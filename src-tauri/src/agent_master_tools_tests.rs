//! The two master tools, driven through `Server::call_tool` against the
//! protocol simulator, the approval answered by the test-only seam. No real
//! Tally is involved.
//!
//! The ledger answer here is an empty envelope, which the ledger-name parser
//! refuses (`master_ledger_index_unavailable`): the committed V3 captures are of
//! another company than the group and company captures these tests read. A test
//! that must get past it scopes the ledgers the book holds into
//! `TEST_LEDGER_INDEX`; the group answer is a committed live capture. The queue's request sequence is the one
//! `agent_master_post_tests::queue` documents.
use super::super::master_ledger::{read_pre_image, MasterVerdictState};
use super::super::master_post::{TEST_LEDGER_INDEX, TEST_LEDGER_INDEX_AFTER_POST};
use super::*;
use crate::agent::master_writes_effective;
use crate::agent::{Redaction, Settings};
use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};
use crate::tally::approved_import::{ApprovedImport, DispatchLicense};
use crate::tally::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::{IndexedMaster, MasterKind};
use bridge_tally_transport::TallyEndpointConfig;
use std::time::Duration;
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator,
    WireEncoding,
};

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const COMPANY: &str = "WR2 Unicode Lab";
const LEDGER: &str = "Zeta Test Ledger";
const LAB_GSTIN: &str = "08ZZZZZ0000Z2ZP";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn companies() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ))
}

fn groups() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-groups.utf16le.xml"
    ))
}

/// Tally's answer to a create: the captured live import response, whose
/// counters all parse.
fn created_one() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/live_education_w4_voucher_sanitized.xml"
    )
    .to_string()
}

fn company_marks(target_masters: u64) -> String {
    company_marks_of(target_masters, 3)
}

fn company_marks_of(target_masters: u64, other_masters: u64) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <COMPANY NAME=\"{COMPANY}\" RESERVEDNAME=\"\"><GUID>{GUID}</GUID><ALTVCHID>10</ALTVCHID><ALTMSTID>{target_masters}</ALTMSTID></COMPANY>\
         <COMPANY NAME=\"Synthetic Other Lab\" RESERVEDNAME=\"\"><GUID>22222222-2222-4222-8222-222222222222</GUID><ALTVCHID>50</ALTVCHID><ALTMSTID>{other_masters}</ALTMSTID></COMPANY>\
         </COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

const NEW_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f-000000d1";
const CASH_GUID: &str = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa";

/// Values a party-ledger master row carries that no answer or journal record
/// may ever hold. Synthetic, none of them a real person's.
const PRIVATE_VALUES: [&str; 8] = [
    "SYNTHPAN01",
    "Synthetic Pan Holder",
    "Synthetic Bank Holder",
    "000123456789",
    "ZZZZ0000001",
    "private.person@example.invalid",
    "9999900000",
    "1 Synthetic Private Road",
];

/// The party-ledger master answer for the parent group of the new ledger: the
/// one row, with every sensitive field filled.
fn party_answer(parent: &str, tax: &str, opening: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <LEDGER NAME=\"{LEDGER}\"><GUID>{NEW_GUID}</GUID><BRIDGECOMPANYGUID>{GUID}</BRIDGECOMPANYGUID>\
         <MASTERID>209</MASTERID><ALTERID>12</ALTERID><PARENT>{parent}</PARENT>\
         <OPENINGBALANCE>{opening}</OPENINGBALANCE>{tax}\
         <INCOMETAXNUMBER>{}</INCOMETAXNUMBER><NAMEONPAN>{}</NAMEONPAN>\
         <BANKACCHOLDERNAME>{}</BANKACCHOLDERNAME><BANKDETAILS>{}</BANKDETAILS><IFSCODE>{}</IFSCODE>\
         <EMAIL>{}</EMAIL><LEDGERPHONE>{}</LEDGERPHONE>\
         <LEDADDRESS.LIST><LEDADDRESS>{}</LEDADDRESS></LEDADDRESS.LIST>\
         </LEDGER></COLLECTION></DATA></BODY></ENVELOPE>",
        PRIVATE_VALUES[0],
        PRIVATE_VALUES[1],
        PRIVATE_VALUES[2],
        PRIVATE_VALUES[3],
        PRIVATE_VALUES[4],
        PRIVATE_VALUES[5],
        PRIVATE_VALUES[6],
        PRIVATE_VALUES[7],
    )
}

fn suspense_answer() -> String {
    party_answer("Suspense A/c", "", "0.00")
}

/// The ledgers the book holds once the create landed: the ones before it and
/// the new one, with a GUID the pre-image did not hold.
fn book_after() -> Vec<IndexedMaster> {
    let mut book = book();
    book.push(IndexedMaster {
        kind: MasterKind::Ledger,
        guid: NEW_GUID.into(),
        alter_id: 12,
        names: vec![LEDGER.into()],
    });
    book
}

/// What the read-back sends after the POST answered: the marks, the paired
/// ledger and group indexes and, when the index shows a new ledger of the
/// name, the paired party-ledger master (19 requests, or 13 without it).
fn read_back(target: u64, other: u64, party: Option<String>) -> Vec<ScenarioPlan> {
    let mut plans = vec![xml(company_marks_of(target, other))];
    plans.extend(paired("<ENVELOPE/>".to_string()));
    plans.extend(paired(groups()));
    if let Some(party) = party {
        plans.extend(paired(party));
    }
    plans
}

/// A reconcile's reads: the identity, then the read-back.
fn reconcile_reads(target: u64, other: u64, party: Option<String>) -> Vec<ScenarioPlan> {
    let mut plans = verified_company();
    plans.extend(read_back(target, other, party));
    plans
}

fn xml(body: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(body))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
        .with_framing(ResponseFraming::ContentLength)
}

/// One identity-bracketed paired read of `body`.
fn paired(body: String) -> Vec<ScenarioPlan> {
    vec![
        xml(companies()),
        xml(body.clone()),
        status(),
        xml(body),
        status(),
        xml(companies()),
    ]
}

/// The company's verified identity.
fn verified_company() -> Vec<ScenarioPlan> {
    vec![xml(companies()), status(), xml(companies()), status()]
}

/// Every read a master tool makes before it asks anyone: the identity, the
/// ledger name index and the group name index, one paired read each (16).
fn before_approval() -> Vec<ScenarioPlan> {
    let mut plans = verified_company();
    plans.extend(paired("<ENVELOPE/>".to_string()));
    plans.extend(paired(groups()));
    plans
}

/// What the queue sends after approval, up to and including the POST: the
/// opening admission and the binding marks, the two index pairs twice over, the
/// closing admission and the aim marks (32 reads), then the one POST.
fn after_approval(post: ScenarioPlan) -> Vec<ScenarioPlan> {
    let mut plans = vec![
        status(),
        xml(companies()),
        xml(companies()),
        xml(company_marks(7)),
    ];
    for body in [
        "<ENVELOPE/>".to_string(),
        "<ENVELOPE/>".to_string(),
        groups(),
        groups(),
    ] {
        plans.extend(paired(body));
    }
    plans.extend([
        status(),
        xml(companies()),
        xml(companies()),
        xml(company_marks(7)),
        post,
    ]);
    plans
}

fn with_sentinel(mut plans: Vec<ScenarioPlan>) -> Vec<ScenarioPlan> {
    plans.push(status());
    plans
}

fn sent(simulator: SequenceSimulator) -> Vec<tally_protocol_simulator::ObservedRequest> {
    simulator.cancel();
    simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .collect()
}

fn server_with(
    address: std::net::SocketAddr,
    directory: &std::path::Path,
    master_writes_enabled: bool,
) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: address.ip().to_string(),
            port: address.port(),
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
        master_writes_enabled,
    })
}

fn server_at(address: std::net::SocketAddr, directory: &std::path::Path) -> Server {
    server_with(address, directory, true)
}

fn identity() -> VerifiedCompanyIdentity {
    let xml = companies();
    let observed = bridge_tally_protocol::parse_companies_from_collection(&xml).unwrap();
    let company = observed
        .iter()
        .find(|company| company.guid.as_deref() == Some(GUID))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        company.name.clone(),
        crate::tally::test_company_guid(GUID),
        company.company_number.clone().unwrap(),
        company.books_from.clone().unwrap(),
        &observed,
    )
    .unwrap()
}

fn item(name: &str) -> LedgerCreateItem {
    LedgerCreateItem::new(MasterName::new(name).unwrap(), AdmittedLedgerKind::Suspense)
}

/// The ledgers the book holds, standing in for a captured answer of this
/// company.
fn book() -> Vec<IndexedMaster> {
    vec![IndexedMaster {
        kind: MasterKind::Ledger,
        guid: "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".into(),
        alter_id: 9,
        names: vec!["Cash".into()],
    }]
}

fn prepare_args(name: &str) -> Value {
    json!({"company_guid": GUID, "ledger": {"name": name, "kind": "suspense"}})
}

fn result(response: &Value) -> &Value {
    &response["structuredContent"]["result"]
}

fn error_code(response: &Value) -> &str {
    result(response)["error"]["code"]
        .as_str()
        .unwrap_or_else(|| panic!("no error code: {response}"))
}

fn journal_path(directory: &std::path::Path) -> std::path::PathBuf {
    directory.join("agent-master-ledger.jsonl")
}

fn journal_bytes(directory: &std::path::Path) -> Vec<u8> {
    std::fs::read(journal_path(directory)).unwrap_or_default()
}

fn record_types(directory: &std::path::Path) -> Vec<String> {
    String::from_utf8(journal_bytes(directory))
        .unwrap()
        .lines()
        .map(|line| {
            serde_json::from_str::<Value>(line).unwrap()["record_type"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

fn state_of(directory: &std::path::Path, batch_id: &str) -> BatchState {
    read_journal(directory)
        .unwrap()
        .state(batch_id)
        .cloned()
        .expect("the batch is in the journal")
}

/// A prepared, never-sent batch exactly as `prepare_ledger_creates` leaves one,
/// without its reads.
fn saved_batch(server: &Server, name: &str) -> (MasterBatch, Value) {
    let identity = identity();
    let item = item(name);
    let request = MasterPostRequest::render(item.clone(), &identity);
    let batch = MasterBatch {
        batch_id: format!("bridge-{}", Uuid::new_v4()),
        batch_sha256: request.sha256().to_string(),
        class: MasterClass::LedgerCreate,
        company: MasterCompany {
            guid: identity.company_guid().to_string(),
            name: identity.display_name().to_string(),
            number: identity.company_number().to_string(),
            books_from: identity.books_from_yyyymmdd().to_string(),
        },
        items: vec![stored_from_item(&item)],
    };
    let lock = ExclusiveAdmission::acquire(&server.settings.data_dir).unwrap();
    append(
        &server.settings.data_dir,
        &lock,
        MasterRecord::Batch(batch.clone()),
    )
    .unwrap();
    let args = json!({"batch_id": batch.batch_id});
    (batch, args)
}

/// Run `call` with the book holding `book()` and the approval scripted. The
/// read-back after a POST sees the same book: the ledger is not there.
async fn posting<F: std::future::Future>(scripted: &ScriptedApproval, call: F) -> F::Output {
    SCRIPTED_APPROVAL
        .scope(scripted.clone(), TEST_LEDGER_INDEX.scope(book(), call))
        .await
}

/// As [`posting`], the book holding `after` once the POST is done.
async fn posting_then<F: std::future::Future>(
    scripted: &ScriptedApproval,
    after: Vec<IndexedMaster>,
    call: F,
) -> F::Output {
    SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            TEST_LEDGER_INDEX.scope(book(), TEST_LEDGER_INDEX_AFTER_POST.scope(after, call)),
        )
        .await
}

/// A `call` with no approval to give: a reconcile, whose book is `after`.
async fn reconciling<F: std::future::Future>(after: Vec<IndexedMaster>, call: F) -> F::Output {
    posting_then(&ScriptedApproval::approving(), after, call).await
}

/// Whether any observed request carried the create's bytes.
fn create_was_sent(
    observed: &[tally_protocol_simulator::ObservedRequest],
    request: &MasterPostRequest,
) -> bool {
    let sha = sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        request.xml(),
    ));
    observed.iter().any(|seen| seen.request_body_sha256 == sha)
}

fn assert_no_private_value(label: &str, text: &str) {
    for private in PRIVATE_VALUES {
        assert!(!text.contains(private), "{label} holds {private}");
    }
}

// --- the setting -----------------------------------------------------------

#[test]
fn the_master_writes_setting_is_off_unset_and_on_for_true_and_one() {
    use std::env::VarError::NotPresent;
    assert_eq!(master_writes_effective(Err(NotPresent), true), Ok(false));
    for on in ["1", "true"] {
        assert_eq!(
            master_writes_effective(Ok(on.into()), true),
            Ok(true),
            "{on}"
        );
    }
    for off in ["0", "false"] {
        assert_eq!(
            master_writes_effective(Ok(off.into()), true),
            Ok(false),
            "{off}"
        );
    }
}

#[test]
fn the_master_writes_setting_is_validated_even_with_posting_off() {
    use std::env::VarError::NotPresent;
    for writes in [false, true] {
        for bad in ["yes", "TRUE", "", "2", " true"] {
            assert_eq!(
                master_writes_effective(Ok(bad.into()), writes),
                Err("boolean_setting_invalid".to_string()),
                "{bad:?} writes={writes}"
            );
        }
    }
    assert_eq!(
        master_writes_effective(Err(std::env::VarError::NotUnicode("x".into())), true),
        Err("boolean_setting_invalid".to_string())
    );
    let _ = NotPresent;
}

#[test]
fn master_writes_on_without_posting_is_ineffective() {
    for on in ["1", "true"] {
        assert_eq!(master_writes_effective(Ok(on.into()), false), Ok(false));
    }
}

// --- listing and refusal when off ------------------------------------------

#[test]
fn the_master_tools_are_listed_only_when_master_writes_are_in_force() {
    let names = |definitions: Value| {
        definitions
            .as_array()
            .unwrap()
            .iter()
            .map(|tool| tool["name"].as_str().unwrap().to_string())
            .collect::<Vec<_>>()
    };
    let master = |names: &[String]| {
        MASTER_WRITE_TOOLS_FOR_TEST
            .iter()
            .filter(|name| names.iter().any(|listed| listed == *name))
            .count()
    };
    for (import, writes) in [(false, false), (true, false), (false, true), (true, true)] {
        let off = names(super::super::super::catalog::tool_definitions_with_masters(
            import, writes, false,
        ));
        assert_eq!(master(&off), 0, "import={import} writes={writes}");
        let on = names(super::super::super::catalog::tool_definitions_with_masters(
            import, writes, true,
        ));
        // Master writes need posting: with it off the setting lists nothing.
        assert_eq!(
            master(&on),
            if writes { 2 } else { 0 },
            "import={import} writes={writes}"
        );
    }
    // The default listing is the master-off listing.
    assert_eq!(
        names(super::super::super::catalog::tool_definitions(true, true)),
        names(super::super::super::catalog::tool_definitions_with_masters(
            true, true, false
        ))
    );
}

const MASTER_WRITE_TOOLS_FOR_TEST: [&str; 2] = ["post_ledger_creates", "prepare_ledger_creates"];

#[tokio::test]
async fn tools_list_serves_the_master_tools_only_when_the_setting_is_effective() {
    use crate::agent::agent_protocol::serve_stdio;
    use tokio::io::BufReader;
    for (master, expected) in [(false, 0), (true, 2)] {
        let directory = tempfile::tempdir().unwrap();
        let server = server_with("127.0.0.1:9".parse().unwrap(), directory.path(), master);
        let input = format!(
            "{}\n{}\n",
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}),
            json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        );
        let mut output = Vec::new();
        serve_stdio(server, BufReader::new(input.as_bytes()), &mut output)
            .await
            .unwrap();
        let responses = String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>();
        let listed = responses[1]["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|tool| {
                MASTER_WRITE_TOOLS_FOR_TEST
                    .iter()
                    .any(|name| tool["name"] == *name)
            })
            .count();
        assert_eq!(listed, expected, "master={master}");
    }
}

#[tokio::test]
async fn with_the_setting_off_both_tools_are_refused_master_writes_disabled_before_anything() {
    let directory = tempfile::tempdir().unwrap();
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let server = server_with(simulator.address(), directory.path(), false);
    for (tool, args) in [
        ("prepare_ledger_creates", prepare_args(LEDGER)),
        ("post_ledger_creates", json!({"batch_id": "bridge-x"})),
        // Even arguments that would otherwise be refused first.
        ("prepare_ledger_creates", json!({"nonsense": 1})),
    ] {
        let response = server.call_tool(tool, args).await;
        assert_eq!(
            error_code(&response),
            "master_writes_disabled",
            "{tool}: {response}"
        );
    }
    assert_eq!(sent(simulator).len(), 0);
    assert!(!journal_path(directory.path()).exists());
}

#[tokio::test]
async fn a_tools_call_for_a_master_tool_with_the_setting_off_is_refused_not_unknown() {
    use crate::agent::agent_protocol::serve_stdio;
    use tokio::io::BufReader;
    let directory = tempfile::tempdir().unwrap();
    let server = server_with("127.0.0.1:9".parse().unwrap(), directory.path(), false);
    let input = format!(
        "{}\n{}\n",
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"post_ledger_creates","arguments":{"batch_id":"bridge-00000000-0000-4000-8000-000000000001"}}}),
    );
    let mut output = Vec::new();
    serve_stdio(server, BufReader::new(input.as_bytes()), &mut output)
        .await
        .unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("master_writes_disabled"), "{text}");
    assert!(!text.contains("tool_not_found"), "{text}");
}

// --- the typed boundary ----------------------------------------------------

#[tokio::test]
async fn every_refusal_of_a_ledger_input_reaches_the_caller_as_its_own_code() {
    let directory = tempfile::tempdir().unwrap();
    // No Tally is listening: the input is refused before any read.
    let server = server_with("127.0.0.1:9".parse().unwrap(), directory.path(), true);
    let too_long = "x".repeat(bridge_tally_core::master_binding::MAX_NAME_CHARS + 1);
    let cases: Vec<(Value, &str)> = vec![
        (
            json!({"name": LEDGER, "kind": "bank"}),
            "master_kind_not_admitted",
        ),
        (
            json!({"name": LEDGER, "kind": "Party"}),
            "master_kind_not_admitted",
        ),
        (
            json!({"name": "Caf\u{e9}", "kind": "suspense"}),
            "master_name_not_ascii",
        ),
        (
            json!({"name": "a\tb", "kind": "suspense"}),
            "master_name_control_character",
        ),
        (
            json!({"name": " lead", "kind": "suspense"}),
            "master_name_surrounding_whitespace",
        ),
        (
            json!({"name": "trail ", "kind": "suspense"}),
            "master_name_surrounding_whitespace",
        ),
        (
            json!({"name": "a  b", "kind": "suspense"}),
            "master_name_repeated_spaces",
        ),
        (
            json!({"name": too_long, "kind": "suspense"}),
            "argument_invalid:ledger",
        ),
        (
            json!({"name": LEDGER, "kind": "party"}),
            "party_side_required",
        ),
        (
            json!({"name": LEDGER, "kind": "party", "side": "both"}),
            "party_side_invalid",
        ),
        (
            json!({"name": LEDGER, "kind": "party", "side": "sundry_debtors", "gstin": "X"}),
            "gstin_shape_invalid",
        ),
        (
            json!({"name": LEDGER, "kind": "party", "side": "sundry_debtors", "gstin": "08ZZZZZ0000Z2ZQ"}),
            "gstin_check_character_invalid",
        ),
        (
            json!({"name": LEDGER, "kind": "party", "side": "sundry_debtors", "duty_head": "CGST"}),
            "ledger_field_not_for_kind",
        ),
        (
            json!({"name": LEDGER, "kind": "suspense", "gstin": LAB_GSTIN}),
            "ledger_field_not_for_kind",
        ),
        (
            json!({"name": LEDGER, "kind": "suspense", "bill_wise": false}),
            "ledger_field_not_for_kind",
        ),
        (
            json!({"name": LEDGER, "kind": "gst_duty"}),
            "gst_duty_head_required",
        ),
        (
            json!({"name": LEDGER, "kind": "gst_duty", "duty_head": "CGST"}),
            "gst_duty_head_not_admitted",
        ),
        (
            json!({"name": LEDGER, "kind": "gst_duty", "duty_head": "Nonsense"}),
            "gst_duty_head_not_admitted",
        ),
        (
            json!({"name": LEDGER, "kind": "gst_duty", "duty_head": "SGST/UTGST", "side": "sundry_debtors"}),
            "ledger_field_not_for_kind",
        ),
        // The published schema denies unknown fields.
        (
            json!({"name": LEDGER, "kind": "suspense", "opening_balance": "5"}),
            "argument_invalid:ledger",
        ),
        (
            json!({"name": LEDGER, "kind": "party", "side": "sundry_debtors", "bill_wise": "yes"}),
            "argument_invalid:ledger",
        ),
    ];
    for (ledger, code) in cases {
        let response = server
            .call_tool(
                "prepare_ledger_creates",
                json!({"company_guid": GUID, "ledger": ledger.clone()}),
            )
            .await;
        assert_eq!(error_code(&response), code, "{ledger}: {response}");
    }
    assert!(!journal_path(directory.path()).exists());
}

#[test]
fn the_admitted_inputs_parse_to_exactly_their_typed_items() {
    let debtor = parse_ledger_item(&json!({
        "name": LEDGER, "kind": "party", "side": "sundry_debtors",
        "gstin": LAB_GSTIN, "bill_wise": true
    }))
    .unwrap();
    assert_eq!(debtor.parent(), "Sundry Debtors");
    let creditor =
        parse_ledger_item(&json!({"name": LEDGER, "kind": "party", "side": "sundry_creditors"}))
            .unwrap();
    assert_eq!(creditor.parent(), "Sundry Creditors");
    assert!(matches!(
        creditor.kind(),
        AdmittedLedgerKind::Party {
            bill_wise: false,
            gstin: None,
            ..
        }
    ));
    let duty =
        parse_ledger_item(&json!({"name": LEDGER, "kind": "gst_duty", "duty_head": "SGST/UTGST"}))
            .unwrap();
    assert_eq!(duty.parent(), "Duties & Taxes");
    let suspense = parse_ledger_item(&json!({"name": LEDGER, "kind": "suspense"})).unwrap();
    assert_eq!(suspense.parent(), "Suspense A/c");
    // A stored record parses back to the same item.
    for original in [debtor, creditor, duty, suspense] {
        let stored = stored_from_item(&original);
        let batch = MasterBatch {
            batch_id: format!("bridge-{}", Uuid::new_v4()),
            batch_sha256: "0".repeat(64),
            class: MasterClass::LedgerCreate,
            company: MasterCompany {
                guid: GUID.into(),
                name: COMPANY.into(),
                number: "100004".into(),
                books_from: "20260401".into(),
            },
            items: vec![stored],
        };
        assert_eq!(item_from_batch(&batch).unwrap(), original);
    }
}

#[test]
fn a_stored_batch_that_is_not_one_admitted_ledger_is_refused() {
    let base = |items: Vec<StoredLedgerCreate>| MasterBatch {
        batch_id: format!("bridge-{}", Uuid::new_v4()),
        batch_sha256: "0".repeat(64),
        class: MasterClass::LedgerCreate,
        company: MasterCompany {
            guid: GUID.into(),
            name: COMPANY.into(),
            number: "100004".into(),
            books_from: "20260401".into(),
        },
        items,
    };
    let good = stored_from_item(&item(LEDGER));
    assert!(item_from_batch(&base(vec![good.clone()])).is_ok());
    assert_eq!(
        item_from_batch(&base(vec![])),
        Err("master_batch_invalid".to_string())
    );
    assert_eq!(
        item_from_batch(&base(vec![good.clone(), good.clone()])),
        Err("master_batch_invalid".to_string())
    );
    let mut bad = good.clone();
    bad.name = "Caf\u{e9}".into();
    assert!(item_from_batch(&base(vec![bad])).is_err());
    // A duty head the build does not admit.
    let mut duty = good.clone();
    duty.kind = StoredLedgerKind::GstDuty;
    duty.gst_duty_head = Some("CGST".into());
    assert!(item_from_batch(&base(vec![duty])).is_err());
    // A suspense ledger that carries a GSTIN is not what the item stores back.
    let mut stray = good;
    stray.party_gstin = Some(LAB_GSTIN.into());
    assert!(item_from_batch(&base(vec![stray])).is_err());
}

// --- prepare ---------------------------------------------------------------

#[tokio::test]
async fn prepare_refuses_an_unreadable_ledger_index_and_saves_nothing() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let response = server
        .call_tool("prepare_ledger_creates", prepare_args(LEDGER))
        .await;
    let observed = sent(simulator);
    assert_eq!(
        error_code(&response),
        "master_ledger_index_unavailable",
        "{response}"
    );
    assert_eq!(
        observed.len(),
        before_approval().len(),
        "the reads, and no more"
    );
    assert!(!journal_path(directory.path()).exists());
}

#[tokio::test]
async fn under_mask_parties_the_preview_a_prepare_returns_names_the_ledger_masked() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    server.settings.redaction = Redaction::MaskParties;
    let response = TEST_LEDGER_INDEX
        .scope(
            book(),
            server.call_tool("prepare_ledger_creates", prepare_args(LEDGER)),
        )
        .await;
    let origin = super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let preview = result(&response)["preview"].as_str().unwrap();
    assert_eq!(
        preview,
        super::super::master_review::masked_review_text(&item(LEDGER), &identity(), &origin)
            .unwrap()
    );
    assert!(!response.to_string().contains(LEDGER), "{response}");
}

#[tokio::test]
async fn prepare_saves_the_batch_and_returns_the_preview_the_person_will_be_shown() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let response = TEST_LEDGER_INDEX
        .scope(
            book(),
            server.call_tool("prepare_ledger_creates", prepare_args(LEDGER)),
        )
        .await;
    let observed = sent(simulator);
    // Reads only: no import was sent (the sentinel would be served and counted).
    assert_eq!(observed.len(), before_approval().len(), "{response}");
    let batch_id = result(&response)["batch_id"].as_str().unwrap().to_string();
    assert!(valid_batch_id(&batch_id));
    let origin = super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let expected = master_review_text(&item(LEDGER), &identity(), &origin).unwrap();
    assert_eq!(result(&response)["preview"], expected.as_str());
    assert_eq!(result(&response)["parent"], "Suspense A/c");
    assert!(result(&response)["next_step"]
        .as_str()
        .unwrap()
        .contains("post_ledger_creates"));
    assert_eq!(record_types(directory.path()), ["master_batch"]);
    let state = state_of(directory.path(), &batch_id);
    assert!(state.intent.is_none());
    assert_eq!(state.batch.items, vec![stored_from_item(&item(LEDGER))]);
    assert_eq!(
        state.batch.batch_sha256,
        MasterPostRequest::render(item(LEDGER), &identity()).sha256()
    );
}

#[tokio::test]
async fn prepare_refuses_a_name_the_book_already_holds_and_saves_nothing() {
    for held in ["Cash", "cash", "CASH"] {
        let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let response = TEST_LEDGER_INDEX
            .scope(
                book(),
                server.call_tool("prepare_ledger_creates", prepare_args(held)),
            )
            .await;
        sent(simulator);
        assert_eq!(
            error_code(&response),
            "master_name_taken",
            "{held}: {response}"
        );
        assert!(!journal_path(directory.path()).exists(), "{held}");
    }
}

// --- post: the approved path -----------------------------------------------

fn expected_request() -> MasterPostRequest {
    MasterPostRequest::render(item(LEDGER), &identity())
}

#[tokio::test]
async fn an_approved_post_sends_exactly_the_bytes_its_intent_recorded() {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    plans.extend(read_back(8, 3, Some(suspense_answer())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let scripted = ScriptedApproval::approving();
    let response = posting_then(
        &scripted,
        book_after(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    let observed = sent(simulator);
    // The POST, then the read-back: the marks and three paired reads.
    assert_eq!(
        observed.len(),
        post_at + 1 + 19,
        "the POST was the last write: {response}"
    );
    assert_eq!(
        observed
            .iter()
            .filter(|seen| seen.request_body_sha256
                == sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
                    expected_request().xml()
                )))
            .count(),
        1,
        "the create was sent once"
    );
    assert_eq!(result(&response)["state"], "posted_verified", "{response}");
    assert_eq!(result(&response)["attempt_recorded"], true);
    assert_eq!(result(&response)["counters"]["created"], 1, "{response}");
    assert_eq!(result(&response)["verification"]["guid"], NEW_GUID);
    assert_eq!(result(&response)["verification"]["settled"], true);
    let checks = &result(&response)["verification"]["checks"];
    for check in [
        "name",
        "guid_new",
        "parent",
        "tax_fields",
        "party_gstin",
        "opening_balance",
        "counters",
        "target_mark_moved",
        "other_marks_unmoved",
        "pre_image_unchanged",
    ] {
        assert_eq!(checks[check], "passed", "{check}: {response}");
    }
    assert!(result(&response).get("alert").is_none());
    // The party read carried PAN, bank, e-mail, phone and address: none of
    // it reaches the answer, the journal or the evidence.
    assert_no_private_value("answer", &response.to_string());
    assert_no_private_value(
        "journal",
        &String::from_utf8(journal_bytes(directory.path())).unwrap(),
    );

    let request = expected_request();
    let state = state_of(directory.path(), &batch.batch_id);
    let intent = state.intent.expect("the intent is recorded");
    // The intent names the digest of the rendered request; the wire carries
    // those bytes, encoded as the transport sends them.
    assert_eq!(intent.native_request_sha256(), request.sha256());
    assert_eq!(
        observed[post_at].request_body_sha256,
        sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
            request.xml()
        ))
    );
    assert_eq!(intent.batch_sha256(), batch.batch_sha256);
    // The aim marks of every loaded company, the target's among them.
    let mut marks = intent
        .pre_post_master_marks()
        .iter()
        .map(|mark| (mark.company_guid.clone(), mark.altmstid))
        .collect::<Vec<_>>();
    marks.sort();
    assert_eq!(
        marks,
        [
            ("22222222-2222-4222-8222-222222222222".to_string(), 3),
            (GUID.to_string(), 7)
        ]
    );
    // The pre-image sidecar the intent names reads back to its digest and count.
    let pre_image = read_pre_image(
        directory.path(),
        &batch.batch_id,
        intent.pre_image_sha256(),
        intent.pre_image_count(),
    )
    .unwrap();
    assert!(pre_image
        .entries()
        .contains_key("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa"));
    // One dialog, asked once, about the preview the review text builds.
    let origin = super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    assert_eq!(
        scripted.masters(),
        [master_review_text(&item(LEDGER), &identity(), &origin).unwrap()]
    );
    assert!(
        scripted.previews().is_empty(),
        "no voucher dialog was shown"
    );
    // The response is journaled with the parsed counters.
    let response_record = state.response.expect("the response is recorded");
    assert_eq!(response_record.counters.unwrap().created, 1);
    assert_eq!(
        record_types(directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
    let verdict = state.verdict.expect("the verdict is recorded");
    assert_eq!(verdict.state, MasterVerdictState::PostedVerified);
    assert_eq!(verdict.items[0].guid.as_deref(), Some(NEW_GUID));
    assert_eq!(verdict.items[0].reason, None);
    // The approval was spent under the lock, not left to lapse unspent.
    assert!(server.post_approvals.lapse_note(&batch.batch_id).is_none());
    assert!(!server.post_approvals.holds(&batch.batch_id));
}

#[tokio::test]
async fn an_already_dispatched_batch_is_reported_and_never_sent_again() {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.extend(read_back(8, 3, Some(suspense_answer())));
    let total = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let first = posting_then(
        &ScriptedApproval::approving(),
        book_after(),
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert_eq!(result(&first)["state"], "posted_verified", "{first}");
    let journal = journal_bytes(directory.path());
    let again = ScriptedApproval::approving();
    let second = posting(&again, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(
        observed.len(),
        total,
        "no request after the first call: {second}"
    );
    // The batch is settled: the journal alone answers, with its verdict.
    assert_eq!(result(&second)["state"], "posted_verified", "{second}");
    assert_eq!(result(&second)["journal"]["verdict"], "posted_verified");
    assert_eq!(result(&second)["dispatch"]["resent"], false);
    assert_eq!(result(&second)["journal"]["response_recorded"], true);
    let note = result(&second)["note"].as_str().unwrap();
    assert!(
        note.starts_with("This batch was sent to Tally once"),
        "{note}"
    );
    assert_eq!(
        result(&second)["journal"]["counters"]["created"],
        1,
        "{second}"
    );
    assert!(again.masters().is_empty(), "nobody was asked again");
    assert_eq!(
        journal_bytes(directory.path()),
        journal,
        "nothing was written"
    );
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_some());
}

/// A batch the journal shows dispatched with no response (the process died
/// between the POST and its record) is reported for reconciliation, never sent.
#[tokio::test]
async fn a_dispatched_batch_with_no_response_is_reported_for_reconciliation() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    record_intent_for_test(&server, &batch);
    let before = journal_bytes(directory.path());
    let scripted = ScriptedApproval::approving();
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    // The company cannot be verified (nothing answers), so the read-back is
    // not attempted: the journal's own account, never a create.
    assert!(!create_was_sent(&sent(simulator), &expected_request()));
    assert_eq!(
        result(&response)["state"],
        "reconciliation_required",
        "{response}"
    );
    assert_eq!(result(&response)["verification"]["attempted"], false);
    assert_eq!(result(&response)["attempt_recorded"], true);
    assert_eq!(result(&response)["dispatch"]["resent"], false);
    assert!(scripted.masters().is_empty());
    assert_eq!(journal_bytes(directory.path()), before);
    // With an intent and no answer recorded the batch "may have been" sent.
    let note = result(&response)["note"].as_str().unwrap();
    assert!(note.contains("may have been sent"), "{note}");
}

/// Records an intent for `batch` as an earlier call would have.
fn record_intent_for_test(server: &Server, batch: &MasterBatch) {
    let dir = &server.settings.data_dir;
    let lock = ExclusiveAdmission::acquire(dir).unwrap();
    let pre_image =
        PreImage::new([("bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb".to_string(), 1)]).unwrap();
    let written = write_pre_image(dir, &lock, batch, &pre_image).unwrap();
    let intent = MasterDispatchIntent::new(
        batch,
        "c".repeat(64),
        written,
        vec![super::super::master_ledger::MasterMark {
            company_guid: GUID.into(),
            altmstid: 7,
        }],
    )
    .unwrap();
    let _ = append_intent(dir, &lock, intent).unwrap();
}

#[tokio::test]
async fn a_declined_approval_records_nothing_and_sends_nothing() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let before = journal_bytes(directory.path());
    let scripted = ScriptedApproval::declining();
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(
        error_code(&response),
        "master_approval_declined",
        "{response}"
    );
    assert_eq!(result(&response)["attempt_recorded"], false);
    assert_eq!(result(&response)["dispatch"]["state"], "not_dispatched");
    assert_eq!(
        observed.len(),
        before_approval().len(),
        "the reads before the dialog, no POST"
    );
    assert_eq!(scripted.masters().len(), 1);
    assert_eq!(journal_bytes(directory.path()), before);
    assert!(!directory
        .path()
        .join(format!("master-pre-image-{}.json", batch.batch_id))
        .exists());
}

#[tokio::test]
async fn an_unscripted_approval_declines_and_nothing_is_sent() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server, LEDGER);
    let response = TEST_LEDGER_INDEX
        .scope(book(), server.call_tool("post_ledger_creates", args))
        .await;
    assert_eq!(sent(simulator).len(), before_approval().len());
    assert_eq!(
        error_code(&response),
        "master_approval_declined",
        "{response}"
    );
    assert_eq!(record_types(directory.path()), ["master_batch"]);
}

#[tokio::test]
async fn a_name_that_appeared_after_prepare_is_refused_before_anyone_is_asked() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server, "Cash");
    let scripted = ScriptedApproval::approving();
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    sent(simulator);
    assert_eq!(error_code(&response), "master_name_taken", "{response}");
    assert!(scripted.masters().is_empty(), "nobody was asked");
    assert_eq!(record_types(directory.path()), ["master_batch"]);
}

#[tokio::test]
async fn an_unknown_batch_is_refused_and_a_malformed_id_too() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let unknown = server
        .call_tool(
            "post_ledger_creates",
            json!({"batch_id": "bridge-00000000-0000-4000-8000-000000000001"}),
        )
        .await;
    assert_eq!(error_code(&unknown), "master_batch_not_found", "{unknown}");
    let malformed = server
        .call_tool("post_ledger_creates", json!({"batch_id": "bridge-nope"}))
        .await;
    assert_eq!(
        error_code(&malformed),
        "argument_invalid:batch_id",
        "{malformed}"
    );
    assert_eq!(sent(simulator).len(), 0);
}

// --- post: nothing sent without an intent ----------------------------------

/// Another process records the batch's intent while the dialog is open: the
/// queue is read, the intent refused under the lock, and no POST is sent.
#[tokio::test]
async fn an_intent_recorded_while_the_dialog_is_open_stops_the_post() {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let data_dir = directory.path().to_path_buf();
    let rival = batch.clone();
    let scripted = ScriptedApproval::approving_after(move || {
        let lock = ExclusiveAdmission::acquire(&data_dir).unwrap();
        let pre_image =
            PreImage::new([("bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb".to_string(), 1)]).unwrap();
        let written = write_pre_image(&data_dir, &lock, &rival, &pre_image).unwrap();
        let intent = MasterDispatchIntent::new(
            &rival,
            "c".repeat(64),
            written,
            vec![super::super::master_ledger::MasterMark {
                company_guid: GUID.into(),
                altmstid: 7,
            }],
        )
        .unwrap();
        let _ = append_intent(&data_dir, &lock, intent).unwrap();
    });
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(
        error_code(&response),
        "import_already_attempted",
        "{response}"
    );
    assert_eq!(observed.len(), post_at, "every queue read, and no POST");
    // Only the other process's intent is in the journal.
    assert_eq!(
        record_types(directory.path()),
        ["master_batch", "master_dispatch_intent"]
    );
}

/// Another batch of the same ledger is sent and unsettled: the post is refused
/// under the lock after the queue's reads, the approval is not spent, no
/// sidecar is written, no POST is sent, and the answer says to prepare a new
/// batch.
#[tokio::test]
async fn an_open_dispatch_of_the_same_ledger_refuses_the_post_and_says_what_to_do() {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (first, _) = saved_batch(&server, LEDGER);
    record_intent_for_test(&server, &first);
    let (second, args) = saved_batch(&server, LEDGER);
    let response = posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    let observed = sent(simulator);
    assert_eq!(
        error_code(&response),
        "master_ledger_open_dispatch_overlaps",
        "{response}"
    );
    assert_eq!(result(&response)["attempt_recorded"], false, "{response}");
    let next = result(&response)["error"]["next_step"].as_str().unwrap();
    assert!(next.contains("prepare_ledger_creates"), "{next}");
    assert_eq!(observed.len(), post_at, "every queue read, and no POST");
    assert!(!sidecar_of(directory.path(), &second.batch_id).exists());
    assert!(state_of(directory.path(), &second.batch_id)
        .intent
        .is_none());
}

/// A journal that cannot be read under the lock refuses before the intent: the
/// POST is not sent without one.
#[tokio::test]
async fn a_journal_that_cannot_be_read_under_the_lock_stops_the_post() {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server, LEDGER);
    let path = journal_path(directory.path());
    let scripted = ScriptedApproval::approving_after(move || {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        file.write_all(b"not a record\n").unwrap();
    });
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(
        error_code(&response),
        "master_ledger_unavailable",
        "{response}"
    );
    assert_eq!(observed.len(), post_at, "every queue read, and no POST");
    assert!(!String::from_utf8(journal_bytes(directory.path()))
        .unwrap()
        .contains("master_dispatch_intent"));
}

/// The wire lock is taken before the intent is recorded and is never waited
/// for: a busy one records nothing and sends nothing.
#[tokio::test]
async fn a_busy_wire_lock_records_nothing_and_sends_nothing() {
    let mut plans = before_approval();
    let queue = after_approval(xml(created_one()));
    // The aim marks read, the last read before the POST.
    let busy_send = plans.len() + queue.len() - 2;
    plans.extend(queue);
    plans.truncate(busy_send);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    server.runtime = crate::tally::TallyRuntime::default().with_wire_gate_config(
        crate::tally::TallyRuntime::default()
            .wire_gate_config()
            .clone()
            .busy_at_send(busy_send, Duration::from_millis(50)),
    );
    let (batch, args) = saved_batch(&server, LEDGER);
    let before = journal_bytes(directory.path());
    let scripted = ScriptedApproval::approving();
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(error_code(&response), "tally_endpoint_busy", "{response}");
    assert_eq!(result(&response)["attempt_recorded"], false, "{response}");
    assert_eq!(observed.len(), busy_send, "no POST");
    assert_eq!(journal_bytes(directory.path()), before);
    assert!(!directory
        .path()
        .join(format!("master-pre-image-{}.json", batch.batch_id))
        .exists());
}

/// The POST is lost after the intent: the intent stands, the outcome is
/// unknown, and the answer says so.
#[tokio::test]
async fn a_dispatch_error_after_the_intent_is_an_unknown_outcome_never_resent() {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one()).with_delivery(
        Delivery::ResetAfterRequestProcessed {
            delay: Duration::from_millis(0),
        },
    )));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let scripted = ScriptedApproval::approving();
    let response = posting(
        &scripted,
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    let first = sent(simulator).len();
    assert_eq!(
        error_code(&response),
        "import_dispatch_outcome_unknown",
        "{response}"
    );
    assert_eq!(result(&response)["attempt_recorded"], true, "{response}");
    assert_eq!(
        result(&response)["dispatch"]["state"],
        "reconciliation_required"
    );
    assert_eq!(result(&response)["dispatch"]["resent"], false);
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_some());
    assert!(first > 0);
    // A later call reports the journal and sends no create.
    let silent = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let later = server_at(silent.address(), directory.path());
    let report = posting(
        &ScriptedApproval::approving(),
        later.call_tool("post_ledger_creates", args),
    )
    .await;
    assert!(!create_was_sent(&sent(silent), &expected_request()));
    assert_eq!(
        result(&report)["state"],
        "reconciliation_required",
        "{report}"
    );
}

// --- post: the approval's own states, through the shared machinery ---------

fn pending_then_posted_plans() -> (Vec<ScenarioPlan>, usize) {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    plans.extend(read_back(8, 3, Some(suspense_answer())));
    (plans, post_at)
}

async fn until_answered(server: &Server, batch_id: &str) {
    tokio::time::timeout(Duration::from_secs(5), async {
        while !server.post_approvals.answered_for_test(batch_id) {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("the held dialog's answer was stamped");
}

#[tokio::test]
async fn an_open_dialog_returns_pending_and_the_next_call_posts_the_click() {
    let (plans, post_at) = pending_then_posted_plans();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let scripted = ScriptedApproval::held();
    let pending = posting(
        &scripted,
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    assert_eq!(result(&pending)["attempt_recorded"], false);
    assert_eq!(result(&pending)["dispatch"]["state"], "not_dispatched");
    assert!(server.post_approvals.holds(&batch.batch_id));
    assert_eq!(record_types(directory.path()), ["master_batch"]);

    scripted.answer(true);
    until_answered(&server, &batch.batch_id).await;
    let posted = posting_then(
        &scripted,
        book_after(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    let observed = sent(simulator);
    assert_eq!(result(&posted)["state"], "posted_verified", "{posted}");
    assert!(observed.len() > post_at, "the POST was sent");
    assert_eq!(scripted.masters().len(), 1, "one dialog, asked once");
    assert_eq!(
        record_types(directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
}

#[tokio::test]
async fn the_next_step_of_an_approved_answer_names_the_master_tool() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    // A post measured at the whole ceiling never fits in the answering call.
    server.post_approvals =
        std::sync::Arc::new(super::super::approval::PostApprovals::with_measured_post(
            directory.path(),
            super::super::approval::CALL_CEILING,
        ));
    let (_, args) = saved_batch(&server, LEDGER);
    let approved = posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    sent(simulator);
    assert_eq!(
        result(&approved)["approval"]["state"],
        "approved",
        "{approved}"
    );
    let next = result(&approved)["approval"]["next_step"].as_str().unwrap();
    assert!(next.contains("post_ledger_creates"), "{next}");
    assert!(!next.contains("post_import"), "{next}");
    assert_eq!(record_types(directory.path()), ["master_batch"]);
}

// --- before_dispatch, directly ---------------------------------------------

fn aim_for_test() -> Mutex<Option<MasterAim>> {
    Mutex::new(Some(MasterAim {
        pre_image: [("aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string(), 9)].into(),
        marks: vec![super::super::master_ledger::MasterMark {
            company_guid: GUID.into(),
            altmstid: 7,
        }],
    }))
}

fn unit_server(directory: &std::path::Path) -> Server {
    server_at("127.0.0.1:9".parse().unwrap(), directory)
}

#[test]
fn before_dispatch_records_the_sidecar_and_the_intent_and_spends_the_approval() {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, _) = saved_batch(&server, LEDGER);
    let approval = server.post_approvals.redeeming_for_test(&batch.batch_id);
    let request = expected_request();
    let licence = server
        .record_master_intent(&batch, &request, approval, &aim_for_test())
        .expect("recorded");
    assert_eq!(licence.batch_id(), batch.batch_id);
    assert_eq!(licence.native_request_sha256(), request.sha256());
    assert_eq!(
        record_types(directory.path()),
        ["master_batch", "master_dispatch_intent"]
    );
    assert!(directory
        .path()
        .join(format!("master-pre-image-{}.json", batch.batch_id))
        .exists());
    // Spent: the slot no longer holds it, so the same approval cannot be used twice.
    assert_eq!(
        server.post_approvals.spend(&batch.batch_id, approval),
        Err(UnderLockRefusal::ApprovalRevoked)
    );
}

#[test]
fn before_dispatch_refusals_record_nothing() {
    use BeforeDispatchError::Refused;
    let refused = |result: Result<RecordedIntent, BeforeDispatchError>| match result {
        Err(Refused(refusal)) => refusal,
        other => panic!("expected a named refusal, got {other:?}"),
    };
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, _) = saved_batch(&server, LEDGER);
    let request = expected_request();
    let approval = server.post_approvals.redeeming_for_test(&batch.batch_id);
    let sidecar = directory
        .path()
        .join(format!("master-pre-image-{}.json", batch.batch_id));
    let before = journal_bytes(directory.path());

    // No aim left by the recheck.
    assert_eq!(
        refused(server.record_master_intent(&batch, &request, approval, &Mutex::new(None))),
        UnderLockRefusal::PreImageUnrecorded
    );
    // An empty pre-image is a failed read, not an empty book.
    let empty = Mutex::new(Some(MasterAim {
        pre_image: Default::default(),
        marks: aim_for_test().lock().unwrap().clone().unwrap().marks,
    }));
    assert_eq!(
        refused(server.record_master_intent(&batch, &request, approval, &empty)),
        UnderLockRefusal::PreImageUnrecorded
    );
    // A batch the caller holds that is not the journal's.
    let mut changed = batch.clone();
    changed.batch_sha256 = "d".repeat(64);
    assert_eq!(
        refused(server.record_master_intent(&changed, &request, approval, &aim_for_test())),
        UnderLockRefusal::BatchChanged
    );
    let mut renamed = batch.clone();
    renamed.items[0].name = "Another Name".into();
    assert_eq!(
        refused(server.record_master_intent(&renamed, &request, approval, &aim_for_test())),
        UnderLockRefusal::BatchChanged
    );
    // A batch the journal does not hold.
    let mut unknown = batch.clone();
    unknown.batch_id = format!("bridge-{}", Uuid::new_v4());
    assert_eq!(
        refused(server.record_master_intent(&unknown, &request, approval, &aim_for_test())),
        UnderLockRefusal::BatchNotFound
    );
    // An approval that is not the one taken (revoked, or never taken).
    assert_eq!(
        refused(server.record_master_intent(&batch, &request, Uuid::new_v4(), &aim_for_test())),
        UnderLockRefusal::ApprovalRevoked
    );
    assert_eq!(journal_bytes(directory.path()), before);
    assert!(!sidecar.exists(), "nothing was written for any refusal");

    // A dispatched batch is never recorded twice.
    record_intent_for_test(&server, &batch);
    let after_first = journal_bytes(directory.path());
    assert_eq!(
        refused(server.record_master_intent(&batch, &request, approval, &aim_for_test())),
        UnderLockRefusal::AlreadyAttempted
    );
    assert_eq!(journal_bytes(directory.path()), after_first);
}

#[test]
fn before_dispatch_refuses_a_second_open_dispatch_of_the_same_ledger() {
    use BeforeDispatchError::Refused;
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (first, _) = saved_batch(&server, LEDGER);
    record_intent_for_test(&server, &first);
    // A new batch of the same ledger while the first is unsettled.
    let (second, _) = saved_batch(&server, LEDGER);
    let approval = server.post_approvals.redeeming_for_test(&second.batch_id);
    let before = journal_bytes(directory.path());
    let result =
        server.record_master_intent(&second, &expected_request(), approval, &aim_for_test());
    assert!(
        matches!(
            result,
            Err(Refused(UnderLockRefusal::MasterOverlapsOpenDispatch))
        ),
        "{result:?}"
    );
    assert_eq!(journal_bytes(directory.path()), before);
    // Refused before the approval was spent and before any sidecar was written:
    // the approval is still there to spend, and the batch has no sidecar.
    assert_eq!(
        server.post_approvals.spend(&second.batch_id, approval),
        Ok(())
    );
    assert!(!sidecar_of(directory.path(), &second.batch_id).exists());
}

fn sidecar_of(directory: &std::path::Path, batch_id: &str) -> std::path::PathBuf {
    directory.join(format!("master-pre-image-{batch_id}.json"))
}

/// The journal refuses the intent only after the sidecar was written (here, a
/// record too large to append, which no pre-check sees): the sidecar this call
/// wrote is removed under the lock, nothing is recorded, and the refusal is a
/// named one.
#[test]
fn an_append_refusal_after_the_sidecar_was_written_leaves_no_sidecar() {
    use BeforeDispatchError::Refused;
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, _) = saved_batch(&server, LEDGER);
    let approval = server.post_approvals.redeeming_for_test(&batch.batch_id);
    let aim = aim_for_test();
    {
        // More company marks than one journal record may hold.
        let mut aimed = aim.lock().unwrap();
        let marks = &mut aimed.as_mut().unwrap().marks;
        marks.extend(
            (0..600_000_u32).map(|n| super::super::master_ledger::MasterMark {
                company_guid: format!("{n:08x}-0000-4000-8000-000000000000"),
                altmstid: 1,
            }),
        );
    }
    let before = journal_bytes(directory.path());
    let result = server.record_master_intent(&batch, &expected_request(), approval, &aim);
    assert!(
        matches!(result, Err(Refused(UnderLockRefusal::MasterJournalRefused))),
        "{result:?}"
    );
    assert_eq!(
        journal_bytes(directory.path()),
        before,
        "no intent recorded"
    );
    assert!(!sidecar_of(directory.path(), &batch.batch_id).exists());
}

// --- Education, cancellation and routing -----------------------------------

#[tokio::test]
async fn an_education_book_is_refused_before_anything_is_saved_or_asked() {
    let education = companies().replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    assert_ne!(education, companies());
    let bracket = |body: &str| {
        vec![
            xml(education.clone()),
            xml(body.to_string()),
            status(),
            xml(body.to_string()),
            status(),
            xml(education.clone()),
        ]
    };
    let mut plans = verified_company();
    plans.extend(bracket("<ENVELOPE/>"));
    plans.extend(paired(groups()));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let response = TEST_LEDGER_INDEX
        .scope(
            book(),
            server.call_tool("prepare_ledger_creates", prepare_args(LEDGER)),
        )
        .await;
    sent(simulator);
    assert_eq!(
        error_code(&response),
        "master_writes_need_licensed_mode",
        "{response}"
    );
    assert!(!journal_path(directory.path()).exists());
}

#[test]
fn the_dispatch_state_a_cancellation_reads_is_the_masters_journal() {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    assert_eq!(
        server
            .recorded_post_attempt("post_ledger_creates", &args)
            .unwrap(),
        Some(false)
    );
    // The voucher journal knows nothing of this batch: asked as `post_import`
    // (which needs a company), the answer would be an error, never `Some(false)`.
    assert!(server.recorded_post_attempt("post_import", &args).is_err());
    let cancelled = server
        .cancelled_post_for_response("post_ledger_creates", &args)
        .unwrap();
    assert_eq!(cancelled.payload["result"]["attempt_recorded"], false);
    assert_eq!(
        cancelled.payload["result"]["dispatch"]["state"],
        "not_dispatched"
    );
    assert_eq!(
        cancelled.payload["result"]["error"]["code"],
        "request_cancelled"
    );
    assert_eq!(cancelled.company_guid.as_deref(), Some(GUID));
    record_intent_for_test(&server, &batch);
    assert_eq!(
        server
            .recorded_post_attempt("post_ledger_creates", &args)
            .unwrap(),
        Some(true)
    );
    let cancelled = server
        .cancelled_post_for_response("post_ledger_creates", &args)
        .unwrap();
    assert_eq!(cancelled.payload["result"]["attempt_recorded"], true);
    assert_eq!(
        cancelled.payload["result"]["dispatch"]["state"],
        "reconciliation_required"
    );
    // An unknown batch is not known to be unsent.
    assert_eq!(
        server
            .recorded_post_attempt(
                "post_ledger_creates",
                &json!({"batch_id": format!("bridge-{}", Uuid::new_v4())})
            )
            .unwrap(),
        None
    );
}

/// `tools/call post_ledger_creates` runs the master post, not the voucher
/// post: an unknown batch is answered by the master tool.
#[tokio::test]
async fn a_tools_call_runs_the_master_post_through_the_shared_post_path() {
    use crate::agent::agent_protocol::serve_stdio;
    use tokio::io::BufReader;
    let directory = tempfile::tempdir().unwrap();
    let server = server_with("127.0.0.1:9".parse().unwrap(), directory.path(), true);
    let input = format!(
        "{}\n{}\n",
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"post_ledger_creates","arguments":{"batch_id":"bridge-00000000-0000-4000-8000-000000000001"}}}),
    );
    let mut output = Vec::new();
    serve_stdio(server, BufReader::new(input.as_bytes()), &mut output)
        .await
        .unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("master_batch_not_found"), "{text}");
}

// --- the licence -----------------------------------------------------------

/// A `RecordedIntent` is made only by the journal: this is the journal's own
/// recorded intent for `request`, from a throwaway data directory, with the id
/// of the batch it was recorded for.
fn recorded_intent_for(request: &MasterPostRequest) -> (RecordedIntent, String) {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, _) = saved_batch(&server, LEDGER);
    let approval = server.post_approvals.redeeming_for_test(&batch.batch_id);
    let intent = server
        .record_master_intent(&batch, request, approval, &aim_for_test())
        .unwrap();
    (intent, batch.batch_id)
}

#[test]
fn only_the_recorded_intent_of_the_same_bytes_and_batch_licenses_a_master_send() {
    let identity = identity();
    let marks = crate::tally::agent_read_request::AgentReadRequest::parse(
        crate::agent::read_profiles::render_agent_company_high_water(identity.display_name()),
    )
    .unwrap();
    let approved = |name: &str| {
        ApprovedImport::approved_masters_for_test(item(name), &identity, marks.clone())
    };
    let request = expected_request();
    let (intent, batch_id) = recorded_intent_for(&request);
    assert!(intent.licenses(&approved(LEDGER).bound_to_batch(&batch_id)));
    // The intent of one ledger's bytes does not license another ledger's.
    let (intent, batch_id) = recorded_intent_for(&request);
    assert!(!intent.licenses(&approved("Other Ledger").bound_to_batch(&batch_id)));
    // Nor does batch A's intent license batch B's send, same bytes or not.
    let (intent, _) = recorded_intent_for(&request);
    let other_batch = format!("bridge-{}", Uuid::new_v4());
    assert!(!intent.licenses(&approved(LEDGER).bound_to_batch(&other_batch)));
    // A master request bound to no batch is licensed by no intent.
    let (intent, _) = recorded_intent_for(&request);
    assert!(!intent.licenses(&approved(LEDGER)));
    // The voucher licence covers no master request.
    assert!(!().licenses(&approved(LEDGER).bound_to_batch(&batch_id)));
}

/// An id the masters journal does not hold has nothing of this tool's to
/// withdraw: the call is refused and an approval held under that id (a voucher
/// batch's, or one for another kind) is untouched.
#[tokio::test]
async fn a_call_for_a_batch_the_masters_journal_lacks_leaves_the_approval_slot_alone() {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let id = format!("bridge-{}", Uuid::new_v4());
    let identity = identity();
    let item = item(LEDGER);
    let marks = crate::tally::agent_read_request::AgentReadRequest::parse(
        crate::agent::read_profiles::render_agent_company_high_water(identity.display_name()),
    )
    .unwrap();
    server
        .post_approvals
        .hold_approved(
            &id,
            ApprovalBinding::masters(
                "a".repeat(64).as_str(),
                "http://127.0.0.1:9",
                &identity,
                &item,
                "p",
            ),
            ApprovedImport::approved_masters_for_test(item.clone(), &identity, marks),
            MasterPostRequest::render(item, &identity),
            crate::tally::approved_import::Answered {
                approved: true,
                at: std::time::Instant::now(),
                at_wall: std::time::SystemTime::now(),
            },
        )
        .unwrap();
    let response = server
        .call_tool("post_ledger_creates", json!({"batch_id": id}))
        .await;
    assert_eq!(
        error_code(&response),
        "master_batch_not_found",
        "{response}"
    );
    assert!(
        server.post_approvals.holds(&id),
        "the held approval is untouched"
    );
    let malformed = server
        .call_tool("post_ledger_creates", json!({"batch_id": "bridge-nope"}))
        .await;
    assert_eq!(error_code(&malformed), "argument_invalid:batch_id");
    assert!(server.post_approvals.holds(&id));
}

// --- the read-back and the verdict, end to end ----------------------------

/// Everything up to the POST: the reads before approval, the queue's, the POST.
fn through_post(post: ScenarioPlan) -> Vec<ScenarioPlan> {
    let mut plans = before_approval();
    plans.extend(after_approval(post));
    plans
}

fn journal_text(directory: &std::path::Path) -> String {
    String::from_utf8(journal_bytes(directory)).unwrap()
}

/// The first key of the answer's `result`: where a loud state puts its alert.
fn first_key(response: &Value) -> String {
    result(response)
        .as_object()
        .unwrap()
        .keys()
        .next()
        .unwrap()
        .clone()
}

#[tokio::test]
async fn a_name_found_on_an_existing_guid_is_posted_into_existing_master_and_is_loud() {
    let mut plans = through_post(xml(created_one()));
    // No party read: the name resolves to a master that was already there.
    plans.extend(read_back(8, 3, None));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let into_cash = vec![IndexedMaster {
        kind: MasterKind::Ledger,
        guid: CASH_GUID.into(),
        alter_id: 9,
        names: vec!["Cash".into(), LEDGER.into()],
    }];
    let response = posting_then(
        &ScriptedApproval::approving(),
        into_cash,
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&response)["state"],
        "posted_into_existing_master",
        "{response}"
    );
    assert_eq!(first_key(&response), "alert", "{response}");
    assert!(result(&response)["alert"]
        .as_str()
        .unwrap()
        .starts_with("ATTENTION"));
    let next = result(&response)["next_step"].as_str().unwrap();
    assert!(
        next.contains("check") && next.contains("Do not retry"),
        "{next}"
    );
    assert_eq!(result(&response)["verification"]["guid"], CASH_GUID);
    assert_eq!(result(&response)["verification"]["settled"], true);
    assert!(result(&response)["verification"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|reason| reason == "name_resolves_to_existing_master"));
    let state = state_of(directory.path(), &batch.batch_id);
    assert_eq!(
        state.verdict.as_ref().map(|verdict| verdict.state),
        Some(MasterVerdictState::PostedIntoExistingMaster)
    );
    assert!(state.is_settled());
    // Later, from the journal alone: the same loud answer, nothing read or sent.
    let silent = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let later = server_at(silent.address(), directory.path());
    let report = posting(
        &ScriptedApproval::approving(),
        later.call_tool("post_ledger_creates", args),
    )
    .await;
    assert_eq!(sent(silent).len(), 0);
    assert_eq!(
        result(&report)["state"],
        "posted_into_existing_master",
        "{report}"
    );
    assert_eq!(first_key(&report), "alert");
    assert!(result(&report)["next_step"]
        .as_str()
        .unwrap()
        .contains("Do not retry"));
    assert_eq!(create_count(&observed), 1);
}

fn create_count(observed: &[tally_protocol_simulator::ObservedRequest]) -> usize {
    let sha = sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        expected_request().xml(),
    ));
    observed
        .iter()
        .filter(|seen| seen.request_body_sha256 == sha)
        .count()
}

#[tokio::test]
async fn another_companys_mark_moving_is_located_elsewhere_and_is_loud() {
    let mut plans = through_post(xml(created_one()));
    plans.extend(read_back(8, 4, Some(suspense_answer())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let response = posting_then(
        &ScriptedApproval::approving(),
        book_after(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    sent(simulator);
    assert_eq!(
        result(&response)["state"],
        "master_write_located_elsewhere",
        "{response}"
    );
    assert_eq!(first_key(&response), "alert", "{response}");
    let next = result(&response)["next_step"].as_str().unwrap();
    assert!(
        next.contains("Tally") && next.contains("Do not retry"),
        "{next}"
    );
    assert_eq!(
        result(&response)["verification"]["checks"]["other_marks_unmoved"],
        "failed"
    );
    assert_eq!(
        state_of(directory.path(), &batch.batch_id)
            .verdict
            .map(|verdict| verdict.state),
        Some(MasterVerdictState::MasterWriteLocatedElsewhere)
    );
    assert_no_private_value("answer", &response.to_string());
    assert_no_private_value("journal", &journal_text(directory.path()));
}

#[tokio::test]
async fn an_unknown_outcome_then_a_reconcile_that_finds_nothing_is_confirmed_absent_and_never_resent(
) {
    let plans = through_post(xml(created_one()).with_delivery(
        Delivery::ResetAfterRequestProcessed {
            delay: Duration::from_millis(0),
        },
    ));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let unknown = posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert_eq!(
        error_code(&unknown),
        "import_dispatch_outcome_unknown",
        "{unknown}"
    );
    assert_eq!(create_count(&sent(simulator)), 1);
    assert!(!state_of(directory.path(), &batch.batch_id).is_settled());
    // The reconcile: the book is as it was and no mark moved since the intent.
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(7, 3, None))).unwrap();
    let later = server_at(reads.address(), directory.path());
    let absent = reconciling(book(), later.call_tool("post_ledger_creates", args.clone())).await;
    let observed = sent(reads);
    assert_eq!(create_count(&observed), 0, "a reconcile sends no create");
    assert_eq!(result(&absent)["state"], "confirmed_absent", "{absent}");
    assert_eq!(result(&absent)["verification"]["moment"], "reconcile");
    assert_eq!(result(&absent)["verification"]["settled"], true);
    assert_eq!(result(&absent)["dispatch"]["resent"], false);
    let next = result(&absent)["next_step"].as_str().unwrap();
    assert!(
        next.contains("prepare_ledger_creates") && next.contains("new approval"),
        "{next}"
    );
    assert!(result(&absent).get("alert").is_none());
    assert_eq!(
        record_types(directory.path()),
        ["master_batch", "master_dispatch_intent", "master_verdict"]
    );
    let journal = read_journal(directory.path()).unwrap();
    assert!(journal.state(&batch.batch_id).unwrap().is_settled());
    assert_eq!(journal.summary().not_settled, 0);
    // A second post of the same batch is still refused: it reads the journal,
    // sends and reads nothing, and writes nothing.
    let silent = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let third = server_at(silent.address(), directory.path());
    let before = journal_bytes(directory.path());
    let scripted = ScriptedApproval::approving();
    let again = posting(&scripted, third.call_tool("post_ledger_creates", args)).await;
    assert_eq!(sent(silent).len(), 0);
    assert_eq!(result(&again)["state"], "confirmed_absent", "{again}");
    assert_eq!(result(&again)["dispatch"]["resent"], false);
    assert!(scripted.masters().is_empty(), "nobody was asked again");
    assert_eq!(journal_bytes(directory.path()), before);
}

#[tokio::test]
async fn a_reconcile_does_not_confirm_absence_when_a_mark_moved_or_the_name_appeared() {
    // The mark of the target moved since the intent: not absent, a doubt.
    for (target, other, held, expect) in [
        (8, 3, book(), "reconciliation_required"),
        (7, 4, book(), "master_write_located_elsewhere"),
        (7, 3, book_after(), "reconciliation_required"),
    ] {
        let plans = through_post(xml(created_one()).with_delivery(
            Delivery::ResetAfterRequestProcessed {
                delay: Duration::from_millis(0),
            },
        ));
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (batch, args) = saved_batch(&server, LEDGER);
        posting(
            &ScriptedApproval::approving(),
            server.call_tool("post_ledger_creates", args.clone()),
        )
        .await;
        sent(simulator);
        // A found ledger needs its party row to be described.
        let party = held.len().gt(&book().len()).then(suspense_answer);
        let reads =
            SequenceSimulator::spawn(with_sentinel(reconcile_reads(target, other, party))).unwrap();
        let later = server_at(reads.address(), directory.path());
        let response = reconciling(held, later.call_tool("post_ledger_creates", args)).await;
        sent(reads);
        assert_eq!(result(&response)["state"], expect, "{response}");
        assert_ne!(result(&response)["state"], "confirmed_absent");
        let _ = batch;
    }
}

#[tokio::test]
async fn an_answer_that_cannot_be_recorded_is_reported_as_answered_not_as_maybe_sent() {
    // Tally answers, but another process holds the journal's lock when the
    // answer is recorded: the call says Tally answered, with its counters.
    let plans = through_post(
        xml(created_one()).with_delivery(Delivery::SlowHeaders(Duration::from_secs(2))),
    );
    let post_at = plans.len() - 1;
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let data_dir = directory.path().to_path_buf();
    let approving = ScriptedApproval::approving();
    let post = posting(&approving, server.call_tool("post_ledger_creates", args));
    let hold_lock = async {
        let started = std::time::Instant::now();
        // Once the POST has arrived the intent's use of the lock is over: hold
        // it while the answer is recorded.
        while simulator.received() <= post_at {
            assert!(
                started.elapsed() < Duration::from_secs(20),
                "the POST never arrived"
            );
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        let lock = ExclusiveAdmission::acquire(&data_dir).expect("unheld during the POST");
        tokio::time::sleep(Duration::from_secs(4)).await;
        drop(lock);
    };
    let (response, ()) = tokio::join!(post, hold_lock);
    sent(simulator);
    let answered = result(&response);
    assert_eq!(answered["state"], "posted_unverified", "{response}");
    assert_eq!(answered["answer_recorded"], false, "{response}");
    assert_eq!(answered["counters"]["created"], 1, "{response}");
    assert!(
        !response.to_string().contains("may have been sent"),
        "{response}"
    );
    // Only the intent is in the journal: a later call reconciles it.
    assert_eq!(
        record_types(directory.path()),
        ["master_batch", "master_dispatch_intent"]
    );
    let _ = batch;
}

#[tokio::test]
async fn right_after_a_post_that_was_answered_a_missing_ledger_is_a_doubt_not_absence() {
    // Tally answered CREATED=1, and the book shows no such ledger, no mark moved.
    let mut plans = through_post(xml(created_one()));
    plans.extend(read_back(7, 3, None));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let response = posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    sent(simulator);
    assert_eq!(
        result(&response)["state"],
        "reconciliation_required",
        "{response}"
    );
    assert!(result(&response)["verification"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|reason| reason == "ledger_absent_after_post"));
    assert!(!state_of(directory.path(), &batch.batch_id).is_settled());
}

#[tokio::test]
async fn an_answer_without_counters_is_never_verified_even_when_the_book_reads_back_clean() {
    // Tally's answer carries no counters; the book shows exactly the new ledger.
    let mut plans = through_post(xml(
        "<ENVELOPE><BODY>no counters here</BODY></ENVELOPE>".to_string()
    ));
    plans.extend(read_back(8, 3, Some(suspense_answer())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let response = posting_then(
        &ScriptedApproval::approving(),
        book_after(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    sent(simulator);
    assert_eq!(
        result(&response)["state"],
        "reconciliation_required",
        "{response}"
    );
    let verification = &result(&response)["verification"];
    assert!(verification["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|reason| reason == "counters_unavailable"));
    assert_eq!(verification["checks"]["counters"], "not_checked");
    assert_eq!(verification["checks"]["name"], "passed");
    assert!(verification["counters_note"].as_str().is_some());
    assert_eq!(verification["settled"], false);
    assert!(!state_of(directory.path(), &batch.batch_id).is_settled());
}

#[tokio::test]
async fn a_reconcile_that_cannot_take_the_observation_lease_is_unconfirmed_never_absent() {
    let simulator = SequenceSimulator::spawn(with_sentinel(reconcile_reads(7, 3, None))).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    record_intent_for_test(&server, &batch);
    // Another process is dispatching on this endpoint: it holds the lease
    // exclusively, and its POST of this batch could still be on the way.
    let dispatching = crate::endpoint_coordination::acquire(&server.settings.endpoint).unwrap();
    let busy = reconciling(
        book(),
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert_eq!(
        result(&busy)["state"],
        "masters_after_post_unconfirmed",
        "{busy}"
    );
    assert_ne!(result(&busy)["state"], "confirmed_absent");
    assert_eq!(
        result(&busy)["verification"]["reasons"][0],
        "endpoint_dispatch_busy"
    );
    assert_eq!(result(&busy)["verification"]["settled"], false);
    assert_eq!(result(&busy)["dispatch"]["resent"], false);
    assert!(!state_of(directory.path(), &batch.batch_id).is_settled());
    drop(dispatching);
    // Nothing was read while the lease was refused.
    let observed_busy = sent(simulator);
    assert_eq!(
        observed_busy.len(),
        0,
        "no read while another dispatch runs"
    );
    // With the lease released the same call reads the book back.
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(7, 3, None))).unwrap();
    let later = server_at(reads.address(), directory.path());
    let released = reconciling(book(), later.call_tool("post_ledger_creates", args)).await;
    assert!(!sent(reads).is_empty(), "{released}");
    assert_ne!(
        result(&released)["verification"]["reasons"][0],
        "endpoint_dispatch_busy"
    );
}

#[tokio::test]
async fn a_read_back_that_is_unavailable_is_unconfirmed_and_a_later_reconcile_settles_it() {
    // The POST answers and nothing after it does.
    let plans = through_post(xml(created_one()));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let first = posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    sent(simulator);
    assert_eq!(
        result(&first)["state"],
        "masters_after_post_unconfirmed",
        "{first}"
    );
    assert_eq!(result(&first)["verification"]["settled"], false);
    assert_eq!(result(&first)["verification"]["verdict_recorded"], true);
    assert_eq!(result(&first)["attempt_recorded"], true);
    assert_eq!(
        record_types(directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
    assert!(!state_of(directory.path(), &batch.batch_id).is_settled());
    // Unavailable again: the same words are not written twice.
    let before = journal_bytes(directory.path());
    let still = SequenceSimulator::spawn(with_sentinel(verified_company())).unwrap();
    let second_server = server_at(still.address(), directory.path());
    let second = reconciling(
        book_after(),
        second_server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert!(!create_was_sent(&sent(still), &expected_request()));
    assert_eq!(
        result(&second)["state"],
        "masters_after_post_unconfirmed",
        "{second}"
    );
    assert_eq!(result(&second)["verification"]["verdict_recorded"], true);
    assert_eq!(result(&second)["verification"]["verdict_written"], false);
    assert_eq!(journal_bytes(directory.path()), before);
    // The company cannot even be verified: the journal answers, writes nothing.
    let down = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let third_server = server_at(down.address(), directory.path());
    let third = reconciling(
        book_after(),
        third_server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    sent(down);
    assert_eq!(
        result(&third)["verification"]["attempted"],
        false,
        "{third}"
    );
    assert_eq!(journal_bytes(directory.path()), before);
    // The read-back works at last: the reconcile settles the batch.
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(
        8,
        3,
        Some(suspense_answer()),
    )))
    .unwrap();
    let fourth_server = server_at(reads.address(), directory.path());
    let settled = reconciling(
        book_after(),
        fourth_server.call_tool("post_ledger_creates", args),
    )
    .await;
    let observed = sent(reads);
    assert_eq!(create_count(&observed), 0);
    assert_eq!(result(&settled)["state"], "posted_verified", "{settled}");
    assert_eq!(result(&settled)["verification"]["moment"], "reconcile");
    assert_eq!(result(&settled)["verification"]["guid"], NEW_GUID);
    let state = state_of(directory.path(), &batch.batch_id);
    assert!(state.is_settled());
    assert_eq!(
        record_types(directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict",
            "master_verdict"
        ]
    );
    for text in [
        result(&settled).to_string(),
        result(&first).to_string(),
        journal_text(directory.path()),
    ] {
        assert_no_private_value("record", &text);
    }
}

#[tokio::test]
async fn a_party_row_that_differs_from_what_was_sent_is_a_doubt_not_a_verified_create() {
    // The ledger was created under another parent and with an opening balance.
    for (parent, opening, reason) in [
        ("Sundry Debtors", "0.00", "parent_differs_from_sent"),
        ("Suspense A/c", "1250.00", "opening_balance_not_zero"),
    ] {
        let mut plans = through_post(xml(created_one()));
        plans.extend(read_back(8, 3, Some(party_answer(parent, "", opening))));
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (batch, args) = saved_batch(&server, LEDGER);
        let response = posting_then(
            &ScriptedApproval::approving(),
            book_after(),
            server.call_tool("post_ledger_creates", args),
        )
        .await;
        sent(simulator);
        assert_eq!(
            result(&response)["state"],
            "reconciliation_required",
            "{response}"
        );
        assert!(result(&response)["verification"]["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .any(|seen| seen == reason));
        assert!(!state_of(directory.path(), &batch.batch_id).is_settled());
        assert_no_private_value("answer", &response.to_string());
        assert_no_private_value("journal", &journal_text(directory.path()));
    }
}

#[test]
fn every_verdict_state_has_its_code_and_only_the_loud_ones_carry_an_alert() {
    use MasterVerdictState::{
        ConfirmedAbsent, MasterWriteLocatedElsewhere, MastersAfterPostUnconfirmed,
        PostedIntoExistingMaster, PostedVerified, ReconciliationRequired,
    };
    for (state, code) in [
        (PostedVerified, "posted_verified"),
        (PostedIntoExistingMaster, "posted_into_existing_master"),
        (
            MasterWriteLocatedElsewhere,
            "master_write_located_elsewhere",
        ),
        (ConfirmedAbsent, "confirmed_absent"),
        (
            MastersAfterPostUnconfirmed,
            "masters_after_post_unconfirmed",
        ),
        (ReconciliationRequired, "reconciliation_required"),
    ] {
        assert_eq!(verdict_state_code(state), code);
        let loud = matches!(
            state,
            PostedIntoExistingMaster | MasterWriteLocatedElsewhere
        );
        assert_eq!(settled_alert(state).is_some(), loud, "{code}");
        let next = settled_next_step(state);
        assert_eq!(next.contains("Do not retry"), loud, "{code}: {next}");
        assert!(!next.is_empty());
    }
}

// --- the test seam ---------------------------------------------------------

/// The seam the tool tests use until they read one company's captured answers
/// end to end is compiled under bare `cfg(test)`, as the approval seam is: no
/// feature or flag compiles it into a shipped build.
#[test]
fn the_ledger_index_seams_are_under_bare_cfg_test() {
    let source = include_str!("agent_master_post.rs");
    for needle in [
        "#[cfg(test)]\n    if let Ok(injected) = TEST_LEDGER_INDEX.try_with(Clone::clone) {",
        "#[cfg(test)]\n    if let Ok(injected) = TEST_LEDGER_INDEX_AFTER_POST.try_with(Clone::clone) {",
    ] {
        assert_eq!(source.matches(needle).count(), 1, "{needle}");
    }
    // Each static is declared in a bare `cfg(test)` task_local block, once.
    assert_eq!(
        source.matches("#[cfg(test)]\ntokio::task_local! {").count(),
        2
    );
    for declaration in [
        "static TEST_LEDGER_INDEX: Vec<IndexedMaster>;",
        "static TEST_LEDGER_INDEX_AFTER_POST: Vec<IndexedMaster>;",
    ] {
        assert_eq!(source.matches(declaration).count(), 1, "{declaration}");
    }
    // The debug-assertions cfg is not listed here: the egress boundary check
    // (`scripts/check-tally-egress-boundary.mjs`) refuses its name in every
    // Rust file under `src-tauri`, this one included.
    for forbidden in ["cfg(any(test", "cfg(feature", "cfg(all(test"] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}
