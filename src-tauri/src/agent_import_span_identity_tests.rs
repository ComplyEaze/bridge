//! Binding by post span, from one capture: 10 untagged Payment, Receipt and
//! Contra vouchers posted to a synthetic company on licensed TallyPrime 7.1
//! Silver (`fixtures/POST_SPAN_CAPTURE_PROVENANCE.md`). Every negative case is
//! the captured value with one field changed, named where it is changed.
use super::*;
use crate::agent::change_parse::parse_all_company_marks;

const COMPANY: &str = "BRIDGE AMEND LAB";
const COMPANY_GUID: &str = "17a10910-773c-42c6-bd66-7bba9a392536";
const IMPORT: &str =
    include_str!("../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-import.xml");
const RESPONSE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-import-response.utf16le.xml"
);
const MARKS_BEFORE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-company-high-water-before.utf16le.xml"
);
const MARKS_AFTER: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-company-high-water-after.utf16le.xml"
);
const SPAN_READ: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-alterid-span-read.utf16le.xml"
);

fn captured(bytes: &[u8]) -> String {
    bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("captured BOM-less UTF-16LE response")
    .text
}

fn target_mark(bytes: &[u8]) -> u64 {
    let rows = parse_all_company_marks(&captured(bytes)).unwrap();
    let mut target = rows
        .iter()
        .filter(|row| row.guid.eq_ignore_ascii_case(COMPANY_GUID));
    let row = target.next().unwrap();
    assert!(target.next().is_none());
    row.vouchers
}

fn entry(ledger: &str, side: EntrySide, amount: &str) -> ImportEntry {
    ImportEntry {
        ledger: ledger.into(),
        amount: ImportAmount::parse(amount).unwrap(),
        side,
    }
}

/// The ten vouchers as sent, in request order. That these are what was sent is
/// checked against the captured request bytes, not assumed.
fn sent() -> Vec<ImportVoucher> {
    use EntrySide::{Cr, Dr};
    use VoucherType::{Contra, Payment, Receipt};
    let v = |n: usize, voucher_type, narration: &str, entries| ImportVoucher {
        bridge_txn_id: format!("span-{n:02}"),
        date: bridge_tally_core::TallyDate::parse("20260710").unwrap(),
        voucher_type,
        narration: Some(narration.into()),
        reference: None,
        voucher_number: None,
        invoice: None,
        entries,
    };
    vec![
        v(
            1,
            Payment,
            "Electricity bill, K No 900000000001",
            vec![
                entry("Test Expense A", Dr, "1180.00"),
                entry("Test Bank", Cr, "1180.00"),
            ],
        ),
        v(
            2,
            Payment,
            "SMS alert charges",
            vec![
                entry("Test Expense B", Dr, "25.00"),
                entry("Test Bank", Cr, "25.00"),
            ],
        ),
        v(
            3,
            Payment,
            "SMS alert charges",
            vec![
                entry("Test Expense B", Dr, "25.00"),
                entry("Test Bank", Cr, "25.00"),
            ],
        ),
        v(
            4,
            Payment,
            "NEFT to Test Party, ref 300000000004",
            vec![
                entry("Test Party", Dr, "7500.00"),
                entry("Test Bank", Cr, "7500.00"),
            ],
        ),
        v(
            5,
            Receipt,
            "NEFT from Test Party, ref 400000000005",
            vec![
                entry("Test Bank", Dr, "12000.00"),
                entry("Test Party", Cr, "12000.00"),
            ],
        ),
        v(
            6,
            Receipt,
            "UPI from Test Party, ref 400000000006",
            vec![
                entry("Test Bank", Dr, "3450.50"),
                entry("Test Party", Cr, "3450.50"),
            ],
        ),
        v(
            7,
            Receipt,
            "Refund of electricity deposit, ref 400000000007",
            vec![
                entry("Test Bank", Dr, "99.00"),
                entry("Test Expense A", Cr, "99.00"),
            ],
        ),
        v(
            8,
            Receipt,
            "Interest credit",
            vec![
                entry("Test Bank", Dr, "1.00"),
                entry("Test Party", Cr, "1.00"),
            ],
        ),
        v(
            9,
            Contra,
            "Cash deposited",
            vec![
                entry("Test Bank", Dr, "5000.00"),
                entry("Cash", Cr, "5000.00"),
            ],
        ),
        v(
            10,
            Contra,
            "Cash withdrawn, cheque 000123",
            vec![
                entry("Cash", Dr, "2000.00"),
                entry("Test Bank", Cr, "2000.00"),
            ],
        ),
    ]
}

fn captured_remote_ids() -> Vec<Uuid> {
    IMPORT
        .split("REMOTEID=\"")
        .skip(1)
        .map(|rest| Uuid::parse_str(&rest[..36]).unwrap())
        .collect()
}

fn span_read() -> ImportReadSource {
    ImportReadSource::admit(parse_import_voucher_rows(&captured(SPAN_READ), COMPANY_GUID).unwrap())
        .unwrap()
}

fn outcome() -> bridge_tally_protocol::TallyImportOutcome {
    bridge_tally_protocol::parse_import_outcome(&captured(RESPONSE)).unwrap()
}

fn outcome_with(from: &str, to: &str) -> bridge_tally_protocol::TallyImportOutcome {
    let xml = captured(RESPONSE);
    assert_eq!(xml.matches(from).count(), 1, "{from}");
    bridge_tally_protocol::parse_import_outcome(&xml.replace(from, to)).unwrap()
}

fn span() -> PostSpan {
    PostSpan::after_clean_post(
        PreMark::recorded(target_mark(MARKS_BEFORE)),
        target_mark(MARKS_AFTER),
        &outcome(),
        10,
    )
    .unwrap()
}

fn refusal(result: Result<Vec<PostedVoucherIdentity>, BindError>) -> SpanRefusal {
    match result {
        Err(BindError::Refused(refusal)) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    }
}

#[test]
fn the_untagged_native_request_is_the_captured_request_byte_for_byte() {
    let vouchers = sent();
    let ids = captured_remote_ids();
    assert_eq!(ids.len(), 10);
    let xml = render_native_vouchers_xml(COMPANY, vouchers.iter().zip(ids));
    assert_eq!(xml, IMPORT.trim_end_matches('\n'));
    assert!(!xml.contains("[BRIDGE:"));
}

#[test]
fn ten_captured_vouchers_bind_in_request_order() {
    assert_eq!(
        (target_mark(MARKS_BEFORE), target_mark(MARKS_AFTER)),
        (1795, 1805)
    );
    let span = span();
    assert_eq!(
        span.alter_id_span(),
        crate::agent::AlterIdSpan {
            after: 1795,
            through: 1805
        }
    );
    let read = span_read();
    let bound = bind(&span, COMPANY_GUID, &sent(), &read, &BTreeSet::new()).unwrap();
    assert_eq!(bound.len(), 10);
    for (position, identity) in bound.iter().enumerate() {
        let master_id = 1724 + position as u64;
        assert_eq!(identity.bridge_txn_id, format!("span-{:02}", position + 1));
        assert_eq!(identity.master_id, master_id);
        assert_eq!(identity.guid, format!("{COMPANY_GUID}-{master_id:08x}"));
    }
    // The identical pair binds by position alone: the second sent to MasterID
    // 1725, the third to 1726.
    assert_eq!((bound[1].master_id, bound[2].master_id), (1725, 1726));
}

/// The binding compares a read narration with the one the post sends, which
/// has no outer whitespace (#1055): saved narrations padded with spaces and
/// U+00A0 send the captured request byte for byte and bind exactly as the
/// captured ones do.
#[test]
fn padded_saved_narrations_bind_as_the_text_the_post_sends() {
    let mut padded = sent();
    for voucher in &mut padded {
        let text = voucher.narration.take().unwrap();
        voucher.narration = Some(format!("  {text}\u{a0}"));
    }
    let xml = render_native_vouchers_xml(COMPANY, padded.iter().zip(captured_remote_ids()));
    assert_eq!(xml, IMPORT.trim_end_matches('\n'));
    let span = span();
    let read = span_read();
    assert_eq!(
        bind(&span, COMPANY_GUID, &padded, &read, &BTreeSet::new()),
        bind(&span, COMPANY_GUID, &sent(), &read, &BTreeSet::new())
    );
    assert_eq!(
        bind(&span, COMPANY_GUID, &padded, &read, &BTreeSet::new())
            .unwrap()
            .len(),
        10
    );
}

