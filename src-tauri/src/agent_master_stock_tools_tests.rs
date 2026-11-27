//! `prepare_stock_item` against the protocol simulator, on the constructed
//! two-book run of `agent_master_post_tests::coordinator`: the shape lab's
//! company list and marks with the R1 book's stock reads rebound to the shape
//! lab's GUID, each edit labelled there. No real Tally is involved.
use super::super::super::master_post::tests::coordinator::{
    marks_at, passing_answers, rebound_prepared, rebound_shown, sent, shape_companies,
    shape_identity, standalone, through_binding, Sent, EVERY_READ,
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
    prepare_under(plans, guid, name, sibling, Redaction::None).await
}

/// [`prepare`] under `redaction`.
async fn prepare_under(
    plans: Vec<ScenarioPlan>,
    guid: &str,
    name: &str,
    sibling: &str,
    redaction: Redaction,
) -> Run {
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
        redaction,
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
    let lines = journal_lines(&journal);
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

/// The journal's lines, each parsed.
fn journal_lines(journal: &[u8]) -> Vec<Value> {
    (journal.split(|byte| *byte == b'\n'))
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice::<Value>(line).expect("a journal line parses"))
        .collect()
}

#[tokio::test]
async fn under_mask_parties_the_preview_masks_the_item_sibling_parent_and_hsn_description_and_records_the_same_batch(
) {
    let plain = prepare(passing_run(), SHAPE_GUID, STOCK_PROPOSED, STOCK_SOURCE).await;
    let masked = prepare_under(
        passing_run(),
        SHAPE_GUID,
        STOCK_PROPOSED,
        STOCK_SOURCE,
        Redaction::MaskParties,
    )
    .await;
    let preview = |run: &Run| match &run.outcome {
        Ok(outcome) => outcome.payload["result"]["preview"]
            .as_str()
            .expect("a preview")
            .to_string(),
        Err(failure) => panic!("the prepare refused: {failure:?}"),
    };
    let (plain_text, masked_text) = (preview(&plain), preview(&masked));
    let prepared = rebound_prepared(&shape_identity());
    let (parent, hsn) = (prepared.writes().parent(), prepared.writes().hsn());
    // The captured sibling's HSN description is not empty, so its mask is seen.
    assert!(!hsn.description().is_empty());
    for text in [STOCK_PROPOSED, STOCK_SOURCE, parent, hsn.description()] {
        let hidden = crate::agent::mask(text);
        // A mask that kept the plain text would pass the checks below.
        assert_ne!(hidden, text);
        assert!(masked_text.contains(&hidden), "{hidden} is shown");
        assert!(!masked_text.contains(text), "{text} leaked");
        assert!(
            plain_text.contains(text),
            "{text} is shown whole without masking"
        );
    }
    // Only those four differ: the same lines; the unit, the HSN code and the
    // rates stay whole.
    assert_eq!(masked_text.lines().count(), plain_text.lines().count());
    let quoted = |text: &str| serde_json::to_string(text).unwrap();
    assert!(masked_text.contains(&format!("Base unit: {}", quoted(prepared.writes().unit()))));
    assert!(masked_text.contains(&format!("HSN code: {}", quoted(hsn.code()))));
    let rate_lines = |text: &str| {
        text.lines()
            .filter(|line| line.contains("Rate row "))
            .map(str::to_string)
            .collect::<Vec<_>>()
    };
    assert!(!rate_lines(&masked_text).is_empty());
    assert_eq!(rate_lines(&masked_text), rate_lines(&plain_text));
    // The recorded batch is the same but for its fresh id.
    let batch = |run: &Run| {
        let mut lines = journal_lines(run.journal.as_deref().expect("a batch is recorded"));
        assert_eq!(lines.len(), 1);
        lines[0]
            .as_object_mut()
            .unwrap()
            .remove("batch_id")
            .expect("a batch id");
        lines.remove(0)
    };
    assert_eq!(batch(&masked), batch(&plain));
}

#[tokio::test]
async fn a_read_refused_past_the_size_edge_answers_its_typed_code_and_records_nothing() {
    // Mark 490 is the coordinator tests' constructed edit, past the candidate
    // edge of 489: the read refuses before any stock request.
    let mut plans = identity_answers(false);
    plans.extend(through_binding(marks_at(490)));
    let run = prepare(plans, SHAPE_GUID, STOCK_PROPOSED, STOCK_SOURCE).await;
    let expected = MastersReadError::TooLarge {
        master_alter_id: 490,
        estimated_bytes: 0,
        limit_bytes: 0,
        limit_master_alter_id: 489,
    };
    assert_eq!(refused(&run), expected.safe_code());
    let mut expected_sent = IDENTITY_READ.to_vec();
    expected_sent.extend([Sent::Health, Sent::Companies, Sent::Companies, Sent::Marks]);
    assert_eq!(run.sent, expected_sent);
    assert_eq!(run.journal, None, "nothing is recorded");
}

// --- post_stock_item: the batch, the book read again, the ask, the dispatch ---

mod post {
    use super::super::super::super::master_ledger::stock_item_journal::record_stock_item_intent;
    use super::super::super::super::master_ledger::{
        read_journal, ExclusiveAdmission, TEST_FAULTS,
    };
    use super::super::super::super::master_post::tests::coordinator::{
        approved_stock, import_answer, imports, minted, rebound_after, rebound_before,
        rebound_shown_from, stock_answers, stock_answers_at, QUEUE_READS,
    };
    use super::super::super::super::master_review::tests::{
        approved_shown_for, approved_stock_item_for, stock_approvals, taken_stock_approval,
    };
    use super::*;
    use crate::tally::approved_import::test_seam::{ScriptedApproval, SCRIPTED_APPROVAL};
    use std::time::Duration;
    use tally_protocol_simulator::{Delivery, DeliveryGate, ObservedRequest};

    const A: u16 = 0xa1;
    const B: u16 = 0xb2;

