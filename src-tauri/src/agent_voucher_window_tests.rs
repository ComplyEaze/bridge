#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::*;

// The four tests below moved here from `agent_import_tests.rs` with the #485
// helpers they pin, unchanged: the helpers now serve every bounded window read,
// not only the verification read.

#[test]
fn a_window_span_counts_both_endpoints() {
    let span = |from: &str, to: &str| window_span_days(&tally_date(from), &tally_date(to));
    assert_eq!(span("20260401", "20260401"), Some(1));
    assert_eq!(span("20260401", "20260402"), Some(2));
    assert_eq!(span("20260401", "20270331"), Some(365));
    // A leap year is counted by the calendar, not by arithmetic on months.
    assert_eq!(span("20240101", "20241231"), Some(366));
    // A window ending before it starts has no span.
    assert_eq!(span("20260402", "20260401"), None);
}

#[test]
fn a_span_already_known_to_fail_is_split_without_another_read() {
    // The point of carrying the failed span forward: a sibling branch of the same
    // size is split immediately rather than spending a full deadline to relearn it.
    assert!(must_split_before_reading(Some(91), Some(91)));
    assert!(must_split_before_reading(Some(182), Some(91)));
    // Smaller than anything known to fail: read it, do not pre-split.
    assert!(!must_split_before_reading(Some(45), Some(91)));
    // Nothing has failed yet, so nothing is known: always read.
    assert!(!must_split_before_reading(Some(365), None));
    // A single day is the floor. Pre-splitting it would spin, and refusing is the
    // reader's job, not this predicate's.
    assert!(!must_split_before_reading(Some(1), Some(1)));
    // An unparseable span falls through to reading rather than being treated as a
    // failure: this is an optimisation and must never decide correctness.
    assert!(!must_split_before_reading(None, Some(30)));
}

#[test]
fn splitting_a_verification_window_partitions_it_exactly() {
    // Every split must cover the original window once and only once. A gap drops
    // vouchers from an attribution check; an overlap double-counts them.
    for (from, to) in [
        ("20260401", "20270331"), // a full financial year
        ("20260401", "20260430"), // a month
        ("20260401", "20260402"), // two days: mid must equal start
        ("20260228", "20260301"), // across a month boundary
        ("20240228", "20240301"), // across a leap day
        ("20251231", "20260101"), // across a year boundary
    ] {
        let ((left_from, left_to), (right_from, right_to)) =
            split_verification_window(&tally_date(from), &tally_date(to))
                .expect("a multi-day window splits");
        assert_eq!(
            left_from.as_str(),
            from,
            "left half must start where the window did"
        );
        assert_eq!(
            right_to.as_str(),
            to,
            "right half must end where the window did"
        );
        // Contiguous, no gap and no overlap: the right half starts exactly the day
        // after the left half ends.
        assert_eq!(
            day(right_from.as_str()),
            day(left_to.as_str()) + chrono::Duration::days(1),
            "{from}..{to} split with a gap or an overlap"
        );
        // And it must actually shrink, or the splitter would never terminate.
        assert!(
            day(left_to.as_str()) < day(to),
            "left half did not shrink {from}..{to}"
        );
        assert!(
            day(right_from.as_str()) > day(from),
            "right half did not shrink {from}..{to}"
        );
    }
}

#[test]
fn a_single_day_verification_window_cannot_be_split() {
    // The recursion floor. Without it the splitter would spin on a day it cannot
    // read; with it, read_verification_window refuses rather than returning a
    // verification over an incomplete window.
    let split =
        |from: &str, to: &str| split_verification_window(&tally_date(from), &tally_date(to));
    assert_eq!(split("20260401", "20260401"), None);
    // A reversed window is refused rather than silently inverted.
    assert_eq!(split("20260430", "20260401"), None);
    // The last day Tally can name splits like any other window, without
    // stepping past it.
    assert_eq!(
        split("99991230", "99991231"),
        Some((
            (tally_date("99991230"), tally_date("99991230")),
            (tally_date("99991231"), tally_date("99991231"))
        ))
    );
}

// ---------------------------------------------------------------------------
// The pre-flight volume bound (protocol reference §11c).
// ---------------------------------------------------------------------------

use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

fn day(value: &str) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%Y%m%d").unwrap()
}

/// A census from `(day, voucher count)`, with AlterIDs numbered in date order.
fn census_of(days: &[(&str, u64)]) -> WindowCensus {
    let mut next = 0;
    WindowCensus::from_rows(days.iter().flat_map(|(date, n)| {
        (0..*n)
            .map(|_| {
                next += 1;
                (day(date), next)
            })
            .collect::<Vec<_>>()
    }))
}

fn plan(
    from: &str,
    to: &str,
    census: &WindowCensus,
    bytes_per_voucher: u64,
    budget: u64,
    max_reads: usize,
) -> Result<Vec<PlannedRead>, PlanRefusal> {
    plan_window_reads(
        day(from),
        day(to),
        census,
        None,
        census.max_alter_id(),
        bytes_per_voucher,
        budget,
        max_reads,
    )
}

/// Every plan must cover its window exactly: consecutive date ranges with no
/// gap or overlap, and a day read in spans covered by spans that tile
/// `(0, ceiling]` for that day. A gap drops vouchers; an overlap reads them twice.
fn assert_tiles(plan: &[PlannedRead], from: &str, to: &str, ceiling: u64) {
    assert_eq!(plan.first().unwrap().from, day(from), "plan starts late");
    assert_eq!(plan.last().unwrap().to, day(to), "plan ends early");
    let mut index = 0;
    while index < plan.len() {
        let read = plan[index];
        let mut end = read.to;
        if let Some(span) = read.span {
            assert_eq!(read.from, read.to, "a span covers one day");
            assert_eq!(span.after, 0, "a day's spans start at the bottom");
            let mut through = span.through;
            while index + 1 < plan.len()
                && plan[index + 1].span.is_some()
                && plan[index + 1].from == read.from
            {
                index += 1;
                let next = plan[index].span.unwrap();
                assert_eq!(next.after, through, "gap or overlap in {:?}", read.from);
                through = next.through;
            }
            assert_eq!(through, ceiling, "a day's spans stop short of the ceiling");
            end = read.from;
        }
        if index + 1 < plan.len() {
            assert_eq!(
                plan[index + 1].from,
                end.succ_opt().unwrap(),
                "gap or overlap after {end}"
            );
        }
        index += 1;
    }
}

#[test]
fn a_window_predicted_within_budget_is_planned_as_itself() {
    // 30 vouchers at 1 KiB against a 64 KiB budget: one read, and that read is
    // the caller's own window — which is how a small book keeps its request.
    let census = census_of(&[("20260402", 10), ("20260415", 20)]);
    let reads = plan("20260401", "20260430", &census, 1024, 64 * 1024, 8).unwrap();
    assert_eq!(
        reads,
        [PlannedRead {
            from: day("20260401"),
            to: day("20260430"),
            span: None,
            vouchers: 30
        }]
    );
    // An empty window is one read of zero vouchers, not no read: emptiness
    // still has to be observed.
    let reads = plan("20260401", "20260430", &census_of(&[]), 1024, 64 * 1024, 8).unwrap();
    assert_eq!(reads.len(), 1);
    assert_eq!(reads[0].vouchers, 0);
}

#[test]
fn a_window_over_budget_is_divided_greedily_and_exactly() {
    // Capacity is 10 vouchers a read. Days of 6, 4, 7, 2, 9 pack as
    // [6+4], [7+2], [9]; the empty days between them ride along.
    let census = census_of(&[
        ("20260403", 6),
        ("20260404", 4),
        ("20260410", 7),
        ("20260411", 2),
        ("20260420", 9),
    ]);
    let reads = plan("20260401", "20260430", &census, 100, 1000, 8).unwrap();
    assert_tiles(&reads, "20260401", "20260430", census.max_alter_id());
    assert_eq!(
        reads.iter().map(|read| read.vouchers).collect::<Vec<_>>(),
        [10, 9, 9]
    );
    assert_eq!(reads[0].to, day("20260409"));
    assert_eq!(reads[1].to, day("20260419"));
    assert!(reads.iter().all(|read| read.span.is_none()));
    // Exactly at capacity still fits; one more voucher does not.
    let exact = census_of(&[("20260401", 5), ("20260402", 5)]);
    assert_eq!(
        plan("20260401", "20260402", &exact, 100, 1000, 8)
            .unwrap()
            .len(),
        1
    );
    let over = census_of(&[("20260401", 5), ("20260402", 6)]);
    let reads = plan("20260401", "20260402", &over, 100, 1000, 8).unwrap();
    assert_tiles(&reads, "20260401", "20260402", over.max_alter_id());
    assert_eq!(reads.len(), 2);
}

#[test]
fn a_day_too_heavy_for_one_read_is_divided_by_alterid_not_refused() {
    // Capacity 10. The 25-voucher day is read alone, in AlterID spans of that
    // day holding 10, 10 and 5; the days around it are read by date.
    let census = census_of(&[("20260401", 3), ("20260415", 25), ("20260420", 2)]);
    let ceiling = 40;
    let reads = plan_window_reads(
        day("20260401"),
        day("20260430"),
        &census,
        None,
        ceiling,
        100,
        1000,
        8,
    )
    .unwrap();
    assert_tiles(&reads, "20260401", "20260430", ceiling);
    let spans = reads
        .iter()
        .filter(|read| read.span.is_some())
        .collect::<Vec<_>>();
    assert_eq!(
        spans.iter().map(|read| read.vouchers).collect::<Vec<_>>(),
        [10, 10, 5]
    );
    assert!(spans.iter().all(|read| read.from == day("20260415")));
    // Each span selects exactly the census's vouchers it was planned for.
    for read in &spans {
        let span = read.span.unwrap();
        let held = census.ids_on(day("20260415"), Some(span)).len() as u64;
        assert_eq!(held, read.vouchers);
        assert!(held * 100 <= 1000);
    }
    // The last span reaches the ceiling: an AlterID above the census's highest
    // is still inside some span while the mark is unchanged.
    assert_eq!(spans.last().unwrap().span.unwrap().through, ceiling);
}

/// #861: a planning day becomes a request's date only through `stamp`, which
/// refuses a day no `YYYYMMDD` date can name by the code `TallyDate::next_day`
/// uses, rather than rendering it or panicking.
#[test]
fn a_planning_day_is_stamped_as_a_tally_date_or_refused_as_overflow() {
    for date in ["20260401", "20240229", "00010101", "99991231"] {
        assert_eq!(stamp(day(date)).unwrap(), tally_date(date));
    }
    assert!(matches!(
        tally_date("99991231").next_day(),
        Err(bridge_tally_core::TallyError::InvalidData { code }) if code == TALLY_DATE_OVERFLOW
    ));
    for beyond in [
        NaiveDate::from_ymd_opt(10_000, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(0, 12, 31).unwrap(),
        NaiveDate::from_ymd_opt(-1, 6, 15).unwrap(),
    ] {
        assert_eq!(
            stamp(beyond).unwrap_err().code,
            TALLY_DATE_OVERFLOW,
            "{beyond}"
        );
    }
}

/// #861: a window ending on the last day Tally can name is planned and stamped
/// without stepping past it, through the AlterID-span path that advances the
/// planner to the following day. A planned read that did reach past it is
/// refused by its typed code when the plan becomes parts, so no part, and
/// therefore no request, exists to render.
#[test]
fn a_window_ending_on_the_last_tally_day_never_yields_a_part_past_it() {
    // Capacity 10: the 25-voucher last day is read alone, in AlterID spans.
    let census = census_of(&[("99991230", 3), ("99991231", 25)]);
    let reads = plan_window_reads(
        day("99991230"),
        day("99991231"),
        &census,
        None,
        40,
        100,
        1000,
        8,
    )
    .unwrap();
    assert_tiles(&reads, "99991230", "99991231", 40);
    let parts = stack_of(&reads).unwrap();
    assert_eq!(parts.len(), reads.len());
    assert!(parts.iter().all(|part| part.to <= tally_date("99991231")));
    let past = PlannedRead {
        from: day("99991231"),
        to: NaiveDate::from_ymd_opt(10_000, 1, 1).unwrap(),
        span: None,
        vouchers: 0,
    };
    assert_eq!(past.part().unwrap_err().code, TALLY_DATE_OVERFLOW);
    let mut reaching_past = reads;
    reaching_past.push(past);
    assert_eq!(
        stack_of(&reaching_past).unwrap_err().code,
        TALLY_DATE_OVERFLOW
    );
}

#[test]
fn only_a_single_voucher_over_budget_is_refused() {
    let one = census_of(&[("20260401", 1)]);
    let refusal = plan("20260401", "20260401", &one, 2000, 1000, 8).unwrap_err();
    assert_eq!(
        refusal,
        PlanRefusal::VoucherOverBudget {
            day: day("20260401")
        }
    );
    assert_eq!(refusal.code(), "voucher_window_part_over_budget");
    // At exactly the budget one voucher is one read.
    assert_eq!(
        plan("20260401", "20260401", &one, 1000, 1000, 8)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn a_window_needing_more_reads_than_allowed_is_refused() {
    let census = census_of(&[("20260401", 10), ("20260402", 10), ("20260403", 10)]);
    let refusal = plan("20260401", "20260403", &census, 100, 1000, 2).unwrap_err();
    assert_eq!(
        refusal,
        PlanRefusal::TooManyReads {
            reads: 3,
            allowed: 2
        }
    );
    assert_eq!(refusal.code(), "voucher_window_too_many_reads");
}

#[test]
fn a_partly_read_day_resumes_above_the_last_alterid_read() {
    // Day one holds AlterIDs 1..=5, day two 6..=7. After a span through 2 on day
    // one, the rest of day one is read from 2 upward, never from 0 again.
    let census = census_of(&[("20260401", 5), ("20260402", 2)]);
    let reads = plan_window_reads(
        day("20260401"),
        day("20260402"),
        &census,
        Some(2),
        7,
        100,
        1000,
        8,
    )
    .unwrap();
    assert_eq!(
        reads,
        [
            PlannedRead {
                from: day("20260401"),
                to: day("20260401"),
                span: Some(AlterIdSpan {
                    after: 2,
                    through: 7
                }),
                vouchers: 3
            },
            PlannedRead {
                from: day("20260402"),
                to: day("20260402"),
                span: None,
                vouchers: 2
            }
        ]
    );
}

/// A year of the heaviest book measured (§11c): 16,367 vouchers, spread evenly
/// at about 45 a day, and one day of 450 invoices imported in bulk.
fn heavy_book() -> WindowCensus {
    let mut rows = Vec::new();
    let mut date = day("20250401");
    let mut next = 0_u64;
    let mut left = 16_367_u64 - 450;
    while left > 0 {
        let today = left.min(45);
        for _ in 0..today {
            next += 1;
            rows.push((date, next));
        }
        left -= today;
        date = date.succ_opt().unwrap();
    }
    for _ in 0..450 {
        next += 1;
        rows.push((day("20250615"), next));
    }
    WindowCensus::from_rows(rows)
}

fn assert_heavy_book_divides(from: &str, to: &str, measured_wire: u64) -> usize {
    let census = heavy_book();
    let reads = plan_window_reads(
        day(from),
        day(to),
        &census,
        None,
        census.max_alter_id(),
        measured_wire,
        WINDOW_READ_BUDGET_BYTES,
        MAX_PLANNED_READS,
    )
    .unwrap();
    assert_tiles(&reads, from, to, census.max_alter_id());
    for read in &reads {
        assert!(
            read.vouchers * measured_wire <= WINDOW_READ_BUDGET_BYTES,
            "{read:?}"
        );
    }
    reads.len()
}

#[test]
fn the_heavy_book_divides_at_its_measured_named_field_cost() {
    // 35 KB of UTF-8 per voucher is 70 KB on the wire. A year divides, and the
    // day of 450 invoices is read in AlterID spans rather than refused — a bulk
    // import verification of one day's invoices must still be possible.
    let year = assert_heavy_book_divides("20250401", "20260331", 70_000);
    assert!(year > 1 && year <= MAX_PLANNED_READS);
    let bulk_day = assert_heavy_book_divides("20250615", "20250615", 70_000);
    assert!(bulk_day >= 2, "450 invoices at 70 KB do not fit one read");
}

#[test]
fn the_heavy_book_divides_at_its_measured_wildcard_cost() {
    // ~128 KB of UTF-8 per voucher with the entry wildcard: 256 KB on the wire,
    // 65 vouchers a read. A month divides; ordinary days of 45 are read whole
    // by date, and the bulk day in spans — neither is refused.
    let month = assert_heavy_book_divides("20250601", "20250630", 256_000);
    assert!(month > 1);
    assert!(assert_heavy_book_divides("20250602", "20250602", 256_000) == 1);
    assert!(assert_heavy_book_divides("20250615", "20250615", 256_000) >= 7);
    // A whole year of the wildcard is more reads than one call may spend: that
    // is refused by name, before any data read, rather than sent.
    let census = heavy_book();
    assert!(matches!(
        plan_window_reads(
            day("20250401"),
            day("20260331"),
            &census,
            None,
            census.max_alter_id(),
            256_000,
            WINDOW_READ_BUDGET_BYTES,
            MAX_PLANNED_READS,
        ),
        Err(PlanRefusal::TooManyReads { .. })
    ));
}

#[test]
fn the_conservative_defaults_stay_above_every_measured_per_voucher_cost() {
    for shape in [
        VoucherReadShape::ImportVerification,
        VoucherReadShape::Movement,
    ] {
        assert!(shape.default_wire_bytes_per_voucher() > 2 * 35_000);
    }
    assert!(VoucherReadShape::EntryWildcard.default_wire_bytes_per_voucher() > 2 * 128_000);
    // The budget is the one margin, and it sits well below the cap.
    assert!(WINDOW_READ_BUDGET_BYTES * 2 <= bridge_tally_transport::XML_RESPONSE_MAX_BYTES as u64);
}

#[test]
fn a_part_tally_cannot_serve_is_halved_by_date_then_by_alterid() {
    let census = census_of(&[("20260401", 4), ("20260402", 1)]);
    let part = |from: &str, to: &str, span| WindowPart {
        from: tally_date(from),
        to: tally_date(to),
        span,
    };
    let halve_part =
        |part: &WindowPart, census, ceiling| halve_part(part, census, ceiling).unwrap();
    // A date range halves by date.
    assert_eq!(
        halve_part(&part("20260401", "20260402", None), Some(&census), 5),
        Some((
            part("20260401", "20260401", None),
            part("20260402", "20260402", None)
        ))
    );
    // One day halves by its counted AlterIDs, the right half reaching the ceiling.
    assert_eq!(
        halve_part(&part("20260401", "20260401", None), Some(&census), 9),
        Some((
            part(
                "20260401",
                "20260401",
                Some(AlterIdSpan {
                    after: 0,
                    through: 2
                })
            ),
            part(
                "20260401",
                "20260401",
                Some(AlterIdSpan {
                    after: 2,
                    through: 9
                })
            ),
        ))
    );
    // A span halves inside itself.
    assert_eq!(
        halve_part(
            &part(
                "20260401",
                "20260401",
                Some(AlterIdSpan {
                    after: 2,
                    through: 9
                })
            ),
            Some(&census),
            9
        ),
        Some((
            part(
                "20260401",
                "20260401",
                Some(AlterIdSpan {
                    after: 2,
                    through: 3
                })
            ),
            part(
                "20260401",
                "20260401",
                Some(AlterIdSpan {
                    after: 3,
                    through: 9
                })
            ),
        ))
    );
    // One voucher cannot be divided; nor can a day with no census to divide by.
    assert_eq!(
        halve_part(&part("20260402", "20260402", None), Some(&census), 5),
        None
    );
    assert_eq!(
        halve_part(&part("20260401", "20260401", None), None, 5),
        None
    );
}

fn captured_utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn three_vouchers() -> String {
    captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ))
}

fn empty_collection() -> String {
    captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-empty-collection.utf16le.xml"
    ))
}

/// The captured three-voucher response with `keep` of its vouchers, the others
/// removed from the captured bytes in memory (the first ones go first).
fn vouchers_kept(keep: usize) -> String {
    let mut xml = three_vouchers();
    for _ in keep..3 {
        let start = xml.find("<VOUCHER ").unwrap();
        let end = start + xml[start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
        xml.replace_range(start..end, "");
    }
    assert_eq!(xml.matches("<VOUCHER ").count(), keep);
    xml
}

/// `xml` with its vouchers, in order, given the AlterIDs and dates of `ids`:
/// the GUID, `REMOTEID`, `ALTERID`, `MASTERID` and `DATE` of each are rewritten
/// together and nothing else changes, so a response keeps its captured size
/// while describing exactly the vouchers a test's census counted.
fn relabelled(xml: &str, ids: &[(u64, &str)]) -> String {
    let mut out = String::new();
    let mut rest = xml;
    for (alter_id, date) in ids {
        let start = rest.find("<VOUCHER ").expect("a voucher to relabel");
        let end = start + rest[start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
        out.push_str(&rest[..start]);
        let voucher = &rest[start..end];
        let suffix = |value: u64| format!("{GUID}-{value:08x}");
        let old = regex_lite_capture(voucher, "<GUID>", "</GUID>");
        let old_alter = regex_lite_capture(voucher, "<ALTERID TYPE=\"Number\">", "</ALTERID>");
        let old_master = regex_lite_capture(voucher, "<MASTERID TYPE=\"Number\">", "</MASTERID>");
        let old_date = regex_lite_capture(voucher, "<DATE TYPE=\"Date\">", "</DATE>");
        out.push_str(
            &voucher
                .replace(&old, &suffix(*alter_id))
                .replace(
                    &format!("<ALTERID TYPE=\"Number\">{old_alter}</ALTERID>"),
                    &format!("<ALTERID TYPE=\"Number\"> {alter_id}</ALTERID>"),
                )
                .replace(
                    &format!("<MASTERID TYPE=\"Number\">{old_master}</MASTERID>"),
                    &format!("<MASTERID TYPE=\"Number\"> {alter_id}</MASTERID>"),
                )
                .replace(
                    &format!("<DATE TYPE=\"Date\">{old_date}</DATE>"),
                    &format!("<DATE TYPE=\"Date\">{date}</DATE>"),
                ),
        );
        rest = &rest[end..];
    }
    assert!(!rest.contains("<VOUCHER "), "every voucher is relabelled");
    out.push_str(rest);
    out
}

fn regex_lite_capture(text: &str, open: &str, close: &str) -> String {
    let start = text.find(open).unwrap_or_else(|| panic!("{open} present")) + open.len();
    let end = start + text[start..].find(close).unwrap();
    text[start..end].to_string()
}

#[test]
fn a_relabelled_response_describes_the_vouchers_it_names() {
    let xml = relabelled(&vouchers_kept(2), &[(7, "20260803"), (9, "20260804")]);
    let rows = parse_agent_rows(&xml, GUID).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["alter_id"], 7);
    assert_eq!(rows[1]["date"], "20260804");
    assert_eq!(rows[1].window_master_id(), Ok(Some(9)));
    let census = parse_voucher_census(&xml, (day("20260803"), day("20260804")), None).unwrap();
    assert_eq!(census[0].guid, rows[0]["guid"].as_str().unwrap());
}

#[test]
fn the_census_reads_a_captured_voucher_row_and_ignores_cmpinfo() {
    let rows =
        parse_voucher_census(&three_vouchers(), (day("20260801"), day("20260802")), None).unwrap();
    assert!(rows.iter().all(|row| !row.guid.is_empty()));
    assert_eq!(
        rows.iter()
            .map(|row| (row.day, row.alter_id))
            .collect::<Vec<_>>(),
        [
            (day("20260801"), 1),
            (day("20260801"), 2),
            (day("20260801"), 3)
        ]
    );
    // §12.7: an empty response's CMPINFO carries a bare <VOUCHER>0</VOUCHER>.
    let empty = empty_collection();
    assert!(empty.contains("<VOUCHER>0</VOUCHER>"));
    assert_eq!(
        parse_voucher_census(&empty, (day("20260801"), day("20260802")), None),
        Ok(vec![])
    );
}

#[test]
fn a_census_that_does_not_describe_what_was_asked_is_refused() {
    let xml = three_vouchers();
    assert_eq!(
        parse_voucher_census(&xml, (day("20260802"), day("20260803")), None),
        Err("window_not_honoured".to_string())
    );
    assert_eq!(
        parse_voucher_census(
            &xml,
            (day("20260801"), day("20260801")),
            Some(AlterIdSpan {
                after: 1,
                through: 3
            })
        ),
        Err("window_not_honoured".to_string())
    );
    let undated = xml.replacen("<DATE TYPE=\"Date\">20260801</DATE>", "", 1);
    assert_ne!(undated, xml);
    assert_eq!(
        parse_voucher_census(&undated, (day("20260801"), day("20260801")), None),
        Err("agent_read_protocol_invalid".to_string())
    );
    // A census row without its GUID cannot admit the part it counts.
    let first_guid = parse_voucher_census(&xml, (day("20260801"), day("20260801")), None).unwrap()
        [0]
    .guid
    .clone();
    let unidentified = xml.replacen(&format!("<GUID>{first_guid}</GUID>"), "", 1);
    assert_ne!(unidentified, xml);
    assert_eq!(
        parse_voucher_census(&unidentified, (day("20260801"), day("20260801")), None),
        Err("agent_read_protocol_invalid".to_string())
    );
}

#[test]
fn an_unobservable_high_water_mark_is_not_read_as_an_empty_book() {
    let guid = "61c6de69-1748-461c-ad3f-162cb949df9f";
    assert_eq!(
        company_marks(&mark_xml(guid, "<ALTVCHID>42</ALTVCHID>"), guid),
        Ok(CompanyMarks {
            vouchers: 42,
            masters: 7
        })
    );
    // Tally omits ALTVCHID for a company that has never held a voucher.
    assert_eq!(
        company_marks(&mark_xml(guid, ""), guid),
        Ok(CompanyMarks {
            vouchers: 0,
            masters: 7
        })
    );
    assert_eq!(
        company_marks(&mark_xml(guid, "<ALTVCHID>x</ALTVCHID>"), guid),
        Err("voucher_checkpoint_invalid".to_string())
    );
    assert_eq!(
        company_marks(&mark_xml(guid, "<ALTVCHID>42</ALTVCHID>"), "another-guid"),
        Err("company_high_water_identity_absent".to_string())
    );
    // Only a row whose master mark was itself observed is that empty book: a row
    // carrying neither mark is unobservable, not empty.
    let neither = mark_xml(guid, "").replace("<ALTMSTID>7</ALTMSTID>", "");
    assert_eq!(
        company_marks(&neither, guid),
        Err("master_checkpoint_not_observed".to_string())
    );
}

#[test]
fn an_undivided_read_is_byte_identical_to_the_request_before_the_bound() {
    let (company, from, to) = (
        "Synthetic Book",
        &tally_date("20260401"),
        &tally_date("20260430"),
    );
    assert_eq!(
        VoucherReadShape::EntryWildcard
            .render(company, from, to, None)
            .unwrap(),
        render_agent_vouchers(company, from, to, None).unwrap()
    );
    assert_eq!(
        VoucherReadShape::Movement
            .render(company, from, to, None)
            .unwrap(),
        render_agent_movement_vouchers(company, from, to).unwrap()
    );
    assert_eq!(
        VoucherReadShape::ImportVerification
            .render(company, from, to, None)
            .unwrap(),
        super::super::agent_import::render_import_verification_read(company, from, to)
    );
    assert_eq!(
        render_agent_vouchers_in_span(company, from, to, None).unwrap(),
        render_agent_vouchers(company, from, to, None).unwrap()
    );
    // A span part differs by exactly its span clause, inside the date formula.
    let span = AlterIdSpan {
        after: 7,
        through: 9,
    };
    for shape in [
        VoucherReadShape::EntryWildcard,
        VoucherReadShape::ClassEntryWildcard,
        VoucherReadShape::Movement,
        VoucherReadShape::ImportVerification,
    ] {
        let whole = shape.render(company, from, to, None).unwrap();
        let part = shape.render(company, from, to, Some(span)).unwrap();
        let clause = " AND $AlterID &gt; 7 AND $AlterID &lt;= 9";
        assert_eq!(part.replacen(clause, "", 1), whole, "{shape:?}");
        assert!(
            part[..part.find("</SYSTEM>").unwrap()].ends_with(clause),
            "{shape:?}"
        );
    }
}

#[test]
fn the_census_request_is_an_admitted_light_collection_export() {
    let dated = render_agent_voucher_census(
        "Synthetic & Book",
        &tally_date("20260401"),
        &tally_date("20260430"),
        None,
    )
    .unwrap();
    assert!(crate::tally::agent_read_request::AgentReadRequest::parse(dated.clone()).is_ok());
    assert!(dated.contains("<FETCH>GUID,ALTERID,DATE</FETCH>"));
    assert!(dated.contains("Synthetic &amp; Book"));
    assert!(dated
        .contains("$Date &gt;= $$Date:\"20260401\" AND $Date &lt;= $$Date:\"20260430\"</SYSTEM>"));
    let spanned = render_agent_voucher_census(
        "Synthetic & Book",
        &tally_date("20260401"),
        &tally_date("20260401"),
        Some(AlterIdSpan {
            after: 0,
            through: 4096,
        }),
    )
    .unwrap();
    assert!(spanned.contains("AND $AlterID &gt; 0 AND $AlterID &lt;= 4096</SYSTEM>"));
}

// --- Through the simulator --------------------------------------------------

const GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";

fn mark_xml(guid: &str, altvchid: &str) -> String {
    format!("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY><GUID>{guid}</GUID>{altvchid}<ALTMSTID>7</ALTMSTID></COMPANY></COLLECTION></DATA></BODY></ENVELOPE>")
}

/// The marks `mark(value)` serves: the master mark of `mark_xml` is 7.
fn marks_of(vouchers: u64) -> CompanyMarks {
    CompanyMarks {
        vouchers,
        masters: 7,
    }
}

/// A high-water response carrying both marks.
fn marks_plan(vouchers: u64, masters: u64) -> ScenarioPlan {
    xml_plan(
        mark_xml(GUID, &format!("<ALTVCHID>{vouchers}</ALTVCHID>")).replace(
            "<ALTMSTID>7</ALTMSTID>",
            &format!("<ALTMSTID>{masters}</ALTMSTID>"),
        ),
    )
}

fn mark(value: u64) -> ScenarioPlan {
    xml_plan(mark_xml(GUID, &format!("<ALTVCHID>{value}</ALTVCHID>")))
}

fn company_plan() -> ScenarioPlan {
    xml_plan(captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-companies.utf16le.xml"
    )))
}

