//! bridge#583: the native post driven end to end through the `post_import`
//! tool call against the protocol simulator, the approval answered by the
//! test-only seam (`approved_import::test_seam`). No real Tally is involved.
use super::*;
use super::{SCRIPTED_REMOTE_ID, SCRIPTED_REMOTE_IDS};
use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};
use bridge_tally_transport::TallyEndpointConfig;
use std::time::Duration;
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator,
    WireEncoding,
};

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";

fn captured(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn companies() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ))
}

fn catalogue() -> String {
    crate::agent::agent_import::tests::with_bill_wise_flags(
        &captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue-v2.utf16le.xml"
        )),
        &[],
    )
}

/// The captured Currency masters of a book with exactly one (`I₹`).
fn single_currency() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ))
}

/// The captured Currency masters of a book with two (`$` and the base).
fn two_currencies() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
    ))
}

fn empty_collection() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-empty-collection.utf16le.xml"
    ))
}

fn marks() -> String {
    marks_at(10)
}

/// The target's own marks with `vouchers` as its voucher mark: what a
/// verification reads once after a native post, to tell a book that kept the
/// post from one rolled back below it.
fn marks_at(vouchers: u64) -> String {
    format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY>\\
         <GUID>{GUID}</GUID><ALTVCHID>{vouchers}</ALTVCHID><ALTMSTID>7</ALTMSTID>\\
         </COMPANY></COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

/// Every loaded company's change marks, shaped as the high-water collection
/// returns them (a `NAME` attribute per row; measured 2026-09-21). The target
/// is the captured laboratory company; the other two are invented.
fn company_marks(target_vouchers: u64, other_vouchers: u64, target_name: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <COMPANY NAME=\"{target_name}\" RESERVEDNAME=\"\"><GUID>{GUID}</GUID><ALTVCHID>{target_vouchers}</ALTVCHID><ALTMSTID>7</ALTMSTID></COMPANY>\
         <COMPANY NAME=\"Synthetic Other Lab\" RESERVEDNAME=\"\"><GUID>22222222-2222-4222-8222-222222222222</GUID><ALTVCHID>{other_vouchers}</ALTVCHID><ALTMSTID>3</ALTMSTID></COMPANY>\
         <COMPANY NAME=\"Synthetic Empty Lab\" RESERVEDNAME=\"\"><GUID>33333333-3333-4333-8333-333333333333</GUID><ALTMSTID>1</ALTMSTID></COMPANY>\
         </COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

fn marks_before() -> ScenarioPlan {
    xml(company_marks(10, 50, "WR2 Unicode Lab"))
}

/// The same marks, read as the queue's binding reads begin (#239): the aim
/// snapshot must show the target's master mark unchanged since.
fn marks_at_binding() -> ScenarioPlan {
    marks_before()
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

/// A product and mode probe: status, then the company list.
fn probe() -> Vec<ScenarioPlan> {
    vec![status(), xml(companies())]
}

/// The company's verified identity.
fn verified_company() -> Vec<ScenarioPlan> {
    vec![xml(companies()), status(), xml(companies()), status()]
}

/// Every read `post_import` makes before it asks for approval, on a book where
/// the batch's vouchers are absent.
fn before_approval() -> Vec<ScenarioPlan> {
    before_approval_with_currencies(single_currency())
}

fn before_approval_with_currencies(currencies: String) -> Vec<ScenarioPlan> {
    let mut plans = Vec::new();
    // verify_import_for_post: opening mode, identity, the window's mark, the
    // window, its replay, and the closing mode an absence needs.
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.extend(probe());
    // The post's own identity, ledger catalogue, Currency masters and
    // qualified mode.
    plans.extend(verified_company());
    plans.extend(paired(catalogue()));
    plans.extend(paired(currencies));
    plans.extend(probe());
    plans
}

/// Tally's answer to one created voucher.
/// Tally's answer to one created voucher: a captured live response
/// (licensed-lab import, sanitized), not a hand-written shape. The earlier
/// hand-written ENVELOPE did not parse as an import outcome at all.
fn created_one() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/live_education_w4_voucher_sanitized.xml"
    )
    .to_string()
}

/// What the dispatch sends after approval, up to and including the import:
/// the opening mode and company admission, the marks at binding, the ledger
/// catalogue, the Currency
/// masters, the closing mode and admission, the absence read twice, the aim
/// snapshot, then the one POST. The index of the POST is
/// `before_approval().len() + after_approval(..).len() - 1`.
fn after_approval(post: ScenarioPlan) -> Vec<ScenarioPlan> {
    after_approval_with_currencies(single_currency(), post)
}

fn after_approval_with_currencies(currencies: String, post: ScenarioPlan) -> Vec<ScenarioPlan> {
    let mut plans = probe();
    plans.push(xml(companies()));
    plans.push(marks_at_binding());
    plans.extend(paired(catalogue()));
    plans.extend(paired(currencies));
    plans.extend(probe());
    plans.push(xml(companies()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.push(marks_before());
    plans.push(post);
    plans
}

/// The one dispatch record a run journaled.
fn dispatch_intent(directory: &std::path::Path) -> Value {
    let intents = String::from_utf8(journal(directory))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "dispatch_intent")
        .collect::<Vec<_>>();
    assert_eq!(intents.len(), 1, "{intents:?}");
    intents[0].clone()
}

/// The one dispatch response a run journaled, with Tally's answer parsed.
/// A POST answered with `created_one()` must journal a parsed, clean single
/// create; `None` here means the post ran without a parsed outcome.
fn journaled_outcome(
    directory: &std::path::Path,
) -> Option<bridge_tally_protocol::TallyImportOutcome> {
    let responses = String::from_utf8(journal(directory))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "dispatch_response")
        .collect::<Vec<_>>();
    assert_eq!(responses.len(), 1, "{responses:?}");
    serde_json::from_value::<ledger::DispatchResponse>(responses[0]["response"].clone())
        .unwrap()
        .outcome
}

fn assert_journaled_clean_create(directory: &std::path::Path) {
    let outcome = journaled_outcome(directory).expect("the POST answer was parsed and journaled");
    assert_eq!(outcome.counters().created, 1);
    assert!(import_outcome_is_clean(Some(&outcome), 1));
}

fn server_at(address: std::net::SocketAddr, directory: &std::path::Path) -> Server {
    server_redacting(address, directory, crate::agent::Redaction::None)
}

fn server_redacting(
    address: std::net::SocketAddr,
    directory: &std::path::Path,
    redaction: crate::agent::Redaction,
) -> Server {
    Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: address.ip().to_string(),
            port: address.port(),
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    })
}

/// Record the build's ledger binding as `build_import_xml` does (#239): each
/// named ledger with the GUID the captured catalogue gives it. These batches
/// answer no bank cash line, so their cash-in-hand ledgers are none (#815).
fn bind_to_captured_catalogue(line: &mut ImportLedgerLine) {
    line.cash_in_hand_ledgers = Some(Vec::new());
    line.on_account_approved = Some(Vec::new());
    let payload = ImportPayload {
        company_guid: line.company_guid.clone(),
        vouchers: line.vouchers.clone(),
        amends_batch_id: None,
    };
    let binding = crate::tally::standard_ledger_catalog::parse_import_catalog_as_v1(
        &catalogue(),
        "WR2 Unicode Lab",
        GUID,
    )
    .unwrap()
    .bind_selected(requested_ledger_names(&payload))
    .unwrap();
    line.ledger_identities = Some(
        binding
            .pairs()
            .map(|(name, guid)| BoundLedger {
                name: name.to_string(),
                guid: guid.to_string(),
            })
            .collect(),
    );
}

/// A built, never-dispatched batch for the captured laboratory company, with
/// its XML file, exactly as `build_import_xml` leaves one.
fn saved_batch(server: &Server) -> (ImportLedgerLine, Value) {
    saved_batch_with_narration(server, "Synthetic test only")
}

/// The saved Journal of `saved_batch`, as any build may have saved it, with
/// `narration`.
fn saved_batch_with_narration(server: &Server, narration: &str) -> (ImportLedgerLine, Value) {
    let origin = super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let mut line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000583", "identity_scheme":"batch_v1",
        "company_guid":GUID,
        "endpoint_origin":origin,
        "company":{"name":"WR2 Unicode Lab","guid":GUID,"company_number":"100004","books_from":"20260401"},
        "txn_ids":["journal-583"],"date_from":"20260901","date_to":"20260901",
        "sha256":"", "built_at":"2026-09-22T00:00:00Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":10,"master_value":7},
        "vouchers":[{"bridge_txn_id":"journal-583","date":"20260901","voucher_type":"Journal",
            "narration":narration,"entries":[
                {"ledger":"WR2 Sales","amount":"12.50","side":"Dr"},
                {"ledger":"Cash","amount":"12.50","side":"Cr"}]}]
    }))
    .unwrap();
    let rendered = render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(rendered.as_bytes());
    bind_to_captured_catalogue(&mut line);
    server.append_import_ledger(&line).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        rendered,
    )
    .unwrap();
    let args = json!({"company_guid":GUID,"batch_id":line.batch_id});
    (line, args)
}

fn journal(directory: &std::path::Path) -> Vec<u8> {
    fs::read(directory.join("agent-import-ledger.jsonl")).unwrap()
}

/// The `record_type` of every record appended to the journal since `before`.
/// The pre-post absence check writes one `verification_status` record; only a
/// dispatch writes `dispatch_intent` or `dispatch_response`.
fn appended_kinds(before: &[u8], after: &[u8]) -> Vec<String> {
    assert!(after.starts_with(before), "the journal is append-only");
    String::from_utf8(after[before.len()..].to_vec())
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

/// `plans` and one more, so a request past the last expected one is served
/// and observed rather than refused by a simulator that has stopped listening.
fn with_sentinel(mut plans: Vec<ScenarioPlan>) -> Vec<ScenarioPlan> {
    plans.push(status());
    plans
}

/// Data requests a run actually made (a cancel's wake-up connection has no method).
fn sent(simulator: SequenceSimulator) -> Vec<tally_protocol_simulator::ObservedRequest> {
    simulator.cancel();
    simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .collect()
}

/// With no scripted decision in scope the test build's approval declines at
/// once and starts no process: no request follows the pre-approval reads, no
/// intent is journaled, and the journal is byte-identical.
#[tokio::test]
async fn an_unscripted_approval_declines_and_nothing_is_sent_or_journaled() {
    let expected = before_approval().len();
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let before = journal(directory.path());
    let response = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_approval_declined",
        "{response}"
    );
    assert_ne!(result["attempt_recorded"], json!(true), "{response}");
    assert_eq!(observed.len(), expected);
    assert_eq!(
        appended_kinds(&before, &journal(directory.path())),
        ["verification_status"]
    );
}

/// Another batch, already dispatched with `remote_id`, in the journal.
fn journal_an_earlier_intent(server: &Server, line: &ImportLedgerLine, remote_id: Uuid) {
    let mut earlier = line.clone();
    earlier.batch_id = "bridge-00000000-0000-4000-8000-000000000584".into();
    server.append_import_ledger(&earlier).unwrap();
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
            &earlier,
            "c".repeat(64),
            remote_id,
        ))
        .unwrap();
}

/// A REMOTEID the journal already records is never sent again, since a resend
/// undoes a person's cancel or delete (protocol reference §9.3). The post is
/// refused from the journal alone: no Tally request, nothing appended.
#[tokio::test]
async fn a_remoteid_the_journal_records_is_refused_before_any_tally_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let reused = Uuid::new_v4();
    journal_an_earlier_intent(&server, &line, reused);
    let before = journal(directory.path());
    let response = SCRIPTED_REMOTE_ID
        .scope(
            reused,
            SCRIPTED_APPROVAL.scope(
                ScriptedApproval::approving(),
                server.call_tool("post_import", args),
            ),
        )
        .await;
    let observed = sent(simulator);
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "import_remote_id_reused",
        "{response}"
    );
    assert_eq!(observed.len(), 0);
    assert_eq!(journal(directory.path()), before);
}

/// #876: a row another batch of the company already sent to Tally is refused
/// from the journal alone, before any Tally request and before the person is
/// asked; nothing is appended.
#[tokio::test]
async fn a_row_another_batch_already_sent_is_refused_before_any_tally_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    journal_an_earlier_intent(&server, &line, Uuid::new_v4());
    let before = journal(directory.path());
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "import_txn_already_posted",
        "{response}"
    );
    let next_step = response["structuredContent"]["result"]["error"]["next_step"]
        .as_str()
        .unwrap_or_default();
    assert!(next_step.contains("verify_import"), "{response}");
    assert!(next_step.contains("Never rebuild"), "{response}");
    assert_eq!(
        response["structuredContent"]["result"]["error"]["blocking_batch_id"],
        "bridge-00000000-0000-4000-8000-000000000584",
        "{response}"
    );
    assert!(
        next_step.contains("bridge-00000000-0000-4000-8000-000000000584"),
        "{response}"
    );
    assert_eq!(observed.len(), 0);
    assert_eq!(journal(directory.path()), before);
}

/// #876: the same row sent by another batch while the dialog is open is
/// refused under the admission lock, before this post's intent, and nothing is
/// sent.
#[tokio::test]
async fn a_row_another_batch_sends_while_approval_is_pending_is_never_sent() {
    let result = refused_under_the_admission_lock(
        |path, line| {
            let mut earlier = line.clone();
            earlier.batch_id = "bridge-00000000-0000-4000-8000-000000000586".into();
            append_record(path, &earlier);
            append_record(
                path,
                &ledger::StatusRecord::dispatch_native(&earlier, "c".repeat(64), Uuid::new_v4()),
            );
        },
        "import_txn_already_posted",
        json!(false),
        0,
    )
    .await;
    assert_eq!(
        result["error"]["blocking_batch_id"], "bridge-00000000-0000-4000-8000-000000000586",
        "{result}"
    );
}

/// While the dialog is open, another process journals an intent carrying
/// `injected`; this post mints `minted`. Returns the response, the requests
/// Tally received, where the POST would be, and the batches with an intent.
async fn race_an_intent_during_approval(
    injected: Uuid,
    minted: Uuid,
) -> (Value, usize, usize, Vec<String>) {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let mut earlier = line.clone();
    earlier.batch_id = "bridge-00000000-0000-4000-8000-000000000585".into();
    // Another transaction, so only the REMOTEID can make this a refusal (#876).
    earlier.txn_ids = vec!["journal-585".into()];
    earlier.vouchers[0].bridge_txn_id = "journal-585".into();
    let mut appended = serde_json::to_vec(&earlier).unwrap();
    appended.push(b'\n');
    appended.extend(
        serde_json::to_vec(&ledger::StatusRecord::dispatch_native(
            &earlier,
            "c".repeat(64),
            injected,
        ))
        .unwrap(),
    );
    appended.push(b'\n');
    let path = directory.path().join("agent-import-ledger.jsonl");
    let scripted = ScriptedApproval::approving_after(move || {
        use std::io::Write;
        std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap()
            .write_all(&appended)
            .unwrap();
    });
    let response = SCRIPTED_REMOTE_ID
        .scope(
            minted,
            SCRIPTED_APPROVAL.scope(scripted, server.call_tool("post_import", args)),
        )
        .await;
    let observed = sent(simulator).len();
    let intents = String::from_utf8(journal(directory.path()))
        .unwrap()
        .lines()
        .map(|record| serde_json::from_str::<Value>(record).unwrap())
        .filter(|record| record["record_type"] == "dispatch_intent")
        .map(|record| record["batch_id"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    (response, observed, post_at, intents)
}

/// #711: while the dialog is open, `change` rewrites this batch's journal;
/// the check under the admission lock then refuses with `code`, before this
/// post's intent is appended, and the POST is never sent. `attempt_recorded`
/// stays what the journal shows (`attempted`), and the batch carries exactly
/// `intents` dispatch intents: only any `change` wrote, none from this post.
async fn refused_under_the_admission_lock(
    change: impl Fn(&std::path::Path, &ImportLedgerLine) + Send + Sync + 'static,
    code: &str,
    attempted: Value,
    intents: usize,
) -> Value {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let batch_id = line.batch_id.clone();
    let path = directory.path().join("agent-import-ledger.jsonl");
    let scripted = ScriptedApproval::approving_after(move || change(&path, &line));
    let response = SCRIPTED_APPROVAL
        .scope(scripted, server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator).len();
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["error"]["code"], code, "{response}");
    assert_eq!(result["attempt_recorded"], attempted, "{response}");
    assert_eq!(observed, post_at, "the POST is never sent: {response}");
    let recorded = String::from_utf8(journal(directory.path()))
        .unwrap()
        .lines()
        .map(|record| serde_json::from_str::<Value>(record).unwrap())
        .filter(|record| {
            record["record_type"] == "dispatch_intent" && record["batch_id"] == batch_id.as_str()
        })
        .count();
    assert_eq!(recorded, intents, "no intent from this post: {response}");
    result.clone()
}

fn append_record(path: &std::path::Path, record: &impl serde::Serialize) {
    use std::io::Write;
    let mut bytes = serde_json::to_vec(record).unwrap();
    bytes.push(b'\n');
    std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap()
        .write_all(&bytes)
        .unwrap();
}

#[tokio::test]
async fn a_batch_gone_from_the_journal_under_the_lock_is_refused_by_name() {
    refused_under_the_admission_lock(
        |path, _| std::fs::write(path, b"").unwrap(),
        "import_batch_not_found",
        json!(null),
        0,
    )
    .await;
}

#[tokio::test]
async fn a_batch_attempted_while_approval_is_pending_is_refused_by_name() {
    refused_under_the_admission_lock(
        |path, line| {
            append_record(
                path,
                &ledger::StatusRecord::dispatch_native(line, "c".repeat(64), Uuid::new_v4()),
            );
        },
        "import_already_attempted",
        json!(true),
        1,
    )
    .await;
}

#[tokio::test]
async fn a_batch_changed_while_approval_is_pending_is_refused_by_name() {
    refused_under_the_admission_lock(
        |path, line| {
            let mut changed = line.clone();
            changed.sha256 = "d".repeat(64);
            append_record(path, &changed);
        },
        "import_batch_changed",
        json!(false),
        0,
    )
    .await;
}

fn batch_server_at(address: std::net::SocketAddr, directory: &std::path::Path) -> Server {
    Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: address.ip().to_string(),
            port: address.port(),
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: true,
    })
}

/// `saved_batch`, with a second Journal on the same ledgers and day.
fn saved_batch_of_two(server: &Server) -> (ImportLedgerLine, Value) {
    let (mut line, args) = saved_batch(server);
    let mut second = line.vouchers[0].clone();
    second.bridge_txn_id = "journal-583-2".into();
    second
        .entries
        .iter_mut()
        .for_each(|entry| entry.amount = "7.25".into());
    line.vouchers.push(second);
    line.txn_ids.push("journal-583-2".into());
    let rendered = render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(rendered.as_bytes());
    bind_to_captured_catalogue(&mut line);
    server.append_import_ledger(&line).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        rendered,
    )
    .unwrap();
    (line, args)
}

/// A batch of two, while another process records `injected` in an intent
/// during approval; the post mints `minted`.
async fn race_a_batch_id_during_approval(
    injected: Uuid,
    minted: [Uuid; 2],
) -> (Value, usize, usize) {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = batch_server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch_of_two(&server);
    let mut earlier = line.clone();
    earlier.batch_id = "bridge-00000000-0000-4000-8000-000000000585".into();
    earlier.vouchers.truncate(1);
    earlier.txn_ids.truncate(1);
    // Another transaction, so only the REMOTEID can make this a refusal (#876).
    earlier.txn_ids = vec!["journal-585".into()];
    earlier.vouchers[0].bridge_txn_id = "journal-585".into();
    let mut appended = serde_json::to_vec(&earlier).unwrap();
    appended.push(b'\n');
    appended.extend(
        serde_json::to_vec(&ledger::StatusRecord::dispatch_native(
            &earlier,
            "c".repeat(64),
            injected,
        ))
        .unwrap(),
    );
    appended.push(b'\n');
    let path = directory.path().join("agent-import-ledger.jsonl");
    let scripted = ScriptedApproval::approving_after(move || {
        use std::io::Write;
        std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap()
            .write_all(&appended)
            .unwrap();
    });
    let response = SCRIPTED_REMOTE_IDS
        .scope(
            minted.to_vec(),
            SCRIPTED_APPROVAL.scope(scripted, server.call_tool("post_import", args)),
        )
        .await;
    (response, sent(simulator).len(), post_at)
}

/// A batch's SECOND REMOTEID, recorded by another process while the dialog
/// is open, is caught inside the queue: no POST. The control, with an
/// unrelated id recorded instead, posts. So every id is checked, not only
/// the first.
#[tokio::test]
async fn a_batch_whose_second_remote_id_is_recorded_during_approval_is_never_sent() {
    let minted = [Uuid::new_v4(), Uuid::new_v4()];
    let (response, observed, post_at) = race_a_batch_id_during_approval(minted[1], minted).await;
    assert_eq!(observed, post_at, "{response}");
    let (response, observed, post_at) =
        race_a_batch_id_during_approval(Uuid::new_v4(), minted).await;
    assert!(
        observed > post_at,
        "the control's POST was sent: {response}"
    );
}

