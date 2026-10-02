//! bridge#1108: what a native post's own answer from Tally says about the
//! vouchers its readback cannot find. The Silver answer and the batch of 50
//! are live responses committed byte for byte; the Education answers are
//! derived from live captures (counter shape only, see
//! EDUCATION_IMPORT_COUNTERS_PROVENANCE.md). Tests marked synthetic exist only
//! to hold the guards that no capture reaches.
use super::{unmatched_cause, UnmatchedCause};
use bridge_tally_protocol::{parse_import_outcome, TallyImportResult};

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
    let partial = committed_49_of_50();
    assert_eq!(
        (partial.created, partial.altered, partial.exceptions),
        (49, 0, 1)
    );
    let clean = created_one();
    assert_eq!((clean.created, clean.altered, clean.exceptions), (1, 0, 0));
}

#[test]
fn a_rejected_single_voucher_is_reported_not_created() {
    for answer in [rejected_one_education(), rejected_one_silver()] {
        assert!(answer.counter_presence.all_reported(), "{answer:?}");
        assert_eq!(
            unmatched_cause(Some(&answer), 1, 1),
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
            unmatched_cause(Some(&committed_49_of_50()), 50, unmatched),
            UnmatchedCause::NotEstablished
        );
    }
}

#[test]
fn a_rejected_voucher_found_after_all_is_not_labelled() {
    // Entered by hand with the same content since, it is found: nothing is
    // claimed about it.
    assert_eq!(
        unmatched_cause(Some(&rejected_one_silver()), 1, 0),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn a_clean_answer_never_reports_a_voucher_not_created() {
    assert_eq!(
        unmatched_cause(Some(&created_one()), 1, 1),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn no_recorded_answer_establishes_nothing() {
    assert_eq!(unmatched_cause(None, 1, 1), UnmatchedCause::NotEstablished);
}

#[test]
fn an_answer_with_a_counter_missing_establishes_nothing() {
    // Synthetic: an omitted counter is not an observed zero (§9.2).
    let mut missing = rejected_one_silver();
    missing.counter_presence.exceptions = false;
    assert_eq!(
        unmatched_cause(Some(&missing), 1, 1),
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
            unmatched_cause(Some(&answer), 1, 1),
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
        unmatched_cause(Some(&rejected_one_silver()), 2, 2),
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
        unmatched_cause(Some(&silent), 1, 1),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn a_batch_is_never_read_as_not_created() {
    // Synthetic: no batch Tally rejected whole has been captured. Two sent,
    // none created, an exception for each, or one exception and one voucher
    // found since: neither is labelled.
    let both_rejected = TallyImportResult {
        exceptions: 2,
        ..rejected_one_silver()
    };
    assert_eq!(
        unmatched_cause(Some(&both_rejected), 2, 2),
        UnmatchedCause::NotEstablished
    );
    assert_eq!(
        unmatched_cause(Some(&rejected_one_silver()), 2, 1),
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
        unmatched_cause(Some(&doubled), 1, 1),
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
        unmatched_cause(Some(&empty), 0, 0),
        UnmatchedCause::NotEstablished
    );
}
