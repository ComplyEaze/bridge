//! #725 slice 1: a post dialog outlives the call that asked it. Driven through
//! `post_import` (and, for cancellation, through the stdio path every agent
//! post takes) against the protocol simulator where a call is involved, and on
//! the held approvals directly where only their rules are. The dialog is the
//! test-only seam, held open until a test answers it.
use super::*;
use crate::agent::agent_import::approval::{
    ApprovalBinding, Begin, Joined, PostApprovals, CALL_CEILING, MAX_KEPT_REFUSALS, MEASURED_POST,
    MEASURED_REDEEM, MEASURED_REDEEM_VOUCHERS,
};
use crate::agent::agent_protocol::{run_post, Framer};
use crate::agent::ToolResponse;
use crate::tally::agent_read_request::AgentReadRequest;
use crate::tally::approved_import::{Answered, ApprovedImport, PendingPostApproval};
use tokio::io::{AsyncWriteExt, BufReader};

/// Bounds a wait on an event, only so that a hang fails instead of stalling
/// the run: nothing here is paced by it.
const HANG_GUARD: Duration = Duration::from_secs(120);

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

const CANCEL_7: &[u8] =
    b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/cancelled\",\"params\":{\"requestId\":7}}\n";

/// The two calls after a pending one, in both orders of click and call: `plans`
/// serves the first call's checks, then the posting call's fresh checks and
/// its dispatch up to the POST, whose index is returned.
fn pending_then_posted_plans() -> (Vec<ScenarioPlan>, usize) {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    (plans, post_at)
}

/// The first call of a held dialog, left `pending` with nothing sent past its
/// checks and the dialog held for the next call.
async fn first_call_pending(
    server: &Server,
    directory: &std::path::Path,
    line: &ImportLedgerLine,
    args: &Value,
    scripted: &ScriptedApproval,
) {
    let pending = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    assert_eq!(
        result(&pending)["dispatch"]["state"],
        "not_dispatched",
        "{pending}"
    );
    assert_eq!(result(&pending)["attempt_recorded"], false, "{pending}");
    assert_eq!(intents(directory), 0);
    assert!(
        server.post_approvals.holds(&line.batch_id),
        "the open dialog is held for the next call"
    );
}

/// What a posting call must leave: one intent for this batch, its clean
/// create, one dialog asked once, and the POST on the wire. The simulator's
/// script ends at the POST, so the readback after it is not served.
fn assert_posted_once(
    posted: &Value,
    directory: &std::path::Path,
    line: &ImportLedgerLine,
    scripted: &ScriptedApproval,
    observed: &[tally_protocol_simulator::ObservedRequest],
    post_at: usize,
) {
    assert_eq!(result(posted)["attempt_recorded"], true, "{posted}");
    assert_eq!(
        dispatch_intent(directory)["batch_id"],
        line.batch_id.as_str()
    );
    assert_journaled_clean_create(directory);
    assert_eq!(scripted.counts(), [1], "one dialog, asked once");
    assert!(observed.len() > post_at, "the POST was sent: {posted}");
}

/// A click that lands while no call waits is posted by the very next call
/// (#725 slice 2.0): that call checks the book afresh and posts exactly once.
/// Before, it only reported `approved`, and an agent that stopped there left
/// the batch unposted (L1-a, 28 Sep 2026: two calls after the click).
#[tokio::test]
async fn a_click_between_calls_is_posted_by_the_next_call() {
    let (plans, post_at) = pending_then_posted_plans();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    first_call_pending(&server, directory.path(), &line, &args, &scripted).await;

    scripted.answer(true);
    until_answered(&server, &line.batch_id).await;
    let posted = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_posted_once(
        &posted,
        directory.path(),
        &line,
        &scripted,
        &observed,
        post_at,
    );
}

/// Answered only once a second call has taken the held dialog to wait on it,
/// so the click lands inside that call's wait and never before it.
async fn click_while_joined(server: &Server, batch_id: &str, scripted: &ScriptedApproval) {
    tokio::time::timeout(Duration::from_secs(5), async {
        while !server.post_approvals.joined_for_test(batch_id) {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("the second call joined the held dialog");
    scripted.answer(true);
}

/// A click that lands while a later call waits on the dialog is reported by
/// that call, not posted: posting after a wait could run past the host's
/// timeout on a large book (#852). The report says to call again now, and
/// that call checks the book afresh and posts (#725 slice 2.0).
#[tokio::test]
async fn a_click_during_a_joining_call_is_reported_and_posted_by_the_next() {
    let (plans, post_at) = pending_then_posted_plans();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    first_call_pending(&server, directory.path(), &line, &args, &scripted).await;

    let (approved, ()) = tokio::join!(
        server.call_tool("post_import", args.clone()),
        click_while_joined(&server, &line.batch_id, &scripted)
    );
    assert_eq!(
        result(&approved)["approval"]["state"],
        "approved",
        "{approved}"
    );
    assert_eq!(
        result(&approved)["approval"]["retry_after_s"],
        0,
        "{approved}"
    );
    assert!(
        result(&approved)["approval"]["next_step"].is_string(),
        "{approved}"
    );
    assert_eq!(intents(directory.path()), 0);

    let posted = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_posted_once(
        &posted,
        directory.path(),
        &line,
        &scripted,
        &observed,
        post_at,
    );
}

/// Answered in its call, but too late for the measured post to fit: the call
/// returns `approved` and sends nothing past its checks; the next call checks
/// again and posts (#725, the in-call rule through a real call).
#[tokio::test]
async fn an_answer_too_late_for_its_call_is_posted_by_the_next() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    // A post measured at the whole ceiling never fits in the answering call.
    server.post_approvals = std::sync::Arc::new(PostApprovals::with_measured_post(
        directory.path(),
        CALL_CEILING,
    ));
    let (_, args) = saved_batch(&server);
    let approved = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&approved)["approval"]["state"],
        "approved",
        "{approved}"
    );
    assert_eq!(intents(directory.path()), 0);
    let posted = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_eq!(result(&posted)["attempt_recorded"], true, "{posted}");
    assert_journaled_clean_create(directory.path());
    assert!(observed.len() > post_at, "{posted}");
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
            .scope(
                scripted.clone(),
                server.call_tool("post_import", args.clone()),
            )
            .await;
        assert_eq!(
            result(&pending)["approval"]["state"],
            "pending",
            "{pending}"
        );
    }
    scripted.answer(false);
    let declined = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"],
        "import_approval_declined",
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
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    let busy = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", other),
        )
        .await;
    assert_eq!(
        result(&busy)["error"]["code"],
        "post_approval_busy",
        "{busy}"
    );
    let still = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_eq!(result(&still)["approval"]["state"], "pending", "{still}");
    assert_eq!(observed.len(), before_approval().len());
    assert_eq!(scripted.counts(), [1]);
}