#[test]
fn a_post_span_needs_clean_counters_a_lastvchid_and_a_step_equal_to_created() {
    let before = PreMark::recorded(1795);
    let refused = |result: Result<PostSpan, BindError>| match result {
        Err(BindError::Refused(refusal)) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    };
    // The mark after stepped by 11, not 10: something else moved it.
    assert_eq!(
        refused(PostSpan::after_clean_post(before, 1806, &outcome(), 10)),
        SpanRefusal::StepNotCreated {
            step: 11,
            created: 10
        }
    );
    assert_eq!(
        refused(PostSpan::after_clean_post(before, 1794, &outcome(), 10)),
        SpanRefusal::MarkWentBack
    );
    // CREATED changed from 10 to 9.
    assert_eq!(
        refused(PostSpan::after_clean_post(
            before,
            1805,
            &outcome_with("<CREATED>10</CREATED>", "<CREATED>9</CREATED>"),
            10
        )),
        SpanRefusal::CreatedNotCount {
            created: 9,
            count: 10
        }
    );
    // EXCEPTIONS changed from 0 to 1.
    assert_eq!(
        refused(PostSpan::after_clean_post(
            before,
            1805,
            &outcome_with("<EXCEPTIONS>0</EXCEPTIONS>", "<EXCEPTIONS>1</EXCEPTIONS>"),
            10
        )),
        SpanRefusal::CountersNotClean
    );
    // LASTVCHID removed.
    assert_eq!(
        refused(PostSpan::after_clean_post(
            before,
            1805,
            &outcome_with("<LASTVCHID>1733</LASTVCHID>", ""),
            10
        )),
        SpanRefusal::LastVchIdAbsent
    );
    assert_eq!(
        refused(PostSpan::after_clean_post(before, 1805, &outcome(), 0)),
        SpanRefusal::EmptyBatch
    );
}

/// A wrapped reply that reports `STATUS=0` failed whatever its counters say
/// (IMPLEMENTATION_GUIDE.md §3.2, I6), so no span is bound from it, with or
/// without a mark read after the post (#2008).
#[test]
fn a_reply_reporting_status_zero_binds_no_span_whatever_its_counters() {
    let before = PreMark::recorded(1795);
    let failed = |status: &str| {
        bridge_tally_protocol::parse_import_outcome(&format!(
            "<ENVELOPE><HEADER><STATUS>{status}</STATUS></HEADER><BODY><DATA><IMPORTRESULT><CREATED>10</CREATED><ALTERED>0</ALTERED><DELETED>0</DELETED><LASTVCHID>1733</LASTVCHID><IGNORED>0</IGNORED><ERRORS>0</ERRORS><CANCELLED>0</CANCELLED><EXCEPTIONS>0</EXCEPTIONS></IMPORTRESULT></DATA></BODY></ENVELOPE>"
        ))
        .unwrap()
    };
    // The same counters under STATUS=1 make a span: only the status differs.
    assert!(PostSpan::after_clean_post(before, 1805, &failed("1"), 10).is_ok());
    assert!(matches!(
        PostSpan::after_clean_post(before, 1805, &failed("0"), 10),
        Err(BindError::Refused(SpanRefusal::StatusFailure))
    ));
    assert_eq!(
        PostSpan::after_clean_response(before, &failed("0"), 10),
        Err(SpanRefusal::StatusFailure)
    );
    assert_eq!(
        SpanRefusal::StatusFailure.code(),
        "span_response_status_failure"
    );
}

/// Verification treats an absent `EFFECTIVEDATE` as not observed, so the
/// binding is decided again by the next read, never refused for good. A real
/// difference later in the span still refuses: the absence, met first, never
/// hides it.
#[test]
fn an_absent_effective_date_leaves_the_binding_unsettled_not_refused() {
    let span = span();
    let mut read = span_read();
    read.rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1796))
        .unwrap()
        .effective_date = None;
    assert_eq!(
        bind(&span, COMPANY_GUID, &sent(), &read, &BTreeSet::new()),
        Err(BindError::Unsettled(
            BindUnsettled::EffectiveDateNotObserved
        ))
    );
    let mut changed = read.clone();
    changed
        .rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1800))
        .unwrap()
        .narration = Some("NEFT from Test Party".into());
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &sent(),
            &changed,
            &BTreeSet::new()
        )),
        SpanRefusal::Content {
            position: 4,
            fields: vec!["narration"]
        }
    );
}

#[test]
fn a_span_that_is_not_exactly_the_post_refuses() {
    let span = span();
    // One row removed from the captured span.
    let mut short = span_read();
    short.rows.remove(4);
    assert_eq!(
        refusal(bind(&span, COMPANY_GUID, &sent(), &short, &BTreeSet::new())),
        SpanRefusal::SpanCount {
            expected: 10,
            observed: 9
        }
    );
    // The first row's ALTERID moved one past the span start.
    let mut shifted = span_read();
    let first = shifted
        .rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1796))
        .unwrap();
    first.alter_id = Some(1807);
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &sent(),
            &shifted,
            &BTreeSet::new()
        )),
        SpanRefusal::Position { position: 0 }
    );
}

#[test]
fn content_that_differs_from_what_was_sent_refuses_at_its_position() {
    let span = span();
    let read = span_read();
    // The first and fourth sent vouchers swapped.
    let mut swapped = sent();
    swapped.swap(0, 3);
    assert!(matches!(
        refusal(bind(&span, COMPANY_GUID, &swapped, &read, &BTreeSet::new())),
        SpanRefusal::Content { position: 0, .. }
    ));
    // One byte of the sixth narration changed.
    let mut renarrated = sent();
    renarrated[5].narration = Some("UPI from Test Party, ref 400000000007".into());
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &renarrated,
            &read,
            &BTreeSet::new()
        )),
        SpanRefusal::Content {
            position: 5,
            fields: vec!["narration"]
        }
    );
    // The seventh amount changed by one paisa.
    let mut reamounted = sent();
    for entry in &mut reamounted[6].entries {
        entry.amount = ImportAmount::parse("99.01").unwrap();
    }
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &reamounted,
            &read,
            &BTreeSet::new()
        )),
        SpanRefusal::Content {
            position: 6,
            fields: vec!["entries"]
        }
    );
    // The ninth voucher's EFFECTIVEDATE read as a different day.
    let mut redated = read.clone();
    let ninth = redated
        .rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1804))
        .unwrap();
    ninth.effective_date = Some(bridge_tally_core::TallyDate::parse("20260711").unwrap());
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &sent(),
            &redated,
            &BTreeSet::new()
        )),
        SpanRefusal::Content {
            position: 8,
            fields: vec!["effective_date"]
        }
    );
}

#[test]
fn a_cancelled_or_unobserved_state_refuses() {
    let span = span();
    let mut cancelled = span_read();
    let third = cancelled
        .rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1798))
        .unwrap();
    third.cancelled = Some(true);
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &sent(),
            &cancelled,
            &BTreeSet::new()
        )),
        SpanRefusal::NotEffective { position: 2 }
    );
    let mut unobserved = span_read();
    let third = unobserved
        .rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1798))
        .unwrap();
    third.optional = None;
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &sent(),
            &unobserved,
            &BTreeSet::new()
        )),
        SpanRefusal::NotEffective { position: 2 }
    );
}

#[test]
fn master_ids_and_guids_must_follow_lastvchid() {
    let read = span_read();
    // LASTVCHID changed from 1733 to 1734: the MasterIDs no longer end on it.
    let off_by_one = PostSpan::after_clean_post(
        PreMark::recorded(1795),
        1805,
        &outcome_with("<LASTVCHID>1733</LASTVCHID>", "<LASTVCHID>1734</LASTVCHID>"),
        10,
    )
    .unwrap();
    assert_eq!(
        refusal(bind(
            &off_by_one,
            COMPANY_GUID,
            &sent(),
            &read,
            &BTreeSet::new()
        )),
        SpanRefusal::MasterIdNotInSequence { position: 0 }
    );
    // The fifth GUID's MasterID suffix changed.
    let mut reguided = read.clone();
    let fifth = reguided
        .rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1800))
        .unwrap();
    fifth.guid = Some(format!("{COMPANY_GUID}-000006ff"));
    assert_eq!(
        refusal(bind(
            &span(),
            COMPANY_GUID,
            &sent(),
            &reguided,
            &BTreeSet::new()
        )),
        SpanRefusal::GuidNotMasterId { position: 4 }
    );
}

