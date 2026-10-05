//! #1234 slice 1 through `build_import_xml`: the flag reads, the per-party
//! refusal and approval, and what a build records.
//!
//! The scripted flag responses are regression doubles (`bill_wise_flag_plans`):
//! their names and parents are the live catalogue capture's and the flag column
//! is set by each test. They prove Bridge's own rules, not what Tally answers.
use super::*;

const PARTY: &str = "Bridge Nested Debtor WR4";
const SALES: &str = "WR2 Sales";

fn server(directory: &std::path::Path, port: u16, max_bytes: usize) -> Server {
    Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port,
        },
        data_dir: directory.into(),
        max_rows: 10,
        max_bytes,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

fn build_args(approvals: Option<Value>) -> Value {
    let mut args = serde_json::to_value(captured_catalogue_payload()).unwrap();
    if let Some(approvals) = approvals {
        args["on_account_approvals"] = approvals;
    }
    args
}

/// The requests a run made, whatever the plans left over.
fn requests_made(simulator: SequenceSimulator) -> usize {
    simulator.cancel();
    simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .count()
}

/// One server and one simulator for a refusal and the build its approval
/// allows. The digest binds the endpoint, port included, so both calls must
/// reach the same one.
fn sitting(plans: Vec<ScenarioPlan>) -> (tempfile::TempDir, SequenceSimulator, Server) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    (directory, simulator, server)
}

fn digest_of_first_party(refusal: &ToolOutcome) -> String {
    refusal.payload["result"]["refused_parties"][0]["party_digest"]
        .as_str()
        .expect("a refused party carries its digest")
        .to_string()
}

fn party_name_of(value: &Value) -> &str {
    value["$bridge_agent_party_name"]
        .as_str()
        .expect("a party name travels marked for egress")
}

/// Builds against a book whose `bill_wise` ledgers are marked Yes in both flag
/// reads, stopping after the first flag read (18 requests).
async fn refusal(bill_wise: &[&str], max_bytes: usize) -> (Value, usize) {
    let plans = journal_build_plans(
        bill_wise_flag_plans(bill_wise),
        bill_wise_flag_plans(bill_wise),
    );
    let simulator = SequenceSimulator::spawn(plans[..18].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), max_bytes);
    let built = server.build_import_xml(&build_args(None)).await.unwrap();
    assert!(!directory.path().join("imports").exists());
    assert!(!directory.path().join("agent-import-ledger.jsonl").exists());
    (built.payload["result"].clone(), requests_made(simulator))
}