fn xml_plan(xml: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(xml))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn status_plan() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
        .with_framing(ResponseFraming::ContentLength)
}

/// The six legs of one paired, identity-bracketed agent read.
fn paired(body: &ScenarioPlan) -> Vec<ScenarioPlan> {
    vec![
        company_plan(),
        body.clone(),
        status_plan(),
        body.clone(),
        status_plan(),
        company_plan(),
    ]
}

/// The two legs a read Tally will not serve costs: the identity bracket, then
/// a response declaring a length over the transport cap.
fn oversized() -> Vec<ScenarioPlan> {
    vec![
        company_plan(),
        xml_plan(three_vouchers()).with_framing(ResponseFraming::DeclaredContentLength {
            bytes: bridge_tally_transport::XML_RESPONSE_MAX_BYTES + 1,
        }),
    ]
}

fn wire_len(plan: &ScenarioPlan) -> u64 {
    tally_protocol_simulator::encode(&plan.fixture.body(), plan.encoding).len() as u64
}

fn request_sha(xml: &str) -> String {
    sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        xml,
    ))
}

/// A window date, as `normalized_date` hands it to the window layer.
fn tally_date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

fn identity() -> VerifiedCompanyIdentity {
    let company = company_plan().fixture.body().into_owned();
    let companies = bridge_tally_protocol::parse_companies_from_collection(&company).unwrap();
    let observed = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(GUID))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        observed.name.clone(),
        observed.guid.clone().unwrap(),
        observed.company_number.clone().unwrap(),
        observed.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

fn company() -> String {
    identity().display_name().to_string()
}

fn server_at(address: std::net::SocketAddr, directory: &std::path::Path) -> Server {
    server_with(address, directory, Redaction::None)
}

fn server_with(
    address: std::net::SocketAddr,
    directory: &std::path::Path,
    redaction: Redaction,
) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: address.ip().to_string(),
            port: address.port(),
        },
        data_dir: directory.to_path_buf(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

/// Limits under which, measured, the captured one-voucher response fits three
/// vouchers a read — and under which, before anything is measured, the default
/// fits only one.
fn three_a_read() -> WindowReadLimits {
    let budget = 3 * wire_len(&xml_plan(vouchers_kept(1)));
    WindowReadLimits {
        budget_bytes: budget,
        default_bytes_per_voucher: budget,
        max_reads: MAX_PLANNED_READS,
        small_books: SmallBooks::Skip,
    }
}

async fn read_window(
    plans: Vec<ScenarioPlan>,
    (from, to): (&str, &str),
    shape: VoucherReadShape,
    source: WindowPlanSource,
    limits: WindowReadLimits,
) -> (
    Result<WindowReadOutcome<Value>, ToolFailure>,
    Vec<tally_protocol_simulator::ObservedRequest>,
) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let identity = identity();
    let outcome = server
        .read_voucher_window(
            &identity,
            identity.display_name(),
            &tally_date(from),
            &tally_date(to),
            shape,
            source,
            limits,
            |xml| parse_agent_rows(xml, GUID),
        )
        .await;
    (outcome, simulator.finish().unwrap())
}

/// #861: a window date that reaches the window layer as text (a bill date, a
/// stored batch's window) and is not a Tally date is refused by its typed code
/// where it is parsed, before any read. A tool's own date is parsed earlier, by
/// `normalized_date` (`a_tool_date_that_is_not_a_tally_date_is_refused_at_the_boundary`).
#[test]
fn a_window_date_that_is_not_a_tally_date_is_refused_where_it_is_parsed() {
    for text in ["20261301", "2026-08-02", "20260229"] {
        assert_eq!(
            parse_window_date(text).err().map(|failure| failure.code),
            Some("invalid_date_range".to_string()),
            "{text}"
        );
    }
    assert_eq!(
        parse_window_date("20260801").ok(),
        Some(tally_date("20260801"))
    );
}

/// The request bodies of the data POSTs among `observed`, by the six-leg pattern:
/// only a leg at offset 1 of a paired read.
fn assert_requests(
    observed: &[tally_protocol_simulator::ObservedRequest],
    at: &[usize],
    expected: &[String],
) {
    assert_eq!(at.len(), expected.len());
    for (leg, request) in at.iter().zip(expected) {
        assert_eq!(
            &observed[*leg].request_body_sha256,
            &request_sha(request),
            "leg {leg}"
        );
    }
}

fn part(from: &str, to: &str, span: Option<AlterIdSpan>) -> WindowPart {
    WindowPart {
        from: tally_date(from),
        to: tally_date(to),
        span,
    }
}

#[tokio::test]
async fn the_first_part_measures_the_book_and_the_rest_of_its_day_is_read_above_it() {
    // Default capacity 1: the heavy day is planned in one-voucher spans. The
    // first span measures one voucher, capacity becomes 2 (the floor at half the
    // default binds here), and the rest of that day is re-planned from above
    // the AlterID already read, then day two whole.
    let limits = three_a_read();
    let census = WindowCensus::from_rows([
        (day("20260801"), 1),
        (day("20260801"), 2),
        (day("20260801"), 3),
        (day("20260802"), 4),
        (day("20260802"), 5),
    ]);
    let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(2, "20260801"), (3, "20260801")],
    ))));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(4, "20260802"), (5, "20260802")],
    ))));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census),
        limits,
    )
    .await;
    let outcome = outcome.unwrap();
    assert_eq!(outcome.rows.len(), 5);
    let expected = [
        part(
            "20260801",
            "20260801",
            Some(AlterIdSpan {
                after: 0,
                through: 1,
            }),
        ),
        part(
            "20260801",
            "20260801",
            Some(AlterIdSpan {
                after: 1,
                through: 5,
            }),
        ),
        part("20260802", "20260802", None),
    ];
    assert_eq!(outcome.reads, expected);
    let shape = VoucherReadShape::EntryWildcard;
    assert_requests(
        &observed,
        &[1, 7, 13],
        &expected
            .iter()
            .map(|read| {
                shape
                    .render(&company(), &read.from, &read.to, read.span)
                    .unwrap()
            })
            .collect::<Vec<_>>(),
    );
    assert_eq!(observed.len(), 18);
}

#[tokio::test]
async fn a_replan_after_a_whole_day_reads_the_days_after_it() {
    // Day one fits the default; day two does not. Measuring day one raises
    // capacity to 2, and the re-plan must begin on day two — not re-read day one.
    let limits = three_a_read();
    let census = WindowCensus::from_rows([
        (day("20260801"), 1),
        (day("20260802"), 2),
        (day("20260802"), 3),
    ]);
    let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(2, "20260802"), (3, "20260802")],
    ))));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census),
        limits,
    )
    .await;
    let outcome = outcome.unwrap();
    assert_eq!(
        outcome.reads,
        [
            part("20260801", "20260801", None),
            part("20260802", "20260802", None)
        ]
    );
    assert_eq!(outcome.rows.len(), 3);
    assert_eq!(observed.len(), 12);
}

#[tokio::test]
async fn a_single_voucher_over_budget_is_refused_before_any_read() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(listener.local_addr().unwrap(), directory.path());
    let identity = identity();
    let failure = server
        .read_voucher_window(
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Counted(WindowCensus::from_rows([(day("20260801"), 1)])),
            WindowReadLimits {
                budget_bytes: 1000,
                default_bytes_per_voucher: 1001,
                max_reads: MAX_PLANNED_READS,
                small_books: SmallBooks::Skip,
            },
            |xml| parse_agent_rows(xml, GUID),
        )
        .await
        .err()
        .expect("refused");
    assert_eq!(failure.code, "voucher_window_part_over_budget");
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

#[tokio::test]
async fn an_unmeasured_book_is_planned_at_the_default_and_an_omitted_voucher_is_refused() {
    // Before anything is measured the plan is at the default: one voucher a
    // read, so the first request is the span holding the first counted
    // voucher alone — never one read of all three on the strength of nothing.
    //
    // #520 negative control (omission): Tally answers that span with no
    // vouchers although the census counted one in it. Before partition
    // admission the empty part was accepted, nothing was measured, and the read
    // went on to return two of the window's three vouchers as complete.
    let limits = three_a_read();
    let census = WindowCensus::from_rows([
        (day("20260801"), 1),
        (day("20260801"), 2),
        (day("20260801"), 3),
    ]);
    let (outcome, observed) = read_window(
        paired(&xml_plan(empty_collection())),
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census),
        limits,
    )
    .await;
    let failure = outcome.err().expect("an omitted voucher refuses the read");
    assert_eq!(failure.code, PART_NOT_ADMITTED);
    assert!(failure.evidence.is_some(), "the part read is accounted for");
    assert_requests(
        &observed,
        &[1],
        &[VoucherReadShape::EntryWildcard
            .render(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                Some(AlterIdSpan {
                    after: 0,
                    through: 1,
                }),
            )
            .unwrap()],
    );
    assert_eq!(observed.len(), 6);
}

#[tokio::test]
async fn an_unobservable_high_water_mark_refuses_the_window_unread() {
    let foreign = mark_xml(
        "00000000-0000-4000-8000-000000000009",
        "<ALTVCHID>3</ALTVCHID>",
    );
    let (outcome, observed) = read_window(
        paired(&xml_plan(foreign)),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate { known_marks: None },
        three_a_read(),
    )
    .await;
    let failure = outcome.err().expect("an unestimated window is refused");
    // Refused under the parser's own cause, not as an unestimated window.
    assert_eq!(failure.code, "company_high_water_identity_absent");
    assert!(
        failure.evidence.is_some(),
        "the high-water read is accounted for"
    );
    assert_eq!(observed.len(), 6);
}

#[tokio::test]
async fn a_book_whose_mark_fits_one_census_is_counted_in_one_date_census() {
    // A mark of exactly one census's rows bounds a date census of any window,
    // so the window is counted by date alone: one census, which here proves the
    // window small enough to read whole — the request it was before the bound.
    let limits = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard);
    let mut plans = paired(&xml_plan(three_vouchers()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(limits.census_capacity())),
        },
        limits,
    )
    .await;
    let outcome = outcome.unwrap();
    assert_eq!(outcome.reads, [part("20260801", "20260801", None)]);
    // The census that proved it small is kept and the read admitted against
    // it, not dropped (#985).
    assert!(outcome.counted());
    assert_eq!(observed.len(), 12);
    assert_requests(
        &observed,
        &[1, 7],
        &[
            render_agent_voucher_census(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                None,
            )
            .unwrap(),
            render_agent_vouchers(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                None,
            )
            .unwrap(),
        ],
    );
}

/// #1029: a book whose mark alone shows it fits one request is counted only by
/// a read that asks for it. The limits a tool is given decide, so the posting
/// read-back and the movement read, which do not ask, send the one request
/// they always did.
#[tokio::test]
async fn a_small_book_is_counted_only_when_the_read_asks_for_it() {
    let skipping = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard);
    assert_eq!(skipping.small_books, SmallBooks::Skip);
    let (outcome, observed) = read_window(
        paired(&xml_plan(three_vouchers())),
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        skipping,
    )
    .await;
    assert!(!outcome.unwrap().counted());
    assert_eq!(observed.len(), 6);

    let counting = skipping.counting_small_books();
    assert_eq!(counting.small_books, SmallBooks::Count);
    // The census is answered from the window's own rows, as a large book's is.
    let mut plans = paired(&xml_plan(three_vouchers()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        counting,
    )
    .await;
    assert!(outcome.unwrap().counted());
    assert_eq!(observed.len(), 12);
    assert_requests(
        &observed,
        &[1, 7],
        &[
            render_agent_voucher_census(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                None,
            )
            .unwrap(),
            render_agent_vouchers(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                None,
            )
            .unwrap(),
        ],
    );
}

/// #1029: a company whose mark is zero has never held a voucher, so there is
/// nothing to count, whatever the read asks for.
#[tokio::test]
async fn a_book_that_never_held_a_voucher_is_not_counted_even_when_the_read_asks() {
    let (outcome, observed) = read_window(
        paired(&xml_plan(empty_collection())),
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(0)),
        },
        WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).counting_small_books(),
    )
    .await;
    assert!(!outcome.unwrap().counted());
    assert_eq!(observed.len(), 6);
}

#[tokio::test]
async fn a_book_one_voucher_past_one_census_is_counted_in_alterid_spans() {
    // #520 / review: a date census is bounded only by the whole book, so a book
    // whose mark exceeds one census is counted in AlterID spans of the window,
    // each bounded before it is sent. Before the fix this book was counted in one
    // date census sized from an estimate of its density.
    let limits = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard);
    let capacity = limits.census_capacity();
    let mut plans = paired(&xml_plan(three_vouchers()));
    plans.extend(paired(&xml_plan(empty_collection())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(capacity + 1)),
        },
        limits,
    )
    .await;
    assert_eq!(outcome.unwrap().rows.len(), 3);
    assert_eq!(observed.len(), 18);
    assert_requests(
        &observed,
        &[1, 7, 13],
        &[
            render_agent_voucher_census(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                Some(AlterIdSpan {
                    after: 0,
                    through: capacity,
                }),
            )
            .unwrap(),
            render_agent_voucher_census(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                Some(AlterIdSpan {
                    after: capacity,
                    through: capacity + 1,
                }),
            )
            .unwrap(),
            render_agent_vouchers(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                None,
            )
            .unwrap(),
        ],
    );
}

#[tokio::test]
async fn a_book_too_large_to_count_is_refused_by_name_before_any_census() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(listener.local_addr().unwrap(), directory.path());
    let identity = identity();
    let limits = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard);
    let failure = server
        .read_voucher_window(
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260801"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Estimate {
                known_marks: Some(marks_of(
                    MAX_CENSUS_READS as u64 * limits.census_capacity() + 1,
                )),
            },
            limits,
            |xml| parse_agent_rows(xml, GUID),
        )
        .await
        .err()
        .expect("refused");
    assert_eq!(failure.code, BOOK_TOO_LARGE);
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock),
        "nothing is sent"
    );
}

#[tokio::test]
async fn a_divided_read_is_bracketed_by_the_high_water_mark() {
    // A census of three vouchers on day one; the default fits one a read, so the
    // read divides. After the last part both marks are read again: unchanged,
    // the read stands; either moved, it is refused, because the parts no longer
    // describe one state of the book.
    //
    // #520 (live, 2026-09-21): a ledger renamed between two parts moves only
    // the master mark while changing the ledger's name in every voucher export.
    // Before the fix the bracket compared the voucher mark alone, and this
    // third case returned `complete`.
    for (closing, expect_ok) in [
        (marks_plan(3, 7), true),
        (marks_plan(4, 7), false),
        (marks_plan(3, 8), false),
    ] {
        let limits = three_a_read();
        let mut plans = paired(&xml_plan(three_vouchers()));
        plans.extend(paired(&xml_plan(relabelled(
            &vouchers_kept(1),
            &[(1, "20260801")],
        ))));
        plans.extend(paired(&xml_plan(relabelled(
            &vouchers_kept(2),
            &[(2, "20260801"), (3, "20260801")],
        ))));
        plans.extend(paired(&xml_plan(empty_collection())));
        plans.extend(paired(&closing));
        let (outcome, observed) = read_window(
            plans,
            ("20260801", "20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Estimate {
                known_marks: Some(marks_of(3)),
            },
            limits,
        )
        .await;
        assert_eq!(observed.len(), 30);
        assert_eq!(
            observed[25].request_body_sha256,
            request_sha(&render_agent_company_high_water(&company()))
        );
        match outcome {
            Ok(read) => {
                assert!(expect_ok);
                assert_eq!(read.rows.len(), 3);
                assert_eq!(
                    read.reads,
                    [
                        part(
                            "20260801",
                            "20260801",
                            Some(AlterIdSpan {
                                after: 0,
                                through: 1
                            })
                        ),
                        part(
                            "20260801",
                            "20260801",
                            Some(AlterIdSpan {
                                after: 1,
                                through: 3
                            })
                        ),
                        part("20260802", "20260802", None),
                    ]
                );
            }
            Err(failure) => {
                assert!(!expect_ok);
                assert_eq!(failure.code, WINDOW_CHANGED_DURING_READ);
                assert!(failure.evidence.is_some());
            }
        }
    }
}

/// #595: every request of a window read is timed by what it was for. The
/// second part is held 200 ms on each of its two paired bodies, so it alone
/// costs at least 400 ms; each part also reports one body's bytes and its rows.
#[tokio::test]
async fn a_window_read_times_its_marks_census_and_each_part() {
    let first = xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")]));
    let second = xml_plan(relabelled(
        &vouchers_kept(2),
        &[(2, "20260801"), (3, "20260801")],
    ))
    .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
        std::time::Duration::from_millis(200),
    ));
    let third = xml_plan(empty_collection());
    let mut plans = paired(&xml_plan(three_vouchers()));
    for body in [&first, &second, &third] {
        plans.extend(paired(body));
    }
    plans.extend(paired(&marks_plan(3, 7)));
    let call_started = std::time::Instant::now();
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
    )
    .await;
    let call_ms = call_started.elapsed().as_millis();
    assert_eq!(observed.len(), 30);
    let timings = outcome.unwrap().timings;
    // The marks were known: only the closing bracket read them.
    assert_eq!(timings.marks.requests, 1);
    assert_eq!(timings.census.requests, 1);
    let spans = timings
        .parts
        .iter()
        .map(|part| {
            (
                part.from.as_str(),
                part.to.as_str(),
                part.after,
                part.through,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        spans,
        [
            ("20260801", "20260801", Some(0), Some(1)),
            ("20260801", "20260801", Some(1), Some(3)),
            ("20260802", "20260802", None, None),
        ]
    );
    assert!(timings.parts.iter().all(|part| part.served));
    assert_eq!(
        timings
            .parts
            .iter()
            .map(|part| part.rows)
            .collect::<Vec<_>>(),
        [Some(1), Some(2), Some(0)]
    );
    assert_eq!(
        timings
            .parts
            .iter()
            .map(|part| part.bytes)
            .collect::<Vec<_>>(),
        [&first, &second, &third]
            .map(|body| Some(wire_len(body)))
            .to_vec()
    );
    // Only the delayed part's lower bound is asserted: a delay can only add to
    // elapsed time, so this holds on any runner. Comparing the undelayed parts
    // against it is not safe, since a loaded runner can stall them as long (#986).
    // Dropping a part's timing, or charging the delay to another part, leaves the
    // second part below the bound.
    assert!(timings.parts[1].ms >= 400, "{timings:?}");
    // Each part is timed on its own (#998). The parts are read one after another,
    // so their own times are disjoint pieces of the call and add up to no more
    // than the call took, however slow the runner. A timing that included the
    // parts before it would count the 400 ms delay again in the third part.
    let parts_ms = timings.parts.iter().map(|part| part.ms).sum::<u128>();
    assert!(parts_ms <= call_ms, "{parts_ms} > {call_ms}: {timings:?}");
    assert_eq!(timings.failed, None);
}

#[tokio::test]
async fn a_census_the_transport_refuses_is_not_divided_but_refused() {
    // Every census is bounded before it is sent: here the mark (4) is within
    // one census, so the window is one date census. Oversized all the same, it
    // means the per-row figure was wrong, not that the range was too wide, so it
    // is not halved into further censuses — the review's objection was to a
    // census whose size was only learned after Tally had built it. The window
    // is refused, and nothing more is sent.
    let one = wire_len(&xml_plan(vouchers_kept(1)));
    let limits = WindowReadLimits {
        budget_bytes: 3 * one,
        default_bytes_per_voucher: one,
        max_reads: MAX_PLANNED_READS,
        small_books: SmallBooks::Skip,
    };
    assert!(limits.census_capacity() >= 4);
    let (outcome, observed) = read_window(
        oversized(),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(4)),
        },
        limits,
    )
    .await;
    let failure = outcome.err().expect("refused");
    assert_eq!(failure.code, VOLUME_UNESTIMATED);
    assert_eq!(failure.cause, Some("census_response_too_large"));
    assert_eq!(observed.len(), 2);
    assert_requests(
        &observed,
        &[1],
        &[render_agent_voucher_census(
            &company(),
            &tally_date("20260801"),
            &tally_date("20260802"),
            None,
        )
        .unwrap()],
    );
}
#[tokio::test]
async fn a_verification_part_tally_cannot_serve_is_halved_left_first() {
    // #485 inside the shared reader: the undivided window is refused by the
    // transport, so it is read again in date halves — the earlier half first,
    // so rows arrive in the order one undivided read would have produced.
    //
    // #520: a read planned whole and divided only after Tally could not serve
    // it is a divided read like any other, so it is bracketed too. Before the
    // fix the bracket was gated on a census, which this read never took, and a
    // moved mark was accepted.
    let shape = VoucherReadShape::ImportVerification;
    for (closing, expect_ok) in [(1, true), (2, false)] {
        let mut plans = oversized();
        plans.extend(paired(&xml_plan(three_vouchers())));
        plans.extend(paired(&xml_plan(empty_collection())));
        plans.extend(paired(&mark(closing)));
        let (outcome, observed) = read_window(
            plans,
            ("20260801", "20260802"),
            shape,
            WindowPlanSource::Estimate {
                known_marks: Some(marks_of(1)),
            },
            WindowReadLimits::for_shape(shape),
        )
        .await;
        assert_eq!(observed.len(), 20);
        assert_eq!(
            observed[15].request_body_sha256,
            request_sha(&render_agent_company_high_water(&company()))
        );
        if !expect_ok {
            let failure = outcome.err().expect("a moved mark refuses the split read");
            assert_eq!(failure.code, WINDOW_CHANGED_DURING_READ);
            continue;
        }
        split_read_checks(outcome.unwrap(), &observed, shape);
    }
    // A shape that does not halve fails on the same response instead.
    let (outcome, _) = read_window(
        oversized(),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(1)),
        },
        WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard),
    )
    .await;
    assert_eq!(
        outcome.err().map(|failure| failure.code).as_deref(),
        Some("response_size_limit_exceeded")
    );
}

fn split_read_checks(
    outcome: WindowReadOutcome<Value>,
    observed: &[tally_protocol_simulator::ObservedRequest],
    shape: VoucherReadShape,
) {
    assert_eq!(
        outcome.reads,
        [
            part("20260801", "20260801", None),
            part("20260802", "20260802", None)
        ]
    );
    assert_requests(
        observed,
        &[1, 3, 9],
        &[
            shape
                .render(
                    &company(),
                    &tally_date("20260801"),
                    &tally_date("20260802"),
                    None,
                )
                .unwrap(),
            shape
                .render(
                    &company(),
                    &tally_date("20260801"),
                    &tally_date("20260801"),
                    None,
                )
                .unwrap(),
            shape
                .render(
                    &company(),
                    &tally_date("20260802"),
                    &tally_date("20260802"),
                    None,
                )
                .unwrap(),
        ],
    );
}

/// #680: the carry-forward #494 added, driven through the reader. A 4-day
/// window is refused, and so is its left 2-day half. The right 2-day half is
/// as wide as a span already refused on this call, so it is split without
/// being sent, and all four days are read singly, in date order.
#[tokio::test]
async fn a_sibling_as_wide_as_a_refused_part_is_split_without_being_sent() {
    let shape = VoucherReadShape::ImportVerification;
    let mut plans = oversized();
    plans.extend(oversized());
    plans.extend(paired(&xml_plan(three_vouchers())));
    for _ in 0..3 {
        plans.extend(paired(&xml_plan(empty_collection())));
    }
    plans.extend(paired(&mark(1)));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260804"),
        shape,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(1)),
        },
        WindowReadLimits::for_shape(shape),
    )
    .await;
    let outcome = outcome.expect("the window is read in single days");
    assert_eq!(
        outcome.reads,
        ["20260801", "20260802", "20260803", "20260804"].map(|day| part(day, day, None))
    );
    let render = |from, to| {
        shape
            .render(&company(), &tally_date(from), &tally_date(to), None)
            .unwrap()
    };
    assert_eq!(observed.len(), 34);
    assert_requests(
        &observed,
        &[1, 3, 5, 11, 17, 23],
        &[
            render("20260801", "20260804"),
            render("20260801", "20260802"),
            render("20260801", "20260801"),
            render("20260802", "20260802"),
            render("20260803", "20260803"),
            render("20260804", "20260804"),
        ],
    );
    let right_half = request_sha(&render("20260803", "20260804"));
    assert!(
        observed
            .iter()
            .all(|request| request.request_body_sha256 != right_half),
        "the right half was sent, though a part as wide was already refused"
    );
    assert_eq!(
        observed[29].request_body_sha256,
        request_sha(&render_agent_company_high_water(&company()))
    );
    // Every row one undivided read would have returned, in date order.
    assert_eq!(
        outcome.rows,
        parse_agent_rows(&three_vouchers(), GUID).unwrap()
    );
}

