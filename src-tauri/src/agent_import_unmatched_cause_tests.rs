//! bridge#1108: what a native post's own answer from Tally says about the
//! vouchers its readback cannot find. The Silver answers (one voucher, and
//! two batches Tally rejected whole) and the batch of 50 are live responses
//! committed byte for byte; the Education answers are
//! derived from live captures (counter shape only, see
//! EDUCATION_IMPORT_COUNTERS_PROVENANCE.md). Tests marked synthetic exist only
//! to hold the guards that no capture reaches.
//! The 7.2 answer of a Journal Tally refused for a Stock-in-Hand line is committed byte for
//! byte too.
use super::*;
use super::{
    later_unmatched_cause, mark_sent_not_attributed_after_partly_created_answer,
    measured_voucher_step, partly_created_sentence, reported_not_created_count, unmatched_cause,
    UnmatchedCause, SENT_NOT_ATTRIBUTED_AFTER_UNCLEAN_ANSWER_NEXT_STEP,
};
use bridge_tally_protocol::{parse_import_outcome, TallyImportResult};
use serde_json::{json, Value};

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn counters(xml: &str) -> TallyImportResult {
    parse_import_outcome(xml)
        .expect("the captured answer parses")
        .counters()
        .clone()
}

/// A rejected single voucher, Education mode (CREATED 0, EXCEPTIONS 1;
/// derived from a live capture, its LINEERROR text redacted).
fn rejected_one_education() -> TallyImportResult {
    counters(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/live_education_w7_baddate_sanitized.xml"
    ))
}

/// A rejected single voucher, licensed 7.1 Silver (CREATED 0, EXCEPTIONS 1).
fn rejected_one_silver() -> TallyImportResult {
    counters(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/single-import-missing-ledger.utf16le.xml"
    )))
}

/// A rejected single Journal, licensed 7.2, refused for a different cause than the
/// others: a line on a ledger under Stock-in-Hand (CREATED 0, EXCEPTIONS 1, the `LINEERROR`
/// ahead of the counters; row 1108-A1, one run, `PARTIAL`).
fn rejected_one_stock_in_hand_journal() -> TallyImportResult {
    counters(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/single-import-stock-in-hand-journal.utf16le.xml"
    )))
}

/// A live batch of 50 that committed 49 (CREATED 49, EXCEPTIONS 1).
fn committed_49_of_50() -> TallyImportResult {
    counters(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/import_line_error_partial_commit_live.utf16le.xml"
    )))
}

/// One clean create (CREATED 1, nothing else).
fn created_one() -> TallyImportResult {
    counters(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/live_education_w4_voucher_sanitized.xml"
    ))
}

/// Three vouchers, each naming a different missing ledger, licensed 7.1
/// Silver (CREATED 0, EXCEPTIONS 3).
fn rejected_all_three() -> TallyImportResult {
    counters(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/batch-import-all-missing-ledgers.utf16le.xml"
    )))
}

/// Two rejected vouchers carrying three missing ledgers between them,
/// licensed 7.1 Silver (CREATED 0, EXCEPTIONS 2: one per voucher).
fn rejected_two_carrying_three_missing_ledgers() -> TallyImportResult {
    counters(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/batch-import-two-missing-ledgers-in-one-voucher.utf16le.xml"
    )))
}

#[test]
fn the_captured_answers_are_what_these_tests_say() {
    let education = rejected_one_education();
    assert_eq!(
        (education.created, education.altered, education.exceptions),
        (0, 0, 1)
    );
    let silver = rejected_one_silver();
    assert_eq!(
        (silver.created, silver.altered, silver.exceptions),
        (0, 0, 1)
    );
    let stock = rejected_one_stock_in_hand_journal();
    assert!(stock.counter_presence.all_reported(), "{stock:?}");
    assert_eq!(
        (stock.created, stock.altered, stock.errors, stock.exceptions),
        (0, 0, 0, 1)
    );
    let partial = committed_49_of_50();
    assert_eq!(
        (partial.created, partial.altered, partial.exceptions),
        (49, 0, 1)
    );
    let clean = created_one();
    assert_eq!((clean.created, clean.altered, clean.exceptions), (1, 0, 0));
    for (answer, exceptions) in [
        (rejected_all_three(), 3),
        (rejected_two_carrying_three_missing_ledgers(), 2),
    ] {
        assert!(answer.counter_presence.all_reported(), "{answer:?}");
        assert_eq!(
            (
                answer.created,
                answer.altered,
                answer.errors,
                answer.exceptions
            ),
            (0, 0, 0, exceptions)
        );
    }
}

#[test]
fn a_rejected_single_voucher_is_reported_not_created() {
    for answer in [
        rejected_one_education(),
        rejected_one_silver(),
        rejected_one_stock_in_hand_journal(),
    ] {
        assert!(answer.counter_presence.all_reported(), "{answer:?}");
        assert_eq!(
            unmatched_cause(Some(&answer), 1, 1, None),
            UnmatchedCause::ReportedNotCreated,
            "{answer:?}"
        );
    }
}

#[test]
fn a_partial_commit_is_never_read_as_not_created() {
    // A count does not say which voucher Tally rejected: an edited voucher it
    // did create could otherwise carry the label.
    for unmatched in [1, 2] {
        assert_eq!(
            unmatched_cause(Some(&committed_49_of_50()), 50, unmatched, Some(49)),
            UnmatchedCause::NotEstablished
        );
    }
}

#[test]
fn one_exception_in_a_batch_of_three_is_never_read_as_not_created() {
    // The Stock-in-Hand refusal counts one exception. Sent in a batch of three, that count does
    // not say which voucher Tally rejected (the other two may be twins), so none is labelled,
    // whatever the voucher mark did and however many the readback cannot find.
    // The count mismatch (one exception against three sent) alone decides it, so the four
    // combinations below are one case seen four ways, not four separate cases; the step and
    // unmatched branches are tested by the other tests of this file.
    for (unmatched, step) in [(1, Some(0)), (1, Some(2)), (3, Some(0)), (1, None)] {
        assert_eq!(
            unmatched_cause(
                Some(&rejected_one_stock_in_hand_journal()),
                3,
                unmatched,
                step
            ),
            UnmatchedCause::NotEstablished,
            "{unmatched} {step:?}"
        );
    }
}