/// Wait until a held dialog's answer has been stamped by its task. Bounded: a
/// dialog that is no longer held is never stamped, which fails the test.
async fn until_answered(server: &Server, batch_id: &str) {
    tokio::time::timeout(Duration::from_secs(5), async {
        while !server.post_approvals.answered_for_test(batch_id) {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("the held dialog's answer was stamped");
}

/// A No given while no call waits is not lost (#725): the same batch's next
/// call returns `import_approval_declined`, sends nothing past the first call's
/// checks, and does not ask the person again.
#[tokio::test]
async fn a_decline_between_calls_is_returned_to_the_next_call_not_asked_again() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    scripted.answer(false);
    until_answered(&server, &line.batch_id).await;
    let declined = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"],
        "import_approval_declined",
        "{declined}"
    );
    assert_eq!(scripted.counts(), [1], "the person was not asked again");
    assert_eq!(observed.len(), before_approval().len());
    assert_eq!(intents(directory.path()), 0);
    assert!(!server.post_approvals.holds(&line.batch_id));
}

/// Two batches refused between calls each read their own No (#725): the
/// second batch's refusal does not replace the first's, and neither person is
/// asked again.
#[tokio::test]
async fn two_batches_refused_between_calls_each_read_their_own_no() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, first_args) = saved_batch(&server);
    let second_args = saved_other_batch(&server, &line);
    let second_batch = second_args["batch_id"].as_str().unwrap().to_string();
    let first = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            first.clone(),
            server.call_tool("post_import", first_args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    first.answer(false);
    until_answered(&server, &line.batch_id).await;
    let second = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            second.clone(),
            server.call_tool("post_import", second_args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    second.answer(false);
    until_answered(&server, &second_batch).await;
    let first_again = SCRIPTED_APPROVAL
        .scope(first.clone(), server.call_tool("post_import", first_args))
        .await;
    let second_again = SCRIPTED_APPROVAL
        .scope(second.clone(), server.call_tool("post_import", second_args))
        .await;
    let observed = sent(simulator);
    for (again, scripted) in [(&first_again, &first), (&second_again, &second)] {
        assert_eq!(
            result(again)["error"]["code"],
            "import_approval_declined",
            "{again}"
        );
        assert_eq!(scripted.counts(), [1], "not asked again");
    }
    assert_eq!(observed.len(), 2 * before_approval().len());
    assert_eq!(intents(directory.path()), 0);
}

/// A dialog declined while no call waited no longer blocks other batches: the
/// next call, for another batch, settles it and asks about its own. The
/// declined batch's own next call still reads its No, and is not asked again.
#[tokio::test]
async fn a_dialog_declined_while_nobody_waits_does_not_block_another_batch() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let other = saved_other_batch(&server, &line);
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    scripted.answer(false);
    until_answered(&server, &line.batch_id).await;
    let asked = ScriptedApproval::declining();
    let declined = SCRIPTED_APPROVAL
        .scope(asked.clone(), server.call_tool("post_import", other))
        .await;
    let again = SCRIPTED_APPROVAL
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"],
        "import_approval_declined",
        "the other batch was asked, not refused as busy: {declined}"
    );
    assert_eq!(asked.counts(), [1]);
    assert_eq!(
        result(&again)["error"]["code"],
        "import_approval_declined",
        "the declined batch read its No: {again}"
    );
    assert_eq!(scripted.counts(), [1], "and was not asked again");
    assert_eq!(observed.len(), 2 * before_approval().len());
}

/// An approval's time runs from the click, not from whichever call collects
/// it: one clicked and left past its time lapses with a note, and the next
/// call asks the person again.
#[tokio::test]
async fn an_approval_left_past_its_time_is_asked_again() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    server.post_approvals = std::sync::Arc::new(PostApprovals::with_ttl(
        directory.path(),
        Duration::from_millis(200),
    ));
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    scripted.answer(true);
    until_answered(&server, &line.batch_id).await;
    tokio::time::sleep(Duration::from_millis(400)).await;
    let asked_again = ScriptedApproval::declining();
    let declined = SCRIPTED_APPROVAL
        .scope(asked_again.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"],
        "import_approval_declined",
        "the old click was not used: {declined}"
    );
    assert_eq!(asked_again.counts(), [1]);
    assert_eq!(intents(directory.path()), 0);
    assert_eq!(
        server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "approval_expired"
    );
    assert_eq!(observed.len(), 2 * before_approval().len());
}

/// After a revocation (a cancelled call), a click on the old dialog approves
/// nothing: its task was aborted, which closed it, and its answer is never
/// read. The next call asks again, and a decline there leaves no intent.
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
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    assert!(scripted.is_waiting(), "the dialog is open");
    server.post_approvals.revoke(&batch_id, "request_cancelled");
    tokio::task::yield_now().await;
    assert!(!scripted.is_waiting(), "revoking closed the dialog");
    scripted.answer(true);
    let asked_again = ScriptedApproval::declining();
    let declined = SCRIPTED_APPROVAL
        .scope(asked_again.clone(), server.call_tool("post_import", args))
        .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"],
        "import_approval_declined",
        "{declined}"
    );
    assert_eq!(asked_again.counts(), [1], "the next call asked afresh");
    assert_eq!(intents(directory.path()), 0);
    assert_eq!(observed.len(), 2 * before_approval().len());
}

/// A call withdrawn while it waits on the dialog stops waiting at once,
/// answers `request_cancelled` rather than `pending`, and closes the dialog it
/// had started.
// current_thread, which `closed()` relies on: the dialog's task finishes in
// the poll that closes it, before this task runs again.
#[tokio::test(flavor = "current_thread")]
async fn a_call_withdrawn_while_it_waits_closes_its_dialog() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    let withdrawn = tokio_util::sync::CancellationToken::new();
    let post = crate::tally::runtime::TOOL_CANCELLATION.scope(
        withdrawn.clone(),
        SCRIPTED_APPROVAL.scope(scripted.clone(), server.call_tool("post_import", args)),
    );
    let cancel = async {
        scripted.opened().await;
        withdrawn.cancel();
    };
    // A call that ends before its dialog opens leaves `cancel` waiting; the
    // bound only stops a hang, since every wait here is on an event.
    let (response, ()) = tokio::time::timeout(HANG_GUARD, async { tokio::join!(post, cancel) })
        .await
        .expect("the withdrawn call stops");
    let observed = sent(simulator);
    assert_eq!(
        result(&response)["error"]["code"],
        "request_cancelled",
        "{response}"
    );
    tokio::task::yield_now().await;
    assert!(!scripted.is_waiting(), "the dialog was closed");
    assert!(!server.post_approvals.holds(&line.batch_id));
    assert_eq!(observed.len(), before_approval().len());
    assert_eq!(intents(directory.path()), 0);
}

/// An answer and a withdrawal ready together, in a call where an approval
/// would be kept for the next call: the withdrawal wins, and nothing is kept
/// (#725, the biased wait and the held-under-lock refusal). The test drives the
/// call itself, so the answer and the cancellation are both ready when it is
/// next polled; repeated, since the pick between two ready arms is what is
/// under test.
// current_thread, which `closed()` relies on: the dialog's task finishes in
// the poll that closes it, before this task runs again.
#[tokio::test(flavor = "current_thread")]
async fn an_answer_arriving_with_the_cancellation_is_not_kept() {
    for _ in 0..16 {
        let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let mut server = server_at(simulator.address(), directory.path());
        // Never fits: an answer would be held for the next call, not posted.
        server.post_approvals = std::sync::Arc::new(PostApprovals::with_measured_post(
            directory.path(),
            CALL_CEILING,
        ));
        let (line, args) = saved_batch(&server);
        let scripted = ScriptedApproval::held();
        let withdrawn = tokio_util::sync::CancellationToken::new();
        let mut post = std::pin::pin!(crate::tally::runtime::TOOL_CANCELLATION.scope(
            withdrawn.clone(),
            SCRIPTED_APPROVAL.scope(scripted.clone(), server.call_tool("post_import", args)),
        ));
        // Every wait is on an event, so a slow runner only slows the test;
        // the bound stops a hang.
        tokio::time::timeout(HANG_GUARD, async {
            // Drive the call until its dialog is open.
            tokio::select! {
                biased;
                response = &mut post => panic!("the call ended early: {response}"),
                () = scripted.opened() => {}
            }
            // Answer, and let the dialog's task take it, without polling the
            // call. On this current-thread runtime the task drops its end of
            // the answer and runs to its end in one poll, so once `closed`
            // resolves its answer is ready for the call's next poll.
            scripted.answer(true);
            scripted.closed().await;
        })
        .await
        .expect("the dialog opened and its task took the answer");
        withdrawn.cancel();
        let response = post.await;
        let _ = sent(simulator);
        assert_eq!(
            result(&response)["error"]["code"],
            "request_cancelled",
            "{response}"
        );
        assert!(!server.post_approvals.holds(&line.batch_id), "nothing kept");
        assert_eq!(intents(directory.path()), 0);
    }
}

