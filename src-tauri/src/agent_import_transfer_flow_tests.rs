//! The transfer view through the tools: what a build is refused for, and the
//! `transfers` section of a build, of a statement of only skipped rows and of a
//! verification. Two bank ledgers of the captured book stand for two accounts,
//! and every row, amount and reference is invented.
use super::bank_tests::{
    bank_build_plans, bank_server, captured_bank_payload, record, skip_only_plans, statement_v2,
    two_bank_plans,
};
use super::*;

const BANK_A: &str = "WR2 Sales";
const BANK_B: &str = "Bridge Nested Debtor WR4";
/// A row bank A's statement skipped as carried by bank B's Contra.
const SKIPPED: &str = "st-20260901-00000000000000b1";
/// The Contra bank B's statement booked for the same transfer.
const BOOKED: &str = "st-20260901-00000000000000c1";
const OTHER_BOOKED: &str = "st-20260901-00000000000000c2";
const UNKNOWN: &str = "st-20260901-00000000000000ff";
const REFERENCE: &str = "600000000001";
/// Every number a section must keep to itself.
const SECRETS: [&str; 6] = ["12.50", REFERENCE, "000417", BANK_A, BANK_B, "Cash"];
/// The requests of a build that writes a batch, of a statement of only skipped
/// rows, and of a build refused when the batches of the company are read: after
/// the book is read for its ledgers and groups and before its mark.
const REFUSED_AT_ADMISSION: usize = 18;

fn window() -> Value {
    json!({"from": "2026-08-02", "to": "2026-09-29"})
}

/// A row of `bank`'s statement skipped as carried by the other bank's Contra.
fn skip_record(bank: &str, txn: &str, reference: Option<&str>) -> Value {
    skip_record_flagged(bank, txn, reference, false)
}

/// The same, with the parse's flag for a row that has an equal amount the other
/// way the same day on its own statement.
fn skip_record_flagged(bank: &str, txn: &str, reference: Option<&str>, flagged: bool) -> Value {
    let other = if bank == BANK_A { BANK_B } else { BANK_A };
    let mut skip = record(txn, json!("skipped"), other, "outflow", flagged);
    skip["reference"] = reference.map_or(Value::Null, |reference| json!(reference));
    skip
}

fn contra_record(txn: &str, reference: Option<&str>) -> Value {
    let mut contra = record(txn, json!({"voucher": "Contra"}), BANK_A, "inflow", false);
    contra["reference"] = reference.map_or(Value::Null, |reference| json!(reference));
    contra
}

/// A transfer into bank B from bank A, as bank B or bank A would book it.
fn contra_voucher(txn: &str) -> Value {
    json!({"bridge_txn_id": txn, "date": "2026-09-01", "voucher_type": "Contra",
        "entries": [{"ledger": BANK_B, "amount": "12.50", "side": "Dr"},
                    {"ledger": BANK_A, "amount": "12.50", "side": "Cr"}]})
}

/// Bank A's statement of only skipped rows.
fn a_skips(directory: &Path, rows: &[(&str, Option<&str>)]) -> Value {
    a_skips_flagged(directory, rows, false)
}

fn a_skips_flagged(directory: &Path, rows: &[(&str, Option<&str>)], flagged: bool) -> Value {
    statement_v2(
        directory,
        BANK_A,
        window(),
        json!([]),
        rows.iter()
            .map(|(txn, reference)| skip_record_flagged(BANK_A, txn, *reference, flagged))
            .collect::<Vec<_>>()
            .into(),
    )
}

/// A statement of `bank` booking `txn` as a Contra.
fn contra_statement(directory: &Path, bank: &str, txn: &str, reference: Option<&str>) -> Value {
    statement_v2(
        directory,
        bank,
        window(),
        json!([contra_voucher(txn)]),
        json!([contra_record(txn, reference)]),
    )
}

/// One simulator and one server over a data directory, for a run of calls.
struct Rig {
    simulator: SequenceSimulator,
    server: Server,
    expected: usize,
}

impl Rig {
    fn new(directory: &Path, plans: Vec<ScenarioPlan>) -> Self {
        let expected = plans.len();
        let simulator = SequenceSimulator::spawn(plans).expect("plans");
        let server = bank_server(directory, simulator.address().port());
        Self {
            simulator,
            server,
            expected,
        }
    }

    async fn build(&self, args: Value) -> Value {
        let response = self
            .server
            .call_tool_response("build_import_xml", args)
            .await
            .value;
        response["structuredContent"]["result"].clone()
    }

    fn finish(self) {
        assert_eq!(
            self.simulator.finish().expect("requests").len(),
            self.expected
        );
    }
}

/// One call, one simulator.
async fn run(directory: &Path, plans: Vec<ScenarioPlan>, args: Value) -> Value {
    let rig = Rig::new(directory, plans);
    let result = rig.build(args).await;
    rig.finish();
    result
}

fn admission_plans() -> Vec<ScenarioPlan> {
    two_bank_plans()[..REFUSED_AT_ADMISSION].to_vec()
}

fn journal(directory: &Path) -> Server {
    bank_server(directory, 9)
}

fn latest(directory: &Path, batch_id: &str) -> ImportLedgerLine {
    journal(directory)
        .latest_import_snapshot(batch_id)
        .unwrap()
        .unwrap()
        .batch
}

/// The batch was sent to Tally.
fn mark_sent(directory: &Path, batch_id: &str) {
    let server = journal(directory);
    let record = ledger::StatusRecord::dispatch(&latest(directory, batch_id));
    let _admission = server.lock_import_admission().unwrap();
    server.append_import_record_while_admitted(&record).unwrap();
}

/// A readback found the batch posted.
fn mark_found(directory: &Path, batch_id: &str) {
    let server = journal(directory);
    let mut line = latest(directory, batch_id);
    line.status = "posted_verified".into();
    let record = ledger::StatusRecord::from(&line);
    let _admission = server.lock_import_admission().unwrap();
    server.append_import_record_while_admitted(&record).unwrap();
}

fn no_journal(directory: &Path) {
    assert!(!directory.join("agent-import-ledger.jsonl").exists());
    let imports = directory.join("imports");
    assert!(!imports.exists() || fs::read_dir(imports).unwrap().next().is_none());
}

fn skip_file(directory: &Path) -> std::path::PathBuf {
    directory.join(statement_facts::SKIPS_FILE)
}

fn open_rows(section: &Value) -> Vec<Value> {
    section["open"].as_array().unwrap().clone()
}

fn assert_nothing_secret(section: &Value) {
    let text = section.to_string();
    for secret in SECRETS {
        assert!(!text.contains(secret), "{secret} in {text}");
    }
}

fn open_row(txn: &str, state: &str, batch: Option<&str>) -> Value {
    match batch {
        Some(batch) => {
            json!({"bridge_txn_id": txn, "date": "20260901", "state": state, "batch_id": batch})
        }
        None => json!({"bridge_txn_id": txn, "date": "20260901", "state": state}),
    }
}

// ---- items 15 and 16: skipped rows are listed until the other side is verified ----