/// The control for the test above: a sibling narrower than every part refused
/// so far is read, not split. A 5-day window divides 3/2; the 3-day half is
/// refused, so its 2-day left part and the window's 2-day right half are both
/// read whole.
#[tokio::test]
async fn a_sibling_narrower_than_every_refused_part_is_read() {
    let shape = VoucherReadShape::ImportVerification;
    let mut plans = oversized();
    plans.extend(oversized());
    plans.extend(paired(&xml_plan(three_vouchers())));
    for _ in 0..2 {
        plans.extend(paired(&xml_plan(empty_collection())));
    }
    plans.extend(paired(&mark(1)));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260805"),
        shape,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(1)),
        },
        WindowReadLimits::for_shape(shape),
    )
    .await;
    let outcome = outcome.expect("the window is read in three parts");
    assert_eq!(
        outcome.reads,
        [
            part("20260801", "20260802", None),
            part("20260803", "20260803", None),
            part("20260804", "20260805", None),
        ]
    );
    let render = |from, to| {
        shape
            .render(&company(), &tally_date(from), &tally_date(to), None)
            .unwrap()
    };
    assert_eq!(observed.len(), 28);
    assert_requests(
        &observed,
        &[1, 3, 5, 11, 17],
        &[
            render("20260801", "20260805"),
            render("20260801", "20260803"),
            render("20260801", "20260802"),
            render("20260803", "20260803"),
            render("20260804", "20260805"),
        ],
    );
}

#[tokio::test]
async fn a_failed_parse_keeps_the_evidence_of_the_part_just_read() {
    let (outcome, _) = read_window(
        paired(&xml_plan(
            "<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER></ENVELOPE>".to_string(),
        )),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(1)),
        },
        WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard),
    )
    .await;
    let failure = outcome.err().expect("a malformed part fails the read");
    assert!(failure.evidence.expect("the part is accounted for").bytes > 0);
}

#[tokio::test]
async fn a_supplied_count_of_another_window_is_refused_before_any_read() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(listener.local_addr().unwrap(), directory.path());
    let identity = identity();
    let failure = server
        .read_voucher_window(
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Counted(WindowCensus::from_rows([
                (day("20260801"), 1),
                (day("20260803"), 2),
            ])),
            three_a_read(),
            |xml| parse_agent_rows(xml, GUID),
        )
        .await
        .err()
        .expect("refused");
    assert_eq!(failure.code, "window_not_honoured");
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

#[tokio::test]
async fn the_vouchers_tool_counts_a_small_book_before_reading_its_window() {
    // Through the vouchers tool (#1029): a book whose mark alone shows it fits
    // one request is still counted. The reads are the high-water mark, one
    // census of the window, then the window read, which is the same request
    // the tool sent before the bound existed.
    let mut plans = vec![company_plan(), status_plan(), company_plan(), status_plan()];
    plans.extend(paired(&mark(3)));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let response = server_at(simulator.address(), directory.path())
        .call_tool(
            "vouchers",
            json!({"company_guid": GUID, "from": "20260801", "to": "20260831"}),
        )
        .await;
    assert_eq!(response["isError"], false, "{response}");
    assert_eq!(
        response["structuredContent"]["result"]["items"]
            .as_array()
            .map(Vec::len),
        Some(3)
    );
    assert_eq!(
        response["structuredContent"]["result"]["state"], "complete",
        "{response}"
    );
    let observed = simulator.finish().unwrap();
    // 4 prelude legs, then three paired reads of six legs: marks, census, window.
    assert_eq!(observed.len(), 22);
    assert_requests(
        &observed,
        &[5, 11, 13, 17, 19],
        &[
            render_agent_company_high_water(&company()),
            render_agent_voucher_census(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260831"),
                None,
            )
            .unwrap(),
            render_agent_voucher_census(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260831"),
                None,
            )
            .unwrap(),
            render_agent_vouchers(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260831"),
                None,
            )
            .unwrap(),
            render_agent_vouchers(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260831"),
                None,
            )
            .unwrap(),
        ],
    );
}

#[tokio::test]
async fn a_lighter_part_never_loosens_the_plan_for_the_rest() {
    // Day one measures one heavy voucher (its narration padded in memory); day
    // two, three far lighter ones. The rest is planned at the heavier figure:
    // day three's five vouchers go in spans of three and two, not in the one
    // read the lighter part alone would allow.
    let heavy = vouchers_kept(1).replace(
        "<NARRATION TYPE=\"String\">",
        &format!("<NARRATION TYPE=\"String\">{}", "N".repeat(20_000)),
    );
    assert_ne!(heavy, vouchers_kept(1));
    // The default fits one voucher a read and its floor (half) no more than
    // the heavy measurement, so the measured figure is what plans the rest.
    let heavy_len = wire_len(&xml_plan(heavy.clone()));
    let limits = WindowReadLimits {
        budget_bytes: 3 * heavy_len,
        default_bytes_per_voucher: 2 * heavy_len,
        max_reads: MAX_PLANNED_READS,
        small_books: SmallBooks::Skip,
    };
    let census = WindowCensus::from_rows(
        [
            ("20260801", 1),
            ("20260802", 2),
            ("20260802", 3),
            ("20260802", 4),
        ]
        .into_iter()
        .map(|(date, id)| (day(date), id))
        .chain((5..=9).map(|id| (day("20260803"), id))),
    );
    let mut plans = paired(&xml_plan(relabelled(&heavy, &[(1, "20260801")])));
    plans.extend(paired(&xml_plan(relabelled(
        &three_vouchers(),
        &[(2, "20260802"), (3, "20260802"), (4, "20260802")],
    ))));
    plans.extend(paired(&xml_plan(relabelled(
        &three_vouchers(),
        &[(5, "20260803"), (6, "20260803"), (7, "20260803")],
    ))));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(8, "20260803"), (9, "20260803")],
    ))));
    let (outcome, _) = read_window(
        plans,
        ("20260801", "20260803"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census),
        limits,
    )
    .await;
    assert_eq!(
        outcome.unwrap().reads,
        [
            part("20260801", "20260801", None),
            part("20260802", "20260802", None),
            part(
                "20260803",
                "20260803",
                Some(AlterIdSpan {
                    after: 0,
                    through: 7
                })
            ),
            part(
                "20260803",
                "20260803",
                Some(AlterIdSpan {
                    after: 7,
                    through: 9
                })
            ),
        ]
    );
}

#[test]
fn a_light_first_measurement_cannot_plan_a_heavy_part_over_the_cap() {
    // The floor: a measured figure never falls below half the shape default.
    let default = VoucherReadShape::ImportVerification.default_wire_bytes_per_voucher();
    assert_eq!(planning_figure(default, 10_000), default / 2);
    assert_eq!(planning_figure(default, 70_000), 70_000);
    // The review's case: a first part of bank receipts at ~10 KB on the wire,
    // then inventory vouchers at the default's weight. Planned from the light
    // figure, every part must still fit the cap at the heavy weight — and fit
    // the budget at the default's own floor.
    let census = heavy_book();
    let figure = planning_figure(default, 10_000);
    let reads = plan_window_reads(
        day("20250601"),
        day("20250630"),
        &census,
        None,
        census.max_alter_id(),
        figure,
        WINDOW_READ_BUDGET_BYTES,
        MAX_PLANNED_READS,
    )
    .unwrap();
    for read in &reads {
        assert!(
            read.vouchers * figure <= WINDOW_READ_BUDGET_BYTES,
            "{read:?}"
        );
        assert!(
            read.vouchers * default <= bridge_tally_transport::XML_RESPONSE_MAX_BYTES as u64,
            "{read:?} would exceed the cap at the default's weight"
        );
    }
}

#[tokio::test]
async fn a_light_first_part_does_not_widen_the_next_beyond_the_floor() {
    // Default capacity 1; the floor allows at most 2 a read. Day one measures a
    // light voucher that alone would allow 4, so day two's four vouchers must
    // still go in two spans of two.
    let one = wire_len(&xml_plan(vouchers_kept(1)));
    let limits = WindowReadLimits {
        budget_bytes: 4 * one,
        default_bytes_per_voucher: 4 * one,
        max_reads: MAX_PLANNED_READS,
        small_books: SmallBooks::Skip,
    };
    let census = WindowCensus::from_rows([
        (day("20260801"), 1),
        (day("20260802"), 2),
        (day("20260802"), 3),
        (day("20260802"), 4),
        (day("20260802"), 5),
    ]);
    let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(2, "20260802"), (3, "20260802")],
    ))));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(4, "20260802"), (5, "20260802")],
    ))));
    let (outcome, _) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census),
        limits,
    )
    .await;
    assert_eq!(
        outcome.unwrap().reads,
        [
            part("20260801", "20260801", None),
            part(
                "20260802",
                "20260802",
                Some(AlterIdSpan {
                    after: 0,
                    through: 3
                })
            ),
            part(
                "20260802",
                "20260802",
                Some(AlterIdSpan {
                    after: 3,
                    through: 5
                })
            ),
        ]
    );
}

#[test]
fn a_book_whose_mark_fits_one_census_is_counted_by_date_and_a_larger_one_in_spans() {
    // The switch point, on each side. A mark of exactly one census's capacity
    // bounds a date census of any window, so that stays the one request it
    // was; one more voucher and no date census is bounded, so the book is
    // counted in AlterID spans, each bounded by construction.
    // Sized against the whole transport cap: 32 MiB at 4 KiB a row.
    let capacity = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).census_capacity();
    assert_eq!(capacity, 8192);
    assert_eq!(
        census_spans(capacity, capacity)
            .unwrap()
            .collect::<Vec<_>>(),
        [None]
    );
    assert_eq!(
        census_spans(capacity + 1, capacity)
            .unwrap()
            .collect::<Vec<_>>(),
        [
            Some(AlterIdSpan {
                after: 0,
                through: capacity
            }),
            Some(AlterIdSpan {
                after: capacity,
                through: capacity + 1
            }),
        ]
    );
    // An empty book is one date census too.
    assert_eq!(
        census_spans(0, capacity).unwrap().collect::<Vec<_>>(),
        [None]
    );
}

#[test]
fn census_spans_tile_the_mark_and_none_holds_more_than_one_census() {
    for mark in [4097_u64, 8192, 8193, 250_000] {
        let spans = census_spans(mark, 4096)
            .unwrap()
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        assert_eq!(spans.len() as u64, mark.div_ceil(4096), "mark {mark}");
        assert_eq!(spans.first().unwrap().after, 0);
        assert_eq!(spans.last().unwrap().through, mark);
        for pair in spans.windows(2) {
            assert_eq!(pair[0].through, pair[1].after, "gap or overlap at {mark}");
        }
        assert!(spans
            .iter()
            .all(|span| span.through - span.after <= 4096 && span.through > span.after));
    }
    // The live book of §11c.1 (a mark of about a quarter of a million) is 31
    // spans at the production width.
    assert_eq!(census_spans(249_948, 8192).unwrap().count(), 31);
}

#[test]
fn a_book_needing_more_census_spans_than_allowed_is_refused_before_any_is_made() {
    let largest = MAX_CENSUS_READS as u64 * 4096;
    assert_eq!(
        census_spans(largest, 4096).unwrap().count(),
        MAX_CENSUS_READS
    );
    assert_eq!(census_spans(largest + 1, 4096).err(), Some(BOOK_TOO_LARGE));
    // A mark far beyond any real book is refused at once, not walked: the
    // refusal is decided from the count, before a span exists.
    assert_eq!(census_spans(u64::MAX, 4096).err(), Some(BOOK_TOO_LARGE));
    // And the spans of an admitted mark are produced one at a time.
    let mut spans = census_spans(largest, 4096).unwrap();
    assert_eq!(
        spans.next(),
        Some(Some(AlterIdSpan {
            after: 0,
            through: 4096
        }))
    );
}

#[test]
fn a_census_refuses_on_an_oversized_response_and_on_a_deadline() {
    use bridge_tally_transport::TallyTransportError;
    let oversized = TallyTransportError::ResponseTooLarge {
        limit: 32 * 1024 * 1024,
        declared_by_peer: true,
    };
    // Every census is bounded before it is sent, so an oversized one is not
    // divided: its per-row figure was wrong, and the window is refused.
    assert_eq!(census_failure(oversized.safe_code()), CensusFailure::Refuse);
    // A deadline is never retried, not even in halves.
    assert_eq!(
        census_failure(TallyTransportError::RequestTimedOut.safe_code()),
        CensusFailure::Refuse
    );
    assert_eq!(
        census_failure(TallyTransportError::ConnectionFailed.safe_code()),
        CensusFailure::Propagate
    );
    assert_eq!(
        census_failure("agent_runtime_read_failed"),
        CensusFailure::Propagate
    );
}

#[tokio::test]
async fn a_census_that_times_out_refuses_the_window_and_sends_nothing_more() {
    // Takes the transport's 20 s deadline. The census never answers; the read
    // is refused as unestimated and nothing else is sent. Spare responses are
    // queued so that a retry, if one were made, would be served and seen.
    let limits = WindowReadLimits {
        budget_bytes: 16 * 1024,
        default_bytes_per_voucher: 16 * 1024,
        max_reads: MAX_PLANNED_READS,
        small_books: SmallBooks::Skip,
    };
    let mut plans = vec![
        company_plan(),
        xml_plan(empty_collection()).with_delivery(
            tally_protocol_simulator::Delivery::SlowHeaders(std::time::Duration::from_secs(25)),
        ),
    ];
    for _ in 0..4 {
        plans.extend(paired(&xml_plan(empty_collection())));
    }
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let identity = identity();
    let failure = server
        .read_voucher_window(
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260801"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Estimate {
                known_marks: Some(marks_of(9)),
            },
            limits,
            |xml| parse_agent_rows(xml, GUID),
        )
        .await
        .err()
        .expect("a census deadline refuses the window");
    assert_eq!(failure.code, VOLUME_UNESTIMATED);
    assert_eq!(failure.cause, Some("census_deadline_exceeded"));
    // #595: the failure carries what the read cost up to it. The marks were
    // known, so the one request sent was the census, and it took the deadline.
    let timings = failure
        .window_timings
        .as_deref()
        .expect("a failed window read carries its timings");
    assert_eq!(timings.marks, RequestTally::default());
    assert_eq!(timings.census.requests, 1);
    assert!(timings.census.ms >= 19_000, "{timings:?}");
    assert!(timings.parts.is_empty());
    assert_eq!(
        timings.failed,
        Some(FailedRequest {
            kind: "census",
            ms: timings.census.ms
        })
    );
    simulator.cancel();
    let observed = simulator.finish().unwrap();
    assert_eq!(
        observed.len(),
        2,
        "nothing was sent after the timed-out census"
    );
    // A mark of 9 over a census capacity of 8: the first AlterID span.
    assert_eq!(limits.census_capacity(), 8);
    assert_eq!(
        observed[1].request_body_sha256,
        request_sha(
            &render_agent_voucher_census(
                &company(),
                &tally_date("20260801"),
                &tally_date("20260801"),
                Some(AlterIdSpan {
                    after: 0,
                    through: 8
                })
            )
            .unwrap()
        )
    );
}

#[tokio::test]
async fn a_read_divided_only_by_date_is_bracketed_too() {
    // Two days of one voucher each; the default fits one a read, so the window
    // is read as two date ranges with no AlterID span. The mark is still read
    // again afterwards, and a moved mark refuses the read.
    let heavy = vouchers_kept(1).replace(
        "<NARRATION TYPE=\"String\">",
        &format!("<NARRATION TYPE=\"String\">{}", "N".repeat(20_000)),
    );
    let heavy_len = wire_len(&xml_plan(heavy.clone()));
    let limits = WindowReadLimits {
        budget_bytes: 2 * heavy_len,
        default_bytes_per_voucher: 2 * heavy_len,
        max_reads: MAX_PLANNED_READS,
        small_books: SmallBooks::Skip,
    };
    // One census: AlterIDs 2 and 3, one on each day.
    let two = vouchers_kept(2);
    let second_date = two.rfind("<DATE TYPE=\"Date\">20260801</DATE>").unwrap();
    let mut census = two.clone();
    census.replace_range(
        second_date..second_date + "<DATE TYPE=\"Date\">20260801</DATE>".len(),
        "<DATE TYPE=\"Date\">20260802</DATE>",
    );
    for (closing, expect_ok) in [(3, true), (4, false)] {
        let mut plans = paired(&xml_plan(census.clone()));
        plans.extend(paired(&xml_plan(relabelled(&heavy, &[(2, "20260801")]))));
        plans.extend(paired(&xml_plan(
            vouchers_kept(1).replace("20260801", "20260802"),
        )));
        plans.extend(paired(&mark(closing)));
        let (outcome, observed) = read_window(
            plans,
            ("20260801", "20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Estimate {
                known_marks: Some(marks_of(3)),
            },
            limits,
        )
        .await;
        assert_eq!(observed.len(), 24);
        match outcome {
            Ok(read) => {
                assert!(expect_ok);
                assert_eq!(
                    read.reads,
                    [
                        part("20260801", "20260801", None),
                        part("20260802", "20260802", None)
                    ]
                );
            }
            Err(failure) => {
                assert!(!expect_ok);
                assert_eq!(failure.code, WINDOW_CHANGED_DURING_READ);
            }
        }
    }
}

#[test]
fn a_census_row_with_alterid_zero_is_refused() {
    // A day divided by AlterID starts every span above 0, so a voucher with
    // AlterID 0 could never be read in parts. Refuse the census instead.
    let zero = three_vouchers().replacen(
        "<ALTERID TYPE=\"Number\"> 1</ALTERID>",
        "<ALTERID TYPE=\"Number\"> 0</ALTERID>",
        1,
    );
    assert_ne!(zero, three_vouchers());
    assert_eq!(
        parse_voucher_census(&zero, (day("20260801"), day("20260801")), None),
        Err("agent_read_protocol_invalid".to_string())
    );
}

// ---------------------------------------------------------------------------
// #520: the replay/evidence contract. Each test below is a labelled negative
// control with an unchanged-book control beside it.
// ---------------------------------------------------------------------------

/// The divided first read the replay tests repeat: a census of three vouchers on
/// day one under a mark of 3, read in spans `(0,1]` and `(1,3]`, then day two
/// whole, and closed at an unchanged mark.
fn divided_first_read() -> (Vec<ScenarioPlan>, Vec<ScenarioPlan>) {
    let parts = vec![
        xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])),
        xml_plan(relabelled(
            &vouchers_kept(2),
            &[(2, "20260801"), (3, "20260801")],
        )),
        xml_plan(empty_collection()),
    ];
    let mut first = paired(&xml_plan(three_vouchers()));
    for part in &parts {
        first.extend(paired(part));
    }
    first.extend(paired(&marks_plan(3, 7)));
    (first, parts)
}

async fn first_read() -> WindowReadOutcome<Value> {
    let (plans, _) = divided_first_read();
    let (outcome, _) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
    )
    .await;
    let outcome = outcome.unwrap();
    assert!(is_divided(&outcome.reads));
    outcome
}

#[tokio::test]
async fn a_replay_refuses_a_voucher_created_above_the_first_reads_ceiling() {
    // #520 P1. A voucher posted in the window after the first read closed takes
    // AlterID 4, above the ceiling (3) the replayed spans were planned to. Both
    // old-ceiling responses exclude it and are byte-identical to the first
    // read's, so the two snapshots match; before the fix the replay read no
    // mark and the corroboration was accepted with the posting missing from
    // both. The replay now closes against the first read's witness.
    let first = first_read().await;
    let witness = first.witness.clone().expect("a divided read has a witness");
    assert_eq!(witness.marks, marks_of(3));
    for (closing, expect_ok) in [(marks_plan(3, 7), true), (marks_plan(4, 7), false)] {
        let (_, parts) = divided_first_read();
        let mut plans = Vec::new();
        for part in &parts {
            plans.extend(paired(part));
        }
        plans.extend(paired(&closing));
        let (outcome, observed) = read_window(
            plans,
            ("20260801", "20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Replay {
                parts: first.reads.clone(),
                witness: Some(witness.clone()),
            },
            three_a_read(),
        )
        .await;
        assert_eq!(observed.len(), 24, "three parts and the closing marks");
        match outcome {
            Ok(replay) => {
                assert!(expect_ok);
                // The data the two reads compare is identical either way,
                // which is exactly why the marks must decide.
                assert_eq!(
                    replay.evidence.response_sha256,
                    first.evidence.response_sha256
                );
            }
            Err(failure) => {
                assert!(!expect_ok);
                assert_eq!(failure.code, WINDOW_CHANGED_DURING_READ);
                assert!(failure.evidence.is_some());
            }
        }
    }
}

// --- the bracketed count (#1241) --------------------------------------------

/// A divided read admitted against a census that names every GUID, and closed
/// on the marks it opened on, is the one read that holds the token.
#[tokio::test]
async fn a_divided_counted_read_closed_on_its_opening_marks_holds_the_bracketed_count() {
    let first = first_read().await;
    assert!(first.counted());
    assert!(first.bracketed.is_some());
    assert!(matches!(
        SecondRead::of(first.reads.clone(), first.witness.clone(), first.bracketed),
        SecondRead::Spared(_)
    ));
}

/// A window counted and read whole is admitted against its census (#985) but
/// has no closing bracket, so it keeps its replay.
#[tokio::test]
async fn a_window_counted_and_read_whole_does_not_hold_the_bracketed_count() {
    let mut plans = paired(&xml_plan(three_vouchers()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).counting_small_books(),
    )
    .await;
    let outcome = outcome.unwrap();
    assert!(outcome.counted());
    assert!(!is_divided(&outcome.reads));
    assert_eq!(
        observed.len(),
        12,
        "the census and the window, no closing marks"
    );
    assert!(outcome.bracketed.is_none());
    assert!(matches!(
        SecondRead::of(
            outcome.reads.clone(),
            outcome.witness.clone(),
            outcome.bracketed
        ),
        SecondRead::Replay(WindowPlanSource::Replay { .. })
    ));
}

/// A window the caller counted itself (here with every GUID, as a held
/// window's census is) opened on no marks, so nothing closes on them: divided
/// and admitted against its census, it still holds no token.
#[tokio::test]
async fn a_divided_read_of_a_window_the_caller_counted_does_not_hold_the_bracketed_count() {
    let guid = |value: u64| format!("{GUID}-{value:08x}").to_ascii_lowercase();
    let census = WindowCensus::from_census_rows([
        CensusRow {
            day: day("20260801"),
            alter_id: 1,
            guid: guid(1),
        },
        CensusRow {
            day: day("20260802"),
            alter_id: 2,
            guid: guid(2),
        },
    ])
    .expect("a census of two distinct AlterIDs");
    assert!(census.names_every_guid());
    let mut plans = Vec::new();
    for part in [
        xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])),
        xml_plan(relabelled(&vouchers_kept(1), &[(2, "20260802")])),
    ] {
        plans.extend(paired(&part));
    }
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census),
        WindowReadLimits {
            budget_bytes: wire_len(&xml_plan(vouchers_kept(1))),
            default_bytes_per_voucher: wire_len(&xml_plan(vouchers_kept(1))),
            max_reads: MAX_PLANNED_READS,
            small_books: SmallBooks::Skip,
        },
    )
    .await;
    let outcome = outcome.unwrap();
    assert!(is_divided(&outcome.reads));
    assert_eq!(observed.len(), 12, "two parts, no marks");
    assert!(outcome.bracketed.is_none());
}

/// A replay is itself the second read: it never holds the token, whatever it
/// was admitted against.
#[tokio::test]
async fn a_replay_does_not_hold_the_bracketed_count() {
    let first = first_read().await;
    let (_, parts) = divided_first_read();
    let mut plans = Vec::new();
    for part in &parts {
        plans.extend(paired(part));
    }
    plans.extend(paired(&marks_plan(3, 7)));
    let (replay, _) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::replay_of(first.reads.clone(), first.witness.clone()),
        three_a_read(),
    )
    .await;
    let replay = replay.unwrap();
    assert!(is_divided(&replay.reads));
    assert!(replay.bracketed.is_none());
}

/// Without the token the second read is the replay of exactly the parts read,
/// carrying the witness.
#[test]
fn without_the_bracketed_count_the_second_read_is_a_replay_of_the_parts_read() {
    let parts = vec![WindowPart {
        from: tally_date("20260801"),
        to: tally_date("20260802"),
        span: None,
    }];
    match SecondRead::of(parts.clone(), None, None) {
        SecondRead::Replay(WindowPlanSource::Replay {
            parts: replayed,
            witness: None,
        }) => assert_eq!(replayed, parts),
        _ => panic!("a read without the token replays its parts"),
    }
}

#[tokio::test]
async fn a_replay_of_a_divided_read_without_its_witness_is_refused_unread() {
    let first = first_read().await;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(listener.local_addr().unwrap(), directory.path());
    let identity = identity();
    let failure = server
        .read_voucher_window(
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Replay {
                parts: first.reads,
                witness: None,
            },
            three_a_read(),
            |xml| parse_agent_rows(xml, GUID),
        )
        .await
        .err()
        .expect("refused");
    assert_eq!(failure.code, REPLAY_UNWITNESSED);
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
    );
}

#[tokio::test]
async fn a_replayed_part_is_admitted_against_the_first_reads_census() {
    // The replay reads no census of its own, so it is admitted against the
    // witness's: a replayed part that returns a different voucher in the same
    // span is refused, even where the first read's snapshot would differ anyway.
    let first = first_read().await;
    let substituted = xml_plan(relabelled(
        &vouchers_kept(2),
        &[(2, "20260801"), (3, "20260801")],
    ));
    let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
    // Same AlterIDs as counted, but the second voucher is not the one counted.
    let other = relabelled(&vouchers_kept(2), &[(2, "20260801"), (3, "20260801")])
        .replace(&format!("{GUID}-00000003"), &format!("{GUID}-000000ff"));
    assert_ne!(other, substituted.fixture.body());
    plans.extend(paired(&xml_plan(other)));
    let (outcome, _) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Replay {
            parts: first.reads,
            witness: first.witness,
        },
        three_a_read(),
    )
    .await;
    assert_eq!(
        outcome.err().map(|failure| failure.code).as_deref(),
        Some(PART_NOT_ADMITTED)
    );
}