/// The wait itself: with an answer and a withdrawal both ready, the withdrawal
/// is taken, every time.
#[tokio::test]
async fn the_wait_takes_a_ready_withdrawal_over_a_ready_answer() {
    for _ in 0..32 {
        let dialog = approved_dialog(1).await;
        tokio::time::timeout(Duration::from_secs(5), async {
            while dialog.answered().is_none() {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("the approving dialog was answered");
        let withdrawn = tokio_util::sync::CancellationToken::new();
        withdrawn.cancel();
        let waited = crate::tally::runtime::TOOL_CANCELLATION
            .scope(withdrawn, wait_for_answer(dialog, Duration::from_secs(5)))
            .await;
        assert!(matches!(waited, Waited::Cancelled));
    }
}

/// The wait stops as soon as its call is withdrawn, well inside its budget,
/// and the dialog it held is closed.
#[tokio::test]
async fn the_wait_stops_at_a_withdrawal() {
    let scripted = ScriptedApproval::held();
    let dialog = SCRIPTED_APPROVAL
        .scope(scripted.clone(), async {
            PendingPostApproval::ask(
                "<ENVELOPE/>".into(),
                "Synthetic preview".into(),
                vec![bridge_tally_core::TallyDate::parse("20260901").unwrap()],
                high_water_read(),
                high_water_read(),
                cash_binding(),
                None,
                high_water_read(),
                high_water_read(),
            )
        })
        .await;
    let withdrawn = tokio_util::sync::CancellationToken::new();
    let cancel = {
        let withdrawn = withdrawn.clone();
        async move {
            tokio::time::sleep(Duration::from_millis(20)).await;
            withdrawn.cancel();
        }
    };
    let wait = crate::tally::runtime::TOOL_CANCELLATION.scope(
        withdrawn.clone(),
        wait_for_answer(dialog, Duration::from_secs(30)),
    );
    let (waited, ()) =
        tokio::time::timeout(Duration::from_secs(2), async { tokio::join!(wait, cancel) })
            .await
            .expect("stopped at the withdrawal, not at the budget");
    assert!(matches!(waited, Waited::Cancelled));
    tokio::task::yield_now().await;
    assert!(!scripted.is_waiting(), "the dialog was closed");
}

/// Holding a dialog or an approval is refused, and nothing held, once the
/// asking call has been withdrawn.
#[tokio::test]
async fn nothing_is_held_for_a_withdrawn_call() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let binding = binding_of(&line, "Synthetic preview");
    let withdrawn = tokio_util::sync::CancellationToken::new();
    withdrawn.cancel();
    let dialog = approved_dialog(1).await;
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    let held = crate::tally::runtime::TOOL_CANCELLATION
        .scope(withdrawn.clone(), async {
            approvals.hold_pending(&line.batch_id, binding.clone(), dialog, native)
        })
        .await;
    assert_eq!(held.err().as_deref(), Some("request_cancelled"));
    assert!(matches!(approvals.begin(&line.batch_id), Begin::Ask));
    let (request, answered, native) = granted(&line, 1).await;
    let held = crate::tally::runtime::TOOL_CANCELLATION
        .scope(withdrawn, async {
            approvals.hold_approved(&line.batch_id, binding, request, native, answered)
        })
        .await;
    assert_eq!(held.err().as_deref(), Some("request_cancelled"));
    assert!(matches!(approvals.begin(&line.batch_id), Begin::Ask));
}

/// A held dialog whose task ended with no answer (it panicked or was aborted)
/// can never be answered, so it is let go: another batch is asked, not refused
/// as busy.
#[tokio::test]
async fn a_dialog_that_ended_unanswered_does_not_block_another_batch() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let dialog = dialog_with(
        ScriptedApproval::approving_after(|| panic!("the dialog ended without an answer")),
        1,
    )
    .await;
    tokio::time::timeout(Duration::from_secs(5), async {
        while dialog.refusal().is_none() {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("the dialog's task ended");
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    approvals
        .hold_pending(
            &line.batch_id,
            binding_of(&line, "Synthetic preview"),
            dialog,
            native,
        )
        .unwrap();
    assert!(matches!(
        approvals.begin("bridge-00000000-0000-4000-8000-000000000586"),
        Begin::Ask
    ));
    assert!(!approvals.holds(&line.batch_id));
    assert!(
        approvals.lapse_note(&line.batch_id).is_none(),
        "an unanswered dialog leaves no note to read as approved"
    );
    // Its own batch's next call reads why, and is not asked again.
    assert!(matches!(
        approvals.begin(&line.batch_id),
        Begin::Refused(code) if code == "import_approval_unavailable"
    ));
}

/// A batch id no test keeps a refusal for.
const OTHER: &str = "bridge-00000000-0000-4000-8000-000000000999";

/// Hold, for `batch_id`, a dialog the person has already declined.
async fn hold_declined(approvals: &PostApprovals, line: &ImportLedgerLine, batch_id: &str) {
    let dialog = dialog_with(ScriptedApproval::declining(), 1).await;
    tokio::time::timeout(Duration::from_secs(5), async {
        while dialog.refusal().is_none() {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("the dialog was declined");
    let native = native_post_request(line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    approvals
        .hold_pending(
            batch_id,
            binding_of(line, "Synthetic preview"),
            dialog,
            native,
        )
        .unwrap();
}

/// A refusal no call has read is returned once, to its own batch's next call,
/// and a withdrawal of that batch drops it. It never holds the slot: another
/// batch is asked throughout, and nothing is left held after the withdrawal.
#[tokio::test]
async fn a_kept_refusal_is_returned_once_and_a_withdrawal_drops_it() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    hold_declined(&approvals, &line, &line.batch_id).await;
    assert!(
        matches!(approvals.begin(OTHER), Begin::Ask),
        "it blocks no other batch"
    );
    assert!(matches!(
        approvals.begin(&line.batch_id),
        Begin::Refused(code) if code == "import_approval_declined"
    ));
    assert!(
        matches!(approvals.begin(&line.batch_id), Begin::Ask),
        "returned once"
    );

    hold_declined(&approvals, &line, &line.batch_id).await;
    assert!(matches!(approvals.begin(OTHER), Begin::Ask));
    approvals.revoke(&line.batch_id, "request_cancelled");
    assert!(
        matches!(approvals.begin(&line.batch_id), Begin::Ask),
        "a withdrawal drops it"
    );
    assert!(!approvals.holds(&line.batch_id));
    assert!(
        matches!(approvals.begin(OTHER), Begin::Ask),
        "nothing is left held"
    );
}

/// Kept refusals age out with the approval limit, and past the cap the oldest
/// is dropped: that batch is asked again, and the others still read their No.
#[tokio::test]
async fn kept_refusals_age_out_and_the_oldest_goes_past_the_cap() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::with_ttl(directory.path(), Duration::from_millis(100));
    // Kept while fresh, then aged out.
    hold_declined(&approvals, &line, &line.batch_id).await;
    assert!(matches!(approvals.begin(OTHER), Begin::Ask));
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(
        matches!(approvals.begin(&line.batch_id), Begin::Ask),
        "aged out"
    );
    // The age runs from the click, not from the later call that notices it.
    hold_declined(&approvals, &line, &line.batch_id).await;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(
        matches!(approvals.begin(&line.batch_id), Begin::Ask),
        "aged from the click"
    );
    assert!(!approvals.holds(&line.batch_id));

    let approvals = PostApprovals::new(directory.path());
    let batch = |i: usize| format!("bridge-00000000-0000-4000-8000-{i:012}");
    for i in 0..=MAX_KEPT_REFUSALS {
        hold_declined(&approvals, &line, &batch(i)).await;
        assert!(matches!(approvals.begin(OTHER), Begin::Ask));
    }
    assert!(
        matches!(approvals.begin(&batch(0)), Begin::Ask),
        "the oldest was dropped"
    );
    for kept in [batch(1), batch(MAX_KEPT_REFUSALS)] {
        assert!(matches!(
            approvals.begin(&kept),
            Begin::Refused(code) if code == "import_approval_declined"
        ));
    }
}

/// An approval collected by a joined call too late for its post to fit reports
/// what is left of it, counted from the click, not the whole window, and tells
/// the agent to call again at once.
#[tokio::test]
async fn an_approval_reports_the_time_left_since_its_click() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    // A redeem measured at the whole ceiling never fits in the joining call,
    // so that call reports the approval instead of posting it.
    server.post_approvals = std::sync::Arc::new(
        PostApprovals::with_ttl(directory.path(), Duration::from_secs(3))
            .with_measured_redeem(CALL_CEILING),
    );
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    scripted.answer(true);
    until_answered(&server, &line.batch_id).await;
    tokio::time::sleep(Duration::from_millis(1_600)).await;
    let approved = server.call_tool("post_import", args).await;
    let _ = sent(simulator);
    assert_eq!(
        result(&approved)["approval"]["state"],
        "approved",
        "{approved}"
    );
    let left = result(&approved)["approval"]["expires_in_s"]
        .as_u64()
        .unwrap();
    assert!(left <= 1, "counted from the click: {approved}");
    assert_eq!(
        result(&approved)["approval"]["retry_after_s"],
        0,
        "{approved}"
    );
    assert!(
        result(&approved)["approval"]["next_step"]
            .as_str()
            .is_some_and(|step| step.contains("post_import") && step.contains("now")),
        "an approved batch says to call again now: {approved}"
    );
    assert_eq!(intents(directory.path()), 0);
}

/// A batch posted by any other route releases whatever was held for it, so it
/// no longer keeps other batches waiting.
#[tokio::test]
async fn a_batch_posted_elsewhere_releases_its_hold() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
            .unwrap();
    }
    let _verified = server.call_tool("post_import", args).await;
    let _ = sent(simulator);
    assert!(!server.post_approvals.holds(&line.batch_id));
    assert!(!scripted.is_waiting(), "its dialog was closed");
}

/// `post_import` over the stdio path every agent post takes, cancelled once
/// `simulator` has received `after` requests when that is given.
async fn post_over_stdio(
    server: &Server,
    args: &Value,
    cancel: Option<(&SequenceSimulator, usize)>,
) -> Option<ToolResponse> {
    let (client, source) = tokio::io::duplex(1 << 16);
    let (_client_read, mut client_write) = tokio::io::split(client);
    let (source_read, mut source_write) = tokio::io::split(source);
    let mut reader = BufReader::new(source_read);
    let post = SCRIPTED_APPROVAL.scope(ScriptedApproval::approving(), async {
        run_post(
            server,
            &json!(7),
            args,
            &mut reader,
            &mut Framer::default(),
            &mut std::collections::VecDeque::new(),
            &mut source_write,
        )
        .await
    });
    let canceller = async {
        if let Some((simulator, after)) = cancel {
            while simulator.received() <= after {
                tokio::time::sleep(Duration::from_millis(2)).await;
            }
            client_write.write_all(CANCEL_7).await.unwrap();
        }
    };
    let (outcome, ()) = tokio::time::timeout(Duration::from_secs(20), async {
        tokio::join!(post, canceller)
    })
    .await
    .unwrap();
    outcome.unwrap()
}

/// A cancellation that lands inside the queue's lease operation, through the
/// stdio path every agent post takes (#725): that operation finishes its reads,
/// every one served in full, with no cap cutting it short, and then, finding
/// its approval revoked, writes no intent and sends no POST. The control, the
/// same run never cancelled, posts: so the cancellation is what stopped it.
#[tokio::test]
async fn a_cancel_inside_the_lease_finishes_its_reads_and_posts_nothing() {
    for cancelled in [true, false] {
        let mut plans = before_approval();
        let lease_start = plans.len();
        let mut lease = after_approval(xml(created_one()));
        // The lease opens with a probe (status, company list), the company
        // list, and the marks at binding; then the ledger catalogue's pair,
        // bracketed by the company list. Hold its first read.
        let held_at = 5;
        lease[held_at] =
            xml(catalogue()).with_delivery(Delivery::SlowHeaders(Duration::from_millis(400)));
        let post_at = lease_start + lease.len() - 1;
        plans.extend(lease);
        let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_at(simulator.address(), directory.path());
        let (line, args) = saved_batch(&server);
        let outcome = post_over_stdio(
            &server,
            &args,
            cancelled.then_some((&simulator, lease_start + held_at)),
        )
        .await;
        let observed = sent(simulator);
        if cancelled {
            assert!(outcome.is_none(), "answered as cancelled");
            assert_eq!(intents(directory.path()), 0);
            assert_eq!(observed.len(), post_at, "every lease read, and no POST");
            assert!(
                observed.iter().all(|request| request.request_processed
                    && !request.cancelled
                    && !request.client_stopped_reading_response),
                "every started request was served in full, the held one too"
            );
            assert_eq!(
                server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
                "request_cancelled"
            );
        } else {
            assert!(outcome.is_some());
            assert_eq!(intents(directory.path()), 1, "the control posts");
            assert!(observed.len() > post_at);
        }
    }
}

/// A refusal in the call redeeming an approval withdraws it: the next call
/// asks the person again rather than posting on the old click. Since slice
/// 2.0 the redeeming call is the first one after the click.
#[tokio::test]
async fn a_refused_redemption_withdraws_its_approval() {
    let mut plans = before_approval();
    plans.extend(before_approval_with_currencies(two_currencies()));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    let pending = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.call_tool("post_import", args.clone()),
        )
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    scripted.answer(true);
    until_answered(&server, &line.batch_id).await;
    let refused = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    assert_eq!(
        result(&refused)["error"]["code"],
        "import_multi_currency_unsupported",
        "{refused}"
    );
    assert!(!server.post_approvals.holds(&line.batch_id));
    assert_eq!(
        server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "post_refused_before_intent"
    );
    assert_eq!(intents(directory.path()), 0);
    // The redeeming call stops at the currency refusal: its last request is
    // the currency pair's closing company list (a POST), and the closing mode
    // probe its checks would otherwise end with (a GET /status, then the
    // company list) is never sent.
    assert_eq!(observed.len(), 2 * before_approval().len() - probe().len());
    let last = observed.last().unwrap();
    assert_eq!(last.method, "POST", "the currency pair's closing read");
    let (first_call, redeeming) = observed.split_at(before_approval().len());
    let gets = |requests: &[tally_protocol_simulator::ObservedRequest]| {
        requests
            .iter()
            .filter(|request| request.method == "GET")
            .count()
    };
    assert_eq!(
        gets(redeeming) + 1,
        gets(first_call),
        "exactly the mode probe's GET /status is missing"
    );
}

/// A redemption that ends before its intent, here because another holder has
/// the endpoint's dispatch lease, is not left taken: the approval lapses, and
/// the batch is neither in use nor blocking another post.
#[tokio::test]
async fn a_redemption_ended_before_its_intent_is_not_left_taken() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let _other_holder = crate::endpoint_coordination::acquire(&server.settings.endpoint).unwrap();
    let refused = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&refused)["error"]["code"],
        "import_admission_busy",
        "{refused}"
    );
    assert_eq!(
        observed.len(),
        before_approval().len(),
        "nothing past the checks"
    );
    assert_eq!(intents(directory.path()), 0);
    assert!(!server.post_approvals.holds(&line.batch_id), "not stranded");
    assert!(matches!(
        server.post_approvals.begin(&line.batch_id),
        Begin::Ask
    ));
    assert_eq!(
        server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "post_refused_before_intent"
    );
}