/// A live batch post records its step verdict durably, before the readback. Tally's captured answer reports one create for this batch of
/// two, and the target's mark moves by two: the step doubt is recorded, and
/// the batch is not verified.
#[tokio::test]
async fn a_batch_post_records_its_step_verdict_before_the_readback() {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(company_marks(12, 50, "WR2 Unicode Lab")));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = batch_server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch_of_two(&server);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let _ = sent(simulator);
    // The dialog was asked about both vouchers, the count that its title
    // names (and on macOS its button, #746); the approval's unit tests check
    // those words.
    assert_eq!(scripted.counts(), [2]);
    let imports = server.imports_dir().unwrap();
    let doubt: Value = serde_json::from_slice(
        &fs::read(imports.join(format!("{}.batch_step_doubt.json", line.batch_id))).unwrap(),
    )
    .unwrap();
    assert_eq!(doubt["state"], "unmatched", "{response}");
    assert_eq!(doubt["target_voucher_step"]["step"], 2, "{doubt}");
    assert_eq!(
        doubt["target_voucher_step"]["reported_created"], 1,
        "{doubt}"
    );
    assert_eq!(
        super::super::read_masters_check(&imports, &line.batch_id).unwrap()["batch_step"]["state"],
        "unmatched"
    );
    assert_ne!(
        response["structuredContent"]["result"]["dispatch"]["state"], "posted_verified",
        "{response}"
    );
}

/// A batch post whose marks readback fails records the cause with the step
/// doubt (#884), in the result and in the durable file; the verdict stays a
/// doubt either way.
#[tokio::test]
async fn a_batch_post_whose_readback_failed_names_the_cause_in_its_step_doubt() {
    for (readback, cause) in [
        (None, "response_encoding_invalid"),
        (Some(xml(created_one())), "marks_readback_unparsed"),
    ] {
        let mut plans = before_approval();
        plans.extend(after_approval(xml(created_one())));
        plans.extend(readback);
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = batch_server_at(simulator.address(), directory.path());
        let (line, args) = saved_batch_of_two(&server);
        let response = SCRIPTED_APPROVAL
            .scope(
                ScriptedApproval::approving(),
                server.call_tool("post_import", args),
            )
            .await;
        let _ = sent(simulator);
        let result = &response["structuredContent"]["result"];
        assert_eq!(
            result["post_location"]["after_read_failure"], cause,
            "{response}"
        );
        assert_eq!(
            result["post_location"]["state"], "after_snapshot_unavailable",
            "{response}"
        );
        let imports = server.imports_dir().unwrap();
        let doubt: Value = serde_json::from_slice(
            &fs::read(imports.join(format!("{}.batch_step_doubt.json", line.batch_id))).unwrap(),
        )
        .unwrap();
        assert_eq!(doubt["state"], "unmatched", "{doubt}");
        assert_eq!(doubt["cause"], cause, "{doubt}");
        assert_ne!(result["dispatch"]["state"], "posted_verified", "{response}");
    }
}

/// With batch posting off, a batch of two is refused before any request.
#[tokio::test]
async fn a_batch_is_refused_while_batch_posting_is_off() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch_of_two(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"],
        "import_post_requires_one_voucher",
        "{response}"
    );
    assert!(sent(simulator).is_empty());
}

/// The same REMOTEID recorded by another process while the dialog is open is
/// caught as the intent is written: no intent for this batch, and no POST. The
/// control, an injected intent with another REMOTEID, posts: so the match is
/// what stopped the first.
#[tokio::test]
async fn a_remoteid_recorded_while_approval_is_pending_is_never_sent() {
    let raced = Uuid::new_v4();
    let (response, observed, post_at, intents) = race_an_intent_during_approval(raced, raced).await;
    // Refused under the admission lock before the intent (#711): it keeps
    // its own code, and nothing was recorded or sent.
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "import_remote_id_reused",
        "{response}"
    );
    assert_eq!(
        response["structuredContent"]["result"]["attempt_recorded"],
        json!(false),
        "{response}"
    );
    assert_eq!(observed, post_at, "{response}");
    assert_eq!(intents, ["bridge-00000000-0000-4000-8000-000000000585"]);

    let (response, observed, post_at, intents) =
        race_an_intent_during_approval(Uuid::new_v4(), raced).await;
    assert!(
        observed > post_at,
        "the control's POST was sent: {response}"
    );
    assert_eq!(
        intents,
        [
            "bridge-00000000-0000-4000-8000-000000000585",
            "bridge-00000000-0000-4000-8000-000000000583"
        ]
    );
}

/// Approved, the post sends exactly the request its dispatch intent recorded:
/// the POST's body hashes to the journal's `native_request_sha256`, and the
/// code's own renderer, given the journal's `native_remote_id`, produces those
/// same bytes. The approval was asked about the preview the real dialog shows.
#[tokio::test]
async fn an_approved_post_sends_exactly_the_request_its_intent_recorded() {
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let company_guid = args["company_guid"].as_str().unwrap().to_string();
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert!(observed.len() > post_at, "{response}");
    // The post drops every ledger listing snapshot of its company (#630).
    let dropped = server.listings.lock().unwrap().dropped_companies().to_vec();
    assert_eq!(dropped.len(), 1, "{dropped:?}");
    assert!(
        dropped[0].eq_ignore_ascii_case(&company_guid),
        "{dropped:?}"
    );

    let intent = dispatch_intent(directory.path());
    assert_journaled_clean_create(directory.path());
    let recorded_sha = intent["native_request_sha256"].as_str().unwrap();
    let recorded_id = intent["native_remote_id"].as_str().unwrap();
    assert_eq!(observed[post_at].request_body_sha256, recorded_sha);
    let remote_id = Uuid::parse_str(recorded_id).unwrap();
    let rendered = native_post_request(&line, RemoteIds::from_ids(vec![remote_id])).unwrap();
    assert_eq!(rendered.request_sha256, recorded_sha);
    assert!(rendered
        .xml
        .contains(&format!("<VOUCHER REMOTEID=\"{recorded_id}\"")));
    // The renderer is a function of the batch and the REMOTEID alone: the same
    // REMOTEID renders the same bytes, and another REMOTEID different ones, so
    // the match above could not come from anything else in the request.
    assert_eq!(
        native_post_request(&line, RemoteIds::from_ids(vec![remote_id]))
            .unwrap()
            .xml,
        rendered.xml
    );
    assert_ne!(
        native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()]))
            .unwrap()
            .request_sha256,
        recorded_sha
    );
    assert_eq!(
        scripted.previews(),
        [agent_review_preview(&line, &server.settings.endpoint).unwrap()]
    );
}

/// #632: an amendment is never posted natively, and this refusal is what
/// keeps the amendment compare-and-swap a build-time check. That check admits
/// a voucher whose ALTERID equals the verified baseline of *any* build in its
/// lineage, which is sound only against the read it has just made. If this
/// test ever has to change because amendments are posted, the post-time check
/// must bind the exact (GUID, MASTERID, ALTERID) the approval showed, never
/// reuse that match (see #632 for the design). Refused through the tool, under
/// an approving script: no Tally request, no approval asked, no attempt.
#[tokio::test]
async fn post_import_refuses_an_amendment_before_any_read_or_approval() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (original, _) = saved_batch(&server);
    let mut amendment = original.clone();
    amendment.batch_id = "bridge-00000000-0000-4000-8000-000000000632".into();
    amendment.amends_batch_id = Some(original.batch_id.clone());
    amendment.vouchers[0].entries[0].amount = "13.50".into();
    amendment.vouchers[0].entries[1].amount = "13.50".into();
    let rendered = render_import_xml(
        "WR2 Unicode Lab",
        &amendment.vouchers,
        amendment.identity_batch_id(),
    );
    amendment.sha256 = sha256_hex(rendered.as_bytes());
    server.append_import_ledger(&amendment).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", amendment.batch_id)),
        rendered,
    )
    .unwrap();
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool(
                "post_import",
                json!({"company_guid":GUID,"batch_id":amendment.batch_id}),
            ),
        )
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_post_amendment_requires_file_import",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(false), "{response}");
    assert!(observed.is_empty(), "no Tally request: {response}");
    assert!(scripted.previews().is_empty(), "no approval asked");
}

/// bridge#626: before approval, the post reads the catalogue again and refuses
/// a named ledger that now folds equal to another live ledger, which Tally's
/// import lookup could take for it. Refused through the tool, under an
/// approving script: no approval asked, no request after that catalogue read,
/// no intent journaled. The twin is a test-local rewrite of the capture (an
/// unrelated ledger renamed `Cash` plus CR LF), no evidence of Tally behaviour.
#[tokio::test]
async fn a_folded_twin_refuses_the_post_before_any_approval() {
    let captured = catalogue();
    assert_eq!(
        captured.matches("Bridge Nested Debtor WR4").count(),
        2,
        "name and NAME.LIST"
    );
    let twinned = captured.replace("Bridge Nested Debtor WR4", "Cash&#13;&#10;");
    let mut plans = Vec::new();
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(twinned));
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "ledger_has_folded_twin",
        "{response}"
    );
    assert_ne!(result["attempt_recorded"], json!(true), "{response}");
    assert_eq!(observed.len(), expected, "nothing after the catalogue read");
    assert!(scripted.previews().is_empty(), "no approval asked");
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
}

/// Declined, the post sends nothing past the pre-approval reads and journals
/// no intent; the approval was asked once.
#[tokio::test]
async fn a_declined_post_sends_nothing_and_journals_no_intent() {
    let expected = before_approval().len();
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let before = journal(directory.path());
    let scripted = ScriptedApproval::declining();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_approval_declined",
        "{response}"
    );
    assert_ne!(result["attempt_recorded"], json!(true), "{response}");
    assert_eq!(observed.len(), expected);
    assert_eq!(
        appended_kinds(&before, &journal(directory.path())),
        ["verification_status"]
    );
    // The readback before the post left a proof of a batch never sent: it says
    // the batch is not verified, and must not forbid sending it (bridge#804).
    let markdown = fs::read_to_string(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.proof.md", line.batch_id)),
    )
    .unwrap();
    assert!(
        markdown.contains("**Not verified — this report does not confirm posting.**"),
        "{markdown}"
    );
    assert!(
        markdown.contains("- Verification status: `verification_incomplete`"),
        "{markdown}"
    );
    assert!(!markdown.contains("resend"), "{markdown}");
    assert_eq!(
        scripted.previews(),
        [agent_review_preview(&line, &server.settings.endpoint).unwrap()]
    );
    assert_eq!(
        scripted.counts(),
        [1],
        "the dialog is asked about one voucher"
    );
}

/// The dispatch intent is in the journal before the POST is received. The
/// simulator holds the POST's response for three seconds; while it is held,
/// with the POST read and nothing answered, the journal already carries the
/// intent. This proves an append-before-send order, not the fsync itself.
#[tokio::test]
async fn the_dispatch_intent_is_journaled_before_the_post_is_received() {
    let held = xml(created_one()).with_delivery(Delivery::SlowHeaders(Duration::from_secs(3)));
    let mut plans = before_approval();
    let post_at = plans.len() + after_approval(held.clone()).len() - 1;
    plans.extend(after_approval(held));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let before = journal(directory.path());
    let post = SCRIPTED_APPROVAL.scope(
        ScriptedApproval::approving(),
        server.call_tool("post_import", args),
    );
    let watch = async {
        let started = std::time::Instant::now();
        while simulator.received() <= post_at {
            assert!(
                started.elapsed() < Duration::from_secs(20),
                "the POST never arrived"
            );
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        appended_kinds(&before, &journal(directory.path()))
    };
    let (response, while_held) = tokio::join!(post, watch);
    assert_eq!(
        while_held.last().map(String::as_str),
        Some("dispatch_intent"),
        "{response}"
    );
    assert!(!while_held.iter().any(|kind| kind == "dispatch_response"));
    let observed = sent(simulator);
    assert!(observed.len() > post_at);
    assert_journaled_clean_create(directory.path());
}

/// A dispatch admission that fails stops the send. While the approval is
/// pending the batch's record changes, so the admission that journals the
/// intent refuses it: no intent is written and nothing follows the absence
/// reads. The POST's plan, and one after it, stay in the sequence, so a post
/// that sent anyway would be served and observed here, not refused unseen.
#[tokio::test]
async fn a_dispatch_admission_that_fails_sends_nothing() {
    let mut plans = before_approval();
    let expected = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let mut changed = line.clone();
    changed.sha256 = "0".repeat(64);
    let data_dir = directory.path().to_path_buf();
    let endpoint = server.settings.endpoint.clone();
    let scripted = ScriptedApproval::approving_after(move || {
        let other = Server::new(crate::agent::Settings {
            endpoint: endpoint.clone(),
            data_dir: data_dir.clone(),
            max_rows: 10,
            max_bytes: 200_000,
            redaction: crate::agent::Redaction::None,
            import_enabled: true,
            writes_enabled: true,
            batch_post_enabled: false,
        });
        other.append_import_ledger(&changed).unwrap();
    });
    let response = SCRIPTED_APPROVAL
        .scope(scripted, server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_eq!(observed.len(), expected, "{response}");
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
    assert_ne!(
        response["structuredContent"]["result"]["attempt_recorded"],
        json!(true),
        "{response}"
    );
}

// ADR 0004 as amended 2026-09-22: one Payment, Receipt or Contra posts under
// every Journal safeguard, and its legs are classified again from the ledgers'
// parents and the group tree before approval and inside the queue.

const BANK_BATCH: &str = "bridge-00000000-0000-4000-8000-000000000466";

fn groups() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-groups.utf16le.xml"
    ))
}

/// `body` with the one occurrence of `from` replaced by `to`.
fn replaced_once(body: &str, from: &str, to: &str) -> String {
    assert_eq!(body.matches(from).count(), 1, "{from} must occur once");
    body.replace(from, to)
}

/// The counterparty ledger moved directly under a cash group.
fn catalogue_with_debtor_under_cash() -> String {
    replaced_once(
        &catalogue(),
        ">Bridge Nested Debtors WR4</PARENT>",
        ">Cash-in-Hand</PARENT>",
    )
}

/// The counterparty ledger's own group moved under Bank Accounts; the ledger
/// row itself is byte-identical.
fn groups_with_debtor_group_under_bank() -> String {
    replaced_once(
        &groups(),
        ">Sundry Debtors</PARENT>",
        ">Bank Accounts</PARENT>",
    )
}

/// A second money ledger, for a Contra: `WR2 Sales` read as a bank account in
/// every read of the run.
fn catalogue_with_sales_as_bank() -> String {
    replaced_once(
        &catalogue(),
        ">Sales Accounts</PARENT>",
        ">Bank Accounts</PARENT>",
    )
}

/// `before_approval`, for a bank voucher: the post's classification reads the
/// group collection after the catalogue, before the qualified mode.
fn bank_before_approval(catalogue: String, groups: String) -> Vec<ScenarioPlan> {
    bank_before_approval_with_currencies(catalogue, groups, single_currency())
}

fn bank_before_approval_with_currencies(
    catalogue: String,
    groups: String,
    currencies: String,
) -> Vec<ScenarioPlan> {
    let mut plans = Vec::new();
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(catalogue));
    plans.extend(paired(groups));
    plans.extend(paired(currencies));
    plans.extend(probe());
    plans
}

/// `after_approval`, for a bank voucher: the queue re-reads the group
/// collection right after the catalogue, inside the same admission brackets.
fn bank_after_approval(catalogue: String, groups: String, post: ScenarioPlan) -> Vec<ScenarioPlan> {
    bank_after_approval_with_currencies(catalogue, groups, single_currency(), post)
}

fn bank_after_approval_with_currencies(
    catalogue: String,
    groups: String,
    currencies: String,
    post: ScenarioPlan,
) -> Vec<ScenarioPlan> {
    let mut plans = probe();
    plans.push(xml(companies()));
    plans.push(marks_at_binding());
    plans.extend(paired(catalogue));
    plans.extend(paired(groups));
    plans.extend(paired(currencies));
    plans.extend(probe());
    plans.push(xml(companies()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.push(marks_before());
    plans.push(post);
    plans
}

/// A built, never-dispatched single-voucher bank batch, as `build_import_xml`
/// leaves one.
fn saved_bank_batch(server: &Server, voucher: Value) -> (ImportLedgerLine, Value) {
    saved_bank_batch_recording(server, voucher, json!([]))
}

/// `saved_bank_batch`, with `cash_in_hand` as the cash-in-hand ledgers its
/// build recorded (#815); `null` for a record written before the field.
fn saved_bank_batch_recording(
    server: &Server,
    voucher: Value,
    cash_in_hand: Value,
) -> (ImportLedgerLine, Value) {
    saved_bank_batch_recording_all(server, voucher, cash_in_hand, json!([]))
}

/// As `saved_bank_batch_recording`, with `on_account` as the bill-wise
/// approvals its build recorded (#1234); `null` for a record written before.
fn saved_bank_batch_recording_all(
    server: &Server,
    voucher: Value,
    cash_in_hand: Value,
    on_account: Value,
) -> (ImportLedgerLine, Value) {
    let origin = super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let mut line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":BANK_BATCH, "identity_scheme":"batch_v1",
        "company_guid":GUID,
        "endpoint_origin":origin,
        "company":{"name":"WR2 Unicode Lab","guid":GUID,"company_number":"100004","books_from":"20260401"},
        "txn_ids":[voucher["bridge_txn_id"].clone()],"date_from":"20260901","date_to":"20260901",
        "sha256":"", "built_at":"2026-09-22T00:00:00Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":10,"master_value":7},
        "vouchers":[voucher]
    }))
    .unwrap();
    let rendered = render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(rendered.as_bytes());
    bind_to_captured_catalogue(&mut line);
    line.cash_in_hand_ledgers = serde_json::from_value(cash_in_hand).unwrap();
    line.on_account_approved = serde_json::from_value(on_account).unwrap();
    server.append_import_ledger(&line).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        rendered,
    )
    .unwrap();
    let args = json!({"company_guid":GUID,"batch_id":line.batch_id});
    (line, args)
}

fn payment() -> Value {
    json!({"bridge_txn_id":"payment-466","date":"20260901","voucher_type":"Payment",
        "narration":"Synthetic test only","entries":[
            {"ledger":"Bridge Nested Debtor WR4","amount":"12.50","side":"Dr"},
            {"ledger":"Cash","amount":"12.50","side":"Cr"}]})
}

fn receipt() -> Value {
    json!({"bridge_txn_id":"receipt-466","date":"20260901","voucher_type":"Receipt",
        "narration":"Synthetic test only","entries":[
            {"ledger":"Cash","amount":"7.50","side":"Dr"},
            {"ledger":"Bridge Nested Debtor WR4","amount":"7.50","side":"Cr"}]})
}

fn contra() -> Value {
    json!({"bridge_txn_id":"contra-466","date":"20260901","voucher_type":"Contra",
        "narration":"Synthetic test only","entries":[
            {"ledger":"WR2 Sales","amount":"5.00","side":"Dr"},
            {"ledger":"Cash","amount":"5.00","side":"Cr"}]})
}

/// Each bank type posts through the tool call exactly as a Journal does: the
/// POST is the request its intent recorded, rendered by the code's own
/// renderer from the recorded REMOTEID, and the approval was asked about a
/// preview that names the type and the classification it relies on.
#[tokio::test]
async fn each_bank_type_posts_the_request_its_intent_recorded() {
    for (voucher, catalogue, type_name, rule) in [
        (
            payment(),
            catalogue(),
            "Payment",
            "every Cr ledger is bank/cash; every Dr ledger holds no money",
        ),
        (
            receipt(),
            catalogue(),
            "Receipt",
            "every Dr ledger is bank/cash; every Cr ledger holds no money",
        ),
        (
            contra(),
            catalogue_with_sales_as_bank(),
            "Contra",
            "every ledger is bank/cash",
        ),
    ] {
        let mut plans = bank_before_approval(catalogue.clone(), groups());
        let after = bank_after_approval(catalogue.clone(), groups(), xml(created_one()));
        let post_at = plans.len() + after.len() - 1;
        plans.extend(after);
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (line, args) = saved_bank_batch(&server, voucher);
        let scripted = ScriptedApproval::approving();
        let response = SCRIPTED_APPROVAL
            .scope(scripted.clone(), server.call_tool("post_import", args))
            .await;
        let observed = sent(simulator);
        assert_eq!(observed.len(), post_at + 1, "{type_name}: {response}");

        let intent = dispatch_intent(directory.path());
        assert_journaled_clean_create(directory.path());
        let recorded_sha = intent["native_request_sha256"].as_str().unwrap();
        let remote_id = Uuid::parse_str(intent["native_remote_id"].as_str().unwrap()).unwrap();
        assert_eq!(
            observed[post_at].request_body_sha256, recorded_sha,
            "{type_name}"
        );
        let rendered = native_post_request(&line, RemoteIds::from_ids(vec![remote_id])).unwrap();
        assert_eq!(rendered.request_sha256, recorded_sha, "{type_name}");
        assert!(
            rendered.xml.contains(&format!("VCHTYPE=\"{type_name}\"")),
            "{type_name}: {}",
            rendered.xml
        );
        let previews = scripted.previews();
        assert_eq!(
            previews,
            [agent_review_preview(&line, &server.settings.endpoint).unwrap()]
        );
        assert!(
            previews[0].starts_with(&format!("Create ONE {type_name} in ")),
            "{}",
            previews[0]
        );
        assert!(
            previews[0].contains(&format!("Checked in Tally: {rule}.")),
            "{}",
            previews[0]
        );
    }
}

fn three_entry_receipt() -> Value {
    json!({"bridge_txn_id":"receipt-3-466","date":"20260901","voucher_type":"Receipt",
        "narration":"Synthetic test only","entries":[
            {"ledger":"Cash","amount":"3.00","side":"Dr"},
            {"ledger":"Bridge Nested Debtor WR4","amount":"1.00","side":"Cr"},
            {"ledger":"Café Naïve Traders","amount":"2.00","side":"Cr"}]})
}

