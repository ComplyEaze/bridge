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
use super::super::master_ledger;
use super::super::master_ledger::{
    read_pre_image, MasterVerdict, MasterVerdictState, VerdictItem, TEST_FAULTS,
};
use super::super::master_post::{TEST_LEDGER_INDEX, TEST_LEDGER_INDEX_AFTER_POST};
use super::verify::{judged_next_step, not_read_back_next_step};
use super::*;
use crate::agent::master_writes_effective;
use crate::agent::{Posting, Redaction, Settings};
use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};
use crate::tally::approved_import::{ApprovedImport, DispatchLicense};
use crate::tally::VerifiedCompanyIdentity;
use bridge_tally_core::master_binding::{absence_key, canonical_master_key};
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
         <COMPANY NAME=\"Synthetic Other Lab\" RESERVEDNAME=\"\"><GUID>{other}</GUID><ALTVCHID>50</ALTVCHID><ALTMSTID>{other_masters}</ALTMSTID></COMPANY>\
         </COLLECTION></DATA></BODY></ENVELOPE>",
        other = crate::synthetic_guid(0x2222),
    )
}

const NEW_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f-000000d1";
fn cash_guid() -> &'static str {
    crate::synthetic_guid(0xaaaa)
}

/// Values a party-ledger master row carries that no answer or journal record
/// may ever hold. Synthetic, none of them a real person's.
const PRIVATE_VALUES: [&str; 8] = [
    "SYNTHPAN01",
    "Synthetic Pan Holder",
    "Synthetic Bank Holder",
    "111111111111",
    "ZZZZ0000001",
    "private.person@example.invalid",
    "9999999999",
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
/// the new one, with a GUID the pre-image did not hold, at the `ALTERID` the
/// first master change after the mark of 7 takes (protocol reference §9.4j).
fn book_after() -> Vec<IndexedMaster> {
    let mut book = book();
    book.push(IndexedMaster {
        kind: MasterKind::Ledger,
        guid: NEW_GUID.into(),
        alter_id: 8,
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
        posting: Posting::On,
        master_writes_enabled,
        audit_enabled: false,
    })
}

fn server_at(address: std::net::SocketAddr, directory: &std::path::Path) -> Server {
    server_with(address, directory, true)
}

/// A server whose answer cap is `max_bytes`.
fn server_capped(
    address: std::net::SocketAddr,
    directory: &std::path::Path,
    max_bytes: usize,
) -> Server {
    let mut server = server_at(address, directory);
    server.settings.max_bytes = max_bytes;
    server
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
        guid: crate::synthetic_guid(0xaaaa).into(),
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
    assert_eq!(
        master_writes_effective(Err(NotPresent), Posting::On),
        Ok(false)
    );
    for on in ["1", "true"] {
        assert_eq!(
            master_writes_effective(Ok(on.into()), Posting::On),
            Ok(true),
            "{on}"
        );
    }
    for off in ["0", "false"] {
        assert_eq!(
            master_writes_effective(Ok(off.into()), Posting::On),
            Ok(false),
            "{off}"
        );
    }
}

#[test]
fn the_master_writes_setting_is_validated_even_with_posting_off() {
    use std::env::VarError::NotPresent;
    for writes in [Posting::Off, Posting::On, Posting::RetiredSettingPresent] {
        for bad in ["yes", "TRUE", "", "2", " true"] {
            assert_eq!(
                master_writes_effective(Ok(bad.into()), writes),
                Err("boolean_setting_invalid".to_string()),
                "{bad:?} writes={writes:?}"
            );
        }
    }
    assert_eq!(
        master_writes_effective(Err(std::env::VarError::NotUnicode("x".into())), Posting::On),
        Err("boolean_setting_invalid".to_string())
    );
    let _ = NotPresent;
}

#[test]
fn master_writes_on_without_posting_is_ineffective() {
    for on in ["1", "true"] {
        assert_eq!(
            master_writes_effective(Ok(on.into()), Posting::Off),
            Ok(false)
        );
    }
}

/// Posting on with the retired batch setting still present makes `post_import`
/// refuse until it is removed; master writes stay off for as long.
#[test]
fn master_writes_stay_off_while_the_retired_batch_setting_is_present() {
    for on in ["1", "true"] {
        assert_eq!(
            master_writes_effective(Ok(on.into()), Posting::RetiredSettingPresent),
            Ok(false)
        );
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
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"post_ledger_creates","arguments":{"batch_id":format!("bridge-{}", crate::synthetic_guid(1))}}}),
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
        "target_mark_step_one",
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
            (crate::synthetic_guid(0x2222).to_string(), 3),
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
        .contains_key(crate::synthetic_guid(0xaaaa)));
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
    assert_eq!(verdict.state(), MasterVerdictState::PostedVerified);
    assert_eq!(verdict.items()[0].guid.as_deref(), Some(NEW_GUID));
    assert_eq!(verdict.items()[0].reason, None);
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
    // The note is the verdict's own, not "sent once, NOT verified".
    assert_eq!(
        result(&second)["note"],
        settled_note(MasterVerdictState::PostedVerified)
    );
    assert_ne!(result(&second)["note"], ALREADY_SENT_NOTE);
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
    // A book that was not read back says what to do, by what stopped it.
    let stopped = result(&response)["verification"]["error"]["code"]
        .as_str()
        .unwrap();
    assert_eq!(
        result(&response)["next_step"],
        not_read_back_next_step(stopped),
        "{response}"
    );
    assert!(scripted.masters().is_empty());
    assert_eq!(journal_bytes(directory.path()), before);
    // With an intent and no answer recorded the batch "may have been" sent.
    let note = result(&response)["note"].as_str().unwrap();
    assert!(note.contains("may have been sent"), "{note}");
}

/// A sent batch whose company is not among the loaded ones is not read back:
/// the answer says to load the same companies and call again, and it sends and
/// writes nothing.
#[tokio::test]
async fn a_sent_batch_whose_company_is_not_loaded_says_to_load_the_same_companies() {
    let simulator = SequenceSimulator::spawn(with_sentinel(verified_company())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let sent_batch = dispatched_batch_in(directory.path(), LEDGER, other_unloaded_guid());
    let before = journal_bytes(directory.path());
    let response = posting(
        &ScriptedApproval::approving(),
        server.call_tool(
            "post_ledger_creates",
            json!({"batch_id": sent_batch.batch_id}),
        ),
    )
    .await;
    let observed = sent(simulator);
    assert!(!create_was_sent(&observed, &expected_request()));
    let answered = result(&response);
    assert_eq!(answered["state"], "reconciliation_required", "{response}");
    assert_eq!(answered["verification"]["attempted"], false);
    assert_eq!(
        answered["verification"]["error"]["code"],
        "company_identity_not_found"
    );
    assert_eq!(answered["next_step"], verify::NEXT_LOAD_SAME_COMPANIES);
    assert_eq!(journal_bytes(directory.path()), before);
}

/// One row of the table of next steps: the step a path gives, the one it should
/// give, and whether it may tell the person to create the ledger by hand or must
/// tell them to create nothing by hand meanwhile.
struct StepRow {
    path: String,
    given: String,
    expected: String,
    may_key_by_hand: bool,
    keys_nothing: bool,
}

/// Every state and path an answer can take after a send, with its next step:
/// the verdicts judged and recorded, the verdicts judged and not recorded (with
/// and without Tally's answer on file), a book that was not read back, the
/// fallback after Tally answered, the failure answers, and the refusals for a
/// held name. Only the row marked may be the step that creates by hand; the
/// rows marked say to create nothing by hand meanwhile do say so.
#[test]
fn every_next_step_after_a_send_is_the_expected_one_and_only_one_creates_by_hand() {
    use super::verdict::{Checks, Judgement};
    use super::verify::unrecorded_next_step;
    use MasterVerdictState::{
        ConfirmedAbsent, MasterWriteLocatedElsewhere, MastersAfterPostUnconfirmed,
        PostedIntoExistingMaster, PostedVerified, ReconciliationRequired,
    };
    let judgement = |state: MasterVerdictState, found: bool, scope: bool, absent: bool| {
        Judgement::for_test(
            state,
            found.then(|| NEW_GUID.to_string()),
            Checks::none(),
            if scope {
                vec!["company_scope_changed"]
            } else {
                vec!["counters_unavailable"]
            },
            absent,
        )
    };
    let mut rows: Vec<StepRow> = Vec::new();
    let mut row = |path: &str, given: String, expected: &str, may: bool, nothing: bool| {
        rows.push(StepRow {
            path: path.to_string(),
            given,
            expected: expected.to_string(),
            may_key_by_hand: may,
            keys_nothing: nothing,
        });
    };
    // Judged and recorded.
    let judged = |j: &Judgement, moment| judged_next_step(j, moment).to_string();
    let r = Moment::Reconcile;
    let p = Moment::JustPosted;
    row(
        "verified",
        judged(&judgement(PostedVerified, true, false, false), p),
        verify::NEXT_VERIFIED,
        false,
        false,
    );
    row(
        "into existing",
        judged(&judgement(PostedIntoExistingMaster, false, false, false), r),
        verify::NEXT_INTO_EXISTING,
        false,
        false,
    );
    row(
        "elsewhere",
        judged(
            &judgement(MasterWriteLocatedElsewhere, false, false, true),
            r,
        ),
        verify::NEXT_ELSEWHERE,
        false,
        false,
    );
    row(
        "confirmed absent",
        judged(&judgement(ConfirmedAbsent, false, false, true), r),
        verify::NEXT_ABSENT,
        false,
        false,
    );
    row(
        "unconfirmed",
        judged(
            &judgement(MastersAfterPostUnconfirmed, false, false, false),
            r,
        ),
        verify::NEXT_UNCONFIRMED,
        false,
        true,
    );
    row(
        "scope changed",
        judged(&judgement(ReconciliationRequired, false, true, true), r),
        verify::NEXT_LOAD_SAME_COMPANIES,
        false,
        true,
    );
    row(
        "reconcile, name absent",
        judged(&judgement(ReconciliationRequired, false, false, true), r),
        verify::NEXT_ABSENT_NOT_SETTLED,
        false,
        false,
    );
    row(
        "reconcile, ledger present",
        judged(&judgement(ReconciliationRequired, true, false, false), r),
        verify::NEXT_PRESENT_NOT_VERIFIED,
        false,
        false,
    );
    row(
        "just posted, present",
        judged(&judgement(ReconciliationRequired, true, false, false), p),
        verify::NEXT_PRESENT_NOT_VERIFIED,
        false,
        false,
    );
    row(
        "just posted, name absent",
        judged(&judgement(ReconciliationRequired, false, false, true), p),
        verify::NEXT_RECONCILE,
        false,
        true,
    );
    row(
        "reconcile, index not usable",
        judged(&judgement(ReconciliationRequired, false, false, false), r),
        verify::NEXT_RECONCILE,
        false,
        true,
    );
    // Judged and not recorded: the quiet final states say only that the verdict
    // is not recorded; the others keep their own step first.
    let unrecorded = |j: &Judgement, moment, answer| unrecorded_next_step(j, moment, answer);
    let not_recorded = verify::NEXT_VERDICT_NOT_RECORDED;
    row(
        "unrecorded verified",
        unrecorded(&judgement(PostedVerified, true, false, false), p, true),
        not_recorded,
        false,
        false,
    );
    row(
        "unrecorded confirmed absent",
        unrecorded(&judgement(ConfirmedAbsent, false, false, true), r, true),
        not_recorded,
        false,
        false,
    );
    row(
        "unrecorded into existing",
        unrecorded(
            &judgement(PostedIntoExistingMaster, false, false, false),
            p,
            true,
        ),
        &format!("{} {not_recorded}", verify::NEXT_INTO_EXISTING),
        false,
        false,
    );
    row(
        "unrecorded unconfirmed",
        unrecorded(
            &judgement(MastersAfterPostUnconfirmed, false, false, false),
            p,
            true,
        ),
        &format!("{} {not_recorded}", verify::NEXT_UNCONFIRMED),
        false,
        true,
    );
    row(
        "unrecorded, name absent",
        unrecorded(
            &judgement(ReconciliationRequired, false, false, true),
            r,
            false,
        ),
        &format!("{} {not_recorded}", verify::NEXT_ABSENT_NOT_SETTLED),
        false,
        false,
    );
    row(
        "unrecorded, answer not on file",
        unrecorded(
            &judgement(MastersAfterPostUnconfirmed, false, false, false),
            p,
            false,
        ),
        &format!(
            "{} {not_recorded}{}",
            verify::NEXT_UNCONFIRMED,
            verify::NEXT_ANSWER_NOT_RECORDED
        ),
        false,
        true,
    );
    // The book was not read back.
    row(
        "not read back, company",
        not_read_back_next_step("company_identity_not_found").to_string(),
        verify::NEXT_LOAD_SAME_COMPANIES,
        false,
        true,
    );
    row(
        "not read back, record",
        not_read_back_next_step("master_batch_invalid").to_string(),
        verify::NEXT_RECORD_NOT_READABLE,
        false,
        false,
    );
    row(
        "not read back, tally",
        not_read_back_next_step("tally_endpoint_busy").to_string(),
        verify::NEXT_UNCONFIRMED,
        false,
        true,
    );
    // Tally answered and the call went no further.
    row(
        "fallback, answer on file",
        fallback_next_step(true),
        NEXT_NOT_READ_BACK_AFTER_ANSWER,
        false,
        true,
    );
    row(
        "fallback, answer not on file",
        fallback_next_step(false),
        &format!("{NEXT_NOT_READ_BACK_AFTER_ANSWER}{NEXT_ANSWER_NOT_RECORDED}"),
        false,
        true,
    );
    // The failure answers.
    let failure = |attempted| {
        master_failure_outcome(
            "bridge-x",
            GUID,
            ToolFailure::from("master_pre_image_write_failed".to_string()),
            empty_evidence(),
            attempted,
        )
        .payload["result"]["error"]["next_step"]
            .as_str()
            .unwrap_or("")
            .to_string()
    };
    row(
        "failure, may have been sent",
        failure(Some(true)),
        NEXT_UNCONFIRMED,
        false,
        true,
    );
    row(
        "failure, journal unread",
        failure(None),
        "Call post_ledger_creates with this batch_id again. If a send of this batch was recorded, that call only reads the book back and sends nothing; if none was, it asks for approval again before anything is sent; if ComplyEaze Bridge holds no such batch, it is refused as master_batch_not_found and nothing is sent. Do not create the ledger in Tally by hand meanwhile: a create that has not run yet would overwrite it.",
        false,
        true,
    );
    row(
        "failure, nothing sent",
        failure(Some(false)),
        NOTHING_RECORDED_STEP,
        false,
        false,
    );
    // An approval box that closed unanswered, or was declined: nothing was sent,
    // and the next call opens a new box.
    let unanswered = |code: &str| {
        master_failure_outcome(
            "bridge-x",
            GUID,
            ToolFailure::from(code.to_string()),
            empty_evidence(),
            Some(false),
        )
        .payload["result"]["error"]["next_step"]
            .as_str()
            .unwrap_or("")
            .to_string()
    };
    row(
        "approval timed out",
        unanswered("master_approval_timed_out"),
        "Nothing was sent: the approval box closed unanswered when its time ran out. To create the ledger, call post_ledger_creates with this batch_id again; that call opens a new approval box.",
        false,
        false,
    );
    row(
        "approval declined",
        unanswered("master_approval_declined"),
        "Nothing was sent: the approval was declined. If the ledger should still be created, call post_ledger_creates with this batch_id again; that call opens a new approval box.",
        false,
        false,
    );
    // The refusals for a held name.
    let one = BlockingBatch {
        batch_id: "bridge-0001".into(),
        company_guid: other_loaded_guid().into(),
    };
    let held_own = OverlapView::Held(BlockerReport {
        ids: vec!["bridge-0000".into()],
        other_companies: vec![],
    });
    let held_other = OverlapView::Held(BlockerReport {
        ids: vec![],
        other_companies: vec![one],
    });
    let held_both = OverlapView::Held(BlockerReport {
        ids: vec!["bridge-0000".into()],
        other_companies: vec![BlockingBatch {
            batch_id: "bridge-0001".into(),
            company_guid: other_loaded_guid().into(),
        }],
    });
    for (path, view, expected, keys) in [
        (
            "held, own company",
            held_own,
            format!("{OVERLAP_HOLD}{OVERLAP_OWN}{OVERLAP_THEN}"),
            true,
        ),
        (
            "held, other company",
            held_other,
            format!("{OVERLAP_HOLD}{OVERLAP_OTHER}{OVERLAP_THEN}"),
            true,
        ),
        (
            "held, both",
            held_both,
            format!("{OVERLAP_HOLD}{OVERLAP_OWN}{OVERLAP_OTHER}{OVERLAP_THEN}"),
            true,
        ),
        (
            "settled meanwhile",
            OverlapView::Settled,
            OVERLAP_SETTLED.to_string(),
            false,
        ),
        (
            "send recorded since",
            OverlapView::SendRecordedSince,
            NEXT_SEND_RECORDED_SINCE.to_string(),
            true,
        ),
        (
            "journal unread",
            OverlapView::Unread,
            OVERLAP_UNREAD.to_string(),
            true,
        ),
    ] {
        row(path, overlap_next_step(&view), &expected, false, keys);
        // The step is what the answer carries.
        let carried = overlap_refusal_outcome("bridge-x", GUID, empty_evidence(), &view).payload
            ["result"]["error"]["next_step"]
            .as_str()
            .unwrap()
            .to_string();
        assert_eq!(carried, expected, "{path}");
    }
    // The journal's own report of a batch, with and without a book read back.
    // (The answer to a call cancelled before its intent carries no next step,
    // only its note, and so is not a row.)
    let directory = tempfile::tempdir().unwrap();
    let held_batch = dispatched_batch_in(directory.path(), LEDGER, GUID);
    let open_state = state_of(directory.path(), &held_batch.batch_id);
    let settled_state = BatchState {
        verdict: Some(MasterVerdict::for_test(
            held_batch.batch_id.clone(),
            held_batch.batch_sha256.clone(),
            ConfirmedAbsent,
            vec![],
        )),
        ..open_state.clone()
    };
    let report_step = |state: &BatchState, code: Option<&str>| {
        Server::journal_report(state, empty_evidence(), code.map(str::to_string)).payload["result"]
            ["next_step"]
            .as_str()
            .unwrap_or("")
            .to_string()
    };
    rows.push(StepRow {
        path: "journal report, settled".into(),
        given: report_step(&settled_state, None),
        expected: verify::NEXT_ABSENT.into(),
        may_key_by_hand: false,
        keys_nothing: false,
    });
    rows.push(StepRow {
        path: "journal report, company not loaded".into(),
        given: report_step(&open_state, Some("company_identity_not_found")),
        expected: verify::NEXT_LOAD_SAME_COMPANIES.into(),
        may_key_by_hand: false,
        keys_nothing: true,
    });
    rows.push(StepRow {
        path: "journal report, no verdict, nothing stopped it".into(),
        given: report_step(&open_state, None),
        expected: String::new(),
        may_key_by_hand: false,
        keys_nothing: false,
    });
    assert!(rows.len() >= 34, "{} rows", rows.len());
    for row in &rows {
        assert_eq!(row.given, row.expected, "{}", row.path);
        assert_eq!(
            row.given.contains("create it in Tally by hand"),
            row.may_key_by_hand,
            "{}",
            row.path
        );
        assert_eq!(
            row.given
                .contains("Do not create the ledger in Tally by hand meanwhile"),
            row.keys_nothing,
            "{}",
            row.path
        );
    }
    assert_eq!(rows.iter().filter(|row| row.may_key_by_hand).count(), 0);
}

/// The README carries the corrected sentences about `post_ledger_creates`,
/// and neither it nor the tool's short description carries the sentences they
/// replaced. The description names its states and points to the guide, which
/// holds every state and field: the tool list goes to every client on every
/// session, so the full text lives in the guide.
#[test]
fn the_readme_and_the_tool_description_carry_the_corrected_sentences() {
    let flat = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let readme = flat(include_str!("../../docs/agent/README.md"));
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    let description = definitions
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "post_ledger_creates")
        .and_then(|tool| tool["description"].as_str())
        .map(flat)
        .expect("the tool is listed");
    for sentence in [
        "is not read back until the same companies are loaded under the same names; the next call then reads the book back and judges it.",
        "a failure answer does not. False means ComplyEaze Bridge has no record of Tally's answer on file",
        "A later call that finds no answer on file shows no counters.",
        "A later call judges the book as it then is, so a state stays unsettled only while the book stays as it was read.",
        "send_of_this_batch_recorded_since",
        "so a company_guid not found there belongs to a company that is not loaded",
    ] {
        let readme_sentence = sentence.replace("company_guid", "`company_guid`");
        assert!(
            readme.contains(sentence) || readme.contains(&readme_sentence),
            "README: {sentence}"
        );
    }
    for named in [
        "posted_verified",
        "posted_into_existing_master",
        "master_write_located_elsewhere",
        "confirmed_absent",
        "masters_after_post_unconfirmed",
        "posted_unverified",
        "reconciliation_required",
        "master_ledger_open_dispatch_overlaps",
        "never retry",
        "never create the ledger in Tally by hand",
        "never rebuild or resend a sent batch",
        "agent guide",
        "make sure Tally is idle",
        "never send the batch again",
    ] {
        assert!(description.contains(named), "description: {named}");
    }
    for replaced in [
        "is settled by loading the same companies and calling again",
        "and every answer that reads the book back, carries",
        "The answer to the call that sent the create",
        "Load that company in Tally",
    ] {
        assert!(!readme.contains(replaced), "README: {replaced}");
        assert!(!description.contains(replaced), "description: {replaced}");
    }
}

