//! Retiring a batch that was never sent, by reading Tally for it, through
//! `verify_import` with `if_absent` set to `retire`. The book is the captured
//! one the import tests share; every identifier is invented.
use super::bank_tests::{skip_only_plans, two_bank_plans};
use super::*;
use std::sync::Arc;

const EMPTY_BOOK: &str =
    "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION></COLLECTION></DATA></BODY></ENVELOPE>";

/// The plans with every voucher a read returns taken out of the book, so a
/// batch that was never imported is absent from it.
fn empty_book(plans: Vec<ScenarioPlan>) -> Vec<ScenarioPlan> {
    plans
        .into_iter()
        .map(|mut plan| {
            if let Fixture::SyntheticXml(body) = &plan.fixture {
                if body.contains("<VOUCHER ") {
                    plan.fixture = Fixture::SyntheticXml(EMPTY_BOOK.to_string());
                }
            }
            plan
        })
        .collect()
}

/// The plans of a build and its verification, and a second probe of the
/// licence, which a retirement makes after its read whatever the read found.
fn book(empty: bool) -> Vec<ScenarioPlan> {
    let base = qualified_import_cycle_plans();
    let mut plans = if empty { empty_book(base) } else { base };
    plans.extend(super::mode_tests::licensed_import_probe());
    plans
}

fn server_at(directory: &Path, port: u16, writes_enabled: bool) -> Server {
    Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port,
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled,
        batch_post_enabled: false,
    })
}

/// A batch built and never sent, the server that built it and the simulator
/// that answers its read.
async fn built(directory: &Path, plans: Vec<ScenarioPlan>) -> (SequenceSimulator, Server, String) {
    let simulator = SequenceSimulator::spawn(plans).expect("simulator");
    let server = server_at(directory, simulator.address().port(), true);
    let built = server
        .build_import_xml(&serde_json::to_value(captured_catalogue_payload()).unwrap())
        .await
        .expect("build");
    let batch_id = built.payload["result"]["batch_id"]
        .as_str()
        .unwrap()
        .to_string();
    (simulator, server, batch_id)
}

fn args(batch_id: &str) -> Value {
    json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id, "if_absent": "retire"})
}

async fn retire(server: &Server, batch_id: &str) -> Value {
    server
        .call_tool_response("verify_import", args(batch_id))
        .await
        .value
}

fn saved_file(server: &Server, batch_id: &str) -> std::path::PathBuf {
    server
        .imports_dir()
        .unwrap()
        .join(format!("{batch_id}.xml"))
}

/// The record types the journal holds for the batch, in order; a full record
/// is `batch`.
fn records_of(directory: &Path, batch_id: &str) -> Vec<String> {
    fs::read_to_string(directory.join("agent-import-ledger.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["batch_id"] == batch_id)
        .map(|record| {
            record["record_type"]
                .as_str()
                .unwrap_or("batch")
                .to_string()
        })
        .collect()
}

fn error_code(response: &Value) -> &Value {
    &response["structuredContent"]["result"]["error"]["code"]
}

#[tokio::test]
async fn an_absent_batch_is_retired_its_file_removed_and_post_refuses() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let file = saved_file(&server, &batch_id);
    assert!(file.exists());
    let response = retire(&server, &batch_id).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["retire"]["state"], "retired", "{response}");
    assert!(!file.exists());
    // one verification record, which is the retirement
    let kinds = records_of(directory.path(), &batch_id);
    assert_eq!(kinds.last().unwrap(), "import_batch_retired", "{kinds:?}");
    assert_eq!(
        kinds.iter().filter(|kind| *kind != "batch").count(),
        1,
        "{kinds:?}"
    );
    assert!(
        server
            .latest_import_snapshot(&batch_id)
            .unwrap()
            .unwrap()
            .retired
    );
    // the page served is that read's verification
    assert!(result["verification_status"].is_string(), "{response}");
    // post never sends it, and asks nothing
    let post = server
        .call_tool(
            "post_import",
            json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id}),
        )
        .await;
    assert_eq!(error_code(&post), "import_batch_retired", "{post}");
}

#[tokio::test]
async fn a_voucher_in_tally_blocks_the_retire() {
    // the captured book holds a voucher with this batch's content
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(false)).await;
    let response = retire(&server, &batch_id).await;
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "not_retired", "{response}");
    assert_eq!(retire["reason"], "voucher_in_tally", "{response}");
    assert!(!saved_file(&server, &batch_id).exists());
    assert!(!records_of(directory.path(), &batch_id).contains(&"import_batch_retired".into()));
    assert!(
        !server
            .latest_import_snapshot(&batch_id)
            .unwrap()
            .unwrap()
            .retired
    );
    // the verification is still recorded
    assert!(response["structuredContent"]["result"]["verification_status"].is_string());
}

#[tokio::test]
async fn an_incomplete_read_retires_nothing_and_says_the_file_is_gone() {
    let directory = tempfile::tempdir().unwrap();
    let mut plans = book(true);
    // Tally stops answering part way through the read
    plans.truncate(plans.len() - 12);
    let (_simulator, server, batch_id) = built(directory.path(), plans).await;
    let response = retire(&server, &batch_id).await;
    assert_ne!(response["isError"], true, "{response}");
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "not_retired", "{response}");
    assert_eq!(retire["file_removed"], true, "{response}");
    assert!(retire["reason"].is_string(), "{response}");
    let step = retire["next_step"].as_str().unwrap();
    assert!(
        step.contains("a new post_import call for it is refused"),
        "{step}"
    );
    assert!(step.contains("if_absent"), "{step}");
    assert!(!saved_file(&server, &batch_id).exists());
    assert!(!records_of(directory.path(), &batch_id).contains(&"import_batch_retired".into()));
}