/// The SECOND counterparty of the three-entry Receipt moved under a cash group;
/// the first counterparty's row is byte-identical.
fn catalogue_with_second_counterparty_under_cash() -> String {
    let body = catalogue();
    let start = body
        .find("<LEDGER NAME=\"Café Naïve Traders\"")
        .expect("ledger row");
    let end = start + body[start..].find("</LEDGER>").expect("row end");
    let row = &body[start..end];
    let moved = replaced_once(row, ">Sundry Debtors</PARENT>", ">Cash-in-Hand</PARENT>");
    format!("{}{}{}", &body[..start], moved, &body[end..])
}

/// A multi-entry bank voucher (bridge#466, #585) posts like any other: the
/// preview lists every entry, and the POST is the recorded request.
#[tokio::test]
async fn a_three_entry_receipt_posts_the_request_its_intent_recorded() {
    let mut plans = bank_before_approval(catalogue(), groups());
    let after = bank_after_approval(catalogue(), groups(), xml(created_one()));
    let post_at = plans.len() + after.len() - 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_bank_batch(&server, three_entry_receipt());
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_eq!(observed.len(), post_at + 1, "{response}");
    let intent = dispatch_intent(directory.path());
    assert_journaled_clean_create(directory.path());
    let recorded_sha = intent["native_request_sha256"].as_str().unwrap();
    assert_eq!(observed[post_at].request_body_sha256, recorded_sha);
    let remote_id = Uuid::parse_str(intent["native_remote_id"].as_str().unwrap()).unwrap();
    assert_eq!(
        native_post_request(&line, RemoteIds::from_ids(vec![remote_id]))
            .unwrap()
            .request_sha256,
        recorded_sha
    );
    let previews = scripted.previews();
    assert_eq!(previews.len(), 1);
    for entry in [
        "Dr 3.00  \"Cash\"",
        "Cr 1.00  \"Bridge Nested Debtor WR4\"",
        "Cr 2.00  \"Café Naïve Traders\"",
    ] {
        assert!(previews[0].contains(entry), "{entry}: {}", previews[0]);
    }
}

/// Every leg is classified again in the queue, not only the first on each
/// side: the second counterparty turning into money refuses the post.
#[tokio::test]
async fn a_second_counterparty_moved_under_cash_after_approval_is_refused() {
    let mut plans = bank_before_approval(catalogue(), groups());
    let after = bank_after_approval(
        catalogue_with_second_counterparty_under_cash(),
        groups(),
        xml(created_one()),
    );
    let expected = plans.len() + after.len() - 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_bank_batch(&server, three_entry_receipt());
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"],
        "import_bank_classification_changed",
        "{response}"
    );
    assert_eq!(observed.len(), expected, "{response}");
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
}

/// Refused inside the queue, after approval: no intent, and nothing past the
/// queued reads. The POST's plan and one after it stay in the sequence, so a
/// post that went ahead would be served and observed here.
async fn refused_in_the_queue(queued_catalogue: String, queued_groups: String) {
    let mut plans = bank_before_approval(catalogue(), groups());
    let after = bank_after_approval(queued_catalogue, queued_groups, xml(created_one()));
    let expected = plans.len() + after.len() - 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_bank_batch(&server, payment());
    let before = journal(directory.path());
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_bank_classification_changed",
        "{response}"
    );
    assert_eq!(scripted.previews().len(), 1, "approval was asked once");
    assert_eq!(observed.len(), expected, "{response}");
    assert_eq!(
        appended_kinds(&before, &journal(directory.path())),
        ["verification_status"]
    );
}

/// A ledger re-parented after approval keeps its name and GUID, so the
/// catalogue binding still matches; only the classification sees it.
#[tokio::test]
async fn a_counterparty_moved_under_cash_after_approval_is_refused_before_the_post() {
    refused_in_the_queue(catalogue_with_debtor_under_cash(), groups()).await;
}

/// A group re-parented after approval changes no ledger row at all.
#[tokio::test]
async fn a_counterparty_group_moved_under_bank_after_approval_is_refused_before_the_post() {
    refused_in_the_queue(catalogue(), groups_with_debtor_group_under_bank()).await;
}

/// A business-cash Contra as its build leaves it (#815): the bank cash line
/// answered "business cash" debits the cash-in-hand ledger `Cash`, which the
/// build recorded, and credits the bank.
fn saved_business_cash_contra(server: &Server) -> Value {
    let contra = json!({"bridge_txn_id":"contra-815","date":"20260901","voucher_type":"Contra",
        "narration":"Synthetic test only","entries":[
            {"ledger":"Cash","amount":"5.00","side":"Dr"},
            {"ledger":"WR2 Sales","amount":"5.00","side":"Cr"}]});
    let recorded = json!([{"bridge_txn_id":"contra-815","ledger":"Cash"}]);
    saved_bank_batch_recording(server, contra, recorded).1
}

/// `Cash` re-parented under Bank Accounts beside `WR2 Sales` as a bank: both
/// legs are still money, so the bank/cash gate admits the Contra, which now
/// moves money bank to bank (#815).
fn catalogue_with_cash_under_bank() -> String {
    replaced_once(
        &catalogue_with_sales_as_bank(),
        ">Cash-in-Hand</PARENT>",
        ">Bank Accounts</PARENT>",
    )
}

const CASH_MOVED_MESSAGE: &str =
    "A ledger named as cash in hand when this batch was built (listed in \
     error.refused_ledgers) is no longer under Cash-in-Hand: its group now reaches the reserved \
     group shown. Nothing was posted. Put the ledger back under Cash-in-Hand, or build the batch \
     again with the cash-in-hand ledger.";

/// The refusal row for `Cash` under Bank Accounts, as the build reports it.
fn cash_under_bank_row() -> Value {
    json!([{"ledger":"Cash","requires":"cash_in_hand","state":"cash_bank",
        "reserved_group":"Bank Accounts","first_bridge_txn_id":"contra-815"}])
}

/// #815: a ledger the build recorded as cash in hand, re-parented under Bank
/// Accounts since, is refused before approval with the build's own code and
/// row. Nothing after the group read is sent, and no approval is asked.
#[tokio::test]
async fn a_cash_in_hand_ledger_moved_under_bank_since_the_build_is_refused_before_approval() {
    let mut plans = bank_before_approval(catalogue_with_cash_under_bank(), groups());
    plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_business_cash_contra(&server);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "cash_ledger_not_cash_in_hand", "{response}");
    assert_eq!(
        error["refused_ledgers"],
        cash_under_bank_row(),
        "{response}"
    );
    assert_eq!(error["refused_ledgers_omitted"], 0, "{response}");
    assert_eq!(error["message"], CASH_MOVED_MESSAGE, "{response}");
    assert!(scripted.previews().is_empty(), "approval must not be asked");
    assert_eq!(observed.len(), expected, "{response}");
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
}

/// #815: the same move made after approval is refused in the queue, before
/// the POST, with the same code and row. The approval was asked once, on a
/// book where `Cash` was still under Cash-in-Hand.
#[tokio::test]
async fn a_cash_in_hand_ledger_moved_under_bank_after_approval_is_refused_in_the_queue() {
    let mut plans = bank_before_approval(catalogue_with_sales_as_bank(), groups());
    let after = bank_after_approval(
        catalogue_with_cash_under_bank(),
        groups(),
        xml(created_one()),
    );
    let expected = plans.len() + after.len() - 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_business_cash_contra(&server);
    let before = journal(directory.path());
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "cash_ledger_not_cash_in_hand", "{response}");
    assert_eq!(
        error["refused_ledgers"],
        cash_under_bank_row(),
        "{response}"
    );
    assert_eq!(error["refused_ledgers_omitted"], 0, "{response}");
    assert_eq!(error["message"], CASH_MOVED_MESSAGE, "{response}");
    assert_eq!(scripted.previews().len(), 1, "approval was asked once");
    assert_eq!(observed.len(), expected, "{response}");
    assert_eq!(
        appended_kinds(&before, &journal(directory.path())),
        ["verification_status"]
    );
}

/// #815: a batch recorded before its cash-in-hand ledgers were has nothing to
/// check again, so it is refused before any Tally request and must be rebuilt,
/// as a batch built before ledger binding is (#239).
#[tokio::test]
async fn a_batch_recorded_before_its_cash_in_hand_ledgers_is_refused_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_bank_batch_recording(&server, contra(), Value::Null);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_batch_predates_cash_ledger_record",
        "{response}"
    );
    assert_eq!(
        result["error"]["message"],
        "This batch was built before ComplyEaze Bridge recorded which of its ledgers must stay \
         under Cash-in-Hand, so it cannot be checked. Nothing was posted. Build the batch again, \
         then post the new batch.",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], false, "{response}");
    assert!(scripted.previews().is_empty(), "approval must not be asked");
    assert!(observed.is_empty(), "{response}");
}

/// #1234: a batch saved before the build recorded its bill-wise approvals is
/// refused before any Tally request and must be rebuilt, as above.
#[tokio::test]
async fn a_batch_recorded_before_its_bill_wise_approvals_is_refused_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_bank_batch_recording_all(&server, contra(), json!([]), Value::Null);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_batch_predates_bill_wise_record",
        "{response}"
    );
    assert_eq!(
        result["error"]["message"],
        "This batch was built before ComplyEaze Bridge began checking ledgers that keep \
         bills in Tally, so it cannot be checked. Nothing was posted. First check in Tally \
         whether its file was already imported by hand, since posting the rebuilt batch would import it a second time. \
         Then build the batch again and post the new batch.",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], false, "{response}");
    assert!(scripted.previews().is_empty(), "approval must not be asked");
    assert!(observed.is_empty(), "{response}");
}

/// A recorded cash ledger moved under a group that holds no money meets the
/// bank/cash gate first: a Contra's every leg must be money, so it is refused
/// as `import_bank_classification_changed` and never reaches the cash-in-hand
/// recheck, whose rows therefore always name a money group (#815, review P3).
#[tokio::test]
async fn a_cash_in_hand_ledger_moved_out_of_money_is_refused_by_the_bank_gate_first() {
    let moved = replaced_once(
        &catalogue_with_sales_as_bank(),
        ">Cash-in-Hand</PARENT>",
        ">Sundry Debtors</PARENT>",
    );
    let mut plans = bank_before_approval(moved, groups());
    plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_business_cash_contra(&server);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let _ = sent(simulator);
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(
        error["code"], "import_bank_classification_changed",
        "{response}"
    );
    assert!(error.get("refused_ledgers").is_none(), "{response}");
    assert!(scripted.previews().is_empty(), "approval must not be asked");
}

/// A cash-in-hand ledger rides only on a business-cash Contra. A Journal
/// whose record names one would skip the recheck, which runs only beside the
/// bank/cash gate, so it is refused before approval as inconsistent, after the
/// catalogue and before the Currency read (#815, review P3).
#[tokio::test]
async fn a_journal_recording_a_cash_in_hand_ledger_is_refused_before_approval() {
    let mut plans = before_approval();
    plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let journal_voucher = json!({"bridge_txn_id":"journal-815","date":"20260901",
        "voucher_type":"Journal","narration":"Synthetic test only","entries":[
            {"ledger":"Bridge Nested Debtor WR4","amount":"5.00","side":"Dr"},
            {"ledger":"Cash","amount":"5.00","side":"Cr"}]});
    let (_, args) = saved_bank_batch_recording(
        &server,
        journal_voucher,
        json!([{"bridge_txn_id":"journal-815","ledger":"Cash"}]),
    );
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_post_admission_inconsistent",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], false, "{response}");
    assert!(scripted.previews().is_empty(), "approval must not be asked");
    assert_eq!(observed.len(), expected, "{response}");
}

/// Before approval the rows are bounded as the queue bounds them
/// (`RECHECK_REFUSAL_BUDGET`), so both answers list the same rows for one
/// regroup (#815, review P3). The record is built here with twelve ledgers
/// the book does not hold, more than that budget lists; a build never records
/// those, which is the only way to reach the bound with this capture.
#[tokio::test]
async fn the_recheck_before_approval_bounds_its_rows_as_the_queue_does() {
    let mut plans = bank_before_approval(catalogue_with_sales_as_bank(), groups());
    plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let contra = json!({"bridge_txn_id":"contra-815","date":"20260901","voucher_type":"Contra",
        "narration":"Synthetic test only","entries":[
            {"ledger":"Cash","amount":"5.00","side":"Dr"},
            {"ledger":"WR2 Sales","amount":"5.00","side":"Cr"}]});
    let recorded = (0..12)
        .map(|index| json!({"bridge_txn_id":"contra-815","ledger":format!("Synthetic absent cash ledger {index:02}")}))
        .collect::<Vec<_>>();
    let (_, args) = saved_bank_batch_recording(&server, contra, Value::Array(recorded));
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let _ = sent(simulator);
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "cash_ledger_not_cash_in_hand", "{response}");
    let listed = error["refused_ledgers"].as_array().unwrap().len();
    let omitted = error["refused_ledgers_omitted"].as_u64().unwrap() as usize;
    assert_eq!(listed + omitted, 12, "{response}");
    // The server's own budget (200,000 bytes) would list all twelve.
    assert!(omitted > 0, "{response}");
}

/// #869: a queue read before the intent that meets a busy wire lock refuses
/// the post as `tally_endpoint_busy`, with when to try again and what to do:
/// nothing is recorded or sent, and the approval lapses. `lease_send` is the
/// read's place in the lease (`after_approval`); the wire gate refuses every
/// try of that send, as another process holding the lock for it would.
async fn refused_busy_at_lease_send(lease_send: usize) {
    let mut plans = before_approval();
    let busy_send = plans.len() + lease_send;
    plans.extend(after_approval(xml(created_one())));
    plans.truncate(busy_send);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    server.runtime = crate::tally::TallyRuntime::default().with_wire_gate_config(
        crate::tally::TallyRuntime::default()
            .wire_gate_config()
            .clone()
            .busy_at_send(busy_send, std::time::Duration::from_millis(50)),
    );
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"],
        json!({
            "code": "tally_endpoint_busy",
            "message": "No posting attempt was recorded. Review the error before requesting approval again.",
            "retry_after_s": bridge_tally_transport::WIRE_BUSY_RETRY_AFTER.as_secs(),
            "next_step": "Nothing was posted: Tally's port was busy. Call post_import with this same batch again after retry_after_s seconds, once Tally is free. Any approval already given has lapsed, so the person is asked to approve it again. Do not rebuild the batch.",
        }),
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], false, "{response}");
    assert_eq!(scripted.previews().len(), 1, "approval was asked once");
    // Every send before the busy one, and nothing after it: no POST.
    assert_eq!(observed.len(), busy_send, "{response}");
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
    assert_eq!(
        server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "post_refused_before_intent"
    );
}

#[tokio::test]
async fn a_busy_wire_lock_at_the_queues_binding_marks_read_refuses_before_the_intent() {
    refused_busy_at_lease_send(3).await;
}

#[tokio::test]
async fn a_busy_wire_lock_at_the_queues_aim_marks_read_refuses_before_the_intent() {
    refused_busy_at_lease_send(after_approval(xml(created_one())).len() - 2).await;
}

/// bridge#676: a group collection the classification cannot parse is refused
/// before approval as `group_export_invalid`, and its `cause` is the group
/// parser's own data-free code, not dropped. Nothing is read after it.
async fn refused_on_the_group_read(groups: String, cause: &str) {
    let mut plans = probe();
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(catalogue()));
    plans.extend(paired(groups));
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_bank_batch(&server, payment());
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "group_export_invalid", "{response}");
    assert_eq!(error["cause"], cause, "{response}");
    assert!(scripted.previews().is_empty(), "no approval asked");
    assert_eq!(observed.len(), expected, "{response}");
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
}

#[tokio::test]
async fn a_group_collection_of_another_company_is_refused_with_its_cause() {
    let groups = groups();
    let other = groups.replacen(
        ">61c6de69-1748-461c-ad3f-162cb949df9f</BRIDGECOMPANYGUID>",
        ">00000000-0000-4000-8000-000000000676</BRIDGECOMPANYGUID>",
        1,
    );
    assert_ne!(other, groups, "one row's company GUID changed");
    refused_on_the_group_read(other, "group_response_company_guid_mismatch").await;
}

#[tokio::test]
async fn a_group_collection_that_reports_failure_is_refused_with_its_cause() {
    let failed = replaced_once(&groups(), "<STATUS>1</STATUS>", "<STATUS>0</STATUS>");
    refused_on_the_group_read(failed, "group_status_not_success").await;
}

/// bridge#717: a group collection with no STATUS answer names its own cause,
/// not Tally's failure answer.
#[tokio::test]
async fn a_group_collection_without_a_status_answer_is_refused_as_status_absent() {
    let silent = replaced_once(&groups(), "<STATUS>1</STATUS>", "<STATUS/>");
    refused_on_the_group_read(silent, "group_status_absent").await;
}

/// bridge#717: the group collection the queue re-reads after approval is
/// refused as `group_export_invalid` with the same data-free `cause` the read
/// before approval carries, not as a causeless queue failure. Nothing is sent
/// and no intent is written.
async fn refused_on_the_queued_group_read(queued_groups: String, cause: &str) {
    let mut plans = bank_before_approval(catalogue(), groups());
    let after = bank_after_approval(catalogue(), queued_groups, xml(created_one()));
    let expected = plans.len() + after.len() - 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_bank_batch(&server, payment());
    let before = journal(directory.path());
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "group_export_invalid",
        "{response}"
    );
    assert_eq!(result["error"]["cause"], cause, "{response}");
    assert_eq!(result["attempt_recorded"], false, "{response}");
    assert_eq!(scripted.previews().len(), 1, "approval was asked once");
    assert_eq!(
        observed.len(),
        expected,
        "the post is never sent: {response}"
    );
    assert_eq!(
        appended_kinds(&before, &journal(directory.path())),
        ["verification_status"]
    );
}

#[tokio::test]
async fn a_queued_group_collection_of_another_company_is_refused_with_its_cause() {
    let groups = groups();
    let other = groups.replacen(
        ">61c6de69-1748-461c-ad3f-162cb949df9f</BRIDGECOMPANYGUID>",
        ">00000000-0000-4000-8000-000000000717</BRIDGECOMPANYGUID>",
        1,
    );
    assert_ne!(other, groups, "one row's company GUID changed");
    refused_on_the_queued_group_read(other, "group_response_company_guid_mismatch").await;
}

#[tokio::test]
async fn a_queued_group_collection_that_reports_failure_is_refused_with_its_cause() {
    let failed = replaced_once(&groups(), "<STATUS>1</STATUS>", "<STATUS>0</STATUS>");
    refused_on_the_queued_group_read(failed, "group_status_not_success").await;
}

#[tokio::test]
async fn a_queued_group_collection_without_a_status_answer_is_refused_as_status_absent() {
    let silent = replaced_once(&groups(), "<STATUS>1</STATUS>", "");
    refused_on_the_queued_group_read(silent, "group_status_absent").await;
}

/// Already changed since the build: refused before approval is asked, and no
/// request follows the classification reads.
#[tokio::test]
async fn a_classification_changed_since_the_build_is_refused_before_approval() {
    for (catalogue, groups) in [
        (catalogue_with_debtor_under_cash(), groups()),
        (catalogue(), groups_with_debtor_group_under_bank()),
    ] {
        let mut plans = bank_before_approval(catalogue, groups);
        // The Currency masters and the qualified mode probe after the group
        // read are never sent.
        plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
        let expected = plans.len();
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (_, args) = saved_bank_batch(&server, payment());
        let scripted = ScriptedApproval::approving();
        let response = SCRIPTED_APPROVAL
            .scope(scripted.clone(), server.call_tool("post_import", args))
            .await;
        let observed = sent(simulator);
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"],
            "import_bank_classification_changed",
            "{response}"
        );
        assert!(scripted.previews().is_empty(), "approval must not be asked");
        assert_eq!(observed.len(), expected, "{response}");
        assert!(!String::from_utf8(journal(directory.path()))
            .unwrap()
            .contains("\"dispatch_intent\""));
    }
}

/// The desktop's post stays Journal-only: the same saved Payment the agent can
/// post is refused before any request.
#[tokio::test]
async fn the_desktop_scope_refuses_a_bank_voucher_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_bank_batch(&server, payment());
    let refused = server
        .post_import_checked(&args, Some(&line.sha256), PostScope::JournalOnly)
        .await
        .expect("a refusal is reported as the post's outcome");
    assert_eq!(
        refused.payload["result"]["error"]["code"], "import_post_requires_one_journal",
        "{}",
        refused.payload
    );
    assert!(sent(simulator).is_empty());
}

// bridge#574: the aim is confirmed on the snapshot sent last before the POST,
// and where the voucher went is read straight after it.

/// The approved Journal post, with `marks` in place of the snapshot sent last
/// before the POST. Returns the response, the requests observed, and the
/// number of requests up to and including that snapshot.
async fn post_with_marks_before(marks: String) -> (Value, usize, usize, Vec<u8>, Vec<u8>) {
    let mut plans = before_approval();
    let mut after = after_approval(xml(created_one()));
    let marks_at = after.len() - 2;
    after[marks_at] = xml(marks);
    let through_marks = plans.len() + marks_at + 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let before = journal(directory.path());
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator).len();
    (
        response,
        observed,
        through_marks,
        before,
        journal(directory.path()),
    )
}

#[tokio::test]
async fn another_company_renamed_to_the_target_refuses_before_the_post() {
    // The residual #574 case: another loaded company now carries the target's
    // name, so a post named by SVCURRENTCOMPANY could land in it.
    let namesake =
        company_marks(10, 50, "WR2 Unicode Lab").replace("Synthetic Other Lab", "WR2 UNICODE LAB");
    let (response, observed, through_marks, before, after) = post_with_marks_before(namesake).await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "post_company_scope_changed",
        "{response}"
    );
    assert_eq!(
        observed, through_marks,
        "nothing after the snapshot: {response}"
    );
    assert_eq!(appended_kinds(&before, &after), ["verification_status"]);
}