fn high_water_read() -> AgentReadRequest {
    AgentReadRequest::parse(
        crate::agent::read_profiles::render_agent_company_high_water("WR2 Unicode Lab"),
    )
    .unwrap()
}

fn cash_binding() -> bridge_tally_protocol::StandardLedgerCatalogBinding {
    bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue(),
        "WR2 Unicode Lab",
        GUID,
    )
    .unwrap()
    .bind_selected(["Cash".to_string()])
    .unwrap()
}

/// A dialog the test seam answers with approval, for `vouchers` vouchers.
async fn approved_dialog(vouchers: usize) -> PendingPostApproval {
    dialog_with(ScriptedApproval::approving(), vouchers).await
}

/// A dialog for `vouchers` vouchers, answered as `scripted` decides.
async fn dialog_with(scripted: ScriptedApproval, vouchers: usize) -> PendingPostApproval {
    SCRIPTED_APPROVAL
        .scope(scripted, async {
            PendingPostApproval::ask(
                "<ENVELOPE/>".into(),
                "Synthetic preview".into(),
                vec![bridge_tally_core::TallyDate::parse("20260901").unwrap(); vouchers],
                high_water_read(),
                high_water_read(),
                cash_binding(),
                None,
                high_water_read(),
                high_water_read(),
            )
        })
        .await
}

