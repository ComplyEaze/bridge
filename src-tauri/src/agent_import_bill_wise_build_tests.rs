//! #1234 slice 1 through `build_import_xml`: the bill-wise flag a build reads from
//! the ledger catalogue (V2 catalogue), the per-party refusal and approval, and what
//! a build records.
//!
//! The scripted catalogue answers are regression doubles (`flagged`): their
//! names, GUIDs and parents are a live capture's and the flag column is set by
//! each test. They prove Bridge's own rules, not what Tally answers; what Tally
//! answers to the V2 catalogue request is pinned on live captures by the `live_`
//! tests in `agent_import_bill_wise_tests.rs`.
use super::*;

const PARTY: &str = "Bridge Nested Debtor WR4";
const SALES: &str = "WR2 Sales";

/// A build stops after its first catalogue read, 12 requests in (the mode probe,
/// the identity and the paired catalogue read), when it refuses at the flags.
const REFUSAL_REQUESTS: usize = 12;
/// The requests of one whole Journal build.
const BUILD_REQUESTS: usize = 32;

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

/// `plans` with every ledger catalogue answer in it marked as `bill_wise` says.
fn flagged(plans: Vec<ScenarioPlan>, bill_wise: &[&str]) -> Vec<ScenarioPlan> {
    plans
        .into_iter()
        .map(|mut plan| {
            let body = plan.fixture.body().into_owned();
            if body.contains("<LEDGER NAME=\"") && body.contains("<ISBILLWISEON") {
                plan.fixture = Fixture::SyntheticXml(with_bill_wise_flags(&body, bill_wise));
            }
            plan
        })
        .collect()
}

/// A Journal build's plans with `bill_wise` ledgers marked bill-wise.
fn journal_plans(bill_wise: &[&str]) -> Vec<ScenarioPlan> {
    flagged(
        qualified_import_cycle_plans()[..BUILD_REQUESTS].to_vec(),
        bill_wise,
    )
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

/// Builds against a book whose `bill_wise` ledgers are marked Yes in the
/// catalogue, stopping after the first catalogue read.
async fn refusal(bill_wise: &[&str], max_bytes: usize) -> (Value, usize) {
    let plans = journal_plans(bill_wise);
    let simulator = SequenceSimulator::spawn(plans[..REFUSAL_REQUESTS].to_vec()).unwrap();
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
    // Probe, identity and the catalogue, whose rows carry the flag: no request
    // beyond the catalogue the build always reads.
    assert_eq!(requests, REFUSAL_REQUESTS);
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
        "does not mark which entries land On Account",
        "importing it also replaces any bill allocations",
    ] {
        assert!(next_step.contains(phrase), "{phrase}");
    }
    // It must not suggest a person's own dialog already covers these entries.
    assert!(!next_step.contains("shows the person its own dialog"));
}