#[tokio::test]
async fn a_part_returning_a_voucher_its_census_did_not_count_is_refused() {
    // #520 P1, partition admission (substitution preserving the count). The
    // census counted AlterID 1 on day one and 2 on day two; day one's part
    // returns one voucher — AlterID 5, which the census never counted. The
    // count matches, so a count check alone passes it; before the fix the read
    // returned it as the window's. The unchanged control is the same read with
    // the counted voucher.
    for (day_one, expect_ok) in [(1, true), (5, false)] {
        let census = WindowCensus::from_rows([(day("20260801"), 1), (day("20260802"), 2)]);
        let mut plans = paired(&xml_plan(relabelled(
            &vouchers_kept(1),
            &[(day_one, "20260801")],
        )));
        if expect_ok {
            plans.extend(paired(&xml_plan(relabelled(
                &vouchers_kept(1),
                &[(2, "20260802")],
            ))));
        }
        let (outcome, _) = read_window(
            plans,
            ("20260801", "20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Counted(census),
            three_a_read(),
        )
        .await;
        match outcome {
            Ok(read) => {
                assert!(expect_ok);
                assert_eq!(read.rows.len(), 2);
            }
            Err(failure) => {
                assert!(!expect_ok);
                assert_eq!(failure.code, PART_NOT_ADMITTED);
                assert!(failure.evidence.is_some());
            }
        }
    }
}

#[tokio::test]
async fn a_voucher_returned_by_two_parts_is_refused_across_the_union() {
    // #520 P1, union identity. Two parts, each valid on its own: day one returns
    // GUID ...01 as AlterID 1, day two returns the same GUID as AlterID 2 (the
    // voucher re-dated between the parts). Each response admits its own rows,
    // and the census here carries no GUIDs, so only the union can see it; before
    // the fix movement summed it twice. The control returns two vouchers.
    for (duplicate, expect_ok) in [(false, true), (true, false)] {
        let census = WindowCensus::from_rows([(day("20260801"), 1), (day("20260802"), 2)]);
        let mut second = relabelled(&vouchers_kept(1), &[(2, "20260802")]);
        if duplicate {
            second = second.replace(&format!("{GUID}-00000002"), &format!("{GUID}-00000001"));
        }
        let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
        plans.extend(paired(&xml_plan(second)));
        let (outcome, _) = read_window(
            plans,
            ("20260801", "20260802"),
            VoucherReadShape::Movement,
            WindowPlanSource::Counted(census),
            three_a_read(),
        )
        .await;
        match outcome {
            Ok(read) => {
                assert!(expect_ok);
                assert_eq!(read.rows.len(), 2);
            }
            Err(failure) => {
                assert!(!expect_ok);
                assert_eq!(failure.code, "voucher_source_identity_invalid");
                assert!(failure.evidence.is_some());
            }
        }
    }
}

#[tokio::test]
async fn the_read_allowance_is_spent_at_dispatch_including_reactive_splits() {
    // #520 P2. The window is planned whole — one read, well within an allowance
    // of two — and Tally cannot serve it, so it is divided into two days: three
    // data requests in all. Before the fix the allowance was checked only when a
    // measurement re-planned, so all three were sent under an allowance of two.
    // The control spends exactly three under an allowance of three.
    let shape = VoucherReadShape::ImportVerification;
    for (allowance, expect_ok) in [(3, true), (2, false)] {
        let limits = WindowReadLimits {
            max_reads: allowance,
            small_books: SmallBooks::Skip,
            ..WindowReadLimits::for_shape(shape)
        };
        let mut plans = oversized();
        plans.extend(paired(&xml_plan(three_vouchers())));
        if expect_ok {
            plans.extend(paired(&xml_plan(empty_collection())));
            plans.extend(paired(&mark(1)));
        }
        let (outcome, observed) = read_window(
            plans,
            ("20260801", "20260802"),
            shape,
            WindowPlanSource::Estimate {
                known_marks: Some(marks_of(1)),
            },
            limits,
        )
        .await;
        match outcome {
            Ok(read) => {
                assert!(expect_ok);
                assert_eq!(read.reads.len(), 2);
                assert_eq!(observed.len(), 20);
            }
            Err(failure) => {
                assert!(!expect_ok);
                assert_eq!(failure.code, "voucher_window_too_many_reads");
                assert!(failure.evidence.is_some(), "the reads made are kept");
                // The oversized attempt and day one: the third is never sent.
                assert_eq!(observed.len(), 8);
            }
        }
    }
}

#[tokio::test]
async fn a_divided_reads_evidence_is_folded_in_the_order_it_was_sent() {
    // Review: the closing bracket is read after the data parts, so every fold of
    // this read's evidence must put it after them, not beside the opening reads.
    let read = first_read().await;
    let opening = read.preflight_evidence.clone().expect("the census");
    let closing = read.closing_evidence.clone().expect("the closing marks");
    let in_order = combine_evidence(
        combine_evidence(opening.clone(), read.evidence.clone()),
        closing.clone(),
    );
    let out_of_order = combine_evidence(combine_evidence(opening, closing), read.evidence.clone());
    let all = read.all_evidence();
    assert_eq!(
        (all.request_sha256, all.response_sha256, all.bytes),
        (
            in_order.request_sha256.clone(),
            in_order.response_sha256.clone(),
            in_order.bytes
        )
    );
    assert_ne!(in_order.request_sha256, out_of_order.request_sha256);
}

#[test]
fn the_pre_post_request_is_admitted_on_what_verification_measured() {
    // Review of #520: the whole-window pre-post request is admitted on the
    // verification read's own measurement, not the pre-flight's default-cost
    // prediction — which refused any window already holding 171 vouchers in
    // the verification shape, however light the book.
    let evidence = |bytes: usize| Evidence {
        request_sha256: String::new(),
        response_sha256: String::new(),
        bytes,
        state: "complete",
        read_at: None,
        duration_ms: None,
        reason_code: None,
    };
    let whole = [part("20260801", "20260831", None)];
    let divided = [
        part("20260801", "20260815", None),
        part("20260816", "20260831", None),
    ];
    let budget = usize::try_from(WINDOW_READ_BUDGET_BYTES).unwrap();
    // Undivided: that read was the whole request, whatever its size.
    assert!(WindowServed::of(&whole, &evidence(4 * budget), false).fits_one_request());
    // Divided, a light book: both parts together (one copy each) fit.
    let light = WindowServed::of(&divided, &evidence(2 * budget), false);
    assert_eq!(light.data_bytes, WINDOW_READ_BUDGET_BYTES);
    assert!(light.fits_one_request());
    // Divided, and one byte more than the budget: refused.
    assert!(!WindowServed::of(&divided, &evidence(2 * budget + 2), false).fits_one_request());
}

#[tokio::test]
async fn a_corroborating_replay_carries_the_first_reads_witness() {
    // Review of #520: verify_import's corroboration builds its replay with
    // `replay_of`; dropping the witness there went unnoticed by every test.
    let first = first_read().await;
    let witness = first.witness.clone().expect("a divided read has a witness");
    match WindowPlanSource::replay_of(first.reads.clone(), first.witness) {
        WindowPlanSource::Replay {
            parts,
            witness: Some(carried),
        } => {
            assert_eq!(parts, first.reads);
            assert_eq!(carried, witness);
        }
        _ => panic!("the replay must carry the witness"),
    }
}
#[test]
fn a_bound_refusal_with_a_concrete_next_step_names_it() {
    // The too-large book must say that narrowing the window does not help —
    // the retry a caller would otherwise try first.
    let book = refusal_remediation(BOOK_TOO_LARGE).expect("guidance");
    assert!(
        book.contains("A shorter date window will not help"),
        "{book}"
    );
    let post = refusal_remediation("import_post_window_not_bounded").expect("guidance");
    assert!(
        post.contains("Build the batch again over fewer days"),
        "{post}"
    );
}

#[tokio::test]
async fn a_replay_reads_exactly_the_parts_it_was_given() {
    // Review of #520: the first read counted one voucher on each of days 1-3,
    // read day 1, planned days 2-4 as one part, and divided it into 2-3 and 4
    // after Tally could not serve it. Before this, the replay re-planned from
    // its first measurement and sent the 2-4 request the first read had just
    // found unservable. It now reads the first read's parts, in order.
    let shape = VoucherReadShape::ImportVerification;
    let window = ("20260801", "20260804");
    let census = relabelled(
        &three_vouchers(),
        &[(1, "20260801"), (2, "20260802"), (3, "20260803")],
    );
    let d1 = xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")]));
    let d23 = xml_plan(relabelled(
        &vouchers_kept(2),
        &[(2, "20260802"), (3, "20260803")],
    ));
    let mut plans = paired(&xml_plan(census));
    plans.extend(paired(&d1));
    plans.extend(oversized());
    plans.extend(paired(&d23));
    plans.extend(paired(&xml_plan(empty_collection())));
    plans.extend(paired(&mark(3)));
    let (first, _) = read_window(
        plans,
        window,
        shape,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
    )
    .await;
    let first = first.unwrap();
    assert_eq!(
        first.reads,
        [
            part("20260801", "20260801", None),
            part("20260802", "20260803", None),
            part("20260804", "20260804", None),
        ]
    );
    let mut plans = paired(&d1);
    plans.extend(paired(&d23));
    plans.extend(paired(&xml_plan(empty_collection())));
    plans.extend(paired(&mark(3)));
    let (replay, observed) = read_window(
        plans,
        window,
        shape,
        WindowPlanSource::Replay {
            parts: first.reads.clone(),
            witness: first.witness.clone(),
        },
        three_a_read(),
    )
    .await;
    assert_eq!(replay.unwrap().reads, first.reads);
    assert_requests(
        &observed,
        &[1, 7, 13],
        &first
            .reads
            .iter()
            .map(|read| {
                shape
                    .render(&company(), &read.from, &read.to, read.span)
                    .unwrap()
            })
            .collect::<Vec<_>>(),
    );
}

#[tokio::test]
async fn a_replay_refuses_a_master_mark_that_moved_since_the_first_read() {
    // A ledger renamed between the first read and its replay changes the
    // replayed exports only through names; the master mark is what shows it.
    let first = first_read().await;
    let (_, parts) = divided_first_read();
    let mut plans = Vec::new();
    for part in &parts {
        plans.extend(paired(part));
    }
    plans.extend(paired(&marks_plan(3, 8)));
    let (outcome, _) = read_window(
        plans,
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Replay {
            parts: first.reads,
            witness: first.witness,
        },
        three_a_read(),
    )
    .await;
    assert_eq!(
        outcome.err().map(|failure| failure.code).as_deref(),
        Some(WINDOW_CHANGED_DURING_READ)
    );
}

#[tokio::test]
async fn a_read_after_tally_refused_the_whole_window_does_not_admit_it_whole() {
    // Re-check of #520 review: verification planned the window whole, Tally
    // refused that request as too large, and the two days were read instead.
    // The parts summed well under the budget, and before this the pre-post
    // check admitted the very request Tally had just refused. A deadline would
    // be the same case with no size at all to sum.
    let shape = VoucherReadShape::ImportVerification;
    let mut plans = oversized();
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(empty_collection())));
    plans.extend(paired(&mark(1)));
    let (outcome, _) = read_window(
        plans,
        ("20260801", "20260802"),
        shape,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(1)),
        },
        WindowReadLimits::for_shape(shape),
    )
    .await;
    let read = outcome.unwrap();
    assert!(read.refused_a_part);
    // #595: the refused request is timed as a part Tally did not serve, and
    // the read that stands reports no failure.
    assert!(!read.timings.parts[0].served);
    assert_eq!(read.timings.parts[0].bytes, None);
    assert!(read.timings.parts[1..].iter().all(|part| part.served));
    assert_eq!(read.timings.failed, None);
    let served = WindowServed::of(&read.reads, &read.evidence, read.refused_a_part);
    assert!(served.data_bytes <= WINDOW_READ_BUDGET_BYTES);
    assert!(!served.fits_one_request());
    // Control: the same parts, read without a refusal, would be admitted.
    assert!(WindowServed::of(&read.reads, &read.evidence, false).fits_one_request());
}

/// Three parts of one day-divided read, as in
/// `the_first_part_measures_the_book_and_the_rest_of_its_day_is_read_above_it`,
/// with the first part's report leg held so a withdrawal lands while it runs.
fn three_part_plans(hold_first_part: bool) -> Vec<ScenarioPlan> {
    let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
    if hold_first_part {
        plans[1] = plans[1]
            .clone()
            .with_delivery(tally_protocol_simulator::Delivery::SlowHeaders(
                std::time::Duration::from_millis(900),
            ));
    }
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(2, "20260801"), (3, "20260801")],
    ))));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(4, "20260802"), (5, "20260802")],
    ))));
    plans
}

fn three_part_census() -> WindowCensus {
    WindowCensus::from_rows([
        (day("20260801"), 1),
        (day("20260801"), 2),
        (day("20260801"), 3),
        (day("20260802"), 4),
        (day("20260802"), 5),
    ])
}

async fn read_three_parts_under(
    cancellation: tokio_util::sync::CancellationToken,
    withdraw_after: Option<std::time::Duration>,
) -> (
    Result<WindowReadOutcome<Value>, ToolFailure>,
    Vec<tally_protocol_simulator::ObservedRequest>,
) {
    let simulator = SequenceSimulator::spawn(three_part_plans(withdraw_after.is_some())).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let identity = identity();
    if let Some(delay) = withdraw_after {
        let withdraw = cancellation.clone();
        tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            withdraw.cancel();
        });
    }
    let outcome = crate::tally::runtime::TOOL_CANCELLATION
        .scope(
            cancellation,
            server.read_voucher_window(
                &identity,
                identity.display_name(),
                &tally_date("20260801"),
                &tally_date("20260802"),
                VoucherReadShape::EntryWildcard,
                WindowPlanSource::Counted(three_part_census()),
                three_a_read(),
                |xml| parse_agent_rows(xml, GUID),
            ),
        )
        .await;
    simulator.cancel();
    // `cancel` wakes the simulator with an empty connection; keep only the
    // requests Bridge actually sent.
    let sent = simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .collect();
    (outcome, sent)
}

#[tokio::test]
async fn a_withdrawal_during_one_part_sends_no_further_part() {
    // #554. The withdrawal lands while the first part's report leg is held. That
    // part runs to completion (its six legs: abandoning a request does not stop
    // Tally), and the next part is never sent. The read is refused as withdrawn
    // with the first part's evidence kept, never returned as a partial window.
    let (outcome, observed) = read_three_parts_under(
        tokio_util::sync::CancellationToken::new(),
        Some(std::time::Duration::from_millis(200)),
    )
    .await;
    assert_eq!(observed.len(), 6, "only the part in flight was sent");
    let failure = outcome.err().expect("a withdrawn read is refused");
    assert_eq!(failure.code, "request_cancelled");
    // The part already read is accounted for; the response layer marks the
    // refusal's evidence partial (see the stdio test).
    let evidence = failure
        .evidence
        .expect("the part already read is accounted");
    assert!(!evidence.response_sha256.is_empty() && evidence.bytes > 0);
}

#[tokio::test]
async fn a_read_whose_withdrawal_never_comes_reads_every_part() {
    // Control: the same read under a cancellation that is never fired.
    let (outcome, observed) =
        read_three_parts_under(tokio_util::sync::CancellationToken::new(), None).await;
    assert_eq!(
        outcome.expect("an unwithdrawn read completes").rows.len(),
        5
    );
    assert_eq!(observed.len(), 18);
}

// --- Education date boundaries (#581) ---------------------------------------

/// The captured licensed company list with Tally's `EDUMODE` flag set, which
/// is how the list reports an instance in Education mode. A deliberate
/// synthetic mutation of the captured bytes; the flag is the only change.
fn education_company_plan() -> ScenarioPlan {
    let licensed = captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-companies.utf16le.xml"
    ));
    let education = licensed.replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    assert_ne!(education, licensed);
    xml_plan(education)
}

/// A capture of what Education serves for a movement or voucher read that
/// starts on a day other than the 1st, 2nd or 31st (an empty collection).
fn education_empty_part_plan() -> ScenarioPlan {
    xml_plan(captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-education-empty-movement-part.utf16le.xml"
    )))
}

/// One paired, identity-bracketed read with the Education company list.
fn education_paired(body: &ScenarioPlan) -> Vec<ScenarioPlan> {
    vec![
        education_company_plan(),
        body.clone(),
        status_plan(),
        body.clone(),
        status_plan(),
        education_company_plan(),
    ]
}

/// The vouchers tool over a window Education cannot serve: it starts on the
/// 5th. Before bridge#581 the part was sent, Education answered it with its
/// captured empty collection, and the tool reported an empty window marked
/// `partial / empty_uncorroborated` after 28 requests. Now the mark read's own
/// identity bracket reports Education, and the plan is refused by name before
/// anything of the part is sent. A small book is counted first (#1029), so the
/// refusal now comes from the census request at the transport guard, one request
/// later, and still before any part is sent.
#[tokio::test]
async fn education_refuses_a_window_starting_on_an_unaccepted_day_before_sending_it() {
    let mut plans = vec![
        education_company_plan(),
        status_plan(),
        education_company_plan(),
        status_plan(),
    ];
    plans.extend(education_paired(&mark(3)));
    // The census of a small book (#1029) is a read of the same window, so it
    // is refused the same way: only its opening identity leg is spent.
    plans.extend(education_paired(&education_empty_part_plan()));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let response = server
        .call_tool(
            "vouchers",
            json!({"company_guid": GUID, "from": "20260405", "to": "20260405"}),
        )
        .await;
    simulator.cancel();
    let observed = simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"],
        "window_part_boundary_unsupported_in_education",
        "{response}"
    );
    // 4 prelude legs, the marks read (six), then the census's opening identity
    // leg alone: the census is refused before its request is sent.
    assert_eq!(observed.len(), 11);
    let census = render_agent_voucher_census(
        &company(),
        &tally_date("20260405"),
        &tally_date("20260405"),
        None,
    )
    .unwrap();
    assert!(observed
        .iter()
        .all(|request| request.request_body_sha256 != request_sha(&census)));
    let part = [
        render_agent_vouchers(
            &company(),
            &tally_date("20260405"),
            &tally_date("20260405"),
            None,
        )
        .unwrap(),
        render_agent_vouchers_in_span(
            &company(),
            &tally_date("20260405"),
            &tally_date("20260405"),
            None,
        )
        .unwrap(),
    ];
    assert!(observed.iter().all(|request| part
        .iter()
        .all(|xml| request.request_body_sha256 != request_sha(xml))));
}

/// The control: the same read on a licensed endpoint is sent as before, and
/// its empty window is corroborated as it always was.
#[tokio::test]
async fn a_licensed_endpoint_still_reads_a_window_starting_on_any_day() {
    let empty = education_empty_part_plan();
    let mut plans = vec![company_plan(), status_plan(), company_plan(), status_plan()];
    plans.extend(paired(&mark(3)));
    // The census (#1029), the window read, the corroboration's wider read, and
    // its marks read: the census is the one read more than before.
    plans.extend(paired(&empty));
    plans.extend(paired(&empty));
    plans.extend(paired(&empty));
    plans.extend(paired(&mark(3)));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let response = server
        .call_tool(
            "vouchers",
            json!({"company_guid": GUID, "from": "20260405", "to": "20260405"}),
        )
        .await;
    simulator.cancel();
    let sent = simulator
        .finish()
        .unwrap()
        .iter()
        .filter(|request| !request.method.is_empty())
        .count();
    assert_eq!(sent, 34);
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "partial", "{response}");
    assert_eq!(result["reason"], "empty_uncorroborated");
}

/// A divided plan is refused whole, before its first part (bridge#581). The
/// census counts one voucher on each of the 1st, 2nd and 10th and one voucher
/// fits a read, so the plan is 1st, 2nd to 9th, 10th to 31st. Its first part
/// is one Education serves; its second is not. Checking only as each part is
/// sent would read the first part and then refuse; the census's own bracket
/// has already reported Education, so nothing is read.
#[tokio::test]
async fn an_education_plan_with_an_unaccepted_part_boundary_is_refused_before_any_part() {
    let census = relabelled(
        &three_vouchers(),
        &[(1, "20260801"), (2, "20260802"), (3, "20260810")],
    );
    let (outcome, observed) = read_window(
        education_paired(&xml_plan(census)),
        ("20260801", "20260831"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
    )
    .await;
    let failure = outcome.err().expect("the plan is refused");
    assert_eq!(failure.code, EDUCATION_BOUNDARY_UNSUPPORTED);
    assert!(failure.evidence.is_some(), "the census is accounted for");
    assert_eq!(observed.len(), 6);
    assert_requests(
        &observed,
        &[1],
        &[render_agent_voucher_census(
            &company(),
            &tally_date("20260801"),
            &tally_date("20260831"),
            None,
        )
        .unwrap()],
    );
}

/// The same census on a licensed endpoint plans the parts the Education test
/// refuses, and reads them: the control that the refusal is the mode's.
#[test]
fn the_refused_education_plan_divides_on_days_education_does_not_honour() {
    let limits = three_a_read();
    let plan = plan_window_reads(
        day("20260801"),
        day("20260831"),
        &census_of(&[("20260801", 1), ("20260802", 1), ("20260810", 1)]),
        None,
        3,
        limits.default_bytes_per_voucher,
        limits.budget_bytes,
        limits.max_reads,
    )
    .unwrap();
    assert_eq!(
        stack_of(&plan)
            .unwrap()
            .into_iter()
            .rev()
            .collect::<Vec<_>>(),
        [
            part("20260801", "20260801", None),
            part("20260802", "20260809", None),
            part("20260810", "20260831", None),
        ]
    );
}

/// Without a preflight read the executor has not yet observed the mode, so the
/// runtime refuses the part itself: after its opening identity bracket reports
/// Education and before its data request is sent.
#[tokio::test]
async fn the_runtime_refuses_an_education_read_the_executor_could_not_check() {
    let (outcome, observed) = read_window(
        vec![education_company_plan()],
        ("20260805", "20260805"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census_of(&[("20260805", 1)])),
        three_a_read(),
    )
    .await;
    let failure = outcome.err().expect("the read is refused");
    assert_eq!(failure.code, EDUCATION_BOUNDARY_UNSUPPORTED);
    assert_eq!(
        observed
            .iter()
            .filter(|request| !request.method.is_empty())
            .count(),
        1
    );
}

/// Education seen on any read of a window holds for the rest of it; a later
/// licensed observation never relaxes it.
#[test]
fn an_observed_education_profile_is_never_relaxed_within_a_window() {
    use DateBoundaryProfile::{EducationRestricted, ModeAgnostic};
    for (observations, expected) in [
        (vec![], None),
        (vec![ModeAgnostic], Some(ModeAgnostic)),
        (vec![EducationRestricted], Some(EducationRestricted)),
        (
            vec![ModeAgnostic, EducationRestricted],
            Some(EducationRestricted),
        ),
        (
            vec![EducationRestricted, ModeAgnostic],
            Some(EducationRestricted),
        ),
    ] {
        let mut boundary = None;
        for observed in observations.iter().copied() {
            observe_boundary(&mut boundary, observed);
        }
        assert_eq!(boundary, expected, "{observations:?}");
    }
}

/// The end of a part is held to the rule as well as its start. Education's
/// treatment of an end day is not measured, so a whole-month window ending on
/// the 30th is refused: the accepted price of admitting only measured
/// boundaries (bridge#581).
#[tokio::test]
async fn an_education_window_ending_on_an_unaccepted_day_is_refused_before_it_is_sent() {
    let (outcome, observed) = read_window(
        education_paired(&mark(1)),
        ("20260901", "20260930"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate { known_marks: None },
        three_a_read(),
    )
    .await;
    let failure = outcome.err().expect("the window is refused");
    assert_eq!(failure.code, EDUCATION_BOUNDARY_UNSUPPORTED);
    assert_eq!(observed.len(), 6, "only the mark read is sent");
}

/// Each way a part fails admission is named beside `voucher_window_part_not_admitted`
/// (bridge#581), and a census disagreement carries its counts: an Education
/// part served empty reads as "returned 0 of 2", not as a bare refusal.
#[test]
fn a_part_not_admitted_names_its_cause_and_a_census_mismatch_its_counts() {
    // AlterIDs 1 and 2 on the 1st, 3 on the 2nd.
    let census = census_of(&[("20260801", 2), ("20260802", 1)]);
    let row = |alter_id: u64, date: &str| json!({"date": date, "alter_id": alter_id});
    let day_one = part("20260801", "20260801", None);
    let first_only = part(
        "20260801",
        "20260801",
        Some(AlterIdSpan {
            after: 0,
            through: 1,
        }),
    );
    let (from, to) = (tally_date("20260801"), tally_date("20260802"));
    let window = (&from, &to);
    for (part, rows, expected) in [
        (&day_one, vec![], Err((PART_CENSUS_MISMATCH, Some((0, 2))))),
        (
            &day_one,
            vec![row(1, "20260801"), row(2, "20260801")],
            Ok(()),
        ),
        (
            &day_one,
            vec![row(1, "20260801"), row(2, "20260801"), row(4, "20260801")],
            Err((PART_CENSUS_MISMATCH, Some((3, 2)))),
        ),
        (
            &day_one,
            vec![row(1, "20260801"), row(9, "20260801")],
            Err((PART_CENSUS_MISMATCH, Some((2, 2)))),
        ),
        (
            &day_one,
            vec![json!({"alter_id": 1})],
            Err((PART_ROW_UNREADABLE, None)),
        ),
        (
            &day_one,
            vec![json!({"date": "20260801"})],
            Err((PART_ROW_UNREADABLE, None)),
        ),
        (
            &day_one,
            vec![row(1, "20260801"), row(3, "20260802")],
            Err((PART_ROW_OUTSIDE_DATES, None)),
        ),
        (
            &day_one,
            vec![row(1, "20260801"), row(1, "20260801")],
            Err((PART_ROW_DUPLICATED, None)),
        ),
        (
            &first_only,
            vec![row(2, "20260801")],
            Err((PART_ROW_OUTSIDE_ALTER_ID_SPAN, None)),
        ),
    ] {
        let outcome = admit_part(part, window, &rows, Some(&census)).map_err(|failure| {
            assert_eq!(failure.code, PART_NOT_ADMITTED);
            (
                failure.cause.expect("a named cause"),
                failure
                    .counts
                    .map(|counts| (counts.returned, counts.counted)),
            )
        });
        assert_eq!(outcome, expected, "{rows:?}");
    }
}

/// The counts reach the caller beside the cause, through the tool's own error.
#[tokio::test]
async fn a_census_mismatch_reaches_the_caller_with_its_counts() {
    // A census of 50 vouchers on one day, more than one read carries at the
    // shipped limits, so the plan's first part is an AlterID span. Tally
    // answers it empty.
    let limits = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard);
    let per_read = limits.budget_bytes / limits.default_bytes_per_voucher;
    let total = per_read + 8;
    let one = vouchers_kept(1);
    let start = one.find("<VOUCHER ").unwrap();
    let end = start + one[start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let many = format!(
        "{}{}{}",
        &one[..start],
        one[start..end].repeat(total as usize),
        &one[end..]
    );
    let ids = (1..=total).map(|id| (id, "20260801")).collect::<Vec<_>>();
    let census = relabelled(&many, &ids);
    let mut plans = vec![company_plan(), status_plan(), company_plan(), status_plan()];
    plans.extend(paired(&marks_plan(total, 7)));
    plans.extend(paired(&xml_plan(census)));
    plans.extend(paired(&xml_plan(empty_collection())));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let response = server
        .call_tool(
            "vouchers",
            json!({"company_guid": GUID, "from": "20260801", "to": "20260801"}),
        )
        .await;
    simulator.cancel();
    let sent = simulator
        .finish()
        .unwrap()
        .iter()
        .filter(|request| !request.method.is_empty())
        .count();
    assert_eq!(sent, 22);
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], PART_NOT_ADMITTED, "{response}");
    assert_eq!(error["cause"], PART_CENSUS_MISMATCH);
    assert_eq!(error["counts"], json!({"returned": 0, "counted": per_read}));
}