/// An approval the test seam grants for `vouchers` vouchers, with when it was
/// given and the native request for `line`.
async fn granted(
    line: &ImportLedgerLine,
    vouchers: usize,
) -> (ApprovedImport, Answered, NativePostRequest) {
    let (answer, answered) = approved_dialog(vouchers)
        .await
        .answer_within(Duration::from_secs(5))
        .await
        .ok()
        .expect("answered");
    let native = native_post_request(line, RemoteIds::from_ids(vec![Uuid::new_v4(); 1])).unwrap();
    (answer.expect("approved"), answered, native)
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

type Change = fn(&mut ImportLedgerLine);

/// Redeemed only for what the person was shown: a different preview, voucher
/// count, batch digest, endpoint, company, or ledger binding is refused, and
/// the approval lapses with a note that is not an approval.
#[tokio::test]
async fn a_redemption_for_anything_else_is_refused_and_lapses() {
    let changes: [(&str, Change, usize, &str); 6] = [
        ("preview", |_| {}, 1, "Another preview"),
        ("count", |_| {}, 2, "Synthetic preview"),
        (
            "digest",
            |line| line.sha256 = "0".repeat(64),
            1,
            "Synthetic preview",
        ),
        (
            "endpoint",
            |line| line.endpoint_origin = Some("http://127.0.0.1:1".into()),
            1,
            "Synthetic preview",
        ),
        (
            "company",
            |line| {
                if let Some(company) = line.company.as_mut() {
                    company.company_number = "100999".into();
                }
            },
            1,
            "Synthetic preview",
        ),
        (
            "ledgers",
            |line| line.vouchers[0].entries[0].ledger = "Cash".into(),
            1,
            "Synthetic preview",
        ),
    ];
    for (case, change, vouchers, preview) in changes {
        let directory = tempfile::tempdir().unwrap();
        let (_server, line) = held_line(directory.path());
        let approvals = PostApprovals::new(directory.path());
        let (request, answered, native) = granted(&line, vouchers).await;
        approvals
            .hold_approved(
                &line.batch_id,
                binding_of(&line, "Synthetic preview"),
                request,
                native,
                answered,
            )
            .unwrap();
        let mut fresh = line.clone();
        change(&mut fresh);
        assert_eq!(
            approvals
                .take_for_dispatch(&line.batch_id, &binding_of(&fresh, preview))
                .err()
                .as_deref(),
            Some("import_approval_binding_changed"),
            "{case}"
        );
        let note = approvals.lapse_note(&line.batch_id).unwrap();
        assert_eq!(note["reason"], "approval_binding_changed", "{case}");
        assert_eq!(note["redeemable"], false, "{case}");
        assert!(
            matches!(approvals.begin(&line.batch_id), Begin::Ask),
            "{case}"
        );
    }
}

/// Spent once, under the lock, before an intent: a second spend, a second
/// redemption (in use, not revoked), and a spend after a revocation are all
/// refused.
#[tokio::test]
async fn an_approval_is_spent_once_and_never_after_a_revocation() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let binding = binding_of(&line, "Synthetic preview");
    let (request, answered, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding.clone(), request, native, answered)
        .unwrap();
    let (taken, _, _) = approvals
        .take_for_dispatch(&line.batch_id, &binding)
        .unwrap();
    assert_eq!(
        approvals
            .take_for_dispatch(&line.batch_id, &binding)
            .err()
            .as_deref(),
        Some("import_approval_in_use")
    );
    approvals.spend(&line.batch_id, taken.id()).unwrap();
    assert_eq!(
        approvals.spend(&line.batch_id, taken.id()).err().as_deref(),
        Some("import_approval_revoked")
    );
    drop(taken);
    assert!(
        approvals.lapse_note(&line.batch_id).is_none(),
        "a spent approval leaves no note"
    );

    let (request, answered, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding.clone(), request, native, answered)
        .unwrap();
    let (taken, _, _) = approvals
        .take_for_dispatch(&line.batch_id, &binding)
        .unwrap();
    approvals.revoke(&line.batch_id, "request_cancelled");
    assert_eq!(
        approvals.spend(&line.batch_id, taken.id()).err().as_deref(),
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
    let approvals = PostApprovals::with_ttl(directory.path(), Duration::from_millis(1));
    let binding = binding_of(&line, "Synthetic preview");
    let (request, answered, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding.clone(), request, native, answered)
        .unwrap();
    std::thread::sleep(Duration::from_millis(5));
    assert!(matches!(approvals.begin(&line.batch_id), Begin::Ask));
    let note = approvals.lapse_note(&line.batch_id).unwrap();
    assert_eq!(note["reason"], "approval_expired");
    assert_eq!(note["state"], "approval_lapsed_unposted");

    // Redeemed late, one is refused as expired: the code the catalogue names.
    let (request, answered, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding.clone(), request, native, answered)
        .unwrap();
    std::thread::sleep(Duration::from_millis(5));
    assert_eq!(
        approvals
            .take_for_dispatch(&line.batch_id, &binding)
            .err()
            .as_deref(),
        Some("import_approval_expired")
    );
}

/// A process that ends holds nothing a new one can redeem: its approval
/// leaves only a note, which the next process reports and never posts from.
#[tokio::test]
async fn an_approval_does_not_survive_its_process() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let (request, answered, native) = granted(&line, 1).await;
    approvals
        .hold_approved(
            &line.batch_id,
            binding_of(&line, "Synthetic preview"),
            request,
            native,
            answered,
        )
        .unwrap();
    drop(approvals);
    let restarted = PostApprovals::new(directory.path());
    assert!(matches!(restarted.begin(&line.batch_id), Begin::Ask));
    let note = restarted.lapse_note(&line.batch_id).unwrap();
    assert_eq!(note["reason"], "process_ended");
    assert_eq!(note["redeemable"], false);
    assert!(restarted
        .take_for_dispatch(&line.batch_id, &binding_of(&line, "Synthetic preview"))
        .is_err());
}