#[test]
fn a_lost_after_mark_binds_later_on_the_span_the_response_implies() {
    let span = PostSpan::after_clean_response(PreMark::recorded(1795), &outcome(), 10).unwrap();
    assert_eq!(
        span.alter_id_span(),
        crate::agent::AlterIdSpan {
            after: 1795,
            through: 1805
        }
    );
    let read = span_read();
    let bound = bind(&span, COMPANY_GUID, &sent(), &read, &BTreeSet::new()).unwrap();
    assert_eq!(bound.len(), 10);
    // Without a clean response there is nothing to bind later.
    assert_eq!(
        PostSpan::after_clean_response(
            PreMark::recorded(1795),
            &outcome_with("<CREATED>10</CREATED>", "<CREATED>9</CREATED>"),
            10
        ),
        Err(SpanRefusal::CreatedNotCount {
            created: 9,
            count: 10
        })
    );
    assert_eq!(
        PostSpan::after_clean_response(
            PreMark::recorded(1795),
            &outcome_with("<LASTVCHID>1733</LASTVCHID>", ""),
            10
        ),
        Err(SpanRefusal::LastVchIdAbsent)
    );
}

#[test]
fn a_guid_another_batch_bound_is_never_bound_again() {
    let read = span_read();
    let elsewhere = BTreeSet::from([format!("{COMPANY_GUID}-{:08x}", 1727)]);
    assert_eq!(
        refusal(bind(&span(), COMPANY_GUID, &sent(), &read, &elsewhere)),
        SpanRefusal::IdentityReused { position: 3 }
    );
}

#[test]
fn a_lastvchid_below_the_count_refuses_rather_than_wrapping() {
    let span = PostSpan::after_clean_response(
        PreMark::recorded(1795),
        &outcome_with("<LASTVCHID>1733</LASTVCHID>", "<LASTVCHID>3</LASTVCHID>"),
        10,
    )
    .unwrap();
    let read = span_read();
    assert_eq!(
        refusal(bind(&span, COMPANY_GUID, &sent(), &read, &BTreeSet::new())),
        SpanRefusal::MasterIdNotInSequence { position: 0 }
    );
}

// The second capture: 5 untagged Journals (`fixtures/POST_SPAN_CAPTURE_PROVENANCE.md`,
// `post-span-journal-*`).
const JOURNAL_IMPORT: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-journal-import.xml"
);
const JOURNAL_RESPONSE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-journal-import-response.utf16le.xml"
);
const JOURNAL_MARKS_BEFORE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-journal-company-high-water-before.utf16le.xml"
);
const JOURNAL_MARKS_AFTER: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-journal-company-high-water-after.utf16le.xml"
);
const JOURNAL_SPAN_READ: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-journal-alterid-span-read.utf16le.xml"
);

/// The five Journals as sent, in request order and in the caller's entry order.
fn sent_journals() -> Vec<ImportVoucher> {
    use EntrySide::{Cr, Dr};
    let v = |n: usize, narration: &str, entries| ImportVoucher {
        bridge_txn_id: format!("journal-{n:02}"),
        date: bridge_tally_core::TallyDate::parse("20260711").unwrap(),
        voucher_type: VoucherType::Journal,
        narration: Some(narration.into()),
        reference: None,
        voucher_number: None,
        invoice: None,
        entries,
    };
    vec![
        v(
            1,
            "Accrual for July electricity",
            vec![
                entry("Test Expense A", Dr, "500.00"),
                entry("Test Party", Cr, "500.00"),
            ],
        ),
        v(
            2,
            "Reclass of bank charges",
            vec![
                entry("Test Expense B", Dr, "75.00"),
                entry("Test Party", Cr, "75.00"),
            ],
        ),
        v(
            3,
            "Reclass of bank charges",
            vec![
                entry("Test Expense B", Dr, "75.00"),
                entry("Test Party", Cr, "75.00"),
            ],
        ),
        v(
            4,
            "Reversal of excess accrual",
            vec![
                entry("Test Party", Dr, "120.00"),
                entry("Test Expense A", Cr, "120.00"),
            ],
        ),
        v(
            5,
            "Split of a shared cost",
            vec![
                entry("Test Expense A", Dr, "300.00"),
                entry("Test Expense B", Dr, "200.00"),
                entry("Test Party", Cr, "500.00"),
            ],
        ),
    ]
}

/// #2062: the cancel candidate of each captured request (Payment, Receipt and
/// Contra; Journal) is that request with only each voucher's `ACTION` changed.
#[test]
fn the_cancel_candidate_of_each_captured_request_changes_only_its_actions() {
    let journal_ids = JOURNAL_IMPORT
        .split("REMOTEID=\"")
        .skip(1)
        .map(|rest| Uuid::parse_str(&rest[..36]).unwrap())
        .collect::<Vec<_>>();
    for (vouchers, ids) in [
        (sent(), captured_remote_ids()),
        (sent_journals(), journal_ids),
    ] {
        let pairs = vouchers.iter().zip(ids).collect::<Vec<_>>();
        super::super::cancel::tests::assert_cancel_changes_only_the_action(COMPANY, &pairs);
    }
}

#[test]
fn five_captured_journals_are_bridges_own_request_and_bind_in_request_order() {
    let ids = JOURNAL_IMPORT
        .split("REMOTEID=\"")
        .skip(1)
        .map(|rest| Uuid::parse_str(&rest[..36]).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(ids.len(), 5);
    let xml = render_native_vouchers_xml(COMPANY, sent_journals().iter().zip(ids));
    assert_eq!(xml, JOURNAL_IMPORT.trim_end_matches('\n'));
    assert_eq!(
        (
            target_mark(JOURNAL_MARKS_BEFORE),
            target_mark(JOURNAL_MARKS_AFTER)
        ),
        (1805, 1810)
    );
    let outcome = bridge_tally_protocol::parse_import_outcome(&captured(JOURNAL_RESPONSE)).unwrap();
    let span = PostSpan::after_clean_post(PreMark::recorded(1805), 1810, &outcome, 5).unwrap();
    let read = ImportReadSource::admit(
        parse_import_voucher_rows(&captured(JOURNAL_SPAN_READ), COMPANY_GUID).unwrap(),
    )
    .unwrap();
    let bound = bind(
        &span,
        COMPANY_GUID,
        &sent_journals(),
        &read,
        &BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(bound.len(), 5);
    for (position, identity) in bound.iter().enumerate() {
        let master_id = 1734 + position as u64;
        assert_eq!(
            identity.bridge_txn_id,
            format!("journal-{:02}", position + 1)
        );
        assert_eq!(identity.master_id, master_id);
        assert_eq!(identity.guid, format!("{COMPANY_GUID}-{master_id:08x}"));
    }
    // The identical pair binds by position alone.
    assert_eq!((bound[1].master_id, bound[2].master_id), (1735, 1736));
}

/// The ten captured vouchers saved as one batch on a server of its own, with
/// the journal on disk that `decide_post_span` records its verdict in.
fn journaled_batch(directory: &std::path::Path) -> (crate::agent::Server, ImportLedgerLine) {
    journaled_batch_answered(directory, outcome())
}

/// [`journaled_batch`], with `answer` as the post's recorded response.
fn journaled_batch_answered(
    directory: &std::path::Path,
    answer: bridge_tally_protocol::TallyImportOutcome,
) -> (crate::agent::Server, ImportLedgerLine) {
    let server = crate::agent::Server::new(crate::agent::Settings {
        endpoint: bridge_tally_transport::TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        posting: crate::agent::Posting::On,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let batch_id = "bridge-00000000-0000-4000-8000-000000001055";
    let vouchers = sent();
    let mut line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id": batch_id, "identity_scheme": "batch_v1",
        "company_guid": COMPANY_GUID,
        "company": {"name": COMPANY, "guid": COMPANY_GUID, "company_number": "100001", "books_from": "20260401"},
        "txn_ids": vouchers.iter().map(|voucher| voucher.bridge_txn_id.clone()).collect::<Vec<_>>(),
        "date_from": "20260710", "date_to": "20260710",
        "sha256": "", "built_at": "2026-10-02T07:45:00.000Z", "status": "built",
        "pre_import_mark": {"kind": "company_high_water", "value": target_mark(MARKS_BEFORE), "master_value": 234},
        "vouchers": vouchers,
    }))
    .unwrap();
    line.sha256 = crate::agent::sha256_hex(
        super::super::render_import_xml(COMPANY, &line.vouchers, batch_id).as_bytes(),
    );
    server.append_import_ledger(&line).unwrap();
    // What the post left in the journal: its dispatch intent, with the
    // pre-POST mark and the captured REMOTEIDs, and its response.
    let native = super::super::post::native_post_request(
        &line,
        super::super::post::RemoteIds::from_ids(captured_remote_ids()),
    )
    .unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&super::super::ledger::StatusRecord::dispatch_for(
                &line,
                &native,
                Some(target_mark(MARKS_BEFORE)),
            ))
            .unwrap();
        server
            .append_import_record_while_admitted(&super::super::ledger::StatusRecord::response(
                &line,
                super::super::ledger::DispatchResponse {
                    request_sha256: native.request_sha256.clone(),
                    response_sha256: crate::agent::sha256_hex(RESPONSE),
                    bytes: RESPONSE.len(),
                    outcome: Some(answer),
                },
            ))
            .unwrap();
    }
    (server, line)
}