#[tokio::test]
async fn a_sent_batch_is_refused_before_the_delete() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let line = server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    {
        let _admission = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
            .unwrap();
    }
    let response = retire(&server, &batch_id).await;
    assert_eq!(
        error_code(&response),
        "import_retire_batch_sent",
        "{response}"
    );
    assert!(saved_file(&server, &batch_id).exists());
}

/// A batch found posted once is not retired, whatever a later status says; the
/// refusal is the journal's alone, so the saved file stays where it was.
#[tokio::test]
async fn a_batch_once_found_is_refused_and_keeps_its_file() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let line = server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    for status in ["posted_verified", "verification_incomplete"] {
        let mut update = line.clone();
        update.status = status.into();
        let _admission = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::from(&update))
            .unwrap();
    }
    let response = retire(&server, &batch_id).await;
    assert_eq!(
        error_code(&response),
        "import_retire_batch_found",
        "{response}"
    );
    assert!(saved_file(&server, &batch_id).exists());
}

/// A verification that found any voucher of the batch, even one that left the
/// batch short of posted, is a record that Tally held it: a later read that no
/// longer sees the voucher (it was re-dated out of the window, say) does not
/// retire the batch.
#[tokio::test]
async fn a_batch_a_verification_found_a_voucher_of_is_not_retired_by_a_later_read() {
    let directory = tempfile::tempdir().unwrap();
    let verify_from = 32;
    let (_build, server, batch_id) = built(directory.path(), qualified_import_cycle_plans()).await;
    let line = server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    let tag = line.attribution_tag(&line.vouchers[0]);
    // the first voucher is found by its tag, the second only by its content
    let (_found, finder) = reading_from(
        directory.path(),
        &batch_id,
        qualified_import_cycle_plans()[verify_from..].to_vec(),
        Some(&tag),
    );
    let plain = json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id});
    let verified = finder
        .call_tool_response("verify_import", plain.clone())
        .await
        .value;
    assert_ne!(verified["isError"], true, "{verified}");
    let snapshot = server.latest_import_snapshot(&batch_id).unwrap().unwrap();
    assert_eq!(snapshot.batch.status, "verification_incomplete");
    // a later read, of a Tally that no longer shows it, is refused up front
    let (_absent, later) = reading_from(
        directory.path(),
        &batch_id,
        book(true)[verify_from..].to_vec(),
        None,
    );
    let response = retire(&later, &batch_id).await;
    assert_eq!(
        error_code(&response),
        "import_retire_batch_found",
        "{response}"
    );
    assert!(saved_file(&server, &batch_id).exists());
}

/// A retirement that read a voucher in Tally and kept the batch is a record
/// that Tally held one, as a plain verification is.
#[tokio::test]
async fn a_retirement_that_kept_the_batch_stops_a_second_one_from_retiring_it() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(false)).await;
    let first = retire(&server, &batch_id).await;
    let kept = &first["structuredContent"]["result"]["retire"];
    assert_eq!(kept["state"], "not_retired", "{first}");
    let (_absent, later) = reading_from(
        directory.path(),
        &batch_id,
        book(true)[32..].to_vec(),
        None,
    );
    let second = retire(&later, &batch_id).await;
    assert_eq!(
        error_code(&second),
        "import_retire_batch_found",
        "{second}"
    );
}

/// A verification that found no voucher of the batch is not a record that
/// Tally held it, so it does not stand in the way of a retirement.
#[tokio::test]
async fn a_verification_that_found_nothing_does_not_stop_a_retirement() {
    let directory = tempfile::tempdir().unwrap();
    let (_build, server, batch_id) = built(directory.path(), qualified_import_cycle_plans()).await;
    let (_a, verifier) = reading_from(
        directory.path(),
        &batch_id,
        book(true)[32..].to_vec(),
        None,
    );
    let plain = json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id});
    let verified = verifier
        .call_tool_response("verify_import", plain)
        .await
        .value;
    assert_ne!(verified["isError"], true, "{verified}");
    assert_eq!(
        server
            .latest_import_snapshot(&batch_id)
            .unwrap()
            .unwrap()
            .batch
            .status,
        "verification_incomplete"
    );
    assert!(!last_record(directory.path(), &batch_id)
        .as_object()
        .unwrap()
        .contains_key("vouchers_found"));
    let (_b, later) = reading_from(
        directory.path(),
        &batch_id,
        book(true)[32..].to_vec(),
        None,
    );
    let response = retire(&later, &batch_id).await;
    assert_eq!(
        response["structuredContent"]["result"]["retire"]["state"], "retired",
        "{response}"
    );
}

/// Tally here is not the one the batch was built for, so a read of it proves
/// nothing about the batch.
#[tokio::test]
async fn a_batch_built_for_another_endpoint_is_refused_and_keeps_its_file() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let elsewhere = server_at(directory.path(), 9, true);
    let response = retire(&elsewhere, &batch_id).await;
    assert_eq!(
        error_code(&response),
        "import_retire_endpoint_mismatch",
        "{response}"
    );
    assert!(saved_file(&server, &batch_id).exists());
}

#[tokio::test]
async fn a_batch_with_no_recorded_mark_is_refused_and_keeps_its_file() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let mut line = server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    line.pre_import_mark.value = None;
    server.append_import_ledger(&line).unwrap();
    let response = retire(&server, &batch_id).await;
    assert_eq!(
        error_code(&response),
        "import_retire_mark_missing",
        "{response}"
    );
    assert!(saved_file(&server, &batch_id).exists());
}

#[tokio::test]
async fn retire_is_refused_while_writes_are_off() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let off = server_at(directory.path(), 9, false);
    let response = retire(&off, &batch_id).await;
    assert_eq!(
        error_code(&response),
        "import_posting_disabled",
        "{response}"
    );
    assert!(saved_file(&server, &batch_id).exists());
}