/// A dialog the person already answered, then revoked (a cancelled call):
/// the answer is dropped unread, whether the dialog was held or a joined call
/// had it out, and nothing is left to redeem.
#[tokio::test]
async fn an_answer_revoked_before_its_redemption_is_never_spent() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let binding = binding_of(&line, "Synthetic preview");
    let native = || native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();

    // Held: answered while no call waits, then revoked.
    approvals
        .hold_pending(
            &line.batch_id,
            binding.clone(),
            approved_dialog(1).await,
            native(),
        )
        .unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    approvals.revoke(&line.batch_id, "request_cancelled");
    assert!(matches!(approvals.begin(&line.batch_id), Begin::Ask));
    assert!(approvals
        .take_for_dispatch(&line.batch_id, &binding)
        .is_err());
    assert_eq!(
        approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "request_cancelled",
        "an approval given, then withdrawn, is noted"
    );

    // Joined: a call has the dialog out when the revocation lands.
    approvals
        .hold_pending(
            &line.batch_id,
            binding.clone(),
            approved_dialog(1).await,
            native(),
        )
        .unwrap();
    let Begin::Join(dialog) = approvals.begin(&line.batch_id) else {
        panic!("a held dialog is joined");
    };
    let answer = dialog
        .answer_within(Duration::from_secs(5))
        .await
        .ok()
        .expect("answered");
    approvals.revoke(&line.batch_id, "request_cancelled");
    assert!(matches!(
        approvals.settle_join(&line.batch_id, Ok(answer)),
        Joined::Refused(code) if code == "import_approval_revoked"
    ));
    assert!(matches!(approvals.begin(&line.batch_id), Begin::Ask));
    assert!(approvals
        .take_for_dispatch(&line.batch_id, &binding)
        .is_err());
}

/// Posted in the answering call only while the measured post still fits under
/// the ceiling (#725): at the boundary it does, a millisecond later it does not,
/// and a batch larger than any measured never does.
#[test]
fn an_approval_is_posted_in_its_call_only_while_the_measured_post_fits() {
    let directory = tempfile::tempdir().unwrap();
    let approvals = PostApprovals::new(directory.path());
    let dispatch_fits_in_call = |elapsed, vouchers| approvals.fits_in_call(elapsed, vouchers);
    let boundary = CALL_CEILING - MEASURED_POST;
    assert_eq!(boundary, Duration::from_millis(24_050));
    assert!(dispatch_fits_in_call(Duration::ZERO, 1));
    assert!(dispatch_fits_in_call(boundary, 50));
    assert!(!dispatch_fits_in_call(
        boundary + Duration::from_millis(1),
        50
    ));
    assert!(dispatch_fits_in_call(Duration::ZERO, 200));
    assert!(!dispatch_fits_in_call(Duration::ZERO, 201));
    assert!(!dispatch_fits_in_call(Duration::MAX, 1));
}

/// A joining call that finds a click already made posts it only while the measured
/// redeem, which re-runs every check before posting, still fits under the
/// ceiling (#725 slice 2.0): at the boundary it does, a millisecond later it
/// does not, and a batch larger than the one redeem measured live never does.
#[test]
fn a_joined_approval_is_posted_in_its_call_only_while_the_measured_redeem_fits() {
    let directory = tempfile::tempdir().unwrap();
    let approvals = PostApprovals::new(directory.path());
    let redeem_fits_in_call = |elapsed, vouchers| approvals.redeem_fits_in_call(elapsed, vouchers);
    let boundary = CALL_CEILING - MEASURED_REDEEM;
    assert_eq!(MEASURED_REDEEM, Duration::from_millis(18_090));
    assert_eq!(MEASURED_REDEEM_VOUCHERS, 50);
    assert_eq!(boundary, Duration::from_millis(26_910));
    assert!(redeem_fits_in_call(Duration::ZERO, 1));
    assert!(redeem_fits_in_call(boundary, 50));
    assert!(!redeem_fits_in_call(
        boundary + Duration::from_millis(1),
        50
    ));
    assert!(!redeem_fits_in_call(Duration::ZERO, 51));
    assert!(!redeem_fits_in_call(Duration::MAX, 1));
}

/// The agent's dialog says when its post happens, in both preview shapes and
/// inside the dialog's caps; the desktop's preview does not carry it.
#[test]
fn the_agent_preview_says_when_the_post_happens() {
    let directory = tempfile::tempdir().unwrap();
    let server = batch_server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let (one, _) = saved_batch(&server);
    let endpoint = server.settings.endpoint.clone();
    let [now] = agent_post_timing_lines();
    assert_eq!(
        now,
        "ComplyEaze Bridge posts now or if asked again within 15 min, unless cancelled, refused or restarted."
    );
    // Exactly at the width cap, which refuses only past it: no margin.
    assert_eq!(now.chars().count(), BATCH_REVIEW_MAX_LINE_CHARS, "{now}");
    let single = agent_review_preview(&one, &endpoint).unwrap();
    assert!(single.ends_with(&format!("\n{now}")), "{single}");
    assert!(!admit_fresh_saved_voucher(&one, &endpoint)
        .unwrap()
        .contains(&now));
    let mut two = one.clone();
    let mut second = two.vouchers[0].clone();
    second.bridge_txn_id = "journal-583-2".into();
    second
        .entries
        .iter_mut()
        .for_each(|entry| entry.amount = "7.25".into());
    two.vouchers.push(second);
    let batch = agent_review_preview(&two, &endpoint).unwrap();
    assert!(batch.ends_with(&format!("\n{now}")), "{batch}");
    // The product is named in full in every line the person reads, in a
    // single voucher's dialog and in a batch's.
    for text in [&single, &batch] {
        assert!(
            text.contains("ComplyEaze Bridge"),
            "the product is not named at all: {text}"
        );
        for (at, _) in text.match_indices("Bridge") {
            assert!(
                text[..at].ends_with("ComplyEaze "),
                "a bare Bridge at {at}: {text}"
            );
        }
        assert!(text.contains(
            "Ledgers checked by identity against the build; ComplyEaze Bridge adds its batch reference."
        ));
        assert!(text.contains(
            "Pause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes."
        ));
    }
    // The footer counts inside the caps: one long enough is refused.
    let crowded = vec!["x".to_string(); BATCH_REVIEW_MAX_LINES];
    for line in [&one, &two] {
        assert_eq!(
            review_preview_with(line, &endpoint, &crowded)
                .err()
                .as_deref(),
            Some("import_review_too_large")
        );
    }
}