/// The decision on the captured post answered `STATUS=0` with `CREATED` 10, by
/// a check that reads the company's mark where it stood before the post, with
/// no mark read after it; `bound`, when given, stands in for the journal's
/// verdict. Returns the decision and the pre-POST mark.
fn status_zero_decision(
    bound: Option<Vec<PostedVoucherIdentity>>,
) -> (super::super::PostSpanDecision, u64) {
    let failed = bridge_tally_protocol::parse_import_outcome(
        "<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER><BODY><DATA><IMPORTRESULT><CREATED>10</CREATED><ALTERED>0</ALTERED><DELETED>0</DELETED><LASTVCHID>1805</LASTVCHID><IGNORED>0</IGNORED><ERRORS>0</ERRORS><CANCELLED>0</CANCELLED><EXCEPTIONS>0</EXCEPTIONS></IMPORTRESULT></DATA></BODY></ENVELOPE>",
    )
    .unwrap();
    assert_eq!(failed.counters().created, 10);
    assert_eq!(
        failed.application_status(),
        bridge_tally_protocol::TallyImportApplicationStatus::Failure
    );
    let directory = tempfile::tempdir().unwrap();
    let (server, line) = journaled_batch_answered(directory.path(), failed);
    let snapshot = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.span_verdict, None);
    let response = snapshot.response.expect("the post's recorded response");
    let before = snapshot
        .pre_post_voucher_mark
        .expect("the recorded pre-POST mark");
    let decision = server.decide_post_span(
        &line,
        PreMark::recorded(before),
        bound.map(super::super::ledger::PostSpanVerdict::Bound),
        Some(&response),
        &span_read(),
        None,
        before,
        snapshot.generation,
    );
    (decision, before)
}

/// A recorded answer that reported `STATUS=0` proves no voucher landed
/// (#2035): the book is not read as rolled back, and the binding refuses the
/// answer (#2008).
#[test]
fn a_status_zero_answer_does_not_read_an_unchanged_book_as_rolled_back() {
    let (decision, _) = status_zero_decision(None);
    assert!(!decision.rolled_back, "{}", decision.report);
    assert_eq!(
        decision.report,
        json!({"state": "refused", "code": SpanRefusal::StatusFailure.code()})
    );
}

/// The vouchers a post bound outrank its answer in the floor, as in the
/// journal's own floor: the same mark reads as rolled back when the post is
/// bound, whatever its answer's status (#2035). No build since #2008 binds
/// from such an answer; the verdict stands in for one journaled earlier.
#[test]
fn a_bound_post_keeps_its_floor_whatever_its_answers_status() {
    let bound = bind(
        &span(),
        COMPANY_GUID,
        &sent(),
        &span_read(),
        &BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(bound.len(), 10);
    let (decision, before) = status_zero_decision(Some(bound));
    assert!(decision.rolled_back, "{}", decision.report);
    assert_eq!(
        decision.report,
        json!({"state": "book_rolled_back", "current_mark": before, "expected_at_least": before + 10})
    );
}

/// One verification's decision on the captured post, as `verify_import` makes
/// it: from the journal as it stands, with the captured response, the marks
/// either side of the POST and `read` as the window's rows.
fn decided(
    server: &crate::agent::Server,
    line: &ImportLedgerLine,
    read: &ImportReadSource,
) -> (Value, Option<super::super::ledger::PostSpanVerdict>) {
    let snapshot = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap();
    let response = snapshot.response.expect("the post's recorded response");
    let decision = server.decide_post_span(
        line,
        PreMark::recorded(
            snapshot
                .pre_post_voucher_mark
                .expect("the recorded pre-POST mark"),
        ),
        snapshot.span_verdict,
        Some(&response),
        read,
        Some(target_mark(MARKS_AFTER)),
        target_mark(MARKS_AFTER),
        snapshot.generation,
    );
    let journaled = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap()
        .span_verdict;
    (decision.report, journaled)
}

/// An unsettled bind is never journaled (#1055, point 9): with one voucher's
/// `EFFECTIVEDATE` left out of the read, the decision is `unsettled` and the
/// journal holds no verdict, so the next verification decides again; the
/// read as captured then binds, and that verdict is journaled. Server level,
/// on the captured rows: the transport and `verify_import`'s own reads are
/// not exercised, since no census of this day is captured.
#[test]
fn an_unsettled_bind_is_left_out_of_the_journal_and_a_later_read_binds() {
    let directory = tempfile::tempdir().unwrap();
    let (server, line) = journaled_batch(directory.path());
    let mut without = span_read();
    without
        .rows
        .iter_mut()
        .find(|row| row.alter_id == Some(1796))
        .unwrap()
        .effective_date = None;

    let (report, journaled) = decided(&server, &line, &without);
    assert_eq!(
        report,
        json!({"state": "unsettled", "code": BindUnsettled::EffectiveDateNotObserved.code()})
    );
    assert_eq!(journaled, None, "an unsettled bind is never journaled");

    let (report, journaled) = decided(&server, &line, &span_read());
    assert_eq!(report, json!({"state": "bound"}));
    let expected = bind(
        &span(),
        COMPANY_GUID,
        &sent(),
        &span_read(),
        &BTreeSet::new(),
    )
    .unwrap();
    assert_eq!(expected.len(), 10);
    assert_eq!(
        journaled,
        Some(super::super::ledger::PostSpanVerdict::Bound(expected))
    );
}

/// The read-back of one Journal posted on a ledger whose name ends in CR LF
/// (`fixtures/POST_LINE_BREAK_CAPTURE_PROVENANCE.md`).
const LINE_BREAK_READ: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-line-break-ledger-readback.utf16le.xml"
);
const LINE_BREAK_COMPANY_GUID: &str = "224e7f8c-a0aa-45ce-889a-ad4b7fdbe852";

/// bridge#626: what the row parser makes of a `LEDGERNAME` that carries a
/// line break at its end. Tally wrote it back as two character references
/// (measured live, 10 Oct 2026), and the row's name ends in CR LF. Written as
/// the two raw characters, which no capture has shown, the row's name ends
/// in CR LF as well: the parser does not fold line ends. That second
/// spelling is the capture with that one value changed.
#[test]
fn a_read_back_ledger_name_keeps_a_line_break_in_either_spelling() {
    let sent = captured(LINE_BREAK_READ);
    let raw = sent.replace(
        ">Lab Line Break Ledger&#13;&#10;</LEDGERNAME>",
        ">Lab Line Break Ledger\r\n</LEDGERNAME>",
    );
    assert_ne!(raw, sent, "the capture spells the break as references");
    for (spelling, xml) in [("references", &sent), ("raw", &raw)] {
        let rows = parse_import_voucher_rows(xml, LINE_BREAK_COMPANY_GUID).unwrap();
        assert_eq!(rows.len(), 1, "{spelling}");
        let named: Vec<&str> = rows[0]
            .entries
            .iter()
            .map(|entry| entry.ledger.as_str())
            .collect();
        assert_eq!(
            named,
            ["Lab Line Break Ledger\r\n", "Lab Plain Ledger"],
            "{spelling}"
        );
    }
}