#[tokio::test]
async fn a_skipped_row_is_listed_until_the_other_statement_is_built_and_verified_and_then_pairs() {
    let directory = tempfile::tempdir().unwrap();
    // one endpoint throughout: a skip and a Contra of two endpoints never pair
    let rig = Rig::new(
        directory.path(),
        [skip_only_plans(), two_bank_plans(), skip_only_plans()].concat(),
    );
    // Bank A's statement skips the row. Nothing of bank B is known.
    let result = rig
        .build(a_skips(directory.path(), &[(SKIPPED, Some(REFERENCE))]))
        .await;
    assert_eq!(result["state"], "skips_recorded", "{result}");
    let section = &result["transfers"];
    assert_eq!(section["state"], "computed", "{result}");
    assert_eq!(section["skips_paired"], 0);
    assert_eq!(section["skips_open"], 1);
    assert_eq!(section["open_omitted"], 0);
    assert_eq!(section["retired"], 0);
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "not_established", None)]
    );
    assert_eq!(
        section["next_steps"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        ["not_established"]
    );
    assert_nothing_secret(section);

    // Bank B's statement books the transfer. Built, not yet verified: the build
    // says which batch to verify.
    let built = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            BOOKED,
            Some(REFERENCE),
        ))
        .await;
    assert_eq!(built["voucher_count"], 1, "{built}");
    let batch = built["batch_id"].as_str().unwrap().to_string();
    let section = &built["transfers"];
    assert_eq!(section["skips_open"], 1, "{built}");
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "other_side_unverified", Some(&batch))]
    );
    let step = section["next_steps"]["other_side_unverified"]
        .as_str()
        .unwrap();
    assert!(step.contains("Run verify_import on that batch"), "{step}");
    assert_nothing_secret(section);

    // Found posted, the same skipped row is paired: counted, and not listed.
    mark_found(directory.path(), &batch);
    let again = rig
        .build(a_skips(directory.path(), &[(SKIPPED, Some(REFERENCE))]))
        .await;
    rig.finish();
    let section = &again["transfers"];
    assert_eq!(section["state"], "computed", "{again}");
    assert_eq!(section["skips_paired"], 1, "{again}");
    assert_eq!(section["skips_open"], 0);
    assert_eq!(section["open"], json!([]));
    assert_eq!(section["next_steps"], json!({}));
    assert_nothing_secret(section);
}

// ---- item 13 and withdraw_skips -------------------------------------------------

#[tokio::test]
async fn posting_a_row_another_statement_skipped_is_refused_unless_the_skip_is_withdrawn() {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [
            skip_only_plans(),
            admission_plans(),
            two_bank_plans(),
            skip_only_plans(),
        ]
        .concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, None)]))
        .await;
    let posting = |extra: Option<Value>| {
        let mut args = contra_statement(directory.path(), BANK_A, SKIPPED, None);
        if let Some(withdraw) = extra {
            args["withdraw_skips"] = withdraw;
        }
        args
    };
    // refused before the book's mark is read, listing the row
    let refused = rig.build(posting(None)).await;
    assert_eq!(refused["state"], "refused", "{refused}");
    assert_eq!(refused["reason"], "skip_row_paired_elsewhere");
    assert_eq!(refused["skipped_rows"], json!([SKIPPED]));
    let step = refused["next_step"].as_str().unwrap();
    assert!(step.contains("withdraw_skips"), "{step}");
    assert!(
        step.contains("the statement again with those rows mapped skip"),
        "{step}"
    );
    assert!(
        step.contains("confirms the other account's statement does not carry it"),
        "{step}"
    );
    assert!(!step.contains("build again without"), "{step}");
    no_journal(directory.path());

    // withdrawn, the row is posted and the skip is held by the new batch
    let built = rig.build(posting(Some(json!([SKIPPED])))).await;
    assert_eq!(built["voucher_count"], 1, "{built}");
    let batch = built["batch_id"].as_str().unwrap().to_string();
    let section = &built["transfers"];
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "held_elsewhere", Some(&batch))],
        "{built}"
    );
    assert_eq!(section["skips_open"], 1);
    assert!(section["next_steps"]["held_elsewhere"].is_string());

    // once the new batch is found posted, the skip is retired
    mark_found(directory.path(), &batch);
    let later = rig
        .build(a_skips(directory.path(), &[(OTHER_BOOKED, None)]))
        .await;
    rig.finish();
    assert_eq!(later["transfers"]["retired"], 1, "{later}");
    assert_eq!(later["transfers"]["skips_open"], 1);
}

#[tokio::test]
async fn a_skip_in_a_batch_only_built_is_not_a_reason_to_refuse_the_row() {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [two_bank_plans(), two_bank_plans()].concat(),
    );
    // a batch with a Contra and a skipped row, built and never sent
    let first = rig
        .build(statement_v2(
            directory.path(),
            BANK_A,
            window(),
            json!([contra_voucher(BOOKED)]),
            json!([
                contra_record(BOOKED, None),
                skip_record(BANK_A, SKIPPED, None)
            ]),
        ))
        .await;
    assert_eq!(first["voucher_count"], 1, "{first}");
    // posting the skipped row elsewhere is allowed: the newer build supersedes it
    let posting = contra_statement(directory.path(), BANK_A, SKIPPED, None);
    let built = rig.build(posting).await;
    rig.finish();
    assert_eq!(built["voucher_count"], 1, "{built}");
    // and a withdrawal of a skip that is not live is invalid
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [two_bank_plans(), admission_plans()].concat(),
    );
    rig.build(statement_v2(
        directory.path(),
        BANK_A,
        window(),
        json!([contra_voucher(BOOKED)]),
        json!([
            contra_record(BOOKED, None),
            skip_record(BANK_A, SKIPPED, None)
        ]),
    ))
    .await;
    let mut withdrawing = contra_statement(directory.path(), BANK_A, SKIPPED, None);
    withdrawing["withdraw_skips"] = json!([SKIPPED]);
    let refused = rig.build(withdrawing).await;
    rig.finish();
    assert_eq!(refused["reason"], "withdraw_skip_invalid", "{refused}");
    assert_eq!(refused["invalid_rows"], json!([SKIPPED]));
}

#[tokio::test]
async fn a_withdrawal_must_name_a_live_skip_and_a_row_of_the_build() {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [
            skip_only_plans(),
            admission_plans(),
            admission_plans(),
            skip_only_plans(),
        ]
        .concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, None)]))
        .await;
    let invalid = |refused: &Value, rows: Value| {
        assert_eq!(refused["state"], "refused", "{refused}");
        assert_eq!(refused["reason"], "withdraw_skip_invalid", "{refused}");
        assert_eq!(refused["invalid_rows"], rows, "{refused}");
        assert_eq!(refused["invalid_rows_omitted"], 0);
    };
    // a live skip that this build does not post
    let mut elsewhere = contra_statement(directory.path(), BANK_A, BOOKED, None);
    elsewhere["withdraw_skips"] = json!([SKIPPED]);
    let refused = rig.build(elsewhere).await;
    invalid(&refused, json!([SKIPPED]));
    // a row nobody skipped, however much the build posts the live one
    let mut unknown = contra_statement(directory.path(), BANK_A, SKIPPED, None);
    unknown["withdraw_skips"] = json!([UNKNOWN]);
    let refused = rig.build(unknown).await;
    invalid(&refused, json!([UNKNOWN]));
    // a statement of only skipped rows posts nothing to withdraw for
    let mut skip_only = a_skips(directory.path(), &[(OTHER_BOOKED, None)]);
    skip_only["withdraw_skips"] = json!([SKIPPED]);
    let refused = rig.build(skip_only).await;
    rig.finish();
    invalid(&refused, json!([SKIPPED]));
    // nothing was written by any of them
    assert_eq!(
        fs::read_to_string(skip_file(directory.path()))
            .unwrap()
            .lines()
            .count(),
        1
    );
    no_journal(directory.path());
}

