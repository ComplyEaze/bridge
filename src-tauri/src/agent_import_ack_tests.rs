//! `acknowledge_post_review` (#239): a person records that they reviewed a
//! post whose masters check found a changed ledger. Every test drives the
//! tool call, against the captured readback the post tests use.
use super::*;

const DOUBT: &str =
    r#"{"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Cash"]}"#;
const ACK: &str = "acknowledge_post_review";

/// A dispatched batch whose saved response is `response`, with the masters
/// records `check` and `doubt` written as given.
fn seeded(
    simulator: &SequenceSimulator,
    directory: &std::path::Path,
    response: ledger::DispatchResponse,
    check: Option<&[u8]>,
    doubt: Option<&[u8]>,
) -> (Server, Value) {
    let server = server_at(simulator.address(), directory);
    let line = saved_captured_line(&server);
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line, &native, None,
            ))
            .unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::response(
                &line,
                ledger::DispatchResponse {
                    request_sha256: native.request_sha256.clone(),
                    ..response
                },
            ))
            .unwrap();
    }
    let imports = server.imports_dir().unwrap();
    if let Some(check) = check {
        fs::write(imports.join(format!("{BATCH}.masters_check.json")), check).unwrap();
    }
    if let Some(doubt) = doubt {
        fs::write(imports.join(format!("{BATCH}.masters_doubt.json")), doubt).unwrap();
    }
    let args = json!({"company_guid":GUID,"batch_id":line.batch_id});
    (server, args)
}

fn clean() -> ledger::DispatchResponse {
    super::super::tests::dispatch_response("success", 1, 0)
}

fn ack_path(server: &Server) -> std::path::PathBuf {
    server
        .imports_dir()
        .unwrap()
        .join(format!("{BATCH}.masters_ack.json"))
}

fn doubted(simulator: &SequenceSimulator, directory: &std::path::Path) -> (Server, Value) {
    seeded(
        simulator,
        directory,
        clean(),
        Some(DOUBT.as_bytes()),
        Some(DOUBT.as_bytes()),
    )
}

/// The captured readback with the voucher's ALTERID replaced.
fn readback_at_alter_id(alter_id: u64) -> Vec<ScenarioPlan> {
    readback_of(replaced_once(
        &captured_posted_journal(),
        "<ALTERID TYPE=\"Number\"> 10</ALTERID>",
        &format!("<ALTERID TYPE=\"Number\"> {alter_id}</ALTERID>"),
    ))
}

/// The captured readback with the voucher's narration changed and its
/// ALTERID left as it was, as an edit that did not move it would read.
fn readback_with_edited_narration() -> Vec<ScenarioPlan> {
    readback_of(replaced_once(
        &captured_posted_journal(),
        "Bridge MCP batch namespace qualification [BRIDGE:",
        "Bridge MCP batch namespace qualification edited [BRIDGE:",
    ))
}

fn readback_of(journal: String) -> Vec<ScenarioPlan> {
    let mut plans = probe();
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(journal.clone()));
    plans.extend(paired(journal));
    plans
}

async fn acknowledge(server: &Server, args: Value, scripted: ScriptedApproval) -> Value {
    SCRIPTED_APPROVAL
        .scope(scripted, server.call_tool(ACK, args))
        .await
}

#[tokio::test]
async fn an_approved_review_is_recorded_once_and_changes_no_verdict() {
    let mut plans = reconcile_readback();
    plans.extend(reconcile_readback());
    plans.extend(reconcile_readback());
    let scripted_plans = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = doubted(&simulator, directory.path());
    let approval = ScriptedApproval::approving();

    let response = acknowledge(&server, args.clone(), approval.clone()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["operator_review"]["state"], "current", "{response}");
    assert_eq!(approval.reviews().len(), 1, "{response}");
    assert_eq!(approval.review_counts(), [1], "{response}");
    assert!(approval.previews().is_empty(), "no post dialog: {response}");
    let review = &approval.reviews()[0];
    assert!(review.contains("Cash"), "the doubt is shown: {review}");
    assert!(
        review.contains("ALTERID: 10"),
        "the voucher as read: {review}"
    );
    let record: Value = serde_json::from_slice(&fs::read(ack_path(&server)).unwrap()).unwrap();
    assert_eq!(record["batch_id"], BATCH);
    assert_eq!(record["alter_id"], 10);
    assert_eq!(
        record["doubt_sha256"],
        crate::agent::sha256_hex(DOUBT.as_bytes())
    );

    // A later readback keeps every verdict it had, and reports the review.
    let verified = server.call_tool("verify_import", args).await;
    let result = &verified["structuredContent"]["result"];
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{verified}"
    );
    assert_eq!(result["error"]["code"], "posted_under_changed_masters");
    assert_eq!(result["operator_review"]["state"], "current", "{verified}");
    assert_eq!(
        server
            .latest_import_snapshot(BATCH)
            .unwrap()
            .unwrap()
            .batch
            .status,
        "verification_incomplete"
    );
    assert_eq!(sent(simulator).len(), scripted_plans);
}