/// #626: a voucher that landed on the other ledger of a line-break pair does
/// not match what was sent, in either direction, so it cannot read verified.
#[test]
fn a_read_back_on_the_other_ledger_of_a_line_break_pair_does_not_match() {
    let rows =
        parse_import_voucher_rows(&captured(LINE_BREAK_READ), LINE_BREAK_COMPANY_GUID).unwrap();
    let read = |ledger: &str| {
        let mut row = rows[0].clone();
        row.entries[0].ledger = ledger.to_string();
        actual_entry_fingerprint(&row)
    };
    assert_eq!(
        read("Lab Line Break Ledger\r\n"),
        actual_entry_fingerprint(&rows[0])
    );
    assert_ne!(
        read("Lab Line Break Ledger"),
        actual_entry_fingerprint(&rows[0])
    );
}

/// #1633: only a mark that moved by more than the post created, or a span that
/// later held another number of vouchers, says another change was made during
/// the post. The match has no wildcard, so a new refusal must be sorted here.
#[test]
fn only_the_step_and_count_refusals_name_a_concurrent_change() {
    fn names_concurrent_change(refusal: &SpanRefusal) -> bool {
        match refusal {
            SpanRefusal::StepNotCreated { .. } | SpanRefusal::SpanCount { .. } => true,
            // A reply reporting `STATUS=0` says Tally failed the post: it is no
            // evidence of another change, and the line's "Tally reported
            // creating each voucher once" would not be true of it (#2008).
            SpanRefusal::EmptyBatch
            | SpanRefusal::StatusFailure
            | SpanRefusal::CountersNotClean
            | SpanRefusal::CreatedNotCount { .. }
            | SpanRefusal::LastVchIdAbsent
            | SpanRefusal::MarkWentBack
            | SpanRefusal::Position { .. }
            | SpanRefusal::Content { .. }
            | SpanRefusal::NotEffective { .. }
            | SpanRefusal::MasterIdNotInSequence { .. }
            | SpanRefusal::GuidNotMasterId { .. }
            | SpanRefusal::IdentityReused { .. } => false,
        }
    }
    let every = [
        SpanRefusal::EmptyBatch,
        SpanRefusal::StatusFailure,
        SpanRefusal::CountersNotClean,
        SpanRefusal::CreatedNotCount {
            created: 1,
            count: 2,
        },
        SpanRefusal::LastVchIdAbsent,
        SpanRefusal::MarkWentBack,
        SpanRefusal::StepNotCreated {
            step: 4,
            created: 1,
        },
        SpanRefusal::SpanCount {
            expected: 1,
            observed: 2,
        },
        SpanRefusal::Position { position: 0 },
        SpanRefusal::Content {
            position: 0,
            fields: vec!["date"],
        },
        SpanRefusal::NotEffective { position: 0 },
        SpanRefusal::MasterIdNotInSequence { position: 0 },
        SpanRefusal::GuidNotMasterId { position: 0 },
        SpanRefusal::IdentityReused { position: 0 },
    ];
    for refusal in &every {
        assert_eq!(
            concurrent_change_line_for(refusal.code()),
            names_concurrent_change(refusal).then_some(CONCURRENT_CHANGE_LINE),
            "{}",
            refusal.code()
        );
    }
    assert_eq!(concurrent_change_line_for("span_unknown_code"), None);
}

/// Sign-rule site 4 (#1872): the span identity compares what was sent with the row in canonical
/// form, so the sixth captured voucher (3450.50) written `03450.50` still binds, and one paisa
/// more does not. The batch-side text is a lexeme; the comparison is not.
#[test]
fn a_padded_amount_binds_against_the_row_tally_holds() {
    let span = span();
    let read = span_read();
    let mut padded = sent();
    for entry in &mut padded[5].entries {
        assert_eq!(entry.amount.as_str(), "3450.50");
        entry.amount = ImportAmount::parse("03450.50").unwrap();
    }
    let bound = bind(&span, COMPANY_GUID, &padded, &read, &BTreeSet::new()).unwrap();
    assert_eq!(bound.len(), 10);
    let mut paisa = sent();
    for entry in &mut paisa[5].entries {
        entry.amount = ImportAmount::parse("03450.51").unwrap();
    }
    assert_eq!(
        refusal(bind(&span, COMPANY_GUID, &paisa, &read, &BTreeSet::new())),
        SpanRefusal::Content {
            position: 5,
            fields: vec!["entries"]
        }
    );
}

/// What the amount type does with the 256-byte amount here (#1872): nothing. A debit is
/// compared signed, and the signed text of the longest amount is 257 bytes, one over what
/// `ExactDecimal` holds, so the comparison errors and is read as a difference in the entries,
/// as it was before the type. The book's row holds that amount too, and its debit is also 257
/// bytes signed, so the read side fails on its own: this test cannot tell which side refused,
/// and the outcome is the same either way. The row is built here from the sixth captured one
/// with its amounts replaced; no Tally capture holds such a voucher.
#[test]
fn a_256_byte_debit_cannot_be_matched_because_its_signed_text_is_257_bytes() {
    let span = span();
    let longest = format!("{}.99", "9".repeat(253));
    assert_eq!(longest.len(), 256);
    let mut rows = parse_import_voucher_rows(&captured(SPAN_READ), COMPANY_GUID).unwrap();
    let position = rows
        .iter()
        .position(|row| {
            row.entries
                .iter()
                .any(|entry| entry.amount.ends_with("3450.50"))
        })
        .expect("the sixth captured voucher");
    for entry in &mut rows[position].entries {
        entry.amount = if entry.is_deemed_positive == "Yes" {
            format!("-{longest}")
        } else {
            longest.clone()
        };
    }
    let read = ImportReadSource::admit(rows).unwrap();
    let mut long = sent();
    for entry in &mut long[5].entries {
        entry.amount = ImportAmount::parse(&longest).unwrap();
    }
    assert_eq!(
        long[5].entries[0]
            .amount
            .signed_lexeme(&EntrySide::Dr)
            .len(),
        257
    );
    assert_eq!(
        refusal(bind(&span, COMPANY_GUID, &long, &read, &BTreeSet::new())),
        SpanRefusal::Content {
            position: 5,
            fields: vec!["entries"]
        }
    );
}

/// A padded debit that is 256 bytes (`000…12.50`) signs to 257 bytes, so it cannot bind by span,
/// where its canonical form (`12.5`) would, and a book row holding `-12.50` is the same figure.
/// That is how it was before the type (#1872), because the sign goes on the text as received.
/// The same row binds the plain `12.50`, which shows the row is the right one. This guards a
/// later change of the span identity to `signed_canonical`.
#[test]
fn a_padded_256_byte_debit_cannot_bind_although_its_canonical_form_would() {
    let span = span();
    let padded = format!("{}12.50", "0".repeat(251));
    assert_eq!(padded.len(), 256);
    let mut rows = parse_import_voucher_rows(&captured(SPAN_READ), COMPANY_GUID).unwrap();
    let position = rows
        .iter()
        .position(|row| {
            row.entries
                .iter()
                .any(|entry| entry.amount.ends_with("3450.50"))
        })
        .expect("the sixth captured voucher");
    for entry in &mut rows[position].entries {
        entry.amount = if entry.is_deemed_positive == "Yes" {
            "-12.50".to_string()
        } else {
            "12.50".to_string()
        };
    }
    let read = ImportReadSource::admit(rows).unwrap();
    let with_amount = |text: &str| {
        let mut sent = sent();
        for entry in &mut sent[5].entries {
            entry.amount = ImportAmount::parse(text).unwrap();
        }
        sent
    };
    assert_eq!(
        bind(
            &span,
            COMPANY_GUID,
            &with_amount("12.50"),
            &read,
            &BTreeSet::new()
        )
        .unwrap()
        .len(),
        10
    );
    let long = with_amount(&padded);
    assert_eq!(
        long[5].entries[0]
            .amount
            .signed_lexeme(&EntrySide::Dr)
            .len(),
        257
    );
    assert_eq!(
        long[5].entries[0].amount.signed_canonical(&EntrySide::Dr),
        "-12.5"
    );
    assert_eq!(
        refusal(bind(&span, COMPANY_GUID, &long, &read, &BTreeSet::new())),
        SpanRefusal::Content {
            position: 5,
            fields: vec!["entries"]
        }
    );
}