/// #985: a window is `counted()` only against a census that names each
/// voucher's GUID. A count of `(day, AlterID)` alone admits the same rows on
/// AlterIDs only, so the read is not called counted.
#[tokio::test]
async fn only_a_census_with_guids_makes_a_window_counted() {
    let window = ("20260801", "20260801");
    let with_guids = WindowCensus::from_census_rows(
        parse_voucher_census(&three_vouchers(), (day(window.0), day(window.1)), None).unwrap(),
    )
    .unwrap();
    let ids_only = WindowCensus::from_rows((1..=3).map(|id| (day("20260801"), id)));
    for (census, counted) in [(with_guids, true), (ids_only, false)] {
        let (outcome, _) = read_window(
            paired(&xml_plan(three_vouchers())),
            window,
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::replay_of(
                vec![part("20260801", "20260801", None)],
                Some(WindowWitness {
                    marks: marks_of(3),
                    census: Some(census),
                }),
            ),
            WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard),
        )
        .await;
        let outcome = outcome.unwrap();
        assert_eq!(outcome.rows.len(), 3);
        assert_eq!(outcome.counted(), counted);
    }
}

/// The `vouchers` plans of a book whose mark needs a census (#985): `census`
/// counts the window, which then fits one read, and `data` is that read.
fn counted_vouchers_plans(census: String, data: String) -> Vec<ScenarioPlan> {
    let limits = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard);
    let mut plans = vec![company_plan(), status_plan(), company_plan(), status_plan()];
    plans.extend(paired(&mark(limits.census_capacity())));
    plans.extend(paired(&xml_plan(census)));
    plans.extend(paired(&xml_plan(data)));
    plans
}

async fn call_vouchers_over(plans: Vec<ScenarioPlan>) -> Value {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let response = server_at(simulator.address(), directory.path())
        .call_tool(
            "vouchers",
            json!({"company_guid": GUID, "from": "20260801", "to": "20260831"}),
        )
        .await;
    simulator.finish().unwrap();
    response
}

/// #985, #1029: one rule labels the window. A window the census counted is
/// `complete`, and the `vouchers` tool counts every book that has held a
/// voucher, including one whose mark alone shows it fits a request, so both
/// are `complete`.
#[tokio::test]
async fn vouchers_labels_a_window_complete_whether_the_book_is_large_or_small() {
    let large =
        call_vouchers_over(counted_vouchers_plans(three_vouchers(), three_vouchers())).await;
    assert_eq!(large["isError"], false, "{large}");
    let result = &large["structuredContent"]["result"];
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["reason"], Value::Null);
    assert_eq!(result["total"], 3);
    assert_eq!(large["structuredContent"]["evidence"]["state"], "complete");

    // A mark of 3 fits one request, yet the census still counts the window.
    let mut plans = vec![company_plan(), status_plan(), company_plan(), status_plan()];
    plans.extend(paired(&mark(3)));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let small = call_vouchers_over(plans).await;
    assert_eq!(small["isError"], false, "{small}");
    let result = &small["structuredContent"]["result"];
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["reason"], Value::Null);
    assert_eq!(result["total"], 3);
    assert_eq!(small["structuredContent"]["evidence"]["state"], "complete");
}

/// #985: a window read whole after a census is admitted against it, so a read
/// that returns fewer vouchers than were counted is refused, not returned
/// short, and the refusal names the next call.
#[tokio::test]
async fn a_window_counted_whole_refuses_a_read_short_of_its_census() {
    let response =
        call_vouchers_over(counted_vouchers_plans(three_vouchers(), vouchers_kept(2))).await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], PART_NOT_ADMITTED, "{response}");
    assert_eq!(error["cause"], PART_CENSUS_MISMATCH);
    assert_eq!(error["counts"], json!({"returned": 2, "counted": 3}));
    assert!(
        error["remediation"]
            .as_str()
            .is_some_and(|text| text.contains("call the same tool again")),
        "{error}"
    );
}

/// #1029: the same refusal on a SMALL book. A mark of 3 fits one request, yet the
/// window is counted, and a read that returns fewer vouchers than the count named
/// is refused with the typed code, cause and counts, as for a large book.
#[tokio::test]
async fn a_small_books_window_counted_whole_refuses_a_read_short_of_its_count() {
    let mut plans = vec![company_plan(), status_plan(), company_plan(), status_plan()];
    plans.extend(paired(&mark(3)));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(vouchers_kept(2))));
    let response = call_vouchers_over(plans).await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], PART_NOT_ADMITTED, "{response}");
    assert_eq!(error["cause"], PART_CENSUS_MISMATCH, "{response}");
    assert_eq!(
        error["counts"],
        json!({"returned": 2, "counted": 3}),
        "{response}"
    );
}

/// A licence that drops to Education while a read is in flight: the opening
/// bracket said licensed, the closing one says Education. Which mode served
/// the read is unknown, so a read Education would have served empty is refused,
/// with the read it made accounted for (bridge#581).
#[tokio::test]
async fn education_reported_after_a_read_refuses_a_boundary_education_serves_empty() {
    let body = xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260805")]));
    let (outcome, observed) = read_window(
        vec![
            company_plan(),
            body.clone(),
            status_plan(),
            body,
            status_plan(),
            education_company_plan(),
        ],
        ("20260805", "20260805"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census_of(&[("20260805", 1)])),
        three_a_read(),
    )
    .await;
    let failure = outcome.err().expect("the read is refused");
    assert_eq!(failure.code, EDUCATION_BOUNDARY_UNSUPPORTED);
    assert!(failure.evidence.is_some(), "the data read is accounted for");
    assert_eq!(observed.len(), 6);
}

/// An Education list whose other capability fields do not parse still
/// restricts: an empty `SILVER` must not switch the guard off.
#[tokio::test]
async fn education_is_observed_when_another_capability_field_is_empty() {
    let mut plan = education_company_plan();
    let body = plan.fixture.body().into_owned();
    let broken = body.replace(
        "<SILVER TYPE=\"Logical\">Yes</SILVER>",
        "<SILVER TYPE=\"Logical\"/>",
    );
    assert_ne!(broken, body);
    assert!(bridge_tally_protocol::parse_company_gateway_capability_observation(&broken).is_err());
    plan = xml_plan(broken);
    let (outcome, observed) = read_window(
        vec![plan],
        ("20260805", "20260805"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census_of(&[("20260805", 1)])),
        three_a_read(),
    )
    .await;
    assert_eq!(
        outcome.err().map(|failure| failure.code).as_deref(),
        Some(EDUCATION_BOUNDARY_UNSUPPORTED)
    );
    assert_eq!(
        observed
            .iter()
            .filter(|request| !request.method.is_empty())
            .count(),
        1
    );
}

/// A census over a window Education cannot serve is refused by the runtime
/// under its own code, not relabelled as an unestimated window.
#[tokio::test]
async fn a_census_education_cannot_serve_is_refused_by_name() {
    let (outcome, observed) = read_window(
        vec![education_company_plan()],
        ("20260805", "20260831"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
    )
    .await;
    assert_eq!(
        outcome.err().map(|failure| failure.code).as_deref(),
        Some(EDUCATION_BOUNDARY_UNSUPPORTED)
    );
    assert_eq!(
        observed
            .iter()
            .filter(|request| !request.method.is_empty())
            .count(),
        1
    );
}

// --- Window readers (audit_read plan step 5b) ---

/// The three legs of one audit part: identity bracket, one unpaired read,
/// identity bracket.
fn single(body: &ScenarioPlan) -> Vec<ScenarioPlan> {
    vec![company_plan(), body.clone(), company_plan()]
}

/// The plans of `a_divided_read_is_bracketed_by_the_high_water_mark`, in the
/// order its reads are sent, with `legs` shaping each read.
fn divided_window_reads(closing: ScenarioPlan) -> Vec<ScenarioPlan> {
    vec![
        xml_plan(three_vouchers()),
        xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])),
        xml_plan(relabelled(
            &vouchers_kept(2),
            &[(2, "20260801"), (3, "20260801")],
        )),
        xml_plan(empty_collection()),
        closing,
    ]
}

fn divided_parts() -> Vec<WindowPart> {
    vec![
        part(
            "20260801",
            "20260801",
            Some(AlterIdSpan {
                after: 0,
                through: 1,
            }),
        ),
        part(
            "20260801",
            "20260801",
            Some(AlterIdSpan {
                after: 1,
                through: 3,
            }),
        ),
        part("20260802", "20260802", None),
    ]
}

async fn read_audit_window(
    plans: Vec<ScenarioPlan>,
    runtime: TallyRuntime,
) -> (
    Result<WindowReadOutcome<Value>, ToolFailure>,
    Option<Vec<RetainedWindowRead>>,
    Option<AuditWindowFailure>,
    Vec<tally_protocol_simulator::ObservedRequest>,
    TallyConfig,
) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let config = TallyConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    };
    let identity = identity();
    // Through the owned entry point, as the orchestrator will read.
    let read = AuditWindowReader::new(&runtime, config.clone())
        .read_window(
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260802"),
            VoucherReadShape::EntryWildcard,
            three_a_read(),
            |xml| parse_agent_rows(xml, GUID),
        )
        .await;
    (
        read.outcome,
        read.retained,
        read.failure,
        simulator.finish().unwrap(),
        config,
    )
}

/// An undivided window for a sealed record is bracketed too: its record states
/// the marks it read, so it reads them again after its one part and refuses
/// if they moved, where an agent read of the same window would not look again.
#[tokio::test]
async fn an_undivided_audit_window_closes_its_bracket() {
    let one = || xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")]));
    let (outcome, retained, failure, observed, _) = read_audit_window(
        [marks_plan(1, 7), one(), marks_plan(1, 7)]
            .iter()
            .flat_map(single)
            .collect(),
        TallyRuntime::default(),
    )
    .await;
    let read = outcome.expect("an unchanged undivided window reads");
    assert!(failure.is_none());
    assert_eq!(read.reads, [part("20260801", "20260802", None)]);
    assert_eq!(observed.len(), 9);
    assert_eq!(
        retained
            .expect("its reads")
            .iter()
            .map(|r| r.kind.clone())
            .collect::<Vec<_>>(),
        [
            WindowReadKind::Marks,
            WindowReadKind::Part(part("20260801", "20260802", None)),
            WindowReadKind::Marks,
        ]
    );

    let (moved, retained, failure, _, _) = read_audit_window(
        [marks_plan(1, 7), one(), marks_plan(2, 7)]
            .iter()
            .flat_map(single)
            .collect(),
        TallyRuntime::default(),
    )
    .await;
    assert_eq!(
        moved.err().map(|f| f.code),
        Some(WINDOW_CHANGED_DURING_READ.to_string())
    );
    assert_eq!(failure, Some(AuditWindowFailure::WindowChanged));
    assert!(retained.is_none());
}

/// `Server::read_voucher_window` and an explicit [`AgentReader`] send the same
/// requests in the same order, so the two entry points cannot drift apart.
/// Both run the one executor, so this cannot show that the agent path is
/// unchanged from before readers existed. The pre-existing window tests show
/// that: they pass unmodified.
#[tokio::test]
async fn the_agent_reader_sends_exactly_the_requests_the_window_read_sent() {
    let plans = || {
        divided_window_reads(marks_plan(3, 7))
            .iter()
            .flat_map(paired)
            .collect::<Vec<_>>()
    };
    let (through_server, server_observed) = read_window(
        plans(),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
    )
    .await;
    let simulator = SequenceSimulator::spawn(plans()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let identity = identity();
    let through_reader = read_voucher_window_with(
        &AgentReader(&server),
        &identity,
        identity.display_name(),
        &tally_date("20260801"),
        &tally_date("20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
        |xml| parse_agent_rows(xml, GUID),
    )
    .await;
    let reader_observed = simulator.finish().unwrap();
    assert_eq!(through_server.unwrap().reads, through_reader.unwrap().reads);
    let shas = |observed: &[tally_protocol_simulator::ObservedRequest]| {
        observed
            .iter()
            .map(|request| request.request_body_sha256.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(shas(&server_observed), shas(&reader_observed));
}

/// A divided window through the audit reader: the same plan, admission and
/// bracket as the agent path, each request one unpaired audit part, and every
/// admitted response kept byte for byte, in order.
#[tokio::test]
async fn an_audit_window_keeps_every_read_it_admitted_as_it_arrived() {
    // The audit window reads its own opening marks first.
    let reads = [
        vec![marks_plan(3, 7)],
        divided_window_reads(marks_plan(3, 7)),
    ]
    .concat();
    let wires = reads
        .iter()
        .map(|plan| tally_protocol_simulator::encode(&plan.fixture.body(), plan.encoding))
        .collect::<Vec<_>>();
    let (outcome, retained, failure, observed, _) = read_audit_window(
        reads.iter().flat_map(single).collect(),
        TallyRuntime::default(),
    )
    .await;
    let read = outcome.expect("the window reads");
    assert!(failure.is_none());
    let retained = retained.expect("a completed window yields its reads");
    assert_eq!(read.rows.len(), 3);
    assert_eq!(read.reads, divided_parts());
    // Divided, counted by GUID and closed on its marks, but read by single
    // reads: no token, so nothing is skipped on its account (#1241).
    assert!(read.bracketed.is_none());
    // Opening marks, census, three parts, closing marks: six reads of three
    // legs each.
    assert_eq!(observed.len(), 18);
    let kinds = retained.iter().map(|r| r.kind.clone()).collect::<Vec<_>>();
    let mut expected = vec![WindowReadKind::Marks, WindowReadKind::Census];
    expected.extend(divided_parts().into_iter().map(WindowReadKind::Part));
    expected.push(WindowReadKind::Marks);
    assert_eq!(kinds, expected);
    for (index, (kept, wire)) in retained.iter().zip(&wires).enumerate() {
        assert_eq!(&kept.part.encoded_body, wire, "read {index}");
        assert_eq!(
            kept.request_sha256,
            observed[index * 3 + 1].request_body_sha256
        );
        assert_eq!(
            kept.part.boundary_profile,
            DateBoundaryProfile::ModeAgnostic
        );
    }
}

/// The reader never changes admission: a part that returns fewer vouchers than
/// the census counted is refused through the audit reader exactly as through
/// the agent path, and is not a transport failure of the part.
#[tokio::test]
async fn an_audit_window_refuses_a_part_the_census_disagrees_with() {
    let mut reads = divided_window_reads(marks_plan(3, 7));
    reads[1] = xml_plan(empty_collection());
    let (agent, _) = read_window(
        reads.iter().take(2).flat_map(paired).collect(),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        three_a_read(),
    )
    .await;
    let agent = agent.err().expect("the agent path refuses");
    let (audit, retained, failure, observed, _) = read_audit_window(
        std::iter::once(marks_plan(3, 7))
            .chain(reads.iter().take(2).cloned())
            .flat_map(|plan| single(&plan))
            .collect(),
        TallyRuntime::default(),
    )
    .await;
    let audit = audit.err().expect("the audit path refuses");
    assert_eq!(audit.code, PART_NOT_ADMITTED);
    assert_eq!(audit.code, agent.code);
    assert_eq!(audit.cause, agent.cause);
    assert_eq!(audit.counts, agent.counts);
    assert_eq!(
        failure,
        Some(AuditWindowFailure::Refused(PART_NOT_ADMITTED.to_string()))
    );
    assert!(!failure.unwrap().retryable());
    // The opening marks, the census and the refused part were read, nothing
    // after, and a refused window yields no retained reads at all.
    assert!(retained.is_none());
    assert_eq!(observed.len(), 9);
}

/// Someone else writing is the normal case on a multi-user book: marks that
/// move across a divided window refuse it, typed as retryable.
#[tokio::test]
async fn an_audit_window_the_book_changed_under_is_retryable() {
    for closing in [marks_plan(4, 7), marks_plan(3, 8)] {
        let (outcome, _, failure, _, _) = read_audit_window(
            std::iter::once(marks_plan(3, 7))
                .chain(divided_window_reads(closing))
                .flat_map(|plan| single(&plan))
                .collect(),
            TallyRuntime::default(),
        )
        .await;
        assert_eq!(
            outcome.err().expect("changed").code,
            WINDOW_CHANGED_DURING_READ
        );
        assert_eq!(failure, Some(AuditWindowFailure::WindowChanged));
        assert!(failure.unwrap().retryable());
    }
}

/// A part whose connection drops stops the window at that part, typed with the
/// part's kind, and the endpoint then owes a drain: no later window reads.
#[tokio::test]
async fn an_audit_window_stops_at_a_dropped_part_and_owes_a_drain() {
    let mut reads = divided_window_reads(marks_plan(3, 7));
    reads[2] = reads[2].clone().with_delivery(
        tally_protocol_simulator::Delivery::ResetAfterRequestProcessed {
            delay: std::time::Duration::ZERO,
        },
    );
    let runtime = TallyRuntime::default();
    let (outcome, retained, failure, observed, config) = read_audit_window(
        single(&marks_plan(3, 7))
            .into_iter()
            .chain(reads.iter().take(3).flat_map(single).take(8))
            .collect(),
        runtime.clone(),
    )
    .await;
    assert!(outcome.is_err());
    assert_eq!(
        failure,
        Some(AuditWindowFailure::Part(
            crate::tally::runtime::AuditPartFailureKind::ConnectionDropped
        ))
    );
    assert!(failure.unwrap().retryable());
    // The marks, the census and the first part arrived before the drop, but a
    // failed window yields no retained reads.
    assert!(retained.is_none());
    assert_eq!(observed.len(), 11);
    // A second window to the same endpoint is refused before sending anything:
    // the drain debt is keyed by endpoint, and the check precedes any request,
    // so it holds even with nothing listening there any more.
    let reader = AuditWindowReader::new(&runtime, config);
    let identity = identity();
    let again = read_voucher_window_with(
        &reader,
        &identity,
        identity.display_name(),
        &tally_date("20260801"),
        &tally_date("20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate { known_marks: None },
        three_a_read(),
        |xml| parse_agent_rows(xml, GUID),
    )
    .await;
    let again_failure = again
        .as_ref()
        .err()
        .map(|failure| audit_window_failure(failure, &reader));
    assert!(again.is_err());
    assert!(reader.into_retained(&again).is_none());
    assert_eq!(
        again_failure,
        Some(AuditWindowFailure::Part(
            crate::tally::runtime::AuditPartFailureKind::DrainRequired
        ))
    );
}

/// A later part is planned at the cost the first part measured, and the audit
/// reader measures it the same as the agent reader, although it reads each part
/// once where the agent reads it twice. Day two's four vouchers must go in two
/// parts through either reader: planned at half their real cost, the audit
/// reader would ask for all four in one part, twice the budget.
#[tokio::test]
async fn the_audit_reader_plans_later_parts_at_the_cost_the_agent_reader_measures() {
    let two = |ids: &[(u64, &str)]| xml_plan(relabelled(&vouchers_kept(2), ids));
    let day_one = two(&[(1, "20260801"), (2, "20260801")]);
    let first_half = two(&[(3, "20260802"), (4, "20260802")]);
    let second_half = two(&[(5, "20260802"), (6, "20260802")]);
    // One response of two vouchers is exactly the budget, so the measured cost
    // allows two vouchers a part, and so does the default.
    let per_voucher = wire_len(&day_one) / 2;
    let limits = WindowReadLimits {
        budget_bytes: 2 * per_voucher,
        default_bytes_per_voucher: per_voucher,
        max_reads: MAX_PLANNED_READS,
        small_books: SmallBooks::Skip,
    };
    let census = || {
        WindowCensus::from_rows([
            (day("20260801"), 1),
            (day("20260801"), 2),
            (day("20260802"), 3),
            (day("20260802"), 4),
            (day("20260802"), 5),
            (day("20260802"), 6),
        ])
    };
    let bodies = [day_one, first_half, second_half];
    let expected = [
        part("20260801", "20260801", None),
        part(
            "20260802",
            "20260802",
            Some(AlterIdSpan {
                after: 0,
                through: 4,
            }),
        ),
        part(
            "20260802",
            "20260802",
            Some(AlterIdSpan {
                after: 4,
                through: 6,
            }),
        ),
    ];

    let (agent, _) = read_window(
        bodies.iter().flat_map(paired).collect(),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census()),
        limits,
    )
    .await;
    assert_eq!(agent.unwrap().reads, expected);

    // The audit reader reads its own marks and census, then the same parts.
    let census_xml = {
        let first = relabelled(
            &three_vouchers(),
            &[(1, "20260801"), (2, "20260801"), (3, "20260802")],
        );
        let second = relabelled(
            &three_vouchers(),
            &[(4, "20260802"), (5, "20260802"), (6, "20260802")],
        );
        let start = second.find("<VOUCHER ").unwrap();
        let end = second.rfind("</VOUCHER>").unwrap() + "</VOUCHER>".len();
        first.replacen(
            "</COLLECTION>",
            &format!("{}</COLLECTION>", &second[start..end]),
            1,
        )
    };
    assert_eq!(census_xml.matches("<VOUCHER ").count(), 6);
    let audit_reads = [
        vec![marks_plan(6, 7), xml_plan(census_xml)],
        bodies.to_vec(),
        vec![marks_plan(6, 7)],
    ]
    .concat();
    let simulator =
        SequenceSimulator::spawn(audit_reads.iter().flat_map(single).collect()).unwrap();
    let runtime = TallyRuntime::default();
    let reader = AuditWindowReader::new(
        &runtime,
        TallyConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
    );
    let identity = identity();
    let audit = read_voucher_window_with(
        &reader,
        &identity,
        identity.display_name(),
        &tally_date("20260801"),
        &tally_date("20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate { known_marks: None },
        limits,
        |xml| parse_agent_rows(xml, GUID),
    )
    .await;
    let _ = simulator.finish();
    assert_eq!(audit.unwrap().reads, expected);
}

/// A reader reused after a failed window, as a retry after `WindowChanged`
/// would, starts the next window empty: the reads it yields are that window's
/// alone, never the rejected attempt's as well.
#[tokio::test]
async fn a_reused_audit_reader_yields_only_the_window_that_succeeded() {
    let attempt = |closing| {
        std::iter::once(marks_plan(3, 7))
            .chain(divided_window_reads(closing))
            .collect::<Vec<_>>()
    };
    let plans = [attempt(marks_plan(4, 7)), attempt(marks_plan(3, 7))]
        .concat()
        .iter()
        .flat_map(single)
        .collect();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let runtime = TallyRuntime::default();
    let reader = AuditWindowReader::new(
        &runtime,
        TallyConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
    );
    let identity = identity();
    let (from, to) = (tally_date("20260801"), tally_date("20260802"));
    let window = || {
        read_voucher_window_with(
            &reader,
            &identity,
            identity.display_name(),
            &from,
            &to,
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Estimate { known_marks: None },
            three_a_read(),
            |xml| parse_agent_rows(xml, GUID),
        )
    };
    let first = window().await;
    assert_eq!(
        first.as_ref().err().unwrap().code,
        WINDOW_CHANGED_DURING_READ
    );
    let second = window().await;
    let _ = simulator.finish();
    assert!(second.is_ok());
    // Opening marks, census, three parts, closing marks: this window's six.
    assert_eq!(
        reader.into_retained(&second).map(|reads| reads.len()),
        Some(6)
    );
}

/// A window for a sealed record reads its own marks: a caller's count or
/// marks, or a replay, is refused before any request is sent.
#[tokio::test]
async fn an_audit_window_that_would_not_read_its_own_marks_is_refused_unread() {
    let runtime = TallyRuntime::default();
    // Nothing listens here: any request sent would fail differently.
    let reader = AuditWindowReader::new(
        &runtime,
        TallyConfig {
            host: "127.0.0.1".to_string(),
            port: 9,
        },
    );
    let identity = identity();
    for source in [
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(3)),
        },
        WindowPlanSource::Counted(census_of(&[("20260801", 1)])),
        WindowPlanSource::replay_of(divided_parts(), None),
    ] {
        let refused = read_voucher_window_with(
            &reader,
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260802"),
            VoucherReadShape::EntryWildcard,
            source,
            three_a_read(),
            |xml| parse_agent_rows(xml, GUID),
        )
        .await;
        assert_eq!(
            refused.err().map(|failure| failure.code),
            Some(AUDIT_WINDOW_NEEDS_ITS_OWN_MARKS.to_string())
        );
    }
}

#[test]
fn a_window_whose_part_ran_past_its_deadline_is_not_retried_as_it_is() {
    use crate::tally::runtime::AuditPartFailureKind;
    assert!(!AuditWindowFailure::Part(AuditPartFailureKind::Deadline).retryable());
    assert!(
        AuditPartFailureKind::Deadline.retryable(),
        "a part may be; the window is not"
    );
    assert!(AuditWindowFailure::Part(AuditPartFailureKind::ConnectionDropped).retryable());
    assert!(AuditWindowFailure::WindowChanged.retryable());
    assert!(!AuditWindowFailure::Refused("x".to_string()).retryable());
}

/// When a census was read, a sealed window admits its data against it, even
/// when the whole window fits one request: the census is sealed beside the
/// data, so a part holding a voucher the census did not count is refused.
#[tokio::test]
async fn an_audit_window_admits_its_data_against_the_census_it_read() {
    // A mark of 3 needs a census, which counts one voucher: the window fits.
    let counted = xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")]));
    let (outcome, _, failure, _, _) = read_audit_window(
        [
            marks_plan(3, 7),
            counted.clone(),
            // The part returns a second voucher the census never counted.
            xml_plan(relabelled(
                &vouchers_kept(2),
                &[(1, "20260801"), (2, "20260801")],
            )),
            // Refused before the closing marks are read.
        ]
        .iter()
        .flat_map(single)
        .collect(),
        TallyRuntime::default(),
    )
    .await;
    assert_eq!(
        outcome.err().map(|f| f.code),
        Some(PART_NOT_ADMITTED.to_string())
    );
    assert_eq!(
        failure,
        Some(AuditWindowFailure::Refused(PART_NOT_ADMITTED.to_string()))
    );
    // Control: the part that returns exactly the counted voucher is admitted.
    let (outcome, retained, _, _, _) = read_audit_window(
        [marks_plan(3, 7), counted.clone(), counted, marks_plan(3, 7)]
            .iter()
            .flat_map(single)
            .collect(),
        TallyRuntime::default(),
    )
    .await;
    // An undivided read that closed on its marks is not a bracketed count: the
    // token is for a read a replay would only repeat (#1241).
    assert!(outcome.expect("admitted").bracketed.is_none());
    assert_eq!(retained.map(|reads| reads.len()), Some(4));
}

/// #595: a refusal's timings give up their per-part list, and only it, when
/// the whole would not fit the share of the response budget allowed to them.
#[test]
fn window_timings_drop_only_their_parts_when_over_the_allowance() {
    let part = PartTiming {
        from: tally_date("20260801"),
        to: tally_date("20260801"),
        after: None,
        through: None,
        served: true,
        bytes: Some(10),
        rows: Some(1),
        ms: 5,
    };
    let timings = WindowReadTimings {
        from: tally_date("20260801"),
        to: tally_date("20260801"),
        marks: RequestTally { requests: 1, ms: 2 },
        census: RequestTally { requests: 3, ms: 4 },
        parts: vec![part; 3],
        failed: Some(FailedRequest {
            kind: "part",
            ms: 5,
        }),
    };
    let whole = serde_json::to_value(&timings).unwrap();
    let size = whole.to_string().len();
    assert_eq!(window_timings_within(&timings, size), whole);
    let trimmed = window_timings_within(&timings, size - 1);
    assert_eq!(
        trimmed,
        json!({
            "from": "20260801",
            "to": "20260801",
            "marks": {"requests": 1, "ms": 2},
            "census": {"requests": 3, "ms": 4},
            "failed": {"kind": "part", "ms": 5},
            "parts_omitted": 3,
        })
    );
}

// -- #674: a withheld voucher is admitted through a divided window -----------

/// `xml` with its first voucher's three amounts (party entry, its bill
/// allocation, sales entry) replaced by the composites captured from the
/// several-currency book. A synthetic mutation of a captured response.
fn with_first_voucher_composite(xml: &str) -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-forex-composite-20260915.utf16le.xml"
    );
    let forex = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let composites: Vec<&str> = forex
        .split("<AMOUNT")
        .skip(1)
        .filter_map(|tail| Some(&tail[tail.find('>')? + 1..tail.find("</AMOUNT>")?]))
        .collect();
    assert_eq!(composites.len(), 3);
    let start = xml.find("<VOUCHER ").unwrap();
    let end = start + xml[start..].find("</VOUCHER>").unwrap();
    let mut voucher = xml[start..end].to_string();
    let plain: Vec<String> = voucher
        .split("<AMOUNT")
        .skip(1)
        .filter_map(|tail| Some(tail[tail.find('>')? + 1..tail.find("</AMOUNT>")?].to_string()))
        .collect();
    assert_eq!(plain.len(), 3, "{plain:?}");
    for (plain, composite) in plain.iter().zip(&composites) {
        let at = voucher.find(&format!(">{plain}</AMOUNT>")).unwrap();
        voucher.replace_range(at + 1..at + 1 + plain.len(), composite);
    }
    format!("{}{voucher}{}", &xml[..start], &xml[end..])
}

#[tokio::test]
async fn a_withheld_voucher_is_admitted_through_a_divided_window() {
    // The plan of the test above, with the first voucher of the second part a
    // composite one: the census, part spans and union checks all see it.
    let census = WindowCensus::from_rows([
        (day("20260801"), 1),
        (day("20260801"), 2),
        (day("20260801"), 3),
        (day("20260802"), 4),
        (day("20260802"), 5),
    ]);
    let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
    plans.extend(paired(&xml_plan(with_first_voucher_composite(
        &relabelled(&vouchers_kept(2), &[(2, "20260801"), (3, "20260801")]),
    ))));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(2),
        &[(4, "20260802"), (5, "20260802")],
    ))));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let identity = identity();
    let outcome = server
        .read_voucher_window(
            &identity,
            identity.display_name(),
            &tally_date("20260801"),
            &tally_date("20260802"),
            VoucherReadShape::EntryWildcard,
            WindowPlanSource::Counted(census),
            three_a_read(),
            |xml| parse_agent_rows_withholding(xml, GUID),
        )
        .await
        .unwrap();
    simulator.finish().unwrap();
    assert_eq!(outcome.rows.len(), 5);
    let withheld: Vec<Option<u64>> = outcome
        .rows
        .iter()
        .filter(|row| matches!(row, VoucherRow::Withheld(_)))
        .map(|row| row.window_alter_id())
        .collect();
    assert_eq!(withheld, vec![Some(2)]);
    assert_eq!(outcome.reads.len(), 3);
}