#[tokio::test]
async fn a_declined_review_writes_nothing() {
    let simulator = SequenceSimulator::spawn(with_sentinel(reconcile_readback())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = doubted(&simulator, directory.path());
    let approval = ScriptedApproval::declining();
    let response = acknowledge(&server, args, approval.clone()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["error"]["code"], "ack_review_declined", "{response}");
    assert_eq!(approval.reviews().len(), 1);
    assert!(!ack_path(&server).exists());
}

/// Each refusal names its reason, shows no dialog and writes nothing.
async fn refused(
    plans: Vec<ScenarioPlan>,
    response: ledger::DispatchResponse,
    check: Option<&[u8]>,
    doubt: Option<&[u8]>,
    code: &str,
) {
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = seeded(&simulator, directory.path(), response, check, doubt);
    let approval = ScriptedApproval::approving();
    let outcome = acknowledge(&server, args, approval.clone()).await;
    assert_eq!(
        outcome["structuredContent"]["result"]["error"]["code"], code,
        "{outcome}"
    );
    assert!(approval.reviews().is_empty(), "{code}: no dialog");
    assert!(!ack_path(&server).exists(), "{code}: nothing written");
}

#[tokio::test]
async fn a_batch_without_an_observed_doubt_is_refused() {
    let unchanged = br#"{"state":"unchanged","trigger":"masters_moved"}"#;
    refused(
        reconcile_readback(),
        clean(),
        Some(unchanged),
        None,
        "ack_no_observed_doubt",
    )
    .await;
}

#[tokio::test]
async fn a_pending_check_is_refused_not_acknowledged() {
    // The readback finishes a pending check only with the catalogue it
    // scripts; none is scripted, so the check stays pending.
    let pending = br#"{"state":"check_pending"}"#;
    refused(
        reconcile_readback(),
        clean(),
        Some(pending),
        None,
        "ack_check_pending",
    )
    .await;
}

/// A doubt outranks a check left pending beside it (a write that failed
/// between the two): nothing would ever finish that check, so refusing would
/// refuse the batch forever.
#[tokio::test]
async fn a_doubt_beside_a_check_left_pending_is_reviewable() {
    let mut plans = reconcile_readback();
    plans.extend(reconcile_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = seeded(
        &simulator,
        directory.path(),
        clean(),
        Some(br#"{"state":"check_pending"}"#),
        Some(DOUBT.as_bytes()),
    );
    let response = acknowledge(&server, args, ScriptedApproval::approving()).await;
    assert_eq!(
        response["structuredContent"]["result"]["operator_review"]["state"], "current",
        "{response}"
    );
}

#[tokio::test]
async fn an_unreadable_masters_record_is_refused() {
    refused(
        reconcile_readback(),
        clean(),
        Some(b"not json"),
        None,
        "ack_masters_record_unreadable",
    )
    .await;
    // A doubt must name each of its ledgers, as Bridge writes it.
    let unnamed = br#"{"state":"posted_under_changed_masters","ledgers":["Cash",7]}"#;
    refused(
        reconcile_readback(),
        clean(),
        Some(unnamed),
        Some(unnamed),
        "ack_masters_record_unreadable",
    )
    .await;
    refused(
        reconcile_readback(),
        clean(),
        Some(DOUBT.as_bytes()),
        Some(b"{\"state\":"),
        "ack_masters_record_unreadable",
    )
    .await;
    // A doubt file holding anything but that verdict is not one to bind to.
    refused(
        reconcile_readback(),
        clean(),
        Some(DOUBT.as_bytes()),
        Some(br#"{"state":"unchanged"}"#),
        "ack_masters_record_unreadable",
    )
    .await;
    // A doubt that names no ledger shows nothing to review.
    let nameless = br#"{"state":"posted_under_changed_masters","ledgers":[]}"#;
    refused(
        reconcile_readback(),
        clean(),
        Some(nameless),
        Some(nameless),
        "ack_masters_record_unreadable",
    )
    .await;
}

#[tokio::test]
async fn a_response_that_was_not_clean_is_refused() {
    let altered = super::super::tests::dispatch_response("success", 1, 1);
    refused(
        reconcile_readback(),
        altered,
        Some(DOUBT.as_bytes()),
        Some(DOUBT.as_bytes()),
        "ack_response_not_clean",
    )
    .await;
}

#[tokio::test]
async fn a_readback_that_does_not_match_is_refused() {
    let mut plans = probe();
    plans.extend(verified_company());
    plans.extend(paired(marks()));
    plans.extend(paired(empty_collection()));
    plans.extend(paired(empty_collection()));
    plans.extend(probe());
    refused(
        plans,
        clean(),
        Some(DOUBT.as_bytes()),
        Some(DOUBT.as_bytes()),
        "ack_readback_not_matched",
    )
    .await;
}

#[tokio::test]
async fn a_review_is_recorded_only_once() {
    let mut plans = reconcile_readback();
    plans.extend(reconcile_readback());
    plans.extend(reconcile_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = doubted(&simulator, directory.path());
    let first = acknowledge(&server, args.clone(), ScriptedApproval::approving()).await;
    assert_eq!(
        first["structuredContent"]["result"]["operator_review"]["state"], "current",
        "{first}"
    );
    let recorded = fs::read(ack_path(&server)).unwrap();
    let approval = ScriptedApproval::approving();
    let second = acknowledge(&server, args, approval.clone()).await;
    assert_eq!(
        second["structuredContent"]["result"]["error"]["code"], "ack_already_recorded",
        "{second}"
    );
    assert!(approval.reviews().is_empty(), "refused before the dialog");
    assert_eq!(fs::read(ack_path(&server)).unwrap(), recorded);
}

#[tokio::test]
async fn a_doubt_that_changes_while_the_dialog_is_open_is_refused() {
    let mut plans = reconcile_readback();
    plans.extend(reconcile_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = doubted(&simulator, directory.path());
    let doubt_path = server
        .imports_dir()
        .unwrap()
        .join(format!("{BATCH}.masters_doubt.json"));
    let approval = ScriptedApproval::approving_after(move || {
        let other = r#"{"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Sales"]}"#;
        fs::write(&doubt_path, other).unwrap();
    });
    let response = acknowledge(&server, args, approval).await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ack_changed_while_reviewing",
        "{response}"
    );
    assert!(!ack_path(&server).exists());
}

#[tokio::test]
async fn a_voucher_that_changes_while_the_dialog_is_open_is_refused() {
    for second in [readback_at_alter_id(11), readback_with_edited_narration()] {
        let mut plans = reconcile_readback();
        plans.extend(second);
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let (server, args) = doubted(&simulator, directory.path());
        let response = acknowledge(&server, args, ScriptedApproval::approving()).await;
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"], "ack_changed_while_reviewing",
            "{response}"
        );
        assert!(!ack_path(&server).exists());
    }
}

/// A recorded review covers only the doubt it showed and the voucher as it
/// was: each binding, changed alone, makes it stale.
#[tokio::test]
async fn a_recorded_review_goes_stale_when_anything_it_bound_changes() {
    type Change = fn(&Server);
    let untouched: Change = |_| {};
    let new_doubt: Change = |server| {
        let other = r#"{"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Sales"]}"#;
        let path = server
            .imports_dir()
            .unwrap()
            .join(format!("{BATCH}.masters_doubt.json"));
        fs::write(path, other).unwrap();
    };
    let other_batch: Change = |server| {
        let path = ack_path(server);
        let mut record: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        record["batch_id"] = json!("bridge-00000000-0000-4000-8000-000000000001");
        fs::write(path, serde_json::to_vec(&record).unwrap()).unwrap();
    };
    let other_company: Change = |server| {
        edit_record(
            server,
            "company_guid",
            json!("00000000-0000-4000-8000-000000000002"),
        )
    };
    let other_fields: Change =
        |server| edit_record(server, "voucher_fingerprint_fields", json!("v0:guid"));
    let other_guid: Change = |server| {
        edit_record(
            server,
            "voucher_guid",
            json!("61c6de69-1748-461c-ad3f-162cb949df9f-00000006"),
        )
    };
    let other_master_id: Change = |server| edit_record(server, "voucher_master_id", json!("6"));
    let cases: [(&str, Vec<ScenarioPlan>, Change); 8] = [
        ("alter_id", readback_at_alter_id(11), untouched),
        ("fingerprint", readback_with_edited_narration(), untouched),
        ("doubt", reconcile_readback(), new_doubt),
        ("identity", reconcile_readback(), other_batch),
        ("company", reconcile_readback(), other_company),
        ("fingerprint_fields", reconcile_readback(), other_fields),
        ("voucher_guid", reconcile_readback(), other_guid),
        ("voucher_master_id", reconcile_readback(), other_master_id),
    ];
    for (name, later, change) in cases {
        let mut plans = reconcile_readback();
        plans.extend(reconcile_readback());
        plans.extend(later);
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let (server, args) = doubted(&simulator, directory.path());
        let recorded = acknowledge(&server, args.clone(), ScriptedApproval::approving()).await;
        assert_eq!(
            recorded["structuredContent"]["result"]["operator_review"]["state"], "current",
            "{name}: {recorded}"
        );
        change(&server);
        let verified = server.call_tool("verify_import", args).await;
        let result = &verified["structuredContent"]["result"];
        assert_eq!(
            result["operator_review"]["state"], "stale",
            "{name}: {verified}"
        );
        assert_eq!(
            result["dispatch"]["state"], "reconciliation_required",
            "{name}"
        );
    }
}

#[tokio::test]
async fn the_tool_needs_the_posting_opt_in() {
    let listed = |writes: bool| {
        crate::agent::catalog::registered_tool_definitions(true, writes)
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"] == ACK)
    };
    assert!(listed(true));
    assert!(!listed(false));
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(crate::agent::Settings {
        writes_enabled: false,
        batch_post_enabled: false,
        ..server_at(simulator.address(), directory.path())
            .settings
            .clone()
    });
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":GUID,"batch_id":BATCH}),
        approval.clone(),
    )
    .await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "import_posting_disabled",
        "{response}"
    );
    assert!(approval.reviews().is_empty());
    assert!(sent(simulator).is_empty());
}

fn edit_record(server: &Server, field: &str, value: Value) {
    let path = ack_path(server);
    let mut record: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    record[field] = value;
    fs::write(path, serde_json::to_vec(&record).unwrap()).unwrap();
}

/// A record in a format this build does not know is reported unreadable,
/// never matched.
#[tokio::test]
async fn a_record_this_build_cannot_read_is_reported_unreadable() {
    type Change = fn(&Server);
    let newer: Change = |server| edit_record(server, "version", json!(2));
    let unknown_field: Change = |server| edit_record(server, "approved", json!(true));
    let garbage: Change = |server| fs::write(ack_path(server), b"not json").unwrap();
    let doubt_unreadable: Change = |server| {
        let path = server
            .imports_dir()
            .unwrap()
            .join(format!("{BATCH}.masters_doubt.json"));
        fs::write(path, b"not json").unwrap();
    };
    for (name, change) in [
        ("version", newer),
        ("field", unknown_field),
        ("garbage", garbage),
        ("doubt_unreadable", doubt_unreadable),
    ] {
        let mut plans = reconcile_readback();
        plans.extend(reconcile_readback());
        plans.extend(reconcile_readback());
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let (server, args) = doubted(&simulator, directory.path());
        let recorded = acknowledge(&server, args.clone(), ScriptedApproval::approving()).await;
        assert_eq!(
            recorded["structuredContent"]["result"]["operator_review"]["state"], "current",
            "{name}: {recorded}"
        );
        change(&server);
        let verified = server.call_tool("verify_import", args).await;
        let result = &verified["structuredContent"]["result"];
        assert_eq!(
            result["operator_review"]["state"], "unreadable",
            "{name}: {verified}"
        );
    }
}

#[tokio::test]
async fn a_batch_never_posted_is_refused_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved_captured_line(&server);
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":GUID,"batch_id":line.batch_id}),
        approval.clone(),
    )
    .await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ack_batch_not_posted",
        "{response}"
    );
    assert!(approval.reviews().is_empty());
    assert!(sent(simulator).is_empty());
}

/// A readback that finds the marked voucher but not as posted is not a match.
#[tokio::test]
async fn a_readback_that_diverges_is_refused() {
    let diverged = readback_of(replaced_once(
        &captured_posted_journal(),
        "Bridge Nested Debtor WR4",
        "Bridge Nested Debtor WR5",
    ));
    refused(
        diverged,
        clean(),
        Some(DOUBT.as_bytes()),
        Some(DOUBT.as_bytes()),
        "ack_readback_not_matched",
    )
    .await;
}

/// A record that appears while the dialog is open is never overwritten.
#[tokio::test]
async fn a_record_written_while_the_dialog_is_open_is_kept() {
    let mut plans = reconcile_readback();
    plans.extend(reconcile_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = doubted(&simulator, directory.path());
    let path = ack_path(&server);
    let planted = path.clone();
    let approval = ScriptedApproval::approving_after(move || {
        fs::write(&planted, b"written elsewhere").unwrap();
    });
    let response = acknowledge(&server, args, approval).await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ack_already_recorded",
        "{response}"
    );
    assert_eq!(fs::read(&path).unwrap(), b"written elsewhere");
}