// Tests added for the #2058 mutation record: each holds a change that the
// tests above let survive (`src-tauri/tests/mutations/agent_import_span_identity.json`).

fn row_mut(read: &mut ImportReadSource, alter_id: u64) -> &mut ReadVoucher {
    read.rows
        .iter_mut()
        .find(|row| row.alter_id == Some(alter_id))
        .unwrap()
}

fn bind_default(
    span: &PostSpan,
    vouchers: &[ImportVoucher],
    read: &ImportReadSource,
) -> Result<Vec<PostedVoucherIdentity>, BindError> {
    bind(span, COMPANY_GUID, vouchers, read, &BTreeSet::new())
}

/// The captured span, ended on `last` instead of the captured `LASTVCHID`.
fn span_ending_at(last: u64) -> PostSpan {
    PostSpan::after_clean_response(
        PreMark::recorded(1795),
        &outcome_with(
            "<LASTVCHID>1733</LASTVCHID>",
            &format!("<LASTVCHID>{last}</LASTVCHID>"),
        ),
        10,
    )
    .unwrap()
}

/// Numbers the ten captured rows, in AlterID order, with the MasterIDs
/// `first`, `first + 1`, ... (wrapping) and the GUIDs that follow them.
fn renumber(read: &mut ImportReadSource, first: u64) {
    for row in &mut read.rows {
        let master_id = first.wrapping_add(row.alter_id.unwrap() - 1796);
        row.master_id = Some(master_id.to_string());
        row.guid = Some(format!("{COMPANY_GUID}-{master_id:08x}"));
    }
}

/// A day that no captured voucher carries.
fn other_day() -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse(concat!("2026", "0712")).unwrap()
}

fn reply(
    status: &str,
    created: &str,
    altered: &str,
    last_vch_id: &str,
) -> bridge_tally_protocol::TallyImportOutcome {
    bridge_tally_protocol::parse_import_outcome(&format!(
        "<ENVELOPE><HEADER><STATUS>{status}</STATUS></HEADER><BODY><DATA><IMPORTRESULT><CREATED>{created}</CREATED><ALTERED>{altered}</ALTERED><DELETED>0</DELETED>{last_vch_id}<IGNORED>0</IGNORED><ERRORS>0</ERRORS><CANCELLED>0</CANCELLED><EXCEPTIONS>0</EXCEPTIONS></IMPORTRESULT></DATA></BODY></ENVELOPE>"
    ))
    .unwrap()
}

#[test]
fn every_refusal_and_unsettled_code_is_its_own_string_whatever_its_fields() {
    let cases = [
        (SpanRefusal::EmptyBatch, "span_empty_batch"),
        (SpanRefusal::StatusFailure, "span_response_status_failure"),
        (SpanRefusal::CountersNotClean, "span_counters_not_clean"),
        (
            SpanRefusal::CreatedNotCount {
                created: 1,
                count: 2,
            },
            "span_created_not_count",
        ),
        (
            SpanRefusal::CreatedNotCount {
                created: 0,
                count: 0,
            },
            "span_created_not_count",
        ),
        (SpanRefusal::LastVchIdAbsent, "span_lastvchid_absent"),
        (SpanRefusal::MarkWentBack, "span_mark_went_back"),
        (
            SpanRefusal::StepNotCreated {
                step: 1,
                created: 1,
            },
            "span_step_not_created",
        ),
        (
            SpanRefusal::StepNotCreated {
                step: 0,
                created: 0,
            },
            "span_step_not_created",
        ),
        (
            SpanRefusal::SpanCount {
                expected: 1,
                observed: 1,
            },
            "span_count_mismatch",
        ),
        (
            SpanRefusal::SpanCount {
                expected: 0,
                observed: 0,
            },
            "span_count_mismatch",
        ),
        (
            SpanRefusal::Position { position: 3 },
            "span_position_mismatch",
        ),
        (
            SpanRefusal::Content {
                position: 3,
                fields: vec!["date"],
            },
            "span_content_mismatch",
        ),
        (
            SpanRefusal::NotEffective { position: 3 },
            "span_voucher_not_effective",
        ),
        (
            SpanRefusal::MasterIdNotInSequence { position: 3 },
            "span_master_id_not_in_sequence",
        ),
        (
            SpanRefusal::GuidNotMasterId { position: 3 },
            "span_guid_not_master_id",
        ),
        (
            SpanRefusal::IdentityReused { position: 3 },
            "span_identity_reused",
        ),
    ];
    for (refusal, code) in &cases {
        assert_eq!(refusal.code(), *code, "{refusal:?}");
    }
    let distinct = cases.iter().map(|(_, code)| *code).collect::<BTreeSet<_>>();
    assert_eq!(distinct.len(), 14);
    assert_eq!(
        BindUnsettled::EffectiveDateNotObserved.code(),
        "binding_effective_date_not_observed"
    );
    assert_eq!(
        BindUnsettled::JournalUnreadable.code(),
        "binding_journal_unreadable"
    );
}

#[test]
fn a_counter_that_is_not_zero_or_not_reported_is_not_a_clean_create() {
    let before = PreMark::recorded(1795);
    for counter in [
        "ALTERED",
        "DELETED",
        "IGNORED",
        "ERRORS",
        "CANCELLED",
        "EXCEPTIONS",
    ] {
        let one = outcome_with(
            &format!("<{counter}>0</{counter}>"),
            &format!("<{counter}>1</{counter}>"),
        );
        assert_eq!(
            PostSpan::after_clean_response(before, &one, 10),
            Err(SpanRefusal::CountersNotClean),
            "{counter} reported as 1"
        );
    }
    // DELETED, CANCELLED and EXCEPTIONS may be left out of a reply: a counter
    // that was not reported is not a zero.
    for counter in ["DELETED", "CANCELLED", "EXCEPTIONS"] {
        let none = outcome_with(&format!("<{counter}>0</{counter}>"), "");
        assert_eq!(
            PostSpan::after_clean_response(before, &none, 10),
            Err(SpanRefusal::CountersNotClean),
            "{counter} not reported"
        );
    }
    // A LINEERROR beside all-zero counters.
    let line_error = outcome_with(
        "<EXCEPTIONS>0</EXCEPTIONS>",
        "<EXCEPTIONS>0</EXCEPTIONS><LINEERROR>synthetic line error</LINEERROR>",
    );
    assert_eq!(
        PostSpan::after_clean_response(before, &line_error, 10),
        Err(SpanRefusal::CountersNotClean)
    );
}

#[test]
fn a_created_count_above_the_batch_refuses_as_one_below_does() {
    let before = PreMark::recorded(1795);
    assert_eq!(
        PostSpan::after_clean_response(
            before,
            &outcome_with("<CREATED>10</CREATED>", "<CREATED>11</CREATED>"),
            10
        ),
        Err(SpanRefusal::CreatedNotCount {
            created: 11,
            count: 10
        })
    );
    // 2^32 + 10: not 10 on a target whose usize is narrower than 64 bits, where
    // a cast would drop the high bits and read the count as matched.
    let wide = (1_u64 << 32) + 10;
    assert_eq!(
        PostSpan::after_clean_response(
            before,
            &outcome_with(
                "<CREATED>10</CREATED>",
                &format!("<CREATED>{wide}</CREATED>")
            ),
            10
        ),
        Err(SpanRefusal::CreatedNotCount {
            created: wide,
            count: 10
        })
    );
}