/// The guide says only what the read-back checks and where each answer comes
/// from, and the tool's short description agrees with it where it speaks. Code
/// names are compared with their backticks removed, since the guide marks them
/// and the description does not.
#[test]
fn the_guide_and_the_tool_description_say_only_what_the_read_back_checks() {
    let flat = |text: &str| {
        text.replace('`', "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let guide = flat(include_str!("../../docs/agent/README.md"));
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    let description = definitions
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "post_ledger_creates")
        .and_then(|tool| tool["description"].as_str())
        .map(flat)
        .expect("the tool is listed");
    for sentence in [
        // A send recorded since the call began: where the hold field is given.
        "When the check before the dialog finds a send of this batch recorded since the call began by another call, and when a held-name refusal finds it in the journal read that follows, the answer is import_already_attempted with error.hold send_of_this_batch_recorded_since",
        "The check under the journal's lock answers the same code without the hold field, as an outcome that may have been sent",
        // A company whose set of loaded companies changed is read and judged.
        "A book whose set of loaded companies changed is read and judged: the answer is reconciliation_required with company_scope_changed among its reasons, and it says to load the same companies and call again; a read that shows a changed existing master or another company's mark moved keeps the state for that.",
        // What posted_verified checks.
        "bill-wise is not read back",
        "no ledger or group that existed before changed",
        // What confirmed_absent needs, and what it does not show.
        "its read found the name absent and the company's master mark unmoved",
        "a create that Tally rejected reaches it too, and it does not show that Tally will not still run a create that ComplyEaze Bridge stopped waiting for",
        "make sure Tally is idle and answering",
        // The failure answers.
        "A failure answer that says the create may have been sent says the same.",
        "When the journal could not be read for that answer, a failure other than a refusal under the journal's lock made after the lock read the journal says it is not known whether a send was recorded: call post_ledger_creates with the same batch_id again, which reads the book back and sends nothing if a send is on file, otherwise asks for approval again before anything is sent, and is refused as master_batch_not_found, sending nothing, if ComplyEaze Bridge holds no such batch",
    ] {
        assert!(guide.contains(sentence), "guide: {sentence}");
    }
    assert!(guide.contains("no other loaded company's master mark moved"));
    assert!(description.contains("no other loaded company's did"));
    // A later call for a sent batch: what it does instead of the read-back.
    assert!(guide.contains(
        "A later call for a sent batch never sends. It repeats the read-back unless the batch already has a terminal verdict (it then answers from the journal alone), another dispatch is using the Tally endpoint"
    ));
    assert!(description.contains(
        "calling again reads the book back, unless another dispatch is using the Tally endpoint or the batch's company cannot be matched"
    ));
    for replaced in [
        "nothing else changed",
        "no existing master changed",
        "a later call found nothing created",
        "only from a later call after an unknown outcome",
        "or whose set of loaded companies changed is not read back",
        "and the failure answer that says the create may have been sent each say",
        "repeats the read-back and never sends",
        "every loaded company's master mark",
        "A send of this batch recorded since the call began by another call is answered as",
    ] {
        assert!(!guide.contains(replaced), "guide: {replaced}");
        assert!(!description.contains(replaced), "description: {replaced}");
    }
}

/// The refusal codes and answer fields the guide names for the ledger create
/// exist in the code that makes them, and the guide names each one.
#[test]
fn the_codes_and_fields_the_guide_names_exist_in_the_code() {
    let guide = include_str!("../../docs/agent/README.md").replace('`', "");
    let sources = [
        include_str!("agent_import_post.rs"),
        include_str!("agent_master_tools.rs"),
        include_str!("agent_master_verify.rs"),
        include_str!("tally/approved_import.rs"),
    ]
    .join("\n");
    for name in [
        "master_writes_need_licensed_mode",
        "group_export_invalid",
        "post_catalogue_unreadable",
        "master_approval_declined",
        "master_approval_unavailable",
        "master_approval_timed_out",
        "verdict_written",
        "verdict_record_error",
        "answer_record_error",
        "counters_note",
        "not_read_back",
        "endpoint_dispatch_busy",
        "company_scope_changed",
    ] {
        assert!(sources.contains(&format!("\"{name}\"")), "code: {name}");
        assert!(guide.contains(name), "guide: {name}");
    }
}

/// What the verdict texts say they checked is what `judge` checks: one new
/// ledger, the fields read back (not bill-wise), the ledgers and groups that
/// existed before, and the other companies' marks. The absent verdict says what
/// it needs and what it does not show.
#[test]
fn the_verified_and_absent_texts_say_what_is_checked_and_what_is_not() {
    for text in [
        settled_note(MasterVerdictState::PostedVerified),
        settled_next_step(MasterVerdictState::PostedVerified),
    ] {
        for fragment in [
            "exactly one ledger of this name, new",
            "bill-wise is not read back",
            "no ledger or group that existed before changed",
            "no other loaded company's master mark moved",
        ] {
            assert!(text.contains(fragment), "{fragment}: {text}");
        }
        for replaced in ["nothing else changed", "no other master or company changed"] {
            assert!(!text.contains(replaced), "{replaced}: {text}");
        }
    }
    for text in [
        settled_note(MasterVerdictState::ConfirmedAbsent),
        settled_next_step(MasterVerdictState::ConfirmedAbsent),
    ] {
        for fragment in [
            "no ledger of this name",
            "Tally rejected",
            "does not show that Tally will not still run a create that ComplyEaze Bridge stopped waiting for",
        ] {
            assert!(text.contains(fragment), "{fragment}: {text}");
        }
        for replaced in ["Nothing was created in the book", "found nothing created"] {
            assert!(!text.contains(replaced), "{replaced}: {text}");
        }
    }
    let next = settled_next_step(MasterVerdictState::ConfirmedAbsent);
    let idle = next.find("make sure Tally is idle and answering").unwrap();
    let prepare = next
        .find("call prepare_ledger_creates for a new batch")
        .unwrap();
    assert!(idle < prepare, "{next}");
    assert!(next.contains("later read"), "{next}");
}

/// What a book that could not be read back is told to do, by what stopped it.
#[test]
fn a_book_that_was_not_read_back_is_told_what_to_do_by_what_stopped_it() {
    for code in [
        "company_identity_not_found",
        "company_identity_ambiguous",
        "company_identity_incomplete",
        "company_identity_mismatch",
        "company_number_invalid",
    ] {
        assert_eq!(
            not_read_back_next_step(code),
            verify::NEXT_LOAD_SAME_COMPANIES,
            "{code}"
        );
    }
    for code in ["master_batch_invalid", "master_intent_missing"] {
        assert_eq!(
            not_read_back_next_step(code),
            verify::NEXT_RECORD_NOT_READABLE,
            "{code}"
        );
    }
    for code in [
        "tally_endpoint_busy",
        "read_failed",
        "import_admission_busy",
    ] {
        assert_eq!(
            not_read_back_next_step(code),
            verify::NEXT_UNCONFIRMED,
            "{code}"
        );
    }
    // None of them tells the person to key the ledger.
    for code in [
        "company_identity_mismatch",
        "master_batch_invalid",
        "read_failed",
    ] {
        assert!(!not_read_back_next_step(code).contains("create it in Tally by hand"));
    }
}

/// The step that tells the person to create the ledger by hand appears for one
/// kind of answer only: a read-back that read the book, found the name absent
/// and did not settle. It is never given for an answer that was not read, one
/// that holds the ledger, or one the loaded companies cannot be compared for.
#[test]
fn the_hand_key_step_appears_only_for_a_read_that_found_the_name_absent_and_did_not_settle() {
    use super::verdict::{Checks, Judgement};
    const STATES: [MasterVerdictState; 6] = [
        MasterVerdictState::PostedVerified,
        MasterVerdictState::ReconciliationRequired,
        MasterVerdictState::PostedIntoExistingMaster,
        MasterVerdictState::MastersAfterPostUnconfirmed,
        MasterVerdictState::MasterWriteLocatedElsewhere,
        MasterVerdictState::ConfirmedAbsent,
    ];
    let mut seen = std::collections::BTreeSet::new();
    for state in STATES {
        for moment in [Moment::JustPosted, Moment::Reconcile] {
            for absent in [false, true] {
                for found in [false, true] {
                    for scope_changed in [false, true] {
                        let judgement = Judgement::for_test(
                            state,
                            found.then(|| NEW_GUID.to_string()),
                            Checks::none(),
                            if scope_changed {
                                vec!["company_scope_changed"]
                            } else {
                                vec!["counters_unavailable"]
                            },
                            absent,
                        );
                        let step = judged_next_step(&judgement, moment);
                        let label =
                            format!("{state:?} {moment:?} {absent} {found} {scope_changed}");
                        let absent_open = step == verify::NEXT_ABSENT_NOT_SETTLED;
                        let expected = state == MasterVerdictState::ReconciliationRequired
                            && moment == Moment::Reconcile
                            && absent
                            && !scope_changed;
                        assert_eq!(absent_open, expected, "{label}");
                        seen.insert(step);
                    }
                }
            }
        }
    }
    // The states that were not judged on a read, or that hold the ledger, or
    // that were settled, have the step of their own.
    let step = |state: MasterVerdictState,
                found: bool,
                scope_changed: bool,
                absent: bool,
                moment: Moment| {
        judged_next_step(
            &Judgement::for_test(
                state,
                found.then(|| NEW_GUID.to_string()),
                Checks::none(),
                if scope_changed {
                    vec!["company_scope_changed"]
                } else {
                    vec![]
                },
                absent,
            ),
            moment,
        )
    };
    use MasterVerdictState::{MastersAfterPostUnconfirmed, ReconciliationRequired};
    assert_eq!(
        step(
            MastersAfterPostUnconfirmed,
            false,
            false,
            true,
            Moment::Reconcile
        ),
        verify::NEXT_UNCONFIRMED
    );
    assert_eq!(
        step(
            ReconciliationRequired,
            true,
            false,
            false,
            Moment::Reconcile
        ),
        verify::NEXT_PRESENT_NOT_VERIFIED
    );
    assert_eq!(
        step(
            ReconciliationRequired,
            true,
            false,
            false,
            Moment::JustPosted
        ),
        verify::NEXT_PRESENT_NOT_VERIFIED
    );
    assert_eq!(
        step(ReconciliationRequired, false, true, true, Moment::Reconcile),
        verify::NEXT_LOAD_SAME_COMPANIES
    );
    assert_eq!(
        step(
            ReconciliationRequired,
            false,
            false,
            true,
            Moment::JustPosted
        ),
        verify::NEXT_RECONCILE
    );
    assert_eq!(
        step(
            ReconciliationRequired,
            false,
            false,
            false,
            Moment::Reconcile
        ),
        verify::NEXT_RECONCILE
    );
    for state in [
        MasterVerdictState::PostedVerified,
        MasterVerdictState::PostedIntoExistingMaster,
        MasterVerdictState::MasterWriteLocatedElsewhere,
        MasterVerdictState::ConfirmedAbsent,
    ] {
        assert_eq!(
            step(state, false, false, true, Moment::Reconcile),
            settled_next_step(state)
        );
    }
    // No step any of them gives tells the person to create or key the ledger
    // by hand; the open absent state says the name stays held and what a later
    // call shows.
    for step in &seen {
        for hand in [
            "create it in Tally by hand",
            "keyed by hand",
            "Keying it by hand",
        ] {
            assert!(!step.contains(hand), "{step}");
        }
    }
    let open = verify::NEXT_ABSENT_NOT_SETTLED;
    assert!(open.contains("cannot settle it now") && open.contains("held"));
    assert!(open.contains("shows whether a ledger of this name now exists"));
}

/// Records an intent for `batch` as an earlier call would have.
fn record_intent_for_test(server: &Server, batch: &MasterBatch) {
    record_intent_in(&server.settings.data_dir, batch);
}

/// As [`record_intent_for_test`], for the journal in `dir`.
fn record_intent_in(dir: &std::path::Path, batch: &MasterBatch) {
    let lock = ExclusiveAdmission::acquire(dir).unwrap();
    let pre_image = PreImage::new([(crate::synthetic_guid(0xbbbb).to_string(), 1)]).unwrap();
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
            json!({"batch_id": format!("bridge-{}", crate::synthetic_guid(1))}),
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
        let pre_image = PreImage::new([(crate::synthetic_guid(0xbbbb).to_string(), 1)]).unwrap();
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
    // The check under the lock answers the code without the hold field, as an
    // outcome that may have been sent, with the step that reads it back.
    let error = &result(&response)["error"];
    assert!(error.get("hold").is_none(), "{response}");
    assert_eq!(result(&response)["attempt_recorded"], true, "{response}");
    assert!(error["message"]
        .as_str()
        .unwrap()
        .contains("may have been sent"));
    assert_eq!(error["next_step"], NEXT_UNCONFIRMED);
    assert_eq!(observed.len(), post_at, "every queue read, and no POST");
    // Only the other process's intent is in the journal.
    assert_eq!(
        record_types(directory.path()),
        ["master_batch", "master_dispatch_intent"]
    );
}

// --- post: a name an open dispatch holds -----------------------------------

fn other_loaded_guid() -> &'static str {
    crate::synthetic_guid(0x2222)
}
fn other_unloaded_guid() -> &'static str {
    crate::synthetic_guid(0x3333)
}