/// A doubt outranks a check record that cannot be read beside it, as it does
/// for the verdict, which never rewrites that record once a doubt exists.
#[tokio::test]
async fn a_doubt_beside_an_unreadable_check_is_reviewable() {
    let mut plans = reconcile_readback();
    plans.extend(reconcile_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = seeded(
        &simulator,
        directory.path(),
        clean(),
        Some(b"not json"),
        Some(DOUBT.as_bytes()),
    );
    let response = acknowledge(&server, args, ScriptedApproval::approving()).await;
    assert_eq!(
        response["structuredContent"]["result"]["operator_review"]["state"], "current",
        "{response}"
    );
}

/// With a doubt and no record, the review reads absent; with a record and
/// no doubt it can bind to, it reads stale, never nothing.
#[tokio::test]
async fn a_review_reads_absent_before_a_record_and_stale_without_its_doubt() {
    let mut plans = reconcile_readback();
    plans.extend(reconcile_readback());
    plans.extend(reconcile_readback());
    plans.extend(reconcile_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = doubted(&simulator, directory.path());
    let before = server.call_tool("verify_import", args.clone()).await;
    assert_eq!(
        before["structuredContent"]["result"]["operator_review"],
        json!({"state":"absent"}),
        "{before}"
    );
    acknowledge(&server, args.clone(), ScriptedApproval::approving()).await;
    let imports = server.imports_dir().unwrap();
    fs::remove_file(imports.join(format!("{BATCH}.masters_doubt.json"))).unwrap();
    fs::write(
        imports.join(format!("{BATCH}.masters_check.json")),
        br#"{"state":"unchanged"}"#,
    )
    .unwrap();
    let after = server.call_tool("verify_import", args).await;
    assert_eq!(
        after["structuredContent"]["result"]["operator_review"]["state"], "stale",
        "{after}"
    );
}

/// A two-voucher batch whose dispatch intent is journaled, as `post_import`
/// leaves one just before its POST.
fn dispatched_batch(server: &Server) -> ImportLedgerLine {
    let mut line = saved_captured_line(server);
    let mut second = line.vouchers[0].clone();
    second.bridge_txn_id = "BRIDGE_MCP_LIVE_20260906_A2".into();
    line.vouchers.push(second);
    line.txn_ids.push("BRIDGE_MCP_LIVE_20260906_A2".into());
    server.append_import_ledger(&line).unwrap();
    // One REMOTEID per voucher: the intent records the batch's two.
    let native = native_post_request(
        &line,
        RemoteIds::from_ids(vec![Uuid::new_v4(), Uuid::new_v4()]),
    )
    .unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line, &native, None,
            ))
            .unwrap();
    }
    line
}

/// A single Payment Bridge posted, captured (fixtures `wa1-payment-*`, one
/// `verify_import`): its debit reads back from Tally as `-1.00`, with the
/// trailing zeros the post dialog showed.
const WA1_BATCH: &str = "bridge-78ce4328-9c2a-4827-8584-821d6d656b40";

/// That readback, in the captured order, as [`d3_batch_readback`] reads the
/// batch's.
fn wa1_payment_readback() -> Vec<ScenarioPlan> {
    let extent = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-company-extent.utf16le.xml"
    ));
    let high_water = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-company-high-water.utf16le.xml"
    ));
    let census = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-voucher-census.utf16le.xml"
    ));
    let readback = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-import-verification.utf16le.xml"
    ));
    "SEESESEHSHSEECSCSEEVSVSEEVSVSE"
        .chars()
        .map(|step| match step {
            'S' => status(),
            'E' => xml(extent.clone()),
            'H' => xml(high_water.clone()),
            'C' => xml(census.clone()),
            _ => xml(readback.clone()),
        })
        .collect()
}

/// The sha256 of each request that capture carried, in the same order; `None`
/// for the status probe. The extent and high-water requests are the d3
/// capture's too.
fn wa1_payment_requests() -> Vec<Option<&'static str>> {
    "SEESESEHSHSEECSCSEEVSVSEEVSVSE"
        .chars()
        .map(|step| match step {
            'S' => None,
            'E' => Some("9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8"),
            'H' => Some("0930288f6eb531926d018cc4762288084831b8fad16a554e48fafbd694e245c2"),
            'C' => Some("bad372eab88eed8009e302e573309d34aba6edf3fdb2da8a6b4bc2b1330ec7eb"),
            _ => Some("a29a7448361733300968450db07bbbbc0754fd8eac8901decc1fc94510bc6955"),
        })
        .collect()
}

/// The review shows each entry of the voucher exactly as the post dialog
/// showed it for the same voucher (#730): the post dialog's lines, rendered
/// from the post's own journal, are each a line of the review, read back from
/// Tally's capture. Tally sent the debit as `-1.00`, so this proves the sign
/// and the digits: a canonical negation would read `Dr 1`.
#[tokio::test]
async fn the_review_shows_each_entry_as_the_post_dialog_did() {
    let mut plans = wa1_payment_readback();
    plans.extend(wa1_payment_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let origin =
        super::super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    fs::write(
        directory.path().join("agent-import-ledger.jsonl"),
        include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-journal.jsonl"
        )
        .replace("http://127.0.0.1:9102", &origin),
    )
    .unwrap();
    let imports = server.imports_dir().unwrap();
    fs::write(
        imports.join(format!("{WA1_BATCH}.xml")),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-import.xml"
        ),
    )
    .unwrap();
    for record in ["masters_check", "masters_doubt"] {
        fs::write(imports.join(format!("{WA1_BATCH}.{record}.json")), DOUBT).unwrap();
    }
    let line = server
        .import_ledger()
        .unwrap()
        .into_iter()
        .find(|line| line.batch_id == WA1_BATCH)
        .unwrap();
    let post = admit_fresh_saved_voucher(&line, &server.settings.endpoint).unwrap();
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":D3_GUID,"batch_id":WA1_BATCH}),
        approval.clone(),
    )
    .await;
    assert!(
        response["structuredContent"]["result"]["error"].is_null(),
        "{response}"
    );
    assert_eq!(approval.reviews().len(), 1, "{response}");
    let review = &approval.reviews()[0];
    let entries = post
        .lines()
        .filter(|line| line.starts_with("Dr ") || line.starts_with("Cr "))
        .collect::<Vec<_>>();
    assert_eq!(
        entries,
        ["Dr 1.00  \"Test Expense B\"", "Cr 1.00  \"Cash\""],
        "{post}"
    );
    for entry in entries {
        assert!(
            review.lines().any(|line| line == entry),
            "{entry}: {review}"
        );
    }
    // Every request Bridge sent is the one the capture answered.
    let requests = sent(simulator);
    let expected = [wa1_payment_requests(), wa1_payment_requests()].concat();
    assert_eq!(requests.len(), expected.len(), "{requests:?}");
    for (index, (request, expected)) in requests.iter().zip(expected).enumerate() {
        match expected {
            None => assert_eq!(request.method, "GET", "request {index}"),
            Some(sha256) => assert_eq!(request.request_body_sha256, sha256, "request {index}"),
        }
    }
}

/// A batch of several vouchers with no doubt recorded is refused before any
/// request: there is nothing to review. (Before batch reviews, slice D2b,
/// any batch was refused here as `ack_batch_not_posted`.)
#[tokio::test]
async fn a_batch_of_several_vouchers_with_no_doubt_is_refused_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = dispatched_batch(&server);
    let response = acknowledge(
        &server,
        json!({"company_guid":GUID,"batch_id":line.batch_id}),
        ScriptedApproval::approving(),
    )
    .await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ack_no_observed_doubt",
        "{response}"
    );
    assert!(sent(simulator).is_empty());
}

/// A batch whose step verdict is still pending is refused before any request:
/// only a post records that verdict, so no read could finish it.
#[tokio::test]
async fn a_batch_step_review_left_pending_is_refused_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = dispatched_batch(&server);
    server
        .record_post_checks_pending(&line.batch_id, true)
        .unwrap();
    let response = acknowledge(
        &server,
        json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"batch_step"}),
        ScriptedApproval::approving(),
    )
    .await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ack_check_pending",
        "{response}"
    );
    assert!(sent(simulator).is_empty());
}