#[tokio::test]
async fn an_unapproved_bill_wise_party_refuses_the_build_and_lists_its_rows() {
    let (result, requests) = refusal(&[PARTY], 200_000).await;
    // Probe, identity, catalogue and the first flag read; nothing after it.
    assert_eq!(requests, 18);
    assert_eq!(result["state"], "refused");
    assert_eq!(result["reason"], "bill_wise_party_unapproved");
    assert_eq!(result["refused_party_count"], 1);
    assert_eq!(result["refused_parties_omitted"], 0);
    let party = &result["refused_parties"][0];
    assert_eq!(party_name_of(&party["ledger"]), PARTY);
    assert_eq!(party["row_count"], 1);
    assert_eq!(party["rows_omitted"], 0);
    // The row the person is asked about carries its date and amount.
    let row = &party["rows"][0];
    assert_eq!(row["bridge_txn_id"], "txn-001");
    assert_eq!(row["voucher_type"], "Journal");
    assert_eq!(row["date"], "20260901");
    assert_eq!(row["entries"][0]["side"], "Dr");
    assert_eq!(row["entries"][0]["amount"], "12.50");
    assert_eq!(party["debit_total"], "12.5");
    assert_eq!(party["credit_total"], "0");
    let digest = party["party_digest"].as_str().unwrap();
    assert_eq!(digest.len(), 64);
    assert_eq!(
        result["bill_wise_response_sha256"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    // The text the assistant relays says what the digest does not prove.
    let next_step = result["next_step"].as_str().unwrap();
    for phrase in [
        "does not prove that a person said yes",
        "never approve on the person's behalf",
        "one party per question",
        "hand import of the file is not checked at all",
        "The native approval dialog does not yet show these entries",
        "importing it also replaces any bill allocations",
    ] {
        assert!(next_step.contains(phrase), "{phrase}");
    }
    // It must not suggest a person's own dialog already covers these entries.
    assert!(!next_step.contains("shows the person its own dialog"));
}

#[tokio::test]
async fn a_party_with_no_bill_wise_ledger_builds_without_an_approval_and_records_none() {
    let simulator =
        SequenceSimulator::spawn(qualified_import_cycle_plans()[..44].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let built = server.build_import_xml(&build_args(None)).await.unwrap();
    let result = &built.payload["result"];
    assert_eq!(result["on_account_approved"], json!([]));
    // Both reads' responses are on record, in the order they were read.
    assert_eq!(
        result["bill_wise_response_sha256"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    let note = result["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .find(|text| text.starts_with("Checked: none of the ledgers"))
        .expect("the build says what it checked");
    assert!(note.contains("as read from Tally during this build"));
    let saved = server.import_ledger().unwrap().pop().unwrap();
    assert_eq!(saved.on_account_approved, Some(Vec::new()));
    assert_eq!(simulator.finish().unwrap().len(), 44);
}

#[tokio::test]
async fn an_approved_party_builds_and_the_record_carries_the_approval() {
    let yes = || {
        journal_build_plans(
            bill_wise_flag_plans(&[PARTY]),
            bill_wise_flag_plans(&[PARTY]),
        )
    };
    let (_directory, simulator, server) = sitting([yes()[..18].to_vec(), yes()].concat());
    let refused = server.build_import_xml(&build_args(None)).await.unwrap();
    let digest = digest_of_first_party(&refused);
    let approvals = json!([{"ledger": PARTY, "party_digest": digest}]);
    let built = server
        .build_import_xml(&build_args(Some(approvals)))
        .await
        .unwrap();
    let result = &built.payload["result"];
    assert_eq!(
        result["on_account_approved"],
        json!([{"ledger": PARTY, "party_digest": digest}])
    );
    let note = result["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .find(|text| text.contains("on_account_approved"))
        .expect("the build says an entry lands On Account");
    assert!(note.contains("cannot tell whether a person said yes"));
    assert!(!note.contains("approved in chat"));
    let saved = server.import_ledger().unwrap().pop().unwrap();
    assert_eq!(
        saved.on_account_approved,
        Some(vec![bill_wise::OnAccountApproved {
            ledger: PARTY.into(),
            party_digest: digest,
        }])
    );
    assert_eq!(simulator.finish().unwrap().len(), 18 + 44);
}

#[tokio::test]
async fn approving_one_of_two_parties_refuses_the_other() {
    let both = || {
        journal_build_plans(
            bill_wise_flag_plans(&[PARTY, SALES]),
            bill_wise_flag_plans(&[PARTY, SALES]),
        )[..18]
            .to_vec()
    };
    let (directory, _simulator, server) = sitting([both(), both()].concat());
    let refused = server.build_import_xml(&build_args(None)).await.unwrap();
    assert_eq!(refused.payload["result"]["refused_party_count"], 2);
    let first = refused.payload["result"]["refused_parties"]
        .as_array()
        .unwrap()
        .iter()
        .find(|party| party_name_of(&party["ledger"]) == PARTY)
        .unwrap();
    let digest = first["party_digest"].as_str().unwrap().to_string();
    let approvals = json!([{"ledger": PARTY, "party_digest": digest}]);
    let built = server
        .build_import_xml(&build_args(Some(approvals)))
        .await
        .unwrap();
    let result = &built.payload["result"];
    assert_eq!(result["reason"], "bill_wise_party_unapproved");
    assert_eq!(result["refused_party_count"], 1);
    assert_eq!(
        party_name_of(&result["refused_parties"][0]["ledger"]),
        SALES
    );
    assert!(!directory.path().join("imports").exists());
}

#[tokio::test]
async fn the_gate_counts_parties_not_the_rows_a_small_response_cap_leaves_out() {
    // At a 256-byte cap the refusal's display budget is 64 bytes: no party
    // header fits, so nothing is listed, and the build must still refuse.
    let (result, _) = refusal(&[PARTY], 256).await;
    assert_eq!(result["reason"], "bill_wise_party_unapproved");
    assert_eq!(result["refused_party_count"], 1);
    assert_eq!(result["refused_parties"], json!([]));
    assert_eq!(result["refused_parties_omitted"], 1);
}

async fn invalid_approval(approvals: Value, plans: usize) -> (&'static str, usize) {
    let all = journal_build_plans(
        bill_wise_flag_plans(&[PARTY]),
        bill_wise_flag_plans(&[PARTY]),
    );
    let simulator = SequenceSimulator::spawn(all[..plans].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let failure = server
        .build_import_xml(&build_args(Some(approvals)))
        .await
        .err()
        .expect("an invalid approval is a failure");
    assert_eq!(failure.code, "on_account_approval_invalid");
    assert!(!directory.path().join("imports").exists());
    assert!(!directory.path().join("agent-import-ledger.jsonl").exists());
    (failure.cause.unwrap(), requests_made(simulator))
}

#[tokio::test]
async fn an_approval_that_is_malformed_or_repeated_is_refused_before_any_read() {
    let digest = "a".repeat(64);
    assert_eq!(invalid_approval(json!("nope"), 2).await, ("malformed", 0));
    assert_eq!(
        invalid_approval(
            json!([{"ledger": PARTY, "party_digest": digest}, {"ledger": PARTY, "party_digest": digest}]),
            2
        )
        .await,
        ("duplicate", 0)
    );
}

#[tokio::test]
async fn an_approval_for_a_ledger_that_is_not_a_party_or_with_another_digest_is_refused() {
    let digest = "a".repeat(64);
    // `Cash` is in the batch but is not bill-wise, so it is not a party.
    assert_eq!(
        invalid_approval(json!([{"ledger": "Cash", "party_digest": digest}]), 18).await,
        ("unknown_ledger", 18)
    );
    assert_eq!(
        invalid_approval(json!([{"ledger": PARTY, "party_digest": digest}]), 18).await,
        ("digest_differs", 18)
    );
}

#[tokio::test]
async fn a_flag_that_changes_between_the_two_reads_is_refused() {
    let yes = journal_build_plans(
        bill_wise_flag_plans(&[PARTY]),
        bill_wise_flag_plans(&[PARTY]),
    );
    // Bill-wise at the first read, not at the second: approved on one
    // observation, then withdrawn.
    let changing = journal_build_plans(bill_wise_flag_plans(&[PARTY]), bill_wise_flag_plans(&[]));
    let (directory, simulator, server) =
        sitting([yes[..18].to_vec(), changing[..38].to_vec()].concat());
    let refused = server.build_import_xml(&build_args(None)).await.unwrap();
    let digest = digest_of_first_party(&refused);
    let approvals = json!([{"ledger": PARTY, "party_digest": digest}]);
    let failure = server
        .build_import_xml(&build_args(Some(approvals)))
        .await
        .err()
        .unwrap();
    assert_eq!(failure.code, "import_bill_wise_changed");
    assert!(!directory.path().join("imports").exists());
    assert!(!directory.path().join("agent-import-ledger.jsonl").exists());
    // Through the second flag read (requests 30 to 35 of the build) and no
    // further: the pre-flight window is never read.
    assert_eq!(requests_made(simulator), 18 + 36);
}

#[tokio::test]
async fn a_flag_that_changes_on_a_ledger_the_batch_does_not_name_is_not_a_change() {
    // Café Naïve Traders is in the book and not in the batch: it turns
    // bill-wise between the two reads, and the build still goes through.
    let plans = journal_build_plans(
        bill_wise_flag_plans(&[]),
        bill_wise_flag_plans(&["Café Naïve Traders"]),
    );
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let built = server.build_import_xml(&build_args(None)).await.unwrap();
    assert_eq!(built.payload["result"]["on_account_approved"], json!([]));
    assert_eq!(simulator.finish().unwrap().len(), 44);
}

/// The flag plans with the first row's name changed, so the row count matches
/// the catalogue and a requested ledger is in none of the rows.
fn flag_plans_renaming(from: &str, to: &str) -> Vec<ScenarioPlan> {
    let mut plans = bill_wise_flag_plans(&[]);
    for index in [1, 3] {
        let body = plans[index].fixture.body().into_owned();
        let renamed = body.replace(&format!("NAME=\"{from}\""), &format!("NAME=\"{to}\""));
        assert_ne!(renamed, body, "the rename must apply");
        plans[index].fixture = Fixture::SyntheticXml(renamed);
    }
    plans
}

async fn not_established(first_flags: Vec<ScenarioPlan>) -> Value {
    let plans = journal_build_plans(first_flags, bill_wise_flag_plans(&[]));
    let simulator = SequenceSimulator::spawn(plans[..18].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let built = server.build_import_xml(&build_args(None)).await.unwrap();
    assert!(!directory.path().join("imports").exists());
    assert_eq!(requests_made(simulator), 18);
    built.payload["result"].clone()
}

#[tokio::test]
async fn a_named_ledger_the_flag_read_does_not_return_is_not_established() {
    let result = not_established(flag_plans_renaming(PARTY, "Another Ledger")).await;
    assert_eq!(result["state"], "refused");
    assert_eq!(result["reason"], "bill_wise_not_established");
    assert_eq!(result["cause"], "ledger_absent");
    assert_eq!(
        result["message"],
        bill_wise::BillWiseError::LedgerAbsent.plain()
    );
    assert!(result["next_step"]
        .as_str()
        .unwrap()
        .contains("Do not drop a ledger"));
}

#[tokio::test]
async fn a_flag_read_with_another_row_count_than_the_catalogue_is_not_established() {
    let mut plans = bill_wise_flag_plans(&[]);
    for index in [1, 3] {
        let body = plans[index].fixture.body().into_owned();
        let start = body.find("<LEDGER NAME=\"Cash\"").unwrap();
        let end = start + body[start..].find("</LEDGER>").unwrap() + "</LEDGER>".len();
        plans[index].fixture = Fixture::SyntheticXml(format!("{}{}", &body[..start], &body[end..]));
    }
    let result = not_established(plans).await;
    assert_eq!(result["reason"], "bill_wise_not_established");
    assert_eq!(result["cause"], "row_count_differs");
}

#[tokio::test]
async fn a_flag_read_that_cannot_be_parsed_is_a_failure_with_its_cause() {
    let mut plans = bill_wise_flag_plans(&[]);
    for index in [1, 3] {
        let body = plans[index].fixture.body().into_owned();
        let broken = body.replace("<ISBILLWISEON>No</ISBILLWISEON>", "");
        assert_ne!(broken, body);
        plans[index].fixture = Fixture::SyntheticXml(broken);
    }
    let all = journal_build_plans(plans, bill_wise_flag_plans(&[]));
    let simulator = SequenceSimulator::spawn(all[..18].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let failure = server
        .build_import_xml(&build_args(None))
        .await
        .err()
        .unwrap();
    assert_eq!(failure.code, "bill_wise_export_invalid");
    assert_eq!(failure.cause, Some("ledger_bill_wise_flag_missing"));
    assert!(!directory.path().join("imports").exists());
}

#[tokio::test]
async fn a_flag_read_for_another_company_is_a_failure() {
    let mut plans = bill_wise_flag_plans(&[]);
    for index in [1, 3] {
        let body = plans[index].fixture.body().into_owned();
        let other = body.replace(CAPTURED_GUID, "00000000-0000-4000-8000-0000000000ff");
        assert_ne!(other, body);
        plans[index].fixture = Fixture::SyntheticXml(other);
    }
    let all = journal_build_plans(plans, bill_wise_flag_plans(&[]));
    let simulator = SequenceSimulator::spawn(all[..18].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let failure = server
        .build_import_xml(&build_args(None))
        .await
        .err()
        .unwrap();
    assert_eq!(failure.code, "bill_wise_export_invalid");
    assert_eq!(failure.cause, Some("ledger_response_company_guid_mismatch"));
}

#[tokio::test]
async fn the_flag_request_is_the_snapshot_the_outstandings_paths_send() {
    let plans = journal_build_plans(bill_wise_flag_plans(&[]), bill_wise_flag_plans(&[]));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    server.build_import_xml(&build_args(None)).await.unwrap();
    simulator.cancel();
    let observed = simulator.finish().unwrap();
    let requests = observed
        .iter()
        .filter(|request| !request.method.is_empty())
        .collect::<Vec<_>>();
    // The flag read is the thirteenth request: after the probe (2), identity
    // (4) and the catalogue (6) come its company, flag, status, flag, status
    // and company legs; its first body is the one-day snapshot at books_from.
    let expected = sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        &bridge_tally_protocol::native_outstandings::render_native_ledger_snapshot_request(
            "WR2 Unicode Lab",
            &bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriod::new(
                bridge_tally_protocol::outstandings_shared::DateBoundaryProfile::ModeAgnostic,
                bridge_tally_core::TallyDate::parse("20260401".to_string()).unwrap(),
                bridge_tally_core::TallyDate::parse("20260401".to_string()).unwrap(),
            )
            .unwrap(),
        ),
    ));
    assert_eq!(requests[13].request_body_sha256, expected);
}

#[tokio::test]
async fn a_bank_batch_names_its_bill_wise_counterparty_and_builds_once_approved() {
    let flagged = || bank_tests::bank_build_plans_flagging(&[PARTY]);
    let simulator =
        SequenceSimulator::spawn([flagged()[..24].to_vec(), flagged()].concat()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = bank_tests::bank_server(directory.path(), simulator.address().port());
    let args = serde_json::to_value(bank_tests::captured_bank_payload()).unwrap();
    let refused = server.build_import_xml(&args).await.unwrap();
    let result = &refused.payload["result"];
    assert_eq!(result["reason"], "bill_wise_party_unapproved");
    // The Payment's Dr counterparty is the party; the Receipt names none.
    assert_eq!(result["refused_party_count"], 1);
    assert_eq!(
        result["refused_parties"][0]["rows"][0]["voucher_type"],
        "Payment"
    );
    let digest = digest_of_first_party(&refused);

    let mut args = args;
    args["on_account_approvals"] = json!([{"ledger": PARTY, "party_digest": digest}]);
    let built = server.build_import_xml(&args).await.unwrap();
    assert_eq!(
        built.payload["result"]["on_account_approved"][0]["ledger"],
        PARTY
    );
    assert_eq!(simulator.finish().unwrap().len(), 24 + 56);
}

#[test]
fn the_schema_admits_an_approval_list_and_the_build_strips_it_before_parsing() {
    let schema = super::super::voucher_input_schema();
    let approvals = &schema["properties"]["on_account_approvals"];
    assert_eq!(approvals["type"], "array");
    assert_eq!(approvals["items"]["additionalProperties"], false);
    assert_eq!(
        approvals["items"]["required"],
        json!(["ledger", "party_digest"])
    );
    assert_eq!(
        approvals["items"]["properties"]["party_digest"]["pattern"],
        "^[0-9a-f]{64}$"
    );
    // The approval's ledger is held to the entry ledger's own pattern, so a
    // name ending in CR LF can be approved.
    assert_eq!(
        approvals["items"]["properties"]["ledger"],
        schema["properties"]["vouchers"]["items"]["properties"]["entries"]["items"]["properties"]
            ["ledger"]
    );
    // The payload itself still refuses the key.
    let mut args = build_args(Some(json!([])));
    assert!(parse_payload(&args).is_err());
    bill_wise::take_approvals(&mut args).unwrap();
    assert!(parse_payload(&args).is_ok());
}

#[test]
fn the_tool_text_says_what_the_digest_does_not_prove_and_where_the_gate_is() {
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    let description = definitions
        .as_array()
        .and_then(|tools| tools.iter().find(|tool| tool["name"] == "build_import_xml"))
        .and_then(|tool| tool["description"].as_str().map(str::to_string))
        .expect("build_import_xml is registered");
    for phrase in [
        "it does NOT prove that a person said yes",
        "never approve on the person's behalf",
        "The native approval dialog does not yet list these entries",
        "a hand import of the file is not checked at all",
        "A ledger switched to bill-wise after the build is not caught before posting",
        "bill_wise_party_unapproved",
        "on_account_approvals",
        "import_batch_predates_bill_wise_record",
    ] {
        assert!(description.contains(phrase), "{phrase}");
    }
}