#[tokio::test]
async fn the_target_renamed_since_admission_refuses_before_the_post() {
    let renamed = company_marks(10, 50, "WR2 Unicode Lab Renamed");
    let (response, observed, through_marks, before, after) = post_with_marks_before(renamed).await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "post_company_scope_changed",
        "{response}"
    );
    assert_eq!(observed, through_marks, "{response}");
    assert_eq!(appended_kinds(&before, &after), ["verification_status"]);
}

#[tokio::test]
async fn an_unreadable_snapshot_refuses_before_the_post() {
    // T2's live answer to an unmatched SVCURRENTCOMPANY.
    let refused = "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>0</STATUS></HEADER><BODY><DATA>\
                   <LINEERROR>Could not set 'SVCurrentCompany' to 'WR2 Unicode Lab'</LINEERROR>\
                   </DATA></BODY></ENVELOPE>"
        .to_string();
    let (response, observed, through_marks, before, after) = post_with_marks_before(refused).await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "post_company_scope_unconfirmed",
        "{response}"
    );
    assert_eq!(observed, through_marks, "{response}");
    assert_eq!(appended_kinds(&before, &after), ["verification_status"]);
}

/// After the POST, the snapshot says which companies' voucher marks moved,
/// and the result carries it even when the readback that follows fails.
async fn located_after(marks_after: String) -> Value {
    located_after_response(created_one(), marks_after).await
}

async fn located_after_response(post_response: String, marks_after: String) -> Value {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(post_response.clone())));
    plans.push(xml(marks_after));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let _ = sent(simulator);
    assert_eq!(
        dispatch_intent(directory.path())["record_type"],
        "dispatch_intent"
    );
    let created = parse_import_outcome(&post_response)
        .expect("the POST answer parses")
        .counters()
        .created;
    assert_eq!(
        journaled_outcome(directory.path()).map(|outcome| outcome.counters().created),
        Some(created),
        "the journaled outcome is the parsed POST answer"
    );
    response["structuredContent"]["result"]["post_location"].clone()
}

#[tokio::test]
async fn only_the_target_moving_is_reported_as_the_landing() {
    let located = located_after(company_marks(11, 50, "WR2 Unicode Lab")).await;
    assert_eq!(located["state"], "target_only", "{located}");
    // The captured answer reports one create, and the target's mark moved by one.
    assert_eq!(
        located["target_voucher_step"],
        json!({"before": 10, "after": 11, "step": 1, "reported_created": 1, "matches_created": true}),
        "{located}"
    );
}

/// A step larger than Tally's CREATED means the post altered or cancelled
/// vouchers itself, or another voucher in the target changed around it
/// (protocol reference §11c.5). It is reported in `post_location`.
#[tokio::test]
async fn a_target_step_beyond_the_create_is_reported() {
    let located = located_after(company_marks(12, 50, "WR2 Unicode Lab")).await;
    assert_eq!(located["state"], "target_only", "{located}");
    assert_eq!(located["target_voucher_step"]["step"], 2, "{located}");
    assert_eq!(
        located["target_voucher_step"]["matches_created"], false,
        "{located}"
    );
}

#[tokio::test]
async fn another_company_moving_instead_is_named_in_the_result() {
    let located = located_after(company_marks(10, 51, "WR2 Unicode Lab")).await;
    assert_eq!(located["state"], "suspected_other_company", "{located}");
    assert_eq!(
        located["other_companies_moved"][0]["name"],
        "Synthetic Other Lab"
    );
}

#[tokio::test]
async fn a_snapshot_lost_in_transport_refuses_as_unconfirmed_not_as_an_unknown_outcome() {
    // The read that aims the post fails before any body arrives: nothing was
    // sent, so the code must say so rather than "outcome unknown".
    let mut plans = before_approval();
    let mut after = after_approval(xml(created_one()));
    let marks_at = after.len() - 2;
    after[marks_at] = marks_before().with_delivery(Delivery::ResetBeforeBody);
    let through_marks = plans.len() + marks_at + 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let before = journal(directory.path());
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator).len();
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "post_company_scope_unconfirmed",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(false), "{response}");
    assert_eq!(observed, through_marks, "{response}");
    assert_eq!(
        appended_kinds(&before, &journal(directory.path())),
        ["verification_status"]
    );
}

#[tokio::test]
async fn the_response_is_journaled_before_the_location_snapshot_is_answered() {
    // The snapshot after the POST is held for three seconds. While it is held
    // the journal already carries the dispatch response, so a slow or lost
    // location read cannot cost the record of the post.
    let mut plans = before_approval();
    let after = after_approval(xml(created_one()));
    let post_at = plans.len() + after.len() - 1;
    plans.extend(after);
    plans.push(
        xml(company_marks(11, 50, "WR2 Unicode Lab"))
            .with_delivery(Delivery::SlowHeaders(Duration::from_secs(3))),
    );
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let before = journal(directory.path());
    let post = SCRIPTED_APPROVAL.scope(
        ScriptedApproval::approving(),
        server.call_tool("post_import", args),
    );
    let watch = async {
        let started = std::time::Instant::now();
        while simulator.received() <= post_at + 1 {
            assert!(
                started.elapsed() < Duration::from_secs(20),
                "the location snapshot never arrived"
            );
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        appended_kinds(&before, &journal(directory.path()))
    };
    let (response, while_held) = tokio::join!(post, watch);
    assert!(
        while_held.iter().any(|kind| kind == "dispatch_response"),
        "{while_held:?} {response}"
    );
    let _ = sent(simulator);
    assert_journaled_clean_create(directory.path());
    assert_eq!(
        response["structuredContent"]["result"]["post_location"]["state"], "target_only",
        "{response}"
    );
}

#[tokio::test]
async fn another_company_moving_while_tally_created_nothing_is_not_blamed() {
    // Tally rejected the post while a colleague's voucher moved another
    // company's mark: that company must not be named as where this post went.
    // The rejection is a captured live response (CREATED 0, EXCEPTIONS 1, one
    // LINEERROR), the shape T7 measured for a post into a company lacking a
    // ledger.
    let rejected = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/live_education_w7_baddate_sanitized.xml"
    )
    .to_string();
    assert!(parse_import_outcome(&rejected).is_ok_and(|outcome| outcome.counters().created == 0));
    let located = located_after_response(rejected, company_marks(10, 51, "WR2 Unicode Lab")).await;
    assert_eq!(located["state"], "no_creation_reported", "{located}");
}

#[tokio::test]
async fn a_post_whose_response_cannot_be_journaled_still_reports_where_it_landed() {
    // The POST is sent and answered, but another process holds the admission
    // lock when the response is journaled, so that local write fails. The
    // location of a post that was sent must survive that failure.
    let held = xml(created_one()).with_delivery(Delivery::SlowHeaders(Duration::from_secs(2)));
    let mut plans = before_approval();
    let after = after_approval(held);
    let post_at = plans.len() + after.len() - 1;
    plans.extend(after);
    plans.push(xml(company_marks(11, 50, "WR2 Unicode Lab")));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let other = server_at(simulator.address(), directory.path());
    let post = SCRIPTED_APPROVAL.scope(
        ScriptedApproval::approving(),
        server.call_tool("post_import", args),
    );
    let hold_lock = async {
        let started = std::time::Instant::now();
        // Wait until the POST has been received, so the intent's own use of
        // the lock is over, then hold the lock while the response is written.
        while simulator.received() <= post_at {
            assert!(
                started.elapsed() < Duration::from_secs(20),
                "the POST never arrived"
            );
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        let lock = other
            .lock_import_admission()
            .expect("lock is free while the POST is held");
        tokio::time::sleep(Duration::from_secs(4)).await;
        drop(lock);
    };
    let (response, ()) = tokio::join!(post, hold_lock);
    let _ = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_admission_busy",
        "the journal write must have failed: {response}"
    );
    assert_eq!(
        result["post_location"]["state"], "target_only",
        "{response}"
    );
}

/// The simulated POST answer every test above posts against must be one Tally
/// actually sends, and must parse as exactly one clean create. When it did not
/// parse, every test here ran the post with no parsed outcome, so none of them
/// exercised the clean-success path.
#[test]
fn the_simulated_post_answer_parses_as_one_clean_create() {
    let outcome = parse_import_outcome(&created_one()).expect("the POST answer parses");
    assert_eq!(outcome.counters().created, 1);
    assert!(import_outcome_is_clean(Some(&outcome), 1));
}

/// A Journal Bridge posted live (bridge#582's lab qualification), as its
/// export was captured: `native-namespaced-journal`. The batch below is the
/// one that produced it, so a readback serving this capture is the post's own
/// voucher, attributed by its `[BRIDGE:…]` marker.
fn captured_posted_journal() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml"
    ))
}

fn saved_captured_batch(server: &Server) -> Value {
    let line = saved_captured_line(server);
    json!({"company_guid":GUID,"batch_id":line.batch_id})
}

fn saved_captured_line(server: &Server) -> ImportLedgerLine {
    let origin = super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let mut line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-6c79872c-aab6-4be5-a181-18182c8148be", "identity_scheme":"batch_v1",
        "company_guid":GUID,
        "endpoint_origin":origin,
        "company":{"name":"WR2 Unicode Lab","guid":GUID,"company_number":"100004","books_from":"20260401"},
        "txn_ids":["BRIDGE_MCP_LIVE_20260906_A1"],"date_from":"20260907","date_to":"20260907",
        "sha256":"", "built_at":"2026-09-06T21:40:26.641Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":7},
        "vouchers":[{"bridge_txn_id":"BRIDGE_MCP_LIVE_20260906_A1","date":"20260907",
            "voucher_type":"Journal","narration":"Bridge MCP batch namespace qualification",
            "reference":null,"voucher_number":null,
            "entries":[{"ledger":"Bridge Nested Debtor WR4","amount":"12.61","side":"Dr"},
                {"ledger":"Cash","amount":"12.61","side":"Cr"}]}]
    }))
    .unwrap();
    let rendered = render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(rendered.as_bytes());
    bind_to_captured_catalogue(&mut line);
    server.append_import_ledger(&line).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        rendered,
    )
    .unwrap();
    line
}

/// The whole native post, end to end: absent before, one clean create, the
/// location snapshot, then the readback binds the untagged voucher the post
/// created and finds it, `posted_verified`.
#[tokio::test]
async fn a_native_post_reads_back_as_posted_verified() {
    native_post_read_back(11, 1, true).await;
}

/// The same post with the target's mark stepping by two for its one create:
/// the location reports the step, which refuses the post's binding for good,
/// so the post is never verified.
#[tokio::test]
async fn a_mark_step_other_than_the_create_is_reported_and_never_verified() {
    native_post_read_back(12, 2, false).await;
}

async fn native_post_read_back(mark_after: u64, step: u64, matches_created: bool) {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(company_marks(mark_after, 50, "WR2 Unicode Lab")));
    // The readback: the same verification read the pre-post check made, now
    // serving the voucher the post created, untagged.
    plans.extend(span_readback(untagged_posted_journal(), mark_after));
    plans.extend(probe());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let _ = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(response["isError"], json!(!matches_created), "{response}");
    if matches_created {
        assert_eq!(result["dispatch"]["state"], "posted_verified", "{response}");
        assert_eq!(result["counts"]["posted_verified"], 1, "{response}");
        assert_eq!(result["post_span_binding"]["state"], "bound", "{response}");
        assert!(result.get("error").is_none(), "{response}");
    } else {
        assert_eq!(
            result["error"]["code"], "import_reconciliation_required",
            "{response}"
        );
        assert_eq!(result["counts"]["posted_verified"], 0, "{response}");
        assert_eq!(
            result["post_span_binding"]["code"], "span_step_not_created",
            "{response}"
        );
    }
    assert_eq!(
        result["post_location"]["state"], "target_only",
        "{response}"
    );
    assert_eq!(
        result["post_location"]["target_voucher_step"]["step"], step,
        "{response}"
    );
    assert_eq!(
        result["post_location"]["target_voucher_step"]["matches_created"], matches_created,
        "{response}"
    );
    assert_journaled_clean_create(directory.path());
}

/// #985: the readback after a sent post is admitted against the census that
/// sized it, so a book that changed between the count and the read refuses the
/// readback. The post was sent and stays sent: the refusal names its cause and
/// sends the caller to verify_import with this batch, never to post again, and
/// carries none of the read-only "call the same tool again" remediation.
#[tokio::test]
async fn a_readback_refused_by_its_census_after_a_post_sends_the_caller_to_verify_import() {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(company_marks(11, 50, "WR2 Unicode Lab")));
    // The readback on a book whose voucher mark needs a census: the census
    // counts the posted voucher, and the read that follows returns none.
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(
        marks().replace("<ALTVCHID>10</ALTVCHID>", "<ALTVCHID>1000</ALTVCHID>"),
    ));
    plans.extend(paired(captured_posted_journal()));
    plans.extend(paired(empty_collection()));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let _ = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "voucher_window_part_not_admitted",
        "{response}"
    );
    assert_eq!(
        result["error"]["cause"], "part_census_mismatch",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(true), "{response}");
    assert_eq!(
        result["error"]["message"],
        "The saved batch requires reconciliation. Use verify_import with this original batch; never rebuild it to retry."
    );
    assert!(result["error"].get("remediation").is_none(), "{response}");
    assert_ne!(result["dispatch"]["state"], "posted_verified", "{response}");
    assert_journaled_clean_create(directory.path());
}

// bridge#551: a post goes only into a book with exactly one Currency master,
// checked before approval and again inside the queue.

/// The refusal both surfaces report for the captured two-master book: the
/// plain reason, naming both masters, with no attempt recorded.
fn assert_refused_as_multi_currency(result: &Value) {
    assert_eq!(
        result["error"]["code"], "import_multi_currency_unsupported",
        "{result}"
    );
    assert_eq!(result["attempt_recorded"], json!(false), "{result}");
    let message = result["error"]["message"].as_str().unwrap();
    assert!(
        message.starts_with("This company has more than one currency defined ("),
        "{message}"
    );
    assert!(
        message.contains("Bridge does not post into multi-currency books yet"),
        "{message}"
    );
    let names =
        bridge_tally_protocol::native_outstandings::parse_company_currency(&two_currencies())
            .unwrap()
            .names;
    assert_eq!(names.len(), 2);
    for name in &names {
        assert!(message.contains(name.as_str()), "{name} in {message}");
    }
}

#[tokio::test]
async fn a_book_with_two_currency_masters_is_refused_before_approval_on_both_surfaces() {
    for desktop in [false, true] {
        let mut plans = before_approval_with_currencies(two_currencies());
        // The qualified mode probe after the Currency read is never sent.
        plans.truncate(plans.len() - probe().len());
        let expected = plans.len();
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (line, args) = saved_batch(&server);
        let scripted = ScriptedApproval::approving();
        let result = if desktop {
            let outcome = SCRIPTED_APPROVAL
                .scope(
                    scripted.clone(),
                    server.post_import_checked(&args, Some(&line.sha256), PostScope::JournalOnly),
                )
                .await
                .expect("a refusal is reported as the post's outcome");
            // What the desktop's webview receives.
            super::super::desktop_journal::DesktopJournalOperation::from_outcome(outcome).result
                ["result"]
                .clone()
        } else {
            let response = SCRIPTED_APPROVAL
                .scope(scripted.clone(), server.call_tool("post_import", args))
                .await;
            let result = response["structuredContent"]["result"].clone();
            assert_eq!(
                result["error"]["currencies_seen"].as_array().map(Vec::len),
                Some(2),
                "{response}"
            );
            result
        };
        let observed = sent(simulator);
        assert_refused_as_multi_currency(&result);
        assert!(scripted.previews().is_empty(), "approval must not be asked");
        assert_eq!(observed.len(), expected, "{result}");
        assert!(!String::from_utf8(journal(directory.path()))
            .unwrap()
            .contains("\"dispatch_intent\""));
    }
}

/// A master added while approval waits: the queue's own Currency read refuses,
/// after the aim snapshot and before the intent, so the POST is never sent.
#[tokio::test]
async fn a_currency_master_added_after_approval_is_refused_in_the_queue() {
    let mut plans = before_approval();
    let mut after = after_approval_with_currencies(two_currencies(), xml(created_one()));
    after.pop();
    let expected = plans.len() + after.len();
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_refused_as_multi_currency(&response["structuredContent"]["result"]);
    assert_eq!(scripted.previews().len(), 1, "approval was asked once");
    assert_eq!(
        observed.len(),
        expected,
        "the POST is never sent: {response}"
    );
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
}

/// A Payment is held to the same gate as a Journal: refused before approval on
/// a two-master book, and in the queue when the master arrives after approval.
#[tokio::test]
async fn a_payment_into_a_book_with_two_currency_masters_is_refused_before_and_after_approval() {
    for in_queue in [false, true] {
        let plans = if in_queue {
            let mut plans = bank_before_approval(catalogue(), groups());
            let mut after = bank_after_approval_with_currencies(
                catalogue(),
                groups(),
                two_currencies(),
                xml(created_one()),
            );
            after.pop();
            plans.extend(after);
            plans
        } else {
            let mut plans =
                bank_before_approval_with_currencies(catalogue(), groups(), two_currencies());
            plans.truncate(plans.len() - probe().len());
            plans
        };
        let expected = plans.len();
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (_, args) = saved_bank_batch(&server, payment());
        let scripted = ScriptedApproval::approving();
        let response = SCRIPTED_APPROVAL
            .scope(scripted.clone(), server.call_tool("post_import", args))
            .await;
        let observed = sent(simulator);
        assert_refused_as_multi_currency(&response["structuredContent"]["result"]);
        assert_eq!(
            scripted.previews().len(),
            usize::from(in_queue),
            "{response}"
        );
        assert_eq!(observed.len(), expected, "{response}");
        assert!(!String::from_utf8(journal(directory.path()))
            .unwrap()
            .contains("\"dispatch_intent\""));
    }
}

/// Currency masters the queue cannot name a base from (none, here: an explicit
/// edit of the one-master capture) refuse with their own code, and nothing is
/// posted.
#[tokio::test]
async fn currency_masters_without_a_base_are_refused_in_the_queue_with_their_own_code() {
    let single = single_currency();
    // The row's own close: CMPINFO's `<CURRENCY>0</CURRENCY>` comes earlier.
    let row_start = single.find("<CURRENCY NAME=").unwrap();
    let row_end =
        row_start + single[row_start..].find("</CURRENCY>").unwrap() + "</CURRENCY>".len();
    let no_master = format!("{}{}", &single[..row_start], &single[row_end..]);
    assert_eq!(
        bridge_tally_protocol::native_outstandings::parse_company_currency(&no_master)
            .unwrap()
            .currency_count,
        0,
        "the edit must leave a readable collection with no master"
    );
    let mut plans = before_approval();
    let mut after = after_approval_with_currencies(no_master, xml(created_one()));
    after.pop();
    let expected = plans.len() + after.len();
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_base_currency_undetermined",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(false), "{response}");
    assert!(
        result["error"].get("currencies_seen").is_none(),
        "{response}"
    );
    assert_eq!(
        observed.len(),
        expected,
        "the POST is never sent: {response}"
    );
}

// bridge#239: a master changed after the queue's catalogue re-read is refused
// before the POST, from the target's ALTMSTID in the aim snapshot compared
// with the one read as the binding reads began.

/// The approved Journal, with the queue's binding-time snapshot answered by
/// `at_binding` and its aim snapshot by `at_aim`. Returns the response, the
/// requests observed, and the number up to and including the aim snapshot.
async fn post_with_master_marks(at_binding: String, at_aim: String) -> (Value, usize, usize) {
    let mut plans = before_approval();
    let mut after = after_approval(xml(created_one()));
    // The binding snapshot follows the opening mode probe and company read.
    let binding_at = probe().len() + 1;
    after[binding_at] = xml(at_binding);
    let aim_at = after.len() - 2;
    after[aim_at] = xml(at_aim);
    let through_aim = plans.len() + aim_at + 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator).len();
    (response, observed, through_aim)
}

fn marks_with_target_masters(masters: u64) -> String {
    replaced_once(
        &company_marks(10, 50, "WR2 Unicode Lab"),
        "<ALTMSTID>7</ALTMSTID>",
        &format!("<ALTMSTID>{masters}</ALTMSTID>"),
    )
}

#[tokio::test]
async fn a_master_changed_after_the_catalogue_re_read_is_refused_before_the_post() {
    let (response, observed, through_aim) =
        post_with_master_marks(marks_with_target_masters(7), marks_with_target_masters(8)).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["error"]["code"], "post_masters_moved", "{response}");
    assert_eq!(result["attempt_recorded"], json!(false), "{response}");
    assert_eq!(observed, through_aim, "the POST is never sent: {response}");
}

/// The control: another company's masters moving is not the target's, so the
/// post goes ahead.
#[tokio::test]
async fn another_companys_master_change_does_not_refuse_the_post() {
    let at_aim = replaced_once(
        &company_marks(10, 50, "WR2 Unicode Lab"),
        "<ALTMSTID>3</ALTMSTID>",
        "<ALTMSTID>4</ALTMSTID>",
    );
    let (response, observed, through_aim) =
        post_with_master_marks(marks_with_target_masters(7), at_aim).await;
    assert!(observed > through_aim, "the POST is sent: {response}");
}