/// A doubt whose own file cannot be written (#722): the write is made to fail
/// by a directory standing where the file goes. The check record keeps the
/// doubt and says why no review can find it, and a review of that doubt is
/// refused before any request, for each kind. The control is the same doubt
/// with its file written, whose check record carries no mark; that such a
/// doubt reads as reviewable is the unit tests' control.
#[tokio::test]
async fn a_batch_doubt_whose_own_file_was_not_written_is_refused_before_any_request() {
    let changed = json!({"state":"posted_under_changed_masters","ledgers":["Cash"]});
    let step =
        json!({"before":10,"after":13,"step":3,"reported_created":2,"matches_created":false});
    for (kind, blocked) in [
        ("masters", "masters_doubt.json"),
        ("batch_step", "batch_step_doubt.json"),
    ] {
        for write_fails in [true, false] {
            let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
            let directory = tempfile::tempdir().unwrap();
            let server = server_at(simulator.address(), directory.path());
            let line = dispatched_batch(&server);
            let imports = server.imports_dir().unwrap();
            server
                .record_post_checks_pending(&line.batch_id, true)
                .unwrap();
            let doubt_path = imports.join(format!("{}.{blocked}", line.batch_id));
            if write_fails {
                block(doubt_path.clone());
            }
            // In the post's own order: the step verdict, then the masters
            // verdict, which rewrites the check record around the step.
            let (step, masters) = if kind == "masters" {
                (json!({"matches_created":true}), changed.clone())
            } else {
                (step.clone(), json!({"state":"unchanged"}))
            };
            server.record_batch_step_verdict(&line.batch_id, &step);
            server.record_masters_verdict_for(&line.batch_id, masters, true);
            if write_fails {
                fs::remove_dir_all(&doubt_path).unwrap();
            }
            assert_eq!(doubt_path.is_file(), !write_fails, "{kind}");
            let check: Value = serde_json::from_slice(
                &fs::read(imports.join(format!("{}.masters_check.json", line.batch_id))).unwrap(),
            )
            .unwrap();
            let verdict = if kind == "masters" {
                &check
            } else {
                &check["batch_step"]
            };
            // The verdict recorded is the doubt itself, for either kind.
            assert_eq!(
                verdict["state"],
                if kind == "masters" {
                    "posted_under_changed_masters"
                } else {
                    "unmatched"
                },
                "{kind}: {check}"
            );
            assert_eq!(
                verdict["doubt_record"],
                if write_fails {
                    json!("unavailable")
                } else {
                    Value::Null
                },
                "{kind}: {check}"
            );
            // Marked or not, the verdict is still doubt.
            let expected = if kind == "masters" {
                "posted_under_changed_masters"
            } else {
                "batch_step_unconfirmed"
            };
            assert_eq!(
                post_doubt(
                    read_masters_check(&imports, &line.batch_id).as_ref(),
                    line.vouchers.len()
                )
                .map(|(code, _)| code),
                Some(expected),
                "{kind}: {check}"
            );
            if !write_fails {
                continue;
            }
            let response = acknowledge(
                &server,
                json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":kind}),
                ScriptedApproval::approving(),
            )
            .await;
            assert_eq!(
                response["structuredContent"]["result"]["error"]["code"],
                "ack_doubt_record_unavailable",
                "{kind}: {response}"
            );
            assert!(sent(simulator).is_empty(), "{kind}: no request");
        }
    }
}

/// With no doubt named, a batch whose masters doubt is held only by the check
/// record, beside a step doubt with its own file, needs a name; one whose two
/// doubts are both held only by the check record is refused as unavailable,
/// since no name could be reviewed. Either way, before any request.
#[tokio::test]
async fn an_unnamed_review_beside_a_doubt_without_its_file_is_refused_before_any_request() {
    let changed = json!({"state":"posted_under_changed_masters","ledgers":["Cash"]});
    let step =
        json!({"before":10,"after":13,"step":3,"reported_created":2,"matches_created":false});
    for (step_file_fails, code) in [
        (false, "ack_doubt_ambiguous"),
        (true, "ack_doubt_record_unavailable"),
    ] {
        let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let line = dispatched_batch(&server);
        let imports = server.imports_dir().unwrap();
        server
            .record_post_checks_pending(&line.batch_id, true)
            .unwrap();
        let masters_doubt = imports.join(format!("{}.masters_doubt.json", line.batch_id));
        let step_doubt = imports.join(format!("{}.batch_step_doubt.json", line.batch_id));
        block(masters_doubt.clone());
        if step_file_fails {
            block(step_doubt.clone());
        }
        server.record_batch_step_verdict(&line.batch_id, &step);
        server.record_masters_verdict_for(&line.batch_id, changed.clone(), true);
        fs::remove_dir_all(&masters_doubt).unwrap();
        if step_file_fails {
            fs::remove_dir_all(&step_doubt).unwrap();
        }
        assert_eq!(step_doubt.is_file(), !step_file_fails, "{code}");
        // The check record holds both doubts, each marked when its file failed.
        let check: Value = serde_json::from_slice(
            &fs::read(imports.join(format!("{}.masters_check.json", line.batch_id))).unwrap(),
        )
        .unwrap();
        assert_eq!(check["state"], "posted_under_changed_masters", "{check}");
        assert_eq!(check["doubt_record"], "unavailable", "{check}");
        assert_eq!(check["batch_step"]["state"], "unmatched", "{check}");
        assert_eq!(
            check["batch_step"]["doubt_record"],
            if step_file_fails {
                json!("unavailable")
            } else {
                Value::Null
            },
            "{check}"
        );
        let response = acknowledge(
            &server,
            json!({"company_guid":GUID,"batch_id":line.batch_id}),
            ScriptedApproval::approving(),
        )
        .await;
        let error = &response["structuredContent"]["result"]["error"];
        assert_eq!(error["code"], code, "{response}");
        if code == "ack_doubt_ambiguous" {
            assert_eq!(error["cause"], "masters_and_batch_step", "{response}");
        }
        assert!(sent(simulator).is_empty(), "{code}: no request");
    }
}

/// A masters review was recorded while its doubt file existed; that file was
/// later lost, and the step doubt's own file was never written. An unnamed
/// review is refused as unavailable before any request, where master before
/// #769 asked for a name (`ack_doubt_ambiguous`), and a review naming
/// `masters` is refused the same way, where before #770 the stale review
/// answered `ack_already_recorded`.
#[tokio::test]
async fn two_doubts_without_their_files_are_refused_named_or_not() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = dispatched_batch(&server);
    let imports = server.imports_dir().unwrap();
    server
        .record_post_checks_pending(&line.batch_id, true)
        .unwrap();
    let step_doubt = imports.join(format!("{}.batch_step_doubt.json", line.batch_id));
    block(step_doubt.clone());
    server.record_batch_step_verdict(
        &line.batch_id,
        &json!({"before":10,"after":13,"step":3,"reported_created":2,"matches_created":false}),
    );
    server.record_masters_verdict_for(
        &line.batch_id,
        json!({"state":"posted_under_changed_masters","ledgers":["Cash"]}),
        true,
    );
    fs::remove_dir_all(&step_doubt).unwrap();
    let masters_doubt = imports.join(format!("{}.masters_doubt.json", line.batch_id));
    assert!(masters_doubt.is_file(), "the masters doubt was written");
    // The check record holds both doubts: the masters one unmarked, since its
    // file was written, and the step one marked, since its file was not.
    let check: Value = serde_json::from_slice(
        &fs::read(imports.join(format!("{}.masters_check.json", line.batch_id))).unwrap(),
    )
    .unwrap();
    assert_eq!(check["state"], "posted_under_changed_masters", "{check}");
    assert_eq!(check["doubt_record"], Value::Null, "{check}");
    assert_eq!(check["batch_step"]["state"], "unmatched", "{check}");
    assert_eq!(
        check["batch_step"]["doubt_record"], "unavailable",
        "{check}"
    );
    // A review of it recorded, then its doubt file lost.
    fs::write(
        imports.join(format!("{}.masters_ack.json", line.batch_id)),
        b"{}",
    )
    .unwrap();
    fs::remove_file(&masters_doubt).unwrap();
    for args in [
        json!({"company_guid":GUID,"batch_id":line.batch_id}),
        json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"masters"}),
        json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"batch_step"}),
    ] {
        let response = acknowledge(&server, args.clone(), ScriptedApproval::approving()).await;
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"],
            "ack_doubt_record_unavailable",
            "{args}: {response}"
        );
    }
    assert!(sent(simulator).is_empty(), "no request");
}

/// One voucher's doubt recorded only in the check record is refused before
/// any request, never as no doubt: no read can bring its file back (#770).
#[tokio::test]
async fn a_doubt_recorded_only_in_the_check_record_is_refused() {
    let marked = br#"{"state":"posted_under_changed_masters","ledgers":["Cash"],"doubt_record":"unavailable"}"#;
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = seeded(&simulator, directory.path(), clean(), Some(marked), None);
    let approval = ScriptedApproval::approving();
    let outcome = acknowledge(&server, args, approval.clone()).await;
    assert_eq!(
        outcome["structuredContent"]["result"]["error"]["code"], "ack_doubt_record_unavailable",
        "{outcome}"
    );
    assert!(approval.reviews().is_empty(), "no dialog");
    assert!(!ack_path(&server).exists(), "nothing written");
    assert!(sent(simulator).is_empty(), "no request");
}

/// A review recorded while its doubt file existed, the file then lost: the
/// doubt is refused as unavailable before any request, not answered
/// `ack_already_recorded` by the stale review, which would leave it
/// unreviewable for good (#770). The check record carries no mark, since the
/// file was written and lost later.
#[tokio::test]
async fn a_review_left_by_a_lost_doubt_file_does_not_answer_for_it() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = seeded(
        &simulator,
        directory.path(),
        clean(),
        Some(DOUBT.as_bytes()),
        None,
    );
    fs::write(ack_path(&server), b"{}").unwrap();
    let approval = ScriptedApproval::approving();
    let outcome = acknowledge(&server, args, approval.clone()).await;
    assert_eq!(
        outcome["structuredContent"]["result"]["error"]["code"], "ack_doubt_record_unavailable",
        "{outcome}"
    );
    assert!(approval.reviews().is_empty(), "no dialog");
    assert_eq!(
        fs::read(ack_path(&server)).unwrap(),
        b"{}",
        "the record is kept"
    );
    assert!(sent(simulator).is_empty(), "no request");
}