/// A batch of `name` prepared for the company `company_guid`, left as a call
/// that sent it would leave it: its intent is recorded and it has no verdict.
fn dispatched_batch_in(dir: &std::path::Path, name: &str, company_guid: &str) -> MasterBatch {
    let identity = identity();
    let item = item(name);
    let request = MasterPostRequest::render(item.clone(), &identity);
    let batch = MasterBatch {
        batch_id: format!("bridge-{}", Uuid::new_v4()),
        batch_sha256: request.sha256().to_string(),
        class: MasterClass::LedgerCreate,
        company: MasterCompany {
            guid: company_guid.to_string(),
            name: identity.display_name().to_string(),
            number: identity.company_number().to_string(),
            books_from: identity.books_from_yyyymmdd().to_string(),
        },
        items: vec![stored_from_item(&item)],
    };
    let lock = ExclusiveAdmission::acquire(dir).unwrap();
    append(dir, &lock, MasterRecord::Batch(batch.clone())).unwrap();
    let pre_image = PreImage::new([(crate::synthetic_guid(0xbbbb).to_string(), 1)]).unwrap();
    let written = write_pre_image(dir, &lock, &batch, &pre_image).unwrap();
    let intent = MasterDispatchIntent::new(
        &batch,
        "c".repeat(64),
        written,
        vec![super::super::master_ledger::MasterMark {
            company_guid: company_guid.into(),
            altmstid: 7,
        }],
    )
    .unwrap();
    let _ = append_intent(dir, &lock, intent).unwrap();
    batch
}

/// Rewrites the keys the journal recorded on a batch's intent, as an older
/// build's fold would have left them: a later fold never releases a name an
/// earlier one recorded.
fn rekey_intent(directory: &std::path::Path, batch_id: &str, canonical: &str, absence: &str) {
    let text = journal_text(directory);
    let rewritten: String = text
        .lines()
        .map(|line| {
            let mut record: Value = serde_json::from_str(line).unwrap();
            if record["record_type"] == "master_dispatch_intent" && record["batch_id"] == batch_id {
                record["items"][0]["canonical_key"] = json!(canonical);
                record["items"][0]["absence_key"] = json!(absence);
            }
            format!("{record}\n")
        })
        .collect();
    std::fs::write(journal_path(directory), rewritten).unwrap();
}

/// What a refusal shows when `own` hold the name in the refused batch's company
/// and `others` hold it in the company beside each.
fn held(own: &[&MasterBatch], others: &[(&MasterBatch, &str)]) -> OverlapView {
    OverlapView::Held(BlockerReport {
        ids: own.iter().map(|batch| batch.batch_id.clone()).collect(),
        other_companies: others
            .iter()
            .map(|(batch, company)| BlockingBatch {
                batch_id: batch.batch_id.clone(),
                company_guid: company.to_string(),
            })
            .collect(),
    })
}

/// Posts `args` with nothing but the reads before the dialog scripted: the
/// answer, how many requests reached the wire, whether any reached it with the
/// create's bytes, and the dialogs asked.
async fn refused_before_the_dialog(
    server: &Server,
    simulator: SequenceSimulator,
    args: Value,
) -> (Value, usize, bool, usize) {
    let scripted = ScriptedApproval::approving();
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    let create = create_was_sent(&observed, &expected_request());
    (response, observed.len(), create, scripted.masters().len())
}

/// A batch of the same ledger is sent and unsettled in the same company: the
/// post is refused before anyone is asked, on the journal it read just before the check,
/// as an answer of its own that names the batch.
#[tokio::test]
async fn a_name_held_by_an_open_dispatch_of_the_same_company_is_refused_before_anyone_is_asked() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let holder = dispatched_batch_in(directory.path(), LEDGER, GUID);
    let (second, args) = saved_batch(&server, LEDGER);
    let before = journal_bytes(directory.path());
    let (response, wire, create, dialogs) =
        refused_before_the_dialog(&server, simulator, args).await;
    assert_eq!(error_code(&response), OVERLAP_CODE, "{response}");
    let answered = result(&response);
    assert_eq!(answered["attempt_recorded"], false, "{response}");
    assert_eq!(answered["dispatch"]["state"], "not_dispatched");
    assert_eq!(answered["dispatch"]["resent"], false);
    let error = &answered["error"];
    assert_eq!(error["blocking_batch_ids"], json!([holder.batch_id]));
    assert_eq!(error["blocking_batches_other_companies"], json!([]));
    assert_eq!(error["hold"], "held");
    assert_eq!(
        error["next_step"],
        overlap_next_step(&held(&[&holder], &[]))
    );
    assert_eq!(dialogs, 0, "nobody was asked");
    assert_eq!(
        wire,
        before_approval().len(),
        "only the reads before the dialog"
    );
    assert!(!create, "nothing reached the wire with the create's bytes");
    assert_eq!(
        journal_bytes(directory.path()),
        before,
        "nothing was recorded"
    );
    assert!(!sidecar_of(directory.path(), &second.batch_id).exists());
}

/// A batch of another company holds the name too, loaded or not: it is listed
/// with its company, so the person can load that company and read it back, and
/// the refusal still comes before the dialog.
#[tokio::test]
async fn a_name_held_in_another_company_is_listed_with_its_company_loaded_or_not() {
    for other in [other_loaded_guid(), other_unloaded_guid()] {
        let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let holder = dispatched_batch_in(directory.path(), LEDGER, other);
        let (_, args) = saved_batch(&server, LEDGER);
        let (response, wire, create, dialogs) =
            refused_before_the_dialog(&server, simulator, args).await;
        assert_eq!(error_code(&response), OVERLAP_CODE, "{other}: {response}");
        let error = &result(&response)["error"];
        assert_eq!(error["blocking_batch_ids"], json!([]), "{other}");
        assert_eq!(
            error["blocking_batches_other_companies"],
            json!([{"batch_id": holder.batch_id, "company_guid": other}]),
            "{other}"
        );
        assert_eq!(
            error["next_step"],
            overlap_next_step(&held(&[], &[(&holder, other)])),
            "{other}"
        );
        assert_eq!(result(&response)["attempt_recorded"], false);
        assert_eq!(dialogs, 0, "{other}: nobody was asked");
        assert_eq!(wire, before_approval().len(), "{other}");
        assert!(!create, "{other}");
    }
}

/// Three batches hold the name, the most the journal admits for one name, by
/// the three ways a name is held: those of this company are all named, in batch
/// id order, and one of another company is listed apart with its company.
#[tokio::test]
async fn three_open_dispatches_holding_a_name_are_told_apart_by_company() {
    let alpha = "Holder Alpha";
    let beta = "Holder Beta";
    for (second_company, ids_of_both) in [(GUID, true), (other_loaded_guid(), false)] {
        let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let first = dispatched_batch_in(directory.path(), alpha, GUID);
        let second = dispatched_batch_in(directory.path(), beta, second_company);
        // Held by its name alone: the same letters and digits as the new name.
        let third = dispatched_batch_in(directory.path(), "ZetaTestLedger", GUID);
        // The first two recorded one of the two keys the new name has; none of
        // the three overlaps another by name or by key, so the journal admits
        // all three.
        rekey_intent(
            directory.path(),
            &first.batch_id,
            &canonical_master_key(LEDGER),
            "alpha-absence",
        );
        rekey_intent(
            directory.path(),
            &second.batch_id,
            "beta-canonical",
            &absence_key(LEDGER),
        );
        assert!(read_journal(directory.path()).is_ok());
        let (_, args) = saved_batch(&server, LEDGER);
        let (response, wire, _, dialogs) =
            refused_before_the_dialog(&server, simulator, args).await;
        assert_eq!(error_code(&response), OVERLAP_CODE, "{response}");
        let error = &result(&response)["error"];
        let mut expected = vec![first.batch_id.clone(), third.batch_id.clone()];
        if ids_of_both {
            expected.push(second.batch_id.clone());
        }
        expected.sort();
        assert_eq!(error["blocking_batch_ids"], json!(expected), "{response}");
        let others = if ids_of_both {
            json!([])
        } else {
            json!([{"batch_id": second.batch_id, "company_guid": second_company}])
        };
        assert_eq!(error["blocking_batches_other_companies"], others);
        assert_eq!(dialogs, 0);
        assert_eq!(wire, before_approval().len());
    }
}

/// The ids a refusal names are the same company's, and those of other companies
/// are listed apart with their company, whichever case the guid is written in.
#[test]
fn the_ids_a_refusal_names_are_the_same_companys_and_the_rest_are_listed_apart() {
    let blocking = |n: usize, company: &str| BlockingBatch {
        batch_id: format!("bridge-{n:04}"),
        company_guid: company.to_string(),
    };
    let held = [
        blocking(0, GUID),
        blocking(1, other_loaded_guid()),
        blocking(2, GUID),
        blocking(3, other_unloaded_guid()),
    ];
    let told = BlockerReport::of(&held, GUID);
    assert_eq!(told.ids, ["bridge-0000", "bridge-0002"]);
    let mut error = json!({});
    told.add_to(&mut error);
    assert_eq!(
        error,
        json!({"blocking_batch_ids": ["bridge-0000", "bridge-0002"],
               "blocking_batches_other_companies": [
                   {"batch_id": "bridge-0001", "company_guid": other_loaded_guid()},
                   {"batch_id": "bridge-0003", "company_guid": other_unloaded_guid()}]})
    );
    let shouted = BlockerReport::of(&held[..1], &GUID.to_uppercase());
    assert_eq!(shouted.ids, ["bridge-0000"]);
    assert!(shouted.other_companies.is_empty());
}

/// The next step of each case of the refusal, by what the journal showed.
#[test]
fn each_case_of_the_refusal_for_a_held_name_says_what_to_do_for_what_it_found() {
    let batch = |n: usize| BlockingBatch {
        batch_id: format!("bridge-{n:04}"),
        company_guid: other_loaded_guid().into(),
    };
    let own = OverlapView::Held(BlockerReport {
        ids: vec!["bridge-0000".into()],
        other_companies: Vec::new(),
    });
    let others = OverlapView::Held(BlockerReport {
        ids: Vec::new(),
        other_companies: vec![batch(1)],
    });
    let both = OverlapView::Held(BlockerReport {
        ids: vec!["bridge-0000".into()],
        other_companies: vec![batch(1)],
    });
    let views = [
        own,
        others,
        both,
        OverlapView::Settled,
        OverlapView::SendRecordedSince,
        OverlapView::Unread,
    ];
    let steps: Vec<String> = views.iter().map(overlap_next_step).collect();
    for (n, step) in steps.iter().enumerate() {
        assert!(!step.contains("machine-wide"), "{n}");
        assert!(
            !step.contains("create it in Tally by hand"),
            "{n}: no hand step for a held name"
        );
        for other in &steps[n + 1..] {
            assert_ne!(step, other, "{n}");
        }
    }
    // The held name: settle the holders, this batch again, and key nothing.
    for held in &steps[..3] {
        assert!(held.contains("every company this data folder holds batches for"));
        assert!(held.contains("call post_ledger_creates with this batch_id again"));
        assert!(held.contains("Do not create the ledger in Tally by hand meanwhile"));
    }
    assert!(steps[0].contains("blocking_batch_ids") && !steps[0].contains("another company"));
    assert!(steps[1].contains("list_companies shows only the companies loaded in Tally"));
    assert!(steps[1].contains("company_guid") && !steps[1].contains("blocking_batch_ids"));
    assert!(steps[2].contains("blocking_batch_ids") && steps[2].contains("list_companies"));
    // The other cases.
    assert_eq!(steps[3], OVERLAP_SETTLED);
    assert_eq!(steps[4], NEXT_SEND_RECORDED_SINCE);
    assert_eq!(steps[5], OVERLAP_UNREAD);
    assert!(steps[5].contains("Do not create the ledger in Tally by hand meanwhile"));
}

/// The approval a person gave to an earlier call is withdrawn when the name is
/// held at the call that redeems it: the second dialog is never asked and the
/// POST is never sent.
#[tokio::test]
async fn an_approval_given_earlier_is_withdrawn_when_the_name_is_held_at_the_redeeming_call() {
    let mut plans = before_approval();
    plans.extend(before_approval());
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
    scripted.answer(true);
    until_answered(&server, &batch.batch_id).await;
    assert!(server.post_approvals.holds(&batch.batch_id));
    // A rival's create of the same ledger is sent before the click is redeemed.
    let holder = dispatched_batch_in(directory.path(), LEDGER, GUID);
    let refused = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(error_code(&refused), OVERLAP_CODE, "{refused}");
    assert_eq!(result(&refused)["attempt_recorded"], false);
    assert_eq!(
        result(&refused)["error"]["blocking_batch_ids"],
        json!([holder.batch_id])
    );
    assert_eq!(scripted.masters().len(), 1, "one dialog, asked once");
    assert!(
        !server.post_approvals.holds(&batch.batch_id),
        "the held approval was withdrawn"
    );
    assert_eq!(
        observed.len(),
        2 * before_approval().len(),
        "no queue read, no POST"
    );
    assert!(!create_was_sent(&observed, &expected_request()));
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_none());
}

/// A rival's create is sent while the dialog is open: the check before the
/// dialog could not see it, and the one under the lock refuses. The approval is
/// not spent, no sidecar is written, no POST is sent, and the answer names the
/// batch from one read of the journal.
#[tokio::test]
async fn an_open_dispatch_recorded_while_the_dialog_is_open_refuses_under_the_lock_and_names_it() {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (second, args) = saved_batch(&server, LEDGER);
    let data_dir = directory.path().to_path_buf();
    let holder = std::sync::Arc::new(std::sync::Mutex::new(None::<MasterBatch>));
    let recorded = holder.clone();
    let scripted = ScriptedApproval::approving_after(move || {
        *recorded.lock().unwrap() = Some(dispatched_batch_in(&data_dir, LEDGER, GUID));
    });
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    let holder = holder
        .lock()
        .unwrap()
        .clone()
        .expect("the rival was recorded");
    assert_eq!(error_code(&response), OVERLAP_CODE, "{response}");
    let answered = result(&response);
    assert_eq!(answered["attempt_recorded"], false, "{response}");
    assert_eq!(answered["dispatch"]["state"], "not_dispatched");
    let error = &answered["error"];
    assert_eq!(error["blocking_batch_ids"], json!([holder.batch_id]));
    assert_eq!(error["blocking_batches_other_companies"], json!([]));
    assert_eq!(error["hold"], "held");
    assert_eq!(
        error["next_step"],
        overlap_next_step(&held(&[&holder], &[]))
    );
    assert_eq!(scripted.masters().len(), 1, "the person was asked once");
    assert_eq!(observed.len(), post_at, "every queue read, and no POST");
    assert!(!sidecar_of(directory.path(), &second.batch_id).exists());
    assert!(state_of(directory.path(), &second.batch_id)
        .intent
        .is_none());
}

/// The same refusal under the lock when the journal is busy for the lookup that
/// names the batches: the answer is the journal the lock read when it refused,
/// which names the holder, never "not known" (#2083).
#[tokio::test]
async fn the_refusal_under_the_lock_answers_from_what_the_lock_read_when_its_lookup_finds_the_journal_busy(
) {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server, LEDGER);
    let holder = std::sync::Arc::new(std::sync::Mutex::new(None::<MasterBatch>));
    let (data_dir, recorded) = (directory.path().to_path_buf(), holder.clone());
    let scripted = ScriptedApproval::approving_after(move || {
        *recorded.lock().unwrap() = Some(dispatched_batch_in(&data_dir, LEDGER, GUID));
    });
    let (response, unused) = faulting(
        vec!["failure_read_busy"],
        posting(&scripted, server.call_tool("post_ledger_creates", args)),
    )
    .await;
    sent(simulator);
    assert!(unused.is_empty(), "the lookup was reached");
    assert_eq!(error_code(&response), OVERLAP_CODE, "{response}");
    let answered = result(&response);
    // The lock read the journal with the holder's send and none of this
    // batch's: this call sent nothing, and the holder is named.
    assert_eq!(answered["attempt_recorded"], false, "{response}");
    assert_eq!(answered["dispatch"]["state"], "not_dispatched");
    let error = &answered["error"];
    assert_eq!(error["hold"], "held");
    let holder = holder
        .lock()
        .unwrap()
        .clone()
        .expect("the holder was recorded");
    assert_eq!(error["blocking_batch_ids"], json!([holder.batch_id]));
    assert_eq!(error["blocking_batches_other_companies"], json!([]));
    assert!(error["next_step"]
        .as_str()
        .unwrap()
        .starts_with(OVERLAP_HOLD));
    assert!(!error["message"]
        .as_str()
        .unwrap()
        .contains("may have been sent"));
    // As of the refusal: true whether or not a send of this batch was
    // recorded since.
    assert_eq!(error["message"], KEPT_HELD_NAME_MESSAGE);
}