#[tokio::test]
async fn retiring_twice_is_refused_the_second_time() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let first = retire(&server, &batch_id).await;
    assert_eq!(
        first["structuredContent"]["result"]["retire"]["state"],
        "retired"
    );
    let again = retire(&server, &batch_id).await;
    assert_eq!(
        error_code(&again),
        "import_retire_already_retired",
        "{again}"
    );
}

#[tokio::test]
async fn a_delete_of_a_file_already_gone_counts_as_done() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    fs::remove_file(saved_file(&server, &batch_id)).unwrap();
    let response = retire(&server, &batch_id).await;
    assert_eq!(
        response["structuredContent"]["result"]["retire"]["state"], "retired",
        "{response}"
    );
}

/// A dispatch intent written between the delete and the record: the record is
/// refused by the batch's generation, so nothing is retired. A sent batch is
/// never recorded as retired.
#[tokio::test]
async fn a_dispatch_intent_between_the_delete_and_the_record_refuses_the_record() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let line = server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    let raced = Server::new(server.settings.clone());
    let hook: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        let _admission = raced.lock_import_admission().unwrap();
        raced
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
            .unwrap();
    });
    let response = AFTER_RETIRE_DELETE
        .scope(hook, retire(&server, &batch_id))
        .await;
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "not_retired", "{response}");
    assert_eq!(
        retire["reason"], "import_verification_conflict_retry",
        "{response}"
    );
    // the batch was just sent: the answer does not say its file is removed
    let step = retire["next_step"].as_str().unwrap();
    assert!(!step.contains("is refused"), "{step}");
    assert!(step.contains("Call verify_import on it again."), "{step}");
    let kinds = records_of(directory.path(), &batch_id);
    assert!(!kinds.contains(&"import_batch_retired".into()), "{kinds:?}");
    assert!(kinds.contains(&"dispatch_intent".into()), "{kinds:?}");
}

fn last_record(directory: &Path, batch_id: &str) -> Value {
    fs::read_to_string(directory.join("agent-import-ledger.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .rfind(|record| record["batch_id"] == batch_id)
        .unwrap()
}

/// A server on a new simulator over the same folder, with the batch moved to
/// its endpoint, so a later read can be scripted with what the batch's own tag
/// is by then known to be.
fn reading_from(
    directory: &Path,
    batch_id: &str,
    plans: Vec<ScenarioPlan>,
    tag: Option<&str>,
) -> (SequenceSimulator, Server) {
    let plans = plans
        .into_iter()
        .map(|mut plan| {
            if let (Some(tag), Fixture::SyntheticXml(body)) = (tag, &plan.fixture) {
                plan.fixture = Fixture::SyntheticXml(
                    body.replace("[BRIDGE:txn-001]", &format!("[BRIDGE:{tag}]")),
                );
            }
            plan
        })
        .collect();
    let simulator = SequenceSimulator::spawn(plans).expect("simulator");
    let server = server_at(directory, simulator.address().port(), true);
    let mut line = server
        .latest_import_snapshot(batch_id)
        .unwrap()
        .unwrap()
        .batch;
    line.endpoint_origin =
        Some(crate::agent::canonical_loopback_origin(&server.settings.endpoint).unwrap());
    server.append_import_ledger(&line).unwrap();
    (simulator, server)
}

/// Once retired, a batch counts again in the transfer facts if a verification
/// sees a voucher carrying its own tag, and not for a content match alone.
#[tokio::test]
async fn a_retired_batch_whose_own_tag_is_seen_again_is_recorded_so() {
    let directory = tempfile::tempdir().unwrap();
    let verify_from = 32;
    let (_build, server, batch_id) = built(directory.path(), qualified_import_cycle_plans()).await;
    let line = server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    let tag = line.attribution_tag(&line.vouchers[0]);
    // read absent: retired
    let (_a, absent) = reading_from(
        directory.path(),
        &batch_id,
        book(true)[verify_from..].to_vec(),
        None,
    );
    let retired = retire(&absent, &batch_id).await;
    assert_eq!(
        retired["structuredContent"]["result"]["retire"]["state"], "retired",
        "{retired}"
    );
    // a verification that finds the content but not the tag records nothing of it
    let (_b, content_only) = reading_from(
        directory.path(),
        &batch_id,
        qualified_import_cycle_plans()[verify_from..].to_vec(),
        None,
    );
    let plain = json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id});
    let seen = content_only
        .call_tool_response("verify_import", plain.clone())
        .await
        .value;
    assert_ne!(seen["isError"], true, "{seen}");
    let record = last_record(directory.path(), &batch_id);
    assert_eq!(record["record_type"], "verification_status", "{record}");
    assert!(record.get("own_tag_observed").is_none(), "{record}");
    // one that finds the tag does
    let (_c, tagged) = reading_from(
        directory.path(),
        &batch_id,
        qualified_import_cycle_plans()[verify_from..].to_vec(),
        Some(&tag),
    );
    let seen = tagged
        .call_tool_response("verify_import", plain)
        .await
        .value;
    assert_ne!(seen["isError"], true, "{seen}");
    let record = last_record(directory.path(), &batch_id);
    assert_eq!(record["own_tag_observed"], true, "{record}");
    assert!(
        tagged
            .latest_import_snapshot(&batch_id)
            .unwrap()
            .unwrap()
            .retired
    );
}

