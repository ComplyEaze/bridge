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
        amount: amount.into(),
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
        date: "2026-07-10".into(),
        voucher_type,
        narration: Some(narration.into()),
        reference: None,
        voucher_number: None,
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
        entry.amount = "99.01".into();
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
    ninth.effective_date = Some("20260711".into());
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
        date: "2026-07-11".into(),
        voucher_type: VoucherType::Journal,
        narration: Some(narration.into()),
        reference: None,
        voucher_number: None,
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
