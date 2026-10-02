//! bridge#1108: what a native post's own answer from Tally says about the
//! vouchers its readback cannot find. Every answer below is a captured live
//! response, except the two marked synthetic, which exist only to hold the
//! guards that no capture reaches.
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

/// A rejected single voucher, Education mode (CREATED 0, EXCEPTIONS 1).
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
        assert_eq!(
            unmatched_cause(Some(&answer), 1, 1),
            UnmatchedCause::ReportedNotCreated,
            "{answer:?}"
        );
    }
}

#[test]
fn the_shortfall_of_a_partial_commit_is_reported_not_created() {
    assert_eq!(
        unmatched_cause(Some(&committed_49_of_50()), 50, 1),
        UnmatchedCause::ReportedNotCreated
    );
}

#[test]
fn more_unmatched_than_the_shortfall_establishes_nothing() {
    // One voucher Tally did not create, and one it did that the readback
    // cannot find: neither can be told apart, so nothing is claimed.
    assert_eq!(
        unmatched_cause(Some(&committed_49_of_50()), 50, 2),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn a_clean_answer_never_reports_a_voucher_not_created() {
    // Tally created the one sent: an unmatched voucher was edited, not refused.
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
fn an_answer_that_altered_a_voucher_establishes_nothing() {
    // Synthetic: no capture has ALTERED with EXCEPTIONS on a voucher post. An
    // altered voucher landed somewhere, so the shortfall is not "not created".
    let altered = TallyImportResult {
        altered: 1,
        ..rejected_one_silver()
    };
    assert_eq!(
        unmatched_cause(Some(&altered), 2, 2),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn an_answer_claiming_more_than_was_sent_establishes_nothing() {
    // Synthetic: a CREATED above the number sent cannot be read as a shortfall,
    // and must not underflow.
    let excess = TallyImportResult {
        created: 3,
        ..rejected_one_silver()
    };
    assert_eq!(
        unmatched_cause(Some(&excess), 2, 1),
        UnmatchedCause::NotEstablished
    );
}

#[test]
fn a_shortfall_without_an_exception_establishes_nothing() {
    // Synthetic: a shortfall Tally answered with no exception (and no error).
    // Only the measured shape, a rejection reported as an exception, is read
    // as "not created"; a silent shortfall is not.
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