/// Limits with the production budget (so one census holds 8,192 rows) under
/// which the first plan reads `per_read` vouchers a request and may send one.
fn one_read_of(per_read: u64) -> WindowReadLimits {
    WindowReadLimits {
        budget_bytes: WINDOW_READ_BUDGET_BYTES,
        default_bytes_per_voucher: WINDOW_READ_BUDGET_BYTES / per_read,
        max_reads: 1,
        small_books: SmallBooks::Skip,
    }
}

/// #945: a census that has already counted more vouchers than the allowed
/// requests can hold at the default figure is a certain refusal, so the read
/// is refused there, before the census spans left. One voucher more than the
/// allowance holds (N + 1) refuses after the first span.
#[tokio::test]
async fn a_census_past_what_the_allowed_reads_can_hold_stops_before_its_next_span() {
    let limits = one_read_of(2);
    let (outcome, observed) = read_window(
        paired(&xml_plan(three_vouchers())),
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(limits.census_capacity() + 1)),
        },
        limits,
    )
    .await;
    let failure = outcome.err().expect("refused");
    assert_eq!(failure.code, "voucher_window_too_many_reads");
    assert_eq!(
        failure.planned_reads(),
        Some(&crate::agent::PlannedReads {
            needed_at_least: 2,
            allowed: 1,
        })
    );
    // The first census span only: its second span, and every data request,
    // was never sent.
    assert_eq!(
        observed.iter().filter(|request| !request.cancelled).count(),
        6
    );
    assert!(failure.evidence.is_some(), "the census read is kept");
}

/// At exactly what the allowance holds (N) the census goes on to its next
/// span and the window is read.
#[tokio::test]
async fn a_census_at_what_the_allowed_reads_can_hold_is_finished_and_read() {
    let limits = one_read_of(3);
    let mut plans = paired(&xml_plan(three_vouchers()));
    plans.extend(paired(&xml_plan(empty_collection())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let (outcome, observed) = read_window(
        plans,
        ("20260801", "20260801"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(limits.census_capacity() + 1)),
        },
        limits,
    )
    .await;
    assert_eq!(outcome.unwrap().rows.len(), 3);
    assert_eq!(observed.len(), 18);
}

/// Limits with the production budget under which the first plan reads
/// `per_read` vouchers a request and may send `allowed` of them.
fn reads_of(per_read: u64, allowed: usize) -> WindowReadLimits {
    WindowReadLimits {
        max_reads: allowed,
        small_books: SmallBooks::Skip,
        ..one_read_of(per_read)
    }
}

/// The early stop's bound is the allowance times what one request holds, not
/// one request's worth: with three requests of one voucher each allowed, a
/// first census span of three vouchers (N) does not stop the census. The read
/// goes on to its second span and is refused there by the plan, which counts
/// the window's parts exactly; a bound of one request's worth would have
/// stopped after the first span.
#[tokio::test]
async fn a_census_at_the_allowance_times_one_requests_worth_goes_on_to_its_next_span() {
    let limits = reads_of(1, 3);
    let mut plans = paired(&xml_plan(three_vouchers()));
    plans.extend(paired(&xml_plan(empty_collection())));
    let (outcome, observed) = read_window(
        plans,
        ("20260731", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(limits.census_capacity() + 1)),
        },
        limits,
    )
    .await;
    let failure = outcome.err().expect("refused by the plan");
    assert_eq!(failure.code, "voucher_window_too_many_reads");
    assert_eq!(
        failure.planned_reads(),
        Some(&crate::agent::PlannedReads {
            // The empty days either side, and the day's three vouchers at one
            // a request: the plan tiles the window.
            needed_at_least: 5,
            allowed: 3,
        })
    );
    // Both census spans: the census was not stopped.
    assert_eq!(
        observed.iter().filter(|request| !request.cancelled).count(),
        12
    );
}

/// One voucher over the allowance times one request's worth (N + 1), with two
/// requests allowed, stops the census after its first span.
#[tokio::test]
async fn a_census_one_past_the_allowance_times_one_requests_worth_stops_with_two_allowed() {
    let limits = reads_of(1, 2);
    let (outcome, observed) = read_window(
        paired(&xml_plan(three_vouchers())),
        ("20260731", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(limits.census_capacity() + 1)),
        },
        limits,
    )
    .await;
    let failure = outcome.err().expect("refused");
    assert_eq!(failure.code, "voucher_window_too_many_reads");
    assert_eq!(
        failure.planned_reads(),
        Some(&crate::agent::PlannedReads {
            needed_at_least: 3,
            allowed: 2,
        })
    );
    assert_eq!(
        observed.iter().filter(|request| !request.cancelled).count(),
        6
    );
}

/// A census that goes over the allowance only on its last span is not
/// stopped early: there is no span left to save, so it is finished and the
/// plan refuses it with its exact count of parts. An early stop there would
/// report only the vouchers' share of it.
#[tokio::test]
async fn a_census_over_the_allowance_only_on_its_last_span_is_refused_by_the_plan() {
    let limits = reads_of(1, 1);
    let mark = limits.census_capacity() + 1;
    let mut plans = paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")])));
    plans.extend(paired(&xml_plan(relabelled(
        &vouchers_kept(1),
        &[(mark, "20260801")],
    ))));
    let (outcome, observed) = read_window(
        plans,
        ("20260731", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Estimate {
            known_marks: Some(marks_of(mark)),
        },
        limits,
    )
    .await;
    let failure = outcome.err().expect("refused by the plan");
    assert_eq!(failure.code, "voucher_window_too_many_reads");
    assert_eq!(
        failure.planned_reads(),
        Some(&crate::agent::PlannedReads {
            // The empty days either side and the day's two vouchers, one a
            // request; an early stop would have said two.
            needed_at_least: 4,
            allowed: 1,
        })
    );
    assert_eq!(
        observed.iter().filter(|request| !request.cancelled).count(),
        12
    );
}

/// #945: a part that measures heavier than the default re-plans the rest of
/// the window under what is left of the allowance. When that is too few, the
/// refusal counts the read as a whole: the requests already sent and the ones
/// the rest still needs, against the read's whole allowance.
#[tokio::test]
async fn a_replan_refusal_counts_the_requests_already_sent() {
    // One voucher measures `one`; the default is half that, so six fit a
    // request before anything is measured and three after.
    let one = wire_len(&xml_plan(vouchers_kept(1)));
    let limits = WindowReadLimits {
        budget_bytes: 3 * one,
        default_bytes_per_voucher: one / 2,
        max_reads: 2,
        small_books: SmallBooks::Skip,
    };
    // Day one's voucher is its own part; day two's six are the second.
    let census = WindowCensus::from_rows([
        (day("20260801"), 1),
        (day("20260802"), 2),
        (day("20260802"), 3),
        (day("20260802"), 4),
        (day("20260802"), 5),
        (day("20260802"), 6),
        (day("20260802"), 7),
    ]);
    let (outcome, observed) = read_window(
        paired(&xml_plan(relabelled(&vouchers_kept(1), &[(1, "20260801")]))),
        ("20260801", "20260802"),
        VoucherReadShape::EntryWildcard,
        WindowPlanSource::Counted(census),
        limits,
    )
    .await;
    let failure = outcome.err().expect("refused");
    assert_eq!(failure.code, "voucher_window_too_many_reads");
    // One request sent, and day two now needs two more, against two in all.
    assert_eq!(
        failure.planned_reads(),
        Some(&crate::agent::PlannedReads {
            needed_at_least: 3,
            allowed: 2,
        })
    );
    assert_eq!(
        observed.iter().filter(|request| !request.cancelled).count(),
        6
    );
}

// -- #485: a later page of a complete window is served from its first page's read -----------------

use super::voucher_type_class::ReservedVoucherClass;
use super::vouchers::{
    holdable, selected_voucher_operation_for_verified, VoucherOperationScope, VoucherPageKey,
    VoucherPageSnapshot, VoucherPages,
};

/// One server over one replayed sequence, so a later call can be served from
/// what an earlier call held.
struct OneServer {
    simulator: SequenceSimulator,
    server: Server,
    _directory: tempfile::TempDir,
}

impl OneServer {
    fn spawn(plans: Vec<ScenarioPlan>) -> Self {
        Self::spawn_with(plans, Redaction::None)
    }

    fn spawn_with(plans: Vec<ScenarioPlan>, redaction: Redaction) -> Self {
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_with(simulator.address(), directory.path(), redaction);
        Self {
            simulator,
            server,
            _directory: directory,
        }
    }

    async fn call(&self, extra: Value) -> Value {
        let mut args = json!({"company_guid": GUID, "from": "20260801", "to": "20260831"});
        for (key, value) in extra.as_object().unwrap() {
            args[key] = value.clone();
        }
        self.server.call_tool("vouchers", args).await
    }

    /// The requests the simulator saw, every scripted answer having been used.
    fn requests(self) -> usize {
        self.simulator
            .finish()
            .unwrap()
            .iter()
            .filter(|request| !request.method.is_empty())
            .count()
    }
}

fn identity_plans() -> Vec<ScenarioPlan> {
    vec![company_plan(), status_plan(), company_plan(), status_plan()]
}

/// The requests of a later page of a book whose marks are `marks`: the
/// identity read every call starts with, then one paired marks read.
fn marks_page_plans(marks: ScenarioPlan) -> Vec<ScenarioPlan> {
    let mut plans = identity_plans();
    plans.extend(paired(&marks));
    plans
}

/// The marks the counted book's first read opened and closed on.
fn counted_marks() -> ScenarioPlan {
    mark(WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).census_capacity())
}

fn page_items_of(response: &Value) -> Vec<Value> {
    assert_ne!(response["isError"], true, "{response}");
    response["structuredContent"]["result"]["items"]
        .as_array()
        .unwrap()
        .clone()
}

fn page_snapshot(response: &Value) -> &Value {
    &response["structuredContent"]["result"]["snapshot"]
}

fn refusal_of(response: &Value) -> &Value {
    assert_eq!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]["error"]
}

/// The rows of the whole counted window, one page.
async fn whole_counted_window() -> Vec<Value> {
    let response =
        call_vouchers_over(counted_vouchers_plans(three_vouchers(), three_vouchers())).await;
    page_items_of(&response)
}

#[tokio::test]
async fn a_later_page_is_served_from_the_first_pages_read() {
    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(marks_page_plans(counted_marks()));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"limit": 1})).await;
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    assert_eq!(page_snapshot(&first)["reused"], false);
    assert_eq!(
        page_snapshot(&first)["voucher_alter_id"],
        counted_marks_vouchers()
    );
    let second = one
        .call(json!({"offset": 1, "limit": 1, "snapshot_id": id}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true);
    assert_eq!(page_snapshot(&second)["id"], id);
    // The second page sent the identity read and one marks read, and nothing
    // of the window: every scripted answer was used and no more were asked.
    assert_eq!(one.requests(), total);
    let whole = whole_counted_window().await;
    assert_eq!(page_items_of(&first).as_slice(), &whole[..1]);
    assert_eq!(page_items_of(&second).as_slice(), &whole[1..2]);
    let result = &second["structuredContent"]["result"];
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 3);
    // A served page that is not the last says so: a caller must not read it as
    // the whole window. It also says where it starts.
    assert_eq!(second["structuredContent"]["truncated"], true, "{second}");
    assert_eq!(result["offset"], 1);
    assert_eq!(first["structuredContent"]["truncated"], true);
}

fn counted_marks_vouchers() -> u64 {
    WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).census_capacity()
}

#[tokio::test]
async fn a_moved_book_refuses_a_page_that_names_its_snapshot() {
    for moved in [
        mark(counted_marks_vouchers() + 1),
        marks_plan(counted_marks_vouchers(), 8),
    ] {
        let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
        plans.extend(marks_page_plans(moved));
        let total = plans.len();
        let one = OneServer::spawn(plans);
        let first = one.call(json!({"limit": 1})).await;
        let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
        let second = one
            .call(json!({"offset": 1, "limit": 1, "snapshot_id": id}))
            .await;
        let error = refusal_of(&second);
        assert_eq!(error["code"], "listing_snapshot_changed", "{second}");
        assert_eq!(error["cause"], "book_changed_since_first_page");
        assert_eq!(one.requests(), total);
    }
}

#[tokio::test]
async fn a_moved_book_reads_the_window_afresh_for_a_page_that_names_nothing() {
    let moved = marks_plan(counted_marks_vouchers(), 8);
    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(identity_plans());
    plans.extend(paired(&moved));
    plans.extend(paired(&moved));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"limit": 1})).await;
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one.call(json!({"offset": 1, "limit": 1})).await;
    assert_ne!(second["isError"], true, "{second}");
    assert_eq!(page_snapshot(&second)["reused"], false);
    assert_ne!(page_snapshot(&second)["id"], id);
    assert_eq!(page_snapshot(&second)["master_alter_id"], 8);
    // The page says its offsets do not continue the earlier ones.
    let earlier = &second["structuredContent"]["result"]["earlier_snapshot"];
    assert_eq!(earlier["id"], id);
    assert_eq!(earlier["cause"], "book_changed_since_first_page");
    assert_eq!(earlier["offsets_do_not_continue"], true);
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn an_unnamed_later_page_is_served_while_the_marks_are_unchanged() {
    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(marks_page_plans(counted_marks()));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"limit": 1})).await;
    let second = one.call(json!({"offset": 1, "limit": 2})).await;
    assert_eq!(page_snapshot(&second)["reused"], true);
    assert_eq!(page_snapshot(&second)["id"], page_snapshot(&first)["id"]);
    assert!(second["structuredContent"]["result"]
        .get("earlier_snapshot")
        .is_none());
    let whole = whole_counted_window().await;
    assert_eq!(page_items_of(&second).as_slice(), &whole[1..3]);
    assert_eq!(one.requests(), total);
}

fn ledger_catalogue() -> String {
    captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    ))
}

/// A ledger first page read whole is held with the `ledger_match` it answered
/// with, and its later page, served from that hold, answers with the same one
/// (#1076): the ledger it read and that the name matched by case. The first
/// page reads the catalogue, the counted window and the catalogue again; the
/// second only the identity and the marks.
#[tokio::test]
async fn a_held_ledger_page_and_its_served_page_name_the_same_ledger() {
    let mut plans = identity_plans();
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(marks_page_plans(counted_marks()));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let ledger = "CAFé NAïVE TRADERS";
    let first = one.call(json!({"ledger": ledger, "limit": 1})).await;
    assert_ne!(first["isError"], true, "{first}");
    let result = &first["structuredContent"]["result"];
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(page_snapshot(&first)["reused"], false, "{result}");
    let ledger_match = result["ledger_match"].clone();
    assert_eq!(
        ledger_match["ledger"], "Café Naïve Traders",
        "{ledger_match}"
    );
    assert_eq!(ledger_match["matched"], "case_or_spacing", "{ledger_match}");
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one
        .call(json!({"ledger": ledger, "offset": 1, "limit": 1, "snapshot_id": id}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    assert_eq!(
        second["structuredContent"]["result"]["ledger_match"], ledger_match,
        "{second}"
    );
    assert_eq!(one.requests(), total);
}

/// A page served from a held ledger window names the ledger it read, as the
/// first page did (#1076): `ledger_match` is held with the rows, and the page
/// still sends only the identity and marks reads.
#[tokio::test]
async fn a_served_page_of_a_ledger_window_names_the_ledger_it_read() {
    let first =
        call_vouchers_over(counted_vouchers_plans(three_vouchers(), three_vouchers())).await;
    let rows = page_items_of(&first);
    let window = first["structuredContent"]["result"]["window"].clone();
    let plans = marks_page_plans(counted_marks());
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let ledger_match = json!({"ledger": "Cash", "matched": "case_or_spacing",
        "similar_ledgers": [], "similar_ledgers_total": 0});
    let key = VoucherPageKey::new(
        &identity(),
        &tally_date("20260801"),
        &tally_date("20260831"),
        Some("cash"),
        None,
        None,
    );
    assert!(one
        .server
        .voucher_pages
        .lock()
        .unwrap()
        .hold(Arc::new(VoucherPageSnapshot::new(
            key,
            marks_of(counted_marks_vouchers()),
            Arc::new(rows),
            window,
            None,
            Some(ledger_match.clone()),
            Some(selected_ledger_for_tests("Cash", None)),
            None,
            None,
        ))));
    let second = one
        .call(json!({"ledger": "cash", "offset": 1, "limit": 1}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    assert_eq!(
        second["structuredContent"]["result"]["ledger_match"], ledger_match,
        "{second}"
    );
    assert_eq!(one.requests(), total);
}

#[test]
fn a_held_window_is_found_by_its_own_question_only() {
    let identity = identity();
    let key = |ledger: Option<&str>, selector: Option<&VoucherTypeSelector>| {
        VoucherPageKey::new(
            &identity,
            &tally_date("20260801"),
            &tally_date("20260831"),
            ledger,
            selector,
            None,
        )
    };
    let held = |key: VoucherPageKey| {
        Arc::new(VoucherPageSnapshot::new(
            key,
            marks_of(3),
            Arc::new(Vec::new()),
            Value::Null,
            None,
            None,
            None,
            None,
            None,
        ))
    };
    let class = VoucherTypeSelector::Class(ReservedVoucherClass::Sales);
    let mut pages = VoucherPages::default();
    assert!(pages.hold(held(key(None, Some(&class)))));
    assert!(pages.current(&key(None, Some(&class))).is_some());
    assert!(pages.current(&key(None, None)).is_none());
    assert!(pages.current(&key(Some("Cash"), Some(&class))).is_none());
    assert!(pages
        .current(&key(
            None,
            Some(&VoucherTypeSelector::Class(ReservedVoucherClass::Purchase))
        ))
        .is_none());
    // Another company's writes leave it held; this company's drop it.
    pages.drop_company("00000000-0000-4000-8000-000000000002");
    assert!(pages.current(&key(None, Some(&class))).is_some());
    pages.drop_company(&identity.company_guid().to_uppercase());
    assert!(pages.current(&key(None, Some(&class))).is_none());
}

#[tokio::test]
async fn a_partial_window_is_not_held_and_a_named_snapshot_of_it_refuses() {
    // A small book is counted now (#1029), so its window is complete and held.
    // The partial window here is one that withholds a composite voucher, which
    // the census still counts (marks, census, then the window).
    let composite = xml_plan(with_first_voucher_composite(&three_vouchers()));
    let mut plans = identity_plans();
    plans.extend(paired(&mark(3)));
    plans.extend(paired(&composite));
    plans.extend(paired(&composite));
    let first_len = plans.len();
    // The second page of a window nothing is held for: no marks read for
    // nothing, then the window afresh.
    plans.extend(identity_plans());
    plans.extend(paired(&mark(3)));
    plans.extend(paired(&composite));
    plans.extend(paired(&composite));
    // The third, naming a snapshot there is none of: refused after the identity read alone.
    plans.extend(identity_plans());
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"limit": 1})).await;
    assert_eq!(first["structuredContent"]["result"]["state"], "partial");
    assert!(page_snapshot(&first).is_null(), "{first}");
    let second = one.call(json!({"offset": 1, "limit": 1})).await;
    assert_eq!(second["structuredContent"]["result"]["state"], "partial");
    assert!(page_snapshot(&second).is_null());
    let third = one
        .call(json!({"offset": 1, "limit": 1, "snapshot_id": "nothing-held"}))
        .await;
    assert_eq!(refusal_of(&third)["cause"], "snapshot_not_held");
    assert_eq!(one.requests(), total);
    assert!(first_len < total);
}

#[tokio::test]
async fn a_write_through_the_server_drops_a_held_window() {
    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(identity_plans());
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"limit": 1})).await;
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    assert_eq!(one.server.voucher_pages.lock().unwrap().held_count(), 1);
    one.server.drop_listing_snapshots(GUID);
    assert_eq!(one.server.voucher_pages.lock().unwrap().held_count(), 0);
    let second = one
        .call(json!({"offset": 1, "limit": 1, "snapshot_id": id}))
        .await;
    assert_eq!(refusal_of(&second)["cause"], "snapshot_not_held");
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn an_expired_or_oversized_window_is_not_held() {
    for (ttl, max_bytes) in [
        (std::time::Duration::ZERO, usize::MAX),
        (std::time::Duration::from_secs(600), 1),
    ] {
        let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
        plans.extend(identity_plans());
        let total = plans.len();
        let one = OneServer::spawn(plans);
        one.server
            .voucher_pages
            .lock()
            .unwrap()
            .limits_for_test(ttl, max_bytes);
        let first = one.call(json!({"limit": 1})).await;
        // A window the byte cap kept out advertises no snapshot it cannot serve.
        let id = match page_snapshot(&first)["id"].as_str() {
            Some(id) => id.to_string(),
            None => {
                assert_eq!(max_bytes, 1, "{first}");
                "nothing-held".to_string()
            }
        };
        assert_eq!(page_snapshot(&first).is_null(), max_bytes == 1);
        let second = one
            .call(json!({"offset": 1, "limit": 1, "snapshot_id": id}))
            .await;
        assert_eq!(
            refusal_of(&second)["cause"],
            "snapshot_not_held",
            "{second}"
        );
        assert_eq!(one.requests(), total);
    }
}

#[test]
fn a_held_window_answers_one_question_only() {
    let identity = identity();
    let base = || {
        VoucherPageKey::new(
            &identity,
            &tally_date("20260801"),
            &tally_date("20260831"),
            None,
            None,
            None,
        )
    };
    assert_eq!(base(), base());
    assert_ne!(
        base(),
        VoucherPageKey::new(
            &identity,
            &tally_date("20260802"),
            &tally_date("20260831"),
            None,
            None,
            None
        )
    );
    assert_ne!(
        base(),
        VoucherPageKey::new(
            &identity,
            &tally_date("20260801"),
            &tally_date("20260830"),
            None,
            None,
            None
        )
    );
    assert_ne!(
        base(),
        VoucherPageKey::new(
            &identity,
            &tally_date("20260801"),
            &tally_date("20260831"),
            Some("Cash"),
            None,
            None
        )
    );
    let sales = VoucherTypeSelector::Name("Sales".to_string());
    let purchase = VoucherTypeSelector::Name("Purchase".to_string());
    let keyed = |selector| {
        VoucherPageKey::new(
            &identity,
            &tally_date("20260801"),
            &tally_date("20260831"),
            None,
            Some(selector),
            None,
        )
    };
    assert_ne!(base(), keyed(&sales));
    assert_ne!(keyed(&sales), keyed(&purchase));
    let searched = |args: Value| {
        let search = VoucherSearch::from_args(&args, Redaction::None).unwrap();
        VoucherPageKey::new(
            &identity,
            &tally_date("20260801"),
            &tally_date("20260831"),
            None,
            None,
            search.as_ref(),
        )
    };
    assert_ne!(base(), searched(json!({"voucher_number": "1"})));
    assert_ne!(
        searched(json!({"voucher_number": "1"})),
        searched(json!({"voucher_number": "2"}))
    );
    assert_eq!(
        searched(json!({"amount": "5"})),
        searched(json!({"amount": "5.00"}))
    );
    // A summary is its own question, and each grouping a different one.
    assert_ne!(base(), base().with_summary(Some(SummaryGroup::Month)));
    assert_ne!(
        base().with_summary(Some(SummaryGroup::Month)),
        base().with_summary(Some(SummaryGroup::Ledger))
    );
    assert_ne!(
        keyed(&sales),
        keyed(&VoucherTypeSelector::Guid("Sales".to_string()))
    );
}

#[tokio::test]
async fn the_desktop_adapter_never_holds_a_window() {
    let one = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    let (company, identity, evidence) = one.server.verified_company(GUID).await.unwrap();
    let outcome = selected_voucher_operation_for_verified(
        &one.server,
        &json!({"company_guid": GUID, "from": "20260801", "to": "20260831", "limit": 1}),
        VoucherOperationScope {
            initial_evidence: Some(evidence),
            ..VoucherOperationScope::desktop(
                GUID.to_string(),
                tally_date("20260801"),
                tally_date("20260831"),
                company,
                identity,
            )
        },
    )
    .await
    .unwrap();
    assert!(outcome.payload["result"].get("snapshot").is_none());
    assert_eq!(one.server.voucher_pages.lock().unwrap().held_count(), 0);
    let _ = one.requests();
}

#[test]
fn only_a_whole_window_with_nothing_withheld_is_held() {
    assert!(holdable(WindowRead::Complete, 0));
    assert!(!holdable(WindowRead::Complete, 1));
    assert!(!holdable(WindowRead::Partial, 0));
    assert!(!holdable(WindowRead::Partial, 1));
}

