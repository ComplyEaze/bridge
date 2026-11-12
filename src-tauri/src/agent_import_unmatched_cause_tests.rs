//! bridge#1108: what a native post's own answer from Tally says about the
//! vouchers its readback cannot find. The Silver answers (one voucher, and
//! two batches Tally rejected whole) and the batch of 50 are live responses
//! committed byte for byte; the Education answers are
//! derived from live captures (counter shape only, see
//! EDUCATION_IMPORT_COUNTERS_PROVENANCE.md). Tests marked synthetic exist only
//! to hold the guards that no capture reaches.
//! The 7.2 answer of a Journal Tally refused for a Stock-in-Hand line is committed byte for
//! byte too.
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