/// A binding-time snapshot without the target cannot be compared, so the post
/// is refused as unconfirmed.
#[tokio::test]
async fn a_binding_snapshot_without_the_target_is_refused_as_unconfirmed() {
    let (response, observed, through_aim) = post_with_master_marks(
        company_marks(10, 50, "Synthetic Renamed Lab"),
        marks_with_target_masters(7),
    )
    .await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "post_masters_unconfirmed",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(false), "{response}");
    assert_eq!(observed, through_aim, "{response}");
}

/// During the approval wait, a ledger renamed and a new one created under its
/// old name is caught by identity: the queue's catalogue holds the approved
/// name with a different GUID, and the post is refused before the intent.
#[tokio::test]
async fn a_new_ledger_under_an_approved_name_during_approval_is_refused_by_identity() {
    let replaced = replaced_once(
        &catalogue(),
        ">61c6de69-1748-461c-ad3f-162cb949df9f-0000001f</GUID>",
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000ff</GUID>",
    );
    let mut plans = before_approval();
    let mut after = after_approval(xml(created_one()));
    // The queue's catalogue: its first report and its replay.
    let catalogue_at = probe().len() + 2;
    after[catalogue_at + 1] = xml(replaced.clone());
    after[catalogue_at + 3] = xml(replaced);
    // The binding is compared once every queue read is in, after the aim
    // snapshot; only the POST is never sent.
    after.pop();
    let expected = plans.len() + after.len();
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_masters_changed",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(false), "{response}");
    assert_eq!(observed.len(), expected, "{response}");
}

/// #1234: the queue re-reads the catalogue after approval, inside
/// its identity brackets, and its rows carry the flag. A ledger switched to
/// bill-wise while the approval waits, with no approval recorded for it, is
/// refused before the intent and the POST under its own code.
#[tokio::test]
async fn a_named_ledger_switched_to_bill_wise_during_approval_is_refused_in_the_queue() {
    let switched =
        crate::agent::agent_import::tests::with_bill_wise_flags(&catalogue(), &["WR2 Sales"]);
    let mut plans = before_approval();
    let mut after = after_approval(xml(created_one()));
    // The queue's catalogue: its first report and its replay.
    let catalogue_at = probe().len() + 2;
    after[catalogue_at + 1] = xml(switched.clone());
    after[catalogue_at + 3] = xml(switched);
    // The recheck runs once every queue read is in; only the POST is never sent.
    after.pop();
    let expected = plans.len() + after.len();
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_bill_wise_changed",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(false), "{response}");
    assert_eq!(observed.len(), expected, "{response}");
}

/// #1234: the approval the build recorded is what lets a bill-wise ledger
/// through. The same ledger, bill-wise at the post and again in the queue, is
/// posted when the saved batch records its approval, and the POST is the
/// request the intent recorded; with no approval recorded it is the refusal
/// above. A post that read the approved list as empty would fail here.
#[tokio::test]
async fn a_ledger_that_is_bill_wise_and_was_approved_at_the_build_still_posts() {
    let bill_wise =
        crate::agent::agent_import::tests::with_bill_wise_flags(&catalogue(), &["WR2 Sales"]);
    let mut plans = before_approval();
    for plan in &mut plans {
        let body = plan.fixture.body().into_owned();
        if body.contains("<LEDGER NAME=\"") && body.contains("<ISBILLWISEON") {
            plan.fixture = Fixture::SyntheticXml(
                crate::agent::agent_import::tests::with_bill_wise_flags(&body, &["WR2 Sales"]),
            );
        }
    }
    let mut after = after_approval(xml(created_one()));
    let catalogue_at = probe().len() + 2;
    after[catalogue_at + 1] = xml(bill_wise.clone());
    after[catalogue_at + 3] = xml(bill_wise);
    let post_at = plans.len() + after.len() - 1;
    plans.extend(after);
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (mut line, args) = saved_batch(&server);
    line.on_account_approved = Some(vec![super::super::bill_wise::OnAccountApproved {
        ledger: "WR2 Sales".into(),
        party_digest: "0".repeat(64),
    }]);
    server.append_import_ledger(&line).unwrap();
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    assert_eq!(observed.len(), post_at + 1, "{response}");
    let intent = dispatch_intent(directory.path());
    assert_eq!(
        observed[post_at].request_body_sha256,
        intent["native_request_sha256"].as_str().unwrap()
    );
}

/// bridge#634, #641: the queue's catalogue re-read at post time holds a
/// repeated ledger. The admission recheck refuses before the intent and the
/// POST under its own code, not the catch-all that says the outcome is
/// unknown, nor #656's `post_queue_read_failed` (the named refusal wins), and
/// carries the catalogue's typed cause. Below the response budget the cause
/// is left out, as on the generic refusal, and the fields a caller acts on
/// survive. The name is never in the response.
#[tokio::test]
async fn a_post_time_catalogue_refusal_names_its_cause_and_no_ledger() {
    let repeated = crate::tally::standard_ledger_catalog::tests::catalogue_with_extra_ledgers(
        &catalogue(),
        [
            ("Twice Named".to_string(), "c0000001".to_string()),
            ("Twice Named".to_string(), "c0000002".to_string()),
        ],
    );
    for (max_bytes, cause) in [
        (200_000, json!("ledger_catalogue_duplicate_identity")),
        (
            crate::agent::REMEDIATION_MIN_RESPONSE_BUDGET - 1,
            Value::Null,
        ),
    ] {
        let mut plans = before_approval();
        let mut after = after_approval(xml(created_one()));
        let catalogue_at = probe().len() + 2;
        after[catalogue_at + 1] = xml(repeated.clone());
        after[catalogue_at + 3] = xml(repeated.clone());
        after.pop();
        let expected = plans.len() + after.len();
        plans.extend(after);
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let mut server = server_at(simulator.address(), directory.path());
        server.settings.max_bytes = max_bytes;
        let (_, args) = saved_batch(&server);
        let response = SCRIPTED_APPROVAL
            .scope(
                ScriptedApproval::approving(),
                server.call_tool("post_import", args),
            )
            .await;
        let observed = sent(simulator);
        let result = &response["structuredContent"]["result"];
        assert_eq!(result["error"]["cause"], cause, "{response}");
        assert_eq!(
            result["error"]["code"], "post_catalogue_unreadable",
            "{response}"
        );
        assert_eq!(result["attempt_recorded"], json!(false), "{response}");
        assert_eq!(
            observed.len(),
            expected,
            "the POST is never sent: {response}"
        );
        assert!(!response.to_string().contains("Twice"), "{response}");
    }
}

/// The binding-time snapshot lost in transport, or answered with T2's live
/// refusal: the masters cannot be compared, so the post is refused as
/// `post_masters_unconfirmed`, never as an unknown outcome. A lost read stops
/// the queue at once; an unreadable one is refused once the queue's reads are
/// in, after the aim snapshot. Neither sends the POST or records an intent.
#[tokio::test]
async fn an_unreadable_binding_snapshot_refuses_as_unconfirmed() {
    let refused = "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>0</STATUS></HEADER><BODY><DATA>\
                   <LINEERROR>Could not set 'SVCurrentCompany' to 'WR2 Unicode Lab'</LINEERROR>\
                   </DATA></BODY></ENVELOPE>"
        .to_string();
    let binding_at = probe().len() + 1;
    for lost in [true, false] {
        let mut plans = before_approval();
        let mut after = after_approval(xml(created_one()));
        after[binding_at] = if lost {
            marks_at_binding().with_delivery(Delivery::ResetBeforeBody)
        } else {
            xml(refused.clone())
        };
        let expected = plans.len()
            + if lost {
                binding_at + 1
            } else {
                after.len() - 1
            };
        plans.extend(after);
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (_, args) = saved_batch(&server);
        let before = journal(directory.path());
        let response = SCRIPTED_APPROVAL
            .scope(
                ScriptedApproval::approving(),
                server.call_tool("post_import", args),
            )
            .await;
        let observed = sent(simulator).len();
        let result = &response["structuredContent"]["result"];
        assert_eq!(
            result["error"]["code"], "post_masters_unconfirmed",
            "{response}"
        );
        assert_eq!(result["attempt_recorded"], json!(false), "{response}");
        assert_eq!(observed, expected, "{response}");
        assert_eq!(
            appended_kinds(&before, &journal(directory.path())),
            ["verification_status"]
        );
    }
}

/// #656: a queue read that fails before the intent is refused under its own
/// code, not the catch-all that says the outcome is unknown. The queue's
/// catalogue legs are lost in transport (the queue stops at once), or disagree
/// (a pair drift); either way no intent is journaled, no POST is sent, and the
/// cause names the failure.
#[tokio::test]
async fn a_queue_read_failing_before_the_intent_is_refused_as_such() {
    let catalogue_at = probe().len() + 2;
    let drifted = replaced_once(
        &catalogue(),
        ">61c6de69-1748-461c-ad3f-162cb949df9f-0000001f</GUID>",
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000ff</GUID>",
    );
    for (lost, cause) in [
        (true, "response_truncated"),
        (false, "native_report_pair_changed"),
    ] {
        let mut plans = before_approval();
        let mut after = after_approval(xml(created_one()));
        let expected = if lost {
            after[catalogue_at + 1] = xml(catalogue()).with_delivery(Delivery::ResetBeforeBody);
            plans.len() + catalogue_at + 2
        } else {
            after[catalogue_at + 3] = xml(drifted.clone());
            // Each leg of the paired read is followed by a health check, and
            // the legs are compared only after the second one.
            plans.len() + catalogue_at + 5
        };
        plans.extend(after);
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (_, args) = saved_batch(&server);
        let before = journal(directory.path());
        let response = SCRIPTED_APPROVAL
            .scope(
                ScriptedApproval::approving(),
                server.call_tool("post_import", args),
            )
            .await;
        let observed = sent(simulator).len();
        let error = &response["structuredContent"]["result"]["error"];
        assert_eq!(error["code"], "post_queue_read_failed", "{response}");
        assert_eq!(error["cause"], cause, "{response}");
        assert_eq!(
            response["structuredContent"]["result"]["attempt_recorded"],
            json!(false),
            "{response}"
        );
        assert_eq!(observed, expected, "{response}");
        assert_eq!(
            appended_kinds(&before, &journal(directory.path())),
            ["verification_status"]
        );
    }
}

/// #656, the other direction: the pre-intent code must never reach a post
/// whose bytes were sent. The POST's response is lost in transport, after the
/// intent was journaled, so the outcome is unknown and the attempt recorded.
#[tokio::test]
async fn a_post_lost_after_the_intent_is_still_an_unknown_outcome() {
    let mut plans = before_approval();
    plans.extend(after_approval(
        xml(created_one()).with_delivery(Delivery::ResetBeforeBody),
    ));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "import_dispatch_outcome_unknown",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], json!(true), "{response}");
}

// bridge#239: the ledgers a batch names must still carry the GUIDs its build
// bound them to; a name alone cannot tell a ledger renamed and replaced.

/// A saved Journal refused before approval by its build-time binding, on the
/// MCP tool or on the desktop's post. A changed GUID is found on the post's
/// catalogue read, the last one sent; a record without identities is refused
/// before any request. No intent is recorded either way.
async fn refused_by_build_binding(
    identities: Option<Vec<BoundLedger>>,
    desktop: bool,
) -> (Value, usize, usize, bool) {
    refused_by_build_binding_under(identities, desktop, crate::agent::Redaction::None).await
}

async fn refused_by_build_binding_under(
    identities: Option<Vec<BoundLedger>>,
    desktop: bool,
    redaction: crate::agent::Redaction,
) -> (Value, usize, usize, bool) {
    let mut plans = before_approval();
    let expected = if identities.is_some() {
        // The Currency read and mode probe after the catalogue are never sent.
        plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
        plans.len()
    } else {
        plans.clear();
        0
    };
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_redacting(simulator.address(), directory.path(), redaction);
    let (mut line, args) = saved_batch(&server);
    line.ledger_identities = identities;
    server.append_import_ledger(&line).unwrap();
    let scripted = ScriptedApproval::approving();
    let result = if desktop {
        let outcome = SCRIPTED_APPROVAL
            .scope(
                scripted.clone(),
                server.post_import_checked(&args, Some(&line.sha256), PostScope::JournalOnly),
            )
            .await
            .expect("a refusal is reported as the post's outcome");
        // What the desktop's webview receives.
        super::super::desktop_journal::DesktopJournalOperation::from_outcome(outcome).result
            ["result"]
            .clone()
    } else {
        SCRIPTED_APPROVAL
            .scope(scripted.clone(), server.call_tool("post_import", args))
            .await["structuredContent"]["result"]
            .clone()
    };
    let observed = sent(simulator).len();
    let intent = String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\"");
    assert!(scripted.previews().is_empty(), "approval must not be asked");
    (result, observed, expected, intent)
}

#[tokio::test]
async fn a_ledger_replaced_under_its_name_since_the_build_is_refused_before_approval() {
    for desktop in [false, true] {
        let identities = vec![
            BoundLedger {
                name: "Cash".into(),
                guid: "61c6de69-1748-461c-ad3f-162cb949df9f-000000ff".into(),
            },
            BoundLedger {
                name: "WR2 Sales".into(),
                guid: "61c6de69-1748-461c-ad3f-162cb949df9f-000000d0".into(),
            },
        ];
        let (result, observed, expected, intent) =
            refused_by_build_binding(Some(identities), desktop).await;
        assert_eq!(
            result["error"]["code"], "import_masters_changed_since_build",
            "{result}"
        );
        assert_eq!(result["attempt_recorded"], json!(false), "{result}");
        if !desktop {
            assert_eq!(
                result["error"]["ledgers_changed"],
                json!(["Cash"]),
                "{result}"
            );
        }
        // The desktop names the ledger; the MCP message leaves it to the list.
        assert_eq!(
            result["error"]["message"]
                .as_str()
                .unwrap()
                .contains("Cash"),
            desktop,
            "{result}"
        );
        assert_eq!(observed, expected, "{result}");
        assert!(!intent);
    }
}

/// The changed ledgers go out in the order the batch names them and never by
/// name, with or without masking. The saved Journal names `WR2 Sales` before
/// `Cash`.
#[tokio::test]
async fn the_changed_ledgers_of_a_refused_post_are_listed_in_the_order_the_batch_names_them() {
    for redaction in [
        crate::agent::Redaction::None,
        crate::agent::Redaction::MaskParties,
    ] {
        let replaced = |name: &str| BoundLedger {
            name: name.into(),
            guid: "61c6de69-1748-461c-ad3f-162cb949df9f-000000ff".into(),
        };
        let (result, ..) = refused_by_build_binding_under(
            Some(vec![replaced("Cash"), replaced("WR2 Sales")]),
            false,
            redaction,
        )
        .await;
        let sent = |name: &str| match redaction {
            crate::agent::Redaction::MaskParties => crate::agent::mask(name),
            _ => name.to_string(),
        };
        assert_eq!(
            result["error"]["code"], "import_masters_changed_since_build",
            "{result}"
        );
        assert_eq!(
            result["error"]["ledgers_changed"],
            json!([sent("WR2 Sales"), sent("Cash")]),
            "{result}"
        );
        assert_eq!(result["error"]["ledgers_changed_total"], 2, "{result}");
    }
}

/// #1234: when a ledger was replaced under its old name AND a named ledger is
/// bill-wise now, the post reports the replacement (which names the ledger and
/// says to confirm the intended one), not the flag, so a rebuild is not
/// advised before the person knows the name now means another ledger.
#[tokio::test]
async fn a_replaced_ledger_is_reported_before_a_flag_that_changed() {
    let mut plans = before_approval();
    for plan in &mut plans {
        let body = plan.fixture.body().into_owned();
        if body.contains("<LEDGER NAME=\"") && body.contains("<ISBILLWISEON") {
            plan.fixture = Fixture::SyntheticXml(
                crate::agent::agent_import::tests::with_bill_wise_flags(&body, &["WR2 Sales"]),
            );
        }
    }
    plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (mut line, args) = saved_batch(&server);
    line.ledger_identities = Some(vec![
        BoundLedger {
            name: "Cash".into(),
            guid: "61c6de69-1748-461c-ad3f-162cb949df9f-000000ff".into(),
        },
        BoundLedger {
            name: "WR2 Sales".into(),
            guid: "61c6de69-1748-461c-ad3f-162cb949df9f-000000d0".into(),
        },
    ]);
    server.append_import_ledger(&line).unwrap();
    let result = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await["structuredContent"]["result"]
        .clone();
    assert_eq!(
        result["error"]["code"], "import_masters_changed_since_build",
        "{result}"
    );
    assert_eq!(sent(simulator).len(), expected, "{result}");
}

/// #1234: the post's own catalogue read, made before approval,
/// carries each ledger's flag. A saved batch whose named ledger reads
/// bill-wise now, with no approval recorded for it, is refused before any
/// approval is asked and before a dispatch intent, on both surfaces.
#[tokio::test]
async fn a_named_ledger_that_became_bill_wise_since_the_build_is_refused_before_approval() {
    for desktop in [false, true] {
        let mut plans = before_approval();
        for plan in &mut plans {
            let body = plan.fixture.body().into_owned();
            if body.contains("<LEDGER NAME=\"") && body.contains("<ISBILLWISEON") {
                plan.fixture = Fixture::SyntheticXml(
                    crate::agent::agent_import::tests::with_bill_wise_flags(&body, &["WR2 Sales"]),
                );
            }
        }
        // The Currency read and mode probe after the catalogue are never sent.
        plans.truncate(plans.len() - paired(single_currency()).len() - probe().len());
        let expected = plans.len();
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (line, args) = saved_batch(&server);
        let scripted = ScriptedApproval::approving();
        let result = if desktop {
            let outcome = SCRIPTED_APPROVAL
                .scope(
                    scripted.clone(),
                    server.post_import_checked(&args, Some(&line.sha256), PostScope::JournalOnly),
                )
                .await
                .expect("a refusal is reported as the post's outcome");
            super::super::desktop_journal::DesktopJournalOperation::from_outcome(outcome).result
                ["result"]
                .clone()
        } else {
            SCRIPTED_APPROVAL
                .scope(scripted.clone(), server.call_tool("post_import", args))
                .await["structuredContent"]["result"]
                .clone()
        };
        let observed = sent(simulator).len();
        let intent = String::from_utf8(journal(directory.path()))
            .unwrap()
            .contains("\"dispatch_intent\"");
        assert_eq!(
            result["error"]["code"], "import_bill_wise_changed",
            "{result}"
        );
        assert_eq!(result["attempt_recorded"], json!(false), "{result}");
        assert!(result["error"]["message"]
            .as_str()
            .unwrap()
            .contains("Build the batch again"));
        assert_eq!(observed, expected, "{result}");
        assert!(!intent);
        assert!(scripted.previews().is_empty(), "approval must not be asked");
    }
}

#[tokio::test]
async fn a_batch_built_before_ledger_binding_is_refused_before_any_request() {
    for desktop in [false, true] {
        let (result, observed, expected, intent) = refused_by_build_binding(None, desktop).await;
        assert_eq!(
            result["error"]["code"], "import_batch_predates_ledger_binding",
            "{result}"
        );
        assert_eq!(result["attempt_recorded"], json!(false), "{result}");
        assert!(result["error"]["message"]
            .as_str()
            .unwrap()
            .contains("Build the batch again"));
        assert_eq!(observed, expected, "{result}");
        assert!(!intent);
    }
}

/// The approval preview says the ledgers were checked by identity.
#[tokio::test]
async fn the_preview_says_the_ledgers_were_checked_by_identity() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let scripted = ScriptedApproval::declining();
    let _ = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let previews = scripted.previews();
    assert_eq!(previews.len(), 1);
    assert!(previews[0].contains("Ledgers checked by identity against the build"));
}

/// A batch dispatched before Bridge recorded ledger identities still
/// reconciles: `post_import` on it goes to the readback, never to the
/// "build it again" refusal, since a dispatched batch must not be rebuilt.
#[tokio::test]
async fn a_dispatched_batch_without_identities_still_reconciles() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (mut line, args) = saved_batch(&server);
    line.ledger_identities = None;
    server.append_import_ledger(&line).unwrap();
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line, &native, None,
            ))
            .unwrap();
    }
    let response = server.call_tool("post_import", args).await;
    let observed = sent(simulator).len();
    let result = &response["structuredContent"]["result"];
    assert_ne!(
        result["error"]["code"], "import_batch_predates_ledger_binding",
        "{response}"
    );
    assert_ne!(result["attempt_recorded"], json!(false), "{response}");
    assert!(observed > 0, "the readback reads Tally: {response}");
}

/// A batch already sent with a narration the agent readers would rewrite still
/// reconciles: the refusal below applies only before a POST, never to a batch
/// that must be reconciled rather than sent again.
#[tokio::test]
async fn a_dispatched_batch_with_a_rewritten_narration_still_reconciles() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch_with_narration(&server, "Synthetic \u{fffd}#4; test only");
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line, &native, None,
            ))
            .unwrap();
    }
    let response = server.call_tool("post_import", args).await;
    let observed = sent(simulator).len();
    let result = &response["structuredContent"]["result"];
    assert_ne!(
        result["error"]["code"], "voucher_text_invalid",
        "{response}"
    );
    assert_ne!(result["attempt_recorded"], json!(false), "{response}");
    assert!(observed > 0, "the readback reads Tally: {response}");
}

/// A batch saved before the build refused a narration the agent readers would
/// rewrite: its untagged post could never be bound, so `post_import` refuses
/// it before approval and before any request, and records no intent.
#[tokio::test]
async fn a_saved_narration_that_would_read_back_rewritten_is_never_sent() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch_with_narration(&server, "Synthetic \u{fffd}#4; test only");
    let scripted = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator).len();
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "voucher_text_invalid",
        "{response}"
    );
    assert_eq!(observed, 0, "{response}");
    assert!(scripted.previews().is_empty(), "approval must not be asked");
    assert!(!String::from_utf8(journal(directory.path()))
        .unwrap()
        .contains("\"dispatch_intent\""));
}