#[test]
fn the_checks_of_a_clean_create_refuse_in_a_fixed_order() {
    let before = PreMark::recorded(1795);
    let last = "<LASTVCHID>1733</LASTVCHID>";
    // The clean reply itself makes a span.
    assert!(PostSpan::after_clean_response(before, &reply("1", "10", "0", last), 10).is_ok());
    // An empty batch is refused before the reply's status is read.
    assert_eq!(
        PostSpan::after_clean_response(before, &reply("0", "10", "0", last), 0),
        Err(SpanRefusal::EmptyBatch)
    );
    // The status is read before the counters.
    assert_eq!(
        PostSpan::after_clean_response(before, &reply("0", "10", "1", last), 10),
        Err(SpanRefusal::StatusFailure)
    );
    // The counters are read before CREATED, and CREATED before LASTVCHID.
    assert_eq!(
        PostSpan::after_clean_response(before, &reply("1", "9", "1", last), 10),
        Err(SpanRefusal::CountersNotClean)
    );
    assert_eq!(
        PostSpan::after_clean_response(before, &reply("1", "10", "1", ""), 10),
        Err(SpanRefusal::CountersNotClean)
    );
    assert_eq!(
        PostSpan::after_clean_response(before, &reply("1", "9", "0", ""), 10),
        Err(SpanRefusal::CreatedNotCount {
            created: 9,
            count: 10
        })
    );
}

#[test]
fn a_mark_that_cannot_hold_the_span_is_refused_and_one_that_just_can_binds() {
    // 10 past u64::MAX - 9 is past the largest mark: nothing is wrapped or
    // saturated into a span.
    assert_eq!(
        PostSpan::after_clean_response(PreMark::recorded(u64::MAX - 9), &outcome(), 10),
        Err(SpanRefusal::MarkWentBack)
    );
    assert_eq!(
        PostSpan::after_clean_response(PreMark::recorded(u64::MAX), &outcome(), 10),
        Err(SpanRefusal::MarkWentBack)
    );
    // A POST whose mark after it was read cannot end past the largest mark
    // either: the step is then not the count.
    assert_eq!(
        PostSpan::after_clean_post(PreMark::recorded(u64::MAX - 9), u64::MAX, &outcome(), 10),
        Err(BindError::Refused(SpanRefusal::StepNotCreated {
            step: 9,
            created: 10
        }))
    );
    assert!(
        PostSpan::after_clean_post(PreMark::recorded(u64::MAX - 10), u64::MAX, &outcome(), 10)
            .is_ok()
    );
    // The span that ends on the largest mark is made, and binds the rows at
    // the top of the range.
    let top = u64::MAX - 10;
    let span = PostSpan::after_clean_response(PreMark::recorded(top), &outcome(), 10).unwrap();
    assert_eq!(
        span.alter_id_span(),
        crate::agent::AlterIdSpan {
            after: top,
            through: u64::MAX
        }
    );
    let mut read = span_read();
    for row in &mut read.rows {
        row.alter_id = Some(top + (row.alter_id.unwrap() - 1795));
    }
    let bound = bind_default(&span, &sent(), &read).unwrap();
    assert_eq!(bound.len(), 10);
    assert_eq!(bound[0].master_id, 1724);
    // One row off by one at the very top refuses at its position.
    row_mut(&mut read, u64::MAX - 1).alter_id = Some(u64::MAX);
    assert_eq!(
        refusal(bind_default(&span, &sent(), &read)),
        SpanRefusal::Position { position: 8 }
    );
}

#[test]
fn a_step_below_the_count_refuses_as_one_above_does() {
    assert_eq!(
        PostSpan::after_clean_post(PreMark::recorded(1795), 1804, &outcome(), 10),
        Err(BindError::Refused(SpanRefusal::StepNotCreated {
            step: 9,
            created: 10
        }))
    );
}

#[test]
fn the_vouchers_sent_and_the_rows_read_must_each_be_the_span_count() {
    let span = span();
    let read = span_read();
    let mut nine = sent();
    nine.pop();
    assert_eq!(
        refusal(bind_default(&span, &nine, &read)),
        SpanRefusal::SpanCount {
            expected: 10,
            observed: 10
        }
    );
    let mut eleven = sent();
    eleven.push(eleven[0].clone());
    assert_eq!(
        refusal(bind_default(&span, &eleven, &read)),
        SpanRefusal::SpanCount {
            expected: 10,
            observed: 10
        }
    );
    let mut more = span_read();
    more.rows.push(more.rows[0].clone());
    assert_eq!(
        refusal(bind_default(&span, &sent(), &more)),
        SpanRefusal::SpanCount {
            expected: 10,
            observed: 11
        }
    );
    // Both short: the count of the rows read is what is reported.
    let mut short = span_read();
    short.rows.pop();
    assert_eq!(
        refusal(bind_default(&span, &nine, &short)),
        SpanRefusal::SpanCount {
            expected: 10,
            observed: 9
        }
    );
    // The counts are checked before the first MasterID is worked out.
    let mut short = span_read();
    short.rows.remove(4);
    assert_eq!(
        refusal(bind_default(&span_ending_at(3), &sent(), &short)),
        SpanRefusal::SpanCount {
            expected: 10,
            observed: 9
        }
    );
}

#[test]
fn rows_are_taken_in_alter_id_order_whatever_their_order_or_masterids() {
    let span = span();
    let expected = bind_default(&span, &sent(), &span_read()).unwrap();
    let mut reversed = span_read();
    reversed.rows.reverse();
    assert_eq!(bind_default(&span, &sent(), &reversed).unwrap(), expected);
    // The AlterIDs of the rows at positions 1 and 2 keep their order while
    // their MasterIDs and GUIDs swap: the row at position 1 holds the wrong id.
    let mut swapped = span_read();
    let (master_id, guid) = {
        let second = row_mut(&mut swapped, 1797);
        (second.master_id.clone(), second.guid.clone())
    };
    let third = row_mut(&mut swapped, 1798);
    let (third_master_id, third_guid) = (third.master_id.clone(), third.guid.clone());
    third.master_id = master_id;
    third.guid = guid;
    let second = row_mut(&mut swapped, 1797);
    second.master_id = third_master_id;
    second.guid = third_guid;
    assert_eq!(
        refusal(bind_default(&span, &sent(), &swapped)),
        SpanRefusal::MasterIdNotInSequence { position: 1 }
    );
    // Two rows with one AlterID keep the order they were read in: the second
    // is the one out of place.
    let mut doubled = span_read();
    row_mut(&mut doubled, 1797).alter_id = Some(1796);
    assert_eq!(
        refusal(bind_default(&span, &sent(), &doubled)),
        SpanRefusal::Position { position: 1 }
    );
}

#[test]
fn the_company_guid_is_trimmed_and_lower_cased_before_it_is_compared() {
    let padded = format!("  {}\t", COMPANY_GUID.to_ascii_uppercase());
    let bound = bind(&span(), &padded, &sent(), &span_read(), &BTreeSet::new()).unwrap();
    assert_eq!(bound, bind_default(&span(), &sent(), &span_read()).unwrap());
    assert_eq!(bound[0].guid, format!("{COMPANY_GUID}-{:08x}", 1724));
}

#[test]
fn the_first_masterid_is_lastvchid_minus_count_plus_one_and_never_wraps() {
    // LASTVCHID 9 ends exactly the ten ids 0 to 9.
    let mut from_zero = span_read();
    renumber(&mut from_zero, 0);
    let bound = bind_default(&span_ending_at(9), &sent(), &from_zero).unwrap();
    assert_eq!(
        bound.iter().map(|id| id.master_id).collect::<Vec<_>>(),
        (0..10).collect::<Vec<u64>>()
    );
    assert_eq!(bound[0].guid, format!("{COMPANY_GUID}-{:08x}", 0));
    // LASTVCHID 3 is below the count: ids 0 to 9 are not saturated into a fit,
    // and ids that wrap around zero are not accepted either.
    assert_eq!(
        refusal(bind_default(&span_ending_at(3), &sent(), &from_zero)),
        SpanRefusal::MasterIdNotInSequence { position: 0 }
    );
    let mut wrapped = span_read();
    renumber(&mut wrapped, u64::MAX - 5);
    assert_eq!(
        refusal(bind_default(&span_ending_at(3), &sent(), &wrapped)),
        SpanRefusal::MasterIdNotInSequence { position: 0 }
    );
    // A LASTVCHID of u64::MAX cannot be added to: nothing binds.
    let mut last = span_read();
    renumber(&mut last, u64::MAX - 9);
    assert_eq!(
        refusal(bind_default(&span_ending_at(u64::MAX), &sent(), &last)),
        SpanRefusal::MasterIdNotInSequence { position: 0 }
    );
    // The largest LASTVCHID that can be added to binds the ids up to it.
    let mut top = span_read();
    renumber(&mut top, u64::MAX - 10);
    let bound = bind_default(&span_ending_at(u64::MAX - 1), &sent(), &top).unwrap();
    assert_eq!(bound[9].master_id, u64::MAX - 1);
    assert_eq!(bound[0].master_id, u64::MAX - 10);
}