/// One Journal with `entries` lines: `entries - 1` debits and one credit.
fn journal_of(saved: &ImportLedgerLine, entries: usize) -> ImportLedgerLine {
    let mut line = saved.clone();
    let debit = line.vouchers[0].entries[0].clone();
    let mut credit = line.vouchers[0].entries[1].clone();
    let mut lines = Vec::new();
    for _ in 0..entries - 1 {
        let mut entry = debit.clone();
        entry.amount = "1.00".into();
        lines.push(entry);
    }
    credit.amount = format!("{}.00", entries - 1);
    lines.push(credit);
    line.vouchers[0].entries = lines;
    line.sha256 =
        sha256_hex(render_import_xml("WR2 Unicode Lab", &line.vouchers, &line.batch_id).as_bytes());
    line
}

/// The build's eligibility and the post decide "fits the dialog" with the one
/// function, on the preview the post will show: a Journal the desktop's
/// preview fits but the agent's, with its timing lines, does not, is refused
/// as the post would refuse it, at the line where the two part.
#[test]
fn build_and_post_agree_on_what_fits_the_dialog() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let endpoint = server.settings.endpoint.clone();
    let (saved, _) = saved_batch(&server);
    let seven = journal_of(&saved, 7);
    let eight = journal_of(&saved, 8);
    assert_eq!(
        review_preview_for(&seven, &endpoint, PostScope::Vouchers)
            .unwrap()
            .lines()
            .count(),
        24
    );
    assert!(review_preview_for(&eight, &endpoint, PostScope::JournalOnly).is_ok());
    for refused in [
        review_preview_for(&eight, &endpoint, PostScope::Vouchers).err(),
        admit_saved_voucher(&eight, &endpoint, PostScope::Vouchers, 1).err(),
    ] {
        assert_eq!(refused.as_deref(), Some("import_review_too_large"));
    }
    assert!(admit_saved_voucher(&seven, &endpoint, PostScope::Vouchers, 1).is_ok());
}

/// An approval answered for a call already withdrawn is not kept for the next
/// call: the refusal under the lock, on its own, apart from the biased wait.
#[tokio::test]
async fn an_approval_is_not_held_for_a_withdrawn_call() {
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let approvals = PostApprovals::new(directory.path());
    let binding = binding_of(&line, "Synthetic preview");
    let (request, answered, native) = granted(&line, 1).await;
    let withdrawn = tokio_util::sync::CancellationToken::new();
    withdrawn.cancel();
    let held = crate::tally::runtime::TOOL_CANCELLATION
        .scope(withdrawn, async {
            approvals.hold_approved(&line.batch_id, binding, request, native, answered)
        })
        .await;
    assert_eq!(held.err().as_deref(), Some("request_cancelled"));
    assert!(!approvals.holds(&line.batch_id));
}

/// A call re-entered to redeem that finds nothing to redeem (revoked between
/// its Join and the re-entry) is refused, asks nobody and sends nothing (#725
/// slice 2.0). It can never fall back to asking, which would show the person a
/// second dialog for one batch: the scripted seam here would approve one.
#[tokio::test]
async fn a_redeem_only_entry_with_nothing_held_is_refused_and_asks_nobody() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::approving();
    let refused = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.post_import_entry(
                &args,
                None,
                PostScope::Vouchers,
                Entry::RedeemOnly {
                    evidence: evidence_from_runtime_read(
                        crate::tally::runtime::RuntimeReadEvidence::empty(),
                    ),
                    call_started: std::time::Instant::now(),
                },
            ),
        )
        .await
        .unwrap();
    let Pass::Done(refused) = refused else {
        panic!("a redeem-only pass never hands on another redeem");
    };
    let observed = sent(simulator);
    assert_eq!(
        refused.payload["result"]["error"]["code"], "import_approval_revoked",
        "{}",
        refused.payload
    );
    assert!(scripted.counts().is_empty(), "no dialog was asked");
    assert!(observed.is_empty(), "nothing was sent to Tally");
    assert_eq!(intents(directory.path()), 0);
    assert!(!server.post_approvals.holds(&line.batch_id));
}

/// A re-entered call goes on only for this batch's approval, in its time: an
/// empty slot, an open dialog, another batch's approval, one taken by another
/// call and one past its time are each refused (#725 slice 2.0).
#[tokio::test]
async fn a_redeem_only_begin_goes_on_only_for_this_batchs_live_approval() {
    const OTHER: &str = "bridge-00000000-0000-4000-8000-000000000586";
    let directory = tempfile::tempdir().unwrap();
    let (_server, line) = held_line(directory.path());
    let binding = binding_of(&line, "Synthetic preview");
    let approvals = PostApprovals::new(directory.path());
    let refusal = |approvals: &PostApprovals| approvals.begin_redeem(&line.batch_id).err();

    assert_eq!(
        refusal(&approvals).as_deref(),
        Some("import_approval_revoked"),
        "nothing held"
    );

    let (_, _, native) = granted(&line, 1).await;
    approvals
        .hold_pending(
            &line.batch_id,
            binding.clone(),
            dialog_with(ScriptedApproval::held(), 1).await,
            native,
        )
        .unwrap();
    assert_eq!(
        refusal(&approvals).as_deref(),
        Some("import_approval_revoked"),
        "a dialog still open is joined, never redeemed"
    );
    approvals.revoke(&line.batch_id, "test_reset");

    let (request, answered, native) = granted(&line, 1).await;
    approvals
        .hold_approved(OTHER, binding.clone(), request, native, answered)
        .unwrap();
    assert_eq!(
        refusal(&approvals).as_deref(),
        Some("import_approval_revoked"),
        "another batch's approval"
    );
    approvals.revoke(OTHER, "test_reset");

    let (request, answered, native) = granted(&line, 1).await;
    approvals
        .hold_approved(&line.batch_id, binding.clone(), request, native, answered)
        .unwrap();
    assert_eq!(approvals.begin_redeem(&line.batch_id), Ok(()));
    let (taken, _, _) = approvals
        .take_for_dispatch(&line.batch_id, &binding)
        .unwrap();
    assert_eq!(
        refusal(&approvals).as_deref(),
        Some("import_approval_in_use"),
        "taken by another call"
    );
    drop(taken);

    let expiring = PostApprovals::with_ttl(directory.path(), Duration::from_millis(1));
    let (request, answered, native) = granted(&line, 1).await;
    expiring
        .hold_approved(&line.batch_id, binding, request, native, answered)
        .unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert_eq!(
        expiring.begin_redeem(&line.batch_id).err().as_deref(),
        Some("import_approval_expired")
    );
}