// bridge#239: the company's masters across the post. Only when the target's
// master mark moved between the aim snapshot and the snapshot after the POST
// is the catalogue read again, and the approved ledgers resolved by name.

/// The captured post read back, with `marks_after` answering the snapshot
/// after the POST and `catalogue` the extra read (if any) before the readback.
/// Returns the response, the requests observed and the requests scripted.
async fn post_with_masters_after(
    marks_after: String,
    catalogue: Vec<ScenarioPlan>,
) -> (Value, usize, usize) {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(marks_after));
    plans.extend(catalogue);
    plans.extend(posted_readback());
    let scripted = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    let observed = sent(simulator).len();
    // Every post leaves a record: the verdict it reports, or, when the check
    // could not run, the pending mark a later readback finishes.
    let recorded = masters_check_of(&server, args["batch_id"].as_str().unwrap());
    let reported = &response["structuredContent"]["result"]["masters_after_post"];
    if reported["state"] == "check_unavailable" {
        assert_eq!(recorded, json!({"state": "check_pending"}), "{response}");
    } else {
        assert_eq!(&recorded, reported, "{response}");
    }
    // A verified post is an amendment's baseline; a doubted one never is.
    let baseline = read_verified_baseline(
        &server.imports_dir().unwrap(),
        args["batch_id"].as_str().unwrap(),
    )
    .is_some();
    let result = &response["structuredContent"]["result"];
    if result["dispatch"]["state"] == "posted_verified" {
        assert!(baseline, "{response}");
    }
    if masters_doubt(Some(reported)).is_some() {
        assert!(!baseline, "{response}");
    }
    (response, observed, scripted)
}

/// What the batch's masters records say, as every reader reads them.
fn masters_check_of(server: &Server, batch_id: &str) -> Value {
    read_masters_check(&server.imports_dir().unwrap(), batch_id).unwrap()
}

/// A write to `path` fails: a directory with an entry is in its place.
fn block(path: std::path::PathBuf) {
    fs::create_dir_all(path.join("entry")).unwrap();
}

/// The readback after a post, and in a later reconcile of the posted batch,
/// for a batch whose dispatch intent records no pre-POST mark (written as an
/// older build wrote it): the window read twice.
fn reconcile_readback() -> Vec<ScenarioPlan> {
    let mut plans = probe();
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(captured_posted_journal()));
    plans.extend(paired(captured_posted_journal()));
    plans
}

/// The same readback for a native post this build made: after the window, the
/// target's marks, which still show the post (a book below them was rolled
/// back).
fn posted_readback() -> Vec<ScenarioPlan> {
    span_readback(untagged_posted_journal(), 11)
}

/// The snapshot after the POST with the target's master mark at `masters`
/// (the aim snapshot has it at 7).
fn masters_moved_to(masters: u64) -> String {
    replaced_once(
        &company_marks(11, 50, "WR2 Unicode Lab"),
        "<ALTMSTID>7</ALTMSTID>",
        &format!("<ALTMSTID>{masters}</ALTMSTID>"),
    )
}

#[tokio::test]
async fn unmoved_masters_are_not_checked_and_cost_no_request() {
    let (response, observed, scripted) =
        post_with_masters_after(company_marks(11, 50, "WR2 Unicode Lab"), Vec::new()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["masters_after_post"]["state"], "not_checked",
        "{response}"
    );
    assert_eq!(result["masters_after_post"]["reason"], "masters_unmoved");
    assert_eq!(result["dispatch"]["state"], "posted_verified", "{response}");
    assert_eq!(observed, scripted, "no extra read: {response}");
}

#[tokio::test]
async fn moved_masters_with_every_approved_ledger_unchanged_stay_verified() {
    let (response, observed, scripted) =
        post_with_masters_after(masters_moved_to(8), paired(catalogue())).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["masters_after_post"]["state"], "unchanged",
        "{response}"
    );
    assert_eq!(result["masters_after_post"]["trigger"], "masters_moved");
    assert_eq!(result["dispatch"]["state"], "posted_verified", "{response}");
    assert_eq!(observed, scripted, "{response}");
}

/// A snapshot after the POST that cannot be read proves nothing unmoved, so
/// the approved ledgers are still read again, never skipped.
#[tokio::test]
async fn an_unreadable_snapshot_after_the_post_still_checks_the_ledgers() {
    let unreadable = "<ENVELOPE><BODY><DATA></DATA></BODY></ENVELOPE>".to_string();
    let (response, observed, scripted) =
        post_with_masters_after(unreadable, paired(catalogue())).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["masters_after_post"],
        json!({"state":"unchanged","trigger":"masters_unconfirmed"}),
        "{response}"
    );
    assert_eq!(result["dispatch"]["state"], "posted_verified", "{response}");
    assert_eq!(observed, scripted, "{response}");
}

#[tokio::test]
async fn a_ledger_now_on_another_guid_after_the_post_is_flagged_not_verified() {
    let replaced = replaced_once(
        &catalogue(),
        ">61c6de69-1748-461c-ad3f-162cb949df9f-0000001f</GUID>",
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000ff</GUID>",
    );
    let (response, observed, scripted) =
        post_with_masters_after(masters_moved_to(8), paired(replaced)).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["masters_after_post"]["state"], "posted_under_changed_masters",
        "{response}"
    );
    assert_eq!(result["masters_after_post"]["ledgers"], json!(["Cash"]));
    // Bound: the masters check alone keeps it from posted_verified.
    assert_eq!(result["post_span_binding"]["state"], "bound", "{response}");
    assert_eq!(result["dispatch"]["state"], "reconciliation_required");
    assert_eq!(result["error"]["code"], "posted_under_changed_masters");
    let message = result["error"]["message"].as_str().unwrap();
    assert!(message.starts_with("Posted to Tally"), "{message}");
    assert!(message.contains("do not rebuild this event"), "{message}");
    assert_eq!(observed, scripted, "{response}");
}

#[tokio::test]
async fn moved_masters_that_cannot_be_re_read_are_not_verified() {
    // T2's live answer: the catalogue read is refused, so nothing confirms
    // the approved ledgers.
    let refused = "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>0</STATUS></HEADER><BODY><DATA>\
                   <LINEERROR>Could not set 'SVCurrentCompany' to 'WR2 Unicode Lab'</LINEERROR>\
                   </DATA></BODY></ENVELOPE>"
        .to_string();
    let (response, _, _) = post_with_masters_after(masters_moved_to(8), paired(refused)).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["masters_after_post"]["state"], "check_unavailable",
        "{response}"
    );
    // Bound: the masters check alone keeps it from posted_verified.
    assert_eq!(result["post_span_binding"]["state"], "bound", "{response}");
    assert_eq!(result["dispatch"]["state"], "reconciliation_required");
    assert_eq!(result["error"]["code"], "masters_after_post_unconfirmed");
}

/// A later `post_import` of a dispatched batch only reconciles: it reads the
/// voucher back by name, and the masters check recorded at the post decides
/// whether that is enough. `after` is scripted after the readback.
async fn reconcile_with_masters_check(
    record: Option<&[u8]>,
    after: Vec<ScenarioPlan>,
) -> (Value, usize, usize, Server, tempfile::TempDir) {
    let mut plans = reconcile_readback();
    plans.extend(after);
    reconcile_scripted(record, plans).await
}

async fn reconcile_with_masters_check_and_doubt(
    record: &[u8],
    doubt: &Value,
    after: Vec<ScenarioPlan>,
) -> (Value, usize, usize, Server, tempfile::TempDir) {
    let mut plans = reconcile_readback();
    plans.extend(after);
    reconcile_seeded(Some(record), Some(doubt), plans).await
}

async fn reconcile_scripted(
    record: Option<&[u8]>,
    plans: Vec<ScenarioPlan>,
) -> (Value, usize, usize, Server, tempfile::TempDir) {
    reconcile_seeded(record, None, plans).await
}

async fn reconcile_seeded(
    record: Option<&[u8]>,
    doubt: Option<&Value>,
    plans: Vec<ScenarioPlan>,
) -> (Value, usize, usize, Server, tempfile::TempDir) {
    let scripted = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved_captured_line(&server);
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line, &native, None,
            ))
            .unwrap();
        // A clean response was saved, so only the readback and the check decide.
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::response(
                &line,
                ledger::DispatchResponse {
                    request_sha256: native.request_sha256.clone(),
                    ..super::tests::dispatch_response("success", 1, 0)
                },
            ))
            .unwrap();
    }
    if let Some(record) = record {
        fs::write(
            server
                .imports_dir()
                .unwrap()
                .join(format!("{}.masters_check.json", line.batch_id)),
            record,
        )
        .unwrap();
    }
    if let Some(doubt) = doubt {
        fs::write(
            server
                .imports_dir()
                .unwrap()
                .join(format!("{}.masters_doubt.json", line.batch_id)),
            serde_json::to_vec(doubt).unwrap(),
        )
        .unwrap();
    }
    let args = json!({"company_guid":GUID,"batch_id":line.batch_id});
    let response = server.call_tool("post_import", args).await;
    let observed = sent(simulator).len();
    (response, observed, scripted, server, directory)
}

const BATCH: &str = "bridge-6c79872c-aab6-4be5-a181-18182c8148be";

fn replaced_cash() -> String {
    replaced_once(
        &catalogue(),
        ">61c6de69-1748-461c-ad3f-162cb949df9f-0000001f</GUID>",
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000ff</GUID>",
    )
}

#[tokio::test]
async fn a_doubted_post_stays_doubted_when_reconciled_later() {
    // The control: a batch dispatched before the record existed reconciles.
    let (response, observed, scripted, ..) = reconcile_with_masters_check(None, Vec::new()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["dispatch"]["state"], "previous_attempt_reconciled",
        "{response}"
    );
    assert_eq!(observed, scripted, "{response}");
    let doubt = json!({"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Cash"]});
    let (response, observed, scripted, server, _directory) =
        reconcile_with_masters_check(Some(&serde_json::to_vec(&doubt).unwrap()), Vec::new()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{response}"
    );
    assert_eq!(result["error"]["code"], "posted_under_changed_masters");
    assert_eq!(result["masters_after_post"], doubt);
    assert_eq!(observed, scripted, "no second check: {response}");
    assert_eq!(masters_check_of(&server, BATCH), doubt);
}

/// A recorded verdict's ledgers are answered in the order the batch names them,
/// never in the order they were recorded (which is by name). The captured
/// batch names its two ledgers in name order too, so this record is written in
/// the opposite order to tell the two apart; the record itself is left as it
/// was saved.
#[tokio::test]
async fn a_recorded_verdict_lists_its_ledgers_in_the_order_the_batch_names_them() {
    let recorded = serde_json::to_vec(&json!({"state":"posted_under_changed_masters",
        "trigger":"masters_moved","ledgers":["Cash","Bridge Nested Debtor WR4"]}))
    .unwrap();
    let (response, _, _, server, _directory) =
        reconcile_with_masters_check(Some(&recorded), Vec::new()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["masters_after_post"]["ledgers"],
        json!(["Bridge Nested Debtor WR4", "Cash"]),
        "{response}"
    );
    assert_eq!(
        result["masters_after_post"]["state"], "posted_under_changed_masters",
        "{response}"
    );
    // The record's own bytes, not a reading of them.
    let record = server
        .imports_dir()
        .unwrap()
        .join(format!("{BATCH}.masters_check.json"));
    assert_eq!(fs::read(record).unwrap(), recorded);
}

/// A check the post could not finish (a crash, a lost read) is finished by the
/// next readback that finds the voucher, against the ledgers bound at build.
#[tokio::test]
async fn a_pending_check_is_finished_by_the_next_readback() {
    let pending = br#"{"state":"check_pending"}"#;
    let (response, observed, scripted, server, _directory) =
        reconcile_with_masters_check(Some(pending), paired(catalogue())).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["dispatch"]["state"], "previous_attempt_reconciled",
        "{response}"
    );
    assert_eq!(
        result["masters_after_post"],
        json!({"state":"unchanged","trigger":"check_pending"})
    );
    assert_eq!(observed, scripted, "{response}");
    assert_eq!(masters_check_of(&server, BATCH)["state"], "unchanged");

    let (response, _, _, server, _directory) =
        reconcile_with_masters_check(Some(pending), paired(replaced_cash())).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "posted_under_changed_masters",
        "{response}"
    );
    assert_eq!(result["masters_after_post"]["ledgers"], json!(["Cash"]));
    assert_eq!(
        masters_check_of(&server, BATCH)["state"],
        "posted_under_changed_masters"
    );

    // A readback that does not find the voucher finishes nothing: before
    // the POST lands, a verdict would vouch for a post not yet made. The
    // catalogue is scripted, so a check that ran anyway would be served.
    let mut plans = probe();
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.extend(probe());
    plans.extend(paired(catalogue()));
    let (response, _, _, server, _directory) = reconcile_scripted(Some(pending), plans).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["counts"]["not_found"], 1, "{response}");
    assert_eq!(
        masters_check_of(&server, BATCH),
        json!({"state":"check_pending"})
    );

    // A check that still cannot run leaves the record pending for next time.
    let (response, _, _, server, _directory) =
        reconcile_with_masters_check(Some(pending), Vec::new()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "masters_after_post_unconfirmed",
        "{response}"
    );
    assert_eq!(
        masters_check_of(&server, BATCH),
        json!({"state":"check_pending"})
    );
}

/// A record that exists but cannot be opened or parsed is a pending check,
/// never an absent one.
#[tokio::test]
async fn an_unreadable_masters_record_is_still_a_doubt() {
    let (response, ..) = reconcile_with_masters_check(Some(b"{not json"), Vec::new()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{response}"
    );
    assert_eq!(result["error"]["code"], "masters_after_post_unconfirmed");
}

#[test]
fn an_unopenable_masters_record_reads_as_pending() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let imports = server.imports_dir().unwrap();
    fs::create_dir(imports.join("batch-a.masters_check.json")).unwrap();
    fs::create_dir(imports.join("batch-c.masters_doubt.json")).unwrap();
    for batch in ["batch-a", "batch-c"] {
        assert_eq!(
            read_masters_check(&imports, batch),
            Some(json!({"state":"check_pending"})),
            "{batch}"
        );
    }
    assert_eq!(read_masters_check(&imports, "batch-b"), None);
}

/// A readback that fails after a doubted post still reports and records it.
#[tokio::test]
async fn a_failed_readback_after_a_doubted_post_still_carries_the_doubt() {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(masters_moved_to(8)));
    plans.extend(paired(replaced_cash()));
    // No readback is scripted: the sentinel answers it, so it fails.
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    drop(sent(simulator));
    let result = &response["structuredContent"]["result"];
    assert_ne!(result["dispatch"]["state"], "posted_verified", "{response}");
    assert_eq!(
        result["masters_after_post"]["state"], "posted_under_changed_masters",
        "{response}"
    );
    assert_eq!(
        masters_check_of(&server, BATCH),
        result["masters_after_post"]
    );
}

/// Only a busy wire lock earns the marks readback its retry (#884): a read
/// that failed on the wire is asked once, so the retry never doubles a send
/// Tally already answered badly.
#[tokio::test]
async fn a_marks_readback_that_failed_for_another_reason_is_sent_once() {
    // No readback is scripted: a status answers it, so it fails. A second
    // one is there to be consumed by a retry that must not happen.
    let simulator = SequenceSimulator::spawn(with_sentinel(vec![status()])).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let request = crate::tally::agent_read_request::AgentReadRequest::parse(
        super::super::super::read_profiles::render_agent_company_high_water("Test Co"),
    )
    .unwrap();
    let result = server
        .read_marks_after_post(request, std::time::Instant::now())
        .await;
    assert!(
        result.is_err(),
        "the sentinel must fail the read: {result:?}"
    );
    assert_eq!(sent(simulator).len(), 1);
}

/// A verdict replaces a pending check; an observed doubt outranks any later
/// verdict; a check that could not run is not recorded; a verdict that cannot
/// be written leaves the check, and the result, pending.
#[test]
fn a_masters_verdict_never_clears_an_observed_doubt() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let imports = server.imports_dir().unwrap();
    let pending = json!({"state":"check_pending"});
    let unchanged = json!({"state":"unchanged","trigger":"masters_moved"});
    let doubt = json!({"state":"posted_under_changed_masters","ledgers":["Cash"]});
    let unavailable = json!({"state":"check_unavailable","trigger":"masters_moved"});
    server.record_masters_check_pending("batch-a").unwrap();
    assert_eq!(
        server.record_masters_verdict("batch-a", unavailable.clone()),
        unavailable
    );
    assert_eq!(
        read_masters_check(&imports, "batch-a"),
        Some(pending.clone())
    );
    assert_eq!(
        server.record_masters_verdict("batch-a", unchanged.clone()),
        unchanged
    );
    assert_eq!(
        server.record_masters_verdict("batch-a", doubt.clone()),
        doubt
    );
    assert_eq!(
        server.record_masters_verdict("batch-a", unchanged.clone()),
        doubt
    );
    assert_eq!(read_masters_check(&imports, "batch-a"), Some(doubt.clone()));

    // The check record cannot be written: a clear verdict stays pending, and
    // an observed doubt is still kept by its own file.
    block(imports.join("batch-b.masters_check.json"));
    assert_eq!(server.record_masters_verdict("batch-b", unchanged), pending);
    assert_eq!(
        server.record_masters_verdict("batch-b", doubt.clone()),
        doubt
    );
    assert!(imports.join("batch-b.masters_doubt.json").is_file());
}

/// Reviewers' A1: the post's own check observes a changed ledger, but its
/// check record is left pending; the ledger is later renamed back. The next
/// readback finds the voucher and would find a clean catalogue, yet the
/// observed doubt stands, and no second check is made.
#[tokio::test]
async fn an_observed_doubt_outlives_a_later_clean_check() {
    let doubt = json!({"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Cash"]});
    let pending = br#"{"state":"check_pending"}"#;
    let (response, observed, scripted, server, _directory) =
        reconcile_with_masters_check_and_doubt(pending, &doubt, paired(catalogue())).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{response}"
    );
    assert_eq!(result["error"]["code"], "posted_under_changed_masters");
    assert_eq!(
        observed,
        scripted - paired(catalogue()).len(),
        "no second check: {response}"
    );
    assert_eq!(masters_check_of(&server, BATCH), doubt);
    assert!(read_verified_baseline(&server.imports_dir().unwrap(), BATCH).is_none());
}

/// A post whose pending masters record cannot be written is refused before
/// its dispatch intent, so nothing is sent.
#[tokio::test]
async fn a_post_whose_masters_record_cannot_be_written_is_not_sent() {
    // The queue's plans stay in the sequence, so a post that went ahead would
    // be served and observed here.
    let mut plans = before_approval();
    let expected = plans.len();
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let imports = server.imports_dir().unwrap();
    block(imports.join(format!("{BATCH}.masters_check.json")));
    let before = journal(directory.path());
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "post_masters_record_unavailable",
        "{response}"
    );
    assert_eq!(result["attempt_recorded"], false, "{response}");
    assert_eq!(
        observed.len(),
        expected,
        "nothing past approval: {response}"
    );
    assert_eq!(
        appended_kinds(&before, &journal(directory.path())),
        ["verification_status"]
    );
}

/// A pending mark never erases an observed doubt: a process that lost the
/// dispatch race cannot erase the winner's.
#[test]
fn a_pending_mark_never_erases_a_doubt() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let imports = server.imports_dir().unwrap();
    let doubt = json!({"state":"posted_under_changed_masters","ledgers":["Cash"]});
    server.record_masters_check_pending("batch-a").unwrap();
    server.record_masters_verdict("batch-a", doubt.clone());
    server.record_masters_check_pending("batch-a").unwrap();
    assert_eq!(read_masters_check(&imports, "batch-a"), Some(doubt));
}

#[path = "agent_import_ack_tests.rs"]
mod ack_tests;
#[path = "agent_import_approval_tests.rs"]
mod approval_tests;

/// A post whose ledger no longer resolves to the master approved names the
/// ledger only in the masters list, where the configured redaction applies:
/// under none it is there, and under mask_parties it is nowhere in the
/// response; the message names none.
#[tokio::test]
async fn a_changed_masters_post_names_no_ledger_under_mask_parties() {
    let replaced = replaced_once(
        &catalogue(),
        ">61c6de69-1748-461c-ad3f-162cb949df9f-0000001f</GUID>",
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000ff</GUID>",
    );
    for redaction in [
        crate::agent::Redaction::None,
        crate::agent::Redaction::MaskParties,
    ] {
        let mut plans = before_approval();
        plans.extend(after_approval(xml(created_one())));
        plans.push(xml(masters_moved_to(8)));
        plans.extend(paired(replaced.clone()));
        plans.extend(posted_readback());
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_redacting(simulator.address(), directory.path(), redaction);
        let args = saved_captured_batch(&server);
        let response = SCRIPTED_APPROVAL
            .scope(
                ScriptedApproval::approving(),
                server.call_tool("post_import", args),
            )
            .await;
        let _ = sent(simulator);
        let result = &response["structuredContent"]["result"];
        assert_eq!(
            result["error"]["code"], "posted_under_changed_masters",
            "{response}"
        );
        assert_eq!(result["error"]["message"], super::CHANGED_MASTERS_MESSAGE);
        // The whole response: the ledger is there under none, and nowhere
        // under mask_parties.
        assert_eq!(
            response.to_string().contains("Cash"),
            redaction == crate::agent::Redaction::None,
            "{response}"
        );
    }
}