/// Settles `holder` as a read-back that found nothing does.
fn settle_holder(dir: &std::path::Path, holder: &MasterBatch) {
    let lock = ExclusiveAdmission::acquire(dir).unwrap();
    append(
        dir,
        &lock,
        MasterRecord::Verdict(MasterVerdict::for_test(
            holder.batch_id.clone(),
            holder.batch_sha256.clone(),
            MasterVerdictState::ConfirmedAbsent,
            vec![VerdictItem {
                name: LEDGER.into(),
                guid: None,
                reason: Some("not_in_book".into()),
            }],
        )),
    )
    .unwrap();
}

/// Runs a post whose dialog is approved while a holder is recorded, and in
/// which `before_read` runs just before the refusal under the lock reads the
/// journal to say why. Returns the answer, the dialogs asked and the requests
/// that reached the wire, with the number the queue's reads come to.
async fn refused_under_the_lock(
    before_read: impl Fn(&std::path::Path, &MasterBatch, &MasterBatch) + Send + Sync + 'static,
) -> (Value, usize, usize, usize) {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server, LEDGER);
    let holder = std::sync::Arc::new(std::sync::Mutex::new(None::<MasterBatch>));
    let (data_dir, recorded) = (directory.path().to_path_buf(), holder.clone());
    let scripted = ScriptedApproval::approving_after(move || {
        *recorded.lock().unwrap() = Some(dispatched_batch_in(&data_dir, LEDGER, GUID));
    });
    let (data_dir, seen) = (directory.path().to_path_buf(), holder.clone());
    let hook: std::sync::Arc<dyn Fn() + Send + Sync> = std::sync::Arc::new(move || {
        let holder = seen
            .lock()
            .unwrap()
            .clone()
            .expect("the holder was recorded");
        before_read(&data_dir, &batch_for_hook(&data_dir), &holder);
    });
    let response = master_ledger::BEFORE_FAILURE_READ
        .scope(
            hook,
            posting(&scripted, server.call_tool("post_ledger_creates", args)),
        )
        .await;
    let observed = sent(simulator);
    (response, scripted.masters().len(), observed.len(), post_at)
}

/// The one prepared batch of the journal in `dir` that has no intent.
fn batch_for_hook(dir: &std::path::Path) -> MasterBatch {
    let journal = read_journal(dir).unwrap();
    let text = journal_text(dir);
    let id = text
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|record| record["record_type"] == "master_batch")
        .filter_map(|record| record["batch_id"].as_str().map(str::to_string))
        .find(|id| {
            journal
                .state(id)
                .is_some_and(|state| state.intent.is_none())
        })
        .expect("a batch without an intent");
    journal.state(&id).unwrap().batch.clone()
}

/// The holder that made the lock refuse settles before the answer reads the
/// journal: the answer says so and to post this batch again, with no batch named.
#[tokio::test]
async fn a_holder_that_settles_before_the_failure_read_says_to_post_again() {
    let (response, dialogs, wire, post_at) =
        refused_under_the_lock(|dir, _, holder| settle_holder(dir, holder)).await;
    assert_eq!(error_code(&response), OVERLAP_CODE, "{response}");
    let answered = result(&response);
    assert_eq!(answered["attempt_recorded"], false, "{response}");
    assert_eq!(answered["dispatch"]["state"], "not_dispatched");
    let error = &answered["error"];
    assert_eq!(error["hold"], "settled_meanwhile");
    assert_eq!(error["next_step"], OVERLAP_SETTLED);
    for field in ["blocking_batch_ids", "blocking_batches_other_companies"] {
        assert!(error.get(field).is_none(), "{field}");
    }
    assert_eq!((dialogs, wire), (1, post_at), "asked once, no POST");
}

/// The refusal under the lock, and a send of this batch is recorded by the time
/// the answer reads the journal: it is answered as the check under the lock
/// answers a batch already attempted, the call sent nothing, the send may have
/// happened, and the step is to read it back later.
#[tokio::test]
async fn a_refusal_when_a_send_of_this_batch_was_recorded_since_is_the_already_attempted_answer() {
    let (response, dialogs, wire, post_at) = refused_under_the_lock(|dir, batch, holder| {
        settle_holder(dir, holder);
        record_intent_in(dir, batch);
    })
    .await;
    assert_eq!(
        error_code(&response),
        "import_already_attempted",
        "{response}"
    );
    let answered = result(&response);
    assert_eq!(answered["attempt_recorded"], true, "{response}");
    assert_eq!(answered["dispatch"]["state"], "reconciliation_required");
    let error = &answered["error"];
    assert_eq!(error["hold"], "send_of_this_batch_recorded_since");
    assert_eq!(error["next_step"], NEXT_SEND_RECORDED_SINCE);
    assert_eq!(
        error["message"],
        "This call sent nothing. A send of this batch has been recorded since by another call; it may have been sent."
    );
    assert!(error.get("blocking_batch_ids").is_none());
    assert_eq!((dialogs, wire), (1, post_at), "asked once, no POST");
}

/// The same before the dialog: a send of this batch lands while the pass reads
/// Tally. It is answered as already attempted (not as a held name), the person
/// is not asked, and nothing reaches the wire after the reads.
#[tokio::test]
async fn a_send_of_this_batch_recorded_while_the_pass_reads_is_refused_before_the_dialog() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let data_dir = directory.path().to_path_buf();
    let sent_batch = batch.clone();
    let hook: std::sync::Arc<dyn Fn() + Send + Sync> =
        std::sync::Arc::new(move || record_intent_in(&data_dir, &sent_batch));
    let scripted = ScriptedApproval::approving();
    let response = master_ledger::BEFORE_HOLD_CHECK
        .scope(
            hook,
            posting(&scripted, server.call_tool("post_ledger_creates", args)),
        )
        .await;
    let observed = sent(simulator);
    assert_eq!(
        error_code(&response),
        "import_already_attempted",
        "{response}"
    );
    let answered = result(&response);
    assert_eq!(answered["attempt_recorded"], true);
    assert_eq!(answered["dispatch"]["state"], "reconciliation_required");
    assert_eq!(
        answered["error"]["hold"],
        "send_of_this_batch_recorded_since"
    );
    assert_eq!(answered["error"]["next_step"], NEXT_SEND_RECORDED_SINCE);
    assert!(scripted.masters().is_empty(), "nobody was asked");
    assert_eq!(observed.len(), before_approval().len());
    assert!(!create_was_sent(&observed, &expected_request()));
}

/// The read before the dialog fails: the check is skipped, as the post went
/// before it existed, and the check under the lock decides. The call adds no
/// failure, and the approval given earlier is not withdrawn by that read: the
/// redeeming call goes on to the queue and is refused under the lock.
#[tokio::test]
async fn a_failed_read_before_the_dialog_skips_that_check_and_the_lock_decides() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
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
    scripted.answer(true);
    until_answered(&server, &batch.batch_id).await;
    let holder = dispatched_batch_in(directory.path(), LEDGER, GUID);
    let (refused, unused) = faulting(
        vec!["hold_check_read_busy"],
        posting(&scripted, server.call_tool("post_ledger_creates", args)),
    )
    .await;
    let observed = sent(simulator);
    assert!(unused.is_empty(), "the read before the dialog was reached");
    // Not a failure of that read: the redeem went on to the queue's reads, and
    // the check under the lock refused it, naming the holder.
    assert_eq!(error_code(&refused), OVERLAP_CODE, "{refused}");
    assert_eq!(result(&refused)["error"]["hold"], "held");
    assert_eq!(
        result(&refused)["error"]["blocking_batch_ids"],
        json!([holder.batch_id])
    );
    assert_eq!(observed.len(), post_at, "every queue read, and no POST");
    assert_eq!(scripted.masters().len(), 1, "one dialog, asked once");
}

/// A rival's send lands while the pass reads Tally, after it first read the
/// journal: the person is not asked.
#[tokio::test]
async fn a_rival_that_appears_while_the_pass_reads_is_refused_before_the_dialog() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server, LEDGER);
    let data_dir = directory.path().to_path_buf();
    let rival = std::sync::Arc::new(std::sync::Mutex::new(None::<MasterBatch>));
    let recorded = rival.clone();
    let hook: std::sync::Arc<dyn Fn() + Send + Sync> = std::sync::Arc::new(move || {
        *recorded.lock().unwrap() = Some(dispatched_batch_in(&data_dir, LEDGER, GUID));
    });
    let scripted = ScriptedApproval::approving();
    let response = master_ledger::BEFORE_HOLD_CHECK
        .scope(
            hook,
            posting(&scripted, server.call_tool("post_ledger_creates", args)),
        )
        .await;
    let observed = sent(simulator);
    let rival = rival.lock().unwrap().clone().expect("the hook ran");
    assert_eq!(error_code(&response), OVERLAP_CODE, "{response}");
    assert_eq!(
        result(&response)["error"]["blocking_batch_ids"],
        json!([rival.batch_id])
    );
    assert!(scripted.masters().is_empty(), "nobody was asked");
    assert_eq!(observed.len(), before_approval().len());
}

/// A holder settles while the pass reads Tally: the check is made on the journal
/// as it is then, so the person is asked.
#[tokio::test]
async fn a_holder_that_settles_while_the_pass_reads_does_not_refuse() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let holder = dispatched_batch_in(directory.path(), LEDGER, GUID);
    let (_, args) = saved_batch(&server, LEDGER);
    let data_dir = directory.path().to_path_buf();
    let hook: std::sync::Arc<dyn Fn() + Send + Sync> =
        std::sync::Arc::new(move || settle_holder(&data_dir, &holder));
    let scripted = ScriptedApproval::declining();
    let response = master_ledger::BEFORE_HOLD_CHECK
        .scope(
            hook,
            posting(&scripted, server.call_tool("post_ledger_creates", args)),
        )
        .await;
    sent(simulator);
    assert_eq!(
        error_code(&response),
        "master_approval_declined",
        "{response}"
    );
    assert_eq!(scripted.masters().len(), 1, "the person was asked");
}

/// A holder that settles lets the same batch post: the refusal was about the
/// holder, not the batch.
#[tokio::test]
async fn a_holder_that_settles_lets_the_same_batch_post() {
    let (plans, post_at) = pending_then_posted_plans();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let holder = dispatched_batch_in(directory.path(), LEDGER, GUID);
    let (batch, args) = saved_batch(&server, LEDGER);
    let scripted = ScriptedApproval::approving();
    let refused = posting(
        &scripted,
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert_eq!(error_code(&refused), OVERLAP_CODE, "{refused}");
    assert!(scripted.masters().is_empty());
    // The holder's outcome is settled, as a read-back that found nothing does.
    {
        let lock = ExclusiveAdmission::acquire(directory.path()).unwrap();
        append(
            directory.path(),
            &lock,
            MasterRecord::Verdict(MasterVerdict::for_test(
                holder.batch_id.clone(),
                holder.batch_sha256.clone(),
                MasterVerdictState::ConfirmedAbsent,
                vec![VerdictItem {
                    name: LEDGER.into(),
                    guid: None,
                    reason: Some("not_in_book".into()),
                }],
            )),
        )
        .unwrap();
    }
    let posted = posting_then(
        &scripted,
        book_after(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    let observed = sent(simulator);
    assert_eq!(result(&posted)["state"], "posted_verified", "{posted}");
    assert!(observed.len() > post_at, "the POST was sent");
    assert_eq!(
        scripted.masters().len(),
        1,
        "asked once, after the holder settled"
    );
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_some());
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
    assert_eq!(
        create_count(&observed),
        0,
        "no request carried the create's body"
    );
    assert!(!String::from_utf8(journal_bytes(directory.path()))
        .unwrap()
        .contains("master_dispatch_intent"));
}

// --- post: the dispatch intent's own append, and a journal that is corrupt ---

/// What a refusal of the journal that recorded nothing tells the caller to do.
const NOTHING_RECORDED_STEP: &str = "Nothing was sent and nothing was recorded for this batch. Call prepare_ledger_creates for a new batch and ask for approval again.";

/// The journal refuses the dispatch intent before a byte of it is written (the
/// pre-checks make this unreachable without a fault, so it is armed): nothing is
/// recorded, the sidecar this call wrote is removed, the create is not sent, the
/// answer says nothing was sent, and the approval is spent, so a later call asks
/// the person again.
#[tokio::test]
async fn an_intent_the_journal_refuses_before_writing_records_nothing_and_sends_nothing() {
    // The first call's reads up to the POST, which it never makes, then the
    // reads a second call makes before its dialog.
    let mut plans = through_post(xml(created_one()));
    let post_at = plans.len() - 1;
    plans.pop();
    plans.extend(before_approval());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let before = journal_bytes(directory.path());
    let scripted = ScriptedApproval::approving();
    let (response, unused) = faulting(
        vec!["master_dispatch_intent"],
        posting(
            &scripted,
            server.call_tool("post_ledger_creates", args.clone()),
        ),
    )
    .await;
    assert!(unused.is_empty(), "the fault was not reached");
    assert_eq!(simulator.received(), post_at, "every queue read, no POST");
    assert_eq!(error_code(&response), "master_ledger_invalid", "{response}");
    let answered = result(&response);
    assert_eq!(answered["attempt_recorded"], false, "{response}");
    assert_eq!(answered["dispatch"]["state"], "not_dispatched");
    assert_eq!(answered["dispatch"]["resent"], false);
    assert_eq!(answered["error"]["next_step"], NOTHING_RECORDED_STEP);
    assert_eq!(scripted.masters().len(), 1, "the person was asked once");
    // Nothing on disk: no intent, no sidecar.
    assert_eq!(journal_bytes(directory.path()), before);
    assert!(!sidecar_of(directory.path(), &batch.batch_id).exists());
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_none());
    // The approval was spent under the lock before the append, and is not left
    // to be redeemed: a later call asks again, and sends nothing before that.
    assert!(!server.post_approvals.holds(&batch.batch_id));
    let again = ScriptedApproval::declining();
    let second = posting(&again, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(error_code(&second), "master_approval_declined", "{second}");
    assert_eq!(
        result(&second)["error"]["next_step"],
        "Nothing was sent: the approval was declined. If the ledger should still be created, call post_ledger_creates with this batch_id again; that call opens a new approval box."
    );
    assert_eq!(again.masters().len(), 1, "asked again");
    assert!(!create_was_sent(&observed, &expected_request()));
    assert_eq!(observed.len(), post_at + before_approval().len());
}

/// A failure after which the intent may be in the file keeps no view of the
/// journal for its answer (#2083): with the answer's own read busy too, the
/// answer does not know whether a send was recorded, and never says that
/// nothing was sent.
#[tokio::test]
async fn an_intent_that_may_be_in_the_file_is_never_answered_unsent_when_the_answer_read_is_busy() {
    let mut plans = through_post(xml(created_one()));
    plans.pop();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let (response, unused) = faulting(
        vec![
            "master_dispatch_intent_lost_after_write",
            "failure_read_busy",
        ],
        posting(
            &ScriptedApproval::approving(),
            server.call_tool("post_ledger_creates", args),
        ),
    )
    .await;
    let observed = sent(simulator);
    assert!(unused.is_empty(), "a fault was not reached: {unused:?}");
    assert!(!create_was_sent(&observed, &expected_request()));
    assert_eq!(
        error_code(&response),
        "import_dispatch_outcome_unknown",
        "{response}"
    );
    let answered = result(&response);
    assert!(answered["attempt_recorded"].is_null(), "{response}");
    assert_eq!(answered["dispatch"]["state"], "reconciliation_required");
    assert_eq!(answered["error"]["next_step"], NEXT_SEND_NOT_KNOWN);
    assert!(!answered["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Nothing was sent"));
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_some());
}

/// The dispatch intent is written and the append is then reported as failed (a
/// directory sync that did not hold), so it may be in the file: this call does
/// not send, the batch is treated as dispatched with the step that reads it
/// back, the sidecar its digest names is kept, and a later call reconciles it
/// and never sends.
#[tokio::test]
async fn an_intent_that_may_be_in_the_file_is_treated_as_sent_and_a_later_call_never_sends() {
    let mut plans = through_post(xml(created_one()));
    let post_at = plans.len() - 1;
    plans.pop();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let scripted = ScriptedApproval::approving();
    let (response, unused) = faulting(
        vec!["master_dispatch_intent_lost_after_write"],
        posting(
            &scripted,
            server.call_tool("post_ledger_creates", args.clone()),
        ),
    )
    .await;
    let observed = sent(simulator);
    assert!(unused.is_empty(), "the fault was not reached");
    assert_eq!(observed.len(), post_at, "every queue read, no POST");
    assert!(
        !create_was_sent(&observed, &expected_request()),
        "no request carried the create's body"
    );
    assert_eq!(
        error_code(&response),
        "import_dispatch_outcome_unknown",
        "{response}"
    );
    let answered = result(&response);
    assert_eq!(answered["attempt_recorded"], true, "{response}");
    assert_eq!(answered["dispatch"]["state"], "reconciliation_required");
    assert_eq!(answered["dispatch"]["resent"], false);
    assert_eq!(answered["error"]["next_step"], NEXT_UNCONFIRMED);
    assert_eq!(scripted.masters().len(), 1, "the person was asked once");
    assert!(!server.post_approvals.holds(&batch.batch_id));
    // The intent is in the file, with its sidecar.
    assert_eq!(
        record_types(directory.path()),
        ["master_batch", "master_dispatch_intent"]
    );
    assert!(sidecar_of(directory.path(), &batch.batch_id).exists());
    // A later call reads the book back, asks nobody and sends nothing; the book
    // is as it was and no mark moved since the intent.
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(7, 3, None))).unwrap();
    let later = server_at(reads.address(), directory.path());
    let again = ScriptedApproval::approving();
    let report = posting(&again, later.call_tool("post_ledger_creates", args)).await;
    let observed = sent(reads);
    assert!(!create_was_sent(&observed, &expected_request()));
    assert!(again.masters().is_empty(), "nobody was asked");
    let reported = result(&report);
    assert_eq!(reported["state"], "confirmed_absent", "{report}");
    assert_eq!(reported["verification"]["moment"], "reconcile");
    assert_eq!(reported["attempt_recorded"], true);
    assert_eq!(reported["dispatch"]["resent"], false);
    assert_eq!(
        record_types(directory.path()),
        ["master_batch", "master_dispatch_intent", "master_verdict"]
    );
}