#[tokio::test]
async fn a_malformed_withdrawal_is_refused_before_any_read() {
    let directory = tempfile::tempdir().unwrap();
    // a simulator whose one plan must stay unused: any request to Tally counts
    let rig = Rig::new(directory.path(), two_bank_plans()[..1].to_vec());
    for (bad, code) in [
        // the schema layer, by the shape of the published fragment
        (json!(["bridge-1"]), "argument_invalid:withdraw_skips"),
        (json!(SKIPPED), "argument_invalid:withdraw_skips"),
        (json!([7]), "argument_invalid:withdraw_skips"),
        (json!(null), "argument_invalid:withdraw_skips"),
        // distinctness is the parser's, since the schema layer does not check it
        (json!([SKIPPED, SKIPPED]), "argument_schema_invalid"),
    ] {
        let mut args = contra_statement(directory.path(), BANK_A, SKIPPED, None);
        args["withdraw_skips"] = bad.clone();
        let result = rig.build(args).await;
        assert_eq!(result["error"]["code"], code, "{bad}: {result}");
    }
    // none of them read anything, or wrote anything
    assert_eq!(rig.simulator.received(), 0);
    no_journal(directory.path());
}

// ---- item 14 ----------------------------------------------------------------

#[tokio::test]
async fn skipping_a_row_a_sent_or_found_batch_holds_is_refused_naming_the_batch() {
    for found in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let rig = Rig::new(
            directory.path(),
            [two_bank_plans(), skip_only_plans(), admission_plans()].concat(),
        );
        let first = rig
            .build(contra_statement(directory.path(), BANK_A, BOOKED, None))
            .await;
        let held = first["batch_id"].as_str().unwrap().to_string();
        let skip_only = |directory: &Path| {
            statement_v2(
                directory,
                BANK_B,
                window(),
                json!([]),
                json!([skip_record(BANK_B, BOOKED, None)]),
            )
        };
        if found {
            mark_found(directory.path(), &held);
        } else {
            mark_sent(directory.path(), &held);
        }
        let refused = rig.build(skip_only(directory.path())).await;
        assert_eq!(refused["state"], "refused", "{refused}");
        assert_eq!(refused["reason"], "skip_row_already_posted");
        assert_eq!(refused["blocking_batch_id"], held);
        assert_eq!(refused["skipped_rows"], json!([BOOKED]));
        assert!(!skip_file(directory.path()).exists());
        // the same row skipped by a build that writes a batch
        let building = statement_v2(
            directory.path(),
            BANK_B,
            window(),
            json!([contra_voucher(OTHER_BOOKED)]),
            json!([
                contra_record(OTHER_BOOKED, None),
                skip_record(BANK_B, BOOKED, None)
            ]),
        );
        let refused = rig.build(building).await;
        rig.finish();
        assert_eq!(refused["reason"], "skip_row_already_posted", "{refused}");
        assert_eq!(refused["blocking_batch_id"], held);
    }
}

#[tokio::test]
async fn skipping_a_row_a_batch_only_built_holds_is_not_refused() {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [two_bank_plans(), skip_only_plans()].concat(),
    );
    rig.build(contra_statement(directory.path(), BANK_A, BOOKED, None))
        .await;
    let recorded = rig
        .build(statement_v2(
            directory.path(),
            BANK_B,
            window(),
            json!([]),
            json!([skip_record(BANK_B, BOOKED, None)]),
        ))
        .await;
    rig.finish();
    assert_eq!(recorded["state"], "skips_recorded", "{recorded}");
}

#[tokio::test]
async fn an_amendment_cannot_skip_a_row_its_own_lineage_built() {
    let directory = tempfile::tempdir().unwrap();
    // first build: two Contras, never sent. second: an amendment that keeps one
    // and skips the other, which would leave the voucher in the book.
    let plans = [two_bank_plans(), admission_plans()].concat();
    let rig = Rig::new(directory.path(), plans);
    let first = rig
        .build(statement_v2(
            directory.path(),
            BANK_A,
            window(),
            json!([contra_voucher(BOOKED), contra_voucher(OTHER_BOOKED)]),
            json!([
                contra_record(BOOKED, None),
                contra_record(OTHER_BOOKED, None)
            ]),
        ))
        .await;
    let original = first["batch_id"].as_str().unwrap().to_string();
    let mut amending = statement_v2(
        directory.path(),
        BANK_A,
        window(),
        json!([contra_voucher(BOOKED)]),
        json!([
            contra_record(BOOKED, None),
            skip_record(BANK_A, OTHER_BOOKED, None)
        ]),
    );
    amending["amends_batch_id"] = json!(original);
    let refused = rig.build(amending).await;
    rig.finish();
    assert_eq!(refused["reason"], "skip_row_already_posted", "{refused}");
    assert_eq!(refused["blocking_batch_id"], original);
    assert_eq!(refused["skipped_rows"], json!([OTHER_BOOKED]));
}

// ---- item 10: a transfer booked from both statements ----------------------------

#[tokio::test]
async fn the_same_transfer_booked_twice_with_one_reference_refuses_the_second_build() {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [two_bank_plans(), admission_plans()].concat(),
    );
    let first = rig
        .build(contra_statement(
            directory.path(),
            BANK_A,
            BOOKED,
            Some(REFERENCE),
        ))
        .await;
    let other = first["batch_id"].as_str().unwrap().to_string();
    let refused = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            OTHER_BOOKED,
            Some(REFERENCE),
        ))
        .await;
    rig.finish();
    assert_eq!(refused["state"], "refused", "{refused}");
    assert_eq!(refused["reason"], "transfer_double_booked");
    assert_eq!(refused["other_batch_id"], other);
    // nothing of the refused build was written
    assert_eq!(
        fs::read_to_string(directory.path().join("agent-import-ledger.jsonl"))
            .unwrap()
            .lines()
            .count(),
        1
    );
}

#[tokio::test]
async fn the_same_transfer_booked_twice_without_a_reference_is_listed_not_refused() {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [two_bank_plans(), two_bank_plans()].concat(),
    );
    let first = rig
        .build(contra_statement(
            directory.path(),
            BANK_A,
            BOOKED,
            Some(REFERENCE),
        ))
        .await;
    let earlier = first["batch_id"].as_str().unwrap().to_string();
    let built = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            OTHER_BOOKED,
            None,
        ))
        .await;
    rig.finish();
    assert_eq!(built["voucher_count"], 1, "{built}");
    let later = built["batch_id"].as_str().unwrap().to_string();
    // each Contra is listed, naming the other statement's batch
    assert_eq!(
        open_rows(&built["transfers"]),
        vec![
            open_row(BOOKED, "double_booked", Some(&later)),
            open_row(OTHER_BOOKED, "double_booked", Some(&earlier)),
        ],
        "{built}"
    );
    assert_eq!(built["transfers"]["skips_open"], 0);
    assert!(built["transfers"]["next_steps"]["double_booked"].is_string());
    assert_nothing_secret(&built["transfers"]);
}

// ---- the facts cannot be read ---------------------------------------------------