/// The row of another account's statement that skipped it is carried by this
/// batch's Contra: retiring the batch names the Contra, so the person builds
/// this statement again with it skipped, not booked a second time from here.
#[tokio::test]
async fn the_answer_lists_the_rows_the_other_statement_carries() {
    use super::transfer_flow_tests::{a_skips, contra_statement, Rig, BANK_B, BOOKED, SKIPPED};
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [skip_only_plans(), two_bank_plans()].concat(),
    );
    rig.build(a_skips(
        directory.path(),
        &[(SKIPPED, Some("600000000001"))],
    ))
    .await;
    let carrying = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            BOOKED,
            Some("600000000001"),
        ))
        .await;
    let batch_id = carrying["batch_id"].as_str().unwrap().to_string();
    let (_simulator, server) =
        reading_from(directory.path(), &batch_id, book(true)[32..].to_vec(), None);
    // the other account's skip was recorded for the endpoint the batch was
    // built for, which is this Tally now
    let moved = server.latest_import_snapshot(&batch_id).unwrap().unwrap();
    let now = moved.batch.endpoint_origin.clone().unwrap();
    let skips = directory.path().join(statement_facts::SKIPS_FILE);
    let text = fs::read_to_string(&skips).unwrap();
    let before = text
        .split("\"endpoint_origin\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .unwrap()
        .to_string();
    fs::write(&skips, text.replace(&before, &now)).unwrap();
    let response = retire(&server, &batch_id).await;
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "retired", "{response}");
    assert_eq!(retire["carried_rows"], json!([BOOKED]), "{response}");
    let step = retire["next_step"].as_str().unwrap();
    assert!(step.contains("carried_rows skipped"), "{step}");
}

/// A batch no other statement carries a row of is retired with none listed.
#[tokio::test]
async fn a_batch_no_statement_carries_is_retired_with_no_rows_listed() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let response = retire(&server, &batch_id).await;
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "retired", "{response}");
    assert_eq!(retire["carried_rows"], json!([]), "{response}");
    assert_eq!(retire["next_step"], retire::RETIRED_STEP);
}

/// When the transfer facts cannot be read the retirement still stands, and the
/// answer does not claim that no row is carried.
#[tokio::test]
async fn an_unreadable_view_retires_all_the_same_and_lists_nothing() {
    use super::transfer_flow_tests::spoil_the_skip_file;
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    spoil_the_skip_file(directory.path());
    let response = retire(&server, &batch_id).await;
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "retired", "{response}");
    assert!(retire.get("carried_rows").is_none(), "{response}");
    assert_eq!(
        retire["carried_rows_state"], "not_established",
        "{response}"
    );
    assert_eq!(retire["next_step"], retire::RETIRED_UNKNOWN_STEP);
    assert!(
        server
            .latest_import_snapshot(&batch_id)
            .unwrap()
            .unwrap()
            .retired
    );
}

#[tokio::test]
async fn if_absent_takes_one_value_and_no_proof_page() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(directory.path(), 9, true);
    let page = server
        .call_tool_response(
            "verify_import",
            json!({"company_guid": CAPTURED_GUID, "batch_id": "bridge-x",
                "if_absent": "retire", "proof_sha256": "a".repeat(64)}),
        )
        .await
        .value;
    assert_eq!(error_code(&page), "argument_schema_invalid", "{page}");
    let other = server
        .call_tool_response(
            "verify_import",
            json!({"company_guid": CAPTURED_GUID, "batch_id": "bridge-x", "if_absent": "delete"}),
        )
        .await
        .value;
    assert_eq!(other["isError"], true, "{other}");
}