/// The live batch post of slice D3 (a licensed TallyPrime 7.1 Silver lab, 50
/// Journals on a synthetic company), as the journal and saved file recorded it.
const D3_BATCH: &str = "bridge-1e5b2cd7-f5c6-4d51-adc9-53a4dfa370bb";
const D3_GUID: &str = "17a10910-773c-42c6-bd66-7bba9a392536";

/// One `verify_import` of that batch, answered in the captured order: `S` the
/// status probe, then the company extents (`E`), the company high water
/// (`H`), the voucher census (`C`) and the import verification read (`V`),
/// each response as Tally sent it (fixtures `d3-batch-*`, one capture).
fn d3_batch_readback() -> Vec<ScenarioPlan> {
    d3_readback_of([
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-company-extent.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-company-high-water.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-voucher-census.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-import-verification.utf16le.xml"
        ),
    ])
}

/// One `verify_import` of that batch in the captured order, answered from one
/// capture's extent, high-water, census and verification responses.
fn d3_readback_of([extent, high_water, census, readback]: [&[u8]; 4]) -> Vec<ScenarioPlan> {
    let (extent, high_water, census, readback) = (
        captured(extent),
        captured(high_water),
        captured(census),
        captured(readback),
    );
    "SEESESEHSHSEECSCSEEVSVSEEVSVSE"
        .chars()
        .map(|step| match step {
            'S' => status(),
            'E' => xml(extent.clone()),
            'H' => xml(high_water.clone()),
            'C' => xml(census.clone()),
            _ => xml(readback.clone()),
        })
        .collect()
}

/// The sha256 of each request that capture carried, in the same order; `None`
/// for the status probe.
fn d3_batch_requests() -> Vec<Option<&'static str>> {
    "SEESESEHSHSEECSCSEEVSVSEEVSVSE"
        .chars()
        .map(|step| match step {
            'S' => None,
            'E' => Some("9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8"),
            'H' => Some("0930288f6eb531926d018cc4762288084831b8fad16a554e48fafbd694e245c2"),
            'C' => Some("5a9690e84fef530985b2444a0bc66cd5784a4467571da04e31559c72c6ba11f1"),
            _ => Some("c25ed5f689596b2620c58f417af74217985fbbe54a12f2c317fd537f901ed85f"),
        })
        .collect()
}

/// A server holding the D3 post's journal and saved file as written, with the
/// journal's origin moved to the simulator (a dispatched batch verifies only
/// on the origin it recorded).
fn d3_server(simulator: &SequenceSimulator, directory: &std::path::Path) -> Server {
    let server = server_at(simulator.address(), directory);
    let origin =
        super::super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    fs::write(
        directory.join("agent-import-ledger.jsonl"),
        include_str!("../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-journal.jsonl")
            .replace("http://127.0.0.1:9001", &origin),
    )
    .unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{D3_BATCH}.xml")),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-import.xml"),
    )
    .unwrap();
    server
}

/// A person reviews the doubted 50-voucher batch through the whole path: the
/// read, the dialog, the second read and the write. The record binds every
/// voucher as Tally holds it, in batch order, and a later readback reports it
/// current. The step doubt is a local record written here; every Tally
/// response is captured, and every request sent equals the captured one.
#[tokio::test]
async fn a_review_of_the_captured_50_voucher_batch_binds_every_voucher() {
    let mut plans = d3_batch_readback();
    plans.extend(d3_batch_readback());
    plans.extend(d3_batch_readback());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());
    let imports = server.imports_dir().unwrap();
    let step = json!({"state":"unmatched","target_voucher_step":{
        "before":1419,"after":1470,"step":51,"reported_created":50,"matches_created":false}});
    fs::write(
        imports.join(format!("{D3_BATCH}.batch_step_doubt.json")),
        serde_json::to_vec(&step).unwrap(),
    )
    .unwrap();
    fs::write(
        imports.join(format!("{D3_BATCH}.masters_check.json")),
        serde_json::to_vec(
            &json!({"state":"not_checked","reason":"masters_unmoved","batch_step":step}),
        )
        .unwrap(),
    )
    .unwrap();
    let args = json!({"company_guid":D3_GUID,"batch_id":D3_BATCH,"doubt":"batch_step"});
    let approval = ScriptedApproval::approving();

    let response = acknowledge(&server, args.clone(), approval.clone()).await;
    assert!(
        response["structuredContent"]["result"]["error"].is_null(),
        "{response}"
    );
    assert_eq!(approval.reviews().len(), 1, "{response}");
    // The dialog's title names the fifty vouchers it shows (#746).
    assert_eq!(approval.review_counts(), [50], "{response}");
    let review = &approval.reviews()[0];
    for shown in [
        "Record that you reviewed 50 vouchers in \"BRIDGE AMEND LAB\"",
        "its voucher mark moved by 51 (from 1419 to 1470); Tally reported creating 50.",
        "Dates: 20260401 to 20260401  ALTERIDs: 1420 to 1469",
        "Dr 1275  Cr 0  50 entries  \"Test Expense B\"",
        "Dr 0  Cr 1275  50 entries  \"Cash\"",
        "I reviewed these 50 vouchers in Tally.",
    ] {
        assert!(review.contains(shown), "{shown}: {review}");
    }

    // The batch record: version 2, the step doubt, every voucher in order.
    let record: Value = serde_json::from_slice(
        &fs::read(imports.join(format!("{D3_BATCH}.batch_step_ack.json"))).unwrap(),
    )
    .unwrap();
    assert_eq!(record["version"], 2, "{record}");
    assert_eq!(record["doubt"], "batch_step");
    assert_eq!(
        record["doubt_sha256"],
        crate::agent::sha256_hex(&serde_json::to_vec(&step).unwrap())
    );
    let vouchers = record["vouchers"].as_array().unwrap();
    assert_eq!(
        vouchers
            .iter()
            .map(|voucher| (
                voucher["bridge_txn_id"].as_str().unwrap().to_string(),
                voucher["alter_id"].as_u64().unwrap()
            ))
            .collect::<Vec<_>>(),
        (1..=50)
            .map(|index| (format!("D3-{index:03}"), 1419 + index))
            .collect::<Vec<_>>()
    );
    assert!(!imports
        .join(format!("{D3_BATCH}.masters_ack.json"))
        .exists());

    // A later readback keeps the batch's verdict and reports the review.
    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":D3_GUID,"batch_id":D3_BATCH}),
        )
        .await;
    let result = &verified["structuredContent"]["result"];
    assert_eq!(result["counts"]["posted_verified"], 50, "{verified}");
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{verified}"
    );
    assert_eq!(
        result["operator_review"]["batch_step"]["state"], "current",
        "{verified}"
    );
    assert_eq!(
        result["operator_review"]["masters"],
        Value::Null,
        "{verified}"
    );

    // Every request Bridge sent is the one the capture answered.
    let requests = sent(simulator);
    let expected = [
        d3_batch_requests(),
        d3_batch_requests(),
        d3_batch_requests(),
    ]
    .concat();
    assert_eq!(requests.len(), expected.len(), "{requests:?}");
    for (index, (request, expected)) in requests.iter().zip(expected).enumerate() {
        match expected {
            None => assert_eq!(request.method, "GET", "request {index}"),
            Some(sha256) => assert_eq!(request.request_body_sha256, sha256, "request {index}"),
        }
    }
}

/// The same batch read back after a person cancelled D3-003 in Tally's own
/// screen (Alt+X), captured at the wire in one run (fixtures `d3-cancelled-*`).
/// Tally keeps the cancelled voucher's number and marker but drops its ledger
/// entries, so its content can never match the build; it must still read as
/// cancelled, not as a content change (bridge#758).
#[tokio::test]
async fn a_voucher_cancelled_in_tally_reads_not_effective_not_divergent() {
    let simulator = SequenceSimulator::spawn(with_sentinel(d3_readback_of([
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-company-extent.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-company-high-water.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-voucher-census.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-import-verification.utf16le.xml"
        ),
    ])))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());

    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":D3_GUID,"batch_id":D3_BATCH}),
        )
        .await;
    let result = &verified["structuredContent"]["result"];
    assert_eq!(result["counts"]["posted_not_effective"], 1, "{verified}");
    assert_eq!(result["counts"]["posted_divergent"], 0, "{verified}");
    assert_eq!(result["counts"]["posted_verified"], 49, "{verified}");
    // The one voucher not verified is D3-003, reported as cancelled.
    assert_eq!(
        result["unverified_vouchers"],
        json!([{"bridge_txn_id":"D3-003","status":"posted_not_effective","marker":"narration_tag",
            "reason":"voucher_cancelled","diffs":[],"voucher_number":"3",
            "guid":"17a10910-773c-42c6-bd66-7bba9a392536-00000550","master_id":"1360","alter_id":1685,
            "effective_copies_observed":{"count":0,"entries":[],"attribution":"not_established"}}]),
        "{verified}"
    );
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{verified}"
    );
    let markdown = fs::read_to_string(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{D3_BATCH}.proof.md")),
    )
    .unwrap();
    assert!(
        markdown
            .contains("Readback counts: matching 49, divergent 0, not effective 1, not found 0"),
        "{markdown}"
    );
    assert!(
        markdown.contains("| D3-003 | posted_not_effective |"),
        "{markdown}"
    );

    // Every request Bridge sent is the one the capture answered.
    let requests = sent(simulator);
    let expected = d3_batch_requests();
    assert_eq!(requests.len(), expected.len(), "{requests:?}");
    for (index, (request, expected)) in requests.iter().zip(expected).enumerate() {
        match expected {
            None => assert_eq!(request.method, "GET", "request {index}"),
            Some(sha256) => assert_eq!(request.request_body_sha256, sha256, "request {index}"),
        }
    }
}