/// A skip-only record whose id is not one, so reading it fails a check.
fn spoil_the_skip_file(directory: &Path) {
    let facts = statement_facts::StatementFacts {
        bank_ledger_guid: "g1".into(),
        coverage: None,
        contra_rows: Vec::new(),
        skip_rows: vec![statement_facts::SkipRow {
            bridge_txn_id: SKIPPED.into(),
            date: bridge_tally_core::TallyDate::parse("20260901").unwrap(),
            amount: "12.50".into(),
            direction: statement_facts::RowDirection::Outflow,
            other_ledger_guid: "g2".into(),
            reference: None,
            instrument: None,
            reversed_same_day: false,
        }],
    };
    let record = statement_facts::SkipOnlyRecord {
        record_id: "skips-spoiled".into(),
        company_guid: CAPTURED_GUID.into(),
        endpoint_origin: "http://127.0.0.1:1".into(),
        built_at: "2026-09-01T10:00:00Z".into(),
        statement: facts,
    };
    fs::write(
        skip_file(directory),
        format!("{}\n", serde_json::to_string(&record).unwrap()),
    )
    .unwrap();
}

#[tokio::test]
async fn a_build_that_needs_the_view_is_refused_when_it_cannot_be_read() {
    let directory = tempfile::tempdir().unwrap();
    spoil_the_skip_file(directory.path());
    // a statement build, a build posting a statement row inline, and a build
    // asking to withdraw a skip
    let inline =
        |txn: &str| json!({"company_guid": CAPTURED_GUID, "vouchers": [contra_voucher(txn)]});
    let inline_row = inline(SKIPPED);
    let mut asking = inline("txn-1");
    asking["withdraw_skips"] = json!([SKIPPED]);
    for args in [
        contra_statement(directory.path(), BANK_A, BOOKED, None),
        inline_row,
        asking,
    ] {
        let refused = run(directory.path(), admission_plans(), args).await;
        assert_eq!(refused["state"], "refused", "{refused}");
        assert_eq!(refused["reason"], "transfer_view_not_established");
        // the failure and the record, and no value from it
        assert_eq!(refused["error"], "facts");
        assert_eq!(refused["check"], "bad_record_id");
        assert_eq!(refused["record_id"], "skips-spoiled");
        let text = refused.to_string();
        for value in [SKIPPED, "12.50", "g1", "g2"] {
            assert!(!text.contains(value), "{value} in {text}");
        }
        no_journal(directory.path());
    }
    // a statement of skipped rows is a statement
    let refused = run(
        directory.path(),
        skip_only_plans(),
        a_skips(directory.path(), &[(OTHER_BOOKED, None)]),
    )
    .await;
    assert_eq!(refused["reason"], "transfer_view_not_established");
}

#[tokio::test]
async fn a_build_that_is_not_about_transfers_goes_on_when_the_view_cannot_be_read() {
    let directory = tempfile::tempdir().unwrap();
    spoil_the_skip_file(directory.path());
    let built = run(
        directory.path(),
        bank_build_plans(),
        serde_json::to_value(captured_bank_payload()).unwrap(),
    )
    .await;
    assert_eq!(built["voucher_count"], 2, "{built}");
    let section = &built["transfers"];
    assert_eq!(section["state"], "not_established", "{built}");
    assert!(section.get("open").is_none());
    assert_eq!(
        section["next_steps"]["check_not_run"].as_str().unwrap(),
        "Not run, or cut to fit: verify again; if it repeats, raise BRIDGE_AGENT_MAX_BYTES."
    );
}

#[tokio::test]
async fn every_build_says_where_the_company_stands_on_transfers() {
    let directory = tempfile::tempdir().unwrap();
    let built = run(
        directory.path(),
        bank_build_plans(),
        serde_json::to_value(captured_bank_payload()).unwrap(),
    )
    .await;
    assert_eq!(
        built["transfers"],
        json!({"state": "computed", "skips_paired": 0, "skips_open": 0, "open": [],
            "open_omitted": 0, "retired": 0, "next_steps": {}}),
        "{built}"
    );
}

// ---- item 17: verify_import -----------------------------------------------------

/// A built Journal batch of the captured book, with a skipped row recorded for
/// the company, and the simulator that answers its verification.
async fn built_batch_with_a_skip(directory: &Path) -> (SequenceSimulator, Server, Value, String) {
    let result = run(
        directory,
        skip_only_plans(),
        a_skips(directory, &[(SKIPPED, None)]),
    )
    .await;
    assert_eq!(result["state"], "skips_recorded", "{result}");
    let simulator = SequenceSimulator::spawn(qualified_import_cycle_plans()).expect("simulator");
    let server = bank_server(directory, simulator.address().port());
    let built = server
        .build_import_xml(&serde_json::to_value(captured_catalogue_payload()).unwrap())
        .await
        .expect("build");
    let batch_id = built.payload["result"]["batch_id"]
        .as_str()
        .unwrap()
        .to_string();
    let args = json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id});
    (simulator, server, args, batch_id)
}

#[tokio::test]
async fn the_first_page_of_a_verification_carries_the_section_and_a_later_page_does_not() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, args, batch_id) = built_batch_with_a_skip(directory.path()).await;
    let first = server
        .call_tool_response("verify_import", args.clone())
        .await
        .value;
    assert_ne!(first["isError"], true, "{first}");
    let page = &first["structuredContent"]["result"];
    let section = &page["transfers"];
    assert_eq!(section["state"], "computed", "{first}");
    assert_eq!(section["skips_open"], 1);
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "not_established", None)]
    );
    assert_nothing_secret(section);
    // the verdict is the proof's, which the section is not part of
    let [proof_path, _] = server.current_proof_paths(&batch_id);
    let persisted = fs::read(&proof_path).unwrap();
    assert_eq!(
        page["proof"]["sha256"],
        crate::agent::sha256_hex(&persisted)
    );
    assert!(!String::from_utf8_lossy(&persisted).contains("\"transfers\""));
    // a later page comes from the proof alone
    let mut next = args.clone();
    next["proof_sha256"] = page["proof"]["sha256"].clone();
    next["offset"] = json!(0);
    let later = server.call_tool_response("verify_import", next).await.value;
    assert_ne!(later["isError"], true, "{later}");
    assert!(later["structuredContent"]["result"]
        .get("transfers")
        .is_none());
}

#[tokio::test]
async fn a_view_that_cannot_be_read_changes_no_verdict_and_fails_no_verification() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, args, _) = built_batch_with_a_skip(directory.path()).await;
    spoil_the_skip_file(directory.path());
    let first = server.call_tool_response("verify_import", args).await.value;
    assert_ne!(first["isError"], true, "{first}");
    let page = &first["structuredContent"]["result"];
    assert_eq!(page["transfers"]["state"], "not_established", "{first}");
    assert!(page["transfers"].get("open").is_none());
    // the verdict is there as it is without the section
    assert!(page["verification_status"].is_string(), "{first}");
    assert!(page["counts"].is_object());
}

#[tokio::test]
async fn contention_for_the_admission_lock_leaves_the_section_not_established() {
    let directory = tempfile::tempdir().unwrap();
    run(
        directory.path(),
        skip_only_plans(),
        a_skips(directory.path(), &[(SKIPPED, None)]),
    )
    .await;
    let server = journal(directory.path());
    let computed = server.transfers_for_verification(CAPTURED_GUID, 0);
    assert_eq!(computed["state"], "computed", "{computed}");
    let held = server.lock_import_admission().unwrap();
    let contended = server.transfers_for_verification(CAPTURED_GUID, 0);
    assert_eq!(contended["state"], "not_established", "{contended}");
    drop(held);
    // a shared holder does not stop a shared reader
    let reading = server.lock_import_admission_shared().unwrap();
    let alongside = server.transfers_for_verification(CAPTURED_GUID, 0);
    assert_eq!(alongside["state"], "computed", "{alongside}");
    drop(reading);
}