/// Moves `batches` and the skip file of a data folder to the Tally `server`
/// reads, so that a run of calls over several batches meets one simulator.
fn move_to_the_tally_of(server: &Server, directory: &Path, batches: &[&str]) {
    let now = crate::agent::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    for batch_id in batches {
        let mut line = server
            .latest_import_snapshot(batch_id)
            .unwrap()
            .unwrap()
            .batch;
        line.endpoint_origin = Some(now.clone());
        server.append_import_ledger(&line).unwrap();
    }
    let skips = directory.join(statement_facts::SKIPS_FILE);
    if let Ok(text) = fs::read_to_string(&skips) {
        let before = text
            .split("\"endpoint_origin\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .unwrap()
            .to_string();
        fs::write(&skips, text.replace(&before, &now)).unwrap();
    }
}

/// A statement that withdrew a skip while the other account's statement was
/// built and never sent, and that statement, on one folder; the Tally of the
/// returned simulator is scripted for what follows.
struct Withdrawn {
    withdrawn: String,
    carrier: String,
    carrier_tag: String,
}

async fn a_withdrawn_skip_waits_on_an_unsent_carrier(directory: &Path) -> Withdrawn {
    use super::transfer_flow_tests::{
        a_skips, contra_statement, post_error, withdrawing, Rig, BANK_B, BOOKED, SKIPPED,
    };
    let rig = Rig::new(
        directory,
        [skip_only_plans(), two_bank_plans(), two_bank_plans()].concat(),
    );
    rig.build(a_skips(directory, &[(SKIPPED, None)])).await;
    let first = rig.build(withdrawing(directory)).await;
    let withdrawn = first["batch_id"].as_str().unwrap().to_string();
    let carrying = rig
        .build(contra_statement(directory, BANK_B, BOOKED, None))
        .await;
    let carrier = carrying["batch_id"].as_str().unwrap().to_string();
    // waiting on a batch never sent: refused, and the step points at the retire
    let refused = post_error(&rig.server, &withdrawn).await;
    assert_eq!(refused["code"], "withdraw_skip_paired", "{refused}");
    assert_eq!(refused["paired_rows"][0]["batch_id"], carrier.as_str());
    assert_eq!(refused["paired_rows"][0]["state"], "other_side_unverified");
    let step = refused["next_step"].as_str().unwrap();
    assert!(step.contains("if_absent set to retire"), "{step}");
    let line = rig
        .server
        .latest_import_snapshot(&carrier)
        .unwrap()
        .unwrap()
        .batch;
    let carrier_tag = line.attribution_tag(&line.vouchers[0]);
    Withdrawn {
        withdrawn,
        carrier,
        carrier_tag,
    }
}

/// The two servers over one scripted Tally: one that may retire, one that
/// sends as the bank rig does.
fn one_tally(directory: &Path, plans: Vec<ScenarioPlan>) -> (SequenceSimulator, Server, Server) {
    let simulator = SequenceSimulator::spawn(plans).expect("simulator");
    let port = simulator.address().port();
    (
        simulator,
        server_at(directory, port, true),
        server_at(directory, port, false),
    )
}

/// The other account's statement is built and never sent, so it counts as
/// carrying the transfer and the batch that withdrew the skip cannot be sent.
/// Once a read of Tally has retired it, the send reads Tally again for the
/// retired batch's own tag before it goes on, and goes on when there is none.
#[tokio::test]
async fn a_retired_carrier_leaves_the_view_and_the_other_side_sends_after_a_second_read() {
    use super::transfer_flow_tests::{post_error, TRANSFER_REASONS};
    let directory = tempfile::tempdir().unwrap();
    let held = a_withdrawn_skip_waits_on_an_unsent_carrier(directory.path()).await;
    let scripted = [read_of(true, None), read_of(true, None)].concat();
    let total = scripted.len();
    let (simulator, retiring, sending) = one_tally(directory.path(), scripted);
    move_to_the_tally_of(&retiring, directory.path(), &[&held.carrier, &held.withdrawn]);
    // a read of a Tally that holds none of it retires the carrier
    let response = retire(&retiring, &held.carrier).await;
    assert_eq!(
        response["structuredContent"]["result"]["retire"]["state"], "retired",
        "{response}"
    );
    // the send reads Tally again for the carrier, finds nothing, and goes on
    // (it fails later, at the network)
    let sent = post_error(&sending, &held.withdrawn).await;
    let code = sent["code"].as_str().unwrap_or_default();
    assert!(!TRANSFER_REASONS.contains(&code), "{sent}");
    assert_ne!(code, "retired_batch_found_in_tally", "{sent}");
    assert!(
        simulator.received() >= total,
        "the second read was made: {} of {total}",
        simulator.received()
    );
    // the second read is a verification of the carrier, recorded as any is, and
    // the carrier stays retired
    let kinds = records_of(directory.path(), &held.carrier);
    assert!(kinds.contains(&"verification_status".into()), "{kinds:?}");
    assert!(
        retiring
            .latest_import_snapshot(&held.carrier)
            .unwrap()
            .unwrap()
            .retired
    );
}

/// The second read finds a voucher with the retired batch's own tag: nothing
/// is sent, the refusal names the batch and tells no one to enter anything in
/// Tally, and the batch counts again.
#[tokio::test]
async fn a_send_that_rests_on_a_retired_batch_is_refused_when_its_tag_is_found() {
    use super::transfer_flow_tests::post_error;
    let directory = tempfile::tempdir().unwrap();
    let held = a_withdrawn_skip_waits_on_an_unsent_carrier(directory.path()).await;
    let scripted = [read_of(true, None), read_of(false, Some(&held.carrier_tag))].concat();
    let (_simulator, retiring, sending) = one_tally(directory.path(), scripted);
    move_to_the_tally_of(&retiring, directory.path(), &[&held.carrier, &held.withdrawn]);
    let response = retire(&retiring, &held.carrier).await;
    assert_eq!(
        response["structuredContent"]["result"]["retire"]["state"], "retired",
        "{response}"
    );
    let refused = post_error(&sending, &held.withdrawn).await;
    assert_eq!(refused["code"], "retired_batch_found_in_tally", "{refused}");
    assert_eq!(
        refused["retired_batches"],
        json!([held.carrier.as_str()]),
        "{refused}"
    );
    let step = refused["next_step"].as_str().unwrap();
    assert!(
        step.starts_with("Nothing was sent to Tally and the approval was not used. "),
        "{step}"
    );
    for by_hand in ["by hand", "enter the voucher", "import the file", "in Tally itself"] {
        assert!(!step.contains(by_hand), "{step}");
    }
    // nothing was sent
    assert!(!records_of(directory.path(), &held.withdrawn).contains(&"dispatch_intent".into()));
    // and the retired batch counts again: the next send meets the ordinary refusal
    assert!(
        records_of(directory.path(), &held.carrier)
            .iter()
            .filter(|kind| *kind == "verification_status")
            .count()
            >= 1
    );
    let again = post_error(&sending, &held.withdrawn).await;
    assert_ne!(again["code"], "retired_batch_found_in_tally", "{again}");
}

/// A retired batch whose exclusion changes nothing for this send is not read
/// for: no request is made to Tally and nothing is written for it.
#[tokio::test]
async fn a_send_that_does_not_rest_on_a_retired_batch_makes_no_second_read() {
    use super::transfer_flow_tests::{
        contra_statement, post_error, Rig, BANK_A, BANK_B, BOOKED, TRANSFER_REASONS,
    };
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [two_bank_plans(), two_bank_plans()].concat(),
    );
    let first = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            BOOKED,
            Some("600000000001"),
        ))
        .await;
    let carrier = first["batch_id"].as_str().unwrap().to_string();
    let other = rig
        .build(contra_statement(
            directory.path(),
            BANK_A,
            "st-20260901-00000000000000c9",
            Some("600000000002"),
        ))
        .await;
    let other = other["batch_id"].as_str().unwrap().to_string();
    let line = rig
        .server
        .latest_import_snapshot(&carrier)
        .unwrap()
        .unwrap()
        .batch;
    let tag = line.attribution_tag(&line.vouchers[0]);
    // a second read, if one were made, would find the tag
    let scripted = [read_of(true, None), read_of(false, Some(&tag))].concat();
    let first_block = read_of(true, None).len();
    let (simulator, retiring, sending) = one_tally(directory.path(), scripted);
    move_to_the_tally_of(&retiring, directory.path(), &[&carrier, &other]);
    let response = retire(&retiring, &carrier).await;
    assert_eq!(
        response["structuredContent"]["result"]["retire"]["state"], "retired",
        "{response}"
    );
    let before = records_of(directory.path(), &carrier);
    let sent = post_error(&sending, &other).await;
    let code = sent["code"].as_str().unwrap_or_default();
    assert!(!TRANSFER_REASONS.contains(&code), "{sent}");
    assert_ne!(code, "retired_batch_found_in_tally", "{sent}");
    assert_eq!(records_of(directory.path(), &carrier), before);
    // the retired batch was not read for: its tag is still unseen
    assert!(
        simulator.received() >= first_block,
        "the retirement's own read was made"
    );
    assert!(
        !last_record(directory.path(), &carrier)
            .as_object()
            .unwrap()
            .contains_key("own_tag_observed")
    );
}