/// Invalid bytes at the end of the masters journal: a record cut short, with no
/// newline, which its reader refuses.
fn corrupt_journal(directory: &std::path::Path) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(journal_path(directory))
        .unwrap();
    file.write_all(CORRUPT_TAIL).unwrap();
}

const CORRUPT_TAIL: &[u8] = br#"{"record_type":"master_dispatch_intent","batch_id":"bridge-"#;

/// The journal is corrupt before the call starts: the first read of it refuses
/// the call before anything is read from Tally, asked or sent.
#[tokio::test]
async fn a_corrupt_journal_stops_the_post_at_its_first_read_before_anything_else() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    corrupt_journal(directory.path());
    let before = journal_bytes(directory.path());
    let scripted = ScriptedApproval::approving();
    let response = posting(&scripted, server.call_tool("post_ledger_creates", args)).await;
    let observed = sent(simulator);
    assert_eq!(error_code(&response), "master_ledger_invalid", "{response}");
    // A refusal before the pass has a batch to speak of: it claims nothing
    // either way about a send.
    for field in ["attempt_recorded", "dispatch"] {
        assert!(result(&response).get(field).is_none(), "{response}");
    }
    assert!(observed.is_empty(), "nothing reached the wire");
    assert!(scripted.masters().is_empty(), "nobody was asked");
    assert_eq!(journal_bytes(directory.path()), before);
    assert!(!sidecar_of(directory.path(), &batch.batch_id).exists());
}

/// The journal turns corrupt while the pass reads Tally, so the read just before
/// the dialog fails for a reason other than a busy lock. That check is skipped,
/// as the guide says, the person is asked, and the check under the lock refuses
/// before the intent: no create is on the wire and nothing is recorded.
#[tokio::test]
async fn a_corrupt_journal_at_the_check_before_the_dialog_is_skipped_and_the_lock_stops_the_post() {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let prepared = journal_bytes(directory.path());
    let data_dir = directory.path().to_path_buf();
    let hook: std::sync::Arc<dyn Fn() + Send + Sync> =
        std::sync::Arc::new(move || corrupt_journal(&data_dir));
    let scripted = ScriptedApproval::approving();
    let response = master_ledger::BEFORE_HOLD_CHECK
        .scope(
            hook,
            posting(&scripted, server.call_tool("post_ledger_creates", args)),
        )
        .await;
    let observed = sent(simulator);
    // Any read error under the lock is the one code, whatever the journal's own.
    assert_eq!(
        error_code(&response),
        "master_ledger_unavailable",
        "{response}"
    );
    assert_eq!(scripted.masters().len(), 1, "the check was skipped");
    assert_eq!(observed.len(), post_at, "every queue read, and no POST");
    assert!(!create_was_sent(&observed, &expected_request()));
    assert!(!sidecar_of(directory.path(), &batch.batch_id).exists());
    // The journal is as the corruption left it: nothing was written after it.
    assert_eq!(
        journal_bytes(directory.path()),
        [prepared, CORRUPT_TAIL.to_vec()].concat()
    );
    // The failure answer reads the same corrupt journal, so it cannot say that
    // no send is recorded. It does not claim one either: the count is null (not
    // known), never true, and the step is the one that is true in both cases.
    // (This call itself sent nothing; the answer speaks for the batch.)
    let answered = result(&response);
    assert!(answered["attempt_recorded"].is_null(), "{response}");
    assert_eq!(answered["error"]["next_step"], NEXT_SEND_NOT_KNOWN);
    assert!(!server.post_approvals.holds(&batch.batch_id));
}

/// One of two posts made at once.
struct RacingPost {
    response: Value,
    /// How many requests carried the create's body.
    creates: usize,
    /// How many requests reached the wire.
    wire: usize,
    /// How many times the person was asked.
    dialogs: usize,
    /// Whether the approval the call took is still held for redemption.
    approval_held: bool,
    /// The armed faults the call never reached.
    faults_unused: Vec<&'static str>,
}

/// Posts `batch_id` from `data_dir` on a thread and a runtime of its own, as a
/// second process would, with its own Tally endpoint (`plans`) and its own
/// approval. It waits at `gate` just before the check that precedes the dialog,
/// so that both posts pass that check before either records an intent.
/// `while_asked` runs while the dialog is open, and `faults` are armed.
fn racing_post(
    data_dir: std::path::PathBuf,
    batch_id: String,
    plans: Vec<ScenarioPlan>,
    gate: std::sync::Arc<std::sync::Barrier>,
    while_asked: impl Fn() + Send + Sync + 'static,
    faults: Vec<&'static str>,
) -> std::thread::JoinHandle<RacingPost> {
    std::thread::spawn(move || {
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let server = server_at(simulator.address(), &data_dir);
        let scripted = ScriptedApproval::approving_after(while_asked);
        let waited = std::sync::atomic::AtomicBool::new(false);
        let hook: std::sync::Arc<dyn Fn() + Send + Sync> = std::sync::Arc::new(move || {
            if !waited.swap(true, std::sync::atomic::Ordering::SeqCst) {
                gate.wait();
            }
        });
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let (response, faults_unused) = runtime.block_on(faulting(
            faults,
            master_ledger::BEFORE_HOLD_CHECK.scope(
                hook,
                posting_then(
                    &scripted,
                    book_after(),
                    server.call_tool("post_ledger_creates", json!({"batch_id": batch_id})),
                ),
            ),
        ));
        let observed = sent(simulator);
        RacingPost {
            response,
            creates: create_count(&observed),
            wire: observed.len(),
            dialogs: scripted.masters().len(),
            approval_held: server.post_approvals.holds(&batch_id),
            faults_unused,
        }
    })
}

/// Two posts for one ledger name that both pass the check before the dialog:
/// `winner` is answered first and sends the create; `loser` is let through its
/// dialog only once the winner's intent is on file and its lock released, so
/// which call wins does not depend on timing. Tally's answer to the winner's
/// POST is held until the loser has answered, so the winner's dispatch is open,
/// and the winner writes nothing to the journal, for the whole of the loser's
/// call: the race is not run against a short timer (#2083). It is bounded by
/// the request timeout of the winner's held POST, the simulator's limit on a
/// held answer and the test's own 20-second wait for the winner's intent. `same_batch` is whether both name one
/// batch id or two batches of the same name in the same company; `loser_faults`
/// are armed in the loser's call.
struct Race {
    winner: RacingPost,
    loser: RacingPost,
    winner_batch: MasterBatch,
    loser_batch: MasterBatch,
    directory: tempfile::TempDir,
}

fn race_two_posts(same_batch: bool, loser_faults: Vec<&'static str>) -> Race {
    let directory = tempfile::tempdir().unwrap();
    let preparer = unit_server(directory.path());
    let (winner_batch, _) = saved_batch(&preparer, LEDGER);
    let loser_batch = if same_batch {
        winner_batch.clone()
    } else {
        saved_batch(&preparer, LEDGER).0
    };
    let answer = tally_protocol_simulator::DeliveryGate::new();
    let mut winning =
        through_post(xml(created_one()).with_delivery(Delivery::Held(answer.clone())));
    winning.extend(clean_read_back());
    // The loser's POST is scripted too, so that a wrongly sent one is counted.
    let losing = through_post(xml(created_one()));
    let gate = std::sync::Arc::new(std::sync::Barrier::new(2));
    let first = racing_post(
        directory.path().to_path_buf(),
        winner_batch.batch_id.clone(),
        winning,
        gate.clone(),
        || {},
        Vec::new(),
    );
    let data_dir = directory.path().to_path_buf();
    let second = racing_post(
        directory.path().to_path_buf(),
        loser_batch.batch_id.clone(),
        losing,
        gate,
        move || {
            let started = std::time::Instant::now();
            while !(String::from_utf8_lossy(&journal_bytes(&data_dir))
                .contains("master_dispatch_intent")
                && ExclusiveAdmission::acquire(&data_dir).is_ok())
            {
                assert!(started.elapsed() < Duration::from_secs(20), "no intent");
                std::thread::sleep(Duration::from_millis(5));
            }
        },
        loser_faults,
    );
    let loser = second.join().unwrap();
    answer.open();
    Race {
        winner: first.join().unwrap(),
        loser,
        winner_batch,
        loser_batch,
        directory,
    }
}

/// What both races must show, whichever batch the rival names: the winner sent
/// the create once and was verified, the loser sent nothing and recorded
/// nothing, and the one create is the only one on either wire.
fn assert_one_create_and_a_refused_loser(race: &Race) {
    let (winner, loser) = (&race.winner, &race.loser);
    assert_eq!(winner.dialogs, 1);
    assert_eq!(loser.dialogs, 1, "both passed the check before the dialog");
    assert_eq!(winner.creates + loser.creates, 1, "one create on the wire");
    assert_eq!(winner.creates, 1);
    assert_eq!(loser.creates, 0);
    assert_eq!(result(&winner.response)["state"], "posted_verified");
    // The loser reached the lock after every queue read, and sent no POST.
    assert_eq!(loser.wire, through_post(xml(created_one())).len() - 1);
    // Its approval was taken and not spent: it lapses, and cannot be redeemed.
    assert!(!loser.approval_held, "the loser's approval is still held");
    assert!(!winner.approval_held);
    assert_eq!(
        record_types(race.directory.path())
            .iter()
            .filter(|kind| *kind == "master_dispatch_intent")
            .count(),
        1,
        "one intent in the journal"
    );
}

/// The loser's answer is what the check under the lock decided on, whether or
/// not its own read of the journal for the answer finds the journal busy: a
/// busy read falls back to the journal the lock read (#2083).
#[test]
fn two_batches_of_one_name_through_the_check_together_send_one_create_and_the_other_is_held_off() {
    for faults in [vec![], vec!["failure_read_busy"]] {
        two_batches_race(faults);
    }
}

fn two_batches_race(loser_faults: Vec<&'static str>) {
    let busy = !loser_faults.is_empty();
    let race = race_two_posts(false, loser_faults);
    assert!(
        race.loser.faults_unused.is_empty(),
        "the busy read was not reached"
    );
    assert_one_create_and_a_refused_loser(&race);
    assert_eq!(
        error_code(&race.loser.response),
        OVERLAP_CODE,
        "{}",
        race.loser.response
    );
    let answered = result(&race.loser.response);
    assert_eq!(answered["attempt_recorded"], false);
    assert_eq!(answered["dispatch"]["state"], "not_dispatched");
    assert_eq!(answered["error"]["hold"], "held");
    // A busy answer read is answered as of the refusal.
    if busy {
        assert_eq!(answered["error"]["message"], KEPT_HELD_NAME_MESSAGE);
    } else {
        assert_eq!(
            answered["error"]["message"],
            "Nothing was sent. An earlier create holds this ledger name."
        );
    }
    assert_eq!(
        answered["error"]["blocking_batch_ids"],
        json!([race.winner_batch.batch_id])
    );
    assert!(!sidecar_of(race.directory.path(), &race.loser_batch.batch_id).exists());
    assert!(state_of(race.directory.path(), &race.loser_batch.batch_id)
        .intent
        .is_none());
    assert!(state_of(race.directory.path(), &race.winner_batch.batch_id)
        .intent
        .is_some());
}

/// As for two batches: a busy read for the answer falls back to the journal
/// the lock read, which holds this batch's send (#2083).
#[test]
fn the_same_batch_through_the_check_together_twice_sends_one_create_and_the_other_reads_it_back() {
    for faults in [vec![], vec!["failure_read_busy"]] {
        same_batch_race(faults);
    }
}

fn same_batch_race(loser_faults: Vec<&'static str>) {
    let race = race_two_posts(true, loser_faults);
    assert!(
        race.loser.faults_unused.is_empty(),
        "the busy read was not reached"
    );
    assert_one_create_and_a_refused_loser(&race);
    assert_eq!(
        error_code(&race.loser.response),
        "import_already_attempted",
        "{}",
        race.loser.response
    );
    let answered = result(&race.loser.response);
    // The check under the lock: a send of this batch is on file, so it may
    // have been sent, and the step is to read it back.
    assert_eq!(answered["attempt_recorded"], true);
    assert_eq!(answered["dispatch"]["state"], "reconciliation_required");
    assert!(answered["error"].get("hold").is_none());
    assert_eq!(answered["error"]["next_step"], NEXT_UNCONFIRMED);
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
    // The create may have been sent: a later call reads, and nothing is keyed.
    assert_eq!(result(&response)["error"]["next_step"], NEXT_UNCONFIRMED);
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

/// A failed post whose own journal read also fails does not know whether a send
/// was recorded. Its step is a later call for the same batch. Here a send was
/// recorded (the POST was lost after its intent): that call reads the book back
/// and sends nothing, and nobody is asked.
#[tokio::test]
async fn a_failure_answer_with_an_unread_journal_says_to_call_again_and_a_recorded_send_is_only_read(
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
    let (first, unused) = faulting(
        vec!["failure_read_busy"],
        posting(
            &ScriptedApproval::approving(),
            server.call_tool("post_ledger_creates", args.clone()),
        ),
    )
    .await;
    assert!(unused.is_empty(), "the journal read was not reached");
    assert_eq!(create_count(&sent(simulator)), 1);
    assert_eq!(
        error_code(&first),
        "import_dispatch_outcome_unknown",
        "{first}"
    );
    let answered = result(&first);
    assert!(answered["attempt_recorded"].is_null(), "{first}");
    assert_eq!(answered["error"]["next_step"], NEXT_SEND_NOT_KNOWN);
    assert_ne!(answered["error"]["next_step"], NEXT_UNCONFIRMED);
    assert!(answered["error"]["message"]
        .as_str()
        .unwrap()
        .contains("not known"));
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_some());
    // The call the step names: the send is on file, so it reads and sends nothing.
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(7, 3, None))).unwrap();
    let later = server_at(reads.address(), directory.path());
    let scripted = ScriptedApproval::approving();
    let second = posting_then(
        &scripted,
        book(),
        later.call_tool("post_ledger_creates", args),
    )
    .await;
    let observed = sent(reads);
    assert_eq!(create_count(&observed), 0, "the later call sends no create");
    assert!(!observed.is_empty(), "the later call read the book back");
    assert!(scripted.masters().is_empty(), "nobody was asked");
    assert_eq!(result(&second)["verification"]["moment"], "reconcile");
    assert_eq!(result(&second)["dispatch"]["resent"], false);
}

/// The same step when no send was recorded (the wire was busy before the
/// intent): the later call asks for approval again, and nothing reaches the wire
/// when it is declined.
#[tokio::test]
async fn a_failure_answer_with_an_unread_journal_says_to_call_again_and_no_send_asks_for_approval()
{
    let mut plans = before_approval();
    let queue = after_approval(xml(created_one()));
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
    let (first, unused) = faulting(
        vec!["failure_read_busy"],
        posting(
            &ScriptedApproval::approving(),
            server.call_tool("post_ledger_creates", args.clone()),
        ),
    )
    .await;
    assert!(unused.is_empty(), "the journal read was not reached");
    assert_eq!(create_count(&sent(simulator)), 0, "no POST");
    assert_eq!(error_code(&first), "tally_endpoint_busy", "{first}");
    let answered = result(&first);
    assert!(answered["attempt_recorded"].is_null(), "{first}");
    assert_eq!(answered["error"]["next_step"], NEXT_SEND_NOT_KNOWN);
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_none());
    // The call the step names: no send is on file, so it asks the person again
    // before anything is sent.
    let again = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let later = server_at(again.address(), directory.path());
    let scripted = ScriptedApproval::declining();
    let second = posting(&scripted, later.call_tool("post_ledger_creates", args)).await;
    let observed = sent(again);
    assert_eq!(error_code(&second), "master_approval_declined", "{second}");
    assert_eq!(scripted.masters().len(), 1, "the person was asked again");
    assert_eq!(create_count(&observed), 0, "nothing was sent");
    assert_eq!(observed.len(), before_approval().len());
    assert!(state_of(directory.path(), &batch.batch_id).intent.is_none());
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
        pre_image: [(crate::synthetic_guid(0xaaaa).to_string(), 9)].into(),
        marks: vec![super::super::master_ledger::MasterMark {
            company_guid: GUID.into(),
            altmstid: 7,
        }],
    }))
}

fn unit_server(directory: &std::path::Path) -> Server {
    server_at("127.0.0.1:9".parse().unwrap(), directory)
}