#[tokio::test]
async fn the_verification_a_post_makes_never_computes_the_section() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server, args, _) = built_batch_with_a_skip(directory.path()).await;
    let outcome = server
        .verify_import_for_post(&args)
        .await
        .unwrap_or_else(|failure| panic!("the post's own verification failed: {failure:?}"));
    let result = &outcome.payload["result"];
    assert!(result["verification_status"].is_string(), "{result}");
    assert!(result.get("transfers").is_none(), "{result}");
}

#[tokio::test]
async fn the_readback_right_after_a_post_never_computes_the_section() {
    let directory = tempfile::tempdir().unwrap();
    run(
        directory.path(),
        skip_only_plans(),
        a_skips(directory.path(), &[(SKIPPED, None)]),
    )
    .await;
    let simulator = SequenceSimulator::spawn(qualified_import_cycle_plans()).expect("simulator");
    let server = Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    });
    let built = server
        .build_import_xml(&serde_json::to_value(captured_catalogue_payload()).unwrap())
        .await
        .expect("build");
    let batch_id = built.payload["result"]["batch_id"]
        .as_str()
        .unwrap()
        .to_string();
    let saved = latest(directory.path(), &batch_id);
    {
        let _admission = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&saved))
            .unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::response(
                &saved,
                persisted_dispatch_response(1, 0),
            ))
            .unwrap();
    }
    let outcome = server
        .verify_import_after_current_dispatch(
            &json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id}),
            json!({"state": "not_checked", "reason": "masters_unmoved"}),
            None,
        )
        .await
        .expect("the readback");
    let result = &outcome.payload["result"];
    assert!(result["dispatch"]["state"].is_string(), "{result}");
    assert!(result.get("transfers").is_none(), "{result}");
}

// ---- a skip the view already pairs is not withdrawn --------------------------------

fn withdrawing(directory: &Path) -> Value {
    let mut args = contra_statement(directory, BANK_A, SKIPPED, None);
    args["withdraw_skips"] = json!([SKIPPED]);
    args
}

#[tokio::test]
async fn a_skip_the_other_statement_already_carries_cannot_be_withdrawn() {
    // paired (found) or waiting in an unverified batch, flagged by the parse or not:
    // the reference says why the row is open, whatever the flag
    for (found, flagged) in [(true, false), (false, false), (true, true), (false, true)] {
        let directory = tempfile::tempdir().unwrap();
        let rig = Rig::new(
            directory.path(),
            [skip_only_plans(), two_bank_plans(), admission_plans()].concat(),
        );
        rig.build(a_skips_flagged(
            directory.path(),
            &[(SKIPPED, Some(REFERENCE))],
            flagged,
        ))
        .await;
        let built = rig
            .build(contra_statement(
                directory.path(),
                BANK_B,
                BOOKED,
                Some(REFERENCE),
            ))
            .await;
        let carrying = built["batch_id"].as_str().unwrap().to_string();
        if found {
            mark_found(directory.path(), &carrying);
        }
        let section = &built["transfers"];
        let refused = rig.build(withdrawing(directory.path())).await;
        rig.finish();
        let case = format!("found {found}, flagged {flagged}");
        assert_eq!(refused["state"], "refused", "{case}: {refused}");
        assert_eq!(refused["reason"], "withdraw_skip_paired", "{case}");
        let state = if found {
            "paired"
        } else {
            "other_side_unverified"
        };
        assert_eq!(
            refused["paired_rows"],
            json!([{"bridge_txn_id": SKIPPED, "batch_id": carrying, "state": state}]),
            "{case}"
        );
        // what the build said of the row, before it was found, is the same reason
        if !found {
            assert_eq!(
                open_rows(section),
                vec![open_row(SKIPPED, "other_side_unverified", Some(&carrying))],
                "{case}: {built}"
            );
        }
        let step = refused["next_step"].as_str().unwrap();
        if found {
            assert_eq!(
                step,
                "No file was written and nothing was recorded. The other account's statement already carries this row as a Contra in the batch named in paired_rows. If that Contra is wrong, correct that statement's batch first; do not post this row.",
                "{case}"
            );
        } else {
            assert_eq!(
                step,
                "No file was written and nothing was recorded. The other account's statement may carry this row as a Contra in the batch named in paired_rows, which is not verified yet. Run verify_import on that batch first (if it was never imported and will not be, build that statement again, import it and verify it); do not post this row.",
                "{case}"
            );
        }
    }
}

#[tokio::test]
async fn a_skip_two_contras_could_carry_can_be_withdrawn_beside_a_paired_one() {
    const PAIRED: &str = "st-20260901-00000000000000d1";
    const THIRD: &str = "st-20260901-00000000000000c3";
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [
            skip_only_plans(),
            two_bank_plans(),
            skip_only_plans(),
            two_bank_plans(),
        ]
        .concat(),
    );
    // a row that pairs by reference, and one that two Contras could carry
    let rows = [(SKIPPED, None), (PAIRED, Some(REFERENCE))];
    rig.build(a_skips(directory.path(), &rows)).await;
    // bank B's statement books three equal Contras, all verified
    let built = rig
        .build(statement_v2(
            directory.path(),
            BANK_B,
            window(),
            json!([
                contra_voucher(BOOKED),
                contra_voucher(OTHER_BOOKED),
                contra_voucher(THIRD)
            ]),
            json!([
                contra_record(BOOKED, None),
                contra_record(OTHER_BOOKED, None),
                contra_record(THIRD, Some(REFERENCE))
            ]),
        ))
        .await;
    let carrying = built["batch_id"].as_str().unwrap().to_string();
    mark_found(directory.path(), &carrying);
    // the row two Contras could carry is listed with the batch that holds them
    let again = rig.build(a_skips(directory.path(), &rows)).await;
    let section = &again["transfers"];
    assert_eq!(section["skips_paired"], 1, "{again}");
    assert_eq!(section["skips_open"], 1, "{again}");
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "ambiguous", Some(&carrying))],
        "{again}"
    );
    assert_nothing_secret(section);
    let built = rig.build(withdrawing(directory.path())).await;
    rig.finish();
    assert_eq!(built["voucher_count"], 1, "{built}");
    // the withdrawn row is held by the new batch
    let rows = open_rows(&built["transfers"]);
    assert!(rows
        .iter()
        .any(|row| row["bridge_txn_id"] == SKIPPED && row["state"] == "held_elsewhere"));
}

#[tokio::test]
async fn every_key_a_withdrawn_row_is_skipped_under_is_read() {
    // the same row skipped again through another endpoint, whose key sorts
    // first and reads not_established: the key that pairs still refuses
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [skip_only_plans(), two_bank_plans(), admission_plans()].concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, Some(REFERENCE))]))
        .await;
    let recorded = fs::read_to_string(skip_file(directory.path())).unwrap();
    let mut again: Value = serde_json::from_str(recorded.lines().next().unwrap()).unwrap();
    again["endpoint_origin"] = json!("http://127.0.0.1:1");
    again["record_id"] = json!(format!("skips-{}", uuid::Uuid::new_v4()));
    fs::write(skip_file(directory.path()), format!("{recorded}{again}\n")).unwrap();
    let built = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            BOOKED,
            Some(REFERENCE),
        ))
        .await;
    let carrying = built["batch_id"].as_str().unwrap().to_string();
    mark_found(directory.path(), &carrying);
    let refused = rig.build(withdrawing(directory.path())).await;
    rig.finish();
    assert_eq!(refused["reason"], "withdraw_skip_paired", "{refused}");
    assert_eq!(
        refused["paired_rows"],
        json!([{"bridge_txn_id": SKIPPED, "batch_id": carrying, "state": "paired"}])
    );
}