/// The party names a page of the counted window carries when nothing is masked.
async fn counted_window_party_names() -> Vec<String> {
    let mut names = Vec::new();
    for row in whole_counted_window().await {
        if let Some(name) = row["party"].as_str() {
            if !name.is_empty() && !names.iter().any(|seen| seen == name) {
                names.push(name.to_string());
            }
        }
    }
    names
}

#[tokio::test]
async fn party_names_are_masked_on_a_first_page_and_on_a_served_page() {
    // What the unmasked tool returns for these rows: the names the masked pages
    // must not carry. Without them the assertions below would prove nothing.
    let names = counted_window_party_names().await;
    assert!(
        names.len() >= 2,
        "the fixture must carry party names: {names:?}"
    );
    let plain_first = {
        let one = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
        one.call(json!({"limit": 3})).await
    };
    let plain_text = plain_first["structuredContent"].to_string();
    assert!(
        names.iter().any(|name| plain_text.contains(name.as_str())),
        "the unmasked page must carry a party name (the control)"
    );

    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(marks_page_plans(counted_marks()));
    let one = OneServer::spawn_with(plans, Redaction::MaskParties);
    let first = one.call(json!({"limit": 1})).await;
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one
        .call(json!({"offset": 1, "limit": 1, "snapshot_id": id}))
        .await;
    // The second page really came from the held rows, not from a fresh read.
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    for (label, response) in [("first page", &first), ("served page", &second)] {
        assert_ne!(response["isError"], true, "{response}");
        let text = response["structuredContent"].to_string();
        for name in &names {
            assert!(
                !text.contains(name.as_str()),
                "the {label} carries the party name {name:?} under mask_parties: {text}"
            );
        }
    }
}

/// The captured window of a renamed purchase type, its company's GUID replaced by this
/// module's, so that its rows belong to the company the plans answer for.
fn classed_window() -> String {
    let captured = captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-vouchers-renamed-purchase-class.utf16le.xml"
    ));
    let theirs = "de2e15f2-6d42-4715-b6e7-b7a95a68abe8";
    assert!(captured.contains(theirs));
    captured.replace(theirs, GUID)
}

/// Identity, the pre-flight marks, the census and the window, for a call that selects by type
/// (which reads the class-resolving shape).
fn counted_class_plans(window: String) -> (Vec<ScenarioPlan>, ScenarioPlan) {
    let limits = WindowReadLimits::for_shape(VoucherReadShape::ClassEntryWildcard);
    let marks = mark(limits.census_capacity());
    let mut plans = identity_plans();
    plans.extend(paired(&marks));
    plans.extend(paired(&xml_plan(window.clone())));
    plans.extend(paired(&xml_plan(window)));
    (plans, marks)
}

#[tokio::test]
async fn a_served_page_of_a_type_filtered_window_keeps_its_type_summary_and_says_when_it_is_last() {
    let (mut plans, marks) = counted_class_plans(classed_window());
    plans.extend(marks_page_plans(marks));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    // The captured window is July 2025.
    let window = json!({"from": "20250701", "to": "20250731"});
    let mut filter = json!({"voucher_type": "purchase a/c", "limit": 1});
    filter["from"] = window["from"].clone();
    filter["to"] = window["to"].clone();
    let first = one.call(filter).await;
    assert_ne!(first["isError"], true, "{first}");
    let first_result = &first["structuredContent"]["result"];
    assert_eq!(first_result["total"], 2, "{first}");
    assert_eq!(first["structuredContent"]["truncated"], true);
    let first_types = first_result["voucher_types"].clone();
    assert!(
        first_types.is_object(),
        "the first page names the types: {first}"
    );
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one
        .call(
            json!({"from": "20250701", "to": "20250731", "voucher_type": "purchase a/c",
                     "offset": 1, "limit": 1, "snapshot_id": id}),
        )
        .await;
    assert_ne!(second["isError"], true, "{second}");
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    let second_result = &second["structuredContent"]["result"];
    // The served page is the other one of the two selected rows, the type
    // summary of the whole selection is carried to it, and as the last page it
    // says nothing more follows.
    assert_eq!(second_result["total"], 2);
    assert_eq!(second_result["voucher_types"], first_types, "{second}");
    assert_eq!(second["structuredContent"]["truncated"], false);
    assert_eq!(second_result["offset"], 1);
    assert_eq!(page_items_of(&second).len(), 1);
    assert_ne!(page_items_of(&first), page_items_of(&second));
    // The second page sent the identity read and one marks read, and nothing of the window.
    assert_eq!(one.requests(), total);
}

/// A page is cut from rows that are held unredacted, so `page_items` itself must mask and
/// mark every row it hands out, whatever a later layer also does with the result.
#[test]
fn page_items_masks_the_party_names_of_every_row_it_cuts() {
    let rows: Vec<Value> = (0..3)
        .map(|n| {
            json!({
                "voucher_number": n.to_string(),
                "party": format!("Lab Party {n}"),
                "party_ledger_name": "Lab Party Pvt",
                "amounts": [{"ledger": "Lab Party Pvt", "amount": "10.00"}],
            })
        })
        .collect();
    let directory = tempfile::tempdir().unwrap();
    let address: std::net::SocketAddr = "127.0.0.1:9".parse().unwrap();
    let masked = server_with(address, directory.path(), Redaction::MaskParties);
    let page = super::super::vouchers::page_items(&masked, &rows, 1, 1);
    assert_eq!(page.len(), 1);
    let text = serde_json::to_string(&page).unwrap();
    assert!(
        !text.contains("Lab Party"),
        "a cut page carries a party name: {text}"
    );
    // The control: with nothing masked the same page carries the names as plain text.
    let plain = server_with(address, directory.path(), Redaction::None);
    let text =
        serde_json::to_string(&super::super::vouchers::page_items(&plain, &rows, 1, 1)).unwrap();
    assert!(
        text.contains("Lab Party 1") && text.contains("Lab Party Pvt"),
        "{text}"
    );
}

// -- #1239: the read cost a `vouchers` result states -------------------------------------------------

/// The plans of a read whose census is `reads` reads (the simulator takes at
/// most 128 scripted requests): the first span holds the window's three
/// vouchers, the others are empty.
fn census_plans_of(reads: u64) -> Vec<ScenarioPlan> {
    let capacity = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).census_capacity();
    let mut plans = identity_plans();
    plans.extend(paired(&mark(reads * capacity)));
    plans.extend(paired(&xml_plan(three_vouchers())));
    for _ in 1..reads {
        plans.extend(paired(&xml_plan(empty_collection())));
    }
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans
}

/// Eighteen census reads are worth a statement in the result; one is not, and a
/// small book's result is unchanged.
#[tokio::test]
async fn vouchers_states_its_read_cost_from_eighteen_census_reads_and_not_from_one() {
    let response = call_vouchers_over(census_plans_of(18)).await;
    assert_eq!(response["isError"], false, "{response}");
    let window = &response["structuredContent"]["result"]["window"];
    assert_eq!(window["census"]["requests"], 18, "{window}");
    let cost = &window["read_cost"];
    assert_eq!(cost["ended"], "read", "{window}");
    assert_eq!(cost["census_reads"], 18, "{window}");
    // Seventeen gaps of half a second, rounded down.
    assert_eq!(cost["floor_seconds"], 8, "{window}");
    assert_eq!(cost["vouchers_read"], 3, "{window}");
    assert_eq!(
        cost["host_240"],
        json!({"state": "window_fits", "vouchers_that_fitted": 3}),
        "{window}"
    );

    let small =
        call_vouchers_over(counted_vouchers_plans(three_vouchers(), three_vouchers())).await;
    let window = &small["structuredContent"]["result"]["window"];
    assert_eq!(window["census"]["requests"], 1, "{window}");
    assert!(window.get("read_cost").is_none(), "{window}");
}