#[test]
fn masterids_are_read_as_full_width_numbers_without_padding() {
    // Above 2^32: a narrower parse would refuse these.
    let wide = 1_u64 << 32;
    let mut read = span_read();
    renumber(&mut read, wide);
    let bound = bind_default(&span_ending_at(wide + 9), &sent(), &read).unwrap();
    assert_eq!(bound[0].master_id, wide);
    assert_eq!(bound[9].master_id, wide + 9);
    assert_eq!(bound[9].guid, format!("{COMPANY_GUID}-{:08x}", wide + 9));
}

#[test]
fn a_row_out_of_place_refuses_at_its_own_position() {
    let span = span();
    // The row at position 3 moved past the end of the span.
    let mut moved = span_read();
    row_mut(&mut moved, 1799).alter_id = Some(1900);
    assert_eq!(
        refusal(bind_default(&span, &sent(), &moved)),
        SpanRefusal::Position { position: 3 }
    );
    // Its MasterID with a space before it is not the number it spells.
    let mut padded = span_read();
    row_mut(&mut padded, 1799).master_id = Some(" 1727".into());
    assert_eq!(
        refusal(bind_default(&span, &sent(), &padded)),
        SpanRefusal::MasterIdNotInSequence { position: 3 }
    );
    // The fifth GUID in upper case is not the GUID that follows its MasterID.
    let mut upper = span_read();
    row_mut(&mut upper, 1800).guid = Some(format!(
        "{}-{:08x}",
        COMPANY_GUID.to_ascii_uppercase(),
        1728
    ));
    assert_eq!(
        refusal(bind_default(&span, &sent(), &upper)),
        SpanRefusal::GuidNotMasterId { position: 4 }
    );
}

#[test]
fn at_one_position_the_checks_run_content_then_effective_then_masterid_then_guid_then_reuse() {
    let span = span();
    let reused = BTreeSet::from([format!("{COMPANY_GUID}-{:08x}", 1726)]);
    let read_with = |cancelled: bool, wrong_master_id: bool, wrong_guid: bool| {
        let mut read = span_read();
        let row = row_mut(&mut read, 1798);
        if cancelled {
            row.cancelled = Some(true);
        }
        if wrong_master_id {
            row.master_id = Some("1".into());
        }
        if wrong_guid {
            row.guid = Some("wrong".into());
        }
        read
    };
    let mut changed = sent();
    changed[2].narration = Some("changed".into());
    assert_eq!(
        refusal(bind(
            &span,
            COMPANY_GUID,
            &changed,
            &read_with(true, true, true),
            &reused
        )),
        SpanRefusal::Content {
            position: 2,
            fields: vec!["narration"]
        }
    );
    let cases = [
        (
            (true, true, true),
            SpanRefusal::NotEffective { position: 2 },
        ),
        (
            (false, true, true),
            SpanRefusal::MasterIdNotInSequence { position: 2 },
        ),
        (
            (false, false, true),
            SpanRefusal::GuidNotMasterId { position: 2 },
        ),
        (
            (false, false, false),
            SpanRefusal::IdentityReused { position: 2 },
        ),
    ];
    for ((cancelled, master_id, guid), expected) in cases {
        assert_eq!(
            refusal(bind(
                &span,
                COMPANY_GUID,
                &sent(),
                &read_with(cancelled, master_id, guid),
                &reused
            )),
            expected
        );
    }
}

/// The first captured voucher bound after `edit_row` changes its row and
/// `edit_sent` what was sent as it.
fn first_voucher_refusal(
    edit_row: impl FnOnce(&mut ReadVoucher),
    edit_sent: impl FnOnce(&mut ImportVoucher),
) -> SpanRefusal {
    let mut read = span_read();
    edit_row(row_mut(&mut read, 1796));
    let mut vouchers = sent();
    edit_sent(&mut vouchers[0]);
    refusal(bind_default(&span(), &vouchers, &read))
}

fn first_differs_in(fields: Vec<&'static str>) -> SpanRefusal {
    SpanRefusal::Content {
        position: 0,
        fields,
    }
}

#[test]
fn each_field_that_differs_is_named_alone_and_a_repeated_entry_is_counted() {
    // A different DATE read back, the EFFECTIVEDATE still the day sent.
    assert_eq!(
        first_voucher_refusal(|row| row.date = other_day(), |_| {}),
        first_differs_in(vec!["date"])
    );
    // A different EFFECTIVEDATE, the DATE still the day sent.
    assert_eq!(
        first_voucher_refusal(|row| row.effective_date = Some(other_day()), |_| {}),
        first_differs_in(vec!["effective_date"])
    );
    // Another bank voucher type sent, or none read, or the same name in
    // another case.
    assert_eq!(
        first_voucher_refusal(|_| {}, |sent| sent.voucher_type = VoucherType::Receipt),
        first_differs_in(vec!["voucher_type"])
    );
    assert_eq!(
        first_voucher_refusal(|row| row.voucher_type = None, |_| {}),
        first_differs_in(vec!["voucher_type"])
    );
    assert_eq!(
        first_voucher_refusal(|row| row.voucher_type = Some("payment".into()), |_| {}),
        first_differs_in(vec!["voucher_type"])
    );
    // A repeated entry read that was sent once, and sent twice but read once.
    assert_eq!(
        first_voucher_refusal(
            |row| {
                let repeat = row.entries[0].clone();
                row.entries.push(repeat);
            },
            |_| {}
        ),
        first_differs_in(vec!["entries"])
    );
    assert_eq!(
        first_voucher_refusal(
            |_| {},
            |sent| {
                let repeat = sent.entries[0].clone();
                sent.entries.push(repeat);
            }
        ),
        first_differs_in(vec!["entries"])
    );
    // Every field differing: named in the order date, effective_date,
    // voucher_type, entries, narration.
    assert_eq!(
        first_voucher_refusal(
            |_| {},
            |sent| {
                sent.date = other_day();
                sent.voucher_type = VoucherType::Receipt;
                sent.entries[0].amount = ImportAmount::parse("1181.00").unwrap();
                sent.narration = Some("changed".into());
            }
        ),
        first_differs_in(vec![
            "date",
            "effective_date",
            "voucher_type",
            "entries",
            "narration"
        ])
    );
}

#[test]
fn a_journal_has_no_effective_date_to_differ_in_and_its_date_alone_is_named() {
    let outcome = bridge_tally_protocol::parse_import_outcome(&captured(JOURNAL_RESPONSE)).unwrap();
    let span = PostSpan::after_clean_post(PreMark::recorded(1805), 1810, &outcome, 5).unwrap();
    let read = ImportReadSource::admit(
        parse_import_voucher_rows(&captured(JOURNAL_SPAN_READ), COMPANY_GUID).unwrap(),
    )
    .unwrap();
    let other_day = other_day();
    // An EFFECTIVEDATE on a Journal row is no difference.
    let mut dated = read.clone();
    row_mut(&mut dated, 1806).effective_date = Some(other_day.clone());
    assert!(bind_default(&span, &sent_journals(), &dated).is_ok());
    // Its DATE is.
    let mut redated = read;
    row_mut(&mut redated, 1806).date = other_day;
    assert_eq!(
        refusal(bind_default(&span, &sent_journals(), &redated)),
        SpanRefusal::Content {
            position: 0,
            fields: vec!["date"]
        }
    );
}

#[test]
fn a_posted_voucher_identity_reads_back_only_its_three_fields() {
    let whole = r#"{"bridge_txn_id":"a","guid":"b","master_id":1}"#;
    assert_eq!(
        serde_json::from_str::<PostedVoucherIdentity>(whole).unwrap(),
        PostedVoucherIdentity {
            bridge_txn_id: "a".into(),
            guid: "b".into(),
            master_id: 1
        }
    );
    let extra = r#"{"bridge_txn_id":"a","guid":"b","master_id":1,"other":2}"#;
    assert!(serde_json::from_str::<PostedVoucherIdentity>(extra).is_err());
}