#[tokio::test]
async fn a_party_with_no_bill_wise_ledger_builds_without_an_approval_and_records_none() {
    let simulator = SequenceSimulator::spawn(journal_plans(&[])).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let built = server.build_import_xml(&build_args(None)).await.unwrap();
    let result = &built.payload["result"];
    assert_eq!(result["on_account_approved"], json!([]));
    // The catalogue's response, which carried the flags, is on record.
    assert_eq!(
        result["bill_wise_response_sha256"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let note = result["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .find(|text| text.starts_with("Checked: none of the ledgers"))
        .expect("the build says what it checked");
    assert!(note.contains("as read from Tally in the ledger list during this build"));
    assert!(note.contains("refuses the post (import_bill_wise_changed)"));
    let saved = server.import_ledger().unwrap().pop().unwrap();
    assert_eq!(saved.on_account_approved, Some(Vec::new()));
    // The same requests as a build before the flag check: it costs none.
    assert_eq!(simulator.finish().unwrap().len(), BUILD_REQUESTS);
}

#[tokio::test]
async fn an_approved_party_builds_and_the_record_carries_the_approval() {
    let yes = || journal_plans(&[PARTY]);
    let (_directory, simulator, server) =
        sitting([yes()[..REFUSAL_REQUESTS].to_vec(), yes()].concat());
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
    assert_eq!(
        simulator.finish().unwrap().len(),
        REFUSAL_REQUESTS + BUILD_REQUESTS
    );
}

/// With party names masked the refusal's ledger reads like `Br…R4` to the
/// assistant, so the approval cannot repeat the name: it passes the digest the
/// refusal listed, with the masked name beside it or none, and the build records
/// the party's real name.
#[tokio::test]
async fn an_approval_by_digest_builds_when_party_names_are_masked() {
    let yes = || journal_plans(&[PARTY]);
    let plans = [yes()[..REFUSAL_REQUESTS].to_vec(), yes()].concat();
    for with_masked_name in [true, false] {
        let simulator = SequenceSimulator::spawn(plans.clone()).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let mut server = server(directory.path(), simulator.address().port(), 200_000);
        server.settings.redaction = crate::agent::Redaction::MaskParties;
        let refused = server.build_import_xml(&build_args(None)).await.unwrap();
        // What the assistant is shown.
        let seen = crate::agent::redact_value(
            refused.payload["result"].clone(),
            crate::agent::Redaction::MaskParties,
        );
        let listed = &seen["refused_parties"][0];
        let masked_name = listed["ledger"].as_str().expect("a masked name is text");
        assert_ne!(masked_name, PARTY);
        let digest = listed["party_digest"].as_str().unwrap().to_string();
        let approval = if with_masked_name {
            json!({"ledger": masked_name, "party_digest": digest})
        } else {
            json!({"party_digest": digest})
        };
        let built = server
            .build_import_xml(&build_args(Some(json!([approval]))))
            .await
            .unwrap();
        assert!(built.payload["result"]["batch_id"].is_string());
        let saved = server.import_ledger().unwrap().pop().unwrap();
        assert_eq!(
            saved.on_account_approved,
            Some(vec![bill_wise::OnAccountApproved {
                ledger: PARTY.into(),
                party_digest: digest,
            }])
        );
    }
}

#[tokio::test]
async fn approving_one_of_two_parties_refuses_the_other() {
    let both = || journal_plans(&[PARTY, SALES])[..REFUSAL_REQUESTS].to_vec();
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
    let all = journal_plans(&[PARTY]);
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
            json!([{"ledger": PARTY, "party_digest": digest}, {"party_digest": digest}]),
            2
        )
        .await,
        ("duplicate", 0)
    );
}

#[tokio::test]
async fn an_approval_with_a_digest_that_is_no_partys_is_refused() {
    let digest = "a".repeat(64);
    // The ledger beside the digest is not read: a bill-wise party's own name
    // and a ledger that is no party (`Cash` is in the batch, not bill-wise) are
    // refused alike, for the digest.
    for ledger in [PARTY, "Cash"] {
        assert_eq!(
            invalid_approval(
                json!([{"ledger": ledger, "party_digest": digest}]),
                REFUSAL_REQUESTS
            )
            .await,
            ("digest_differs", REFUSAL_REQUESTS),
            "{ledger}"
        );
    }
}

/// A ledger switched to or from bill-wise between the build's two catalogue
/// reads changes the catalogue's bytes, which the build compares, so the flag
/// is held to the same stability as the names with no read of its own.
#[tokio::test]
async fn a_flag_that_changes_between_the_two_catalogue_reads_is_refused() {
    let yes = journal_plans(&[PARTY]);
    // Bill-wise at the first read, not at the repeat (offset 18 onwards):
    // approved on one observation, then withdrawn.
    let mut changing = yes.clone();
    for (index, plan) in flagged(yes.clone(), &[]).into_iter().enumerate() {
        if index >= 18 {
            changing[index] = plan;
        }
    }
    let (directory, simulator, server) =
        sitting([yes[..REFUSAL_REQUESTS].to_vec(), changing[..24].to_vec()].concat());
    let refused = server.build_import_xml(&build_args(None)).await.unwrap();
    let digest = digest_of_first_party(&refused);
    let approvals = json!([{"ledger": PARTY, "party_digest": digest}]);
    let failure = server
        .build_import_xml(&build_args(Some(approvals)))
        .await
        .err()
        .unwrap();
    assert_eq!(failure.code, "import_catalogue_changed");
    assert!(!directory.path().join("imports").exists());
    assert!(!directory.path().join("agent-import-ledger.jsonl").exists());
    // Through the catalogue repeat and no further: the pre-flight window is
    // never read.
    assert_eq!(requests_made(simulator), REFUSAL_REQUESTS + 24);
}

#[tokio::test]
async fn a_flag_that_changes_on_a_ledger_the_batch_does_not_name_changes_the_catalogue_too() {
    // Café Naïve Traders is in the book and not in the batch: it turns
    // bill-wise between the two reads. The catalogue bytes differ, so the build
    // refuses as it does for any change to the book's ledgers.
    let first = journal_plans(&[]);
    let mut plans = first.clone();
    for (index, plan) in flagged(first, &["Café Naïve Traders"])
        .into_iter()
        .enumerate()
    {
        if index >= 18 {
            plans[index] = plan;
        }
    }
    let simulator = SequenceSimulator::spawn(plans[..24].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let failure = server
        .build_import_xml(&build_args(None))
        .await
        .err()
        .unwrap();
    assert_eq!(failure.code, "import_catalogue_changed");
    assert!(!directory.path().join("imports").exists());
}

async fn catalogue_failure(break_it: impl Fn(&str) -> String) -> ToolFailure {
    let plans = journal_plans(&[])
        .into_iter()
        .map(|mut plan| {
            let body = plan.fixture.body().into_owned();
            if body.contains("<LEDGER NAME=\"") && body.contains("<ISBILLWISEON") {
                let broken = break_it(&body);
                assert_ne!(broken, body);
                plan.fixture = Fixture::SyntheticXml(broken);
            }
            plan
        })
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans[..REFUSAL_REQUESTS].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    let failure = server
        .build_import_xml(&build_args(None))
        .await
        .err()
        .unwrap();
    assert!(!directory.path().join("imports").exists());
    failure
}

/// A V1 catalogue answer (no flag on the rows) is never read as "no ledger is
/// bill-wise": it is a failure with the cause named.
#[tokio::test]
async fn a_catalogue_answer_without_the_flag_is_a_failure_with_its_cause() {
    let failure = catalogue_failure(|body| {
        body.replace("<ISBILLWISEON TYPE=\"Logical\">No</ISBILLWISEON>", "")
            .replace("<ISBILLWISEON TYPE=\"Logical\">Yes</ISBILLWISEON>", "")
    })
    .await;
    assert_eq!(failure.code, "ledger_export_invalid");
    assert_eq!(
        failure.cause,
        Some("ledger_catalogue_bill_wise_flag_missing")
    );
}

#[tokio::test]
async fn a_catalogue_answer_with_an_unreadable_flag_is_a_failure_with_its_cause() {
    let failure = catalogue_failure(|body| {
        body.replacen(
            "<ISBILLWISEON TYPE=\"Logical\">No</ISBILLWISEON>",
            "<ISBILLWISEON TYPE=\"Logical\">Maybe</ISBILLWISEON>",
            1,
        )
    })
    .await;
    assert_eq!(failure.code, "ledger_export_invalid");
    assert_eq!(
        failure.cause,
        Some("ledger_catalogue_bill_wise_flag_invalid")
    );
}

/// The build sends the V2 catalogue request, which is V1's plus the one flag
/// method, and never the V1 request, for the catalogue.
#[tokio::test]
async fn the_catalogue_request_the_build_sends_is_the_v2_profile() {
    let simulator = SequenceSimulator::spawn(journal_plans(&[])).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), simulator.address().port(), 200_000);
    server.build_import_xml(&build_args(None)).await.unwrap();
    let observed = simulator.finish().unwrap();
    let wire = |xml: String| {
        sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
            &xml,
        ))
    };
    let v2 = wire(
        crate::tally::standard_ledger_catalog::render_import_ledger_catalog_request(
            "WR2 Unicode Lab",
        )
        .unwrap(),
    );
    let v1 = wire(
        crate::tally::standard_ledger_catalog::render_standard_ledger_catalog_request(
            "WR2 Unicode Lab",
        )
        .unwrap(),
    );
    let sent = |hash: &str| {
        observed
            .iter()
            .filter(|request| request.request_body_sha256 == hash)
            .count()
    };
    // Two reads, each posted twice.
    assert_eq!(sent(&v2), 4);
    assert_eq!(sent(&v1), 0);
}