/// A later page is served from the held window in about a second, so it must not
/// repeat what the first page's read cost: it keeps the first page's timings and
/// no `read_cost`.
#[tokio::test]
async fn a_later_page_does_not_repeat_the_first_pages_read_cost() {
    let capacity = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).census_capacity();
    // Sixteen reads: fifteen gaps, 7.5 s, enough to speak, and the page's own
    // scripted requests still fit the simulator.
    let mut plans = census_plans_of(16);
    plans.extend(marks_page_plans(mark(16 * capacity)));
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"limit": 1})).await;
    let window = &first["structuredContent"]["result"]["window"];
    assert_eq!(window["read_cost"]["census_reads"], 16, "{window}");
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one
        .call(json!({"offset": 1, "limit": 1, "snapshot_id": id}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    let window = &second["structuredContent"]["result"]["window"];
    assert_eq!(window["census"]["requests"], 16, "{window}");
    assert!(window.get("read_cost").is_none(), "{window}");
}

/// One `vouchers` call over the plans, at a response cap and with a page limit.
async fn call_vouchers_capped(plans: Vec<ScenarioPlan>, cap: usize, limit: Option<u64>) -> Value {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_at(simulator.address(), directory.path());
    server.settings.max_bytes = cap;
    let mut args = json!({"company_guid": GUID, "from": "20260801", "to": "20260831"});
    if let Some(limit) = limit {
        args["limit"] = json!(limit);
    }
    let response = server.call_tool("vouchers", args).await;
    simulator.finish().unwrap();
    response
}

/// A page is judged by its smallest form (one item), not by the page as asked for:
/// at a cap where the whole page of three vouchers would not have carried the
/// block three times over but a one-voucher page does, the block is still there,
/// and the call never turns into an oversize refusal.
#[tokio::test]
async fn the_read_cost_is_judged_against_the_smallest_page_and_never_causes_an_oversize_refusal() {
    let whole = call_vouchers_over(census_plans_of(18)).await;
    let block_len = whole["structuredContent"]["result"]["window"]["read_cost"]
        .to_string()
        .len();
    let one = call_vouchers_capped(census_plans_of(18), 200_000, Some(1)).await;
    let one_page = one["structuredContent"].to_string().len() - block_len;
    // Exactly enough for the one-item page and the block, not for the whole page.
    let cap = 3 * (one_page + block_len) + 1_024 + 256;
    let capped = call_vouchers_capped(census_plans_of(18), cap, None).await;
    assert_eq!(capped["isError"], false, "{capped}");
    let result = &capped["structuredContent"]["result"];
    assert!(
        result["items"]
            .as_array()
            .is_some_and(|rows| !rows.is_empty()),
        "{result}"
    );
    let window = &result["window"];
    assert!(window.get("read_cost").is_some(), "{window}");
    assert!(window.get("read_cost_left_out").is_none(), "{window}");
    assert_eq!(window["census"]["requests"], 18, "{window}");
}

/// #1250 with #1239: a summary page keys its rows as `buckets`, not `items`, so the
/// block is judged against the whole summary page (conservative: a summary page is
/// already bounded to a fifth of the budget). It carries the block when that page
/// fits three times over with the block, and the window says when it was left out.
#[tokio::test]
async fn a_summary_page_is_judged_against_its_whole_payload_for_the_read_cost() {
    let ample = OneServer::spawn(census_plans_of(18))
        .call(json!({"summarise_by": "ledger"}))
        .await;
    let result = result_of(&ample);
    assert_eq!(result["profile"], "agent_vouchers_v1_summary", "{result}");
    assert!(result.get("items").is_none(), "{result}");
    let buckets = result["buckets"].as_array().unwrap();
    assert!(buckets.len() >= 3, "{result}");
    assert_eq!(
        result["window"]["read_cost"]["census_reads"], 18,
        "{result}"
    );
    let block_len = result["window"]["read_cost"].to_string().len();
    // The page without the block, its key and the comma beside it.
    let page_len = ample["structuredContent"].to_string().len() - block_len - 13;
    let others: usize = buckets[1..].iter().map(|b| b.to_string().len() + 1).sum();
    assert!(others > 200, "{others}");
    // A cap that would admit the block if only the first bucket counted, but not the
    // whole page: the block is left out, said so, and the call still succeeds.
    let cap = 3 * (page_len + block_len) + 1_024 - 3 * others / 2;
    let mut tight = OneServer::spawn(census_plans_of(18));
    tight.server.settings.max_bytes = cap;
    let capped = tight.call(json!({"summarise_by": "ledger"})).await;
    let capped_result = result_of(&capped);
    assert_eq!(
        capped_result["profile"], "agent_vouchers_v1_summary",
        "{capped_result}"
    );
    let window = &capped_result["window"];
    assert!(window.get("read_cost").is_none(), "{window}");
    assert_eq!(window["read_cost_left_out"], "response_budget", "{window}");
}

// -- #1230: search and summaries over the labelled window --

fn result_of(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn bucket_names(response: &Value) -> Vec<String> {
    result_of(response)["buckets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|bucket| bucket["group"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn a_voucher_number_search_returns_that_voucher_from_the_counted_window() {
    let one = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    let response = one.call(json!({"voucher_number": "2"})).await;
    let result = result_of(&response);
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["total"], 1);
    assert_eq!(result["items"][0]["voucher_number"], "2");
    assert_eq!(
        result["items"][0]["matched"],
        json!({"voucher_number": true})
    );
    // The whole window was read once; the search is client-side and sent nothing more.
    one.requests();
}

#[tokio::test]
async fn a_search_that_finds_nothing_in_a_counted_window_is_a_checked_zero() {
    let one = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    let response = one
        .call(json!({"narration_contains": "no such phrase"}))
        .await;
    let result = result_of(&response);
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["total"], 0);
    assert_eq!(result["items"], json!([]));
}

#[tokio::test]
async fn a_later_page_of_a_search_is_served_only_for_the_same_search() {
    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(marks_page_plans(counted_marks()));
    // The third call names the first page's snapshot with a different search: refused after the
    // identity read alone, no marks read for a window that is not the one held.
    plans.extend(identity_plans());
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let search = json!({"narration_contains": "WR2-N", "limit": 1});
    let first = one.call(search.clone()).await;
    assert_eq!(result_of(&first)["total"], 3);
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one
        .call(json!({"narration_contains": "WR2-N", "limit": 1, "offset": 1, "snapshot_id": id}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    assert_eq!(result_of(&second)["items"][0]["voucher_number"], "2");
    let other = one
        .call(json!({"narration_contains": "WR2-N6", "limit": 1, "offset": 1, "snapshot_id": id}))
        .await;
    let error = refusal_of(&other);
    assert_eq!(error["code"], "listing_snapshot_changed", "{error}");
    assert_eq!(error["cause"], "snapshot_not_held", "{error}");
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_refused_search_costs_only_the_identity_read() {
    for (args, code) in [
        (json!({"voucher_number": " "}), "search_criterion_empty"),
        (
            json!({"narration_contains": "ab"}),
            "search_narration_too_short",
        ),
        (json!({"amount": "-5"}), "search_amount_invalid"),
    ] {
        let one = OneServer::spawn(identity_plans());
        let response = one.call(args).await;
        assert_eq!(refusal_of(&response)["code"], code, "{response}");
        assert_eq!(one.requests(), 4);
    }
}

/// The catalogue's own schema admits the three groupings and the four search criteria, and
/// refuses a grouping it does not list, before any read.
#[test]
fn the_vouchers_schema_lists_the_groupings_and_the_search_criteria() {
    let call = |extra: Value| {
        let mut args = json!({"company_guid": GUID, "from": "20260801", "to": "20260831"});
        for (key, value) in extra.as_object().unwrap() {
            args[key] = value.clone();
        }
        validate_tool_arguments("vouchers", &args)
    };
    for grouping in ["ledger", "month", "voucher_type", "group", "primary_group"] {
        assert_eq!(call(json!({"summarise_by": grouping})), Ok(()));
    }
    assert!(call(json!({"summarise_by": "groups"})).is_err());
    for criterion in [
        "voucher_number",
        "reference",
        "narration_contains",
        "amount",
    ] {
        assert_eq!(call(json!({ criterion: "x1" })), Ok(()), "{criterion}");
        assert!(call(json!({ criterion: "" })).is_err(), "{criterion}");
        assert!(
            call(json!({ criterion: "x".repeat(257) })).is_err(),
            "{criterion}"
        );
    }
}

#[tokio::test]
async fn a_narration_search_is_refused_where_narrations_are_withheld() {
    let one = OneServer::spawn_with(identity_plans(), Redaction::DropNarration);
    let response = one.call(json!({"narration_contains": "WR2-N3"})).await;
    assert_eq!(refusal_of(&response)["code"], "search_narration_redacted");
    assert_eq!(one.requests(), 4);
}

#[tokio::test]
async fn a_month_summary_replaces_items_with_buckets_and_keeps_the_window_label() {
    let one = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    let response = one.call(json!({"summarise_by": "month"})).await;
    let result = result_of(&response);
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["profile"], "agent_vouchers_v1_summary");
    assert!(result.get("items").is_none(), "{result}");
    assert_eq!(result["summarised_by"], "month");
    assert_eq!(result["total"], 1);
    assert_eq!(result["vouchers_summarised"], 3);
    assert_eq!(result["buckets"][0]["group"], "2026-08");
    assert_eq!(result["buckets"][0]["debit"], "-306.06");
    assert_eq!(result["buckets"][0]["credit"], "306.06");
    assert_eq!(
        result["totals"],
        json!({"debit": "-306.06", "credit": "306.06"})
    );
    assert_eq!(result["post_dated_included"], 0);
    // The older captured window was read before the fetch list asked for the flag, so its three vouchers carry none.
    assert_eq!(result["post_dated_flag_absent"], 3);
    assert!(result["basis"].as_str().unwrap().contains("memorandum"));
    assert_eq!(result["buckets"][0]["position"], 1);
    assert_eq!(
        result["excluded_from_buckets"],
        json!({"cancelled": 0, "optional": 0, "no_accounting_entries": 0})
    );
    assert_eq!(response["structuredContent"]["truncated"], false);
}

/// Mutant killed: summing the bucket totals from a different page than the one served, or
/// dropping `search` from the summarised rows.
#[tokio::test]
async fn a_summary_of_a_search_adds_only_the_vouchers_the_search_found() {
    let one = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    let response = one
        .call(json!({"summarise_by": "voucher_type", "narration_contains": "WR2-N3"}))
        .await;
    let result = result_of(&response);
    assert_eq!(result["vouchers_summarised"], 2, "{result}");
    assert_eq!(result["buckets"][0]["credit"], "203.03");
}

#[tokio::test]
async fn buckets_page_from_the_held_summary_window() {
    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(marks_page_plans(counted_marks()));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one
        .call(json!({"summarise_by": "ledger", "limit": 2}))
        .await;
    assert_eq!(page_snapshot(&first)["reused"], false);
    assert_eq!(first["structuredContent"]["truncated"], true);
    let second = one
        .call(json!({"summarise_by": "ledger", "offset": 2, "limit": 2}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    assert_eq!(result_of(&second)["total"], 4);
    assert_eq!(result_of(&second)["offset"], 2);
    assert_eq!(bucket_names(&second).len(), 2);
    assert_eq!(second["structuredContent"]["truncated"], false);
    assert_eq!(one.requests(), total);
    // The two buckets served are the third and fourth of the whole summary, positions included.
    let whole = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    let all = whole.call(json!({"summarise_by": "ledger"})).await;
    assert_eq!(
        result_of(&all)["buckets"].as_array().unwrap()[2..],
        result_of(&second)["buckets"].as_array().unwrap()[..]
    );
}

/// A ledger summary through the tool under `mask_parties`: every bucket label is a ledger name, so
/// every label is masked and no real name appears anywhere in the response, while the figures, the
/// order and each bucket's `position` stay the same as unmasked (#1250 review, P3 2). The masking
/// of a bucket label was pinned only below the tool
/// (`a_ledger_name_is_a_party_marked_value_so_masking_reaches_it`), so a label built without the
/// party marker would have passed the suite. The bucket redaction in `render_page_body` is a second,
/// redundant layer under the whole-response pass in `redact_tool_response`; this test does not tell
/// the two apart.
/// Mutant run and killed: neutering the `MaskParties` branch of `redact_value`. Mutant run and NOT
/// killed (by design, the layer is redundant): removing the bucket redaction in `render_page_body`.
/// Not run, reasoned from the code: building a bucket label without the party-name marker.
#[tokio::test]
async fn a_ledger_summary_masks_every_bucket_label_under_mask_parties() {
    let plain = OneServer::spawn(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    let plain = plain.call(json!({"summarise_by": "ledger"})).await;
    let masked = OneServer::spawn_with(
        counted_vouchers_plans(three_vouchers(), three_vouchers()),
        Redaction::MaskParties,
    );
    let masked = masked.call(json!({"summarise_by": "ledger"})).await;
    let (plain_names, masked_names) = (bucket_names(&plain), bucket_names(&masked));
    assert_eq!(plain_names.len(), 4, "{plain}");
    assert_eq!(masked_names.len(), plain_names.len(), "{masked}");
    let leaked = masked.to_string();
    for (real, shown) in plain_names.iter().zip(&masked_names) {
        assert_ne!(real, shown, "a bucket label was not masked: {masked}");
        assert!(
            !leaked.contains(real.as_str()),
            "{real} appears in a masked response"
        );
    }
    // Masking changes the label only: the figures, the order and each bucket's position are the same.
    let strip = |response: &Value| -> Vec<Value> {
        result_of(response)["buckets"]
            .as_array()
            .unwrap()
            .iter()
            .map(|bucket| {
                let mut bucket = bucket.clone();
                bucket.as_object_mut().unwrap().remove("group");
                bucket
            })
            .collect()
    };
    assert_eq!(strip(&plain), strip(&masked));
    let positions: Vec<u64> = result_of(&masked)["buckets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|bucket| bucket["position"].as_u64().unwrap())
        .collect();
    assert_eq!(positions, [1, 2, 3, 4], "{masked}");
}

/// A listing and a summary of the same question hold separate windows, so one never replaces or
/// serves the other: pages of a listing cannot continue from a summary's read, nor the reverse.
/// Mutant killed: leaving the grouping out of the held-window key.
#[tokio::test]
async fn a_listing_and_a_summary_do_not_replace_or_serve_each_other() {
    let mut plans = counted_vouchers_plans(three_vouchers(), three_vouchers());
    plans.extend(counted_vouchers_plans(three_vouchers(), three_vouchers()));
    plans.extend(marks_page_plans(counted_marks()));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let listing = one.call(json!({"limit": 1})).await;
    let listing_id = page_snapshot(&listing)["id"].as_str().unwrap().to_string();
    // A summary read of the same window holds its own snapshot; the listing's stays held.
    let summary = one.call(json!({"summarise_by": "month"})).await;
    let summary_id = page_snapshot(&summary)["id"].as_str().unwrap().to_string();
    assert_ne!(listing_id, summary_id);
    let page = one
        .call(json!({"offset": 1, "limit": 1, "snapshot_id": listing_id}))
        .await;
    assert_eq!(page_snapshot(&page)["id"], listing_id, "{page}");
    assert_eq!(page_snapshot(&page)["reused"], true);
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_held_ledger_window_summarises_only_that_ledgers_entries_by_month() {
    let first =
        call_vouchers_over(counted_vouchers_plans(three_vouchers(), three_vouchers())).await;
    let mut rows = page_items_of(&first);
    rows[0]["date"] = json!("20260715");
    rows[1]["date"] = json!("20260801");
    rows[2]["date"] = json!("20260901");
    let window = first["structuredContent"]["result"]["window"].clone();
    let plans = marks_page_plans(counted_marks());
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let key = VoucherPageKey::new(
        &identity(),
        &tally_date("20260801"),
        &tally_date("20260831"),
        Some("WR2 Sales"),
        None,
        None,
    )
    .with_summary(Some(SummaryGroup::Month));
    assert!(one
        .server
        .voucher_pages
        .lock()
        .unwrap()
        .hold(Arc::new(VoucherPageSnapshot::new(
            key,
            marks_of(counted_marks_vouchers()),
            Arc::new(rows),
            window,
            None,
            Some(json!({"ledger": "WR2 Sales", "matched": "exact",
                "similar_ledgers": [], "similar_ledgers_total": 0})),
            Some(selected_ledger_for_tests("WR2 Sales", None)),
            None,
            None,
        ))));
    let second = one
        .call(json!({"ledger": "WR2 Sales", "summarise_by": "month", "offset": 1, "limit": 1}))
        .await;
    let result = result_of(&second);
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    assert_eq!(result["entries_counted"], "selected_ledger");
    assert_eq!(result["total"], 3);
    assert_eq!(result["buckets"][0]["group"], "2026-08");
    assert_eq!(result["buckets"][0]["debit"], "0");
    assert_eq!(result["buckets"][0]["credit"], "102.02");
    assert_eq!(one.requests(), total);
}

/// The bucket page stops at a fifth of the response budget (the response carries it twice, and
/// the text copy escapes quotes). Through the whole tool the cap refuses first on a four-bucket
/// fixture, so the bound is exercised on the page renderer itself: the captured ledger summary's
/// buckets serialize to about 500, 290, 270 and 300 bytes, so a 6,000-byte cap (1,200 for
/// buckets) holds three and not four. Mutant killed: an unbounded budget.
#[tokio::test]
async fn a_bucket_page_stops_at_a_fifth_of_the_response_budget_and_says_more_remain() {
    let first =
        call_vouchers_over(counted_vouchers_plans(three_vouchers(), three_vouchers())).await;
    let rows = page_items_of(&first);
    let mut one = OneServer::spawn(identity_plans());
    one.server.settings.max_bytes = 6_000;
    let request = SummaryRequest {
        group: SummaryGroup::Ledger,
        selected_ledger: None,
        placements: None,
    };
    let body =
        vouchers::render_page_body(&one.server, &rows, Some(&request), (0, 500)).expect("renders");
    assert_eq!(body.items.len(), 3, "{:?}", body.items);
    assert_eq!(body.total, 4);
    assert!(body.truncated);
    // Under a budget that holds everything the page is whole.
    one.server.settings.max_bytes = 200_000;
    let whole =
        vouchers::render_page_body(&one.server, &rows, Some(&request), (0, 500)).expect("renders");
    assert_eq!(whole.items.len(), 4);
    assert!(!whole.truncated);
}

// -- #1230: summarise_by group and primary_group ----------------------------------------------
//
// The masters the scripted Tally answers are captured: the WR2 book's ledger catalogue and the live group
// snapshot of another synthetic book. That snapshot is DERIVED here for this book (its GUID and the GUIDs
// that carry it moved to this book's, and one user group the catalogue needs added), because no group
// snapshot of this book was captured.

fn wr2_group_snapshot() -> String {
    let live = captured_utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/shape-lab-group-snapshot.utf16le.xml"
    ))
    .replace("3a6bd6e1-b835-4bff-89dd-8a6af138c346", GUID);
    let start = live.find("<GROUP NAME=\"Trade Debtors - Local\"").unwrap();
    let end = live[start..].find("</GROUP>").unwrap() + start + "</GROUP>".len();
    let row = live[start..end]
        .replace("Trade Debtors - Local", "Bridge Nested Debtors WR4")
        .replace("-000000dd", "-000000f0")
        .replace("> 222<", "> 230<")
        .replace("> 221<", "> 229<");
    format!("{}\r\n    {}{}", &live[..end], row, &live[end..])
}

/// The scripted sequence of one first-page group summary: the identity, the masters, the counted window,
/// the masters again.
fn group_summary_plans(before: (String, String), after: (String, String)) -> Vec<ScenarioPlan> {
    group_plans_sized_by(counted_marks(), before, after)
}

/// The plans of a group summary whose sizing read (the company's marks, read before the ledger list)
/// is `sizing`.
fn group_plans_sized_by(
    sizing: ScenarioPlan,
    before: (String, String),
    after: (String, String),
) -> Vec<ScenarioPlan> {
    let mut plans = identity_plans();
    // The group modes size the ledger list first, from the company's marks.
    plans.extend(paired(&sizing));
    plans.extend(paired(&xml_plan(before.0)));
    plans.extend(paired(&xml_plan(before.1)));
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(after.0)));
    plans.extend(paired(&xml_plan(after.1)));
    plans
}

fn masters() -> (String, String) {
    (ledger_catalogue(), wr2_group_snapshot())
}

fn buckets_of(response: &Value) -> Vec<Value> {
    assert_ne!(response["isError"], true, "{response}");
    response["structuredContent"]["result"]["buckets"]
        .as_array()
        .unwrap()
        .clone()
}

#[tokio::test]
async fn a_group_summary_reads_the_masters_around_the_window_and_sums_by_group() {
    let plans = group_summary_plans(masters(), masters());
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let response = one.call(json!({"summarise_by": "group"})).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["summarised_by"], "group");
    assert!(
        result["basis"]
            .as_str()
            .unwrap()
            .starts_with(super::vouchers::GROUP_BASIS),
        "the group caveat leads the basis: {result}"
    );
    assert_eq!(
        result["totals"],
        json!({"debit": "-306.06", "credit": "306.06"}),
        "{result}"
    );
    let buckets = buckets_of(&response);
    let by_name = |name: &str| {
        buckets
            .iter()
            .find(|b| b["group"] == name)
            .unwrap_or_else(|| panic!("{name}: {buckets:?}"))
    };
    // The three captured vouchers: one sales ledger credited, parties under Sundry Debtors debited.
    assert_eq!(by_name("Sales Accounts")["credit"], "306.06");
    assert_eq!(
        by_name("Sales Accounts")["primary_group"]["name"],
        "Sales Accounts"
    );
    let debtors = buckets
        .iter()
        .filter(|b| b["primary_group"]["name"] == "Current Assets")
        .count();
    assert!(debtors >= 1, "{buckets:?}");
    for bucket in &buckets {
        assert!(
            bucket["chain"]
                .as_array()
                .is_some_and(|chain| !chain.is_empty()),
            "{bucket}"
        );
        assert_eq!(
            bucket["members_total"],
            bucket["members"].as_array().unwrap().len()
        );
    }
    // Every scripted answer was used: the masters were read before and again after the window.
    assert_eq!(one.requests(), total);
    // The same window summarised by primary group.
    let plans = group_summary_plans(masters(), masters());
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let response = one.call(json!({"summarise_by": "primary_group"})).await;
    let primary = buckets_of(&response);
    assert!(
        primary.iter().all(|b| b["reserved_name"] == b["group"]),
        "{primary:?}"
    );
    assert_eq!(
        response["structuredContent"]["result"]["totals"],
        json!({"debit": "-306.06", "credit": "306.06"})
    );
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_group_summary_refuses_when_a_ledger_or_a_group_moved_while_the_window_was_read() {
    let run = |after: (String, String)| async move {
        let one = OneServer::spawn(group_summary_plans(masters(), after));
        let response = one.call(json!({"summarise_by": "group"})).await;
        refusal_of(&response)["code"].clone()
    };
    // A ledger moved to another group.
    let moved = ledger_catalogue().replacen(
        "<PARENT TYPE=\"String\">Cash-in-Hand</PARENT>",
        "<PARENT TYPE=\"String\">Sundry Debtors</PARENT>",
        1,
    );
    assert_ne!(moved, ledger_catalogue());
    assert_eq!(
        run((moved, wr2_group_snapshot())).await,
        "group_snapshot_drifted"
    );
    // A group the catalogued ledgers sit under moved under another one (`Sundry Debtors` out of `Current
    // Assets`). A group that no listed ledger sits under could move without changing any chain, and is
    // not seen.
    let snapshot = wr2_group_snapshot();
    let start = snapshot.find("<GROUP NAME=\"Sundry Debtors\"").unwrap();
    let end = snapshot[start..].find("</GROUP>").unwrap() + start;
    let row = &snapshot[start..end];
    assert!(
        row.contains("<PARENT TYPE=\"String\">Current Assets</PARENT>"),
        "{row}"
    );
    let regrouped = format!(
        "{}{}{}",
        &snapshot[..start],
        row.replacen(
            "<PARENT TYPE=\"String\">Current Assets</PARENT>",
            "<PARENT TYPE=\"String\">Current Liabilities</PARENT>",
            1
        ),
        &snapshot[end..]
    );
    assert_ne!(regrouped, snapshot);
    assert_eq!(
        run((ledger_catalogue(), regrouped)).await,
        "group_snapshot_drifted"
    );
    // A ledger gone from the catalogue.
    let removed =
        ledger_catalogue().replacen("<LEDGER NAME=\"Cash\"", "<LEDGER NAME=\"Cash Renamed\"", 1);
    assert_ne!(removed, ledger_catalogue());
    assert_eq!(
        run((removed, wr2_group_snapshot())).await,
        "ledger_snapshot_drifted"
    );
}

#[tokio::test]
async fn a_group_summary_refuses_with_the_gap_when_a_chain_cannot_be_walked() {
    // Derived: the group snapshot without `Sundry Debtors`, which the captured parties sit under.
    let snapshot = wr2_group_snapshot();
    let start = snapshot.find("<GROUP NAME=\"Sundry Debtors\"").unwrap();
    let end = snapshot[start..].find("</GROUP>").unwrap() + start + "</GROUP>".len();
    let without = format!("{}{}", &snapshot[..start], &snapshot[end..]);
    let one = OneServer::spawn(group_summary_plans(
        (ledger_catalogue(), without.clone()),
        (ledger_catalogue(), without),
    ));
    let response = one.call(json!({"summarise_by": "group"})).await;
    let refusal = refusal_of(&response);
    assert_eq!(refusal["code"], "summary_group_unresolved", "{refusal}");
    assert_eq!(refusal["cause"], "group_absent", "{refusal}");
}

#[tokio::test]
async fn a_later_page_of_a_group_summary_is_served_from_the_held_window_with_its_placements() {
    let mut plans = group_summary_plans(masters(), masters());
    plans.extend(marks_page_plans(counted_marks()));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"summarise_by": "group", "limit": 1})).await;
    let first_buckets = buckets_of(&first);
    assert_eq!(first_buckets.len(), 1);
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one
        .call(json!({"summarise_by": "group", "limit": 1, "offset": 1, "snapshot_id": id}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    let second_buckets = buckets_of(&second);
    assert_eq!(second_buckets.len(), 1);
    assert_ne!(second_buckets[0]["group"], first_buckets[0]["group"]);
    assert!(second_buckets[0]["chain"]
        .as_array()
        .is_some_and(|chain| !chain.is_empty()));
    // The page came from the held window: only the identity and the marks were read for it.
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_held_group_summary_does_not_serve_a_page_of_another_grouping() {
    let mut plans = group_summary_plans(masters(), masters());
    plans.extend(marks_page_plans(counted_marks()));
    let one = OneServer::spawn(plans);
    let first = one.call(json!({"summarise_by": "group", "limit": 1})).await;
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let other = one
        .call(json!({"summarise_by": "primary_group", "limit": 1, "offset": 1, "snapshot_id": id}))
        .await;
    let refusal = refusal_of(&other);
    assert_eq!(refusal["code"], "listing_snapshot_changed", "{refusal}");
    assert_eq!(refusal["cause"], "snapshot_not_held", "{refusal}");
}

#[tokio::test]
async fn a_group_summary_with_a_ledger_counts_every_entry_of_that_ledgers_vouchers() {
    // `ledger` resolves its name against the ledger list the placements are built from, so the list is
    // read once before the window and once after it, as without `ledger`: the sizing marks, the list, the
    // groups, the window's marks and parts, then the list and the groups again.
    let mut plans = identity_plans();
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let response = one
        .call(json!({"summarise_by": "group", "ledger": "CAFé NAïVE TRADERS"}))
        .await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["entries_counted"], "all_entries", "{result}");
    assert_eq!(
        result["ledger_match"]["ledger"], "Café Naïve Traders",
        "{result}"
    );
    let buckets = buckets_of(&response);
    assert!(!buckets.is_empty());
    // Every entry of the vouchers that touch the ledger is counted, so debits and credits are equal in size.
    let (debit, credit) = (
        result["totals"]["debit"].as_str().unwrap(),
        result["totals"]["credit"].as_str().unwrap(),
    );
    assert_eq!(debit.trim_start_matches('-'), credit, "{result}");
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_group_summary_refuses_a_catalogue_that_names_a_ledger_twice_at_the_catalogue_parse() {
    let catalogue = ledger_catalogue();
    let start = catalogue.find("<LEDGER NAME=\"Cash\"").unwrap();
    let end = catalogue[start..].find("</LEDGER>").unwrap() + start + "</LEDGER>".len();
    let twice = format!(
        "{}{}{}",
        &catalogue[..end],
        &catalogue[start..end],
        &catalogue[end..]
    );
    let mut plans = identity_plans();
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(twice)));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    let one = OneServer::spawn(plans);
    let response = one.call(json!({"summarise_by": "group"})).await;
    let refusal = refusal_of(&response);
    // The catalogue parse refuses a repeated ledger before any placement is built.
    assert_eq!(refusal["code"], "ledger_export_invalid", "{refusal}");
    assert_eq!(
        refusal["cause"], "ledger_catalogue_duplicate_identity",
        "{refusal}"
    );
}

/// A catalogue in which `ledger` has no parent group (its `PARENT` is empty).
fn catalogue_without_parent_of(ledger: &str) -> String {
    let catalogue = ledger_catalogue();
    let start = catalogue
        .find(&format!("<LEDGER NAME=\"{ledger}\""))
        .expect("the ledger is in the catalogue");
    let open = catalogue[start..]
        .find("<PARENT TYPE=\"String\">")
        .expect("it has a parent")
        + start;
    let close = catalogue[open..].find("</PARENT>").unwrap() + open + "</PARENT>".len();
    format!(
        "{}<PARENT TYPE=\"String\"></PARENT>{}",
        &catalogue[..open],
        &catalogue[close..]
    )
}

#[tokio::test]
async fn a_group_summary_names_the_ledger_it_could_not_place_and_masks_it_under_mask_parties() {
    let ledger = "Café Naïve Traders";
    let broken = catalogue_without_parent_of(ledger);
    let plans = || {
        group_summary_plans(
            (broken.clone(), wr2_group_snapshot()),
            (broken.clone(), wr2_group_snapshot()),
        )
    };
    let plain = OneServer::spawn(plans())
        .call(json!({"summarise_by": "group"}))
        .await;
    let refusal = refusal_of(&plain);
    assert_eq!(refusal["code"], "summary_group_unresolved", "{refusal}");
    assert_eq!(refusal["cause"], "no_parent", "{refusal}");
    assert_eq!(refusal["ledger"], ledger, "{refusal}");
    assert!(refusal["remediation"].is_string(), "{refusal}");
    let masked = OneServer::spawn_with(plans(), Redaction::MaskParties)
        .call(json!({"summarise_by": "primary_group"}))
        .await;
    let refusal = refusal_of(&masked);
    assert_eq!(refusal["code"], "summary_group_unresolved", "{refusal}");
    assert!(
        !masked.to_string().contains("Naïve"),
        "the unplaced ledger is named under mask_parties: {masked}"
    );
    assert!(refusal["ledger"].is_string(), "{refusal}");
}

#[tokio::test]
async fn a_ledger_the_vouchers_name_and_the_list_lacks_is_refused_by_name_with_its_cause() {
    // The ledger list of the second read omits nothing, the vouchers name a ledger the list has under
    // another spelling: renamed in both reads of the list, so it is not drift.
    let renamed = ledger_catalogue().replacen(
        "<LEDGER NAME=\"WR2 Sales\"",
        "<LEDGER NAME=\"WR2 Sales Renamed\"",
        1,
    );
    assert_ne!(renamed, ledger_catalogue());
    let one = OneServer::spawn(group_summary_plans(
        (renamed.clone(), wr2_group_snapshot()),
        (renamed, wr2_group_snapshot()),
    ));
    let response = one.call(json!({"summarise_by": "group"})).await;
    let refusal = refusal_of(&response);
    assert_eq!(refusal["code"], "summary_group_unresolved", "{refusal}");
    assert_eq!(refusal["cause"], "ledger_not_in_catalogue", "{refusal}");
    assert_eq!(refusal["ledger"], "WR2 Sales", "{refusal}");
}

#[tokio::test]
async fn the_group_modes_size_the_ledger_list_first_from_the_masters_mark() {
    let limit = super::vouchers::GROUP_LEDGER_LIST_MARK_LIMIT;
    // One over the limit: refused after the identity and the marks reads, with no ledger list request.
    let mut plans = identity_plans();
    plans.extend(paired(&marks_plan(counted_marks_vouchers(), limit + 1)));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let response = one.call(json!({"summarise_by": "group"})).await;
    let refusal = refusal_of(&response);
    assert_eq!(refusal["code"], "summary_group_book_too_large", "{refusal}");
    assert_eq!(refusal["size"]["master_alter_id"], limit + 1, "{refusal}");
    assert_eq!(refusal["size"]["limit_master_alter_id"], limit, "{refusal}");
    assert!(refusal["remediation"].is_string(), "{refusal}");
    assert_eq!(one.requests(), total);
    // At the limit: read.
    for grouping in ["group", "primary_group"] {
        let one = OneServer::spawn(group_plans_sized_by(
            marks_plan(counted_marks_vouchers(), limit),
            masters(),
            masters(),
        ));
        let response = one.call(json!({"summarise_by": grouping})).await;
        assert_eq!(response["isError"], false, "{grouping}: {response}");
    }
}

#[tokio::test]
async fn a_group_summary_masks_every_ledger_name_and_no_group_name_under_mask_parties() {
    let plain = OneServer::spawn(group_summary_plans(masters(), masters()))
        .call(json!({"summarise_by": "group"}))
        .await;
    let masked = OneServer::spawn_with(
        group_summary_plans(masters(), masters()),
        Redaction::MaskParties,
    )
    .call(json!({"summarise_by": "group"}))
    .await;
    let members = |response: &Value| -> Vec<String> {
        buckets_of(response)
            .iter()
            .flat_map(|bucket| bucket["members"].as_array().unwrap().clone())
            .map(|member| member["ledger"].as_str().unwrap().to_string())
            .collect()
    };
    let (real, shown) = (members(&plain), members(&masked));
    assert!(!real.is_empty());
    assert_eq!(real.len(), shown.len());
    let text = masked.to_string();
    for (real, shown) in real.iter().zip(&shown) {
        assert_ne!(real, shown, "a member ledger was not masked");
        assert!(
            !text.contains(real.as_str()),
            "{real} appears in a masked response"
        );
    }
    // Group names are the book's configuration labels and stay as they are, in both modes.
    let groups = |response: &Value| -> Vec<Value> {
        buckets_of(response)
            .iter()
            .map(|b| b["group"].clone())
            .collect()
    };
    assert_eq!(groups(&plain), groups(&masked));
}

#[tokio::test]
async fn a_later_page_of_a_primary_group_summary_is_served_from_the_held_window() {
    let mut plans = group_summary_plans(masters(), masters());
    plans.extend(marks_page_plans(counted_marks()));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one
        .call(json!({"summarise_by": "primary_group", "limit": 1}))
        .await;
    assert_eq!(buckets_of(&first).len(), 1);
    let id = page_snapshot(&first)["id"].as_str().unwrap().to_string();
    let second = one
        .call(json!({"summarise_by": "primary_group", "limit": 1, "offset": 1, "snapshot_id": id}))
        .await;
    assert_eq!(page_snapshot(&second)["reused"], true, "{second}");
    assert_ne!(
        buckets_of(&second)[0]["group"],
        buckets_of(&first)[0]["group"]
    );
    assert_eq!(one.requests(), total);
}

/// The group snapshot with the row of the group `name` rewritten by `edit`.
fn with_group_row(name: &str, edit: impl Fn(&str) -> String) -> String {
    let snapshot = wr2_group_snapshot();
    let start = snapshot
        .find(&format!("<GROUP NAME=\"{name}\""))
        .expect("the group is in the snapshot");
    let end = snapshot[start..].find("</GROUP>").unwrap() + start + "</GROUP>".len();
    format!(
        "{}{}{}",
        &snapshot[..start],
        edit(&snapshot[start..end]),
        &snapshot[end..]
    )
}

#[tokio::test]
async fn every_way_a_chain_cannot_be_walked_is_refused_with_its_own_cause_and_the_ledger() {
    // Derived from the live snapshot, each by one edit to a group row the touched parties sit under.
    let cases: [(&str, String); 4] = [
        (
            "group_name_repeated",
            with_group_row("Sundry Debtors", |row| format!("{row}{row}")),
        ),
        (
            "reserved_name_missing",
            with_group_row("Sundry Debtors", |row| {
                row.replacen(" RESERVEDNAME=\"Sundry Debtors\"", "", 1)
            }),
        ),
        (
            "cycle",
            with_group_row("Sundry Debtors", |row| {
                row.replacen(
                    "<PARENT TYPE=\"String\">Current Assets</PARENT>",
                    "<PARENT TYPE=\"String\">Sundry Debtors</PARENT>",
                    1,
                )
            }),
        ),
        (
            "top_group_not_under_root",
            with_group_row("Current Assets", |row| {
                let open = row.find("<PARENT TYPE=\"String\">").unwrap();
                let close = row[open..].find("</PARENT>").unwrap() + open + "</PARENT>".len();
                format!(
                    "{}<PARENT TYPE=\"String\"></PARENT>{}",
                    &row[..open],
                    &row[close..]
                )
            }),
        ),
    ];
    for (cause, snapshot) in cases {
        assert_ne!(
            snapshot,
            wr2_group_snapshot(),
            "{cause}: the edit changed nothing"
        );
        let one = OneServer::spawn(group_summary_plans(
            (ledger_catalogue(), snapshot.clone()),
            (ledger_catalogue(), snapshot),
        ));
        let response = one.call(json!({"summarise_by": "group"})).await;
        let refusal = refusal_of(&response);
        assert_eq!(
            refusal["code"], "summary_group_unresolved",
            "{cause}: {refusal}"
        );
        assert_eq!(refusal["cause"], cause, "{refusal}");
        // The first unplaced ledger is one of the three parties the window touches.
        assert!(
            [
                "Café Naïve Traders",
                "WR2 XML Café Naïve Ledger 01A01A2F",
                "नमस्ते ट्रेडर्स"
            ]
            .contains(&refusal["ledger"].as_str().unwrap_or_default()),
            "{cause}: {refusal}"
        );
    }
}

#[tokio::test]
async fn a_parent_tally_returned_that_the_parse_withholds_is_refused_as_no_parent_by_ledger() {
    // A parent group name that ends in a carriage return and a line feed is not carried by the
    // catalogue parse, so the ledger has no parent as far as the placements can tell.
    let catalogue = ledger_catalogue();
    let start = catalogue
        .find("<LEDGER NAME=\"Café Naïve Traders\"")
        .unwrap();
    let open = catalogue[start..].find("<PARENT TYPE=\"String\">").unwrap() + start;
    let close = catalogue[open..].find("</PARENT>").unwrap() + open;
    let unsupported = format!("{}&#13;&#10;{}", &catalogue[..close], &catalogue[close..]);
    let one = OneServer::spawn(group_summary_plans(
        (unsupported.clone(), wr2_group_snapshot()),
        (unsupported, wr2_group_snapshot()),
    ));
    let response = one.call(json!({"summarise_by": "group"})).await;
    let refusal = refusal_of(&response);
    assert_eq!(refusal["code"], "summary_group_unresolved", "{refusal}");
    assert_eq!(refusal["cause"], "no_parent", "{refusal}");
    assert_eq!(refusal["ledger"], "Café Naïve Traders", "{refusal}");
}

#[tokio::test]
async fn a_group_summary_with_a_ledger_given_by_its_stored_name_pairs_each_ledger_with_its_own_parent(
) {
    // Derived: the ledger's stored name (the `NAME` child) differs from its row spelling (the `NAME`
    // attribute), as in the book of #1085. The same list serves the name and the placements, so the
    // ledger resolves by the stored name and each entry is placed under the parent of its own ledger.
    let catalogue = ledger_catalogue().replace(
        "<NAME>Café Naïve Traders</NAME>",
        "<NAME>Cafe Traders</NAME>",
    );
    assert_ne!(catalogue, ledger_catalogue());
    let mut plans = identity_plans();
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(catalogue.clone())));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(catalogue)));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let response = one
        .call(json!({"summarise_by": "group", "ledger": "Cafe Traders"}))
        .await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "complete", "{result}");
    assert_eq!(result["ledger_match"]["matched"], "exact", "{result}");
    assert_eq!(result["ledger_match"]["ledger"], "Cafe Traders", "{result}");
    assert_eq!(
        result["ledger_match"]["ledger_row_spelling"], "Café Naïve Traders",
        "{result}"
    );
    // The one voucher that touches the ledger: its debit sits under the ledger's own group, its credit
    // under the sales ledger's.
    let buckets = buckets_of(&response);
    let shape: Vec<(String, Vec<String>)> = buckets
        .iter()
        .map(|bucket| {
            (
                bucket["group"].as_str().unwrap().to_string(),
                bucket["members"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|member| member["ledger"].as_str().unwrap().to_string())
                    .collect(),
            )
        })
        .collect();
    assert_eq!(
        shape,
        vec![
            (
                "Sundry Debtors".to_string(),
                vec!["Café Naïve Traders".to_string()]
            ),
            ("Sales Accounts".to_string(), vec!["WR2 Sales".to_string()]),
        ],
        "{result}"
    );
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_primary_group_summary_masks_its_members_under_mask_parties() {
    let masked = OneServer::spawn_with(
        group_summary_plans(masters(), masters()),
        Redaction::MaskParties,
    )
    .call(json!({"summarise_by": "primary_group"}))
    .await;
    let text = masked.to_string();
    let members: Vec<String> = buckets_of(&masked)
        .iter()
        .flat_map(|bucket| bucket["members"].as_array().unwrap().clone())
        .map(|member| member["ledger"].as_str().unwrap().to_string())
        .collect();
    // The window touches four ledgers; each is listed under a label that is not its name.
    assert_eq!(members.len(), 4, "{masked}");
    for real in [
        "Café Naïve Traders",
        "WR2 Sales",
        "नमस्ते ट्रेडर्स",
        "WR2 XML Café Naïve Ledger 01A01A2F",
    ] {
        assert!(
            !text.contains(real),
            "{real} appears in a masked primary_group summary: {text}"
        );
        assert!(!members.iter().any(|label| label == real), "{real}");
    }
}

#[tokio::test]
async fn a_group_summary_of_a_window_with_a_withheld_voucher_is_partial_and_says_so() {
    let mut plans = identity_plans();
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(with_first_voucher_composite(
        &three_vouchers(),
    ))));
    plans.extend(paired(&xml_plan(with_first_voucher_composite(
        &three_vouchers(),
    ))));
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    let one = OneServer::spawn(plans);
    let response = one.call(json!({"summarise_by": "group"})).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "partial", "{result}");
    // The composite voucher is in no bucket: two of the three vouchers are summarised, and the
    // result says the totals are short by the withheld one.
    assert_eq!(result["vouchers_summarised"], 2, "{result}");
    assert!(result["coverage"].is_string(), "{result}");
    assert!(
        result["withheld_vouchers"]
            .as_array()
            .is_some_and(|w| w.len() == 1),
        "{result}"
    );
}

#[tokio::test]
async fn a_group_summary_with_a_ledger_masks_the_ledger_and_every_member_under_mask_parties() {
    let mut plans = identity_plans();
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    plans.extend(paired(&counted_marks()));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(three_vouchers())));
    plans.extend(paired(&xml_plan(ledger_catalogue())));
    plans.extend(paired(&xml_plan(wr2_group_snapshot())));
    let one = OneServer::spawn_with(plans, Redaction::MaskParties);
    let response = one
        .call(json!({"summarise_by": "group", "ledger": "Café Naïve Traders"}))
        .await;
    assert_eq!(response["isError"], false, "{response}");
    let text = response.to_string();
    for real in ["Café Naïve Traders", "WR2 Sales"] {
        assert!(
            !text.contains(real),
            "{real} appears under mask_parties: {text}"
        );
    }
    let buckets = buckets_of(&response);
    assert_eq!(buckets.len(), 2, "{response}");
    assert!(buckets
        .iter()
        .all(|bucket| bucket["members"].as_array().unwrap().len() == 1));
}

#[tokio::test]
async fn a_moved_ledger_is_refused_with_the_group_drift_advice() {
    let moved = ledger_catalogue().replacen(
        "<PARENT TYPE=\"String\">Cash-in-Hand</PARENT>",
        "<PARENT TYPE=\"String\">Sundry Debtors</PARENT>",
        1,
    );
    assert_ne!(moved, ledger_catalogue());
    let one = OneServer::spawn(group_summary_plans(
        (ledger_catalogue(), wr2_group_snapshot()),
        (moved, wr2_group_snapshot()),
    ));
    let response = one.call(json!({"summarise_by": "group"})).await;
    let refusal = refusal_of(&response);
    assert_eq!(refusal["code"], "group_snapshot_drifted", "{refusal}");
    assert!(refusal["remediation"].is_string(), "{refusal}");
}

#[tokio::test]
async fn only_a_group_summary_carries_subtree_totals() {
    let one = OneServer::spawn(group_summary_plans(masters(), masters()));
    let response = one.call(json!({"summarise_by": "group"})).await;
    let result = &response["structuredContent"]["result"];
    let totals = result["subtree_totals"]
        .as_array()
        .expect("a group summary carries subtree totals");
    assert!(
        totals.iter().any(|t| t["group"] == "Sundry Debtors"),
        "{totals:?}"
    );
    assert_eq!(result["subtree_totals_complete"], true);
    // A bucket and a subtree row of the same group say, each in its own row, what their figure covers.
    let buckets = buckets_of(&response);
    let bucket_covers = buckets[0]["covers"]
        .as_str()
        .expect("a bucket says what it covers");
    let subtree_covers = totals[0]["covers"]
        .as_str()
        .expect("a subtree row says what it covers");
    assert_eq!(
        bucket_covers,
        "only the ledgers directly under the group, not its sub-groups"
    );
    assert_eq!(
        subtree_covers,
        "the group and everything under it, sub-groups included"
    );
    let one = OneServer::spawn(group_summary_plans(masters(), masters()));
    let response = one.call(json!({"summarise_by": "primary_group"})).await;
    assert!(response["structuredContent"]["result"]
        .get("subtree_totals")
        .is_none());
    assert_eq!(
        buckets_of(&response)[0]["covers"],
        "every ledger under the group"
    );
}

#[test]
fn a_ledger_name_with_colons_survives_the_trip_through_the_refusal_string() {
    let failure = super::vouchers::summary_failure(
        "summary_group_unresolved:no_parent:Acme: North, Ltd: Unit 2".to_string(),
    );
    assert_eq!(failure.code, "summary_group_unresolved");
    assert_eq!(failure.cause, Some("no_parent"));
    assert_eq!(
        failure.read_detail.unwrap().unplaced_ledger.as_deref(),
        Some("Acme: North, Ltd: Unit 2")
    );
}
