//! `prepare_stock_item` against the protocol simulator, on the constructed
//! two-book run of `agent_master_post_tests::coordinator`: the shape lab's
//! company list and marks with the R1 book's stock reads rebound to the shape
//! lab's GUID, each edit labelled there. No real Tally is involved.
use super::super::super::master_post::tests::coordinator::{
    passing_answers, rebound_prepared, rebound_shown, sent, shape_companies, shape_identity,
    standalone, Sent, EVERY_READ,
};
use super::super::super::master_post::tests::{STOCK_GUID, STOCK_PROPOSED, STOCK_SOURCE};
use super::*;
use crate::agent::{Posting, Redaction, Settings};
use crate::tally::stock_item_create::render_stock_item_create;
use bridge_tally_transport::TallyEndpointConfig;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const SHAPE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const STOCK_JOURNAL: &str = "agent-master-stock-item.jsonl";

fn answer(body: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(body))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn health() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

/// The company's verified identity: the company list, paired.
fn identity_answers(education: bool) -> Vec<ScenarioPlan> {
    let companies = answer(shape_companies(education));
    vec![companies.clone(), health(), companies, health()]
}

/// The requests the identity read sends.
const IDENTITY_READ: [Sent; 4] = [Sent::Companies, Sent::Health, Sent::Companies, Sent::Health];

struct Run {
    outcome: Result<ToolOutcome, ToolFailure>,
    sent: Vec<Sent>,
    origin: String,
    _directory: tempfile::TempDir,
    journal: Option<Vec<u8>>,
}

/// Runs `prepare_stock_item` once against `plans`, with a fresh data directory.
async fn prepare(plans: Vec<ScenarioPlan>, guid: &str, name: &str, sibling: &str) -> Run {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let endpoint = TallyEndpointConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    };
    let origin = crate::agent::canonical_loopback_origin(&endpoint).unwrap();
    let server = Server::new(Settings {
        endpoint,
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: true,
        posting: Posting::On,
        master_writes_enabled: true,
        audit_enabled: false,
    });
    let outcome = server.prepare_stock_item(guid, name, sibling).await;
    let sent = sent(&simulator.finish().unwrap());
    let journal = std::fs::read(directory.path().join(STOCK_JOURNAL)).ok();
    Run {
        outcome,
        sent,
        origin,
        _directory: directory,
        journal,
    }
}

/// The identity read, then the stock reads' whole exchange: the constructed
/// run's answers in the order the stock read sends them.
fn passing_run() -> Vec<ScenarioPlan> {
    let mut plans = identity_answers(false);
    plans.extend(standalone(passing_answers()));
    plans
}

fn every_request() -> Vec<Sent> {
    IDENTITY_READ.iter().chain(&EVERY_READ).copied().collect()
}

#[tokio::test]
async fn prepare_records_one_batch_of_the_shown_item_and_returns_its_preview() {
    let run = prepare(passing_run(), SHAPE_GUID, STOCK_PROPOSED, STOCK_SOURCE).await;
    let outcome = match run.outcome {
        Ok(outcome) => outcome,
        Err(failure) => panic!("the prepare refused: {failure:?}"),
    };
    // One identity read and one stock read, nothing else: every request an
    // export or a health check.
    assert_eq!(run.sent, every_request());
    let identity = shape_identity();
    let result = &outcome.payload["result"];
    let batch_id = result["batch_id"].as_str().expect("a batch id");
    assert!(batch_id.starts_with("bridge-"), "{batch_id}");
    // The oracle: the same run's draft, shown by the one coordinator.
    let shown = rebound_shown(&identity, &run.origin);
    assert_eq!(result["preview"].as_str(), Some(shown.preview()));
    assert_eq!(result["next_step"].as_str(), Some(PREPARED_STOCK_NEXT_STEP));
    assert_eq!(outcome.company_guid.as_deref(), Some(SHAPE_GUID));

    let journal = run.journal.expect("the stock journal is written");
    let lines = journal
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice::<Value>(line).expect("a journal line parses"))
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 1, "exactly one record");
    assert_eq!(lines[0]["record_type"], "master_batch");
    assert_eq!(lines[0]["batch_id"], batch_id);
    let rendered = render_stock_item_create(&rebound_prepared(&identity));
    assert_eq!(
        lines[0]["batch_sha256"].as_str(),
        Some(bridge_tally_protocol::sha256_hex(rendered.as_bytes()).as_str()),
        "the digest of the bytes the renderer gives the shown item"
    );
    assert_eq!(lines[0]["item"]["name"], STOCK_PROPOSED);
    assert_eq!(lines[0]["item"]["sibling_source_name"], STOCK_SOURCE);
}

fn refused(run: &Run) -> &str {
    match &run.outcome {
        Ok(outcome) => panic!("the prepare answered: {}", outcome.payload),
        Err(failure) => &failure.code,
    }
}

#[tokio::test]
async fn prepare_refuses_by_its_declared_code_and_records_nothing() {
    // Each refusal is driven by the caller's values on the passing run.
    for (name, sibling, code) in [
        // The sibling's own name is taken.
        (STOCK_SOURCE, STOCK_SOURCE, StockItemCreateCode::NameTaken),
        (
            STOCK_PROPOSED,
            "SC Absent Sibling",
            StockItemCreateCode::SiblingNotFound,
        ),
        // The draft builds; the preview refuses what the renderer would.
        (
            "SC Copied & Marked",
            STOCK_SOURCE,
            StockItemCreateCode::TextNeedsEscaping,
        ),
    ] {
        let run = prepare(passing_run(), SHAPE_GUID, name, sibling).await;
        assert_eq!(refused(&run), code.as_str(), "{name} from {sibling}");
        assert_eq!(run.sent, every_request());
        assert_eq!(run.journal, None, "nothing is recorded");
    }
}

#[tokio::test]
async fn prepare_refuses_another_companys_guid_before_any_stock_read() {
    // The R1 book's GUID: not a company the run's list holds.
    let run = prepare(
        identity_answers(false),
        STOCK_GUID,
        STOCK_PROPOSED,
        STOCK_SOURCE,
    )
    .await;
    assert_eq!(refused(&run), "company_identity_not_found");
    assert_eq!(run.sent, IDENTITY_READ);
    assert_eq!(run.journal, None);
}

#[tokio::test]
async fn a_refusal_at_the_stock_reads_records_nothing_and_education_is_the_licensed_mode_refusal() {
    // The stock read refuses Education before any stock request, with the
    // ledger prepare's own code.
    let mut plans = identity_answers(true);
    plans.extend([health(), answer(shape_companies(true))]);
    let run = prepare(plans, SHAPE_GUID, STOCK_PROPOSED, STOCK_SOURCE).await;
    let expected = require_licensed_mode(DateBoundaryProfile::EducationRestricted)
        .expect_err("Education is refused");
    assert_eq!(refused(&run), expected.code);
    assert_eq!(refused(&run), "master_writes_need_licensed_mode");
    let mut expected_sent = IDENTITY_READ.to_vec();
    expected_sent.extend([Sent::Health, Sent::Companies]);
    assert_eq!(run.sent, expected_sent);
    assert_eq!(run.journal, None, "no batch is written");
}