/// A post refused because a ledger changed since the build names the ledger
/// only in its list, where the configured redaction applies: under none it is
/// there, and under mask_parties it is nowhere in the result.
#[tokio::test]
async fn a_changed_ledger_refusal_names_no_ledger_under_mask_parties() {
    let identities = vec![
        BoundLedger {
            name: "Cash".into(),
            guid: "61c6de69-1748-461c-ad3f-162cb949df9f-000000ff".into(),
        },
        BoundLedger {
            name: "WR2 Sales".into(),
            guid: "61c6de69-1748-461c-ad3f-162cb949df9f-000000d0".into(),
        },
    ];
    for redaction in [
        crate::agent::Redaction::None,
        crate::agent::Redaction::MaskParties,
    ] {
        let (result, _, _, _) =
            refused_by_build_binding_under(Some(identities.clone()), false, redaction).await;
        assert_eq!(
            result["error"]["code"], "import_masters_changed_since_build",
            "{result}"
        );
        // The whole result: the ledger is listed under none, and appears
        // nowhere under mask_parties.
        assert_eq!(
            result.to_string().contains("Cash"),
            redaction == crate::agent::Redaction::None,
            "{result}"
        );
    }
}

/// A post whose readback fails after its masters check found a changed ledger
/// reports that check with the ledger marked: named under none, masked under
/// mask_parties.
#[tokio::test]
async fn a_failed_readback_reports_changed_masters_with_the_ledger_marked() {
    let replaced = replaced_once(
        &catalogue(),
        ">61c6de69-1748-461c-ad3f-162cb949df9f-0000001f</GUID>",
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000ff</GUID>",
    );
    for redaction in [
        crate::agent::Redaction::None,
        crate::agent::Redaction::MaskParties,
    ] {
        // No readback is scripted, so the read after the post fails.
        let mut plans = before_approval();
        plans.extend(after_approval(xml(created_one())));
        plans.push(xml(masters_moved_to(8)));
        plans.extend(paired(replaced.clone()));
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_redacting(simulator.address(), directory.path(), redaction);
        let args = saved_captured_batch(&server);
        let response = SCRIPTED_APPROVAL
            .scope(
                ScriptedApproval::approving(),
                server.call_tool("post_import", args),
            )
            .await;
        let _ = sent(simulator);
        let result = &response["structuredContent"]["result"];
        // The failure path: no dispatch verdict, and the failure's own error.
        assert!(result.get("dispatch").is_none(), "{response}");
        assert_ne!(
            result["error"]["code"], "posted_under_changed_masters",
            "{response}"
        );
        assert!(result["error"]["code"].is_string(), "{response}");
        assert_eq!(
            result["masters_after_post"]["state"], "posted_under_changed_masters",
            "{response}"
        );
        let ledgers = result["masters_after_post"]["ledgers"].as_array().unwrap();
        assert_eq!(ledgers.len(), 1, "{response}");
        assert_eq!(
            ledgers[0] == "Cash",
            redaction == crate::agent::Redaction::None,
            "{response}"
        );
    }
}

/// The answer of a post whose readback fails lists the changed ledgers in the
/// order the batch names them too. The saved Journal names `WR2 Sales` before
/// `Cash`, so the two orders differ; the record keeps the order it was written
/// in.
#[tokio::test]
async fn a_failed_readback_lists_the_changed_ledgers_in_the_order_the_batch_names_them() {
    let replaced = replaced_once(
        &replaced_once(
            &catalogue(),
            ">61c6de69-1748-461c-ad3f-162cb949df9f-0000001f</GUID>",
            ">61c6de69-1748-461c-ad3f-162cb949df9f-000000ff</GUID>",
        ),
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000d0</GUID>",
        ">61c6de69-1748-461c-ad3f-162cb949df9f-000000fe</GUID>",
    );
    // No readback is scripted, so the read after the post fails.
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(masters_moved_to(8)));
    plans.extend(paired(replaced));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let _ = sent(simulator);
    let result = &response["structuredContent"]["result"];
    assert!(result.get("dispatch").is_none(), "{response}");
    assert_eq!(
        result["masters_after_post"]["ledgers"],
        json!(["WR2 Sales", "Cash"]),
        "{response}"
    );
    assert_eq!(
        masters_check_of(&server, &line.batch_id)["ledgers"],
        json!(["Cash", "WR2 Sales"]),
        "{response}"
    );
}

/// A voucher the book already holds that matches the saved batch's row by
/// accounting fingerprint (date, type, ledgers, amounts, sides) but was posted by
/// no batch of this journal: entered by hand, before the batch's pre-import mark.
fn a_hand_entered_twin_of_the_saved_row() -> String {
    format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>\
         <VOUCHER REMOTEID=\"{GUID}-00000005\"><DATE>20260901</DATE><VOUCHERNUMBER>5</VOUCHERNUMBER>\
         <VOUCHERTYPENAME>Journal</VOUCHERTYPENAME><GUID>{GUID}-00000005</GUID><MASTERID>5</MASTERID>\
         <ALTERID>5</ALTERID><NARRATION>Entered by hand</NARRATION>\
         <ISCANCELLED>No</ISCANCELLED><ISOPTIONAL>No</ISOPTIONAL>\
         <ALLLEDGERENTRIES.LIST><LEDGERNAME>Cash</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>12.50</AMOUNT></ALLLEDGERENTRIES.LIST>\
         <ALLLEDGERENTRIES.LIST><LEDGERNAME>WR2 Sales</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-12.50</AMOUNT></ALLLEDGERENTRIES.LIST>\
         </VOUCHER></COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

/// The reads up to the absence check, on a book that already holds the twin.
fn before_approval_on_a_book_holding_a_twin() -> Vec<ScenarioPlan> {
    let mut plans = Vec::new();
    plans.extend(probe());
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(a_hand_entered_twin_of_the_saved_row()));
    plans.extend(paired(a_hand_entered_twin_of_the_saved_row()));
    plans.extend(probe());
    plans
}

/// A batch one of whose rows the book already holds is refused before the dialog
/// and names that row, with the way on (#901). Before, the answer was a bare
/// code an agent could not act on.
#[tokio::test]
async fn a_refusal_for_a_row_already_in_the_book_names_the_row() {
    let simulator =
        SequenceSimulator::spawn(with_sentinel(before_approval_on_a_book_holding_a_twin()))
            .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_line, args) = saved_batch(&server);
    let refused = server.call_tool("post_import", args).await;
    let _ = sent(simulator);
    let error = &refused["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "import_preexisting_identity", "{refused}");
    assert_eq!(
        error["preexisting_txn_ids"],
        json!(["journal-583"]),
        "{refused}"
    );
    let step = error["next_step"].as_str().unwrap();
    assert!(step.contains("preexisting_txn_ids"), "{step}");
    assert!(step.contains("on whole days"), "{step}");
    assert!(step.contains("do not enter the row by hand"), "{step}");
    assert!(
        step.contains("build the batch again and ComplyEaze Bridge checks the book again"),
        "{step}"
    );
    assert!(step.contains("never change a row"), "{step}");
    assert_eq!(
        refused["structuredContent"]["result"]["attempt_recorded"], false,
        "{refused}"
    );
    assert!(String::from_utf8(journal(directory.path()))
        .unwrap()
        .lines()
        .all(|record| !record.contains("dispatch_intent")));
}

/// On a response budget too small for remediation the rows and the next step are
/// withheld, as `cause` is, and the refusal keeps its code (#901): the added
/// fields cannot turn it into the oversize answer.
#[tokio::test]
async fn a_tiny_response_budget_keeps_the_code_and_withholds_the_row_list() {
    let simulator =
        SequenceSimulator::spawn(with_sentinel(before_approval_on_a_book_holding_a_twin()))
            .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: crate::agent::REMEDIATION_MIN_RESPONSE_BUDGET - 1,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    });
    let (_line, args) = saved_batch(&server);
    let refused = server.call_tool("post_import", args).await;
    let _ = sent(simulator);
    let error = &refused["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "import_preexisting_identity", "{refused}");
    assert!(error.get("preexisting_txn_ids").is_none(), "{refused}");
    assert!(error.get("next_step").is_none(), "{refused}");
}

// Binding a native post by its own AlterID span (agent_import_span_identity.rs).
// The post sends an untagged narration; the readback below is the captured
// posted Journal (`native-namespaced-journal`) with four changes, each named:
// its `[BRIDGE:…]` tag removed (the post no longer writes one), ALTERID 10 →
// 11 (inside the span (10, 11] the scripted marks give), and MASTERID 5 → 295
// with the GUID suffix to match (`created_one()`'s captured LASTVCHID is 295).
fn untagged_posted_journal() -> String {
    let body = captured_posted_journal();
    let body = replaced_once(&body, " [BRIDGE:9c8d8de4-c06c-847b-8309-60ba702bf663]", "");
    let body = replaced_once(
        &body,
        "<ALTERID TYPE=\"Number\"> 10</ALTERID>",
        "<ALTERID TYPE=\"Number\"> 11</ALTERID>",
    );
    let body = replaced_once(
        &body,
        "<MASTERID TYPE=\"Number\"> 5</MASTERID>",
        "<MASTERID TYPE=\"Number\"> 295</MASTERID>",
    );
    body.replace(&format!("{GUID}-00000005"), &format!("{GUID}-00000127"))
}

/// A verification of the posted batch: the window twice serving `window`,
/// then the target's marks at `mark`.
fn span_readback(window: String, mark: u64) -> Vec<ScenarioPlan> {
    let mut plans = probe();
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(window.clone()));
    plans.extend(paired(window));
    plans.extend(paired(marks_at(mark)));
    plans
}

/// Posts the captured batch with `marks_after` answering the snapshot after the
/// POST and `readback` the verification after it, then runs one later
/// `verify_import` per entry of `later`, each against its own simulator (the
/// journal's origin moved to it, since a dispatched batch verifies only on the
/// origin it recorded, and one sequence is capped at 128 requests). Returns the
/// post's response, each later verification's response, and the journal's
/// post-span verdict records.
async fn post_and_verify(
    marks_after: String,
    readback: Vec<ScenarioPlan>,
    later: Vec<Vec<ScenarioPlan>>,
) -> (Value, Vec<Value>, Vec<Value>) {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(marks_after));
    plans.extend(readback);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let posted = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    let _ = sent(simulator);
    let mut origin =
        super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let mut verified = Vec::new();
    for plans in later {
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let later_server = server_at(simulator.address(), directory.path());
        let later_origin =
            super::super::super::canonical_loopback_origin(&later_server.settings.endpoint)
                .unwrap();
        let text = String::from_utf8(journal(directory.path())).unwrap();
        fs::write(
            directory.path().join("agent-import-ledger.jsonl"),
            text.replace(&origin, &later_origin),
        )
        .unwrap();
        origin = later_origin;
        verified.push(later_server.call_tool("verify_import", args.clone()).await);
        let _ = sent(simulator);
    }
    let verdicts = String::from_utf8(journal(directory.path()))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "post_span_verdict")
        .collect::<Vec<_>>();
    (posted, verified, verdicts)
}

fn bound_journal_identity() -> Value {
    json!([{"bridge_txn_id":"BRIDGE_MCP_LIVE_20260906_A1","guid":format!("{GUID}-00000127"),"master_id":295}])
}

/// The untagged post binds to the voucher its own POST created and reads back
/// `posted_verified` by that binding; the binding is journaled once.
#[tokio::test]
async fn an_untagged_native_post_binds_to_its_own_span_and_verifies() {
    let (posted, _, verdicts) = post_and_verify(
        company_marks(11, 50, "WR2 Unicode Lab"),
        span_readback(untagged_posted_journal(), 11),
        Vec::new(),
    )
    .await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(posted["isError"], json!(false), "{posted}");
    assert_eq!(result["dispatch"]["state"], "posted_verified", "{posted}");
    assert_eq!(result["post_span_binding"]["state"], "bound", "{posted}");
    assert_eq!(
        result["vouchers"][0]["marker"], "post_span_binding",
        "{posted}"
    );
    assert_eq!(verdicts.len(), 1, "{verdicts:?}");
    assert_eq!(verdicts[0]["bindings"], bound_journal_identity());
    assert!(verdicts[0].get("binding_refusal").is_none());
}

/// The proof's `alter_id_delta` measures a native post from the mark read just
/// before its POST, not from the mark recorded when the batch was built
/// (#1087). This batch was built at 8 and the aim snapshot before the POST read
/// 10, so measuring from the build mark would report a delta of 3 beside one
/// voucher created; the build mark stays in `pre_import_mark`.
#[tokio::test]
async fn a_native_posts_alter_id_delta_is_measured_from_the_mark_before_its_post() {
    let (posted, _, _) = post_and_verify(
        company_marks(11, 50, "WR2 Unicode Lab"),
        span_readback(untagged_posted_journal(), 11),
        Vec::new(),
    )
    .await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(result["dispatch"]["state"], "posted_verified", "{posted}");
    assert_eq!(
        result["alter_id_delta"],
        json!({"before": 10, "after_seen": 11, "delta": 1, "from": "pre_post_mark"}),
        "{posted}"
    );
    assert_eq!(result["post_location"]["target_voucher_step"]["before"], 10);
    assert_eq!(result["pre_import_mark"]["value"], 8, "{posted}");
}

/// A bound voucher a later window does not hold is `bound_not_in_window`,
/// never `not_found`, and the batch is not verified.
#[tokio::test]
async fn a_bound_voucher_missing_from_a_later_window_is_never_read_as_absent() {
    let (_, verified, verdicts) = post_and_verify(
        company_marks(11, 50, "WR2 Unicode Lab"),
        span_readback(untagged_posted_journal(), 11),
        vec![span_readback(empty_collection(), 11)],
    )
    .await;
    let result = &verified[0]["structuredContent"]["result"];
    assert_eq!(result["counts"]["bound_not_in_window"], 1, "{result}");
    assert_eq!(result["counts"]["not_found"], 0, "{result}");
    assert_eq!(
        result["unverified_vouchers"][0]["status"], "bound_not_in_window",
        "{result}"
    );
    assert_eq!(
        result["unverified_vouchers"][0]["next_step"],
        json!(
            super::super::verification::plain_next_step("bound_not_in_window")
                .expect("a plain line for bound_not_in_window")
        ),
        "{result}"
    );
    assert_eq!(
        result["verification_status"], "verification_incomplete",
        "{result}"
    );
    assert_eq!(
        result["error"]["code"], "import_reconciliation_required",
        "{result}"
    );
    assert_eq!(
        verdicts.len(),
        1,
        "the later verify journals no second verdict"
    );
}

/// A book that reads below the mark the post left was rolled back (a backup
/// restored over it): its vouchers are `book_rolled_back`, never `not_found`.
#[tokio::test]
async fn a_book_rolled_back_below_the_post_reads_as_rolled_back_never_absent() {
    let (_, verified, _) = post_and_verify(
        company_marks(11, 50, "WR2 Unicode Lab"),
        span_readback(untagged_posted_journal(), 11),
        vec![span_readback(empty_collection(), 10)],
    )
    .await;
    let result = &verified[0]["structuredContent"]["result"];
    assert_eq!(
        result["post_span_binding"]["state"], "book_rolled_back",
        "{result}"
    );
    assert_eq!(result["post_span_binding"]["current_mark"], 10, "{result}");
    assert_eq!(
        result["post_span_binding"]["expected_at_least"], 11,
        "{result}"
    );
    assert_eq!(result["counts"]["not_found"], 0, "{result}");
    assert_eq!(result["counts"]["book_rolled_back"], 1, "{result}");
    assert_eq!(
        result["unverified_vouchers"][0]["status"], "book_rolled_back",
        "{result}"
    );
    assert_eq!(
        result["unverified_vouchers"][0]["next_step"],
        json!(
            super::super::verification::plain_next_step("book_rolled_back")
                .expect("a plain line for book_rolled_back")
        ),
        "{result}"
    );
    assert_eq!(
        result["verification_status"], "verification_incomplete",
        "{result}"
    );
    assert_eq!(
        result["error"]["code"], "import_reconciliation_required",
        "{result}"
    );
}

/// When the mark after the POST could not be read, the post is bound on the
/// span its clean response implies, in the same call: the measurement is
/// missing, not failed.
#[tokio::test]
async fn an_unread_after_mark_binds_on_the_span_the_clean_response_implies() {
    let unreadable = "<ENVELOPE><BODY><DATA></DATA></BODY></ENVELOPE>".to_string();
    // An unreadable snapshot proves no master unmoved, so the approved ledgers
    // are read again before the readback.
    let mut readback = paired(catalogue());
    readback.extend(span_readback(untagged_posted_journal(), 11));
    let (posted, _, verdicts) = post_and_verify(unreadable, readback, Vec::new()).await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(result["post_span_binding"]["state"], "bound", "{posted}");
    assert_eq!(result["dispatch"]["state"], "posted_verified", "{posted}");
    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0]["bindings"], bound_journal_identity());
}

/// A mark after the POST that stepped by more than Tally created means
/// something else changed the book during the post: the binding is refused,
/// journaled, and never retried by a later verification.
#[tokio::test]
async fn a_step_other_than_created_refuses_the_binding_for_good() {
    let (posted, verified, verdicts) = post_and_verify(
        company_marks(12, 50, "WR2 Unicode Lab"),
        span_readback(untagged_posted_journal(), 12),
        vec![span_readback(untagged_posted_journal(), 12)],
    )
    .await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(result["post_span_binding"]["state"], "refused", "{posted}");
    assert_eq!(
        result["post_span_binding"]["code"], "span_step_not_created",
        "{posted}"
    );
    assert_ne!(result["dispatch"]["state"], "posted_verified", "{posted}");
    let later = &verified[0]["structuredContent"]["result"];
    assert_eq!(later["post_span_binding"]["state"], "refused", "{later}");
    assert_eq!(
        later["post_span_binding"]["code"], "span_step_not_created",
        "{later}"
    );
    assert_eq!(verdicts.len(), 1, "{verdicts:?}");
    assert_eq!(verdicts[0]["binding_refusal"], "span_step_not_created");
}

/// The tagged capture, renumbered into the span an untagged post of it left.
fn tagged_capture_in_span() -> String {
    let mut tagged_in_span = captured_posted_journal();
    tagged_in_span = replaced_once(
        &tagged_in_span,
        "<ALTERID TYPE=\"Number\"> 10</ALTERID>",
        "<ALTERID TYPE=\"Number\"> 11</ALTERID>",
    );
    tagged_in_span = replaced_once(
        &tagged_in_span,
        "<MASTERID TYPE=\"Number\"> 5</MASTERID>",
        "<MASTERID TYPE=\"Number\"> 295</MASTERID>",
    );
    tagged_in_span.replace(&format!("{GUID}-00000005"), &format!("{GUID}-00000127"))
}

/// The tagged capture is not what the untagged post sent: its narration
/// carries the tag, so the binding refuses on content, for good.
#[tokio::test]
async fn a_readback_whose_narration_differs_refuses_the_binding() {
    let (posted, _, verdicts) = post_and_verify(
        company_marks(11, 50, "WR2 Unicode Lab"),
        span_readback(tagged_capture_in_span(), 11),
        Vec::new(),
    )
    .await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(result["post_span_binding"]["state"], "refused", "{posted}");
    assert_eq!(
        result["post_span_binding"]["code"], "span_content_mismatch",
        "{posted}"
    );
    assert_eq!(verdicts[0]["binding_refusal"], "span_content_mismatch");
    // The readback's tag is not this untagged post's: the voucher is matched
    // by content only and never reads verified through the tag.
    assert_eq!(result["counts"]["posted_verified"], 0, "{posted}");
    assert_eq!(
        result["vouchers"][0]["status"], "matching_content_observed",
        "{posted}"
    );
    assert_eq!(
        result["vouchers"][0]["marker"], "accounting_fingerprint",
        "{posted}"
    );
    // The first line says the book holds a voucher of the same content, and
    // never that it is this post's (#1039).
    assert_eq!(
        result["post_span_binding"]["summary"],
        "ComplyEaze Bridge could not confirm which Tally vouchers this post created, so the batch stays open. For each voucher it sent, the book holds a voucher with the same date, voucher type and ledger entries, but ComplyEaze Bridge cannot tell whether that one is this post's: check each voucher in Tally before posting any of them again.",
        "{posted}"
    );
}

/// A voucher matched by content only that is optional is not in the accounts:
/// the first line never says the book holds it (#1039).
#[tokio::test]
async fn a_refused_binding_matched_only_by_an_optional_voucher_keeps_the_open_line() {
    let optional = replaced_once(
        &tagged_capture_in_span(),
        "<ISOPTIONAL TYPE=\"Logical\">No</ISOPTIONAL>",
        "<ISOPTIONAL TYPE=\"Logical\">Yes</ISOPTIONAL>",
    );
    let (posted, _, _) = post_and_verify(
        company_marks(11, 50, "WR2 Unicode Lab"),
        span_readback(optional, 11),
        Vec::new(),
    )
    .await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(result["post_span_binding"]["state"], "refused", "{posted}");
    assert_eq!(
        result["vouchers"][0]["status"], "matching_content_observed",
        "{posted}"
    );
    assert_eq!(
        result["vouchers"][0]["accounting_effective"], false,
        "{posted}"
    );
    assert_eq!(
        result["post_span_binding"]["summary"],
        "ComplyEaze Bridge could not confirm which Tally vouchers this post created, so the batch stays open: check its vouchers in Tally before posting any of them again.",
        "{posted}"
    );
}