/// A hand import of the saved file, re-dated a few days past the batch's last
/// day, is seen only by a read wider than the batch's own days.
#[tokio::test]
async fn a_voucher_with_the_batchs_tag_dated_past_its_days_blocks_the_retire() {
    let directory = tempfile::tempdir().unwrap();
    let (_build, server, batch_id) = built(directory.path(), qualified_import_cycle_plans()).await;
    let line = server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    let tag = line.attribution_tag(&line.vouchers[0]);
    let past = (chrono::NaiveDate::parse_from_str(line.date_to.as_str(), "%Y%m%d").unwrap()
        + chrono::Duration::days(3))
    .format("%Y%m%d")
    .to_string();
    let first_day = line.vouchers[0].date.as_str().to_string();
    let plans = book(false)[32..]
        .iter()
        .cloned()
        .map(|mut plan| {
            if let Fixture::SyntheticXml(body) = &plan.fixture {
                plan.fixture = Fixture::SyntheticXml(body.replace(
                    &format!("<DATE>{first_day}</DATE>"),
                    &format!("<DATE>{past}</DATE>"),
                ));
            }
            plan
        })
        .collect();
    let (_read, reader) = reading_from(directory.path(), &batch_id, plans, Some(&tag));
    let response = retire(&reader, &batch_id).await;
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "not_retired", "{response}");
    assert_eq!(retire["reason"], "own_tag_in_tally", "{response}");
    assert!(!records_of(directory.path(), &batch_id).contains(&"import_batch_retired".into()));
    // the read covered the days past the batch's own
    let proof: Value = serde_json::from_slice(
        &fs::read(reader.current_proof_paths(&batch_id)[0].clone()).unwrap(),
    )
    .unwrap();
    assert!(
        proof["retire_read"]["to"].as_str().unwrap() >= past.as_str(),
        "{proof}"
    );
}

/// What the refusals and steps of a retirement say is what the code does: the
/// read takes a voucher for the batch by its narration tag or its content and
/// only after the mark the build recorded; a kept batch's file is removed, which
/// refuses a new send of it, not one already waiting for approval.
#[test]
fn the_retire_texts_say_only_what_the_read_does() {
    assert_eq!(
        crate::agent::refusal_remediation("import_retire_mark_missing"),
        Some("This batch recorded no voucher mark when it was built. A read of Tally takes a voucher for this batch's own, by its narration tag or its content, only if the voucher was created after that mark, so without one every voucher of the batch would read as not found whatever Tally holds. Nothing was changed; the batch keeps counting.")
    );
    assert_eq!(
        retire::KEPT_STEP,
        "The read of Tally did not find this batch absent: a voucher carries its narration tag, matches its content, or could not be read as absent (`reason` says which), so it was not retired and still counts. Its saved file is removed, so a new post_import call for it is refused; a send already waiting for approval is not stopped by the removal."
    );
    for step in [
        retire::RETIRED_STEP,
        retire::RETIRED_CARRIED_STEP,
        retire::RETIRED_UNKNOWN_STEP,
    ] {
        assert!(
            step.starts_with("This batch is retired: ComplyEaze Bridge read Tally, found no voucher with its narration tag or its content, removed its saved file and will not send it."),
            "{step}"
        );
    }
}

/// The tool text says what the code reads and what it answers: the window as
/// `widened_window` builds it, the three states, and which refusals a repeat
/// of the call cannot clear.
#[test]
fn the_tool_text_states_the_retire_read_and_its_three_answers() {
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    let tool = definitions
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "verify_import")
        .expect("verify_import");
    let text = tool["description"].as_str().unwrap();
    assert!(
        text.contains("7 days either side of the batch's dates, limited to the book (not before it began, not past today unless the batch's own last day is), then moved further out to the nearest day the licence accepts as a boundary; refused when no such window exists (`import_retire_window_unavailable`)"),
        "{text}"
    );
    assert!(!text.contains("covers up to 7 days"), "{text}");
    assert!(
        text.contains("`state` (`retired`, `not_retired` or `not_established`)"),
        "{text}"
    );
    assert!(
        text.contains(
            "a repeat of the call can help, except after a refusal of the read's size or limits"
        ),
        "{text}"
    );
    for code in [
        "voucher_window_too_many_reads",
        "voucher_window_part_over_budget",
        "voucher_window_volume_unestimated",
        "verification_window_day_not_readable",
        "verification_mode_unqualified",
        "window_part_boundary_unsupported_in_education",
    ] {
        assert!(text.contains(code), "{code}: {text}");
    }
    assert!(
        text.contains("`not_established`: the batch may be retired"),
        "{text}"
    );
}