/// The check under the lock keeps what it read only when it refuses: a
/// recorded intent keeps nothing (a later failure must not be answered from a
/// journal read before the intent), and a refusal keeps whether a send was on
/// file and what holds the name (#2083).
#[test]
fn the_check_under_the_lock_keeps_its_view_only_when_it_refuses() {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, _) = saved_batch(&server, LEDGER);
    let request = expected_request();
    let refused = |result: Result<RecordedIntent, BeforeDispatchError>| match result {
        Err(BeforeDispatchError::Refused(refusal)) => refusal,
        other => panic!("expected a named refusal, got {other:?}"),
    };
    let kept = Mutex::new(None);
    let approval = server.post_approvals.redeeming_for_test(&batch.batch_id);
    let _licence = server
        .record_master_intent(&batch, &request, approval, &aim_for_test(), &kept)
        .expect("recorded");
    assert!(
        kept.lock().unwrap().is_none(),
        "a recorded intent keeps nothing"
    );
    // The same batch again: a send of it is on file.
    let again = server.post_approvals.redeeming_for_test(&batch.batch_id);
    assert_eq!(
        refused(server.record_master_intent(&batch, &request, again, &aim_for_test(), &kept)),
        UnderLockRefusal::AlreadyAttempted
    );
    let view = kept
        .lock()
        .unwrap()
        .take()
        .expect("the refusal kept its view");
    assert!(view.attempted);
    assert_eq!(view.overlap, OverlapView::SendRecordedSince);
    // Another batch of the same name: the first one's open send holds it.
    let (rival, _) = saved_batch(&server, LEDGER);
    let approval = server.post_approvals.redeeming_for_test(&rival.batch_id);
    assert_eq!(
        refused(server.record_master_intent(&rival, &request, approval, &aim_for_test(), &kept)),
        UnderLockRefusal::MasterOverlapsOpenDispatch
    );
    let view = kept
        .lock()
        .unwrap()
        .take()
        .expect("the refusal kept its view");
    assert!(!view.attempted);
    match view.overlap {
        OverlapView::Held(held) => assert_eq!(held.ids, vec![batch.batch_id.clone()]),
        other => panic!("{other:?}"),
    }
}

/// Only an answer made from the journal the lock read, that says no send was
/// on file, gets the message that is true as of the refusal; every field and
/// every other answer is left as it is (#2083).
#[test]
fn only_an_answer_from_the_kept_view_that_shows_no_send_is_worded_as_of_the_refusal() {
    let answer = |attempted: Option<bool>| {
        master_failure_outcome(
            "batch",
            GUID,
            ToolFailure::from("import_approval_revoked".to_string()),
            empty_evidence(),
            attempted,
        )
    };
    let kept = as_of_refusal(answer(Some(false)), true, false);
    assert_eq!(
        kept.payload["result"]["error"]["message"],
        KEPT_NOT_SENT_MESSAGE
    );
    let mut expected = answer(Some(false)).payload;
    expected["result"]["error"]["message"] = json!(KEPT_NOT_SENT_MESSAGE);
    assert_eq!(kept.payload, expected, "only the message changes");
    for (attempted, from_view) in [(Some(false), false), (Some(true), true), (None, true)] {
        assert_eq!(
            as_of_refusal(answer(attempted), from_view, false).payload,
            answer(attempted).payload,
            "{attempted:?} {from_view}"
        );
    }
}

#[test]
fn before_dispatch_records_the_sidecar_and_the_intent_and_spends_the_approval() {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, _) = saved_batch(&server, LEDGER);
    let approval = server.post_approvals.redeeming_for_test(&batch.batch_id);
    let request = expected_request();
    let licence = server
        .record_master_intent(
            &batch,
            &request,
            approval,
            &aim_for_test(),
            &Mutex::new(None),
        )
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
        refused(server.record_master_intent(
            &batch,
            &request,
            approval,
            &Mutex::new(None),
            &Mutex::new(None)
        )),
        UnderLockRefusal::PreImageUnrecorded
    );
    // An empty pre-image is a failed read, not an empty book.
    let empty = Mutex::new(Some(MasterAim {
        pre_image: Default::default(),
        marks: aim_for_test().lock().unwrap().clone().unwrap().marks,
    }));
    assert_eq!(
        refused(server.record_master_intent(&batch, &request, approval, &empty, &Mutex::new(None))),
        UnderLockRefusal::PreImageUnrecorded
    );
    // A batch the caller holds that is not the journal's.
    let mut changed = batch.clone();
    changed.batch_sha256 = "d".repeat(64);
    assert_eq!(
        refused(server.record_master_intent(
            &changed,
            &request,
            approval,
            &aim_for_test(),
            &Mutex::new(None)
        )),
        UnderLockRefusal::BatchChanged
    );
    let mut renamed = batch.clone();
    renamed.items[0].name = "Another Name".into();
    assert_eq!(
        refused(server.record_master_intent(
            &renamed,
            &request,
            approval,
            &aim_for_test(),
            &Mutex::new(None)
        )),
        UnderLockRefusal::BatchChanged
    );
    // A batch the journal does not hold.
    let mut unknown = batch.clone();
    unknown.batch_id = format!("bridge-{}", Uuid::new_v4());
    assert_eq!(
        refused(server.record_master_intent(
            &unknown,
            &request,
            approval,
            &aim_for_test(),
            &Mutex::new(None)
        )),
        UnderLockRefusal::BatchNotFound
    );
    // An approval that is not the one taken (revoked, or never taken).
    assert_eq!(
        refused(server.record_master_intent(
            &batch,
            &request,
            Uuid::new_v4(),
            &aim_for_test(),
            &Mutex::new(None)
        )),
        UnderLockRefusal::ApprovalRevoked
    );
    assert_eq!(journal_bytes(directory.path()), before);
    assert!(!sidecar.exists(), "nothing was written for any refusal");

    // A dispatched batch is never recorded twice.
    record_intent_for_test(&server, &batch);
    let after_first = journal_bytes(directory.path());
    assert_eq!(
        refused(server.record_master_intent(
            &batch,
            &request,
            approval,
            &aim_for_test(),
            &Mutex::new(None)
        )),
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
    let result = server.record_master_intent(
        &second,
        &expected_request(),
        approval,
        &aim_for_test(),
        &Mutex::new(None),
    );
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
                company_guid: format!("{n:08x}-{:04x}-4{:03x}-8{:03x}-{:012x}", 0, 0, 0, 0),
                altmstid: 1,
            }),
        );
    }
    let before = journal_bytes(directory.path());
    let result = server.record_master_intent(
        &batch,
        &expected_request(),
        approval,
        &aim,
        &Mutex::new(None),
    );
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

/// A post withdrawn by its caller is answered in the three cases a failure is:
/// no send recorded, a send recorded, and a journal that cannot say. Only the
/// second says the create may have been sent; the third does not claim it was
/// not sent either, and gives the step that is true whichever it was.
#[test]
fn a_cancelled_post_is_answered_for_nothing_sent_a_send_recorded_and_not_known() {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let cancelled = |args: &Value| {
        let outcome = server
            .cancelled_post_for_response("post_ledger_creates", args)
            .unwrap();
        outcome.payload["result"].clone()
    };
    // Nothing sent: no step, as a failure answer with no remedy of its own.
    let nothing = cancelled(&args);
    assert_eq!(nothing["attempt_recorded"], false);
    assert_eq!(nothing["dispatch"]["state"], "not_dispatched");
    assert_eq!(nothing["dispatch"]["resent"], false);
    assert_eq!(nothing["error"]["code"], "request_cancelled");
    assert_eq!(nothing["error"]["message"], CANCELLED_NOT_SENT_MESSAGE);
    assert!(nothing["error"].get("next_step").is_none());
    // A send recorded: the step that reads the book back.
    record_intent_for_test(&server, &batch);
    let recorded = cancelled(&args);
    assert_eq!(recorded["attempt_recorded"], true);
    assert_eq!(recorded["dispatch"]["state"], "reconciliation_required");
    assert_eq!(recorded["dispatch"]["resent"], false);
    assert_eq!(recorded["error"]["code"], "request_cancelled");
    assert_eq!(
        recorded["error"]["message"],
        CANCELLED_SEND_RECORDED_MESSAGE
    );
    assert_eq!(recorded["error"]["next_step"], NEXT_UNCONFIRMED);
    // Not known, for a journal that cannot be read and for a batch it lacks.
    corrupt_journal(directory.path());
    let unread = cancelled(&args);
    let unknown_batch = {
        let other = tempfile::tempdir().unwrap();
        let other_server = unit_server(other.path());
        saved_batch(&other_server, "Other Ledger");
        other_server
            .cancelled_post_for_response("post_ledger_creates", &args)
            .unwrap()
            .payload["result"]
            .clone()
    };
    for not_known in [&unread, &unknown_batch] {
        assert!(not_known["attempt_recorded"].is_null(), "{not_known}");
        assert_eq!(not_known["dispatch"]["state"], "reconciliation_required");
        assert_eq!(not_known["dispatch"]["resent"], false);
        assert_eq!(not_known["error"]["code"], "request_cancelled");
        assert_eq!(
            not_known["error"]["message"],
            CANCELLED_SEND_NOT_KNOWN_MESSAGE
        );
        assert_eq!(not_known["error"]["next_step"], NEXT_SEND_NOT_KNOWN);
    }
    // The three are three answers.
    assert_ne!(CANCELLED_NOT_SENT_MESSAGE, CANCELLED_SEND_RECORDED_MESSAGE);
    assert_ne!(
        CANCELLED_SEND_RECORDED_MESSAGE,
        CANCELLED_SEND_NOT_KNOWN_MESSAGE
    );
    assert_ne!(CANCELLED_NOT_SENT_MESSAGE, CANCELLED_SEND_NOT_KNOWN_MESSAGE);
    assert_ne!(NEXT_UNCONFIRMED, NEXT_SEND_NOT_KNOWN);
}

/// A post whose company read is cut at the request limit, as on a gateway kept busy by another
/// call's long read (#2123), names that as its cause and says to post again once Tally is idle,
/// through the whole tool answer: the code stays the company read's, nothing was sent and no
/// intent was recorded.
#[tokio::test]
async fn a_post_whose_company_read_is_cut_at_the_limit_names_a_busy_tally() {
    let mut plans = verified_company();
    plans[0] = plans[0]
        .clone()
        .with_delivery(Delivery::SlowHeaders(Duration::from_millis(1_500)));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    server.runtime = crate::tally::runtime::TallyRuntime::with_transport_policy(
        bridge_tally_transport::TransportPolicy {
            request_timeout: Duration::from_millis(300),
            ..Default::default()
        },
    );
    let (batch, args) = saved_batch(&server, LEDGER);
    let response = posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args),
    )
    .await;
    let observed = sent(simulator);
    let error = &result(&response)["error"];
    assert_eq!(error["code"], "company_collection_invalid", "{response}");
    assert_eq!(error["cause"], "request_deadline_exceeded", "{response}");
    assert_eq!(error["next_step"], NEXT_TALLY_BUSY, "{response}");
    assert_eq!(result(&response)["attempt_recorded"], false, "{response}");
    assert_eq!(
        result(&response)["dispatch"]["state"],
        "not_dispatched",
        "{response}"
    );
    assert!(!create_was_sent(&observed, &expected_request()));
    assert!(
        !journal_text(directory.path()).contains("master_dispatch_intent"),
        "no intent for {}",
        batch.batch_id
    );
}

/// A request cut at the limit is named as the cause whatever was sent, but the busy-gateway step,
/// which says nothing was sent, is given only when the journal shows nothing sent: a cut create
/// (a send recorded) keeps the read-back step, and a journal that could not be read keeps the
/// step that is true either way.
#[test]
fn the_busy_tally_step_is_given_only_when_nothing_was_sent() {
    let answer = |attempted: Option<bool>| {
        let failure = ToolFailure::from_runtime(
            "post_queue_read_failed",
            anyhow::Error::new(bridge_tally_transport::TallyTransportError::RequestTimedOut),
        );
        let outcome =
            master_failure_outcome("bridge-batch", GUID, failure, empty_evidence(), attempted);
        outcome.payload["result"]["error"].clone()
    };
    let nothing_sent = answer(Some(false));
    let send_recorded = answer(Some(true));
    let not_known = answer(None);
    for error in [&nothing_sent, &send_recorded, &not_known] {
        assert_eq!(error["cause"], "request_deadline_exceeded", "{error}");
    }
    assert_eq!(nothing_sent["next_step"], NEXT_TALLY_BUSY);
    assert_eq!(send_recorded["next_step"], NEXT_UNCONFIRMED);
    assert_eq!(not_known["next_step"], NEXT_SEND_NOT_KNOWN);
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
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"post_ledger_creates","arguments":{"batch_id":format!("bridge-{}", crate::synthetic_guid(1))}}}),
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
        .record_master_intent(
            &batch,
            request,
            approval,
            &aim_for_test(),
            &Mutex::new(None),
        )
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
        guid: cash_guid().into(),
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
    assert_eq!(result(&response)["verification"]["guid"], cash_guid());
    assert_eq!(result(&response)["verification"]["settled"], true);
    assert!(result(&response)["verification"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|reason| reason == "name_resolves_to_existing_master"));
    let state = state_of(directory.path(), &batch.batch_id);
    assert_eq!(
        state.verdict.as_ref().map(|verdict| verdict.state()),
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
            .map(|verdict| verdict.state()),
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
    for (target, other, held, expect, next_step) in [
        // Read, the name absent, and not settled: the name stays held.
        (
            8,
            3,
            book(),
            "reconciliation_required",
            verify::NEXT_ABSENT_NOT_SETTLED,
        ),
        (
            7,
            4,
            book(),
            "master_write_located_elsewhere",
            verify::NEXT_ELSEWHERE,
        ),
        // A ledger of the name is there, without counters: nothing to key.
        (
            7,
            3,
            book_after(),
            "reconciliation_required",
            verify::NEXT_PRESENT_NOT_VERIFIED,
        ),
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
        assert_eq!(result(&response)["next_step"], next_step, "{response}");
        let _ = batch;
    }
}