/// Bank B's statement booking the transfer as a Contra with no reference, read
/// over `coverage`.
fn b_contra_read_over(directory: &Path, coverage: Value) -> Value {
    statement_v2(
        directory,
        BANK_B,
        coverage,
        json!([contra_voucher(BOOKED)]),
        json!([contra_record(BOOKED, None)]),
    )
}

const MATCHED_UNCONFIRMED_STEP: &str = "A verified Contra of the same day and amount, booked from the other account's statement, is in the batch named. Whether it is the only one is not established, because the statement records of the two accounts do not both cover this date. Keep the skip. The row can be decided only once statement records of both accounts cover this date: for each account that needs a statement whose rows begin before this date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on this date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). Until then the row stays as it is.";

#[tokio::test]
async fn a_skip_one_verified_contra_matches_by_date_is_not_withdrawn_whatever_the_windows_cover() {
    // the other statement read over a window that holds the day, and read with
    // none: the Contra is verified and matches the row in both
    for (coverage, state, step) in [
        (
            window(),
            "paired",
            "No file was written and nothing was recorded. The other account's statement already carries this row as a Contra in the batch named in paired_rows. If that Contra is wrong, correct that statement's batch first; do not post this row.",
        ),
        (
            Value::Null,
            "matched_unconfirmed",
            "No file was written and nothing was recorded. A verified Contra of the same day and amount as this row, booked from the other account's statement, is in the batch named in paired_rows. Whether it is the only one is not established, so the skip stays: do not post this row. The row can be decided only once statement records of both accounts cover its date: for each account that needs a statement whose rows begin before that date and end after it, built and, where it books a voucher, verified (a statement that begins or ends on that date does not cover it, and a day blocked by a batch that holds no statement records is covered by none). Until then the row stays as it is.",
        ),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let rig = Rig::new(
            directory.path(),
            [skip_only_plans(), two_bank_plans(), admission_plans()].concat(),
        );
        rig.build(a_skips(directory.path(), &[(SKIPPED, None)]))
            .await;
        let built = rig
            .build(b_contra_read_over(directory.path(), coverage))
            .await;
        assert_eq!(built["voucher_count"], 1, "{state}: {built}");
        let carrying = built["batch_id"].as_str().unwrap().to_string();
        mark_found(directory.path(), &carrying);
        let refused = rig.build(withdrawing(directory.path())).await;
        assert_eq!(refused["state"], "refused", "{state}: {refused}");
        assert_eq!(refused["reason"], "withdraw_skip_paired", "{state}");
        assert_eq!(
            refused["paired_rows"],
            json!([{"bridge_txn_id": SKIPPED, "batch_id": carrying, "state": state}]),
            "{state}"
        );
        assert_eq!(refused["paired_rows_omitted"], 0, "{state}");
        assert_eq!(refused["next_step"], step, "{state}");
        rig.finish();
    }
}

#[tokio::test]
async fn a_date_match_whose_days_the_windows_do_not_cover_is_listed_with_the_contras_batch() {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [skip_only_plans(), two_bank_plans(), skip_only_plans()].concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, None)]))
        .await;
    let built = rig
        .build(b_contra_read_over(directory.path(), Value::Null))
        .await;
    let carrying = built["batch_id"].as_str().unwrap().to_string();
    mark_found(directory.path(), &carrying);
    let again = rig
        .build(a_skips(directory.path(), &[(SKIPPED, None)]))
        .await;
    rig.finish();
    let section = &again["transfers"];
    assert_eq!(section["state"], "computed", "{again}");
    // open, not paired: the row is listed with the batch that holds the Contra
    assert_eq!(section["skips_paired"], 0, "{again}");
    assert_eq!(section["skips_open"], 1, "{again}");
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "matched_unconfirmed", Some(&carrying))],
        "{again}"
    );
    assert_eq!(
        section["next_steps"],
        json!({"matched_unconfirmed": MATCHED_UNCONFIRMED_STEP})
    );
    assert_nothing_secret(section);
}

const OTHER_REFERENCE: &str = "600000000002";

const REFERENCE_DIFFERS_STEP: &str = "A Contra of the same day and amount in the batch named, booked from the other account's statement, carries a different reference from this row. ComplyEaze Bridge cannot tell whether it is this row's other side. If that batch is not verified yet, run verify_import on it first. Then check that Contra in Tally: keep the skip if it carries this transfer, and name the row in withdraw_skips only when the person confirms it does not.";

#[tokio::test]
async fn a_skip_is_not_withdrawn_while_a_contra_of_its_day_and_amount_with_another_reference_is_unverified(
) {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [skip_only_plans(), two_bank_plans(), admission_plans()].concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, Some(REFERENCE))]))
        .await;
    let built = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            BOOKED,
            Some(OTHER_REFERENCE),
        ))
        .await;
    assert_eq!(built["voucher_count"], 1, "{built}");
    let other = built["batch_id"].as_str().unwrap().to_string();
    let refused = rig.build(withdrawing(directory.path())).await;
    assert_eq!(refused["state"], "refused", "{refused}");
    assert_eq!(refused["reason"], "withdraw_skip_paired", "{refused}");
    assert_eq!(
        refused["paired_rows"],
        json!([{"bridge_txn_id": SKIPPED, "batch_id": other, "state": "other_side_unverified"}])
    );
    assert_eq!(
        refused["next_step"],
        "No file was written and nothing was recorded. The other account's statement may carry this row as a Contra in the batch named in paired_rows, which is not verified yet. Run verify_import on that batch first (if it was never imported and will not be, build that statement again, import it and verify it); do not post this row."
    );
    rig.finish();
    // what the other statement's build said of the row: listed with that
    // batch, under its own state and step
    let section = &built["transfers"];
    assert_eq!(section["skips_open"], 1, "{built}");
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "reference_differs", Some(&other))],
        "{built}"
    );
    assert_eq!(
        section["next_steps"],
        json!({"reference_differs": REFERENCE_DIFFERS_STEP})
    );
    assert_nothing_secret(section);
    assert!(!section.to_string().contains(OTHER_REFERENCE));
}

#[tokio::test]
async fn a_skip_beside_a_verified_contra_of_another_reference_is_listed_and_withdrawn_on_the_persons_word(
) {
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [
            skip_only_plans(),
            two_bank_plans(),
            skip_only_plans(),
            two_bank_plans(),
        ]
        .concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, Some(REFERENCE))]))
        .await;
    let built = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            BOOKED,
            Some(OTHER_REFERENCE),
        ))
        .await;
    let other = built["batch_id"].as_str().unwrap().to_string();
    mark_found(directory.path(), &other);
    // the row is not said to have no Contra: it is listed with the batch that
    // holds the Contra of another reference
    let again = rig
        .build(a_skips(directory.path(), &[(SKIPPED, Some(REFERENCE))]))
        .await;
    let section = &again["transfers"];
    assert_eq!(section["skips_paired"], 0, "{again}");
    assert_eq!(section["skips_open"], 1, "{again}");
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "reference_differs", Some(&other))],
        "{again}"
    );
    assert_eq!(
        section["next_steps"],
        json!({"reference_differs": REFERENCE_DIFFERS_STEP})
    );
    assert_nothing_secret(section);
    // that Contra is verified, so the person may name the row: the build is
    // written, and the row is then held by the new batch
    let mut posting = contra_statement(directory.path(), BANK_A, SKIPPED, Some(REFERENCE));
    posting["withdraw_skips"] = json!([SKIPPED]);
    let posted = rig.build(posting).await;
    rig.finish();
    assert_eq!(posted["voucher_count"], 1, "{posted}");
    let holder = posted["batch_id"].as_str().unwrap();
    assert_eq!(
        open_rows(&posted["transfers"]),
        vec![open_row(SKIPPED, "held_elsewhere", Some(holder))],
        "{posted}"
    );
}