const CONTRA_IN_TALLY: &str = "<VOUCHER REMOTEID=\"tally-assigned-9\"><DATE>20260901</DATE><VOUCHERNUMBER>CV-1</VOUCHERNUMBER><VOUCHERTYPENAME>Contra</VOUCHERTYPENAME><GUID>61c6de69-1748-461c-ad3f-162cb949df9f-00000009</GUID><MASTERID>9</MASTERID><ALTERID>12</ALTERID><NARRATION></NARRATION><ISCANCELLED>No</ISCANCELLED><ISOPTIONAL>No</ISOPTIONAL><ALLLEDGERENTRIES.LIST><LEDGERNAME>Bridge Nested Debtor WR4</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-12.50</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>WR2 Sales</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>12.50</AMOUNT></ALLLEDGERENTRIES.LIST></VOUCHER>";

/// The plans with every voucher a read returns replaced by `voucher`.
fn book_holding(voucher: &str) -> Vec<ScenarioPlan> {
    let envelope = format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>{voucher}</COLLECTION></DATA></BODY></ENVELOPE>"
    );
    book(false)
        .into_iter()
        .map(|mut plan| {
            if let Fixture::SyntheticXml(body) = &plan.fixture {
                if body.contains("<VOUCHER ") {
                    plan.fixture = Fixture::SyntheticXml(envelope.clone());
                }
            }
            plan
        })
        .collect()
}

/// What another batch of the statement's rows is in the journal.
#[derive(Clone, Copy)]
enum Sibling {
    Absent,
    SentOnly,
    Found,
    /// Found posted, then a later read did not find it.
    FoundThenLost,
}

/// A never-sent batch holding a statement Contra; Tally holds a voucher of
/// that Contra's content, with no tag. Another batch of another lineage holds
/// the same statement row as `sibling` says. Returns the retirement's answer.
async fn retire_beside(sibling: Sibling) -> Value {
    retire_beside_counting(sibling).await.0
}

/// [`retire_beside`], and the requests the retirement made, with how many
/// answers were scripted.
async fn retire_beside_counting(
    sibling: Sibling,
) -> (
    Value,
    Vec<tally_protocol_simulator::ObservedRequest>,
    usize,
) {
    use super::transfer_flow_tests::{contra_statement, Rig, BANK_B, BOOKED};
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(directory.path(), two_bank_plans());
    let built = rig
        .build(contra_statement(directory.path(), BANK_B, BOOKED, None))
        .await;
    let batch_id = built["batch_id"].as_str().unwrap().to_string();
    let mut other = rig
        .server
        .latest_import_snapshot(&batch_id)
        .unwrap()
        .unwrap()
        .batch;
    other.batch_id = "bridge-00000000-0000-4000-8000-0000000007f1".into();
    if !matches!(sibling, Sibling::Absent) {
        rig.server.append_import_ledger(&other).unwrap();
        let record = match sibling {
            Sibling::Found | Sibling::FoundThenLost => {
                other.status = "posted_verified".into();
                ledger::StatusRecord::from(&other)
            }
            _ => ledger::StatusRecord::dispatch(&other),
        };
        let _admission = rig.server.lock_import_admission().unwrap();
        rig.server
            .append_import_record_while_admitted(&record)
            .unwrap();
        if matches!(sibling, Sibling::FoundThenLost) {
            other.status = "verification_incomplete".into();
            rig.server
                .append_import_record_while_admitted(&ledger::StatusRecord::from(&other))
                .unwrap();
        }
    }
    let plans = book_holding(CONTRA_IN_TALLY)[32..].to_vec();
    let scripted = plans.len();
    let (simulator, server) = reading_from(directory.path(), &batch_id, plans, None);
    let response = retire(&server, &batch_id).await;
    assert_eq!(simulator.received(), scripted, "{response}");
    (
        response["structuredContent"]["result"]["retire"].clone(),
        simulator.finish().expect("requests"),
        scripted,
    )
}

/// The licence no longer qualifies when the closing probe asks: the file is
/// already removed, nothing is recorded, and the answer says a repeat will not
/// help until the mode changes.
#[tokio::test]
async fn a_closing_probe_that_does_not_qualify_is_reported_after_the_removal() {
    let directory = tempfile::tempdir().unwrap();
    let (_build, server, batch_id) = built(directory.path(), qualified_import_cycle_plans()).await;
    let mut plans = read_of(true, None);
    plans.truncate(plans.len() - 2);
    plans.extend(super::mode_tests::import_profile_probe("product"));
    let (_read, reader) = reading_from(directory.path(), &batch_id, plans, None);
    let response = retire(&reader, &batch_id).await;
    assert_ne!(response["isError"], true, "{response}");
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "not_retired", "{response}");
    assert_eq!(retire["reason"], "verification_mode_unqualified", "{response}");
    assert_eq!(retire["file_removed"], true, "{response}");
    assert_eq!(retire["verification_recorded"], false, "{response}");
    assert!(
        retire["next_step"]
            .as_str()
            .unwrap()
            .contains("a repeat will not help until"),
        "{response}"
    );
    assert!(!saved_file(&server, &batch_id).exists());
    assert!(!records_of(directory.path(), &batch_id).contains(&"import_batch_retired".into()));
}

/// A retirement asserts an absence, so the licence is qualified again after the
/// read whatever the read found: here nothing was not found, and the closing
/// probe is still sent. It is the request the opening probe was, not another
/// read.
#[tokio::test]
async fn the_closing_licence_probe_is_sent_even_when_nothing_was_not_found() {
    for sibling in [Sibling::Found, Sibling::Absent] {
        let (retire, observed, scripted) = retire_beside_counting(sibling).await;
        assert!(retire["state"].is_string(), "{retire}");
        assert_eq!(observed.len(), scripted, "{retire}");
        let request = |at: usize| {
            let seen = &observed[at];
            (
                seen.method.clone(),
                seen.path.clone(),
                seen.request_body_sha256.clone(),
            )
        };
        let last = observed.len() - 1;
        assert_eq!(request(last - 1), request(0), "{retire}");
        assert_eq!(request(last), request(1), "{retire}");
    }
}

