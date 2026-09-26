//! #725 slice 1: a post dialog outlives the call that asked it. Driven through
//! `post_import` against the protocol simulator where a call is involved, and
//! on the held approvals directly where only their rules are. The dialog is
//! the test-only seam, held open until a test answers it.
use super::*;
use crate::agent::agent_import::approval::{ApprovalBinding, Begin, PostApprovals};
use crate::tally::agent_read_request::AgentReadRequest;
use crate::tally::approved_import::{ApprovedImport, PendingPostApproval};

fn result(response: &Value) -> &Value {
    &response["structuredContent"]["result"]
}

fn intents(directory: &std::path::Path) -> usize {
    String::from_utf8(journal(directory))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "dispatch_intent")
        .count()
}

/// A dialog nobody has answered by the end of the call leaves the call
/// `pending`, having sent nothing past its checks. Once the person approves,
/// the next call returns `approved` without reading Tally, and the one after
/// checks the book afresh and posts exactly once.
#[tokio::test]
async fn a_dialog_answered_after_its_call_returned_is_posted_by_a_later_call() {
    let mut plans = before_approval();
    let first_call = plans.len();
    plans.extend(before_approval());
    plans.extend(after_approval(xml(created_one())));
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();

    let pending = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args.clone()))
        .await;
    assert_eq!(result(&pending)["approval"]["state"], "pending", "{pending}");
    assert_eq!(result(&pending)["dispatch"]["state"], "not_dispatched", "{pending}");
    assert_eq!(intents(directory.path()), 0);

    scripted.answer(true);
    let approved = server.call_tool("post_import", args.clone()).await;
    assert_eq!(result(&approved)["approval"]["state"], "approved", "{approved}");
    assert_eq!(intents(directory.path()), 0);

    let posted = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert!(result(&posted)["error"].is_null(), "{posted}");
    assert_eq!(dispatch_intent(directory.path())["batch_id"], line.batch_id.as_str());
    assert_journaled_clean_create(directory.path());
    assert_eq!(scripted.counts(), [1], "one dialog, asked once");
    assert!(observed.len() > first_call);
    assert_eq!(observed.len(), expected);
}

/// A second call while the dialog is open waits on the same dialog: no second
/// dialog is ever shown for one batch, and a decline ends it with no intent.
#[tokio::test]
async fn a_second_call_joins_the_open_dialog_and_a_decline_ends_it() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    for _ in 0..2 {
        let pending = SCRIPTED_APPROVAL
            .scope(scripted.clone(), server.call_tool("post_import", args.clone()))
            .await;
        assert_eq!(result(&pending)["approval"]["state"], "pending", "{pending}");
    }
    scripted.answer(false);
    let declined = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"], "import_approval_declined",
        "{declined}"
    );
    assert_eq!(scripted.counts(), [1]);
    assert_eq!(observed.len(), before_approval().len());
    assert_eq!(intents(directory.path()), 0);
}

/// `saved_batch` under another id: a second batch for the same company.
fn saved_other_batch(server: &Server, saved: &ImportLedgerLine) -> Value {
    let mut line = saved.clone();
    line.batch_id = "bridge-00000000-0000-4000-8000-000000000586".into();
    let rendered = render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(rendered.as_bytes());
    server.append_import_ledger(&line).unwrap();
    fs::write(
        server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        rendered,
    )
    .unwrap();
    json!({"company_guid":GUID,"batch_id":line.batch_id})
}

/// One process holds one batch's dialog or approval at a time. Another batch
/// is refused before any Tally request, and the first is left as it was.
#[tokio::test]
async fn another_batch_is_refused_while_one_waits_for_its_person() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let other = saved_other_batch(&server, &line);
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args.clone()))
        .await;
    assert_eq!(result(&pending)["approval"]["state"], "pending", "{pending}");
    let busy = SCRIPTED_APPROVAL
        .scope(ScriptedApproval::approving(), server.call_tool("post_import", other))
        .await;
    assert_eq!(result(&busy)["error"]["code"], "post_approval_busy", "{busy}");
    let still = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_eq!(result(&still)["approval"]["state"], "pending", "{still}");
    assert_eq!(observed.len(), before_approval().len());
    assert_eq!(scripted.counts(), [1]);
}