/// The batch of the L1 capture (#806): 50 Journals posted to the same lab
/// company as D3, after which a person cancelled L1A-050 (voucher 352) at
/// Tally's screen and entered its content again by hand as voucher 353
/// (`L1_REENTRY_CAPTURE_PROVENANCE.md`).
const L1_BATCH: &str = "bridge-4e4af679-b48b-48ac-b9ca-71f3ea6a507d";

/// A server holding the L1 batch's journal and saved file as written, with
/// the journal's origin (the capture proxy's) moved to the simulator.
fn l1_server(simulator: &SequenceSimulator, directory: &std::path::Path) -> Server {
    let server = server_at(simulator.address(), directory);
    let origin =
        super::super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    fs::write(
        directory.join("agent-import-ledger.jsonl"),
        include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-journal.jsonl"
        )
        .replace("http://127.0.0.1:9102", &origin),
    )
    .unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{L1_BATCH}.xml")),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-import.xml"
        ),
    )
    .unwrap();
    server
}

/// The sha256 of each request the L1 capture carried, in order; its extent
/// and high-water requests are D3's, its census and verification reads its own.
fn l1_requests() -> Vec<Option<&'static str>> {
    "SEESESEHSHSEECSCSEEVSVSEEVSVSE"
        .chars()
        .map(|step| match step {
            'S' => None,
            'E' => Some("9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8"),
            'H' => Some("0930288f6eb531926d018cc4762288084831b8fad16a554e48fafbd694e245c2"),
            'C' => Some("e58e009cb88cb483f89723370fcf89fb0de3aff7db29d1bccb41757826c3dc86"),
            _ => Some("0fe49c89693f59aa011a488f86c2beb97b8f24a87cf32c2aa448e4c51f205418"),
        })
        .collect()
}

/// A cancelled batch voucher whose content a person entered again by hand is
/// still `posted_not_effective`, and now names the effective copy it was
/// re-entered as, without attributing it (#806): 352 stays not effective, 353
/// is listed under `effective_copies_observed` with `attribution`
/// `not_established`, and no verdict changes. Every Tally response is
/// captured, and every request sent equals the captured one.
#[tokio::test]
async fn a_cancelled_voucher_entered_again_by_hand_is_reported_not_attributed() {
    let simulator = SequenceSimulator::spawn(with_sentinel(d3_readback_of([
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-company-extent.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-company-high-water.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-voucher-census.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-import-verification.utf16le.xml"
        ),
    ])))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = l1_server(&simulator, directory.path());
    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":D3_GUID,"batch_id":L1_BATCH}),
        )
        .await;
    let result = &verified["structuredContent"]["result"];
    // The verdict is the one the capture's proof recorded, unchanged.
    assert_eq!(result["counts"]["posted_verified"], 49, "{verified}");
    assert_eq!(result["counts"]["posted_not_effective"], 1, "{verified}");
    assert_eq!(result["counts"]["posted_divergent"], 0, "{verified}");
    assert_eq!(result["counts"]["not_found"], 0, "{verified}");
    assert_eq!(
        result["counts"]["matching_content_observed"], 0,
        "{verified}"
    );
    assert_eq!(
        result["counts"]["cancelled_with_effective_copy"], 1,
        "{verified}"
    );
    assert_eq!(
        result["verification_status"], "verification_incomplete",
        "{verified}"
    );
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{verified}"
    );
    assert_eq!(
        result["unverified_vouchers"],
        json!([{"bridge_txn_id":"L1A-050","status":"posted_not_effective","marker":"narration_tag",
            "reason":"voucher_cancelled","diffs":[],"voucher_number":"352",
            "guid":"17a10910-773c-42c6-bd66-7bba9a392536-000006b7","master_id":"1719","alter_id":1789,
            "effective_copies_observed":{"count":1,"attribution":"not_established","entries":[
                {"guid":"17a10910-773c-42c6-bd66-7bba9a392536-000006b8","master_id":"1720",
                 "alter_id":1790,"voucher_number":"353","before_pre_import_mark":false}]}}]),
        "{verified}"
    );
    let markdown = fs::read_to_string(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{L1_BATCH}.proof.md")),
    )
    .unwrap();
    assert!(
        markdown
            .contains("Readback counts: matching 49, divergent 0, not effective 1, not found 0"),
        "{markdown}"
    );
    assert!(
        markdown.contains(
            "- Cancelled vouchers with an effective copy (report only, not attributed): 1\n"
        ),
        "{markdown}"
    );
    let requests = sent(simulator);
    let expected = l1_requests();
    assert_eq!(requests.len(), expected.len(), "{requests:?}");
    for (index, (request, expected)) in requests.iter().zip(expected).enumerate() {
        match expected {
            None => assert_eq!(request.method, "GET", "request {index}"),
            Some(sha256) => assert_eq!(request.request_body_sha256, sha256, "request {index}"),
        }
    }
}

/// Add a synthetic, unmarked voucher to one captured response: a copy of
/// D3-005 under a new GUID, MASTERID and AlterID, with Bridge's marker removed
/// from its narration. It is not live evidence. Its IDs are arbitrary, chosen
/// only to be unused in the capture: AlterID 1419 with MASTERID 1998 is not a
/// pair Tally would produce for a voucher entered again by hand. The outcome
/// does not depend on them, since D3-005 itself matches by its marker.
fn with_d3_005_copy(bytes: &[u8]) -> Vec<u8> {
    let text = captured(bytes);
    let guid = "<GUID>17a10910-773c-42c6-bd66-7bba9a392536-00000552</GUID>";
    assert_eq!(text.matches(guid).count(), 1);
    let start = text[..text.find(guid).unwrap()].rfind("<VOUCHER ").unwrap();
    let end = start + text[start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let original = &text[start..end];
    let mut copy = original
        .replace("-00000552", "-000009f5")
        .replace(
            "<MASTERID TYPE=\"Number\"> 1362</MASTERID>",
            "<MASTERID TYPE=\"Number\"> 1998</MASTERID>",
        )
        .replace(
            "<ALTERID TYPE=\"Number\"> 1424</ALTERID>",
            "<ALTERID TYPE=\"Number\"> 1419</ALTERID>",
        );
    if let Some(marker) = copy.find(" [BRIDGE:") {
        let close = marker + copy[marker..].find(']').unwrap() + 1;
        copy.replace_range(marker..close, "");
    }
    for field in ["-000009f5", "1998</MASTERID>", "1419</ALTERID>"] {
        assert!(copy.contains(field), "{field}: {copy}");
    }
    assert!(!copy.contains("[BRIDGE:"), "{copy}");
    format!("{}\n    {copy}{}", &text[..end], &text[end..])
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect()
}

/// A batch the accountant imported by hand (the D3 journal with its dispatch
/// records left out), read back with all 50 vouchers matching and a synthetic
/// unmarked copy of D3-005 beside them. The JSON says verification_incomplete
/// for the duplicate. The Markdown proof must say so too, and must not read as
/// a clean post (bridge#804).
#[tokio::test]
async fn a_hand_imported_batch_with_a_duplicate_reads_unverified_in_the_markdown() {
    let census = with_d3_005_copy(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-voucher-census.utf16le.xml"
    ));
    let readback = with_d3_005_copy(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-import-verification.utf16le.xml"
    ));
    let simulator = SequenceSimulator::spawn(with_sentinel(d3_readback_of([
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-company-extent.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-company-high-water.utf16le.xml"
        ),
        &census,
        &readback,
    ])))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());
    // Keep only the built batch and its first verification: no dispatch.
    let journal = directory.path().join("agent-import-ledger.jsonl");
    let lines = fs::read_to_string(&journal).unwrap();
    let kept = lines
        .lines()
        .filter(|line| !line.contains("\"record_type\":\"dispatch_"))
        .take(2)
        .collect::<Vec<_>>();
    assert_eq!(kept.len(), 2);
    assert!(
        !kept.iter().any(|line| line.contains("dispatch")),
        "{kept:?}"
    );
    fs::write(&journal, format!("{}\n", kept.join("\n"))).unwrap();

    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":D3_GUID,"batch_id":D3_BATCH}),
        )
        .await;
    let result = &verified["structuredContent"]["result"];
    assert_eq!(result["counts"]["posted_verified"], 50, "{verified}");
    assert!(result.get("dispatch").is_none(), "{verified}");
    let duplicates = result["duplicates"].as_array().unwrap();
    assert_eq!(duplicates.len(), 1, "{verified}");
    assert_eq!(
        duplicates[0]["kind"], "accounting_fingerprint",
        "{verified}"
    );
    assert_eq!(
        result["verification_status"], "verification_incomplete",
        "{verified}"
    );
    let markdown = fs::read_to_string(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{D3_BATCH}.proof.md")),
    )
    .unwrap();
    assert!(
        markdown.contains("this report does not confirm posting"),
        "{markdown}"
    );
    assert!(
        markdown.contains("- Verification status: `verification_incomplete`"),
        "{markdown}"
    );
    assert!(
        markdown.contains("- Duplicates in this batch: 1"),
        "{markdown}"
    );
    let fingerprint = duplicates[0]["fingerprint_sha256"].as_str().unwrap();
    assert!(
        markdown.contains(&format!("| accounting_fingerprint | `{fingerprint}` | 2 |")),
        "{markdown}"
    );
}