#[tokio::test]
async fn a_row_without_a_reference_pairs_by_date_beside_a_row_of_another_reference() {
    const UNREFERENCED: &str = "st-20260901-00000000000000b0";
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [
            skip_only_plans(),
            two_bank_plans(),
            skip_only_plans(),
            admission_plans(),
        ]
        .concat(),
    );
    let rows = [(UNREFERENCED, None), (SKIPPED, Some(REFERENCE))];
    rig.build(a_skips(directory.path(), &rows)).await;
    let built = rig
        .build(contra_statement(
            directory.path(),
            BANK_B,
            BOOKED,
            Some(OTHER_REFERENCE),
        ))
        .await;
    let carrying = built["batch_id"].as_str().unwrap().to_string();
    mark_found(directory.path(), &carrying);
    // the one Contra pairs by date with the row that prints no reference, and
    // is then no Contra of the other row
    let again = rig.build(a_skips(directory.path(), &rows)).await;
    let section = &again["transfers"];
    assert_eq!(section["skips_paired"], 1, "{again}");
    assert_eq!(section["skips_open"], 1, "{again}");
    assert_eq!(
        open_rows(section),
        vec![open_row(SKIPPED, "not_found", None)],
        "{again}"
    );
    // and the paired row is not one to withdraw
    let mut posting = contra_statement(directory.path(), BANK_A, UNREFERENCED, None);
    posting["withdraw_skips"] = json!([UNREFERENCED]);
    let refused = rig.build(posting).await;
    rig.finish();
    assert_eq!(refused["reason"], "withdraw_skip_paired", "{refused}");
    assert_eq!(
        refused["paired_rows"],
        json!([{"bridge_txn_id": UNREFERENCED, "batch_id": carrying, "state": "paired"}])
    );
}

/// Bank B's statement building `rows` as Contras, none with a reference.
fn b_contras(directory: &Path, rows: &[&str]) -> Value {
    statement_v2(
        directory,
        BANK_B,
        window(),
        rows.iter()
            .map(|row| contra_voucher(row))
            .collect::<Vec<_>>()
            .into(),
        rows.iter()
            .map(|row| contra_record(row, None))
            .collect::<Vec<_>>()
            .into(),
    )
}

#[tokio::test]
async fn a_skip_a_contra_not_yet_verified_may_carry_cannot_be_withdrawn_in_any_state() {
    // ambiguous between two unsent builds of one statement; ambiguous between a
    // Contra found posted and one not; reversed the same day with a Contra found
    // posted and one not. In each the person cannot confirm in Tally what is
    // not there yet.
    for case in ["two unsent", "one found, one not", "reversed"] {
        let directory = tempfile::tempdir().unwrap();
        let rig = Rig::new(
            directory.path(),
            [
                skip_only_plans(),
                two_bank_plans(),
                two_bank_plans(),
                admission_plans(),
            ]
            .concat(),
        );
        rig.build(a_skips_flagged(
            directory.path(),
            &[(SKIPPED, None)],
            case == "reversed",
        ))
        .await;
        let first = rig.build(b_contras(directory.path(), &[BOOKED])).await;
        let first = first["batch_id"].as_str().unwrap().to_string();
        let second_row = if case == "two unsent" {
            BOOKED
        } else {
            OTHER_BOOKED
        };
        let second_built = rig.build(b_contras(directory.path(), &[second_row])).await;
        let second = second_built["batch_id"].as_str().unwrap().to_string();
        // the second build lists the row once for each of the two batches
        let section = &second_built["transfers"];
        let state = if case == "reversed" {
            "reversed_same_day"
        } else {
            "ambiguous"
        };
        let mut listed = vec![
            open_row(SKIPPED, state, Some(&first)),
            open_row(SKIPPED, state, Some(&second)),
        ];
        listed.sort_by_key(|row| row["batch_id"].as_str().unwrap().to_string());
        assert_eq!(open_rows(section), listed, "{case}: {second_built}");
        assert_eq!(section["skips_open"], 1, "{case}: {second_built}");
        assert_nothing_secret(section);
        if case != "two unsent" {
            mark_found(directory.path(), &first);
        }
        let refused = rig.build(withdrawing(directory.path())).await;
        rig.finish();
        assert_eq!(
            refused["reason"], "withdraw_skip_paired",
            "{case}: {refused}"
        );
        let unverified = |batch: &str| json!({"bridge_txn_id": SKIPPED, "batch_id": batch, "state": "other_side_unverified"});
        let mut expected = if case == "two unsent" {
            vec![unverified(&first), unverified(&second)]
        } else {
            vec![unverified(&second)]
        };
        expected.sort_by_key(|row| row["batch_id"].as_str().unwrap().to_string());
        assert_eq!(refused["paired_rows"], json!(expected), "{case}");
        assert_eq!(
            refused["next_step"],
            "No file was written and nothing was recorded. The other account's statement may carry this row as a Contra in the batch named in paired_rows, which is not verified yet. Run verify_import on that batch first (if it was never imported and will not be, build that statement again, import it and verify it); do not post this row.",
            "{case}"
        );
    }
}

#[tokio::test]
async fn a_transfer_posted_inline_leaves_the_other_skip_not_established_rather_than_not_found() {
    // the same transfer skipped on both statements; bank B's skip is withdrawn
    // through a build of inline vouchers, which records no statement
    const B_SKIPPED: &str = "st-20260901-00000000000000b2";
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [skip_only_plans(), skip_only_plans(), two_bank_plans()].concat(),
    );
    rig.build(a_skips(directory.path(), &[(SKIPPED, None)]))
        .await;
    let mut b_skip = record(B_SKIPPED, json!("skipped"), BANK_A, "inflow", false);
    b_skip["reference"] = Value::Null;
    rig.build(statement_v2(
        directory.path(),
        BANK_B,
        window(),
        json!([]),
        json!([b_skip]),
    ))
    .await;
    let built = rig
        .build(json!({"company_guid": CAPTURED_GUID,
            "vouchers": [contra_voucher(B_SKIPPED)], "withdraw_skips": [B_SKIPPED]}))
        .await;
    rig.finish();
    let batch = built["batch_id"].as_str().unwrap();
    // bank A's skip is not said to be absent from bank B's statement: the
    // Contra that carries it is one the recorded facts cannot see
    assert_eq!(
        open_rows(&built["transfers"]),
        vec![
            open_row(SKIPPED, "not_established", None),
            open_row(B_SKIPPED, "held_elsewhere", Some(batch)),
        ],
        "{built}"
    );
}

// ---- a small response cap, hundreds of open rows --------------------------------

const SMALL_CAP: usize = 20_000;