    fn id(serial: u16) -> &'static str {
        crate::synthetic_batch_id(serial)
    }

    /// A server over a double answering `plans`, in a data folder of its own.
    struct PostRun {
        server: Server,
        simulator: SequenceSimulator,
        origin: String,
        directory: tempfile::TempDir,
    }

    fn posting(plans: Vec<ScenarioPlan>, redaction: Redaction) -> PostRun {
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
            redaction,
            import_enabled: true,
            posting: Posting::On,
            master_writes_enabled: true,
            audit_enabled: false,
        });
        PostRun {
            server,
            simulator,
            origin,
            directory,
        }
    }

    impl PostRun {
        fn dir(&self) -> &std::path::Path {
            self.directory.path()
        }

        async fn post(
            &self,
            scripted: &ScriptedApproval,
            batch_id: &str,
        ) -> Result<ToolOutcome, ToolFailure> {
            SCRIPTED_APPROVAL
                .scope(scripted.clone(), self.server.post_stock_item(batch_id))
                .await
        }

        /// The record types of the stock journal.
        fn records(&self) -> Vec<String> {
            let journal = std::fs::read(self.dir().join(STOCK_JOURNAL)).unwrap_or_default();
            (journal_lines(&journal).iter())
                .map(|line| line["record_type"].as_str().unwrap().to_string())
                .collect()
        }

        fn finish(self) -> Vec<ObservedRequest> {
            self.simulator.finish().unwrap()
        }

        /// The requests received before the double is stopped, its unused
        /// answer left out.
        fn cancelled(self) -> Vec<ObservedRequest> {
            self.simulator.cancel();
            let observed = self.simulator.finish().unwrap();
            observed
                .into_iter()
                .filter(|request| !request.method.is_empty())
                .collect()
        }
    }

    /// The batch of the rebound draft shown at `origin`, as the prepare
    /// records it, under the id of `serial`.
    fn shape_batch(serial: u16, origin: &str) -> StockItemBatch {
        rebound_shown(&shape_identity(), origin)
            .batch(id(serial).to_string())
            .unwrap()
    }

    fn record(dir: &std::path::Path, batch: &StockItemBatch) {
        let lock = ExclusiveAdmission::acquire(dir).unwrap();
        append(dir, &lock, JournalRecord::Batch(batch.clone())).unwrap();
    }

    /// `batch` with the hand edit `edit` made to its line.
    fn hand_edited(batch: &StockItemBatch, edit: impl FnOnce(&mut Value)) -> StockItemBatch {
        let mut line = serde_json::to_value(batch).unwrap();
        edit(&mut line);
        serde_json::from_value(line).unwrap()
    }

    /// The post's own reads: the identity read, then the stock reads.
    fn post_reads() -> Vec<ScenarioPlan> {
        let mut plans = identity_answers(false);
        plans.extend(standalone(passing_answers()));
        plans
    }

    /// The queue's reads after the approval, then Tally's answer to the Create.
    fn dispatch() -> Vec<ScenarioPlan> {
        let mut plans = passing_answers();
        plans.push(import_answer());
        plans
    }

    /// The read-back after the Create, in the standalone read's order: the R1
    /// read after the marked create, rebound (three rows, so the count is 3, a
    /// hand edit), and marks 290 in both answers: one above the queue's 289, a
    /// constructed edit, since the marks after a create were not captured
    /// (stock-create-lab `PROVENANCE.md`).
    fn read_back() -> Vec<ScenarioPlan> {
        standalone(stock_answers_at(
            rebound_after(),
            3,
            marks_at(290),
            marks_at(290),
        ))
    }

    /// [`dispatch`], then the [`read_back`].
    fn dispatch_and_read_back() -> Vec<ScenarioPlan> {
        let mut plans = dispatch();
        plans.extend(read_back());
        plans
    }

    /// The records of a batch sent, answered and judged.
    const RECORDED: [&str; 4] = [
        "master_batch",
        "master_dispatch_intent",
        "master_dispatch_response",
        "master_verdict",
    ];

    /// The verified item's GUID: the rebound R1 row of the marked create.
    fn created_guid() -> String {
        format!("{SHAPE_GUID}-000000d2")
    }

    fn refused_code(outcome: &Result<ToolOutcome, ToolFailure>) -> &str {
        match outcome {
            Ok(outcome) => panic!("the post answered: {}", outcome.payload),
            Err(failure) => &failure.code,
        }
    }

    /// The Create's bytes as the renderer gives the rebound draft.
    fn create_bytes() -> String {
        render_stock_item_create(&rebound_prepared(&shape_identity()))
    }

    /// Prepare, then post: one dialog, the stored bytes sent once, one intent,
    /// the approval used and gone, Tally's answer recorded, the book read back
    /// and the item verified, with the verdict recorded. Under `mask_parties`
    /// the answer's preview is masked as the prepare masks it, and the dialog
    /// shows the whole names. Posted again, the settled batch is answered from
    /// the journal alone.
    #[tokio::test]
    async fn a_prepared_stock_item_is_asked_once_sent_once_and_verified_by_its_read_back() {
        for redaction in [Redaction::None, Redaction::MaskParties] {
            let mut plans = passing_run();
            let prepared_at = plans.len();
            plans.extend(post_reads());
            let posted_at = plans.len() + passing_answers().len();
            plans.extend(dispatch_and_read_back());
            let run = posting(plans, redaction);
            let prepared = run
                .server
                .prepare_stock_item(SHAPE_GUID, STOCK_PROPOSED, STOCK_SOURCE);
            let prepared = prepared.await.expect("the prepare records a batch");
            let batch_id = prepared.payload["result"]["batch_id"]
                .as_str()
                .unwrap()
                .to_string();
            let scripted = ScriptedApproval::approving();
            let outcome = run
                .post(&scripted, &batch_id)
                .await
                .expect("the post answers");
            let result = &outcome.payload["result"];
            assert_eq!(result["state"], "posted_verified", "{result}");
            assert_eq!(result["answer_recorded"], true);
            let verification = &result["verification"];
            assert_eq!(verification["moment"], "after_post");
            assert_eq!(verification["settled"], true);
            assert_eq!(verification["guid"], json!(created_guid()));
            assert_eq!(verification["reasons"], json!([]));
            assert_eq!(verification["read_back"]["state"], "read");
            assert_eq!(verification["verdict_written"], true);
            assert_eq!(result["dispatch"]["state"], "dispatched");
            assert_eq!(result["counters"]["created"], 1);
            let step = result["next_step"].as_str().unwrap();
            assert!(
                !step.contains("post_") && !step.contains("prepare_"),
                "{step}"
            );
            let shown = rebound_shown(&shape_identity(), &run.origin);
            let (dialogs, preview) = (scripted.stock_items(), result["preview"].as_str().unwrap());
            assert_eq!(dialogs.len(), 1, "asked once");
            assert!(
                dialogs[0].contains(STOCK_PROPOSED),
                "the dialog shows the whole name"
            );
            if redaction == Redaction::MaskParties {
                assert_eq!(preview, shown.masked_preview());
                assert!(!preview.contains(STOCK_PROPOSED) && !preview.contains(STOCK_SOURCE));
            } else {
                assert_eq!(preview, shown.preview());
            }
            assert_eq!(run.records(), RECORDED);
            assert!(
                !run.server.post_approvals.holds(&batch_id),
                "nothing is held"
            );
            let stored = journal_lines(&std::fs::read(run.dir().join(STOCK_JOURNAL)).unwrap());
            assert_eq!(stored[3]["state"], "posted_verified");
            assert_eq!(stored[3]["items"][0]["guid"], json!(created_guid()));
            assert_eq!(stored[3]["items"][0]["reason"], Value::Null);
            // Posted again: the settled batch answers from the journal, and
            // nothing is asked, read or sent.
            let again = run.post(&scripted, &batch_id).await.expect("an answer");
            let again = &again.payload["result"];
            assert_eq!(again["state"], "posted_verified", "{again}");
            assert_eq!(again["journal"]["verdict"], "posted_verified");
            assert_eq!(again["attempt_recorded"], true);
            assert_eq!(scripted.stock_items().len(), 1);
            let observed = run.finish();
            assert_eq!(
                sent(&observed[posted_at + 1..]),
                EVERY_READ,
                "after the one import, the read-back alone"
            );
            assert_eq!(sent(&observed[..prepared_at]), every_request());
            assert_eq!(
                sent(&observed[prepared_at..posted_at]),
                [every_request(), QUEUE_READS.to_vec()].concat()
            );
            let sent_imports = imports(&observed);
            assert_eq!(
                sent_imports,
                [create_bytes()],
                "exactly the stored request, once"
            );
            assert_eq!(
                stored[0]["batch_sha256"].as_str(),
                Some(bridge_tally_protocol::sha256_hex(sent_imports[0].as_bytes()).as_str())
            );
        }
    }

    /// An id the stock journal does not hold, and a malformed one, are refused
    /// before any request, and nothing is asked.
    #[tokio::test]
    async fn an_unknown_or_malformed_batch_id_is_refused_before_anything() {
        // One answer the post must not use, so the double stays up.
        let run = posting(vec![health()], Redaction::None);
        let scripted = ScriptedApproval::approving();
        let unknown = run.post(&scripted, id(A)).await;
        assert_eq!(refused_code(&unknown), "master_batch_not_found");
        let malformed = run.post(&scripted, "not-a-batch").await;
        assert_eq!(refused_code(&malformed), "import_batch_identifier_invalid");
        assert!(scripted.stock_items().is_empty());
        assert!(run.cancelled().is_empty(), "no request is sent");
    }

    /// A batch whose stored company is not the one verified now (a hand edit
    /// of the stored company name), with an approval held for it.
    #[tokio::test]
    async fn another_company_identity_is_refused_after_the_identity_read() {
        let run = posting(identity_answers(false), Redaction::None);
        let batch = hand_edited(&shape_batch(A, &run.origin), |line| {
            line["company"]["name"] = json!("Another Company");
        });
        record(run.dir(), &batch);
        // An approval held for the batch: the refusal before the take
        // withdraws it.
        hold_for(&run, id(A)).await;
        let scripted = ScriptedApproval::approving();
        let outcome = run.post(&scripted, id(A)).await;
        assert_eq!(refused_code(&outcome), "company_identity_mismatch");
        assert!(scripted.stock_items().is_empty());
        assert!(!run.server.post_approvals.holds(id(A)), "it is withdrawn");
        assert_eq!(run.records(), ["master_batch"]);
        assert_eq!(sent(&run.finish()), IDENTITY_READ);
    }

    /// The book as it stands makes another batch than the stored one: the
    /// sibling's GST rate changed (the digest differs), or only its GUID
    /// changed (the digest is the same, the stored writes are not), or the
    /// stored digest is not the batch's (a hand edit of the line). Each is
    /// refused before the ask, and nothing is recorded or imported.
    #[tokio::test]
    async fn a_book_that_makes_another_batch_is_refused_before_the_ask() {
        let rate = "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>";
        let changed_rate =
            rebound_before().replacen(rate, "<GSTRATE TYPE=\"Number\"> 10</GSTRATE>", 1);
        let sibling = format!("{SHAPE_GUID}-000000d0");
        assert_eq!(rebound_before().matches(&sibling).count(), 1);
        let changed_guid = rebound_before().replace(&sibling, &format!("{SHAPE_GUID}-000000d9"));
        let tampered = |batch: &StockItemBatch| {
            hand_edited(batch, |line| line["batch_sha256"] = json!("e".repeat(64)))
        };
        type Edit = fn(&StockItemBatch) -> StockItemBatch;
        let cases: [(String, Edit); 3] = [
            (changed_rate, Clone::clone),
            (changed_guid, Clone::clone),
            (rebound_before(), tampered),
        ];
        for (candidate, edit) in cases {
            let mut plans = identity_answers(false);
            plans.extend(standalone(stock_answers(candidate, 1, marks_at(289))));
            let run = posting(plans, Redaction::None);
            record(run.dir(), &edit(&shape_batch(A, &run.origin)));
            let scripted = ScriptedApproval::approving();
            let outcome = run.post(&scripted, id(A)).await;
            assert_eq!(refused_code(&outcome), "import_batch_changed");
            assert!(scripted.stock_items().is_empty(), "nothing is asked");
            assert_eq!(run.records(), ["master_batch"]);
            assert_eq!(sent(&run.finish()), every_request(), "no import");
        }
    }

    /// A batch prepared in licensed mode and posted in Education: refused by
    /// the stock read, before any stock request and before the ask, with the
    /// prepare's code.
    #[tokio::test]
    async fn a_batch_posted_in_education_is_refused_before_the_ask_with_the_prepares_code() {
        let mut plans = identity_answers(true);
        plans.extend([health(), answer(shape_companies(true))]);
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let scripted = ScriptedApproval::approving();
        let outcome = run.post(&scripted, id(A)).await;
        assert_eq!(refused_code(&outcome), "master_writes_need_licensed_mode");
        assert!(scripted.stock_items().is_empty());
        assert_eq!(run.records(), ["master_batch"]);
        let mut expected = IDENTITY_READ.to_vec();
        expected.extend([Sent::Health, Sent::Companies]);
        assert_eq!(sent(&run.finish()), expected);
    }

    /// Records batch `b`'s intent in `dir`, as a post that sent it leaves it:
    /// an open stock dispatch.
    async fn dispatch_open(dir: &std::path::Path, b: &StockItemBatch) {
        let holder = tempfile::tempdir().unwrap();
        let approvals = stock_approvals(holder.path());
        let taken = taken_stock_approval(&approvals, b.batch_id()).await;
        let lock = ExclusiveAdmission::acquire(dir).unwrap();
        let token = minted(b.batch_id()).await;
        let _recorded =
            record_stock_item_intent(dir, &lock, b, token, &taken).expect("the intent is recorded");
    }

    /// An open stock dispatch of the same name refuses before the ask and
    /// names its batch; an open ledger dispatch of the name does not, and the
    /// ledger post's own check does not see the open stock dispatch.
    #[tokio::test]
    async fn an_open_stock_dispatch_of_the_name_refuses_before_the_ask_and_a_ledgers_does_not() {
        let run = posting(post_reads(), Redaction::None);
        let (a, b) = (shape_batch(A, &run.origin), shape_batch(B, &run.origin));
        record(run.dir(), &a);
        record(run.dir(), &b);
        dispatch_open(run.dir(), &b).await;
        // An approval held for A: the refusal withdraws it.
        hold_for(&run, id(A)).await;
        let scripted = ScriptedApproval::approving();
        let outcome = run
            .post(&scripted, id(A))
            .await
            .expect("the refusal answers");
        let error = &outcome.payload["result"]["error"];
        assert_eq!(error["code"], OVERLAP_CODE);
        assert_eq!(error["hold"], "held");
        assert_eq!(error["blocking_batch_ids"], json!([id(B)]));
        assert!(error["message"].as_str().unwrap().contains("stock item"));
        assert!(!error["next_step"].as_str().unwrap().contains("post_"));
        assert_eq!(outcome.payload["result"]["attempt_recorded"], false);
        assert_eq!(
            outcome.payload["result"]["dispatch"]["state"],
            "not_dispatched"
        );
        assert!(scripted.stock_items().is_empty(), "nothing is asked");
        assert!(
            !run.server.post_approvals.holds(id(A)),
            "the approval is withdrawn"
        );
        // The ledger post's hold check, on this same folder, which holds B's
        // open stock dispatch: a ledger batch of the name (its own intent
        // aside) is not held by it.
        assert_eq!(
            run.records(),
            ["master_batch", "master_batch", "master_dispatch_intent"]
        );
        let ledger_dir = tempfile::tempdir().unwrap();
        let ledger = super::super::super::tests::dispatched_batch_in(
            ledger_dir.path(),
            STOCK_PROPOSED,
            SHAPE_GUID,
        );
        let lock = ExclusiveAdmission::acquire(run.dir()).unwrap();
        append(run.dir(), &lock, JournalRecord::Batch(ledger.clone())).unwrap();
        drop(lock);
        let ledgers = read_journal(run.dir()).unwrap();
        assert_eq!(
            overlap_view(Some(&ledgers), &ledger.batch_id, SHAPE_GUID),
            OverlapView::Settled
        );
        assert_eq!(sent(&run.finish()), every_request());

        // An open ledger dispatch of the same name: the stock post asks.
        let run = posting(post_reads(), Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        super::super::super::tests::dispatched_batch_in(run.dir(), STOCK_PROPOSED, SHAPE_GUID);
        let declined = ScriptedApproval::declining();
        let outcome = run.post(&declined, id(A)).await;
        assert_ne!(refused_code(&outcome), OVERLAP_CODE);
        assert_eq!(
            declined.stock_items().len(),
            1,
            "the stock item is asked about"
        );
        assert_eq!(run.records(), ["master_batch"]);
        assert_eq!(sent(&run.finish()), every_request());
    }

    /// An approval held with the fresh binding but for another item: the
    /// holder's take admits it (its bytes are its held item's), and the post's
    /// own check of the held item refuses it, before the owed digest check
    /// and with nothing recorded or imported.
    #[tokio::test]
    async fn an_approval_held_for_another_item_is_refused_at_the_take() {
        let run = posting(post_reads(), Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let fresh = rebound_shown(&shape_identity(), &run.origin).binding();
        // The R1 book's own item, asked for this batch: another company's.
        let (_, held, approved, answered) =
            approved_stock_item_for(STOCK_PROPOSED, id(A).to_string()).await;
        (run.server.post_approvals)
            .hold_approved(id(A), fresh, approved, held, answered)
            .unwrap();
        let scripted = ScriptedApproval::approving();
        let outcome = run.post(&scripted, id(A)).await;
        assert_eq!(refused_code(&outcome), "import_approval_binding_changed");
        assert!(
            scripted.stock_items().is_empty(),
            "the held approval is redeemed, not asked"
        );
        assert!(!run.server.post_approvals.holds(id(A)), "it is withdrawn");
        assert_eq!(run.records(), ["master_batch"]);
        assert_eq!(sent(&run.finish()), every_request(), "no import");
    }

    /// An approval held with the fresh binding for this company's item, but
    /// for other writes (the sibling's GST rate 10, a constructed edit of the
    /// rebound capture): the company is the same, so the writes are what
    /// refuses it, before the owed digest check and with nothing recorded,
    /// imported or left held.
    #[tokio::test]
    async fn an_approval_held_for_other_writes_of_this_company_is_refused_at_the_take() {
        let run = posting(post_reads(), Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let identity = shape_identity();
        let fresh = rebound_shown(&identity, &run.origin).binding();
        let rate = "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>";
        let changed = rebound_before().replacen(rate, "<GSTRATE TYPE=\"Number\"> 10</GSTRATE>", 1);
        let other = rebound_shown_from(&identity, &run.origin, &changed);
        let marks = stock_marks(&identity).unwrap_or_else(|_| panic!("the marks parse"));
        let (_, held, approved, answered) =
            approved_shown_for(other, marks, id(A).to_string()).await;
        let HeldRequest::StockItem(item) = &held else {
            panic!("a stock item is held")
        };
        assert_eq!(item.prepared().company(), &identity, "the same company");
        (run.server.post_approvals)
            .hold_approved(id(A), fresh, approved, held, answered)
            .unwrap();
        let scripted = ScriptedApproval::approving();
        let outcome = run.post(&scripted, id(A)).await;
        assert_eq!(refused_code(&outcome), "import_approval_binding_changed");
        assert!(scripted.stock_items().is_empty());
        assert!(!run.server.post_approvals.holds(id(A)));
        assert_eq!(run.records(), ["master_batch"]);
        assert_eq!(sent(&run.finish()), every_request(), "no import");
    }

    /// Holds an approved stock approval for `batch_id` under the fresh
    /// binding, as a click the post has not yet redeemed leaves it.
    async fn hold_for(run: &PostRun, batch_id: &str) {
        let fresh = rebound_shown(&shape_identity(), &run.origin).binding();
        let (_, held, approved, answered) =
            approved_stock_item_for(STOCK_PROPOSED, batch_id.to_string()).await;
        (run.server.post_approvals)
            .hold_approved(batch_id, fresh, approved, held, answered)
            .unwrap();
        assert!(run.server.post_approvals.holds(batch_id));
    }

    /// A batch whose send is recorded, with an approval held for it: the
    /// approval is withdrawn, and the batch is reconciled: the company
    /// checked, the book read back and judged, never verified, and nothing
    /// imported.
    #[tokio::test]
    async fn an_approval_held_for_a_sent_batch_is_withdrawn() {
        let mut plans = identity_answers(false);
        plans.extend(read_back());
        let run = posting(plans, Redaction::None);
        let a = shape_batch(A, &run.origin);
        record(run.dir(), &a);
        dispatch_open(run.dir(), &a).await;
        hold_for(&run, id(A)).await;
        let outcome = run.post(&ScriptedApproval::approving(), id(A)).await;
        let result = outcome.expect("the answer").payload["result"].clone();
        assert_eq!(result["state"], "reconciliation_required", "{result}");
        assert_eq!(result["verification"]["moment"], "reconcile");
        assert_eq!(result["verification"]["read_back"]["state"], "read");
        let reasons = &result["verification"]["reasons"];
        assert!(
            reasons
                .as_array()
                .unwrap()
                .contains(&json!("later_call_does_not_verify")),
            "{reasons}"
        );
        assert_eq!(result["attempt_recorded"], true);
        assert!(!run.server.post_approvals.holds(id(A)));
        assert_eq!(
            sent(&run.finish()),
            every_request(),
            "the read-back, no import"
        );
    }

    /// A later call whose company check fails (the company list answered
    /// unpaired, its second copy in Education: a constructed edit): it answers
    /// as a recorded send with the check's code, reads nothing back and records
    /// nothing.
    #[tokio::test]
    async fn a_reconcile_whose_company_check_fails_reads_nothing_back() {
        let mut plans = identity_answers(false);
        plans[2] = identity_answers(true).remove(2);
        let run = posting(plans, Redaction::None);
        let a = shape_batch(A, &run.origin);
        record(run.dir(), &a);
        dispatch_open(run.dir(), &a).await;
        let outcome = run.post(&ScriptedApproval::approving(), id(A)).await;
        let result = outcome.expect("the answer").payload["result"].clone();
        assert_eq!(
            result["dispatch"]["state"], "reconciliation_required",
            "{result}"
        );
        assert_eq!(result["verification"]["attempted"], false);
        assert_eq!(result["attempt_recorded"], true);
        assert_eq!(result["error"]["code"], "company_collection_invalid");
        assert_eq!(run.records(), ["master_batch", "master_dispatch_intent"]);
        assert_eq!(
            sent(&run.finish()),
            IDENTITY_READ,
            "the identity read alone"
        );
    }

    /// The name appears in the book between the ask and the dispatch: the
    /// queue's stock recheck refuses it, and the answer is the prepare's
    /// name-taken code, with nothing imported and no intent.
    #[tokio::test]
    async fn a_name_taken_after_the_ask_is_refused_in_the_queue_with_the_prepares_code() {
        let mut plans = post_reads();
        plans.extend(stock_answers(rebound_after(), 3, marks_at(289)));
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let scripted = ScriptedApproval::approving();
        let outcome = run.post(&scripted, id(A)).await;
        assert_eq!(refused_code(&outcome), "master_name_taken");
        assert_eq!(scripted.stock_items().len(), 1, "asked once");
        assert_eq!(run.records(), ["master_batch"]);
        assert!(imports(&run.finish()).is_empty(), "no import");
    }

    /// The book makes another draft between the ask and the dispatch (the
    /// sibling's GST rate 9 made 10 in the queue's read, the constructed edit
    /// of the rebound capture the same-company test uses): the queue's stock
    /// recheck refuses it, and the answer is the code the post gives a changed
    /// batch before the ask, with nothing imported, recorded or held.
    #[tokio::test]
    async fn a_draft_changed_after_the_ask_is_refused_in_the_queue_as_a_changed_batch() {
        let rate = "<GSTRATE TYPE=\"Number\"> 9</GSTRATE>";
        let changed = rebound_before().replacen(rate, "<GSTRATE TYPE=\"Number\"> 10</GSTRATE>", 1);
        assert_ne!(changed, rebound_before());
        let mut plans = post_reads();
        plans.extend(stock_answers(changed, 1, marks_at(289)));
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let scripted = ScriptedApproval::approving();
        let outcome = run.post(&scripted, id(A)).await;
        assert_eq!(refused_code(&outcome), "import_batch_changed");
        assert_eq!(scripted.stock_items().len(), 1, "asked once");
        assert_eq!(run.records(), ["master_batch"]);
        assert!(!run.server.post_approvals.holds(id(A)));
        assert!(imports(&run.finish()).is_empty(), "no import");
    }

    /// The create is lost after the intent is recorded: the answer is that a
    /// send is recorded and NOT verified, through the tool.
    #[tokio::test]
    async fn a_send_lost_after_the_intent_answers_sent_and_not_verified() {
        let mut plans = post_reads();
        plans.extend(passing_answers());
        plans.push(
            import_answer().with_delivery(Delivery::ResetAfterRequestProcessed {
                delay: Duration::from_millis(0),
            }),
        );
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let outcome = run.post(&ScriptedApproval::approving(), id(A)).await;
        let result = outcome.expect("the answer").payload["result"].clone();
        assert_eq!(result["attempt_recorded"], true, "{result}");
        assert_eq!(result["dispatch"]["state"], "reconciliation_required");
        assert_eq!(result["error"]["code"], "import_dispatch_outcome_unknown");
        assert!(result["error"]["message"]
            .as_str()
            .unwrap()
            .contains("NOT verified"));
        assert_eq!(run.records(), ["master_batch", "master_dispatch_intent"]);
        assert!(!run.server.post_approvals.holds(id(A)));
        assert_eq!(imports(&run.finish()), [create_bytes()], "sent once");
    }

    /// The create is lost after the intent: the post answers as today and
    /// reads nothing (its next answers are the later call's, which would not
    /// line up otherwise). A later call reconciles it: the company checked, the
    /// book read back with the item found, and the verdict open, never
    /// verified nor attributed, with no answer from Tally on file.
    #[tokio::test]
    async fn a_lost_answer_is_reconciled_by_a_later_call_with_a_read_and_never_verified() {
        let mut plans = post_reads();
        plans.extend(passing_answers());
        let posted_at = plans.len();
        plans.push(
            import_answer().with_delivery(Delivery::ResetAfterRequestProcessed {
                delay: Duration::from_millis(0),
            }),
        );
        plans.extend(identity_answers(false));
        plans.extend(read_back());
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let scripted = ScriptedApproval::approving();
        let lost = run.post(&scripted, id(A)).await.expect("the answer");
        let lost = &lost.payload["result"];
        assert_eq!(lost["error"]["code"], "import_dispatch_outcome_unknown");
        assert_eq!(lost["verification"]["attempted"], false, "{lost}");
        assert_eq!(run.records(), ["master_batch", "master_dispatch_intent"]);
        let later = run.post(&scripted, id(A)).await.expect("the answer");
        let later = &later.payload["result"];
        assert_eq!(later["state"], "reconciliation_required", "{later}");
        assert_eq!(later["answer_recorded"], false);
        let verification = &later["verification"];
        assert_eq!(verification["moment"], "reconcile");
        assert_eq!(verification["read_back"]["state"], "read");
        assert_eq!(verification["guid"], Value::Null);
        assert_eq!(
            verification["reasons"],
            json!(["later_call_does_not_verify"])
        );
        assert_eq!(
            run.records(),
            ["master_batch", "master_dispatch_intent", "master_verdict"]
        );
        let stored = journal_lines(&std::fs::read(run.dir().join(STOCK_JOURNAL)).unwrap());
        assert_eq!(stored[2]["state"], "reconciliation_required");
        assert_eq!(
            stored[2]["items"][0]["reason"],
            "later_call_does_not_verify"
        );
        assert_eq!(scripted.stock_items().len(), 1, "asked once");
        let observed = run.finish();
        assert_eq!(imports(&observed), [create_bytes()], "sent once");
        assert_eq!(sent(&observed[posted_at + 1..]), every_request());
    }

    /// The read-back at post time fails in transport (its binding marks
    /// request reset, a constructed delivery): the verdict is unconfirmed and recorded, with
    /// Tally's answer. A later call reads back cleanly (the marks one above the
    /// aim read, the item found) and still cannot verify it.
    #[tokio::test]
    async fn a_read_back_failed_at_post_time_is_unconfirmed_and_a_reconcile_cannot_verify_it() {
        let mut plans = post_reads();
        plans.extend(dispatch());
        let reset = Delivery::ResetAfterRequestProcessed {
            delay: Duration::from_millis(0),
        };
        let read_at = plans.len();
        let mut failing = read_back();
        failing.truncate(4);
        let binding = failing.pop().unwrap();
        failing.push(binding.with_delivery(reset));
        plans.extend(failing);
        plans.extend(identity_answers(false));
        plans.extend(read_back());
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let scripted = ScriptedApproval::approving();
        let posted = run.post(&scripted, id(A)).await.expect("the answer");
        let posted = &posted.payload["result"];
        assert_eq!(
            posted["state"], "masters_after_post_unconfirmed",
            "{posted}"
        );
        assert_eq!(posted["verification"]["read_back"]["state"], "failed");
        assert_eq!(
            posted["verification"]["reasons"],
            json!(["masters_unconfirmed"])
        );
        assert_eq!(posted["answer_recorded"], true);
        assert_eq!(posted["counters"]["created"], 1);
        let note = posted["note"].as_str().unwrap();
        let step = posted["next_step"].as_str().unwrap();
        assert!(note.contains("NOT verified"), "{note}");
        for text in [note, step] {
            assert!(
                !text.contains("post_") && !text.contains("prepare_"),
                "{text}"
            );
            assert!(!text.contains("ledger"), "{text}");
        }
        assert_eq!(run.records(), RECORDED);
        let later = run.post(&scripted, id(A)).await.expect("the answer");
        let later = &later.payload["result"];
        assert_eq!(later["state"], "reconciliation_required", "{later}");
        assert_eq!(later["verification"]["read_back"]["state"], "read");
        assert_eq!(
            later["verification"]["reasons"],
            json!(["later_call_does_not_verify"])
        );
        assert_eq!(later["counters"]["created"], 1, "the answer on file");
        let mut records = RECORDED.to_vec();
        records.push("master_verdict");
        assert_eq!(run.records(), records);
        let observed = run.finish();
        assert_eq!(imports(&observed), [create_bytes()], "sent once");
        let failed = [Sent::Health, Sent::Companies, Sent::Companies, Sent::Marks];
        assert_eq!(sent(&observed[read_at..read_at + 4]), failed);
        assert_eq!(sent(&observed[read_at + 4..]), every_request());
    }

    /// The read-back at post time is answered in full, but its aim marks
    /// answer does not parse (the ALTMSTID of 290 made `x`, a constructed edit
    /// of the captured marks): the verdict is unconfirmed, not open.
    #[tokio::test]
    async fn a_read_back_whose_marks_do_not_parse_is_unconfirmed() {
        let mark = "<ALTMSTID TYPE=\"Number\"> 290</ALTMSTID>";
        let unparsed = marks_at(290).replacen(mark, "<ALTMSTID TYPE=\"Number\">x</ALTMSTID>", 1);
        assert_ne!(unparsed, marks_at(290));
        let mut plans = post_reads();
        plans.extend(dispatch());
        let answers = stock_answers_at(rebound_after(), 3, marks_at(290), unparsed);
        plans.extend(standalone(answers));
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let posted = run.post(&ScriptedApproval::approving(), id(A)).await;
        let result = posted.expect("the answer").payload["result"].clone();
        assert_eq!(
            result["state"], "masters_after_post_unconfirmed",
            "{result}"
        );
        assert_eq!(result["verification"]["read_back"]["state"], "unparsed");
        assert_eq!(
            result["verification"]["reasons"],
            json!(["masters_unconfirmed"])
        );
        assert_eq!(run.records(), RECORDED);
        let observed = run.finish();
        assert_eq!(imports(&observed), [create_bytes()], "sent once");
    }

    /// Tally's answer cannot be recorded, after the send and again under the
    /// verdict's lock: a clean read-back is NOT verified (the journal admits no
    /// verified verdict without the answer), and the answer says so. Refused
    /// only once, the retry records it and the item is verified.
    #[tokio::test]
    async fn an_answer_that_cannot_be_recorded_leaves_the_item_unverified() {
        for (faults, verified) in [
            (
                vec!["master_dispatch_response", "master_dispatch_response"],
                false,
            ),
            (vec!["master_dispatch_response"], true),
        ] {
            let mut plans = post_reads();
            plans.extend(dispatch_and_read_back());
            let run = posting(plans, Redaction::None);
            record(run.dir(), &shape_batch(A, &run.origin));
            let outcome = TEST_FAULTS
                .scope(
                    std::cell::RefCell::new(faults),
                    run.post(&ScriptedApproval::approving(), id(A)),
                )
                .await;
            let result = outcome.expect("the answer").payload["result"].clone();
            assert_eq!(result["verification"]["read_back"]["state"], "read");
            assert_eq!(result["counters"]["created"], 1);
            if verified {
                assert_eq!(result["state"], "posted_verified", "{result}");
                assert_eq!(result["answer_recorded"], true);
                assert_eq!(run.records(), RECORDED);
            } else {
                assert_eq!(result["state"], "posted_unverified", "{result}");
                assert_eq!(result["answer_recorded"], false);
                assert_eq!(result["verification"]["settled"], false);
                assert_eq!(result["verification"]["verdict_recorded"], false);
                assert_eq!(result["note"], UNRECORDED_NOTE);
                let step = result["next_step"].as_str().unwrap();
                assert!(step.contains(STOCK_VERDICT_UNRECORDED), "{step}");
                assert!(step.contains(STOCK_ANSWER_UNRECORDED), "{step}");
                assert_eq!(run.records(), ["master_batch", "master_dispatch_intent"]);
            }
            assert_eq!(imports(&run.finish()), [create_bytes()], "sent once");
        }
    }

    /// Another action holds the admission lock shared from after the Create is
    /// sent (its request read in full, Tally's answer held) until the post
    /// returns: a real held lock, no injected fault. The answer's append after
    /// the send finds the lock busy, and so does the retry under the verdict's
    /// lock, so nothing past the intent is recorded. A clean read-back is then
    /// NOT verified, and the answer is a recorded send, never "nothing sent".
    /// Released while the read-back's first answer is held, after the first
    /// append failed, the retry records Tally's answer and the verdict, and the
    /// item is verified.
    #[tokio::test]
    async fn an_answer_append_under_an_admission_lock_held_after_the_send_is_a_recorded_send() {
        for released_before_retry in [false, true] {
            let (sent_gate, read_gate) = (DeliveryGate::new(), DeliveryGate::new());
            let mut plans = post_reads();
            plans.extend(passing_answers());
            let posted_at = plans.len();
            plans.push(import_answer().with_delivery(Delivery::Held(sent_gate.clone())));
            let mut reads = read_back();
            if released_before_retry {
                reads[0] = reads[0]
                    .clone()
                    .with_delivery(Delivery::Held(read_gate.clone()));
            } else {
                read_gate.open();
            }
            plans.extend(reads);
            let run = posting(plans, Redaction::None);
            record(run.dir(), &shape_batch(A, &run.origin));
            let simulator = &run.simulator;
            let until_received = move |count: usize| async move {
                tokio::time::timeout(Duration::from_secs(20), async {
                    while simulator.received() < count {
                        tokio::time::sleep(Duration::from_millis(1)).await;
                    }
                })
                .await
                .expect("the request was read");
            };
            let other_action = async {
                until_received(posted_at + 1).await;
                assert_eq!(run.records(), ["master_batch", "master_dispatch_intent"]);
                let held = (run.server.lock_import_admission_shared())
                    .expect("the post holds no lock while Tally's answer is held");
                sent_gate.open();
                if released_before_retry {
                    until_received(posted_at + 2).await;
                    drop(held);
                    read_gate.open();
                    None
                } else {
                    Some(held)
                }
            };
            let scripted = ScriptedApproval::approving();
            let (outcome, held) = tokio::join!(run.post(&scripted, id(A)), other_action);
            drop(held);
            let result = outcome.expect("not a refusal").payload["result"].clone();
            assert_eq!(result["attempt_recorded"], true, "{result}");
            assert_eq!(result["dispatch"]["state"], "dispatched");
            assert_eq!(result["verification"]["read_back"]["state"], "read");
            assert_eq!(result["counters"]["created"], 1);
            let text = result.to_string();
            for not_sent in ["Nothing was sent", "sent nothing", "Nothing was written"] {
                assert!(!text.contains(not_sent), "{not_sent}: {result}");
            }
            if released_before_retry {
                assert_eq!(result["state"], "posted_verified", "{result}");
                assert_eq!(result["answer_recorded"], true);
                assert_eq!(run.records(), RECORDED, "recorded on the retry");
            } else {
                assert_eq!(result["state"], "posted_unverified", "{result}");
                assert_eq!(result["answer_recorded"], false);
                assert_eq!(result["verification"]["verdict_recorded"], false);
                assert_eq!(
                    result["verification"]["verdict_record_error"],
                    "import_admission_busy"
                );
                assert_eq!(result["note"], UNRECORDED_NOTE);
                let step = result["next_step"].as_str().unwrap();
                assert!(step.contains("never post this batch again"), "{step}");
                assert!(step.contains(STOCK_ANSWER_UNRECORDED), "{step}");
                assert_eq!(
                    run.records(),
                    ["master_batch", "master_dispatch_intent"],
                    "not recorded on the retry either"
                );
            }
            assert_eq!(imports(&run.finish()), [create_bytes()], "sent once");
        }
    }

    /// The writer's refusals through the tool, by the journal faults the
    /// ledger tests arm: one refused before any byte of the intent (nothing
    /// recorded, nothing sent), and one written and then reported failed (the
    /// intent stands, so the answer is a recorded send, NOT verified).
    #[tokio::test]
    async fn the_writers_refusals_reach_the_answer_through_the_queue() {
        for (point, recorded) in [
            ("master_dispatch_intent", false),
            ("master_dispatch_intent_lost_after_write", true),
        ] {
            let mut plans = post_reads();
            plans.extend(passing_answers());
            let run = posting(plans, Redaction::None);
            record(run.dir(), &shape_batch(A, &run.origin));
            let outcome = TEST_FAULTS
                .scope(
                    std::cell::RefCell::new(vec![point]),
                    run.post(&ScriptedApproval::approving(), id(A)),
                )
                .await;
            if recorded {
                let result = outcome.expect("the answer").payload["result"].clone();
                assert_eq!(result["attempt_recorded"], true, "{result}");
                assert_eq!(result["error"]["code"], "import_dispatch_outcome_unknown");
                assert_eq!(run.records(), ["master_batch", "master_dispatch_intent"]);
            } else {
                assert_eq!(
                    refused_code(&outcome),
                    UnderLockRefusal::MasterJournalRefused.code()
                );
                assert_eq!(run.records(), ["master_batch"]);
            }
            assert!(imports(&run.finish()).is_empty(), "{point}: nothing sent");
        }
    }

    /// A read holds the admission lock shared through the post, so the queue's
    /// closure finds it busy after every queue read: the lock's own code, no
    /// import, no intent and no sidecar.
    #[tokio::test]
    async fn a_post_whose_admission_lock_is_held_shared_answers_busy_and_records_nothing() {
        let mut plans = post_reads();
        plans.extend(passing_answers());
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let held = (run.server.lock_import_admission_shared())
            .expect("no one holds the lock before the post");
        let scripted = ScriptedApproval::approving();
        let outcome = run.post(&scripted, id(A)).await;
        drop(held);
        assert_eq!(refused_code(&outcome), "import_admission_busy");
        assert_eq!(scripted.stock_items().len(), 1, "asked once");
        assert_eq!(run.records(), ["master_batch"], "no intent");
        assert!(!run
            .dir()
            .join(format!("master-pre-image-{}.json", id(A)))
            .exists());
        let observed = run.finish();
        assert_eq!(imports(&observed).len(), 0, "no import request");
        assert_eq!(
            sent(&observed),
            [every_request(), QUEUE_READS.to_vec()].concat(),
            "the queue read everything before the lock"
        );
    }

    /// The writer's refusals as the queue takes them: a token or approval of
    /// another batch, or another digest, is a changed batch; the rest pass
    /// through.
    #[test]
    fn the_writers_refusal_maps_to_the_queues_refusal() {
        use BeforeDispatchError::{Other, Refused};
        for refusal in [
            StockIntentRefusal::OtherBatch,
            StockIntentRefusal::OtherDigest,
        ] {
            assert!(matches!(
                stock_intent_refusal(refusal),
                Refused(UnderLockRefusal::BatchChanged)
            ));
        }
        assert!(matches!(
            stock_intent_refusal(StockIntentRefusal::Refused(
                UnderLockRefusal::AlreadyAttempted
            )),
            Refused(UnderLockRefusal::AlreadyAttempted)
        ));
        assert!(matches!(
            stock_intent_refusal(StockIntentRefusal::Other("x_y".into())),
            Other(code) if code == "x_y"
        ));
    }

    /// The owed check: an approval whose bytes digest to another value than
    /// the batch's is refused. Through the tool it cannot be reached with the
    /// rebuild check and the held-item check in place (the take ties the
    /// approved bytes to the held item, and the held item to the stored
    /// writes), so it is tested here, on a hand edit of the stored digest.
    #[tokio::test]
    async fn the_owed_check_refuses_an_approval_of_other_bytes_than_the_batch() {
        let approved = approved_stock(id(A)).await;
        let batch = shape_batch(A, "http://127.0.0.1:9000");
        assert!(owed_digest(&approved, &batch).is_ok());
        let other = hand_edited(&batch, |line| line["batch_sha256"] = json!("e".repeat(64)));
        let refused = owed_digest(&approved, &other).expect_err("other bytes");
        assert_eq!(refused.code, "import_batch_changed");
    }

    async fn until_answered(server: &Server, batch_id: &str) {
        tokio::time::timeout(Duration::from_secs(5), async {
            while !server.post_approvals.answered_for_test(batch_id) {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("the held dialog's answer was stamped");
    }

    /// A dialog left open by one call and clicked while no call waits is
    /// posted by the next call: its first pass joins the answered dialog and
    /// hands it to a redeem-only pass, which reads the book again and sends,
    /// as the ledger post's test of the same flow shows. One dialog in all.
    #[tokio::test]
    async fn a_click_after_the_call_returned_is_redeemed_by_the_next_call() {
        let mut plans = post_reads();
        let first = plans.len();
        plans.extend(post_reads());
        plans.extend(dispatch_and_read_back());
        let run = posting(plans, Redaction::None);
        record(run.dir(), &shape_batch(A, &run.origin));
        let scripted = ScriptedApproval::held();
        let pending = run.post(&scripted, id(A)).await.expect("pending");
        assert_eq!(pending.payload["result"]["approval"]["state"], "pending");
        assert_eq!(run.records(), ["master_batch"]);
        scripted.answer(true);
        until_answered(&run.server, id(A)).await;
        let posted = run.post(&scripted, id(A)).await.expect("posted");
        assert_eq!(posted.payload["result"]["state"], "posted_verified");
        assert_eq!(scripted.stock_items().len(), 1, "one dialog, asked once");
        assert_eq!(run.records(), RECORDED);
        let observed = run.finish();
        assert_eq!(sent(&observed[..first]), every_request());
        assert_eq!(imports(&observed), [create_bytes()]);
    }
}