/// The same capture with D3-004 read as cancelled too, derived in memory: its
/// `ISCANCELLED` and ledger entries as the captured D3-003 cancel reads them,
/// every other field as captured for D3-004. That is a state Tally would not
/// produce (a cancel also raised D3-003's AlterID), kept so the census still
/// counts it; the second cancel is synthetic, not live evidence. Two cancelled
/// Journals of one date share a fingerprint with no entries, which says nothing
/// about their content, so they are not duplicates (bridge#767).
#[tokio::test]
async fn two_cancelled_vouchers_of_one_date_and_type_are_not_duplicates() {
    let readback = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-import-verification.utf16le.xml"
    ));
    let guid = "<GUID>17a10910-773c-42c6-bd66-7bba9a392536-00000551</GUID>";
    assert_eq!(readback.matches(guid).count(), 1);
    let start = readback[..readback.find(guid).unwrap()]
        .rfind("<VOUCHER ")
        .unwrap();
    let end = start + readback[start..].find("</VOUCHER>").unwrap();
    let block = &readback[start..end];
    assert!(
        block.contains("<VOUCHERNUMBER>4</VOUCHERNUMBER>"),
        "{block}"
    );
    let effective = r#"<ISCANCELLED TYPE="Logical">No</ISCANCELLED>"#;
    assert_eq!(block.matches(effective).count(), 1, "{block}");
    let first_entry = block.find("<ALLLEDGERENTRIES.LIST>").unwrap();
    let closing = "</ALLLEDGERENTRIES.LIST>";
    let after_entries = block.rfind(closing).unwrap() + closing.len();
    let cancelled_block = format!(
        "{}<ALLLEDGERENTRIES.LIST>     </ALLLEDGERENTRIES.LIST>{}",
        block[..first_entry].replace(
            effective,
            r#"<ISCANCELLED TYPE="Logical">Yes</ISCANCELLED>"#
        ),
        &block[after_entries..]
    );
    let derived = format!(
        "{}{cancelled_block}{}",
        &readback[..start],
        &readback[end..]
    );
    assert_eq!(
        derived
            .matches(r#"<ISCANCELLED TYPE="Logical">Yes</ISCANCELLED>"#)
            .count(),
        2
    );
    let derived = derived
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(with_sentinel(d3_readback_of([
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-company-extent.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-company-high-water.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-voucher-census.utf16le.xml"
        ),
        &derived,
    ])))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());

    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":D3_GUID,"batch_id":D3_BATCH}),
        )
        .await;
    let result = &verified["structuredContent"]["result"];
    assert_eq!(result["counts"]["posted_not_effective"], 2, "{verified}");
    assert_eq!(result["counts"]["posted_verified"], 48, "{verified}");
    let unverified = result["unverified_vouchers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| (row["bridge_txn_id"].as_str(), row["reason"].as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        unverified,
        [
            (Some("D3-003"), Some("voucher_cancelled")),
            (Some("D3-004"), Some("voucher_cancelled"))
        ],
        "{verified}"
    );
    assert_eq!(result["duplicates"], json!([]), "{verified}");
    assert_eq!(
        result["unrelated_duplicates_in_window"],
        json!([]),
        "{verified}"
    );
    assert_eq!(
        result["dispatch"]["state"], "reconciliation_required",
        "{verified}"
    );
    let requests = sent(simulator);
    let expected = d3_batch_requests();
    assert_eq!(requests.len(), expected.len(), "{requests:?}");
    for (index, (request, expected)) in requests.iter().zip(expected).enumerate() {
        match expected {
            None => assert_eq!(request.method, "GET", "request {index}"),
            Some(sha256) => assert_eq!(request.request_body_sha256, sha256, "request {index}"),
        }
    }
}

/// A paired read of `catalogue`, bracketed the way the D3 capture brackets its
/// reads, with its company extent. The one capture (fixture
/// `d3-amend-lab-ledger-catalogue`, read once) answers both reads of the pair.
fn d3_catalogue_read(catalogue: String) -> Vec<ScenarioPlan> {
    let extent = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-company-extent.utf16le.xml"
    ));
    vec![
        xml(extent.clone()),
        xml(catalogue.clone()),
        status(),
        xml(catalogue),
        status(),
        xml(extent),
    ]
}

/// The post path's catalogue request for the D3 company: the one the capture
/// answered (its sidecar's `source_request_sha256`).
const D3_CATALOGUE_REQUEST: &str =
    "589566214e5ab516d415e7a9ea3e143ed63452a8a732de0d50d6cb857c0aef2c";

/// How many of `observed` are that catalogue request.
fn catalogue_requests(observed: &[tally_protocol_simulator::ObservedRequest]) -> usize {
    observed
        .iter()
        .filter(|request| request.request_body_sha256 == D3_CATALOGUE_REQUEST)
        .count()
}

fn d3_catalogue() -> String {
    captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-amend-lab-ledger-catalogue.utf16le.xml"
    ))
}

/// The same catalogue with Cash on another GUID, as a ledger renamed and
/// replaced would read.
fn d3_catalogue_with_cash_replaced() -> String {
    replaced_once(
        &d3_catalogue(),
        ">17a10910-773c-42c6-bd66-7bba9a392536-0000001f</GUID>",
        ">17a10910-773c-42c6-bd66-7bba9a392536-000000ff</GUID>",
    )
}

/// The D3 batch with its step doubt observed and its masters check left
/// pending, as a post that ended before the check finished leaves it.
fn d3_step_doubt_beside_a_pending_check(server: &Server) {
    let imports = server.imports_dir().unwrap();
    let step = json!({"state":"unmatched","target_voucher_step":{
        "before":1419,"after":1470,"step":51,"reported_created":50,"matches_created":false}});
    fs::write(
        imports.join(format!("{D3_BATCH}.batch_step_doubt.json")),
        serde_json::to_vec(&step).unwrap(),
    )
    .unwrap();
    fs::write(
        imports.join(format!("{D3_BATCH}.masters_check.json")),
        serde_json::to_vec(&json!({"state":"check_pending","batch_step":step})).unwrap(),
    )
    .unwrap();
}

/// An unnamed review chooses the step doubt, the only one observed, while
/// the masters check is pending. Its own read finishes that check as a doubt
/// (#756). The review is refused as ambiguous after that read, with no dialog
/// and no record, rather than admitting a step review the person did not
/// name while a masters doubt now stands.
#[tokio::test]
async fn an_unnamed_review_is_refused_when_its_read_finishes_a_second_doubt() {
    let mut plans = d3_batch_readback();
    plans.extend(d3_catalogue_read(d3_catalogue_with_cash_replaced()));
    let scripted = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());
    d3_step_doubt_beside_a_pending_check(&server);
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":D3_GUID,"batch_id":D3_BATCH}),
        approval.clone(),
    )
    .await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["error"]["code"], "ack_doubt_ambiguous", "{response}");
    assert_eq!(
        result["error"]["cause"], "masters_and_batch_step",
        "{response}"
    );
    assert!(approval.reviews().is_empty(), "no dialog: {response}");
    let imports = server.imports_dir().unwrap();
    assert!(!imports
        .join(format!("{D3_BATCH}.batch_step_ack.json"))
        .exists());
    // The read did finish the check as a doubt: that is what refused it.
    assert!(imports
        .join(format!("{D3_BATCH}.masters_doubt.json"))
        .is_file());
    let observed = sent(simulator);
    assert_eq!(observed.len(), scripted, "{response}");
    assert_eq!(catalogue_requests(&observed), 2, "{response}");
}

/// The control: the same read finishes the check as unchanged, the step doubt
/// is still the only one, and the unnamed review goes ahead to its dialog.
#[tokio::test]
async fn an_unnamed_review_goes_ahead_when_its_read_finishes_the_check_unchanged() {
    let mut plans = d3_batch_readback();
    plans.extend(d3_catalogue_read(d3_catalogue()));
    plans.extend(d3_batch_readback());
    let scripted = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());
    d3_step_doubt_beside_a_pending_check(&server);
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":D3_GUID,"batch_id":D3_BATCH}),
        approval.clone(),
    )
    .await;
    assert!(
        response["structuredContent"]["result"]["error"].is_null(),
        "{response}"
    );
    assert_eq!(approval.review_counts(), [50], "{response}");
    let imports = server.imports_dir().unwrap();
    assert!(imports
        .join(format!("{D3_BATCH}.batch_step_ack.json"))
        .is_file());
    assert!(!imports
        .join(format!("{D3_BATCH}.masters_doubt.json"))
        .exists());
    // The first read finished the check, unchanged: the second read sends no
    // catalogue request.
    assert_eq!(masters_check_of(&server, D3_BATCH)["state"], "unchanged");
    let observed = sent(simulator);
    assert_eq!(observed.len(), scripted, "{response}");
    assert_eq!(catalogue_requests(&observed), 2, "{response}");
}