/// A copy that holds no voucher at all (one never used, put in place of the
/// book) omits its voucher mark: that reads as 0, so the post's vouchers are
/// `book_rolled_back`, never a failed verification and never `not_found`.
#[tokio::test]
async fn a_copy_with_no_vouchers_in_place_of_the_book_reads_as_rolled_back() {
    let no_voucher_mark = format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY>\
         <GUID>{GUID}</GUID><ALTMSTID>7</ALTMSTID>\
         </COMPANY></COLLECTION></DATA></BODY></ENVELOPE>"
    );
    let mut later = probe();
    later.extend(verified_company());
    later.extend(paired(marks()));
    later.extend(paired(empty_collection()));
    later.extend(paired(empty_collection()));
    later.extend(paired(no_voucher_mark));
    let (_, verified, _) = post_and_verify(
        company_marks(11, 50, "WR2 Unicode Lab"),
        span_readback(untagged_posted_journal(), 11),
        vec![later],
    )
    .await;
    let result = &verified[0]["structuredContent"]["result"];
    assert_eq!(
        result["post_span_binding"]["state"], "book_rolled_back",
        "{result}"
    );
    assert_eq!(result["post_span_binding"]["current_mark"], 0, "{result}");
    assert_eq!(result["counts"]["not_found"], 0, "{result}");
    assert_eq!(
        result["unverified_vouchers"][0]["status"], "book_rolled_back",
        "{result}"
    );
    assert_eq!(
        result["unverified_vouchers"][0]["next_step"],
        json!(
            super::super::verification::plain_next_step("book_rolled_back")
                .expect("a plain line for book_rolled_back")
        ),
        "{result}"
    );
}

/// An untagged post whose binding was refused cannot be found by a tag, so a
/// later window its content does not match (an edit in Tally) reads it as
/// `sent_not_attributed`, never `not_found`.
#[tokio::test]
async fn an_unbound_untagged_voucher_the_window_lacks_is_never_read_as_absent() {
    let (_, verified, verdicts) = post_and_verify(
        company_marks(12, 50, "WR2 Unicode Lab"),
        span_readback(untagged_posted_journal(), 12),
        vec![span_readback(empty_collection(), 12)],
    )
    .await;
    assert_eq!(verdicts[0]["binding_refusal"], "span_step_not_created");
    let result = &verified[0]["structuredContent"]["result"];
    assert_eq!(result["counts"]["not_found"], 0, "{result}");
    assert_eq!(result["counts"]["sent_not_attributed"], 1, "{result}");
    assert_eq!(
        result["unverified_vouchers"][0]["status"], "sent_not_attributed",
        "{result}"
    );
    assert_eq!(
        result["unverified_vouchers"][0]["next_step"],
        json!(
            super::super::verification::plain_next_step("sent_not_attributed")
                .expect("a plain line for sent_not_attributed")
        ),
        "{result}"
    );
    assert_eq!(
        result["verification_status"], "verification_incomplete",
        "{result}"
    );
}

// bridge#1108: a single voucher Tally rejects. Its post's own answer says Tally
// created none of the one sent and raised an exception, so the voucher is
// reported as not created by Tally, never as possibly edited in Tally.

/// The Education-mode answer to a rejected single voucher (CREATED 0,
/// EXCEPTIONS 1), derived from a live capture with its LINEERROR text redacted:
/// counter shape only, see EDUCATION_IMPORT_COUNTERS_PROVENANCE.md.
fn rejected_one_education() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/live_education_w7_baddate_sanitized.xml"
    )
    .to_string()
}

/// The licensed 7.1 Silver answer to a rejected single voucher, committed byte
/// for byte (`single-import-missing-ledger`, 2026-10-02): CREATED 0,
/// EXCEPTIONS 1, one LINEERROR naming a ledger the book did not hold. Only the
/// answer is borrowed; the voucher posted here is this suite's own.
fn rejected_one_silver() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/single-import-missing-ledger.utf16le.xml"
    ))
}

/// Posts the captured single-voucher batch with `answer` as Tally's answer to
/// the POST, the marks after it unmoved and an empty window: Tally created
/// nothing.
async fn post_single_rejected(answer: String) -> Value {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(answer)));
    plans.push(xml(company_marks(10, 50, "WR2 Unicode Lab")));
    plans.extend(span_readback(empty_collection(), 10));
    let expected_requests = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let posted = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    assert_eq!(sent(simulator).len(), expected_requests, "{posted}");
    posted
}

fn assert_reported_not_created(posted: &Value) {
    let result = &posted["structuredContent"]["result"];
    assert_eq!(posted["isError"], json!(true), "{posted}");
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{posted}"
    );
    assert_eq!(result["counts"]["posted_verified"], 0, "{posted}");
    assert_eq!(result["counts"]["not_found"], 0, "{posted}");
    // Only the status a voucher moved to is counted: none moved to
    // sent_not_attributed.
    assert_eq!(
        result["counts"].get("sent_not_attributed"),
        None,
        "{posted}"
    );
    assert_eq!(
        result["counts"]["tally_reported_not_created"], 1,
        "{posted}"
    );
    // Tally created nothing, so the post is not bound to its span, and its
    // first line names Tally's answer as the cause (#1108).
    assert_eq!(result["post_span_binding"]["state"], "refused", "{posted}");
    assert_eq!(
        result["post_span_binding"]["summary"],
        "Tally reported that this post created none of its vouchers: follow each voucher's next step.",
        "{posted}"
    );
    let voucher = &result["vouchers"][0];
    assert_eq!(voucher["status"], "tally_reported_not_created", "{posted}");
    let next_step = voucher["next_step"].as_str().expect("a next step");
    assert_eq!(
        Some(next_step),
        super::super::verification::plain_next_step("tally_reported_not_created"),
        "{posted}"
    );
    // Safety phrases, pinned before any shortening.
    for phrase in [
        "Tally reported this voucher as not created",
        "Check that it is not in Tally",
        "enter this one voucher",
        "do not import it again through Tally's Import menu",
        "will not send this saved voucher again",
    ] {
        assert!(next_step.contains(phrase), "{phrase}: {next_step}");
    }
}

/// A single voucher Tally rejected (Education answer) reads as not created by
/// Tally, with the next step that says so.
#[tokio::test]
async fn a_rejected_single_voucher_reads_as_not_created_on_the_education_answer_shape() {
    let posted = post_single_rejected(rejected_one_education()).await;
    assert_eq!(
        posted["structuredContent"]["result"]["dispatch"]["counters"]["created"],
        0
    );
    assert_reported_not_created(&posted);
}

/// The same on licensed 7.1 Silver's own answer.
#[tokio::test]
async fn a_rejected_single_voucher_reads_as_not_created_on_the_silver_answer() {
    let posted = post_single_rejected(rejected_one_silver()).await;
    assert_eq!(
        posted["structuredContent"]["result"]["dispatch"]["counters"]["created"],
        0
    );
    assert_reported_not_created(&posted);
}

/// A later `verify_import` never reads the post's answer: by then someone may
/// have entered the voucher by hand and edited it, so a voucher it cannot find
/// is `sent_not_attributed`, with the line that says to check in Tally.
#[tokio::test]
async fn a_rejected_single_voucher_verified_later_is_not_labelled_from_the_old_answer() {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(rejected_one_silver())));
    plans.push(xml(company_marks(10, 50, "WR2 Unicode Lab")));
    plans.extend(span_readback(empty_collection(), 10));
    let post_requests = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let args = saved_captured_batch(&server);
    let posted = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(sent(simulator).len(), post_requests, "{posted}");
    assert_reported_not_created(&posted);
    // The later check runs against its own simulator; a dispatched batch
    // verifies only on the origin it recorded, so the journal moves with it.
    let later_plans = span_readback(empty_collection(), 10);
    let expected_requests = later_plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(later_plans)).unwrap();
    let later_server = server_at(simulator.address(), directory.path());
    let origin = super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let later_origin =
        super::super::super::canonical_loopback_origin(&later_server.settings.endpoint).unwrap();
    let text = String::from_utf8(journal(directory.path())).unwrap();
    fs::write(
        directory.path().join("agent-import-ledger.jsonl"),
        text.replace(&origin, &later_origin),
    )
    .unwrap();
    let later = later_server.call_tool("verify_import", args).await;
    assert_eq!(sent(simulator).len(), expected_requests, "{later}");
    let result = &later["structuredContent"]["result"];
    assert_eq!(later["isError"], json!(true), "{later}");
    assert_eq!(result["counts"]["sent_not_attributed"], 1, "{later}");
    assert_eq!(
        result["counts"].get("tally_reported_not_created"),
        None,
        "{later}"
    );
    let voucher = &result["unverified_vouchers"][0];
    assert_eq!(voucher["status"], "sent_not_attributed", "{later}");
    assert_eq!(
        voucher["next_step"].as_str(),
        super::super::verification::plain_next_step("sent_not_attributed"),
        "{later}"
    );
}

// A batch that lands partly (the batch-conditions lab plan, condition 2c).
// The POST is answered by the live response to a three-voucher import whose
// second voucher Tally rejected (`partial-import-missing-ledger`, captured
// 2026-10-02): CREATED 2, ERRORS 0, EXCEPTIONS 1, one LINEERROR. Bridge's own
// path refuses a missing ledger at build, so the batch here names ledgers the
// catalogue holds; the rejection stands for one Tally makes after approval (a
// ledger removed in Tally's screens meanwhile). The LINEERROR names the
// captured run's ledger, which this batch does not carry: §9.2 says its text
// is not a reliable cause, and no verdict reads it. The capture was three
// Payments in another synthetic lab company; this batch is three Journals in
// the harness's captured company, and the window reuses the capture's
// MasterIDs (LASTVCHID 1746). The counter check refuses the binding before
// anything that depends on the voucher type or the company.

/// The captured live answer to an import that created two of three vouchers.
fn created_two_of_three() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/partial-import-missing-ledger.utf16le.xml"
    ))
}

const PARTIAL_AMOUNTS: [&str; 3] = ["12.61", "13.61", "14.61"];

/// `saved_captured_line`'s Journal three times, each with its own amount, as
/// one batch: the second (13.61) is the one Tally rejects.
fn saved_partial_batch(server: &Server) -> (ImportLedgerLine, Value) {
    let mut line = saved_captured_line(server);
    let template = line.vouchers[0].clone();
    line.vouchers = PARTIAL_AMOUNTS
        .iter()
        .enumerate()
        .map(|(index, amount)| {
            let mut voucher = template.clone();
            voucher.bridge_txn_id = format!("partial-{}", index + 1);
            voucher
                .entries
                .iter_mut()
                .for_each(|entry| entry.amount = (*amount).into());
            voucher
        })
        .collect();
    line.txn_ids = line
        .vouchers
        .iter()
        .map(|voucher| voucher.bridge_txn_id.clone())
        .collect();
    let rendered = render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(rendered.as_bytes());
    bind_to_captured_catalogue(&mut line);
    server.append_import_ledger(&line).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        rendered,
    )
    .unwrap();
    let args = json!({"company_guid":GUID,"batch_id":line.batch_id});
    (line, args)
}

/// The window after the partial post: the captured posted Journal, untagged,
/// twice, as the first and third vouchers. Each copy's named changes: the
/// amount (both entries), ALTERID (11 and 12, inside the span (10, 12] the
/// marks give), and MASTERID with the GUID and REMOTEID suffixes to match
/// (1745 and 1746, the captured response's LASTVCHID being 1746): the window's
/// MasterIDs are derived by named edits from the Payment capture. Each copy's
/// VCHKEY suffix is set to its MasterID only to keep the two keys distinct: it
/// is a synthetic value, not derived from the capture.
fn partial_window() -> String {
    let body = untagged_posted_journal();
    let start = body.find("<VOUCHER ").expect("the capture holds a voucher");
    // Searched from the voucher on: CMPINFO's `<VOUCHER>12</VOUCHER>` count
    // closes first.
    let end = start
        + body[start..]
            .find("</VOUCHER>")
            .expect("the voucher closes")
        + "</VOUCHER>".len();
    let voucher = &body[start..end];
    let copy = |amount: &str, alter_id: u64, master_id: u64| {
        let copy = replaced_once(voucher, ">-12.61<", &format!(">-{amount}<"));
        let copy = replaced_once(&copy, ">12.61<", &format!(">{amount}<"));
        let copy = replaced_once(
            &copy,
            "<ALTERID TYPE=\"Number\"> 11</ALTERID>",
            &format!("<ALTERID TYPE=\"Number\"> {alter_id}</ALTERID>"),
        );
        let copy = replaced_once(
            &copy,
            "<MASTERID TYPE=\"Number\"> 295</MASTERID>",
            &format!("<MASTERID TYPE=\"Number\"> {master_id}</MASTERID>"),
        );
        let copy = copy.replace(
            &format!("{GUID}-00000127"),
            &format!("{GUID}-{master_id:08x}"),
        );
        replaced_once(
            &copy,
            &format!("{GUID}-0000b4bf:00000008"),
            &format!("{GUID}-0000b4bf:{master_id:08x}"),
        )
    };
    let both = format!(
        "{}\n    {}",
        copy(PARTIAL_AMOUNTS[0], 11, 1745),
        copy(PARTIAL_AMOUNTS[2], 12, 1746)
    );
    format!("{}{}{}", &body[..start], both, &body[end..])
}

/// The captured answer is what the test says it is.
#[test]
fn the_partial_post_answer_parses_as_two_created_and_one_exception() {
    let outcome = parse_import_outcome(&created_two_of_three()).expect("the answer parses");
    assert_eq!(outcome.counters().created, 2);
    assert_eq!(outcome.counters().errors, 0);
    assert_eq!(outcome.counters().exceptions, 1);
    assert_eq!(outcome.counters().line_error_count, 1);
    assert_eq!(outcome.last_vch_id(), Some(1746));
    assert!(!import_outcome_is_clean(Some(&outcome), 3));
    // Not clean even against the number it did create: the exception decides.
    assert!(!import_outcome_is_clean(Some(&outcome), 2));
}

/// A batch that lands partly is loud and never verified: the binding is
/// refused on the counters and journaled once; the two vouchers that landed
/// are matched by content only; the one that did not is never read as absent.
#[tokio::test]
async fn a_batch_that_lands_partly_is_never_verified_and_shows_which_rows_landed() {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_two_of_three())));
    // The POST is the last request of the dispatch (see `after_approval`).
    let post_at = plans.len() - 1;
    plans.push(xml(company_marks(12, 50, "WR2 Unicode Lab")));
    plans.extend(span_readback(partial_window(), 12));
    let expected_requests = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = batch_server_at(simulator.address(), directory.path());
    let (line, args) = saved_partial_batch(&server);
    let posted = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    // Every scripted answer was asked for, and nothing more.
    let observed = sent(simulator);
    assert_eq!(observed.len(), expected_requests, "{posted}");
    // The POST carried exactly the saved batch, rendered with the REMOTEIDs
    // its dispatch intent journaled before the send.
    let intent = dispatch_intent(directory.path());
    let recorded_sha = intent["native_request_sha256"].as_str().unwrap();
    assert_eq!(observed[post_at].request_body_sha256, recorded_sha);
    let remote_ids = intent["native_remote_ids"]
        .as_array()
        .expect("a batch intent journals one REMOTEID per voucher")
        .iter()
        .map(|id| Uuid::parse_str(id.as_str().unwrap()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(remote_ids.len(), 3);
    let rendered = native_post_request(&line, RemoteIds::from_ids(remote_ids)).unwrap();
    assert_eq!(rendered.request_sha256, recorded_sha);
    assert_eq!(posted["isError"], json!(true), "{posted}");
    let result = &posted["structuredContent"]["result"];
    assert_eq!(result["post_span_binding"]["state"], "refused", "{posted}");
    assert_eq!(
        result["post_span_binding"]["code"], "span_counters_not_clean",
        "{posted}"
    );
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{posted}"
    );
    assert_eq!(
        result["dispatch"]["response_state"], "response_not_clean",
        "{posted}"
    );
    assert_eq!(result["dispatch"]["counters"]["created"], 2, "{posted}");
    assert_eq!(result["dispatch"]["counters"]["exceptions"], 1, "{posted}");
    assert_eq!(
        result["error"]["code"], "import_reconciliation_required",
        "{posted}"
    );
    assert_eq!(result["counts"]["posted_verified"], 0, "{posted}");
    assert_eq!(result["counts"]["matching_content_observed"], 2, "{posted}");
    assert_eq!(result["counts"]["sent_not_attributed"], 1, "{posted}");
    // Two of three are matched by content: the first line says so, and never
    // that they are this post's (#1039).
    assert_eq!(
        result["post_span_binding"]["summary"],
        "ComplyEaze Bridge could not confirm which Tally vouchers this post created, so the batch stays open. For some vouchers it sent, the book holds a voucher with the same date, voucher type and ledger entries, but ComplyEaze Bridge cannot tell whether that one is this post's: check each voucher in Tally before posting any of them again.",
        "{posted}"
    );
    assert_eq!(result["counts"]["not_found"], 0, "{posted}");
    // Which rows landed, by transaction id: each matched by the content it
    // carries (its amount), never by position.
    let voucher = |id: &str| {
        result["vouchers"]
            .as_array()
            .expect("the result lists its vouchers")
            .iter()
            .find(|voucher| voucher["bridge_txn_id"] == id)
            .unwrap_or_else(|| panic!("{id} is listed: {posted}"))
            .clone()
    };
    for (id, alter_id, master_id) in [("partial-1", 11, "1745"), ("partial-3", 12, "1746")] {
        let landed = voucher(id);
        assert_eq!(
            landed["status"], "matching_content_observed",
            "{id}: {landed}"
        );
        assert_eq!(landed["alter_id"], alter_id, "{id}: {landed}");
        assert_eq!(landed["master_id"], master_id, "{id}: {landed}");
        assert_eq!(landed["diffs"], json!([]), "{id}: {landed}");
    }
    assert_eq!(
        voucher("partial-2")["status"],
        "sent_not_attributed",
        "{posted}"
    );
    let verdicts = String::from_utf8(journal(directory.path()))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "post_span_verdict")
        .collect::<Vec<_>>();
    assert_eq!(verdicts.len(), 1, "{verdicts:?}");
    assert_eq!(verdicts[0]["binding_refusal"], "span_counters_not_clean");
}

// bridge#1108, the batch case: a batch Tally rejected whole. Its post's own
// answer says Tally created none of the vouchers sent, each an exception, and
// the company's voucher mark measurably did not move, so every voucher is
// reported as not created by Tally.

/// The captured live answer to an import of three vouchers, each naming a
/// different missing ledger (`batch-import-all-missing-ledgers`, 2026-10-03):
/// CREATED 0, EXCEPTIONS 3, one LINEERROR. Only the answer is borrowed; the
/// batch posted here is this suite's own three Journals.
fn rejected_all_three() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/batch-import-all-missing-ledgers.utf16le.xml"
    ))
}

/// Posts the three-Journal batch with `answer` as Tally's answer to the POST,
/// the voucher mark read after it at `after_mark` (10 before) and an empty
/// window, and asserts that every scripted answer was asked for.
async fn post_batch_rejected(answer: String, after_mark: u64) -> Value {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(answer)));
    plans.push(xml(company_marks(after_mark, 50, "WR2 Unicode Lab")));
    plans.extend(span_readback(empty_collection(), after_mark));
    let expected_requests = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = batch_server_at(simulator.address(), directory.path());
    let (_, args) = saved_partial_batch(&server);
    let posted = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    assert_eq!(sent(simulator).len(), expected_requests, "{posted}");
    posted
}

#[tokio::test]
async fn a_batch_tally_rejected_whole_reads_as_not_created_by_tally() {
    let posted = post_batch_rejected(rejected_all_three(), 10).await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(posted["isError"], json!(true), "{posted}");
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{posted}"
    );
    assert_eq!(result["dispatch"]["counters"]["created"], 0, "{posted}");
    assert_eq!(result["dispatch"]["counters"]["exceptions"], 3, "{posted}");
    assert_eq!(result["counts"]["posted_verified"], 0, "{posted}");
    assert_eq!(result["counts"]["not_found"], 0, "{posted}");
    assert_eq!(
        result["counts"].get("sent_not_attributed"),
        None,
        "{posted}"
    );
    assert_eq!(
        result["counts"]["tally_reported_not_created"], 3,
        "{posted}"
    );
    assert_eq!(
        result["post_span_binding"]["summary"],
        "Tally reported that this post created none of its vouchers: follow each voucher's next step.",
        "{posted}"
    );
    let vouchers = result["vouchers"].as_array().expect("the vouchers");
    assert_eq!(vouchers.len(), 3, "{posted}");
    for voucher in vouchers {
        assert_eq!(voucher["status"], "tally_reported_not_created", "{posted}");
        assert_eq!(
            voucher["next_step"].as_str(),
            super::super::verification::plain_next_step("tally_reported_not_created"),
            "{posted}"
        );
    }
}

/// The same answer with the voucher mark moved by one across the post: Tally
/// said it created nothing, but something changed the book, so nothing is
/// claimed about any voucher.
#[tokio::test]
async fn a_batch_rejected_whole_whose_mark_moved_is_not_labelled() {
    let posted = post_batch_rejected(rejected_all_three(), 11).await;
    let result = &posted["structuredContent"]["result"];
    assert_eq!(posted["isError"], json!(true), "{posted}");
    assert_eq!(result["counts"]["sent_not_attributed"], 3, "{posted}");
    assert_eq!(
        result["counts"].get("tally_reported_not_created"),
        None,
        "{posted}"
    );
    assert_ne!(
        result["post_span_binding"]["summary"],
        "Tally reported that this post created none of its vouchers: follow each voucher's next step.",
        "{posted}"
    );
}