#[tokio::test]
async fn a_bank_batch_names_its_bill_wise_counterparty_and_builds_once_approved() {
    let flagged = || flagged(bank_tests::bank_build_plans(), &[PARTY]);
    let simulator =
        SequenceSimulator::spawn([flagged()[..REFUSAL_REQUESTS + 6].to_vec(), flagged()].concat())
            .unwrap();
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
    simulator.cancel();
}

#[test]
fn the_schema_admits_an_approval_list_and_the_build_strips_it_before_parsing() {
    let schema = super::super::voucher_input_schema();
    let approvals = &schema["properties"]["on_account_approvals"];
    assert_eq!(approvals["type"], "array");
    assert_eq!(approvals["items"]["additionalProperties"], false);
    // The digest alone is an approval; the ledger beside it is optional.
    assert_eq!(approvals["items"]["required"], json!(["party_digest"]));
    assert_eq!(approvals["items"]["properties"]["ledger"]["type"], "string");
    assert_eq!(
        approvals["items"]["properties"]["party_digest"]["pattern"],
        "^[0-9a-f]{64}$"
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
        "does not mark which entries land On Account",
        "a hand import of the file is not checked at all",
        "is refused at post as import_bill_wise_changed",
        "bill_wise_party_unapproved",
        "on_account_approvals",
        "import_batch_predates_bill_wise_record",
    ] {
        assert!(description.contains(phrase), "{phrase}");
    }
}
