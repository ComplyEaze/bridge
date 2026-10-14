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
    assert!(step.contains("can no longer be sent"), "{step}");
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

/// The other account's statement is built and never sent, so it counts as
/// carrying the transfer and the batch that withdrew the skip cannot be sent.
/// Once a read of Tally has retired it, that batch goes through.
#[tokio::test]
async fn a_retired_carrier_leaves_the_view_and_the_other_side_sends() {
    use super::transfer_flow_tests::{
        a_skips, contra_statement, post_error, withdrawing, Rig, BANK_B, BOOKED, SKIPPED,
        TRANSFER_REASONS,
    };
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [skip_only_plans(), two_bank_plans(), two_bank_plans()].concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, None)]))
        .await;
    let first = rig.build(withdrawing(directory.path())).await;
    let withdrawn = first["batch_id"].as_str().unwrap().to_string();
    let carrying = rig
        .build(contra_statement(directory.path(), BANK_B, BOOKED, None))
        .await;
    let carrier = carrying["batch_id"].as_str().unwrap().to_string();
    // waiting on a batch never sent: refused, and the step points at the retire
    let refused = post_error(&rig.server, &withdrawn).await;
    assert_eq!(refused["code"], "withdraw_skip_paired", "{refused}");
    assert_eq!(refused["paired_rows"][0]["batch_id"], carrier.as_str());
    assert_eq!(refused["paired_rows"][0]["state"], "other_side_unverified");
    let step = refused["next_step"].as_str().unwrap();
    assert!(step.contains("if_absent set to retire"), "{step}");
    // a read of a Tally that holds none of it retires the carrier
    let (_simulator, server) =
        reading_from(directory.path(), &carrier, book(true)[32..].to_vec(), None);
    let response = retire(&server, &carrier).await;
    assert_eq!(
        response["structuredContent"]["result"]["retire"]["state"], "retired",
        "{response}"
    );
    // now the batch passes the transfer check (it fails later, at the network)
    let sent = post_error(&rig.server, &withdrawn).await;
    let code = sent["code"].as_str().unwrap_or_default();
    assert!(!TRANSFER_REASONS.contains(&code), "{sent}");
}