fn server_with_cap(directory: &Path, port: u16, max_bytes: usize) -> Server {
    Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port,
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

/// A statement of only skipped rows, hundreds of them.
async fn record_many_skips(directory: &Path, count: u32) {
    let ids: Vec<String> = (0..count)
        .map(|n| format!("st-20260901-{n:016x}"))
        .collect();
    let rows: Vec<(&str, Option<&str>)> = ids.iter().map(|id| (id.as_str(), None)).collect();
    let result = run(directory, skip_only_plans(), a_skips(directory, &rows)).await;
    assert_eq!(result["state"], "skips_recorded", "{result}");
    assert_eq!(result["transfers"]["skips_open"], count, "{result}");
}

#[tokio::test]
async fn a_build_at_a_small_cap_cuts_the_open_rows_and_is_not_too_large() {
    let directory = tempfile::tempdir().unwrap();
    record_many_skips(directory.path(), 400).await;
    let simulator = SequenceSimulator::spawn(qualified_import_cycle_plans()).expect("simulator");
    let server = server_with_cap(directory.path(), simulator.address().port(), SMALL_CAP);
    let response = server
        .call_tool_response(
            "build_import_xml",
            serde_json::to_value(captured_catalogue_payload()).unwrap(),
        )
        .await;
    let value = &response.value;
    assert_ne!(value["isError"], true, "{value}");
    let result = &value["structuredContent"]["result"];
    assert!(result["batch_id"].is_string(), "{value}");
    let section = &result["transfers"];
    assert_eq!(section["skips_open"], 400, "{value}");
    assert!(section["open_omitted"].as_u64().unwrap() > 0, "{value}");
    assert_eq!(
        section["open"].as_array().unwrap().len() as u64
            + section["open_omitted"].as_u64().unwrap(),
        400
    );
    assert!(
        section["next_steps"]["open_omitted"]
            .as_str()
            .unwrap()
            .contains("BRIDGE_AGENT_MAX_BYTES"),
        "{value}"
    );
    assert!(
        value.to_string().len() <= SMALL_CAP,
        "{}",
        value.to_string().len()
    );
}

#[tokio::test]
async fn a_verification_at_a_small_cap_cuts_the_open_rows_and_keeps_its_verdict() {
    let directory = tempfile::tempdir().unwrap();
    record_many_skips(directory.path(), 400).await;
    let simulator = SequenceSimulator::spawn(qualified_import_cycle_plans()).expect("simulator");
    let wide = server_with_cap(directory.path(), simulator.address().port(), 200_000);
    let built = wide
        .build_import_xml(&serde_json::to_value(captured_catalogue_payload()).unwrap())
        .await
        .expect("build");
    let batch_id = built.payload["result"]["batch_id"]
        .as_str()
        .unwrap()
        .to_string();
    let args = json!({"company_guid": CAPTURED_GUID, "batch_id": batch_id});
    let small = server_with_cap(directory.path(), simulator.address().port(), SMALL_CAP);
    let first = small.call_tool_response("verify_import", args).await.value;
    assert_ne!(first["isError"], true, "{first}");
    let page = &first["structuredContent"]["result"];
    let section = &page["transfers"];
    assert_eq!(section["skips_open"], 400, "{first}");
    assert!(section["open_omitted"].as_u64().unwrap() > 0, "{first}");
    assert!(first.to_string().len() <= SMALL_CAP);
    // the verdict is the persisted proof's, whatever the section lost
    let [proof_path, _] = small.current_proof_paths(&batch_id);
    let proof: Value = serde_json::from_slice(&fs::read(proof_path).unwrap()).unwrap();
    assert_eq!(page["verification_status"], proof["verification_status"]);
    assert_eq!(page["counts"], proof["counts"]);
}

/// The facts hold a row by its id, which already hashes the account, so a live
/// skip of the row through another endpoint still refuses the posting: the
/// posting would otherwise retire that skip without the other account's word.
#[tokio::test]
async fn a_live_skip_of_the_row_through_another_endpoint_refuses_the_posting() {
    let directory = tempfile::tempdir().unwrap();
    // the skip is recorded through one endpoint, and a build through another
    // posts the same row
    run(
        directory.path(),
        skip_only_plans(),
        a_skips(directory.path(), &[(SKIPPED, None)]),
    )
    .await;
    let posting = contra_statement(directory.path(), BANK_A, SKIPPED, None);
    let refused = run(directory.path(), admission_plans(), posting).await;
    assert_eq!(refused["reason"], "skip_row_paired_elsewhere", "{refused}");
    assert_eq!(refused["skipped_rows"], json!([SKIPPED]));
    // and a withdrawal of it, named by the person, is for a live skip
    let mut withdrawn = contra_statement(directory.path(), BANK_A, SKIPPED, None);
    withdrawn["withdraw_skips"] = json!([SKIPPED]);
    let built = run(directory.path(), two_bank_plans(), withdrawn).await;
    assert_eq!(built["voucher_count"], 1, "{built}");
}

/// The step is chosen from every carried row, not only from the rows listed.
#[tokio::test]
async fn the_step_for_carried_rows_counts_the_rows_the_list_leaves_out() {
    let skipped = |n: u32| format!("st-20260901-{:016x}", 0x1000 + n);
    let booked = |n: u32| format!("st-20260901-{:016x}", 0x2000 + n);
    let reference = |n: u32| format!("6000{:08}", n + 1);
    let directory = tempfile::tempdir().unwrap();
    let rig = Rig::new(
        directory.path(),
        [
            skip_only_plans(),
            two_bank_plans(),
            two_bank_plans(),
            admission_plans(),
        ]
        .concat(),
    );
    // 53 rows skipped, each with its own reference
    let ids: Vec<String> = (0..53).map(skipped).collect();
    let refs: Vec<String> = (0..53).map(reference).collect();
    let rows: Vec<(&str, Option<&str>)> = ids
        .iter()
        .zip(&refs)
        .map(|(id, reference)| (id.as_str(), Some(reference.as_str())))
        .collect();
    rig.build(a_skips(directory.path(), &rows)).await;
    // the first 52 are carried by a batch found posted, the last by one not yet
    // verified, which the list of the first 50 does not reach
    let contras = |range: std::ops::Range<u32>| {
        statement_v2(
            directory.path(),
            BANK_B,
            window(),
            range
                .clone()
                .map(|n| contra_voucher(&booked(n)))
                .collect::<Vec<_>>()
                .into(),
            range
                .map(|n| contra_record(&booked(n), Some(&reference(n))))
                .collect::<Vec<_>>()
                .into(),
        )
    };
    let found = rig.build(contras(0..52)).await;
    mark_found(directory.path(), found["batch_id"].as_str().unwrap());
    rig.build(contras(52..53)).await;
    let mut posting = statement_v2(
        directory.path(),
        BANK_A,
        window(),
        ids.iter()
            .map(|id| contra_voucher(id))
            .collect::<Vec<_>>()
            .into(),
        ids.iter()
            .map(|id| contra_record(id, None))
            .collect::<Vec<_>>()
            .into(),
    );
    posting["withdraw_skips"] = json!(ids);
    let refused = rig.build(posting).await;
    rig.finish();
    assert_eq!(refused["reason"], "withdraw_skip_paired", "{refused}");
    let listed = refused["paired_rows"].as_array().unwrap();
    assert_eq!(listed.len(), 50);
    assert!(listed.iter().all(|row| row["state"] == "paired"));
    assert_eq!(refused["paired_rows_omitted"], 3);
    // all listed rows are paired, yet the step covers the row not listed
    let step = refused["next_step"].as_str().unwrap();
    assert!(step.contains("state other_side_unverified"), "{step}");
    assert!(step.contains("state paired"), "{step}");
    // it is the step of both kinds among the refused rows, and of no other
    assert_eq!(
        step,
        transfers::Carrier::refusal_step(&BTreeSet::from([
            transfers::Carrier::Paired,
            transfers::Carrier::OtherSideUnverified,
        ]))
    );
}
