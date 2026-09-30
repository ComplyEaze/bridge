use super::*;
use sha2::{Digest, Sha256};

const LIMITS: CensusLimits = CensusLimits {
    slice_width: 10,
    max_slices: 5,
};

fn plan(mark: u64) -> Result<LedgerCensusPlan, LedgerCensusError> {
    LedgerCensusPlan::new(mark, LIMITS)
}

fn guids(prefix: &str, count: u64) -> Vec<String> {
    (0..count)
        .map(|index| format!("{prefix}-{index:04x}"))
        .collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn the_slices_tile_the_mark_with_no_gap_and_no_overlap() {
    for mark in [1, 9, 10, 11, 20, 21, 50] {
        let plan = plan(mark).expect("within the limits");
        let slices = plan.slices();
        assert_eq!(slices.first().unwrap().after(), 0, "mark {mark}");
        assert_eq!(slices.last().unwrap().through(), mark, "mark {mark}");
        for pair in slices.windows(2) {
            assert_eq!(pair[0].through(), pair[1].after(), "mark {mark}");
        }
        for slice in slices {
            assert!(slice.after() < slice.through(), "mark {mark}");
            assert!(slice.width() <= LIMITS.slice_width, "mark {mark}");
        }
        assert_eq!(
            slices.len() as u64,
            mark.div_ceil(LIMITS.slice_width),
            "mark {mark}"
        );
    }
}

#[test]
fn a_plan_at_the_slice_limit_is_made_and_one_more_is_refused() {
    assert_eq!(plan(50).unwrap().slices().len(), 5);
    assert_eq!(
        plan(51),
        Err(LedgerCensusError::TooManySlices { slices: 6 })
    );
    // A mark near the top of u64 must be refused, not overflow the count.
    assert!(matches!(
        plan(u64::MAX),
        Err(LedgerCensusError::TooManySlices { .. })
    ));
}

#[test]
fn a_plan_of_nothing_is_refused() {
    assert_eq!(plan(0), Err(LedgerCensusError::PlanInvalid));
    for limits in [
        CensusLimits {
            slice_width: 0,
            max_slices: 5,
        },
        CensusLimits {
            slice_width: 10,
            max_slices: 0,
        },
    ] {
        assert_eq!(
            LedgerCensusPlan::new(10, limits),
            Err(LedgerCensusError::PlanInvalid)
        );
    }
}

fn run(
    plan: LedgerCensusPlan,
    per_slice: Vec<Vec<String>>,
) -> Result<LedgerCount, LedgerCensusError> {
    let mut census = LedgerCensus::new(plan);
    for guids in per_slice {
        census.accept(guids)?;
    }
    census.finish()
}

#[test]
fn a_complete_census_counts_every_ledger_once() {
    let count = run(
        plan(25).unwrap(),
        vec![guids("a", 4), guids("b", 0), guids("c", 5)],
    )
    .expect("three slices, all accepted");
    assert_eq!(count.get(), 9);
}

#[test]
fn slices_are_offered_in_order_and_only_once_each() {
    let mut census = LedgerCensus::new(plan(25).unwrap());
    let mut offered = Vec::new();
    while let Some(slice) = census.next_slice() {
        offered.push((slice.after(), slice.through()));
        // Asking again before accepting offers the same slice, not the next.
        assert_eq!(census.next_slice(), Some(slice));
        census
            .accept(guids(
                &format!("s{}", offered.len()),
                1 + offered.len() as u64,
            ))
            .unwrap();
    }
    assert_eq!(offered, vec![(0, 10), (10, 20), (20, 25)]);
    // Nothing is left to accept.
    assert_eq!(
        census.accept(Vec::new()),
        Err(LedgerCensusError::Incomplete)
    );
}

#[test]
fn a_census_that_stopped_early_yields_no_count() {
    assert_eq!(
        run(plan(25).unwrap(), vec![guids("a", 3), guids("b", 3)]).map(LedgerCount::get),
        Err(LedgerCensusError::Incomplete)
    );
    assert_eq!(
        run(plan(25).unwrap(), Vec::new()).map(LedgerCount::get),
        Err(LedgerCensusError::Incomplete)
    );
}

#[test]
fn a_census_that_found_no_ledger_is_refused_not_counted_as_zero() {
    assert_eq!(
        run(plan(25).unwrap(), vec![Vec::new(), Vec::new(), Vec::new()]).map(LedgerCount::get),
        Err(LedgerCensusError::CensusEmpty)
    );
}

#[test]
fn a_slice_wider_than_its_span_is_refused_at_its_span() {
    // The last slice of a mark of 25 spans five AlterIDs.
    let mut census = LedgerCensus::new(plan(25).unwrap());
    census
        .accept(guids("a", 10))
        .expect("a full slice holds its width");
    census.accept(guids("b", 10)).unwrap();
    assert_eq!(
        census.accept(guids("c", 6)),
        Err(LedgerCensusError::SliceOverBound { rows: 6 })
    );
    // Exactly the span is fine.
    let mut census = LedgerCensus::new(plan(25).unwrap());
    census.accept(guids("a", 10)).unwrap();
    census.accept(guids("b", 10)).unwrap();
    census.accept(guids("c", 5)).unwrap();
    assert_eq!(census.finish().unwrap().get(), 25);
}

#[test]
fn a_guid_seen_twice_refuses_the_census_within_a_slice_and_across_slices() {
    let mut within = LedgerCensus::new(plan(25).unwrap());
    assert_eq!(
        within.accept(vec!["x".into(), "y".into(), "x".into()]),
        Err(LedgerCensusError::DuplicateIdentity)
    );
    let mut across = LedgerCensus::new(plan(25).unwrap());
    across.accept(vec!["x".into(), "y".into()]).unwrap();
    assert_eq!(
        across.accept(vec!["z".into(), "x".into()]),
        Err(LedgerCensusError::DuplicateIdentity)
    );
}

#[test]
fn a_guid_differing_only_in_case_is_the_same_ledger() {
    let mut census = LedgerCensus::new(plan(25).unwrap());
    census.accept(vec!["AbC-0001".into()]).unwrap();
    assert_eq!(
        census.accept(vec!["abc-0001".into()]),
        Err(LedgerCensusError::DuplicateIdentity)
    );
}

#[test]
fn a_refused_slice_keeps_nothing() {
    let mut census = LedgerCensus::new(plan(25).unwrap());
    // The duplicate is the third row: the two before it must not be retained.
    assert!(census
        .accept(vec!["p".into(), "q".into(), "q".into()])
        .is_err());
    assert_eq!(
        census.next_slice().unwrap().after(),
        0,
        "still on slice one"
    );
    census.accept(vec!["p".into(), "q".into()]).unwrap();
}

#[test]
fn every_refusal_has_its_own_data_free_code() {
    let codes = [
        LedgerCensusError::PlanInvalid,
        LedgerCensusError::TooManySlices { slices: 6 },
        LedgerCensusError::SliceOverBound { rows: 6 },
        LedgerCensusError::DuplicateIdentity,
        LedgerCensusError::CensusEmpty,
        LedgerCensusError::Incomplete,
    ]
    .map(LedgerCensusError::safe_code);
    let distinct: HashSet<_> = codes.iter().collect();
    assert_eq!(distinct.len(), codes.len());
    assert!(codes.iter().all(|code| code.starts_with("ledger_span_")));
}

/// The bytes Bridge sends are the bytes measured: these two requests were sent
/// to a live TallyPrime 7.1 Silver on 30 Sep 2026 (the fixtures' sidecars carry
/// the same hashes as `source_request_sha256`), so a change to the template
/// that would make a shipped request differ from the measured one fails here.
#[test]
fn the_slice_request_is_the_request_that_was_sent_live() {
    let plan = LedgerCensusPlan::new(
        508,
        CensusLimits {
            slice_width: 500,
            max_slices: 2,
        },
    )
    .unwrap();
    // (0, 500] and (500, 508]: the second is the captured eight-row slice.
    let second = plan.slices()[1];
    assert_eq!((second.after(), second.through()), (500, 508));
    let sent = crate::encode_tally_xml_request_utf16le(&render_ledger_census_slice_request(
        "BRIDGE SIZE 2K",
        &second,
    ));
    assert_eq!(sent.len(), 1906);
    assert_eq!(
        sha256_hex(&sent),
        "19d973c32bad65373fcc8df09758522849ee36e19338d7f875dbf214dd4657bb"
    );
    let first_captured = LedgerCensusPlan::new(
        8,
        CensusLimits {
            slice_width: 8,
            max_slices: 1,
        },
    )
    .unwrap();
    let sent = crate::encode_tally_xml_request_utf16le(&render_ledger_census_slice_request(
        "BRIDGE SIZE 2K",
        &first_captured.slices()[0],
    ));
    assert_eq!(sent.len(), 1898);
    assert_eq!(
        sha256_hex(&sent),
        "aa953b2927bc55e394947b54f8621ac0fd3ad6d5e23bf2e8909b796a4e54b53d"
    );
}

#[test]
fn the_company_name_is_escaped_and_the_formula_is_digits_only() {
    let plan = plan(10).unwrap();
    let request = render_ledger_census_slice_request("A & B <LAB>", &plan.slices()[0]);
    assert!(request.contains("<SVCURRENTCOMPANY>A &amp; B &lt;LAB&gt;</SVCURRENTCOMPANY>"));
    assert!(request.contains(r#"NAME="BridgeSpan">$AlterID &gt; 0 AND $AlterID &lt;= 10</SYSTEM>"#));
    assert!(request.contains("<FILTERS>BridgeSpan</FILTERS>"));
    // The only `$$` is the export format: the formula holds no function, so the
    // space hazard of section 6.1 cannot apply.
    assert_eq!(request.matches("$$").count(), 1);
    assert!(request.contains("$$SysName:XML"));
}