/// A batch dispatched by another route while a joining call waits is not
/// posted when the person then approves: that call only reports the approval,
/// and the next reloads the batch, finds it dispatched, withdraws the approval
/// and only reconciles (#725 slice 2.0). The note's reason tells the two
/// apart: a post refused under the lock would lapse it as refused.
#[tokio::test]
async fn a_batch_dispatched_while_its_approval_waits_is_reconciled_not_posted() {
    let simulator = SequenceSimulator::spawn(with_sentinel(before_approval())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    first_call_pending(&server, directory.path(), &line, &args, &scripted).await;

    let elsewhere_then_click = async {
        tokio::time::timeout(Duration::from_secs(5), async {
            while !server.post_approvals.joined_for_test(&line.batch_id) {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("the second call joined the held dialog");
        {
            let _lock = server.lock_import_admission().unwrap();
            server
                .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
                .unwrap();
        }
        scripted.answer(true);
    };
    let (approved, ()) = tokio::join!(
        server.call_tool("post_import", args.clone()),
        elsewhere_then_click
    );
    assert_eq!(
        result(&approved)["approval"]["state"],
        "approved",
        "{approved}"
    );
    let reconciled = server.call_tool("post_import", args).await;
    let _ = sent(simulator);
    assert_eq!(
        intents(directory.path()),
        1,
        "only the other route's intent: {reconciled}"
    );
    assert!(!server.post_approvals.holds(&line.batch_id));
    assert_eq!(
        server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "batch_already_dispatched",
        "{reconciled}"
    );
    assert_eq!(scripted.counts(), [1]);
}

/// What happens between a call's two passes, run by the test seam there.
type Between = std::sync::Arc<dyn Fn() + Send + Sync>;

/// A click made while no call waited, then a call run with `between` between
/// its passes. The simulator serves further reads, so a second pass that
/// tried to ask the person again reaches Tally and is seen, by its dialog or
/// by the refusal it ends in (an Entry::Fresh second pass ended in a mode
/// probe refusal when measured, 28 Sep 2026).
async fn two_pass_call_with(
    between: impl FnOnce(&Server, &ImportLedgerLine) -> Between,
) -> (
    Value,
    ImportLedgerLine,
    tempfile::TempDir,
    Server,
    ScriptedApproval,
) {
    two_pass_call_recording(between, Default::default()).await
}

/// `two_pass_call_with`, noting the start each pass took its budgets from.
async fn two_pass_call_recording(
    between: impl FnOnce(&Server, &ImportLedgerLine) -> Between,
    starts: std::sync::Arc<std::sync::Mutex<Vec<std::time::Instant>>>,
) -> (
    Value,
    ImportLedgerLine,
    tempfile::TempDir,
    Server,
    ScriptedApproval,
) {
    let (plans, _) = pending_then_posted_plans();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::held();
    first_call_pending(&server, directory.path(), &line, &args, &scripted).await;
    scripted.answer(true);
    until_answered(&server, &line.batch_id).await;
    let hook = between(&server, &line);
    let answer = CALL_STARTS
        .scope(
            starts,
            BETWEEN_PASSES.scope(
                hook,
                SCRIPTED_APPROVAL.scope(scripted.clone(), server.call_tool("post_import", args)),
            ),
        )
        .await;
    let _ = sent(simulator);
    (answer, line, directory, server, scripted)
}

/// Both passes of a call, and the marks readback that spends the budget, take
/// it from the call's start, not their own (V4 P3(1) on #889): the redeem-only
/// pass is entered later, and a start taken afresh there would hand it a full
/// ceiling it no longer has.
#[tokio::test]
async fn both_passes_of_a_call_take_their_budgets_from_the_calls_start() {
    let starts: std::sync::Arc<std::sync::Mutex<Vec<std::time::Instant>>> = Default::default();
    let (answer, ..) = two_pass_call_recording(
        |_, _| std::sync::Arc::new(|| std::thread::sleep(Duration::from_millis(25))),
        std::sync::Arc::clone(&starts),
    )
    .await;
    let starts = starts.lock().unwrap().clone();
    assert_eq!(
        starts.len(),
        3,
        "each pass's entry, then the marks readback that reads the budget: {answer}"
    );
    assert!(
        starts.iter().all(|start| *start == starts[0]),
        "a pass or the readback restarted the clock: {starts:?}"
    );
}

/// An approval lost between the passes (here withdrawn, as an expiry landing
/// there would lapse it) is refused by the second pass, which never asks the
/// person again (#725 slice 2.0): one dialog in all, and no intent.
#[tokio::test]
async fn an_approval_lost_between_the_passes_is_refused_and_asks_nobody_again() {
    let (answer, line, directory, server, scripted) = two_pass_call_with(|server, line| {
        let approvals = std::sync::Arc::clone(&server.post_approvals);
        let batch_id = line.batch_id.clone();
        std::sync::Arc::new(move || approvals.revoke(&batch_id, "approval_expired"))
    })
    .await;
    assert_eq!(
        result(&answer)["error"]["code"],
        "import_approval_revoked",
        "{answer}"
    );
    assert_eq!(scripted.counts(), [1], "no second dialog: {answer}");
    assert_eq!(intents(directory.path()), 0);
    assert!(!server.post_approvals.holds(&line.batch_id));
}

/// A post of the batch by another route landing between the passes (another
/// process's intent in the shared journal) is reconciled by the second pass,
/// never posted again (#725 slice 2.0).
#[tokio::test]
async fn a_post_elsewhere_between_the_passes_is_reconciled_not_posted() {
    let (answer, line, directory, server, scripted) = two_pass_call_with(|server, line| {
        let other = server_at("127.0.0.1:9".parse().unwrap(), &server.settings.data_dir);
        let line = line.clone();
        std::sync::Arc::new(move || {
            let _lock = other.lock_import_admission().unwrap();
            other
                .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
                .unwrap();
        })
    })
    .await;
    assert_eq!(
        intents(directory.path()),
        1,
        "only the other route's intent: {answer}"
    );
    assert_eq!(
        server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "batch_already_dispatched",
        "{answer}"
    );
    assert_eq!(scripted.counts(), [1]);
}

/// A cancellation landing between the passes, as `withdraw_post` makes one
/// (the call's token cancelled, then its approval revoked), posts nothing and
/// asks nobody again (#725 slice 2.0).
#[tokio::test]
async fn a_cancel_between_the_passes_posts_nothing() {
    let token = tokio_util::sync::CancellationToken::new();
    let cancel = token.clone();
    let (answer, line, directory, server, scripted) = crate::tally::runtime::TOOL_CANCELLATION
        .scope(
            token,
            two_pass_call_with(move |server, line| {
                let approvals = std::sync::Arc::clone(&server.post_approvals);
                let batch_id = line.batch_id.clone();
                std::sync::Arc::new(move || {
                    cancel.cancel();
                    approvals.revoke(&batch_id, "request_cancelled");
                })
            }),
        )
        .await;
    assert_eq!(intents(directory.path()), 0, "{answer}");
    assert_eq!(scripted.counts(), [1], "no second dialog: {answer}");
    assert_eq!(
        server.post_approvals.lapse_note(&line.batch_id).unwrap()["reason"],
        "request_cancelled"
    );
}

/// A redeem-only pass of the desktop's Journal post, a pair the flow never
/// builds but the types allow, is refused before anything could ask the
/// person: the desktop asks through its own dialog, so no redeem-only pass of
/// any scope shows a second one (#725 slice 2.0).
#[tokio::test]
async fn a_redeem_only_pass_of_the_desktop_post_is_refused_and_asks_nobody() {
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (_line, args) = saved_batch(&server);
    let scripted = ScriptedApproval::approving();
    let refused = SCRIPTED_APPROVAL
        .scope(
            scripted.clone(),
            server.post_import_entry(
                &args,
                None,
                PostScope::JournalOnly,
                Entry::RedeemOnly {
                    evidence: evidence_from_runtime_read(
                        crate::tally::runtime::RuntimeReadEvidence::empty(),
                    ),
                    call_started: std::time::Instant::now(),
                },
            ),
        )
        .await
        .unwrap();
    let Pass::Done(refused) = refused else {
        panic!("a redeem-only pass never hands on another redeem");
    };
    let observed = sent(simulator);
    assert_eq!(
        refused.payload["result"]["error"]["code"], "import_approval_revoked",
        "{}",
        refused.payload
    );
    assert!(scripted.counts().is_empty(), "no dialog was asked");
    assert!(observed.is_empty(), "nothing was sent to Tally");
    assert_eq!(intents(directory.path()), 0);
}