/// After a revocation (a cancelled call), a click on the old dialog approves
/// nothing: its task was aborted and its answer is never read. The next call
/// asks again, and a decline there leaves no intent.
#[tokio::test]
async fn a_click_after_revocation_approves_nothing() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_, args) = saved_batch(&server);
    let batch_id = args["batch_id"].as_str().unwrap().to_string();
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args.clone()))
        .await;
    assert_eq!(result(&pending)["approval"]["state"], "pending", "{pending}");
    server.post_approvals.revoke(&batch_id, "request_cancelled");
    scripted.answer(true);
    let asked_again = ScriptedApproval::declining();
    let declined = SCRIPTED_APPROVAL
        .scope(asked_again.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"], "import_approval_declined",
        "{declined}"
    );
    assert_eq!(asked_again.counts(), [1], "the next call asked afresh");
    assert_eq!(intents(directory.path()), 0);
    assert_eq!(observed.len(), 2 * before_approval().len());
}

/// An approval the test seam grants for `vouchers` vouchers, with the native
/// request for `line`.
async fn granted(line: &ImportLedgerLine, vouchers: usize) -> (ApprovedImport, NativePostRequest) {
    let read = || {
        AgentReadRequest::parse(crate::agent::read_profiles::render_agent_company_high_water(
            "WR2 Unicode Lab",
        ))
        .unwrap()
    };
    let binding = bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue(),
        "WR2 Unicode Lab",
        GUID,
    )
    .unwrap()
    .bind_selected(["Cash".to_string()])
    .unwrap();
    let dates = vec![bridge_tally_core::TallyDate::parse("20260901".into()).unwrap(); vouchers];
    let dialog = SCRIPTED_APPROVAL.scope(
        ScriptedApproval::approving(),
        async {
            PendingPostApproval::ask(ApprovedImport::confirm(
                "<ENVELOPE/>".into(),
                "Synthetic preview",
                dates,
                read(),
                read(),
                binding,
                None,
                read(),
                read(),
            ))
        },
    );
    let request = dialog
        .await
        .answer_within(std::time::Duration::from_secs(5))
        .await
        .ok()
        .expect("answered")
        .expect("approved");
    let native = native_post_request(line, RemoteIds::from_ids(vec![Uuid::new_v4(); 1]))
        .unwrap();
    (request, native)
}

fn binding_of(line: &ImportLedgerLine, preview: &str) -> ApprovalBinding {
    let binding = bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue(),
        "WR2 Unicode Lab",
        GUID,
    )
    .unwrap()
    .bind_selected(requested_ledger_names(&ImportPayload {
        company_guid: line.company_guid.clone(),
        vouchers: line.vouchers.clone(),
        amends_batch_id: None,
    }))
    .unwrap();
    ApprovalBinding::new(line, preview, &binding)
}

fn held_line(directory: &std::path::Path) -> (Server, ImportLedgerLine) {
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory);
    let (line, _) = saved_batch(&server);
    (server, line)
}

/// Redeemed only for what the person was shown: another preview, or a count
/// other than the dialog's (#746's), is refused, and the approval lapses with
/// a note that is not an approval.
#[tokio::test]
async fn a_redemption_for_anything_else_is_refused_and_lapses() {
    for (preview, vouchers) in [("Another preview", 1), ("Synthetic preview", 2)] {
        let directory = tempfile::tempdir().unwrap();
        let (server, line) = held_line(directory.path());
        let approvals = PostApprovals::new(directory.path());
        let (request, native) = granted(&line, vouchers).await;
        approvals
            .hold_approved(&line.batch_id, binding_of(&line, "Synthetic preview"), request, native)
            .unwrap();
        assert_eq!(
            approvals
                .take_for_dispatch(&line.batch_id, &binding_of(&line, preview))
                .err()
                .as_deref(),
            Some("import_approval_binding_changed")
        );
        let note = approvals.lapse_note(&line.batch_id).unwrap();
        assert_eq!(note["reason"], "approval_binding_changed");
        assert_eq!(note["redeemable"], false);
        assert!(matches!(
            approvals.begin(&line.batch_id),
            Begin::Ask
        ));
        drop(server);
    }
}