/// A voucher in Tally that matches the batch by content alone keeps the batch
/// counting, whatever another batch of the company holds in the journal: a
/// retirement credits a content match to no other batch.
#[tokio::test]
async fn a_content_match_keeps_the_batch_whatever_another_batch_holds() {
    for sibling in [
        Sibling::Absent,
        Sibling::SentOnly,
        Sibling::Found,
        Sibling::FoundThenLost,
    ] {
        let kept = retire_beside(sibling).await;
        assert_eq!(kept["state"], "not_retired", "{kept}");
        assert_eq!(kept["reason"], "voucher_in_tally", "{kept}");
    }
}

/// The record of the retirement is appended and the publication's marker then
/// cannot be removed: the record is in the journal, so the answer is that it is
/// not established whether the batch is retired, not that nothing was retired.
#[cfg(unix)]
#[tokio::test]
async fn an_error_after_the_record_is_appended_is_not_established() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, batch_id) = built(directory.path(), book(true)).await;
    let marker = directory.path().join("imports").join(".proof-publication");
    let hook_marker = marker.clone();
    let hook: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        fs::set_permissions(&hook_marker, fs::Permissions::from_mode(0o500)).unwrap();
    });
    let response = AFTER_RECORD_APPENDED
        .scope(hook, retire(&server, &batch_id))
        .await;
    fs::set_permissions(&marker, fs::Permissions::from_mode(0o700)).unwrap();
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "not_established", "{response}");
    assert_eq!(
        retire["reason"], "proof_publication_recovery_required",
        "{response}"
    );
    let step = retire["next_step"].as_str().unwrap();
    assert_eq!(
        step,
        "The read of Tally finished, but ComplyEaze Bridge could not confirm that the record of it was written, so it is not established whether the batch is retired. The import folder is marked as needing recovery, and verify_import is refused for this batch until that is cleared, so there is no call to make on it now. Nothing sends it meanwhile: its saved file is removed. Report this with the reason named here."
    );
    // the call the step used to advise is refused while the marker stands
    let plain = server
        .call_tool_response(
            "verify_import",
            json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id}),
        )
        .await
        .value;
    assert_eq!(
        error_code(&plain),
        "proof_publication_recovery_required",
        "{plain}"
    );
    // the record is in the journal, as the answer allows for
    let kinds = records_of(directory.path(), &batch_id);
    assert_eq!(kinds.last().unwrap(), "import_batch_retired", "{kinds:?}");
}

/// The plans of a read of the retire's shape: the opening licence probe, the
/// window, and the closing probe. `empty` is a Tally that holds none of the
/// batch; `tag`, if given, replaces the tag the captured voucher carries, so the
/// book holds a voucher with that batch's narration tag.
fn read_of(empty: bool, tag: Option<&str>) -> Vec<ScenarioPlan> {
    book(empty)[32..]
        .iter()
        .cloned()
        .map(|mut plan| {
            if let (Some(tag), Fixture::SyntheticXml(body)) = (tag, &plan.fixture) {
                plan.fixture = Fixture::SyntheticXml(
                    body.replace("[BRIDGE:txn-001]", &format!("[BRIDGE:{tag}]")),
                );
            }
            plan
        })
        .collect()
}

/// The plans of the retire's read in a book whose voucher high-water mark is
/// past what can be counted in bounded requests.
fn read_of_a_book_too_large_to_count() -> Vec<ScenarioPlan> {
    read_of(true, None)
        .into_iter()
        .map(|mut plan| {
            if let Fixture::SyntheticXml(body) = &plan.fixture {
                plan.fixture = Fixture::SyntheticXml(
                    body.replace("<ALTVCHID>10</ALTVCHID>", "<ALTVCHID>3000000</ALTVCHID>"),
                );
            }
            plan
        })
        .collect()
}

/// The window is planned and sized before the saved file is removed: a read the
/// plan refuses leaves the file where it was, retires nothing, records nothing,
/// and says the batch is unchanged.
#[tokio::test]
async fn a_window_plan_refusal_leaves_the_saved_file_and_says_nothing_changed() {
    let directory = tempfile::tempdir().unwrap();
    let (_build, server, batch_id) = built(directory.path(), qualified_import_cycle_plans()).await;
    let before = records_of(directory.path(), &batch_id);
    let (_read, reader) = reading_from(
        directory.path(),
        &batch_id,
        read_of_a_book_too_large_to_count(),
        None,
    );
    let response = retire(&reader, &batch_id).await;
    assert_ne!(response["isError"], true, "{response}");
    let retire = &response["structuredContent"]["result"]["retire"];
    assert_eq!(retire["state"], "not_retired", "{response}");
    assert_eq!(retire["reason"], "voucher_window_book_too_large", "{response}");
    assert_eq!(retire["file_removed"], false, "{response}");
    assert_eq!(retire["verification_recorded"], false, "{response}");
    assert_eq!(
        retire["next_step"],
        "The read of Tally was refused before anything was changed: the saved file of this batch is still in place, it was not retired and it still counts. A repeat will not help until the read limit or the licence mode that stopped the read changes.",
        "{response}"
    );
    assert!(saved_file(&server, &batch_id).exists());
    // nothing was recorded but the move to this endpoint
    let after = records_of(directory.path(), &batch_id);
    assert_eq!(after.len(), before.len() + 1, "{after:?}");
    assert!(!after.contains(&"import_batch_retired".into()), "{after:?}");
    assert!(!after.contains(&"verification_status".into()), "{after:?}");
    // the batch can still be sent from its file: it is not retired
    assert!(
        !server
            .latest_import_snapshot(&batch_id)
            .unwrap()
            .unwrap()
            .retired
    );
}
