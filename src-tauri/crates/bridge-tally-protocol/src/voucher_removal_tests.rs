use super::*;
use crate::{
    decode_tally_xml_response_bytes_limited, parse_import_result, ExpectedTallyTextEncoding,
};

/// A native post's live answer (`CREATED` 10, every other counter reported and
/// 0; tests/fixtures/agent), which Bridge's own import parse reads.
const POSTED: &[u8] =
    include_bytes!("../tests/fixtures/agent/post-span-import-response.utf16le.xml");

/// The captured answer with its counters set to those §9.14 observed for a
/// cancel (`ALTERED` 1, the rest 0), then each `(from, to)` replaced, once each.
fn counters_with(edits: &[(&str, &str)]) -> anyhow::Result<TallyImportResult> {
    let mut xml = decode_tally_xml_response_bytes_limited(
        POSTED,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        POSTED.len(),
    )
    .unwrap()
    .text;
    let cancelled = [
        ("<CREATED>10</CREATED>", "<CREATED>0</CREATED>"),
        ("<ALTERED>0</ALTERED>", "<ALTERED>1</ALTERED>"),
    ];
    for (from, to) in cancelled.iter().chain(edits) {
        assert_eq!(xml.matches(from).count(), 1, "{from}");
        xml = xml.replace(from, to);
    }
    parse_import_result(&xml)
}

fn cancel(edits: &[(&str, &str)]) -> CancelOutcome {
    CancelOutcome::from_counters(&counters_with(edits).unwrap())
}

fn delete(edits: &[(&str, &str)]) -> DeleteOutcome {
    DeleteOutcome::from_counters(&counters_with(edits).unwrap())
}

const DELETED_ONE: [(&str, &str); 2] = [
    ("<ALTERED>1</ALTERED>", "<ALTERED>0</ALTERED>"),
    ("<DELETED>0</DELETED>", "<DELETED>1</DELETED>"),
];

#[test]
fn one_altered_is_a_cancel_and_one_deleted_is_a_delete_and_neither_is_the_other() {
    assert!(matches!(cancel(&[]), CancelOutcome::Altered(_)));
    assert!(matches!(delete(&DELETED_ONE), DeleteOutcome::Deleted(_)));
    assert_eq!(
        delete(&[]),
        DeleteOutcome::Refused(RemovalRefusal::OtherNotZero(ImportCounter::Altered, 1))
    );
    assert_eq!(
        cancel(&DELETED_ONE),
        CancelOutcome::Refused(RemovalRefusal::IntendedNotOne(ImportCounter::Altered, 0))
    );
}

#[test]
fn the_intended_counter_must_be_exactly_one() {
    for value in ["0", "2"] {
        let altered = format!("<ALTERED>{value}</ALTERED>");
        assert_eq!(
            cancel(&[("<ALTERED>1</ALTERED>", &altered)]),
            CancelOutcome::Refused(RemovalRefusal::IntendedNotOne(
                ImportCounter::Altered,
                value.parse().unwrap()
            ))
        );
        let deleted = format!("<DELETED>{value}</DELETED>");
        assert_eq!(
            delete(&[DELETED_ONE[0], ("<DELETED>0</DELETED>", &deleted)]),
            DeleteOutcome::Refused(RemovalRefusal::IntendedNotOne(
                ImportCounter::Deleted,
                value.parse().unwrap()
            ))
        );
    }
}

/// Each other counter not 0 refuses, `CANCELLED` included (a rule keyed on it
/// would have called the observed cancel a failure, §9.14), and `EXCEPTIONS`
/// refuses with no line error beside it.
#[test]
fn any_other_counter_not_zero_refuses() {
    for (element, counter) in [
        ("CREATED", ImportCounter::Created),
        ("IGNORED", ImportCounter::Ignored),
        ("ERRORS", ImportCounter::Errors),
        ("CANCELLED", ImportCounter::Cancelled),
        ("EXCEPTIONS", ImportCounter::Exceptions),
    ] {
        let zero = format!("<{element}>0</{element}>");
        let one = format!("<{element}>1</{element}>");
        let refused = RemovalRefusal::OtherNotZero(counter, 1);
        let counted = counters_with(&[(&zero, &one)]).unwrap();
        assert_eq!(counted.line_error_count, 0);
        assert_eq!(
            CancelOutcome::from_counters(&counted),
            CancelOutcome::Refused(refused)
        );
        assert_eq!(
            delete(&[DELETED_ONE[0], DELETED_ONE[1], (&zero, &one)]),
            DeleteOutcome::Refused(refused)
        );
    }
    assert_eq!(
        cancel(&[("<DELETED>0</DELETED>", "<DELETED>1</DELETED>")]),
        CancelOutcome::Refused(RemovalRefusal::OtherNotZero(ImportCounter::Deleted, 1))
    );
}

/// The parse refuses an answer without `CREATED`, `ALTERED`, `IGNORED` or
/// `ERRORS`; it reads an absent `DELETED`, `CANCELLED` or `EXCEPTIONS` as 0
/// and marks it absent, which refuses here.
#[test]
fn an_absent_counter_refuses() {
    for (element, counter) in [
        ("DELETED", ImportCounter::Deleted),
        ("CANCELLED", ImportCounter::Cancelled),
        ("EXCEPTIONS", ImportCounter::Exceptions),
    ] {
        let present = format!("<{element}>0</{element}>");
        assert_eq!(
            cancel(&[(&present, "")]),
            CancelOutcome::Refused(RemovalRefusal::CounterAbsent(counter))
        );
    }
    for element in ["CREATED", "ALTERED", "IGNORED", "ERRORS"] {
        let value = if element == "ALTERED" { "1" } else { "0" };
        let present = format!("<{element}>{value}</{element}>");
        assert!(counters_with(&[(&present, "")]).is_err(), "{element}");
    }
    // A saved record from before presence was kept reads every counter absent.
    type Unmark = fn(&mut crate::TallyImportCounterPresence);
    let unmarks: [(ImportCounter, Unmark); 7] = [
        (ImportCounter::Created, |p| p.created = false),
        (ImportCounter::Altered, |p| p.altered = false),
        (ImportCounter::Deleted, |p| p.deleted = false),
        (ImportCounter::Ignored, |p| p.ignored = false),
        (ImportCounter::Errors, |p| p.errors = false),
        (ImportCounter::Cancelled, |p| p.cancelled = false),
        (ImportCounter::Exceptions, |p| p.exceptions = false),
    ];
    for (counter, unmark) in unmarks {
        let mut unobserved = counters_with(&[]).unwrap();
        unmark(&mut unobserved.counter_presence);
        assert_eq!(
            CancelOutcome::from_counters(&unobserved),
            CancelOutcome::Refused(RemovalRefusal::CounterAbsent(counter))
        );
    }
}

#[test]
fn a_line_error_refuses_even_with_clean_counters() {
    let with_line_error = (
        "<EXCEPTIONS>0</EXCEPTIONS>",
        "<EXCEPTIONS>0</EXCEPTIONS><LINEERROR>Synthetic line error</LINEERROR>",
    );
    assert_eq!(
        cancel(&[with_line_error]),
        CancelOutcome::Refused(RemovalRefusal::LineErrors(1))
    );
    assert_eq!(
        delete(&[DELETED_ONE[0], DELETED_ONE[1], with_line_error]),
        DeleteOutcome::Refused(RemovalRefusal::LineErrors(1))
    );
}