/// Spent once, under the lock, before an intent: a second spend, a second
/// redemption, and a spend after a revocation are all refused.
#[tokio::test]
async fn an_approval_is_spent_once_and_never_after_a_revocation() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let binding = binding_of(&line, "Synthetic preview");
    let (request, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding.clone(), request, native)
        .unwrap();
    let (id, _, _) = approvals.take_for_dispatch(&line.batch_id, &binding).unwrap();
    assert!(approvals.take_for_dispatch(&line.batch_id, &binding).is_err());
    approvals.spend(&line.batch_id, id).unwrap();
    assert!(approvals.spend(&line.batch_id, id).is_err());

    let (request, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding.clone(), request, native)
        .unwrap();
    let (id, _, _) = approvals.take_for_dispatch(&line.batch_id, &binding).unwrap();
    approvals.revoke(&line.batch_id, "request_cancelled");
    assert_eq!(
        approvals.spend(&line.batch_id, id).err().as_deref(),
        Some("import_approval_revoked")
    );
    assert_eq!(
        approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "request_cancelled"
    );
}

/// An approval not redeemed in time lapses, leaving a note, and the batch is
/// asked about afresh.
#[tokio::test]
async fn an_expired_approval_lapses_with_a_note() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::with_ttl(directory.path(), std::time::Duration::from_millis(1));
    let (request, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding_of(&line, "Synthetic preview"), request, native)
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(5));
    assert!(matches!(
        approvals.begin(&line.batch_id),
        Begin::Ask
    ));
    let note = approvals.lapse_note(&line.batch_id).unwrap();
    assert_eq!(note["reason"], "approval_expired");
    assert_eq!(note["state"], "approval_lapsed_unposted");
}

/// A process that ends holds nothing a new one can redeem: its approval
/// leaves only a note, which the next process reports and never posts from.
#[tokio::test]
async fn an_approval_does_not_survive_its_process() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let (request, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding_of(&line, "Synthetic preview"), request, native)
        .unwrap();
    drop(approvals);
    let restarted = PostApprovals::new(directory.path());
    assert!(matches!(
        restarted.begin(&line.batch_id),
        Begin::Ask
    ));
    let note = restarted.lapse_note(&line.batch_id).unwrap();
    assert_eq!(note["reason"], "process_ended");
    assert_eq!(note["redeemable"], false);
    assert!(restarted
        .take_for_dispatch(&line.batch_id, &binding_of(&line, "Synthetic preview"))
        .is_err());
}

/// Posted in the answering call only while the measured post still fits under
/// the ceiling (#725): at the boundary it does, a millisecond later it does not,
/// and a batch larger than any measured never does.
#[test]
fn an_approval_is_posted_in_its_call_only_while_the_measured_post_fits() {
    use crate::agent::agent_import::approval::{dispatch_fits_in_call, CALL_CEILING};
    let measured = std::time::Duration::from_millis(20_950);
    let boundary = CALL_CEILING - measured;
    assert!(dispatch_fits_in_call(std::time::Duration::ZERO, 1));
    assert!(dispatch_fits_in_call(boundary, 50));
    assert!(!dispatch_fits_in_call(
        boundary + std::time::Duration::from_millis(1),
        50
    ));
    assert!(!dispatch_fits_in_call(std::time::Duration::ZERO, 201));
    assert!(dispatch_fits_in_call(std::time::Duration::ZERO, 200));
    assert!(!dispatch_fits_in_call(std::time::Duration::MAX, 1));
}
