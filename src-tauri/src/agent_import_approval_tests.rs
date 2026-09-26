//! #725 slice 1: a post dialog outlives the call that asked it. Driven through
//! `post_import` (and, for cancellation, through the stdio path every agent
//! post takes) against the protocol simulator where a call is involved, and on
//! the held approvals directly where only their rules are. The dialog is the
//! test-only seam, held open until a test answers it.
use super::*;
use crate::agent::agent_import::approval::{
    ApprovalBinding, Begin, Joined, PostApprovals, CALL_CEILING, MEASURED_POST,
};
use crate::agent::agent_protocol::{run_post, Framer};
use crate::agent::ToolResponse;
use crate::tally::agent_read_request::AgentReadRequest;
use crate::tally::approved_import::{Answered, ApprovedImport, PendingPostApproval};
use tokio::io::{AsyncWriteExt, BufReader};

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

/// A dialog nobody has answered by the end of the call leaves the call
/// `pending`, having sent nothing past its checks. Once the person approves,
/// the next call returns `approved` without reading Tally, and the one after
/// checks the book afresh and posts exactly once.
#[tokio::test]
async fn a_dialog_answered_after_its_call_returned_is_posted_by_a_later_call() {
    let mut plans = before_approval();
    plans.extend(before_approval());
    let post_at = plans.len() + after_approval(xml(created_one())).len() - 1;
    plans.extend(after_approval(xml(created_one())));
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
    assert_eq!(
        result(&pending)["dispatch"]["state"],
        "not_dispatched",
        "{pending}"
    );
    assert_eq!(result(&pending)["attempt_recorded"], false, "{pending}");
    assert_eq!(intents(directory.path()), 0);

    scripted.answer(true);
    let approved = server.call_tool("post_import", args.clone()).await;
    assert_eq!(
        result(&approved)["approval"]["state"],
        "approved",
        "{approved}"
    );
    assert_eq!(intents(directory.path()), 0);

    let posted = server.call_tool("post_import", args).await;
    let observed = sent(simulator);
    // The simulator's script ends at the POST, so the readback after it is
    // not served; what is asserted is the one intent and its clean create.
    assert_eq!(result(&posted)["attempt_recorded"], true, "{posted}");
    assert_eq!(
        dispatch_intent(directory.path())["batch_id"],
        line.batch_id.as_str()
    );
    assert_journaled_clean_create(directory.path());
    assert_eq!(scripted.counts(), [1], "one dialog, asked once");
    assert!(observed.len() > post_at, "the POST was sent: {posted}");
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

/// Wait until a held dialog's task has ended after its answer.
async fn until_closed(scripted: &ScriptedApproval) {
    while scripted.is_waiting() {
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    tokio::time::sleep(Duration::from_millis(20)).await;
}

/// A dialog declined while no call waited no longer blocks other batches: the
/// next call, for another batch, settles it and asks about its own.
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
        .scope(scripted.clone(), server.call_tool("post_import", args))
        .await;
    assert_eq!(
        result(&pending)["approval"]["state"],
        "pending",
        "{pending}"
    );
    scripted.answer(false);
    until_closed(&scripted).await;
    let asked = ScriptedApproval::declining();
    let declined = SCRIPTED_APPROVAL
        .scope(asked.clone(), server.call_tool("post_import", other))
        .await;
    let observed = sent(simulator);
    assert_eq!(
        result(&declined)["error"]["code"],
        "import_approval_declined",
        "the other batch was asked, not refused as busy: {declined}"
    );
    assert_eq!(asked.counts(), [1]);
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
    until_closed(&scripted).await;
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
#[tokio::test]
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
        while !scripted.is_waiting() {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        withdrawn.cancel();
    };
    let (response, ()) = tokio::join!(post, cancel);
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

/// An answer and a withdrawal ready together: the withdrawal wins, so the
/// approval does not outlive the cancelled call (#725, the biased wait and the
/// held-under-lock refusal). Repeated, since the two race across threads.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_answer_arriving_with_the_cancellation_is_not_kept() {
    for _ in 0..8 {
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
        let both = async {
            while !scripted.is_waiting() {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            scripted.answer(true);
            withdrawn.cancel();
        };
        let (response, ()) = tokio::join!(post, both);
        let _ = sent(simulator);
        assert_eq!(
            result(&response)["error"]["code"],
            "request_cancelled",
            "{response}"
        );
        assert!(!server.post_approvals.holds(&line.batch_id));
        assert_eq!(intents(directory.path()), 0);
    }
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
/// asks the person again rather than posting on the old click.
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
    let approved = server.call_tool("post_import", args.clone()).await;
    assert_eq!(
        result(&approved)["approval"]["state"],
        "approved",
        "{approved}"
    );
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
    // The redeeming call stops at the currency refusal, before the closing
    // mode probe its checks would otherwise end with.
    assert_eq!(observed.len(), 2 * before_approval().len() - probe().len());
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
    assert!(result(&refused)["error"]["code"].is_string(), "{refused}");
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
    SCRIPTED_APPROVAL
        .scope(ScriptedApproval::approving(), async {
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
    std::thread::sleep(Duration::from_millis(5));
    assert!(matches!(approvals.begin(&line.batch_id), Begin::Ask));
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

/// The agent's dialog says when its post happens, in both preview shapes and
/// inside the dialog's caps; the desktop's preview does not carry it.
#[test]
fn the_agent_preview_says_when_the_post_happens() {
    let directory = tempfile::tempdir().unwrap();
    let server = batch_server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let (one, _) = saved_batch(&server);
    let endpoint = server.settings.endpoint.clone();
    let [now, not] = agent_post_timing_lines();
    assert!(now.contains("within 15 minutes"), "{now}");
    for line in [&now, &not] {
        assert!(
            line.chars().count() <= BATCH_REVIEW_MAX_LINE_CHARS,
            "{line}"
        );
    }
    let single = agent_review_preview(&one, &endpoint).unwrap();
    assert!(single.ends_with(&format!("{now}\n{not}")), "{single}");
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
    assert!(batch.ends_with(&format!("{now}\n{not}")), "{batch}");
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