/// The catalogue read refused, as Tally answers when the company cannot be
/// selected (the live answer `moved_masters_that_cannot_be_re_read_are_not_verified`
/// scripts, for this company): the masters check cannot finish and stays
/// pending.
fn d3_catalogue_refused() -> Vec<ScenarioPlan> {
    // Both reads of the pair are sent even when the first is refused.
    d3_catalogue_read(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>0</STATUS></HEADER><BODY><DATA>\
         <LINEERROR>Could not set 'SVCurrentCompany' to 'BRIDGE AMEND LAB'</LINEERROR>\
         </DATA></BODY></ENVELOPE>"
            .to_string(),
    )
}

/// The first read cannot finish the pending masters check (its catalogue read
/// is refused), so the step doubt is still the only one and the dialog is
/// shown. The read after the dialog finishes the check as a doubt (#756): the
/// unnamed review is refused then, and nothing is recorded.
#[tokio::test]
async fn an_unnamed_review_is_refused_when_its_second_read_finishes_a_second_doubt() {
    let mut plans = d3_batch_readback();
    plans.extend(d3_catalogue_refused());
    plans.extend(d3_batch_readback());
    plans.extend(d3_catalogue_read(d3_catalogue_with_cash_replaced()));
    let scripted = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());
    d3_step_doubt_beside_a_pending_check(&server);
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":D3_GUID,"batch_id":D3_BATCH}),
        approval.clone(),
    )
    .await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["error"]["code"], "ack_doubt_ambiguous", "{response}");
    assert_eq!(
        result["error"]["cause"], "masters_and_batch_step",
        "{response}"
    );
    assert_eq!(
        approval.review_counts(),
        [50],
        "the dialog was shown: {response}"
    );
    let imports = server.imports_dir().unwrap();
    assert!(!imports
        .join(format!("{D3_BATCH}.batch_step_ack.json"))
        .exists());
    assert!(imports
        .join(format!("{D3_BATCH}.masters_doubt.json"))
        .is_file());
    // Both reads ran in full, the refused catalogue pair included.
    let observed = sent(simulator);
    assert_eq!(observed.len(), scripted, "{response}");
    assert_eq!(catalogue_requests(&observed), 4, "{response}");
}

/// A review NAMING the step doubt goes ahead when its first read finishes the
/// masters check as a second doubt (#756): the person named what they review,
/// and the record covers only that doubt. The masters doubt stands for its own
/// review.
#[tokio::test]
async fn a_named_review_goes_ahead_when_its_read_finishes_a_second_doubt() {
    let mut plans = d3_batch_readback();
    plans.extend(d3_catalogue_read(d3_catalogue_with_cash_replaced()));
    plans.extend(d3_batch_readback());
    let scripted = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());
    d3_step_doubt_beside_a_pending_check(&server);
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":D3_GUID,"batch_id":D3_BATCH,"doubt":"batch_step"}),
        approval.clone(),
    )
    .await;
    assert!(
        response["structuredContent"]["result"]["error"].is_null(),
        "{response}"
    );
    assert_eq!(approval.review_counts(), [50], "{response}");
    let imports = server.imports_dir().unwrap();
    let record: Value = serde_json::from_slice(
        &fs::read(imports.join(format!("{D3_BATCH}.batch_step_ack.json"))).unwrap(),
    )
    .unwrap();
    assert_eq!(record["doubt"], "batch_step", "{record}");
    assert!(imports
        .join(format!("{D3_BATCH}.masters_doubt.json"))
        .is_file());
    assert!(!imports
        .join(format!("{D3_BATCH}.masters_ack.json"))
        .exists());
    let observed = sent(simulator);
    assert_eq!(observed.len(), scripted, "{response}");
    assert_eq!(catalogue_requests(&observed), 2, "{response}");
}

/// A review on file answers "already recorded" only for the doubt it covers
/// (#808). Beside a different doubt it is refused as stale, before any
/// request and with no dialog; beside its own doubt, or when the record names
/// no doubt this build can read, it is refused as already recorded.
#[tokio::test]
async fn a_recorded_review_answers_only_for_the_doubt_it_covers() {
    let other =
        r#"{"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Bank"]}"#;
    for (record, code) in [
        (
            json!({"doubt_sha256": crate::agent::sha256_hex(other.as_bytes())}),
            "ack_recorded_review_stale",
        ),
        (
            json!({"doubt_sha256": crate::agent::sha256_hex(DOUBT.as_bytes())}),
            "ack_already_recorded",
        ),
        (json!({}), "ack_already_recorded"),
    ] {
        let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let (server, args) = doubted(&simulator, directory.path());
        let bytes = serde_json::to_vec(&record).unwrap();
        fs::write(ack_path(&server), &bytes).unwrap();
        let approval = ScriptedApproval::approving();
        let response = acknowledge(&server, args, approval.clone()).await;
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"], code,
            "{record}: {response}"
        );
        assert!(approval.reviews().is_empty(), "{record}: no dialog");
        assert_eq!(fs::read(ack_path(&server)).unwrap(), bytes, "{record}");
        assert!(sent(simulator).is_empty(), "{record}: no request");
    }
}

/// The #808 path: a masters review was recorded for an earlier doubt, and
/// the check was later set pending again (a losing second post's mark). The
/// review's own read finishes the check as a new doubt, and the call is
/// refused as stale before the dialog, never as that new doubt's review. The
/// control: a record of that same doubt answers as already recorded.
#[tokio::test]
async fn a_review_of_an_earlier_doubt_does_not_answer_for_the_doubt_a_read_finishes() {
    let earlier =
        r#"{"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Bank"]}"#;
    let (code, finished) =
        review_beside_a_pending_check(&crate::agent::sha256_hex(earlier.as_bytes())).await;
    assert_eq!(code, "ack_recorded_review_stale");
    let (code, again) = review_beside_a_pending_check(&crate::agent::sha256_hex(&finished)).await;
    assert_eq!(
        again, finished,
        "the read finishes the same doubt each time"
    );
    assert_eq!(code, "ack_already_recorded");
}

/// One named masters review of the D3 batch, whose masters check is pending
/// and whose review record covers `doubt_sha256`, while its read finishes the
/// check as a doubt. Returns the refusal code and the doubt the read wrote,
/// after checking that no dialog was shown, the record is unchanged, and
/// every scripted request was sent.
async fn review_beside_a_pending_check(doubt_sha256: &str) -> (String, Vec<u8>) {
    let mut plans = d3_batch_readback();
    plans.extend(d3_catalogue_read(d3_catalogue_with_cash_replaced()));
    let scripted = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = d3_server(&simulator, directory.path());
    d3_step_doubt_beside_a_pending_check(&server);
    let imports = server.imports_dir().unwrap();
    let record_path = imports.join(format!("{D3_BATCH}.masters_ack.json"));
    let record = serde_json::to_vec(&json!({ "doubt_sha256": doubt_sha256 })).unwrap();
    fs::write(&record_path, &record).unwrap();
    let approval = ScriptedApproval::approving();
    let response = acknowledge(
        &server,
        json!({"company_guid":D3_GUID,"batch_id":D3_BATCH,"doubt":"masters"}),
        approval.clone(),
    )
    .await;
    assert!(approval.reviews().is_empty(), "no dialog: {response}");
    assert_eq!(fs::read(&record_path).unwrap(), record);
    // The read did finish the check as a doubt: that is what was compared.
    let finished = fs::read(imports.join(format!("{D3_BATCH}.masters_doubt.json"))).unwrap();
    let observed = sent(simulator);
    assert_eq!(observed.len(), scripted, "{response}");
    assert_eq!(catalogue_requests(&observed), 2, "{response}");
    let code = response["structuredContent"]["result"]["error"]["code"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    (code, finished)
}

/// A batch's step verdict left pending is recorded only by a post, so no read
/// can finish it: a review already on file for it still answers first, before
/// any request, as it did before #808.
#[tokio::test]
async fn a_recorded_step_review_beside_a_pending_step_answers_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = dispatched_batch(&server);
    server
        .record_post_checks_pending(&line.batch_id, true)
        .unwrap();
    let record = serde_json::to_vec(&json!({"doubt_sha256": "0".repeat(64)})).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.batch_step_ack.json", line.batch_id)),
        record,
    )
    .unwrap();
    let response = acknowledge(
        &server,
        json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"batch_step"}),
        ScriptedApproval::approving(),
    )
    .await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ack_already_recorded",
        "{response}"
    );
    assert!(sent(simulator).is_empty());
}

/// A record this build cannot read names no doubt, so no read could change
/// its answer: beside a pending masters check it answers before any request.
#[tokio::test]
async fn an_unreadable_record_beside_a_pending_check_answers_before_any_request() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let (server, args) = seeded(
        &simulator,
        directory.path(),
        clean(),
        Some(br#"{"state":"check_pending"}"#),
        None,
    );
    fs::write(ack_path(&server), b"{}").unwrap();
    let approval = ScriptedApproval::approving();
    let response = acknowledge(&server, args, approval.clone()).await;
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "ack_already_recorded",
        "{response}"
    );
    assert!(approval.reviews().is_empty(), "no dialog");
    assert!(sent(simulator).is_empty(), "no request");
}