#[test]
fn a_rejected_voucher_found_after_all_is_not_labelled() {
    // Entered by hand with the same content since, it is found: nothing is
    // claimed about it.
    assert_eq!(
        unmatched_cause(Some(&rejected_one_silver()), 1, 0, None),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn a_clean_answer_never_reports_a_voucher_not_created() {
    assert_eq!(
        unmatched_cause(Some(&created_one()), 1, 1, None),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn no_recorded_answer_establishes_nothing() {
    assert_eq!(
        unmatched_cause(None, 1, 1, None),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn an_answer_with_a_counter_missing_establishes_nothing() {
    // Synthetic: an omitted counter is not an observed zero (§9.2).
    let mut missing = rejected_one_silver();
    missing.counter_presence.exceptions = false;
    assert_eq!(
        unmatched_cause(Some(&missing), 1, 1, None),
        UnmatchedCause::NotEstablished
    );
}

/// Sets one counter of an answer.
type Bump = fn(&mut TallyImportResult);

#[test]
fn any_other_counter_establishes_nothing() {
    // Synthetic, one counter at a time: only the measured shape is read. A
    // voucher Tally created and also reported as an exception is not read as
    // not created.
    let cases: [(&str, Bump); 6] = [
        ("created", |c| c.created = 1),
        ("altered", |c| c.altered = 1),
        ("deleted", |c| c.deleted = 1),
        ("ignored", |c| c.ignored = 1),
        ("errors", |c| c.errors = 1),
        ("cancelled", |c| c.cancelled = 1),
    ];
    for (name, set) in cases {
        let mut answer = rejected_one_silver();
        set(&mut answer);
        assert_eq!(
            unmatched_cause(Some(&answer), 1, 1, None),
            UnmatchedCause::NotEstablished,
            "{name}"
        );
    }
}

#[test]
fn fewer_exceptions_than_vouchers_establishes_nothing() {
    // Synthetic: two sent, none created, one exception. The second voucher's
    // fate is not reported, so neither is labelled.
    assert_eq!(
        unmatched_cause(Some(&rejected_one_silver()), 2, 2, Some(0)),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn no_exception_establishes_nothing() {
    // Synthetic: none created and no exception reported.
    let silent = TallyImportResult {
        exceptions: 0,
        line_error_count: 0,
        ..rejected_one_silver()
    };
    assert_eq!(
        unmatched_cause(Some(&silent), 1, 1, None),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn a_batch_tally_rejected_whole_is_reported_not_created_when_its_mark_did_not_move() {
    for (answer, sent) in [
        (rejected_all_three(), 3),
        (rejected_two_carrying_three_missing_ledgers(), 2),
    ] {
        assert_eq!(
            unmatched_cause(Some(&answer), sent, sent as u64, Some(0)),
            UnmatchedCause::ReportedNotCreated,
            "{answer:?}"
        );
    }
}

#[test]
fn a_batch_rejected_whole_needs_its_voucher_mark_measured_unmoved() {
    // A mark not read on both sides, or one that moved although Tally said it
    // created nothing, establishes nothing for a batch.
    for step in [None, Some(1)] {
        assert_eq!(
            unmatched_cause(Some(&rejected_all_three()), 3, 3, step),
            UnmatchedCause::NotEstablished,
            "{step:?}"
        );
    }
}

#[test]
fn a_batch_with_a_voucher_found_is_never_read_as_not_created() {
    // Synthetic count: one of the three rejected vouchers matched by content
    // (for example entered by hand since), so nothing is claimed about any.
    assert_eq!(
        unmatched_cause(Some(&rejected_all_three()), 3, 2, Some(0)),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn more_exceptions_than_vouchers_establishes_nothing() {
    // Synthetic: two exceptions for one voucher is not the captured shape.
    let doubled = TallyImportResult {
        exceptions: 2,
        ..rejected_one_silver()
    };
    assert_eq!(
        unmatched_cause(Some(&doubled), 1, 1, None),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn an_empty_post_establishes_nothing() {
    // Synthetic: nothing sent, nothing to label.
    let empty = TallyImportResult {
        exceptions: 0,
        line_error_count: 0,
        ..rejected_one_silver()
    };
    assert_eq!(
        unmatched_cause(Some(&empty), 0, 0, Some(0)),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn a_voucher_step_is_measured_only_when_both_marks_were_read_and_did_not_go_down() {
    assert_eq!(measured_voucher_step(Some(10), Some(10)), Some(0));
    assert_eq!(measured_voucher_step(Some(10), Some(12)), Some(2));
    // A mark that reads lower after the post is no step at all: never 0.
    assert_eq!(measured_voucher_step(Some(10), Some(9)), None);
    assert_eq!(measured_voucher_step(None, Some(10)), None);
    assert_eq!(measured_voucher_step(Some(10), None), None);
}

/// A later verification reads what the post left on record: a post Tally
/// rejected whole is "reported as not created when sent", with no count of
/// what is found now. One voucher needs the answer alone; a batch also needs
/// the post's own verdict that its voucher step matched. It is never
/// `ReportedNotCreated`, which only the post's own readback gives.
#[test]
fn a_later_verification_reads_what_the_post_left_on_record() {
    for (answer, sent, step_matched) in [
        (rejected_one_education(), 1, false),
        (rejected_one_silver(), 1, false),
        (rejected_all_three(), 3, true),
        (rejected_two_carrying_three_missing_ledgers(), 2, true),
    ] {
        assert_eq!(
            later_unmatched_cause(Some(&answer), sent, step_matched),
            UnmatchedCause::ReportedNotCreatedWhenSent,
            "{answer:?}"
        );
    }
    // A batch whose post did not record its step as matched: nothing.
    for (answer, sent) in [
        (rejected_all_three(), 3),
        (rejected_two_carrying_three_missing_ledgers(), 2),
    ] {
        assert_eq!(
            later_unmatched_cause(Some(&answer), sent, false),
            UnmatchedCause::NotEstablished,
            "{answer:?}"
        );
    }
}

/// Every other saved answer, and none, establishes nothing for a later
/// verification: a partial commit, a clean create, a missing counter, any
/// other counter set, fewer or more exceptions than vouchers, an empty post.
#[test]
fn a_later_verification_establishes_nothing_from_any_other_answer() {
    assert_eq!(
        later_unmatched_cause(None, 1, true),
        UnmatchedCause::NotEstablished
    );
    let mut missing = rejected_one_silver();
    missing.counter_presence.exceptions = false;
    let doubled = TallyImportResult {
        exceptions: 2,
        ..rejected_one_silver()
    };
    let empty = TallyImportResult {
        exceptions: 0,
        line_error_count: 0,
        ..rejected_one_silver()
    };
    for (name, answer, sent) in [
        ("49 of 50 committed", committed_49_of_50(), 50),
        ("one clean create", created_one(), 1),
        ("a counter missing", missing, 1),
        ("fewer exceptions than vouchers", rejected_one_silver(), 2),
        ("more exceptions than vouchers", doubled, 1),
        ("nothing sent", empty, 0),
    ] {
        assert_eq!(
            later_unmatched_cause(Some(&answer), sent, true),
            UnmatchedCause::NotEstablished,
            "{name}"
        );
    }
    let cases: [(&str, Bump); 6] = [
        ("created", |c| c.created = 1),
        ("altered", |c| c.altered = 1),
        ("deleted", |c| c.deleted = 1),
        ("ignored", |c| c.ignored = 1),
        ("errors", |c| c.errors = 1),
        ("cancelled", |c| c.cancelled = 1),
    ];
    for (name, set) in cases {
        let mut answer = rejected_one_silver();
        set(&mut answer);
        assert_eq!(
            later_unmatched_cause(Some(&answer), 1, true),
            UnmatchedCause::NotEstablished,
            "{name}"
        );
    }
}

// bridge#1108, option A: a post that landed in part says how many of its
// vouchers Tally reported as not created, and names none.

/// Row P2 of the lab sitting (`IMPORT_POST_PARTIAL_LANDING_CAPTURE_PROVENANCE.md`): three
/// vouchers sent, the middle one refused (CREATED 2, EXCEPTIONS 1).
fn landed_in_part_one_refused() -> TallyImportResult {
    counters(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/import_post_partial_landing_p2_live.utf16le.xml"
    )))
}

/// Row P3: the same post with the two vouchers Tally created identical twins
/// (CREATED 2, EXCEPTIONS 1).
fn landed_in_part_twins() -> TallyImportResult {
    counters(&utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/import_post_partial_landing_p3_live.utf16le.xml"
    )))
}

#[test]
fn the_captured_partial_landing_answers_are_what_these_tests_say() {
    for answer in [landed_in_part_one_refused(), landed_in_part_twins()] {
        assert!(answer.counter_presence.all_reported(), "{answer:?}");
        assert_eq!(
            (
                answer.created,
                answer.altered,
                answer.deleted,
                answer.ignored,
                answer.errors,
                answer.cancelled,
                answer.exceptions
            ),
            (2, 0, 0, 0, 0, 0, 1)
        );
    }
}

#[test]
fn a_post_that_landed_in_part_reports_how_many_vouchers_tally_said_were_not_created() {
    for answer in [landed_in_part_one_refused(), landed_in_part_twins()] {
        assert_eq!(
            reported_not_created_count(&answer, 3, |created| created == 2),
            Some(1)
        );
    }
    // Also the live batch of 50 that committed 49.
    assert_eq!(
        reported_not_created_count(&committed_49_of_50(), 50, |created| created == 49),
        Some(1)
    );
}

#[test]
fn the_count_needs_the_voucher_mark_confirmed_by_what_tally_reported_creating() {
    let answer = landed_in_part_one_refused();
    let mut asked = None;
    assert_eq!(
        reported_not_created_count(&answer, 3, |created| {
            asked = Some(created);
            false
        }),
        None
    );
    // The mark is asked about with what Tally reported creating.
    assert_eq!(asked, Some(2));
    assert_eq!(reported_not_created_count(&answer, 3, |_| true), Some(1));
}

#[test]
fn no_other_answer_reports_a_count() {
    let on = |answer: &TallyImportResult, sent: usize| {
        reported_not_created_count(answer, sent, |_| true)
    };
    // Rejected whole, rejected alone, clean, and no exception reported.
    assert_eq!(on(&rejected_all_three(), 3), None);
    assert_eq!(on(&rejected_one_silver(), 1), None);
    assert_eq!(on(&created_one(), 1), None);
    // The sum is not what was sent, either way, and nothing was sent.
    let partial = landed_in_part_one_refused();
    assert_eq!(on(&partial, 4), None);
    assert_eq!(on(&partial, 2), None);
    assert_eq!(on(&partial, 0), None);
    // An omitted counter is not an observed zero (§9.2).
    let cases: [(&str, Bump); 7] = [
        ("created missing", |c| c.counter_presence.created = false),
        ("exceptions missing", |c| {
            c.counter_presence.exceptions = false
        }),
        ("altered", |c| c.altered = 1),
        ("deleted", |c| c.deleted = 1),
        ("ignored", |c| c.ignored = 1),
        ("errors", |c| c.errors = 1),
        ("cancelled", |c| c.cancelled = 1),
    ];
    for (name, set) in cases {
        let mut answer = landed_in_part_one_refused();
        set(&mut answer);
        assert_eq!(on(&answer, 3), None, "{name}");
    }
}

/// The whole of what the line says, so a change of one word is seen.
#[test]
fn the_count_sentence_names_no_voucher_and_says_which_is_not_known() {
    assert_eq!(
        partly_created_sentence(1, 3),
        "Tally's answer to this post reported 1 of the 3 vouchers as not created; ComplyEaze Bridge cannot say which of the vouchers it sent was that one."
    );
    assert_eq!(
        partly_created_sentence(2, 5),
        "Tally's answer to this post reported 2 of the 5 vouchers as not created; ComplyEaze Bridge cannot say which of the vouchers it sent were those."
    );
}

/// A verification result as the post's readback writes it, before any rewrite.
fn readback(rows: &[(&str, &str)]) -> Value {
    let unmatched = rows
        .iter()
        .filter(|(_, status)| *status == "not_found")
        .count();
    json!({
        "vouchers": rows
            .iter()
            .map(|(id, status)| json!({"bridge_txn_id": id, "status": status}))
            .collect::<Vec<_>>(),
        "counts": {
            "not_found": unmatched,
            "bound_not_in_window": 0,
            "sent_not_attributed": 0,
            "matching_content_observed": rows.iter().filter(|(_, s)| *s == "matching_content_observed").count(),
            "duplicate_fingerprint": rows.iter().filter(|(_, s)| *s == "duplicate_fingerprint").count(),
        },
    })
}

/// The pin of the issue's rule: whatever the rows are, the count rewrite names
/// no row as one Tally did not create, and never says one was not posted. It
/// changes only a row the readback could not find, and only its next step.
#[test]
fn the_count_line_reads_no_row_as_not_created_and_never_says_not_posted() {
    let sentence = partly_created_sentence(1, 3);
    let line = format!("{SENT_NOT_ATTRIBUTED_AFTER_UNCLEAN_ANSWER_NEXT_STEP} {sentence}");
    // P2 (the middle voucher not found), P3 (twins found, the middle not),
    // and the shapes the counts alone do not exclude: the refused voucher
    // found by an equal one, a created one not found, every row not found.
    let shapes: [&[(&str, &str)]; 5] = [
        &[
            ("a", "matching_content_observed"),
            ("b", "not_found"),
            ("c", "matching_content_observed"),
        ],
        &[
            ("a", "duplicate_fingerprint"),
            ("b", "not_found"),
            ("c", "duplicate_fingerprint"),
        ],
        &[
            ("a", "matching_content_observed"),
            ("b", "matching_content_observed"),
            ("c", "matching_content_observed"),
        ],
        &[
            ("a", "not_found"),
            ("b", "matching_content_observed"),
            ("c", "not_found"),
        ],
        &[("a", "not_found"), ("b", "not_found"), ("c", "not_found")],
    ];
    for rows in shapes {
        let before = readback(rows);
        let mut after = before.clone();
        mark_sent_not_attributed_after_partly_created_answer(&mut after, 1, 3);
        let vouchers = after["vouchers"].as_array().unwrap();
        assert_eq!(vouchers.len(), rows.len());
        for (at, (row, (id, status))) in vouchers.iter().zip(rows).enumerate() {
            assert_eq!(row["bridge_txn_id"], *id);
            if *status == "not_found" {
                assert_eq!(row["status"], "sent_not_attributed", "{rows:?}");
                assert_eq!(row["next_step"], line.as_str(), "{rows:?}");
            } else {
                // A row that was found is left exactly as it was.
                assert_eq!(row, &before["vouchers"][at], "{rows:?}");
                assert!(row.get("next_step").is_none(), "{rows:?}");
            }
            assert_ne!(row["status"], "tally_reported_not_created", "{rows:?}");
            let words = row["next_step"].as_str().unwrap_or_default().to_lowercase();
            for banned in [
                "not posted",
                "was not created",
                "did not create",
                "rejected",
            ] {
                assert!(!words.contains(banned), "{banned}: {words}");
            }
        }
        // No row counts as one Tally reported not created, and the counts
        // still add up to the rows.
        assert_eq!(
            after["counts"].get("tally_reported_not_created"),
            None,
            "{rows:?}"
        );
        assert_eq!(after["counts"]["not_found"], 0);
        let unmatched = rows.iter().filter(|(_, s)| *s == "not_found").count();
        assert_eq!(
            after["counts"]["sent_not_attributed"], unmatched,
            "{rows:?}"
        );
        assert_eq!(
            after["counts"]["matching_content_observed"],
            before["counts"]["matching_content_observed"]
        );
        assert_eq!(
            after["counts"]["duplicate_fingerprint"],
            before["counts"]["duplicate_fingerprint"]
        );
    }
}

// ---------------------------------------------------------------------------
// #2058: the readback's own matching, held at its edges. The rows and batches
// are written here and do not claim a live Tally capture; each test names the
// rule it holds.

const BATCH_COMPANY: &str = "company-guid-synthetic";

// The days are joined with `concat!` so that no line of this file holds a run
// of eight digits; the readback reads each as one `YYYYMMDD` date.
const FIRST_DAY: &str = concat!("2026", "0901");
const SECOND_DAY: &str = concat!("2026", "0902");
const THIRD_DAY: &str = concat!("2026", "0903");

fn day(text: &str) -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse(text).unwrap()
}

fn read_entry(ledger: &str, amount: &str, positive: &str) -> ReadEntry {
    ReadEntry {
        ledger: ledger.into(),
        amount: amount.into(),
        is_deemed_positive: positive.into(),
    }
}

/// A Journal row of 10.00 between two ledgers, number `n` in the book: GUID
/// `row-guid-<n>`, MasterID `<n>`, AlterID `100 + n`, 1 Sep 2026.
fn read_row(n: u64) -> ReadVoucher {
    ReadVoucher {
        remote_id: None,
        guid: Some(format!("row-guid-{n}")),
        alter_id: Some(100 + n),
        date: day(FIRST_DAY),
        voucher_type: Some("Journal".into()),
        narration: None,
        voucher_number: None,
        master_id: Some(n.to_string()),
        cancelled: Some(false),
        optional: Some(false),
        effective_date: None,
        entries: vec![
            read_entry("Ledger Alpha", "-10.00", "Yes"),
            read_entry("Ledger Beta", "10.00", "No"),
        ],
    }
}

fn source_of(rows: Vec<ReadVoucher>) -> ImportReadSource {
    ImportReadSource::admit(rows).unwrap()
}

fn journal_voucher(txn: &str) -> Value {
    json!({"bridge_txn_id":txn, "date":FIRST_DAY, "voucher_type":"Journal",
        "narration":null, "reference":null, "voucher_number":null,
        "entries":[{"ledger":"Ledger Alpha", "amount":"10.00", "side":"Dr"},
            {"ledger":"Ledger Beta", "amount":"10.00", "side":"Cr"}]})
}

fn batch_of(mark: Option<u64>, vouchers: Vec<Value>) -> ImportLedgerLine {
    serde_json::from_value(json!({
        "batch_id":"batch-verify-synthetic", "company_guid":BATCH_COMPANY, "company":null,
        "txn_ids":[], "date_from":FIRST_DAY, "date_to":FIRST_DAY,
        "sha256":"hash", "built_at":"2026-09-07T00:00:00Z", "status":"built",
        "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water", "value":mark, "master_value":10},
        "vouchers":vouchers
    }))
    .unwrap()
}

fn verified(
    line: &ImportLedgerLine,
    rows: Vec<ReadVoucher>,
    attribution: Attribution<'_>,
) -> Value {
    verify_batch(line, &source_of(rows), attribution).unwrap()
}

fn status_of(result: &Value, index: usize) -> &str {
    result["vouchers"][index]["status"].as_str().unwrap()
}

#[test]
fn a_window_row_must_carry_both_halves_of_its_identity() {
    let mut no_guid = read_row(1);
    no_guid.guid = None;
    let mut no_alter_id = read_row(2);
    no_alter_id.alter_id = None;
    for row in [no_guid, no_alter_id] {
        assert_eq!(
            verification_window_identities(&source_of(vec![row]), FIRST_DAY, FIRST_DAY),
            Err("verification_incomplete:window_not_corroborated".to_string())
        );
    }
}

#[test]
fn a_window_row_is_identified_by_its_stored_guid_and_alter_id_and_every_row_is_read() {
    let window = source_of(vec![read_row(1), read_row(2)]);
    assert_eq!(
        verification_window_identities(&window, FIRST_DAY, FIRST_DAY),
        Ok(BTreeSet::from([
            ("row-guid-1".to_string(), 101),
            ("row-guid-2".to_string(), 102)
        ]))
    );
}

#[test]
fn an_unreadable_corroboration_is_never_read_as_an_empty_window() {
    let mut no_guid = read_row(1);
    no_guid.guid = None;
    assert_eq!(
        corroborate_verification_window(
            &source_of(vec![]),
            &source_of(vec![no_guid]),
            FIRST_DAY,
            FIRST_DAY
        ),
        Err("verification_incomplete:window_not_corroborated".to_string())
    );
}

#[test]
fn a_fingerprint_serialises_as_its_own_shape() {
    let content = VerificationFingerprint::Content(
        Some(FIRST_DAY.into()),
        Some("Journal".into()),
        vec!["entry".into()],
    );
    let numbered =
        VerificationFingerprint::Numbered(Some(FIRST_DAY.into()), Some("Sales".into()), "7".into());
    assert_eq!(
        serde_json::to_value(&content).unwrap(),
        json!([FIRST_DAY, "Journal", ["entry"]])
    );
    assert_eq!(
        serde_json::to_value(&numbered).unwrap(),
        json!({"numbered":[FIRST_DAY, "Sales", "7"]})
    );
}

#[test]
fn an_invoice_is_recognised_by_figures_only_beside_an_unsettled_twin() {
    assert_eq!(
        InvoiceIdentity::beside(Some("twin")),
        InvoiceIdentity::ByNumberOrFigures
    );
    assert_eq!(InvoiceIdentity::beside(None), InvoiceIdentity::ByNumber);
}

#[test]
fn a_group_is_consumed_once_any_removal_took_a_candidate() {
    let mut group = VerificationCandidates::default();
    group.insert(1, true);
    group.insert(2, false);
    group.consume(5);
    assert!(!group.consumed, "an index the group never held");
    group.consume(1);
    assert!(group.consumed);
    group.consume(5);
    assert!(group.consumed, "a later miss does not undo it");
    assert_eq!(group.remaining, BTreeSet::from([2]));
    assert!(group.after_mark.is_empty());
}

fn span_binding(txn: &str, n: u64) -> super::super::span_identity::PostedVoucherIdentity {
    super::super::span_identity::PostedVoucherIdentity {
        bridge_txn_id: txn.into(),
        guid: format!("row-guid-{n}"),
        master_id: n,
    }
}

fn tagged_row(n: u64, txn: &str) -> ReadVoucher {
    let mut row = read_row(n);
    row.narration = Some(format!("[BRIDGE:{txn}]"));
    row
}

#[test]
fn an_amount_or_identity_the_book_cannot_state_refuses_the_whole_readback() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let mut bad_amount = read_row(1);
    bad_amount.entries[0].amount = "not-an-amount".into();
    assert_eq!(
        verify_batch(
            &line,
            &ImportReadSource {
                rows: vec![bad_amount]
            },
            Attribution::Tag
        ),
        Err("import_verification_amount_invalid".to_string())
    );
    let mut nameless = read_row(1);
    nameless.guid = Some(" ".into());
    nameless.master_id = None;
    assert_eq!(
        verify_batch(
            &line,
            &ImportReadSource {
                rows: vec![nameless]
            },
            Attribution::Tag
        ),
        Err("import_verification_identity_invalid".to_string())
    );
}

#[test]
fn a_row_is_after_the_mark_only_by_a_later_alter_id_and_only_when_there_is_a_mark() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let attribution = || Attribution::Span(None);
    let mut at_mark = read_row(1);
    at_mark.alter_id = Some(100);
    let mut no_alter_id = read_row(1);
    no_alter_id.alter_id = None;
    let after = verified(&line, vec![read_row(1)], attribution());
    assert_eq!(status_of(&after, 0), "matching_content_observed");
    assert_eq!(after["counts"]["matching_content_observed"], 1);
    for before in [at_mark, no_alter_id] {
        let result = verified(&line, vec![before], attribution());
        assert_eq!(
            result["vouchers"][0],
            json!({"bridge_txn_id":"t1", "status":"not_attributable",
                "marker":"accounting_fingerprint",
                "reason":"fingerprint_precedes_pre_import_voucher_mark"})
        );
        assert_eq!(result["counts"]["not_attributable"], 1);
    }
    let unmarked = batch_of(None, vec![journal_voucher("t1")]);
    let result = verified(&unmarked, vec![read_row(1)], attribution());
    assert_eq!(
        result["vouchers"][0],
        json!({"bridge_txn_id":"t1", "status":"not_found"})
    );
    assert_eq!(result["counts"]["not_found"], 1);
    assert_eq!(result["counts"]["not_attributable"], 0);
}

#[test]
fn a_tagged_row_before_the_mark_is_named_by_its_tag() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let mut early = tagged_row(1, "t1");
    early.alter_id = Some(90);
    let result = verified(&line, vec![early], Attribution::Tag);
    assert_eq!(
        result["vouchers"][0],
        json!({"bridge_txn_id":"t1", "status":"not_attributable", "marker":"narration_tag",
            "reason":"tag_precedes_pre_import_voucher_mark"})
    );
    assert_eq!(result["counts"]["not_attributable"], 1);
}

#[test]
fn a_content_match_takes_the_row_after_the_mark_not_the_first_row() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let mut early = read_row(1);
    early.alter_id = Some(90);
    let result = verified(&line, vec![early, read_row(2)], Attribution::Span(None));
    assert_eq!(status_of(&result, 0), "matching_content_observed");
    assert_eq!(result["vouchers"][0]["guid"], "row-guid-2");
    assert_eq!(result["vouchers"][0]["alter_id"], 102);
}

#[test]
fn a_group_whose_rows_were_all_taken_finds_nothing_more() {
    // Two vouchers of one content and one row after the mark: the first takes
    // it, and the second is absent, not unattributable.
    let line = batch_of(
        Some(100),
        vec![journal_voucher("t1"), journal_voucher("t2")],
    );
    let result = verified(&line, vec![read_row(1)], Attribution::Span(None));
    assert_eq!(status_of(&result, 0), "matching_content_observed");
    assert_eq!(result["vouchers"][0]["ambiguous_within_batch"], true);
    assert_eq!(
        result["vouchers"][1],
        json!({"bridge_txn_id":"t2", "status":"not_found", "ambiguous_within_batch":true})
    );
    assert_eq!(result["ambiguous_within_batch"], json!(["t1", "t2"]));
    assert_eq!(result["counts"]["not_found"], 1);
    assert_eq!(result["counts"]["not_attributable"], 0);
}

#[test]
fn a_tagged_match_is_never_called_ambiguous_by_its_content() {
    let line = batch_of(
        Some(100),
        vec![journal_voucher("t1"), journal_voucher("t2")],
    );
    let result = verified(
        &line,
        vec![tagged_row(1, "t1"), tagged_row(2, "t2")],
        Attribution::Tag,
    );
    assert_eq!(result["counts"]["posted_verified"], 2);
    assert_eq!(result["ambiguous_within_batch"], json!([]));
    for voucher in result["vouchers"].as_array().unwrap() {
        assert!(voucher.get("ambiguous_within_batch").is_none(), "{voucher}");
    }
}

#[test]
fn a_tag_taken_by_one_voucher_is_named_when_another_voucher_repeats_it() {
    // A saved batch holds each bridge_txn_id once; this record repeats one to
    // reach the guard that a taken tag is not matched twice.
    let line = batch_of(
        Some(100),
        vec![journal_voucher("t1"), journal_voucher("t1")],
    );
    let result = verified(&line, vec![tagged_row(1, "t1")], Attribution::Tag);
    assert_eq!(status_of(&result, 0), "posted_verified");
    assert_eq!(
        result["vouchers"][1],
        json!({"bridge_txn_id":"t1", "status":"not_attributable", "marker":"narration_tag",
            "reason":"observed_voucher_already_attributed"})
    );
    assert_eq!(result["counts"]["not_attributable"], 1);
}

#[test]
fn a_bound_voucher_the_window_does_not_hold_names_its_binding() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let bindings = [span_binding("t1", 9)];
    let result = verified(&line, vec![read_row(1)], Attribution::Span(Some(&bindings)));
    assert_eq!(
        result["vouchers"][0],
        json!({"bridge_txn_id":"t1", "status":"bound_not_in_window",
            "marker":"post_span_binding", "guid":"row-guid-9", "master_id":"9",
            "next_step":plain_next_step("bound_not_in_window")})
    );
    assert_eq!(result["counts"]["bound_not_in_window"], 1);
}

#[test]
fn a_bound_row_on_another_master_id_is_refused_with_its_reason() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let mut binding = span_binding("t1", 1);
    binding.master_id = 77;
    let bindings = [binding];
    let result = verified(&line, vec![read_row(1)], Attribution::Span(Some(&bindings)));
    assert_eq!(
        result["vouchers"][0],
        json!({"bridge_txn_id":"t1", "status":"not_attributable",
            "marker":"post_span_binding", "reason":"bound_master_id_changed"})
    );
    assert_eq!(result["counts"]["not_attributable"], 1);
}

#[test]
fn a_bound_row_is_found_by_its_guid_whichever_place_it_has_and_compared_by_its_entries() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let bindings = [span_binding("t1", 2)];
    let mut changed = read_row(2);
    changed.entries[0].amount = "-11.00".into();
    let result = verified(
        &line,
        vec![read_row(1), changed],
        Attribution::Span(Some(&bindings)),
    );
    assert_eq!(status_of(&result, 0), "posted_divergent");
    assert_eq!(result["vouchers"][0]["guid"], "row-guid-2");
    assert_eq!(result["vouchers"][0]["marker"], "post_span_binding");
    assert_eq!(
        result["vouchers"][0]["diffs"][0]["entries"]["observed"][0]["amount"],
        "-11"
    );
    assert_eq!(result["counts"]["posted_divergent"], 1);
}

#[test]
fn a_bound_row_takes_part_in_no_other_match_and_its_identity_is_not_a_duplicate() {
    let two = vec![journal_voucher("t1"), journal_voucher("t2")];
    let line = batch_of(Some(100), two);
    // Both bound and both verified: identical content under two identities is
    // what the batch holds, so no duplicate is reported.
    let both = [span_binding("t1", 1), span_binding("t2", 2)];
    let result = verified(
        &line,
        vec![read_row(1), read_row(2)],
        Attribution::Span(Some(&both)),
    );
    assert_eq!(result["counts"]["posted_verified"], 2);
    assert_eq!(result["duplicates"], json!([]));
    // One bound (to the second row) and one not: the unbound voucher takes the
    // other row, and the bound row is not offered to it as well.
    let one = [span_binding("t1", 2)];
    let result = verified(
        &line,
        vec![read_row(1), read_row(2)],
        Attribution::Span(Some(&one)),
    );
    assert_eq!(status_of(&result, 0), "posted_verified");
    assert_eq!(result["vouchers"][0]["guid"], "row-guid-2");
    assert_eq!(status_of(&result, 1), "matching_content_observed");
    assert_eq!(result["vouchers"][1]["guid"], "row-guid-1");
    assert_eq!(result["duplicates"][0]["kind"], "accounting_fingerprint");
    assert_eq!(
        result["duplicates"][0]["voucher_ids"],
        json!(["guid:row-guid-1", "guid:row-guid-2"])
    );
}

#[test]
fn a_bound_bank_voucher_says_when_its_effective_date_was_not_read() {
    let mut payment = journal_voucher("t1");
    payment["voucher_type"] = json!("Payment");
    let line = batch_of(Some(100), vec![payment]);
    let bindings = [span_binding("t1", 1)];
    let mut payment_row = read_row(1);
    payment_row.voucher_type = Some("Payment".into());
    let unread = verified(
        &line,
        vec![payment_row.clone()],
        Attribution::Span(Some(&bindings)),
    );
    assert_eq!(status_of(&unread, 0), "posted_verified");
    assert_eq!(
        unread["vouchers"][0]["not_observed"],
        json!(["effective_date"])
    );
    payment_row.effective_date = Some(day(FIRST_DAY));
    let read = verified(&line, vec![payment_row], Attribution::Span(Some(&bindings)));
    assert_eq!(status_of(&read, 0), "posted_verified");
    assert!(read["vouchers"][0].get("not_observed").is_none(), "{read}");
}

fn import_voucher(value: Value) -> ImportVoucher {
    serde_json::from_value(value).unwrap()
}

#[test]
fn a_book_rolled_back_flag_is_set_and_each_not_found_voucher_is_moved() {
    let mut result = json!({"counts":{"not_found":1, "bound_not_in_window":1},
        "vouchers":[{"status":"not_found"}, {"status":"bound_not_in_window"}, {"status":"posted_verified"}]});
    mark_book_rolled_back(&mut result);
    assert_eq!(result["book_rolled_back"], json!(true));
    assert_eq!(result["counts"]["book_rolled_back"], 2);
    assert_eq!(result["counts"]["not_found"], 0);
    assert_eq!(result["counts"]["bound_not_in_window"], 0);
    assert_eq!(status_of(&result, 2), "posted_verified");
}

#[test]
fn a_count_already_held_under_the_new_status_is_added_to() {
    let mut result = json!({"counts":{"not_found":1, "sent_not_attributed":2},
        "vouchers":[{"status":"not_found"}]});
    mark_sent_not_attributed(&mut result);
    assert_eq!(result["counts"]["sent_not_attributed"], 3);
}

#[test]
fn an_attributed_status_counts_only_a_counter_the_result_already_holds() {
    let expected = import_voucher(journal_voucher("t1"));
    let mut counts = BTreeMap::new();
    let value = attributed_status(
        &expected,
        &read_row(1),
        "narration_tag",
        true,
        WrittenNarration::Untagged,
        &mut counts,
    )
    .unwrap();
    assert_eq!(value["status"], "posted_verified");
    assert!(counts.is_empty(), "{counts:?}");
}

#[test]
fn a_divergent_voucher_shows_its_own_master_id_and_guid() {
    let expected = import_voucher(journal_voucher("t1"));
    let mut row = read_row(4);
    row.voucher_number = Some("N-4".into());
    let mut counts = BTreeMap::from([("posted_divergent", 0_u64)]);
    let value = attributed_status(
        &expected,
        &row,
        "narration_tag",
        false,
        WrittenNarration::Untagged,
        &mut counts,
    )
    .unwrap();
    assert_eq!(
        value,
        json!({"bridge_txn_id":"t1", "status":"posted_divergent", "marker":"narration_tag",
            "diffs":[{"entries":{
                "expected":[{"ledger":party_name("Ledger Alpha"), "amount":"10", "side":"Dr", "is_deemed_positive":"Yes"},
                    {"ledger":party_name("Ledger Beta"), "amount":"10", "side":"Cr", "is_deemed_positive":"No"}],
                "observed":[{"ledger":party_name("Ledger Alpha"), "amount":"-10.00", "is_deemed_positive":"Yes"},
                    {"ledger":party_name("Ledger Beta"), "amount":"10.00", "is_deemed_positive":"No"}]}}],
            "voucher_number":"N-4", "guid":"row-guid-4", "master_id":"4", "alter_id":104})
    );
    assert_eq!(counts["posted_divergent"], 1);
}

#[test]
fn a_voucher_that_differs_everywhere_lists_each_difference_in_order() {
    let mut payment = journal_voucher("t1");
    payment["voucher_type"] = json!("Payment");
    payment["voucher_number"] = json!("V-1");
    payment["narration"] = json!("written");
    let expected = import_voucher(payment);
    let mut row = read_row(1);
    row.date = day(SECOND_DAY);
    row.effective_date = Some(day(THIRD_DAY));
    row.voucher_number = Some("V-2".into());
    row.narration = Some("read".into());
    row.entries[0].amount = "-11.00".into();
    let diffs = voucher_diffs(&expected, &row, false, WrittenNarration::Untagged);
    let names = diffs
        .iter()
        .map(|diff| diff.as_str().unwrap_or("entries"))
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            "date",
            "effective_date",
            "voucher_type",
            "voucher_number",
            "narration",
            "entries"
        ]
    );
    assert_eq!(diffs[5]["entries"]["expected"][0]["side"], "Dr");
    assert_eq!(diffs[5]["entries"]["expected"][0]["amount"], "10");
    assert_eq!(diffs[5]["entries"]["observed"][0]["amount"], "-11.00");
    assert_eq!(
        diffs[5]["entries"]["observed"][0]["is_deemed_positive"],
        "Yes"
    );
}

#[test]
fn a_narration_is_compared_exactly_on_the_read_side() {
    let mut spaced = journal_voucher("t1");
    spaced["narration"] = json!("  note  ");
    let expected = import_voucher(spaced);
    let untagged = WrittenNarration::Untagged;
    let tagged = WrittenNarration::Tagged("t1");
    assert!(!narration_differs(&expected, untagged, Some("note")));
    assert!(!narration_differs(
        &expected,
        tagged,
        Some("note [BRIDGE:t1]")
    ));
    assert!(narration_differs(&expected, untagged, Some(" note")));
    assert!(narration_differs(&expected, untagged, Some("note ")));
    assert!(narration_differs(&expected, untagged, Some("NOTE")));
    assert!(narration_differs(&expected, untagged, None));
    let bare = import_voucher(journal_voucher("t1"));
    assert!(!narration_differs(&bare, untagged, None));
    assert!(!narration_differs(&bare, tagged, Some("[BRIDGE:t1]")));
    assert!(narration_differs(&bare, tagged, None));
}

#[test]
fn a_count_that_is_not_in_the_result_is_none() {
    assert_eq!(unmatched_count(&json!({"counts":{}})), 0);
    assert_eq!(unmatched_count(&json!({"counts":{"not_found":3}})), 3);
}

#[test]
fn a_verdict_needs_the_exact_count_and_a_duplicates_list_that_is_present_and_empty() {
    let clean = json!({"counts":{"posted_verified":2}, "duplicates":[]});
    assert_eq!(verification_status(&clean, 2), "posted_verified");
    assert_eq!(verification_status(&clean, 3), "verification_incomplete");
    assert_eq!(
        verification_status(&json!({"counts":{"posted_verified":2}}), 2),
        "verification_incomplete"
    );
    assert_eq!(
        verification_status(
            &json!({"counts":{"posted_verified":2}, "duplicates":[{}]}),
            2
        ),
        "verification_incomplete"
    );
}

#[test]
fn a_page_of_a_proof_without_vouchers_lists_none() {
    let page = verification_response_page(&json!({"batch_id":"b"}), "digest", 0);
    assert_eq!(page["verified_total"], 0);
    assert_eq!(page["items"], json!([]));
    assert_eq!(page["unverified_vouchers"], json!([]));
}

#[test]
fn a_headline_reads_the_status_of_each_row_and_the_batch_verdict() {
    let result = |row: Value, verdict: &str| {
        json!({"dispatch":{"state":"posted_verified"}, "vouchers":[row],
            "verification_status":verdict})
    };
    let verified = batch_headline(&result(
        json!({"status":"posted_verified"}),
        "posted_verified",
    ));
    assert!(verified.is_some());
    assert_eq!(
        batch_headline(&result(json!({}), "posted_verified")),
        batch_headline(&result(json!({"status":""}), "posted_verified"))
    );
    assert_ne!(
        batch_headline(&result(json!({}), "posted_verified")),
        verified
    );
    assert_ne!(
        verified,
        batch_headline(&result(
            json!({"status":"posted_verified"}),
            "verification_incomplete"
        ))
    );
}

#[test]
fn an_invoice_readback_changes_nothing_when_nothing_differs_and_never_counts_below_zero() {
    let row = json!({"bridge_txn_id":"t1", "status":"posted_verified"});
    let mut same = json!({"vouchers":[row.clone()], "counts":{}});
    mark_invoice_readback(&mut same, "t1", &[], &[]);
    assert_eq!(same, json!({"vouchers":[row.clone()], "counts":{}}));
    let differences = ["gst_field".to_string()];
    // A verified count of 0 under a moved row stays 0.
    let mut low = json!({"vouchers":[row.clone()],
        "counts":{"posted_verified":0, "posted_divergent":0}});
    mark_invoice_readback(&mut low, "t1", &differences, &[]);
    assert_eq!(
        low["counts"],
        json!({"posted_verified":0, "posted_divergent":1})
    );
    // Nothing moved and no counts held: each reads 0.
    let mut bare = json!({"vouchers":[], "counts":{}});
    mark_invoice_readback(&mut bare, "t1", &differences, &[]);
    assert_eq!(
        bare["counts"],
        json!({"posted_verified":0, "posted_divergent":0})
    );
    // The two counts are each their own.
    let mut counted = json!({"vouchers":[row],
        "counts":{"posted_verified":5, "posted_divergent":2}});
    mark_invoice_readback(&mut counted, "t1", &differences, &[]);
    assert_eq!(
        counted["counts"],
        json!({"posted_verified":4, "posted_divergent":3})
    );
}

#[test]
fn a_post_that_landed_in_part_needs_one_created_and_counts_every_exception() {
    for (created, exceptions) in [(1, 1), (1, 2), (2, 1)] {
        let mut answer = committed_49_of_50();
        answer.created = created;
        answer.exceptions = exceptions;
        assert_eq!(
            reported_not_created_count(&answer, (created + exceptions) as usize, |_| true),
            Some(exceptions),
            "{created} {exceptions}"
        );
    }
}

#[test]
fn a_rejected_voucher_is_not_labelled_when_more_were_unmatched_than_sent() {
    assert_eq!(
        unmatched_cause(Some(&rejected_one_silver()), 1, 2, None),
        UnmatchedCause::NotEstablished
    );
}

fn bare_row(guid: Option<&str>, master_id: Option<&str>) -> ReadVoucher {
    let mut row = read_row(1);
    row.guid = guid.map(str::to_string);
    row.master_id = master_id.map(str::to_string);
    row
}

#[test]
fn a_rows_identity_is_its_guid_lowered_else_its_master_id_and_blank_is_none() {
    let identity = |guid, master| observed_voucher_identity(&bare_row(guid, master));
    assert_eq!(identity(Some("ABC"), Some("5")), Ok("guid:abc".to_string()));
    assert_eq!(
        identity(Some("  "), Some("5")),
        Ok("master_id:5".to_string())
    );
    assert_eq!(identity(None, Some("5")), Ok("master_id:5".to_string()));
    for none in [
        identity(None, None),
        identity(None, Some("  ")),
        identity(Some(" "), None),
    ] {
        assert_eq!(
            none,
            Err("import_verification_identity_invalid".to_string())
        );
    }
}

#[test]
fn the_duplicates_of_a_window_name_the_remote_id_or_the_shared_fingerprint() {
    let mut rows = Vec::new();
    for (n, remote) in [
        (1, Some("r-1")),
        (2, Some("r-1")),
        (3, Some("r-2")),
        (4, Some(" ")),
        (5, Some(" ")),
    ] {
        let mut row = read_row(n);
        row.remote_id = remote.map(str::to_string);
        rows.push(row);
    }
    // A cancelled row whose entries the read dropped does not pair, and one
    // that came back with its entries does.
    let mut dropped = read_row(6);
    dropped.cancelled = Some(true);
    dropped.entries.clear();
    rows.push(dropped);
    let mut kept = read_row(7);
    kept.cancelled = Some(true);
    rows.push(kept);
    let identities = (1..=7)
        .map(|n| format!("guid:row-guid-{n}"))
        .collect::<Vec<_>>();
    let prints = ["fp", "fp", "fp", "other", "other", "fp", "fp"]
        .map(str::to_string)
        .to_vec();
    assert_eq!(
        duplicates(&rows, &identities, &prints),
        vec![
            json!({"kind":"remote_id", "remote_id":"r-1", "count":2}),
            json!({"kind":"accounting_fingerprint", "fingerprint":"fp",
                "voucher_ids":["guid:row-guid-1", "guid:row-guid-2", "guid:row-guid-3", "guid:row-guid-7"],
                "remote_ids":["r-1", "r-2"]}),
            json!({"kind":"accounting_fingerprint", "fingerprint":"other",
                "voucher_ids":["guid:row-guid-4", "guid:row-guid-5"],
                "remote_ids":[" "]}),
        ]
    );
}

#[test]
fn the_company_mark_needs_both_of_its_counters() {
    let mark = company_high_water_mark(&json!({"altvchid":7, "altmstid":3})).unwrap();
    assert_eq!(
        (mark.kind.as_str(), mark.value, mark.master_value),
        ("company_high_water", Some(7), Some(3))
    );
    for partial in [json!({"altvchid":7}), json!({"altmstid":3}), json!({})] {
        assert_eq!(
            company_high_water_mark(&partial).err(),
            Some("pre_import_mark_unobserved".to_string()),
            "{partial}"
        );
    }
}

#[test]
fn the_alter_id_delta_is_the_distance_from_the_mark_and_not_observed_below_it() {
    let rows = |ids: &[u64]| {
        ids.iter()
            .map(|id| {
                let mut row = read_row(1);
                row.alter_id = Some(*id);
                row
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        alter_id_delta(DeltaBasis::PrePost(3), &rows(&[5, 9, 4])),
        json!({"before":3, "after_seen":9, "delta":6, "from":"pre_post_mark"})
    );
    assert_eq!(
        alter_id_delta(DeltaBasis::PrePost(5), &rows(&[5])),
        json!({"before":5, "after_seen":5, "delta":0, "from":"pre_post_mark"})
    );
    assert_eq!(
        alter_id_delta(DeltaBasis::PrePost(5), &rows(&[4])),
        json!({"before":5, "after_seen":4, "delta":"not_observed", "from":"pre_post_mark"})
    );
    assert_eq!(
        alter_id_delta(DeltaBasis::PrePost(5), &[]),
        json!({"before":5, "after_seen":null, "delta":"not_observed", "from":"pre_post_mark"})
    );
    let mark = PreImportMark {
        kind: "company_high_water".into(),
        value: Some(2),
        master_value: None,
    };
    assert_eq!(
        alter_id_delta(DeltaBasis::Build(&mark), &rows(&[8])),
        json!({"before":2, "after_seen":8, "delta":6, "from":"build_mark"})
    );
    let unmarked = PreImportMark {
        value: None,
        ..mark
    };
    assert_eq!(
        alter_id_delta(DeltaBasis::Build(&unmarked), &rows(&[8])),
        json!({"before":null, "after_seen":8, "delta":"not_observed", "from":"build_mark"})
    );
}

#[test]
fn a_code_span_is_fenced_longer_than_any_backtick_run_and_padded_where_a_span_would_drop_a_space() {
    for (text, span) in [
        ("plain", "`plain`"),
        ("", "`  `"),
        ("a`b", "`` a`b ``"),
        ("a``b`c", "``` a``b`c ```"),
        (" x ", "`  x  `"),
        (" x", "` x`"),
        ("x ", "`x `"),
        ("   ", "`   `"),
        ("a\nb", "`a b`"),
        ("a|b", "`a|b`"),
    ] {
        assert_eq!(markdown_code(text), span, "{text:?}");
    }
    assert_eq!(markdown_table_code("a|b"), "`a\\|b`");
}

fn verification_xml(identity: &str, polarity: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><VOUCHER>{identity}<ISCANCELLED>No</ISCANCELLED><ISOPTIONAL>No</ISOPTIONAL><ALLLEDGERENTRIES.LIST><LEDGERNAME>Ledger Alpha</LEDGERNAME><ISDEEMEDPOSITIVE>{polarity}</ISDEEMEDPOSITIVE><AMOUNT>-10.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Ledger Beta</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>10.00</AMOUNT></ALLLEDGERENTRIES.LIST></VOUCHER></COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

const WHOLE_ROW: &str = concat!(
    "<GUID>company-guid-synthetic-0001</GUID><DATE>2026",
    "0901</DATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>"
);

fn rows_of(xml: &str) -> Vec<ReadVoucher> {
    parse_import_voucher_rows(xml, BATCH_COMPANY).unwrap()
}

#[test]
fn a_padded_polarity_is_read_as_its_canonical_value() {
    // The shared parser emits `Yes` and `No`, which is why the rows are not
    // trimmed again here.
    let rows = rows_of(&verification_xml(WHOLE_ROW, " Yes "));
    assert_eq!(rows[0].entries[0].is_deemed_positive, "Yes");
    assert_eq!(rows[0].entries[1].is_deemed_positive, "No");
    let second = rows_of(&verification_xml(WHOLE_ROW, " No "));
    assert_eq!(second[0].entries[1].is_deemed_positive, "No");
}

#[test]
fn a_row_the_shared_parser_refuses_is_refused_under_the_verification_code_of_its_kind() {
    let no_date =
        "<GUID>company-guid-synthetic-0001</GUID><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>";
    assert_eq!(
        parse_import_voucher_rows(&verification_xml(no_date, "Yes"), BATCH_COMPANY),
        Err("import_verification_export_invalid".to_string())
    );
    let bad_reference = format!("{WHOLE_ROW}<REFERENCEDATE>15-Apr-2026</REFERENCEDATE>");
    assert_eq!(
        parse_import_voucher_rows(&verification_xml(&bad_reference, "Yes"), BATCH_COMPANY),
        Err("import_verification_export_invalid".to_string())
    );
    let no_identity = concat!(
        "<DATE>2026",
        "0901</DATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>"
    );
    assert_eq!(
        parse_import_voucher_rows(&verification_xml(no_identity, "Yes"), BATCH_COMPANY),
        Err("import_verification_identity_invalid".to_string())
    );
}

#[test]
fn an_amount_is_canonical_only_as_exactly_written() {
    assert_eq!(
        canonical_verification_amount("-12.50"),
        Ok("-12.5".to_string())
    );
    assert_eq!(
        canonical_verification_amount("0012.00"),
        Ok("12".to_string())
    );
    for bad in [" 1.50", "1.50 ", "1e2", "abc", "", "1.5.0"] {
        assert_eq!(
            canonical_verification_amount(bad),
            Err("import_verification_amount_invalid".to_string()),
            "{bad:?}"
        );
    }
}

fn whole_proof() -> Value {
    json!({"batch_id":"batch-1", "verification_status":"posted_verified",
        "dispatch":{"state":"posted_verified", "response_state":"response_clean"},
        "company":{"name":"Lab `Co`"}, "batch_sha256":"aa11", "verified_at":"2026-09-01T10:00:00Z",
        "counts":{"posted_verified":2, "posted_divergent":3, "posted_not_effective":4,
            "not_found":5, "cancelled_with_effective_copy":6},
        "alter_id_delta":{"delta":3},
        "duplicates":[{"kind":"remote_id", "remote_id":"r|1", "count":2},
            {"kind":"accounting_fingerprint", "fingerprint_sha256":"ff", "voucher_ids":["a", "b", "c"]}],
        "unrelated_duplicates_in_window":[{}],
        "vouchers":[{"bridge_txn_id":"t1", "status":"posted_verified"},
            {"bridge_txn_id":"t2", "status":"not_found"}],
        "evidence":{"company":{"response_sha256":"cc22"}, "voucher_read_sha256":"dd33"}})
}

fn proof_whole() -> String {
    [
        "# Voucher import verification — batch-1",
        "",
        "- Verification status: `posted_verified`",
        "- Dispatch verdict: `posted_verified`",
        "- Response state: `response_clean`",
        "",
        "- Company: `` Lab `Co` ``",
        "- Batch SHA-256: `aa11`",
        "- Readback checked: `2026-09-01T10:00:00Z`",
        "- Readback counts: matching 2, divergent 3, not effective 4, not found 5",
        "- AlterID delta: `{\"delta\":3}`",
        "- Duplicates in this batch: 2",
        "- Unrelated duplicates in window: 1",
        "- Cancelled vouchers with an effective copy (report only, not attributed): 6",
        "",
        "| Transaction | Readback status |",
        "| --- | --- |",
        "| t1 | posted_verified |",
        "| t2 | not_found |",
        "",
        "| Duplicate in this batch | Key | Vouchers |",
        "| --- | --- | --- |",
        "| remote_id | `r\\|1` | 2 |",
        "| accounting_fingerprint | `ff` | 3 |",
        "",
        "Evidence hashes: company `cc22`, voucher read `dd33`.",
        "",
    ]
    .join("\n")
}

fn proof_empty() -> String {
    [
        "# Voucher import verification — unknown",
        "",
        "**Not verified — this report does not confirm posting.**",
        "",
        "The verification status, the counts and any duplicates below say what the readback found.",
        "",
        "- Verification status: `unknown`",
        "",
        "- Company: `unknown`",
        "- Batch SHA-256: `unknown`",
        "- Readback checked: `unknown`",
        "- Readback counts: matching null, divergent null, not effective null, not found null",
        "- AlterID delta: `null`",
        "- Duplicates in this batch: 0",
        "- Unrelated duplicates in window: 0",
        "- Cancelled vouchers with an effective copy (report only, not attributed): null",
        "",
        "| Transaction | Readback status |",
        "| --- | --- |",
        "",
        "Evidence hashes: company `unknown`, voucher read `unknown`.",
        "",
    ]
    .join("\n")
}

fn proof_reconciliation() -> String {
    [
        "# Voucher import verification — batch-1",
        "",
        "**Reconciliation required — this report does not confirm posting.**",
        "",
        "A matching voucher readback alone is insufficient. Reconcile the original saved batch; do not rebuild or resend it.",
        "",
        "- Verification status: `posted_verified`",
        "- Dispatch verdict: `reconciliation_required`",
        "- Response state: `response_clean`",
        "- Dispatch record: none on this computer — found in the book, not sent by ComplyEaze Bridge",
        "- Error: `some_code`",
        "",
        "- Company: `` Lab `Co` ``",
        "- Batch SHA-256: `aa11`",
        "- Readback checked: `2026-09-01T10:00:00Z`",
        "- Readback counts: matching 2, divergent 3, not effective 4, not found 5",
        "- AlterID delta: `{\"delta\":3}`",
        "- Duplicates in this batch: 2",
        "- Unrelated duplicates in window: 1",
        "- Cancelled vouchers with an effective copy (report only, not attributed): 6",
        "",
        "| Transaction | Readback status |",
        "| --- | --- |",
        "| t1 | posted_verified |",
        "| t2 | not_found |",
        "",
        "| Duplicate in this batch | Key | Vouchers |",
        "| --- | --- | --- |",
        "| remote_id | `r\\|1` | 2 |",
        "| accounting_fingerprint | `ff` | 3 |",
        "",
        "Evidence hashes: company `cc22`, voucher read `dd33`.",
        "",
    ]
    .join("\n")
}

fn proof_book() -> String {
    [
        "# Voucher import verification — batch-1",
        "",
        "**Not verified — this report does not confirm posting.**",
        "",
        "The verification status, the counts and any duplicates below say what the readback found.",
        "",
        "- Verification status: `verification_incomplete`",
        "",
        "- Company: `` Lab `Co` ``",
        "- Batch SHA-256: `aa11`",
        "- Readback checked: `2026-09-01T10:00:00Z`",
        "- Readback counts: matching 2, divergent 3, not effective 4, not found 5",
        "- AlterID delta: `{\"delta\":3}`",
        "- Duplicates in this batch: 2",
        "- Unrelated duplicates in window: 1",
        "- Cancelled vouchers with an effective copy (report only, not attributed): 6",
        "",
        "| Transaction | Readback status |",
        "| --- | --- |",
        "| t1 | posted_verified |",
        "| t2 | not_found |",
        "",
        "| Duplicate in this batch | Key | Vouchers |",
        "| --- | --- | --- |",
        "| remote_id | `r\\|1` | 2 |",
        "| accounting_fingerprint | `ff` | 3 |",
        "",
        "Evidence hashes: company `cc22`, voucher read `dd33`.",
        "",
    ]
    .join("\n")
}

fn proof_gaps() -> String {
    [
        "# Voucher import verification — unknown",
        "",
        "- Verification status: `posted_verified`",
        "- Dispatch verdict: `posted_verified`",
        "- Response state: `unknown`",
        "",
        "- Company: `unknown`",
        "- Batch SHA-256: `unknown`",
        "- Readback checked: `unknown`",
        "- Readback counts: matching null, divergent null, not effective null, not found null",
        "- AlterID delta: `null`",
        "- Duplicates in this batch: 3",
        "- Unrelated duplicates in window: 0",
        "- Cancelled vouchers with an effective copy (report only, not attributed): null",
        "",
        "| Transaction | Readback status |",
        "| --- | --- |",
        "| unknown | unknown |",
        "",
        "| Duplicate in this batch | Key | Vouchers |",
        "| --- | --- | --- |",
        "| remote_id | `unknown` | 0 |",
        "| accounting_fingerprint | `unknown` | 0 |",
        "| unknown | `unknown` | 0 |",
        "",
        "Evidence hashes: company `unknown`, voucher read `unknown`.",
        "",
    ]
    .join("\n")
}

#[test]
fn a_proof_is_rendered_line_by_line() {
    let mut reconciliation = whole_proof();
    reconciliation["dispatch"]["state"] = json!("reconciliation_required");
    reconciliation["error"] = json!({"code":"some_code"});
    reconciliation["dispatch_record"] = json!({"state":"none_on_record"});
    let mut book = whole_proof();
    book["verification_status"] = json!("verification_incomplete");
    book.as_object_mut().unwrap().remove("dispatch");
    let gaps = json!({"verification_status":"posted_verified",
        "dispatch":{"state":"posted_verified"}, "vouchers":[{}],
        "duplicates":[{"kind":"remote_id"}, {"kind":"accounting_fingerprint"}, {}]});
    assert_eq!(render_proof_markdown(&whole_proof()), proof_whole());
    assert_eq!(render_proof_markdown(&json!({})), proof_empty());
    assert_eq!(
        render_proof_markdown(&reconciliation),
        proof_reconciliation()
    );
    assert_eq!(render_proof_markdown(&book), proof_book());
    assert_eq!(render_proof_markdown(&gaps), proof_gaps());
}

#[test]
fn a_content_group_with_a_row_before_the_mark_is_named_by_the_mark_not_by_a_taken_row() {
    let line = batch_of(
        Some(100),
        vec![journal_voucher("t1"), journal_voucher("t2")],
    );
    let mut early = read_row(2);
    early.alter_id = Some(90);
    let result = verified(&line, vec![read_row(1), early], Attribution::Span(None));
    assert_eq!(status_of(&result, 0), "matching_content_observed");
    assert_eq!(result["vouchers"][0]["guid"], "row-guid-1");
    assert_eq!(
        result["vouchers"][1],
        json!({"bridge_txn_id":"t2", "status":"not_attributable",
            "marker":"accounting_fingerprint",
            "reason":"fingerprint_precedes_pre_import_voucher_mark",
            "ambiguous_within_batch":true})
    );
}

#[test]
fn a_cancelled_vouchers_copies_are_listed_oldest_first_with_their_place_to_the_mark() {
    let line = batch_of(Some(100), vec![journal_voucher("t1")]);
    let mut cancelled = tagged_row(1, "t1");
    cancelled.cancelled = Some(true);
    let mut late = read_row(4);
    late.alter_id = Some(150);
    let mut at_mark = read_row(3);
    at_mark.alter_id = Some(100);
    let result = verified(&line, vec![cancelled, late, at_mark], Attribution::Tag);
    assert_eq!(result["vouchers"][0]["reason"], "voucher_cancelled");
    assert_eq!(result["counts"]["cancelled_with_effective_copy"], 1);
    assert_eq!(
        result["vouchers"][0]["effective_copies_observed"],
        json!({"count":2, "attribution":"not_established", "entries":[
            {"guid":"row-guid-3", "master_id":"3", "alter_id":100, "voucher_number":null,
                "before_pre_import_mark":true},
            {"guid":"row-guid-4", "master_id":"4", "alter_id":150, "voucher_number":null,
                "before_pre_import_mark":false}]})
    );
}

#[test]
fn a_window_never_holds_one_guid_twice() {
    // So the row a binding finds by its GUID is the only one.
    let mut again = read_row(2);
    again.guid = Some(" ROW-GUID-1 ".into());
    assert_eq!(
        ImportReadSource::admit(vec![read_row(1), again]).err(),
        Some("import_verification_identity_invalid".to_string())
    );
}

#[test]
fn a_status_with_no_absence_tells_the_person_what_happened_and_what_to_do() {
    // The texts a person reads. A text is held by how it opens and by the
    // facts it names, as the other next-step tests of this module do (an
    // opening with starts_with, a fact with contains), never by a whole copy.
    let moved = plain_next_step("bound_not_in_window").unwrap();
    assert!(
        moved.starts_with("This voucher was posted, but it is not in the book for these dates"),
        "{moved}"
    );
    assert!(moved.contains("re-dated in Tally"), "{moved}");
    assert!(
        moved.ends_with("Check in Tally before posting it again."),
        "{moved}"
    );

    let older = plain_next_step("book_rolled_back").unwrap();
    assert!(
        older.starts_with("This voucher was posted, but the company's books are now older"),
        "{older}"
    );
    assert!(older.contains("restored from a backup"), "{older}");
    assert!(
        older.ends_with("Check in Tally before posting it again."),
        "{older}"
    );

    let unmatched = plain_next_step("sent_not_attributed").unwrap();
    assert!(
        unmatched
            .starts_with("This voucher was sent to Tally, but ComplyEaze Bridge cannot match it"),
        "{unmatched}"
    );
    assert!(unmatched.contains("edited in Tally"), "{unmatched}");
    assert!(
        unmatched.ends_with("Check in Tally before posting it again."),
        "{unmatched}"
    );

    let not_created = plain_next_step("tally_reported_not_created").unwrap();
    assert!(
        not_created.starts_with("Tally reported this voucher as not created."),
        "{not_created}"
    );
    assert!(
        not_created.contains("acknowledge_post_review with doubt not_created"),
        "{not_created}"
    );
    assert_eq!(plain_next_step("posted_verified"), None);
}

#[test]
fn the_next_step_of_a_divergent_invoice_and_of_a_sent_voucher_reported_not_created_name_what_to_do()
{
    assert!(
        INVOICE_DIVERGENT_NEXT_STEP
            .starts_with("This invoice is in Tally, but it is not confirmed as built."),
        "{INVOICE_DIVERGENT_NEXT_STEP}"
    );
    assert!(INVOICE_DIVERGENT_NEXT_STEP.contains("`diffs` says why"));
    assert!(INVOICE_DIVERGENT_NEXT_STEP.ends_with("Do not post it again."));

    assert!(
        REPORTED_NOT_CREATED_WHEN_SENT_NEXT_STEP.starts_with(
            "When ComplyEaze Bridge sent this voucher, Tally reported it as not created"
        ),
        "{REPORTED_NOT_CREATED_WHEN_SENT_NEXT_STEP}"
    );
    assert!(REPORTED_NOT_CREATED_WHEN_SENT_NEXT_STEP
        .contains("acknowledge_post_review with doubt not_created"));
}

#[test]
fn a_row_the_shared_parser_admits_is_always_read_whole() {
    // The premise of the record's argument that the closure of
    // `parse_import_voucher_rows` which reports `import_verification_export_invalid`
    // for a parsed row that does not deserialise cannot run. The rows come from the shared parser's one
    // `json!` of a voucher, whose every field is a string, a bool, an absent
    // key or a u64 that its own scalar parsers produced, and each date was
    // admitted by the same `TallyDate::parse` that the deserialiser runs. This
    // holds the extremes: every optional element at its widest, and at its
    // narrowest, and the first value past the u64 range refused by the parser
    // before any row exists.
    let max = u64::MAX.to_string();
    let widest = format!(
        "<GUID>{BATCH_COMPANY}-{max}</GUID><DATE>{d}</DATE><EFFECTIVEDATE>{e}</EFFECTIVEDATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME><VOUCHERNUMBER> 7 </VOUCHERNUMBER><NARRATION>Ünï | `text`</NARRATION><PARTYLEDGERNAME>Party</PARTYLEDGERNAME><ALTERID>{max}</ALTERID><MASTERID> {max} </MASTERID><REMOTEID>remote-1</REMOTEID><ISPOSTDATED>Yes</ISPOSTDATED>",
        d = FIRST_DAY,
        e = SECOND_DAY,
    );
    let rows = rows_of(&verification_xml(&widest, "Yes"));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].alter_id, Some(u64::MAX));
    assert_eq!(rows[0].master_id, Some(max.clone()));
    assert_eq!(rows[0].effective_date, Some(day(SECOND_DAY)));
    assert_eq!(rows[0].remote_id.as_deref(), Some("remote-1"));

    let narrowest = format!(
        "<GUID>{BATCH_COMPANY}-1</GUID><DATE>{}</DATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>",
        FIRST_DAY
    );
    let rows = rows_of(&verification_xml(&narrowest, "No"));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].alter_id, None);
    assert_eq!(rows[0].master_id, None);
    assert_eq!(rows[0].effective_date, None);
    assert_eq!(rows[0].narration, None);

    let past = (u128::from(u64::MAX) + 1).to_string();
    let too_far = format!("{narrowest}<ALTERID>{past}</ALTERID>");
    assert_eq!(
        parse_import_voucher_rows(&verification_xml(&too_far, "Yes"), BATCH_COMPANY),
        Err("voucher_alter_id_invalid".to_string())
    );
}