#[tokio::test]
async fn an_answer_that_cannot_be_recorded_is_reported_as_answered_not_as_maybe_sent() {
    // Tally answers, and another process holds the journal's lock from the POST
    // until the call is over: neither the answer nor a verdict can be recorded.
    // The read-back is still made, from the journal read without its lock, and
    // the call says Tally answered, with its counters.
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
    let (finished, over) = tokio::sync::oneshot::channel::<()>();
    let post = async {
        let response = posting(&approving, server.call_tool("post_ledger_creates", args)).await;
        let _ = finished.send(());
        response
    };
    let hold_lock = async {
        let started = std::time::Instant::now();
        // Once the POST has arrived the intent's use of the lock is over: hold
        // it until the call has answered.
        while simulator.received() <= post_at {
            assert!(
                started.elapsed() < Duration::from_secs(20),
                "the POST never arrived"
            );
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        let lock = ExclusiveAdmission::acquire(&data_dir).expect("unheld during the POST");
        let _ = over.await;
        drop(lock);
    };
    let (response, ()) = tokio::join!(post, hold_lock);
    let observed = sent(simulator);
    assert_eq!(create_count(&observed), 1, "one POST on the wire");
    assert!(
        observed.len() > post_at + 1,
        "the read-back was attempted after the POST"
    );
    let answered = result(&response);
    assert!(answered.get("error").is_none(), "{response}");
    assert_eq!(
        answered["state"], "masters_after_post_unconfirmed",
        "{response}"
    );
    assert_eq!(answered["answer_recorded"], false, "{response}");
    assert_eq!(answered["counters"]["created"], 1, "{response}");
    assert_eq!(answered["verification"]["verdict_recorded"], false);
    assert_eq!(answered["verification"]["settled"], false);
    assert_eq!(
        answered["verification"]["verdict_record_error"],
        "import_admission_busy"
    );
    // The state is open, so it keeps its own step, then what was not recorded.
    assert_eq!(
        answered["next_step"],
        format!(
            "{} {}{}",
            verify::NEXT_UNCONFIRMED,
            verify::NEXT_VERDICT_NOT_RECORDED,
            verify::NEXT_ANSWER_NOT_RECORDED
        )
    );
    assert_ne!(answered["note"], MAY_HAVE_BEEN_SENT_NOTE, "{response}");
    assert_eq!(
        answered["note"],
        settled_note(MasterVerdictState::MastersAfterPostUnconfirmed)
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

// --- the answered call: Tally's answer and the verdict, whatever the journal does ---

/// `call` with the journal faults `points` armed (see [`TEST_FAULTS`]), and the
/// ones it never reached.
async fn faulting<F: std::future::Future>(
    points: Vec<&'static str>,
    call: F,
) -> (F::Output, Vec<&'static str>) {
    TEST_FAULTS
        .scope(std::cell::RefCell::new(points), async {
            let output = call.await;
            (output, TEST_FAULTS.with(|armed| armed.borrow().clone()))
        })
        .await
}

/// One approved post of `LEDGER` that Tally answers with `answer`, then
/// `read_back` plans, the book holding `after` once it is done, with `points`
/// armed.
struct Answered {
    response: Value,
    directory: tempfile::TempDir,
    batch: MasterBatch,
    args: Value,
    /// The armed faults the call never reached.
    unused: Vec<&'static str>,
    /// How many times the create's bytes reached the wire.
    posts: usize,
    /// How many requests reached the wire after the POST: the read-back's.
    reads_after_post: usize,
}

async fn answered_post(
    points: Vec<&'static str>,
    answer: String,
    read_back: Vec<ScenarioPlan>,
    after: Vec<IndexedMaster>,
) -> Answered {
    let mut plans = through_post(xml(answer));
    let through_the_post = plans.len();
    plans.extend(read_back);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let (response, unused) = faulting(
        points,
        posting_then(
            &ScriptedApproval::approving(),
            after,
            server.call_tool("post_ledger_creates", args.clone()),
        ),
    )
    .await;
    let observed = sent(simulator);
    let posts = create_count(&observed);
    Answered {
        response,
        directory,
        batch,
        args,
        unused,
        posts,
        reads_after_post: observed.len().saturating_sub(through_the_post),
    }
}

/// A later call for the batch of `earlier`: the identity and the read-back,
/// the book holding `after`. Returns the answer and how many creates it sent.
async fn later_call(
    earlier: &Answered,
    read_back: Vec<ScenarioPlan>,
    after: Vec<IndexedMaster>,
) -> (Value, usize) {
    let reads = SequenceSimulator::spawn(with_sentinel(read_back)).unwrap();
    let server = server_at(reads.address(), earlier.directory.path());
    let response = reconciling(
        after,
        server.call_tool("post_ledger_creates", earlier.args.clone()),
    )
    .await;
    (response, create_count(&sent(reads)))
}

fn clean_read_back() -> Vec<ScenarioPlan> {
    read_back(8, 3, Some(suspense_answer()))
}

#[tokio::test]
async fn a_failed_first_append_of_the_answer_is_recorded_under_the_verdicts_lock() {
    let done = answered_post(
        vec!["master_dispatch_response"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(
        done.reads_after_post,
        clean_read_back().len(),
        "the book was read back after the answer was lost"
    );
    assert_eq!(answered["state"], "posted_verified", "{}", done.response);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], true);
    assert_eq!(answered["verification"]["settled"], true);
    assert_eq!(answered["counters"]["created"], 1);
    // The answer is journaled before the verdict that was judged on it.
    assert_eq!(
        record_types(done.directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
    let state = state_of(done.directory.path(), &done.batch.batch_id);
    assert_eq!(state.response.unwrap().counters.unwrap().created, 1);
    assert_eq!(
        state.verdict.unwrap().state(),
        MasterVerdictState::PostedVerified
    );
}

#[tokio::test]
async fn an_answer_that_cannot_be_recorded_twice_is_judged_and_a_second_call_has_no_counters() {
    let done = answered_post(
        vec!["master_dispatch_response", "master_dispatch_response"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "both faults were reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(
        done.reads_after_post,
        clean_read_back().len(),
        "the book was read back after the answer was lost twice"
    );
    assert!(answered.get("error").is_none(), "{}", done.response);
    // Judged on the counters this call holds, and said not to be recorded.
    assert_eq!(answered["state"], "posted_verified", "{}", done.response);
    assert_eq!(answered["counters"]["created"], 1);
    assert_eq!(answered["answer_recorded"], false);
    assert_eq!(answered["verification"]["verdict_recorded"], false);
    assert_eq!(answered["verification"]["settled"], false);
    assert_eq!(
        answered["verification"]["verdict_record_error"],
        "master_ledger_unavailable"
    );
    assert_eq!(
        answered["next_step"],
        format!(
            "{}{}",
            verify::NEXT_VERDICT_NOT_RECORDED,
            verify::NEXT_ANSWER_NOT_RECORDED
        )
    );
    assert_eq!(
        record_types(done.directory.path()),
        ["master_batch", "master_dispatch_intent"],
        "no verdict is written without its answer"
    );
    // A second call reads the book back, sends nothing and has no counters.
    // The ledger was changed after the create (it is not the first master
    // change after the intent), so without counters it stays open.
    let (later, creates) = later_call(
        &done,
        reconcile_reads(9, 3, Some(suspense_answer())),
        book_with_new_ledger(9),
    )
    .await;
    let second = result(&later);
    assert_eq!(creates, 0, "a later call sends nothing");
    assert_eq!(second["state"], "reconciliation_required", "{later}");
    assert_eq!(second["answer_recorded"], false);
    assert_eq!(second["counters"], Value::Null);
    assert_eq!(second["verification"]["moment"], "reconcile");
    assert_eq!(second["verification"]["verdict_recorded"], true);
    assert_eq!(second["verification"]["settled"], false);
    assert!(second["verification"]["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|reason| reason == "counters_unavailable"));
    // The ledger is in the book without counters: nothing to key.
    assert_eq!(second["next_step"], verify::NEXT_PRESENT_NOT_VERIFIED);
    assert_eq!(
        second["note"],
        settled_note(MasterVerdictState::ReconciliationRequired)
    );
    assert_ne!(second["note"], MAY_HAVE_BEEN_SENT_NOTE);
    assert_ne!(
        second["verification"]["counters_note"],
        verify::COUNTERS_ABSENT_NOTE
    );
    assert_eq!(
        second["verification"]["counters_note"],
        verify::NO_ANSWER_RECORDED_NOTE
    );
    assert_eq!(
        record_types(done.directory.path()),
        ["master_batch", "master_dispatch_intent", "master_verdict"]
    );
}

#[tokio::test]
async fn a_failed_first_append_does_not_hide_an_altered_count() {
    let altered = created_one().replace("<ALTERED>0</ALTERED>", "<ALTERED>1</ALTERED>");
    let done = answered_post(
        vec!["master_dispatch_response"],
        altered,
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(
        done.reads_after_post,
        clean_read_back().len(),
        "the book was read back after the answer was lost"
    );
    assert_eq!(
        answered["state"], "posted_into_existing_master",
        "{}",
        done.response
    );
    assert_eq!(first_key(&done.response), "alert");
    assert_eq!(answered["counters"]["altered"], 1);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["settled"], true);
    assert!(state_of(done.directory.path(), &done.batch.batch_id).is_settled());
}

#[tokio::test]
async fn a_loud_state_whose_verdict_is_not_recorded_keeps_its_alert_and_its_own_step() {
    let altered = created_one().replace("<ALTERED>0</ALTERED>", "<ALTERED>1</ALTERED>");
    let done = answered_post(
        vec!["master_dispatch_response", "master_verdict"],
        altered,
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "a fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(
        done.reads_after_post,
        clean_read_back().len(),
        "the book was read back after the answer was not recorded"
    );
    assert_eq!(
        answered["state"], "posted_into_existing_master",
        "{}",
        done.response
    );
    assert_eq!(first_key(&done.response), "alert");
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], false);
    assert_eq!(answered["verification"]["settled"], false);
    assert_eq!(
        answered["next_step"],
        format!(
            "{} {}",
            settled_next_step(MasterVerdictState::PostedIntoExistingMaster),
            verify::NEXT_VERDICT_NOT_RECORDED
        )
    );
    assert!(!state_of(done.directory.path(), &done.batch.batch_id).is_settled());
}

#[tokio::test]
async fn a_busy_shared_read_after_the_answer_is_recorded_still_gives_a_verdict() {
    let done = answered_post(
        vec!["answer_read_busy"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the busy read was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert!(answered.get("error").is_none(), "{}", done.response);
    assert_eq!(answered["state"], "posted_verified", "{}", done.response);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], true);
    assert_eq!(
        record_types(done.directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
    assert!(state_of(done.directory.path(), &done.batch.batch_id).is_settled());
}

/// The shared read after the answer fails for a reason other than a busy lock
/// (here the lock file cannot be opened): the journal is read without the lock
/// all the same, so the book is still read back and judged.
#[tokio::test]
async fn a_shared_read_after_the_answer_that_fails_for_any_reason_is_retried_without_the_lock() {
    let done = answered_post(
        vec!["answer_read_lock_unavailable"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the failed read was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(
        done.reads_after_post,
        clean_read_back().len(),
        "the book was read back"
    );
    assert!(answered.get("error").is_none(), "{}", done.response);
    assert_eq!(answered["state"], "posted_verified", "{}", done.response);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], true);
    assert!(state_of(done.directory.path(), &done.batch.batch_id).is_settled());
}

#[tokio::test]
async fn a_busy_journal_lock_right_after_the_answer_is_recorded_gives_a_verdict_not_an_error() {
    // The lock is held by another process from the answer's append on: the
    // shared read is busy, so the journal is read without it, and the verdict
    // cannot be written.
    let done = answered_post(
        vec!["answer_read_busy", "verdict_lock_busy"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "a fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert!(answered.get("error").is_none(), "{}", done.response);
    assert_eq!(answered["state"], "posted_verified", "{}", done.response);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], false);
    assert_eq!(answered["verification"]["settled"], false);
    assert_eq!(
        answered["verification"]["verdict_record_error"],
        "import_admission_busy"
    );
    assert_eq!(answered["next_step"], verify::NEXT_VERDICT_NOT_RECORDED);
    assert_eq!(
        record_types(done.directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response"
        ]
    );
}

#[tokio::test]
async fn a_later_call_whose_verdict_is_not_recorded_does_not_claim_counters_it_lacks() {
    // No answer on file, and the verdict of a second call cannot be written.
    let done = answered_post(
        vec!["master_dispatch_response", "master_dispatch_response"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    assert!(done.unused.is_empty(), "both faults were reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    let ((later, creates), unused) = faulting(
        vec!["verdict_lock_busy"],
        // The ledger was changed after the create, so without counters it
        // stays open.
        later_call(
            &done,
            reconcile_reads(9, 3, Some(suspense_answer())),
            book_with_new_ledger(9),
        ),
    )
    .await;
    assert!(unused.is_empty(), "the busy lock was not reached");
    assert_eq!(creates, 0, "a later call sends nothing");
    let second = result(&later);
    assert_eq!(second["state"], "reconciliation_required", "{later}");
    assert_eq!(second["answer_recorded"], false);
    assert_eq!(second["verification"]["verdict_recorded"], false);
    assert_eq!(second["verification"]["settled"], false);
    // It claims no counters it lacks, and says there is no answer on file.
    assert_eq!(second["counters"], Value::Null);
    assert_eq!(
        second["verification"]["counters_note"],
        verify::NO_ANSWER_RECORDED_NOTE
    );
    // The state is open and present without counters: its own step comes first.
    assert_eq!(
        second["next_step"],
        format!(
            "{} {}",
            verify::NEXT_PRESENT_NOT_VERIFIED,
            verify::NEXT_VERDICT_NOT_RECORDED
        )
    );
    assert_eq!(
        record_types(done.directory.path()),
        ["master_batch", "master_dispatch_intent"]
    );
}

/// An open verdict the journal already holds, in the same words, stays recorded
/// when the lock is busy on a repeat reconcile: there is nothing left to write.
#[tokio::test]
async fn a_repeat_reconcile_of_an_open_verdict_stays_recorded_when_the_lock_is_busy() {
    // The first call's read-back is unavailable: an open verdict is recorded.
    let done = answered_post(Vec::new(), created_one(), Vec::new(), book()).await;
    assert_eq!(
        result(&done.response)["state"],
        "masters_after_post_unconfirmed"
    );
    let before = journal_bytes(done.directory.path());
    assert_eq!(
        record_types(done.directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
    let ((later, creates), unused) = faulting(
        vec!["verdict_lock_busy"],
        later_call(&done, verified_company(), book()),
    )
    .await;
    assert!(unused.is_empty(), "the busy lock was reached");
    assert_eq!(creates, 0, "a later call sends nothing");
    let second = result(&later);
    assert_eq!(second["state"], "masters_after_post_unconfirmed", "{later}");
    assert_eq!(second["verification"]["verdict_recorded"], true, "{later}");
    assert_eq!(second["verification"]["verdict_written"], false);
    assert!(second["verification"].get("verdict_record_error").is_none());
    // A recorded open verdict is not claimed unrecorded: its own step stands.
    assert_eq!(second["next_step"], verify::NEXT_UNCONFIRMED);
    assert_eq!(journal_bytes(done.directory.path()), before);
}

/// An open state whose verdict could not be written keeps the step it has of
/// its own (the check to make in Tally) and adds that the verdict is not
/// recorded and that calling again only reads.
#[tokio::test]
async fn an_open_state_whose_verdict_is_not_recorded_keeps_its_own_step() {
    // Tally answered, the book shows no such ledger and no mark moved.
    let done = answered_post(
        vec!["master_verdict"],
        created_one(),
        read_back(7, 3, None),
        book(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(
        answered["state"], "reconciliation_required",
        "{}",
        done.response
    );
    assert_eq!(answered["verification"]["verdict_recorded"], false);
    assert_eq!(answered["verification"]["settled"], false);
    assert_eq!(
        answered["next_step"],
        format!(
            "{} {}",
            verify::NEXT_RECONCILE,
            verify::NEXT_VERDICT_NOT_RECORDED
        )
    );
    // The step that is added says that calling again only reads: it does not
    // tell the person both to never post again and to call again.
    assert!(verify::NEXT_VERDICT_NOT_RECORDED
        .contains("Call post_ledger_creates with this batch_id again"));
    assert!(verify::NEXT_VERDICT_NOT_RECORDED
        .contains("sends nothing and is not posting the batch again"));
}

/// A verdict append that fails is never taken for a recorded one. The state the
/// call holds is stale: it shows the open verdict, while the journal now holds a
/// terminal one, so the append of the same open verdict is refused.
#[test]
fn a_failed_verdict_append_is_an_error_even_when_the_state_held_the_same_words() {
    let directory = tempfile::tempdir().unwrap();
    let server = unit_server(directory.path());
    let (batch, _) = saved_batch(&server, LEDGER);
    record_intent_for_test(&server, &batch);
    let open = MasterVerdict::for_test(
        batch.batch_id.clone(),
        batch.batch_sha256.clone(),
        MasterVerdictState::MastersAfterPostUnconfirmed,
        vec![VerdictItem {
            name: LEDGER.into(),
            guid: None,
            reason: Some("company_marks_unavailable".into()),
        }],
    );
    {
        let lock = ExclusiveAdmission::acquire(directory.path()).unwrap();
        append(directory.path(), &lock, MasterRecord::Verdict(open.clone())).unwrap();
    }
    let stale = state_of(directory.path(), &batch.batch_id);
    assert_eq!(stale.verdict.as_ref(), Some(&open));
    settle_holder(directory.path(), &batch);
    let before = journal_bytes(directory.path());
    let mut answer_recorded = false;
    let recorded = server.append_under_lock(&stale, open, None, &mut answer_recorded);
    assert_eq!(recorded, Err("master_ledger_invalid".to_string()));
    assert_eq!(journal_bytes(directory.path()), before);
}

/// The alert of a loud state under the answer cap. No tool refuses a cap below
/// a floor: the smallest `BRIDGE_AGENT_MAX_BYTES` accepted is 256, and an answer
/// over the cap is withheld as `agent_response_too_large`, alert and counters
/// with it. The verdict and the counters are in the journal, and a later call
/// under a cap that fits reports them. The sizes are measured, not assumed.
#[tokio::test]
async fn a_loud_answer_under_a_small_cap_is_withheld_and_the_journal_still_has_it() {
    let altered = created_one().replace("<ALTERED>0</ALTERED>", "<ALTERED>1</ALTERED>");
    let loud = |cap: usize| {
        let altered = altered.clone();
        async move {
            let mut plans = through_post(xml(altered));
            plans.extend(clean_read_back());
            let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
            let directory = tempfile::tempdir().unwrap();
            let server = server_capped(simulator.address(), directory.path(), cap);
            let (batch, args) = saved_batch(&server, LEDGER);
            let response = posting_then(
                &ScriptedApproval::approving(),
                book_after(),
                server.call_tool("post_ledger_creates", args.clone()),
            )
            .await;
            (response, directory, batch, args)
        }
    };
    // Under a cap that fits: the loud answer, and the size it takes.
    let (fits, directory, batch, args) = loud(200_000).await;
    assert_eq!(
        result(&fits)["state"],
        "posted_into_existing_master",
        "{fits}"
    );
    let loud_bytes = fits.to_string().len();
    // The same batch reported by a later call, from the journal alone.
    let silent = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let later = server_at(silent.address(), directory.path());
    let report = posting(
        &ScriptedApproval::approving(),
        later.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert!(result(&report).get("alert").is_some(), "{report}");
    let report_bytes = report.to_string().len();
    assert!(
        (3_000..4_000).contains(&loud_bytes) && (1_800..2_600).contains(&report_bytes),
        "loud answer {loud_bytes} bytes, its journal report {report_bytes}"
    );
    // At the smallest cap the setting accepts, the answer is withheld whole.
    let (capped, directory, batch_capped, args) = loud(256).await;
    assert_eq!(
        capped["structuredContent"]["error"]["code"], "agent_response_too_large",
        "{capped}"
    );
    assert!(capped.to_string().len() < loud_bytes);
    assert!(!capped.to_string().contains("ATTENTION"), "{capped}");
    // The journal holds the answer and the loud verdict all the same.
    let state = state_of(directory.path(), &batch_capped.batch_id);
    assert_eq!(state.response.unwrap().counters.unwrap().altered, 1);
    assert_eq!(
        state.verdict.unwrap().state(),
        MasterVerdictState::PostedIntoExistingMaster
    );
    // A later call under a cap that fits reports the alert.
    let silent = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let later = server_at(silent.address(), directory.path());
    let report = posting(
        &ScriptedApproval::approving(),
        later.call_tool("post_ledger_creates", args),
    )
    .await;
    assert_eq!(result(&report)["state"], "posted_into_existing_master");
    assert!(result(&report).get("alert").is_some(), "{report}");
    let _ = batch;
}

#[tokio::test]
async fn an_answer_written_but_reported_failed_is_not_appended_a_second_time() {
    let done = answered_post(
        vec!["master_dispatch_response_lost_after_write"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(
        done.reads_after_post,
        clean_read_back().len(),
        "the book was read back after the answer was reported failed"
    );
    assert_eq!(answered["state"], "posted_verified", "{}", done.response);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], true);
    assert_eq!(answered["verification"]["settled"], true);
    assert_eq!(
        record_types(done.directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
}

#[tokio::test]
async fn an_unavailable_read_back_after_a_failed_first_append_still_records_the_counters() {
    // The POST answers and nothing after it does.
    let done = answered_post(
        vec!["master_dispatch_response"],
        created_one(),
        Vec::new(),
        book(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert!(
        done.reads_after_post > 0,
        "the read-back was attempted after the answer was lost"
    );
    assert_eq!(
        answered["state"], "masters_after_post_unconfirmed",
        "{}",
        done.response
    );
    assert_eq!(answered["counters"]["created"], 1);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], true);
    assert_eq!(
        record_types(done.directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
    assert_eq!(
        state_of(done.directory.path(), &done.batch.batch_id)
            .response
            .unwrap()
            .counters
            .unwrap()
            .created,
        1
    );
}

#[tokio::test]
async fn a_failed_verdict_append_leaves_the_answer_and_a_second_call_settles_with_counters() {
    let done = answered_post(
        vec!["master_verdict"],
        created_one(),
        clean_read_back(),
        book_after(),
    )
    .await;
    let answered = result(&done.response);
    assert!(done.unused.is_empty(), "the fault was not reached");
    assert_eq!(done.posts, 1, "one POST on the wire");
    assert_eq!(answered["state"], "posted_verified", "{}", done.response);
    assert_eq!(answered["answer_recorded"], true);
    assert_eq!(answered["verification"]["verdict_recorded"], false);
    assert_eq!(answered["verification"]["settled"], false);
    // The answer is on file, so the step does not add that it is not.
    assert_eq!(answered["next_step"], verify::NEXT_VERDICT_NOT_RECORDED);
    assert_eq!(
        record_types(done.directory.path()),
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response"
        ]
    );
    // The company cannot be verified: the journal alone answers. It holds the
    // answer and no verdict, so the answer-recorded note is the one.
    let before = journal_bytes(done.directory.path());
    let down = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let unreachable_server = server_at(down.address(), done.directory.path());
    let report = reconciling(
        book_after(),
        unreachable_server.call_tool("post_ledger_creates", done.args.clone()),
    )
    .await;
    sent(down);
    assert_eq!(result(&report)["state"], "posted_unverified", "{report}");
    assert_eq!(result(&report)["journal"]["response_recorded"], true);
    assert_eq!(result(&report)["note"], ALREADY_SENT_NOTE);
    assert_eq!(journal_bytes(done.directory.path()), before);
    // The answer is on file, so the second call has the counters to settle on.
    let (later, creates) = later_call(
        &done,
        reconcile_reads(8, 3, Some(suspense_answer())),
        book_after(),
    )
    .await;
    let second = result(&later);
    assert_eq!(creates, 0, "a later call sends nothing");
    assert_eq!(second["state"], "posted_verified", "{later}");
    assert_eq!(second["answer_recorded"], true);
    assert_eq!(second["counters"]["created"], 1);
    assert_eq!(second["verification"]["verdict_recorded"], true);
    assert_eq!(second["verification"]["settled"], true);
    assert_eq!(
        second["next_step"],
        settled_next_step(MasterVerdictState::PostedVerified)
    );
}

#[tokio::test]
async fn a_journal_that_cannot_be_read_after_the_answer_is_answered_from_what_tally_said() {
    for (points, recorded) in [
        (vec!["answer_read_failed"], true),
        (
            vec!["master_dispatch_response", "answer_read_failed"],
            false,
        ),
    ] {
        let done = answered_post(points, created_one(), Vec::new(), book()).await;
        let answered = result(&done.response);
        assert!(done.unused.is_empty(), "a fault was not reached");
        assert_eq!(done.posts, 1, "one POST on the wire");
        assert_eq!(done.reads_after_post, 0, "the book was not read back");
        assert!(answered.get("error").is_none(), "{}", done.response);
        assert_eq!(answered["state"], "posted_unverified", "{}", done.response);
        assert_eq!(answered["answer_recorded"], recorded);
        assert_eq!(answered["counters"]["created"], 1);
        assert_eq!(answered["verification"]["attempted"], false);
        assert_eq!(
            answered["verification"]["error"]["code"],
            "master_journal_unavailable"
        );
        assert_eq!(
            answered["verification"]
                .get("answer_record_error")
                .is_some(),
            !recorded
        );
        assert_ne!(answered["note"], MAY_HAVE_BEEN_SENT_NOTE);
        assert_eq!(
            answered["note"],
            if recorded {
                POSTED_UNVERIFIED_NOTE
            } else {
                ANSWER_UNRECORDED_NOTE
            }
        );
        // The fallback says what to do next: a later call reads and sends nothing.
        assert_eq!(
            answered["next_step"],
            if recorded {
                NEXT_NOT_READ_BACK_AFTER_ANSWER.to_string()
            } else {
                format!("{NEXT_NOT_READ_BACK_AFTER_ANSWER}{NEXT_ANSWER_NOT_RECORDED}")
            }
        );
        // What the journal holds, and what a later call makes of it.
        let mut expected = vec!["master_batch", "master_dispatch_intent"];
        if recorded {
            expected.push("master_dispatch_response");
        }
        assert_eq!(record_types(done.directory.path()), expected);
        // Without the answer on file, the ledger was changed after the create,
        // so the later call stays open.
        let (mark, book) = if recorded {
            (8, book_after())
        } else {
            (9, book_with_new_ledger(9))
        };
        let (later, creates) = later_call(
            &done,
            reconcile_reads(mark, 3, Some(suspense_answer())),
            book,
        )
        .await;
        assert_eq!(creates, 0, "a later call sends nothing");
        let second = result(&later);
        if recorded {
            // The answer is on file, so the later call has the counters.
            assert_eq!(second["state"], "posted_verified", "{later}");
            assert_eq!(second["counters"]["created"], 1);
        } else {
            assert_eq!(second["state"], "reconciliation_required", "{later}");
            assert_eq!(second["counters"], Value::Null);
        }
    }
}

#[tokio::test]
async fn a_terminal_verdict_with_no_answer_on_file_is_not_reported_as_maybe_sent() {
    // The answer is lost on its way, and a reconcile finds nothing created and
    // no mark moved: a terminal verdict with no answer on file.
    let plans = through_post(xml(created_one()).with_delivery(
        Delivery::ResetAfterRequestProcessed {
            delay: Duration::from_millis(0),
        },
    ));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server, LEDGER);
    posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert_eq!(create_count(&sent(simulator)), 1, "one POST on the wire");
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(7, 3, None))).unwrap();
    let later = server_at(reads.address(), directory.path());
    let settled = reconciling(book(), later.call_tool("post_ledger_creates", args.clone())).await;
    assert_eq!(create_count(&sent(reads)), 0);
    assert_eq!(result(&settled)["state"], "confirmed_absent", "{settled}");
    // Reported from the journal alone: it holds the verdict and no answer.
    let silent = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let later = server_at(silent.address(), directory.path());
    let report = posting(
        &ScriptedApproval::approving(),
        later.call_tool("post_ledger_creates", args),
    )
    .await;
    assert_eq!(sent(silent).len(), 0);
    let reported = result(&report);
    assert_eq!(reported["state"], "confirmed_absent", "{report}");
    assert_eq!(reported["journal"]["response_recorded"], false);
    assert_ne!(reported["note"], MAY_HAVE_BEEN_SENT_NOTE);
    assert_eq!(
        reported["note"],
        settled_note(MasterVerdictState::ConfirmedAbsent)
    );
    assert_eq!(
        reported["next_step"],
        settled_next_step(MasterVerdictState::ConfirmedAbsent)
    );
}

/// The book after a create whose ledger has `alter_id`.
fn book_with_new_ledger(alter_id: u64) -> Vec<IndexedMaster> {
    let mut book = book_after();
    book.last_mut().unwrap().alter_id = alter_id;
    book
}

/// A create sent, its answer lost, and a later call: the first call ends with
/// the outcome unknown, and the reconcile reads the book as `after`.
async fn reconcile_after_a_lost_answer(
    after: Vec<IndexedMaster>,
    target_mark: u64,
) -> (Value, std::path::PathBuf, tempfile::TempDir, String) {
    let plans = through_post(xml(created_one()).with_delivery(
        Delivery::ResetAfterRequestProcessed {
            delay: Duration::from_millis(0),
        },
    ));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let first = posting(
        &ScriptedApproval::approving(),
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    assert_eq!(
        error_code(&first),
        "import_dispatch_outcome_unknown",
        "{first}"
    );
    assert_eq!(create_count(&sent(simulator)), 1, "one POST on the wire");
    assert!(state_of(directory.path(), &batch.batch_id)
        .response
        .is_none());
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(
        target_mark,
        3,
        Some(suspense_answer()),
    )))
    .unwrap();
    let later = server_at(reads.address(), directory.path());
    let answer = reconciling(after, later.call_tool("post_ledger_creates", args)).await;
    assert_eq!(create_count(&sent(reads)), 0, "a reconcile never sends");
    let path = directory.path().to_path_buf();
    (answer, path, directory, batch.batch_id)
}

/// The answer is lost on its way, and a reconcile finds the ledger as sent,
/// written by the first master change after the intent's mark of 7: the batch
/// settles as `posted_unattributed`, never verified, and releases its name.
#[tokio::test]
async fn a_lost_answer_whose_ledger_is_the_first_change_after_the_intent_settles_unattributed() {
    let (answer, path, _directory, batch_id) =
        reconcile_after_a_lost_answer(book_with_new_ledger(8), 8).await;
    let reported = result(&answer);
    assert_eq!(reported["state"], "posted_unattributed", "{answer}");
    assert_eq!(reported["answer_recorded"], false);
    assert!(reported["counters"].is_null());
    let verification = &reported["verification"];
    assert_eq!(verification["moment"], "reconcile");
    assert_eq!(verification["settled"], true);
    assert_eq!(verification["guid"], NEW_GUID.to_ascii_lowercase());
    assert_eq!(verification["checks"]["counters"], "not_checked");
    assert_eq!(
        verification["checks"]["first_change_after_intent"],
        "passed"
    );
    assert_eq!(verification["reasons"], json!(["counters_unavailable"]));
    assert_eq!(
        verification["counters_note"],
        verify::NO_ANSWER_RECORDED_NOTE
    );
    assert_eq!(reported["next_step"], verify::NEXT_UNATTRIBUTED);
    assert_eq!(
        reported["note"],
        settled_note(MasterVerdictState::PostedUnattributed)
    );
    assert!(reported["note"]
        .as_str()
        .unwrap()
        .contains("the create is NOT verified"));
    assert!(reported.get("alert").is_none());
    let state = state_of(&path, &batch_id);
    assert!(state.is_settled());
    assert_eq!(
        state.verdict.as_ref().map(|verdict| verdict.state()),
        Some(MasterVerdictState::PostedUnattributed)
    );
    // Reported from the journal alone afterwards, and nothing is sent.
    let silent = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let again = server_at(silent.address(), &path);
    let report = posting(
        &ScriptedApproval::approving(),
        again.call_tool("post_ledger_creates", json!({"batch_id": batch_id.clone()})),
    )
    .await;
    assert_eq!(sent(silent).len(), 0);
    let reported = result(&report);
    assert_eq!(reported["state"], "posted_unattributed", "{report}");
    assert_eq!(reported["journal"]["response_recorded"], false);
    assert_ne!(reported["note"], MAY_HAVE_BEEN_SENT_NOTE);
    assert_eq!(reported["next_step"], verify::NEXT_UNATTRIBUTED);
}

/// Tally answered without counters ComplyEaze Bridge could read, so the answer
/// is on file and was not lost: a later reconcile that finds the ledger as
/// sent at the first master change still does not settle it.
#[tokio::test]
async fn an_answer_on_file_without_counters_never_settles_unattributed() {
    let mut plans = through_post(xml(
        "<ENVELOPE><BODY>no counters here</BODY></ENVELOPE>".to_string()
    ));
    plans.extend(read_back(8, 3, Some(suspense_answer())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (batch, args) = saved_batch(&server, LEDGER);
    let first = posting_then(
        &ScriptedApproval::approving(),
        book_with_new_ledger(8),
        server.call_tool("post_ledger_creates", args.clone()),
    )
    .await;
    sent(simulator);
    assert_eq!(
        result(&first)["state"],
        "reconciliation_required",
        "{first}"
    );
    assert!(state_of(directory.path(), &batch.batch_id)
        .response
        .is_some());
    let reads = SequenceSimulator::spawn(with_sentinel(reconcile_reads(
        8,
        3,
        Some(suspense_answer()),
    )))
    .unwrap();
    let later = server_at(reads.address(), directory.path());
    let answer = reconciling(
        book_with_new_ledger(8),
        later.call_tool("post_ledger_creates", args),
    )
    .await;
    assert_eq!(create_count(&sent(reads)), 0);
    let reported = result(&answer);
    assert_eq!(reported["state"], "reconciliation_required", "{answer}");
    assert_eq!(reported["verification"]["moment"], "reconcile");
    assert_eq!(
        reported["verification"]["checks"]["first_change_after_intent"],
        "not_checked"
    );
    assert!(!state_of(directory.path(), &batch.batch_id).is_settled());
}

/// The same lost answer, with the ledger changed after the first master change
/// (a same-name ledger made before the create, which then altered it): the
/// batch stays open, with its own reason, and holds the name.
#[tokio::test]
async fn a_lost_answer_whose_ledger_changed_since_stays_open_as_altered_since_create() {
    let (answer, path, _directory, batch_id) =
        reconcile_after_a_lost_answer(book_with_new_ledger(9), 9).await;
    let reported = result(&answer);
    assert_eq!(reported["state"], "reconciliation_required", "{answer}");
    let verification = &reported["verification"];
    assert_eq!(verification["settled"], false);
    assert_eq!(
        verification["checks"]["first_change_after_intent"],
        "failed"
    );
    assert_eq!(
        verification["reasons"],
        json!(["ledger_altered_since_create", "counters_unavailable"])
    );
    assert_eq!(reported["next_step"], verify::NEXT_PRESENT_NOT_VERIFIED);
    assert!(!state_of(&path, &batch_id).is_settled());
}

#[test]
fn the_fault_points_are_under_bare_cfg_test() {
    let ledger = include_str!("agent_master_ledger.rs");
    for needle in [
        "#[cfg(test)]\ntokio::task_local! {",
        "#[cfg(test)]\n    if test_fault_fires(record.record_type()) {",
        "#[cfg(test)]\n    let lost_after_write =",
        "#[cfg(test)]\n    if lost_after_write && appended.is_ok() {",
        "#[cfg(test)]\n    if test_fault_fires(\"master_dispatch_intent\") {",
        "#[cfg(test)]\n    let intent_lost_after_write = test_fault_fires(\"master_dispatch_intent_lost_after_write\");",
        "#[cfg(test)]\n    if intent_lost_after_write && recorded.is_ok() {",
        "#[cfg(test)]\npub(super) fn test_fault_fires(",
        "#[cfg(test)]\n    fn record_type(&self)",
    ] {
        assert_eq!(ledger.matches(needle).count(), 1, "{needle}");
    }
    let tools = include_str!("agent_master_tools.rs");
    for needle in [
        "#[cfg(test)]\n        if super::master_ledger::test_fault_fires(\"answer_read_failed\") {",
        "#[cfg(test)]\n        let shared = if super::master_ledger::test_fault_fires(\"answer_read_busy\") {",
        "} else if super::master_ledger::test_fault_fires(\"answer_read_lock_unavailable\") {",
        "#[cfg(test)]\n        if super::master_ledger::test_fault_fires(\"failure_read_busy\") {",
        "#[cfg(test)]\n        if super::master_ledger::test_fault_fires(\"hold_check_read_busy\") {",
        "#[cfg(test)]\n            let _ = super::master_ledger::BEFORE_HOLD_CHECK.try_with(|before| before());",
        "#[cfg(test)]\n                let _ = super::master_ledger::BEFORE_FAILURE_READ.try_with(|before| before());",
    ] {
        assert_eq!(tools.matches(needle).count(), 1, "{needle}");
    }
    let verify = include_str!("agent_master_verify.rs");
    assert_eq!(
        verify
            .matches("#[cfg(test)]\n            if super::super::master_ledger::test_fault_fires(\"verdict_lock_busy\") {")
            .count(),
        1
    );
    for source in [ledger, tools, verify] {
        for forbidden in ["cfg(any(test", "cfg(feature", "cfg(all(test"] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
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
