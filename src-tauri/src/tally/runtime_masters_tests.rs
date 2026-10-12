//! Masters reads replayed from captures, with one admission or stability fault
//! at a time. The sequence is the Trial Balance's without its currency pair:
//! mode probe and identity (3), opening extent (4), the collection pair (4),
//! closing extent (4), identity and mode again (3).
use super::masters::{
    admit_masters_size, check_masters_premise, limit_for, sized_before_the_read, MastersRead,
};
use super::trial_balance_tests::{
    companies, config, decode, education, extents, identity, pair, status, xml, GUID,
};
use super::*;
use bridge_tally_protocol::native_masters::{
    masters_worst_row_bytes, parse_native_masters, NativeMasterKind, NativeMastersError,
    MASTERS_RESPONSE_BUDGET_BYTES,
};
use tally_protocol_simulator::{ScenarioPlan, SequenceSimulator};

/// The synthetic book the masters captures came from. Its GUID is replaced by
/// the test double's company, so a captured collection binds to it.
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const GROUPS_GUID: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";
const GODOWNS_KIND: MastersKind = MastersKind::Native(NativeMasterKind::Godowns);

fn capture(kind: NativeMasterKind) -> String {
    let bytes: &[u8] = match kind {
        NativeMasterKind::VoucherTypes => include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/masters_voucher_types_shape_lab_live.utf16le.xml"
        ),
        NativeMasterKind::Godowns => include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/masters_godowns_shape_lab_live.utf16le.xml"
        ),
        NativeMasterKind::Units => include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/masters_units_shape_lab_live.utf16le.xml"
        ),
        NativeMasterKind::StockGroups => include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/masters_stock_groups_shape_lab_live.utf16le.xml"
        ),
        NativeMasterKind::CostCentres => include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/masters_cost_centres_shape_lab_flag_no_live.utf16le.xml"
        ),
        NativeMasterKind::CostCategories => include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/masters_cost_categories_shape_lab_live.utf16le.xml"
        ),
    };
    decode(bytes).replace(CAPTURE_GUID, GUID)
}

/// The rows the capture holds, counted independently of the parser.
fn captured_rows(kind: NativeMasterKind) -> usize {
    match kind {
        NativeMasterKind::VoucherTypes => 26,
        NativeMasterKind::Godowns => 2,
        NativeMasterKind::Units => 4,
        NativeMasterKind::StockGroups => 3,
        NativeMasterKind::CostCentres | NativeMasterKind::CostCategories => 2,
    }
}

fn godowns() -> String {
    capture(NativeMasterKind::Godowns)
}

fn groups() -> String {
    include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/group_snapshot_aarav_with_computed_company_guid.xml"
    )
    .replace(GROUPS_GUID, GUID)
}

/// The books' extent with its master mark set to `mark`.
fn extent_with_mark(mark: u64) -> String {
    let captured = extents();
    let mark_element = "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>";
    assert!(captured.contains(mark_element));
    captured.replace(
        mark_element,
        &format!("<ALTMSTID TYPE=\"Number\"> {mark}</ALTMSTID>"),
    )
}

/// The mode probe, the identity bracket and the opening extent.
fn opening(extent: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    pair(&mut plans, xml(extent));
    plans
}

/// The opening, then the collection pair and the closing extent.
fn through_closing_extent(
    opening_extent: String,
    collection: String,
    closing_extent: String,
) -> Vec<ScenarioPlan> {
    let mut plans = opening(opening_extent);
    pair(&mut plans, xml(collection));
    pair(&mut plans, xml(closing_extent));
    plans
}

/// A whole read: also the identity and mode brackets after the closing extent.
fn complete(
    opening_extent: String,
    collection: String,
    closing_extent: String,
) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = through_closing_extent(opening_extent, collection, closing_extent);
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// The largest master mark a collection read of `kind` is admitted at.
fn largest_admitted_mark(kind: NativeMasterKind) -> u64 {
    u64::try_from(MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(kind)).unwrap()
}

async fn run(
    plans: Vec<ScenarioPlan>,
    kind: MastersKind,
) -> (anyhow::Result<(MastersRead, CompanyBookExtent)>, usize) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let result = TallyRuntime::default()
        .fetch_masters_with_extent(config(&simulator), &identity(), kind)
        .await;
    (result, simulator.finish().unwrap().len())
}

fn cause<T: std::error::Error + 'static>(error: &anyhow::Error) -> Option<&T> {
    error.chain().find_map(|cause| cause.downcast_ref::<T>())
}

/// The kinds sized by the master mark before their request. Voucher types are
/// not: they are held by the checks after the read only.
const SIZED_KINDS: [NativeMasterKind; 5] = [
    NativeMasterKind::Godowns,
    NativeMasterKind::Units,
    NativeMasterKind::StockGroups,
    NativeMasterKind::CostCentres,
    NativeMasterKind::CostCategories,
];

#[test]
fn the_admission_edge_is_the_largest_mark_that_fits_and_the_next_refuses() {
    for (kind, largest) in [
        (NativeMasterKind::Godowns, 1_152),
        (NativeMasterKind::Units, 1_168),
        (NativeMasterKind::StockGroups, 1_160),
        (NativeMasterKind::CostCentres, 1_037),
        (NativeMasterKind::CostCategories, 1_264),
    ] {
        assert!(sized_before_the_read(kind), "{kind:?}");
        assert_eq!(largest_admitted_mark(kind), largest, "{kind:?}");
        let row_bytes = u64::try_from(masters_worst_row_bytes(kind)).unwrap();
        assert_eq!(
            admit_masters_size(kind, largest).ok(),
            Some(largest * row_bytes),
            "{kind:?}"
        );
        assert!(matches!(
            admit_masters_size(kind, largest + 1),
            Err(MastersReadError::TooLarge {
                master_alter_id,
                estimated_bytes,
                limit_bytes: 16_000_000,
                limit_master_alter_id,
            }) if master_alter_id == largest + 1
                && estimated_bytes == (largest + 1) * row_bytes
                && limit_master_alter_id == largest
        ));
    }
    assert!(!sized_before_the_read(NativeMasterKind::VoucherTypes));
    // A mark so large that its product would overflow refuses; it never wraps.
    // The admitted-mark limit comes from the row size, not from the saturated
    // estimate, so it stays exact.
    assert!(matches!(
        admit_masters_size(NativeMasterKind::Godowns, u64::MAX),
        Err(MastersReadError::TooLarge {
            estimated_bytes: u64::MAX,
            limit_master_alter_id: 1_152,
            ..
        })
    ));
}

#[test]
fn the_premise_checks_refuse_each_way_a_response_can_outgrow_its_mark() {
    let rows = |kind| {
        parse_native_masters(kind, &capture(kind), GUID)
            .unwrap()
            .rows
    };
    let godowns = rows(NativeMasterKind::Godowns);
    let violated = |result: Result<(), MastersReadError>| match result {
        Err(error @ MastersReadError::PremiseViolated(reason)) => {
            assert_eq!(error.safe_code(), "masters_bound_premise_violated");
            reason
        }
        other => panic!("{other:?}"),
    };
    // Alter ids 213 and 100, mark 213, a response that just fits.
    assert!(check_masters_premise(&godowns, 213, 1_000, 1_000).is_ok());
    // More rows than the mark: 26 voucher types against a mark of 25.
    assert_eq!(
        violated(check_masters_premise(
            &rows(NativeMasterKind::VoucherTypes),
            25,
            0,
            u64::MAX
        )),
        "masters_rows_exceed_master_mark"
    );
    // A row above the mark, with no more rows than it.
    assert_eq!(
        violated(check_masters_premise(&godowns, 212, 0, u64::MAX)),
        "masters_alter_id_above_master_mark"
    );
    // Two rows with one AlterID.
    let mut repeated = godowns.clone();
    repeated[1].alter_id = repeated[0].alter_id;
    assert_eq!(
        violated(check_masters_premise(&repeated, 1_000, 0, u64::MAX)),
        "masters_alter_id_repeated"
    );
    // A response one byte over what was admitted.
    assert_eq!(
        violated(check_masters_premise(&godowns, 213, 1_001, 1_000)),
        "masters_response_over_admitted_bytes"
    );
}

#[test]
fn rows_equal_to_the_mark_are_admitted_and_one_more_row_than_the_mark_refuses() {
    let mut rows = parse_native_masters(
        NativeMasterKind::Godowns,
        &capture(NativeMasterKind::Godowns),
        GUID,
    )
    .unwrap()
    .rows;
    assert_eq!(rows.len(), 2);
    // Distinct AlterIDs at or under both marks below, so only the row count
    // differs between them.
    rows[0].alter_id = 1;
    rows[1].alter_id = 2;
    // Two rows, a mark of two: admitted (the count equals the mark).
    assert!(check_masters_premise(&rows, 2, 0, u64::MAX).is_ok());
    // Two rows, a mark of one: the count is one over the mark.
    assert!(matches!(
        check_masters_premise(&rows, 1, 0, u64::MAX),
        Err(MastersReadError::PremiseViolated(
            "masters_rows_exceed_master_mark"
        ))
    ));
}

#[tokio::test]
async fn every_kind_replays_through_every_bracket_at_its_admission_edge() {
    for kind in NativeMasterKind::ALL {
        // Voucher types are not sized by the mark: any mark above their ids reads.
        let mark = if sized_before_the_read(kind) {
            largest_admitted_mark(kind)
        } else {
            50_000_000
        };
        let (result, dispatched) = run(
            complete(
                extent_with_mark(mark),
                capture(kind),
                extent_with_mark(mark),
            ),
            MastersKind::Native(kind),
        )
        .await;

        let (read, extent) = result.unwrap();
        let MastersRows::Native(masters) = read.rows else {
            panic!("a native kind reads a native collection");
        };
        assert_eq!(masters.rows.len(), captured_rows(kind), "{kind:?}");
        assert_eq!(
            extent.master_alter_id_high_water().map(|mark| mark.get()),
            Some(mark)
        );
        assert_eq!(dispatched, 18, "{kind:?}");
    }
}

#[tokio::test]
async fn groups_are_read_by_the_group_snapshot_with_no_size_admission() {
    // A mark no collection read would be admitted at.
    let mark = 50_000_000;
    let (result, dispatched) = run(
        complete(extent_with_mark(mark), groups(), extent_with_mark(mark)),
        MastersKind::Groups,
    )
    .await;

    let MastersRows::Groups(groups) = result.unwrap().0.rows else {
        panic!("groups read the group snapshot");
    };
    assert!(!groups.is_empty());
    assert_eq!(dispatched, 18);
}

#[tokio::test]
async fn a_book_over_the_budget_is_refused_before_the_collection_request() {
    for kind in SIZED_KINDS {
        let mark = largest_admitted_mark(kind) + 1;
        let (result, dispatched) =
            run(opening(extent_with_mark(mark)), MastersKind::Native(kind)).await;

        let error = result.err().expect("refused");
        let row_bytes = u64::try_from(masters_worst_row_bytes(kind)).unwrap();
        assert!(
            matches!(
                cause::<MastersReadError>(&error),
                Some(MastersReadError::TooLarge {
                    master_alter_id,
                    estimated_bytes,
                    limit_bytes: 16_000_000,
                    ..
                }) if *master_alter_id == mark && *estimated_bytes == mark * row_bytes
            ),
            "{kind:?}: {error:?}"
        );
        assert_eq!(
            cause::<MastersReadError>(&error).map(MastersReadError::safe_code),
            Some("masters_too_large")
        );
        // Mode probe, identity and the opening extent: no collection request.
        assert_eq!(dispatched, 7, "{kind:?}");
    }
}

#[tokio::test]
async fn education_is_refused_before_identity_or_extent_dispatch() {
    let (result, dispatched) =
        run(vec![status(), xml(education(&companies()))], GODOWNS_KIND).await;

    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<MastersReadError>(&error),
        Some(MastersReadError::EducationUnqualified)
    ));
    assert_eq!(dispatched, 2);
}

#[tokio::test]
async fn a_book_that_moved_during_the_read_is_refused_and_keeps_the_completed_source() {
    let (result, dispatched) = run(
        through_closing_extent(extents(), godowns(), extent_with_mark(225)),
        GODOWNS_KIND,
    )
    .await;

    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(changed, PairedReadValidationError::MastersExtent));
    assert_eq!(changed.safe_code(), "masters_extent_changed");
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    assert!(!evidence.request_sha256.is_empty());
    assert_eq!(dispatched, 15);
}

#[tokio::test]
async fn a_collection_that_changed_between_its_paired_reads_is_refused() {
    let collection = godowns();
    let mut plans = opening(extents());
    plans.extend([
        xml(collection.clone()),
        status(),
        xml(format!("{collection}\n")),
        status(),
    ]);
    let (result, dispatched) = run(plans, GODOWNS_KIND).await;

    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::MastersCollection
    ));
    assert_eq!(changed.safe_code(), "masters_collection_changed");
    assert_eq!(dispatched, 11);
}

#[tokio::test]
async fn a_transport_failure_stays_a_transport_failure() {
    let mut plans = opening(extents());
    plans.push(xml(godowns()).with_http_status(500));
    let (result, dispatched) = run(plans, GODOWNS_KIND).await;

    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<bridge_tally_transport::TallyTransportError>(&error),
        Some(bridge_tally_transport::TallyTransportError::HttpStatus { status: 500 })
    ));
    // Not read as an absent collection, nor as any refusal of Bridge's own.
    assert!(cause::<NativeMastersError>(&error).is_none());
    assert!(cause::<MastersReadError>(&error).is_none());
    assert!(cause::<PairedReadValidationError>(&error).is_none());
    assert_eq!(dispatched, 8);
}

#[tokio::test]
async fn a_response_with_no_collection_is_the_parsers_typed_refusal() {
    let captured = godowns();
    let start = captured.find("<COLLECTION").unwrap();
    let end = captured.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    let absent = format!("{}{}", &captured[..start], &captured[end..]);
    // No closing extent is provided: a response Bridge cannot read refuses at
    // once, and a sequence that expected one would never finish.
    let mut plans = opening(extents());
    pair(&mut plans, xml(absent));
    let (result, dispatched) = run(plans, GODOWNS_KIND).await;

    let error = result.err().expect("refused");
    assert_eq!(
        cause::<NativeMastersError>(&error),
        Some(&NativeMastersError::CollectionAbsent)
    );
    // Mode probe and identity (3), opening extent (4), the collection pair (4).
    assert_eq!(dispatched, 11);
    // The completed collection read is kept as evidence.
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    assert!(!evidence.response_sha256.is_empty());
}

#[tokio::test]
async fn a_group_answer_bridge_cannot_read_refuses_at_once() {
    // The captured group snapshot with its collection removed: the group parser
    // refuses it, and that refusal is returned at once, never held behind a
    // closing extent read (no closing extent is provided).
    let captured = groups();
    let start = captured.find("<COLLECTION").unwrap();
    let end = captured.rfind("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    let absent = format!("{}{}", &captured[..start], &captured[end..]);
    let mut plans = opening(extents());
    pair(&mut plans, xml(absent));
    let (result, dispatched) = run(plans, MastersKind::Groups).await;

    let error = result.err().expect("refused");
    assert!(cause::<PairedReadValidationError>(&error).is_none());
    assert!(cause::<MastersReadError>(&error).is_none());
    // Mode probe and identity (3), opening extent (4), the group pair (4).
    assert_eq!(dispatched, 11);
}

#[tokio::test]
async fn a_voucher_type_answer_with_no_rows_is_refused_at_once() {
    // The captured voucher-type response with its rows removed: a present,
    // empty collection, which is an answer for units and no answer for voucher
    // types (every company has predefined ones).
    let captured = capture(NativeMasterKind::VoucherTypes);
    let open_end = captured.find("<COLLECTION").unwrap();
    let open_end = open_end + captured[open_end..].find('>').unwrap() + 1;
    let close = captured.find("</COLLECTION>").unwrap();
    let empty = format!("{}{}", &captured[..open_end], &captured[close..]);
    assert!(!empty.contains("<VOUCHERTYPE "));
    let mut plans = opening(extents());
    pair(&mut plans, xml(empty));
    let (result, dispatched) =
        run(plans, MastersKind::Native(NativeMasterKind::VoucherTypes)).await;

    let error = result.err().expect("refused");
    assert_eq!(
        cause::<NativeMastersError>(&error),
        Some(&NativeMastersError::VoucherTypesEmpty)
    );
    assert_eq!(dispatched, 11);
}

#[tokio::test]
async fn a_response_that_breaks_the_marks_premise_is_refused_after_the_read() {
    // Two rows, but a mark below the larger AlterID (213).
    let plans = through_closing_extent(extent_with_mark(2), godowns(), extent_with_mark(2));
    let (result, dispatched) = run(plans, GODOWNS_KIND).await;

    let error = result.err().expect("refused");
    let violated = cause::<MastersReadError>(&error).expect("a premise refusal");
    assert!(matches!(
        violated,
        MastersReadError::PremiseViolated("masters_alter_id_above_master_mark")
    ));
    assert_eq!(violated.safe_code(), "masters_bound_premise_violated");
    // The completed collection read is kept as evidence.
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    assert!(!evidence.response_sha256.is_empty());
    assert_eq!(dispatched, 15);

    // The mark fits every row, but the response is larger than was admitted for
    // it: 213 godown rows at most 13,888 bytes each is 2,958,144 bytes, and this
    // response carries 3.2 MB (1.6 million UTF-16 characters) beside its rows.
    let padded = godowns().replacen(
        "<DATA>",
        &format!("<PAD>{}</PAD><DATA>", "x".repeat(1_600_000)),
        1,
    );
    let plans = through_closing_extent(extent_with_mark(213), padded, extent_with_mark(213));
    let (result, dispatched) = run(plans, GODOWNS_KIND).await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<MastersReadError>(&error),
        Some(MastersReadError::PremiseViolated(
            "masters_response_over_admitted_bytes"
        ))
    ));
    assert_eq!(dispatched, 15);
}

#[tokio::test]
async fn a_row_longer_than_the_assumed_worst_row_is_the_parsers_typed_refusal() {
    // A filler element the parser does not read, in a row: only the row's own
    // length can refuse it.
    let filler = "x".repeat(masters_worst_row_bytes(NativeMasterKind::Godowns));
    let long_row = godowns().replacen(
        "</GODOWN>",
        &format!("<FILLER>{filler}</FILLER></GODOWN>"),
        2,
    );
    let mut plans = opening(extents());
    pair(&mut plans, xml(long_row));
    let (result, dispatched) = run(plans, GODOWNS_KIND).await;

    let error = result.err().expect("refused");
    assert_eq!(
        cause::<NativeMastersError>(&error),
        Some(&NativeMastersError::RowExceedsBound)
    );
    // Refused at once, with no closing extent read.
    assert_eq!(dispatched, 11);
}

#[test]
fn the_size_limit_is_the_estimate_for_a_sized_kind_and_the_budget_for_the_rest() {
    let budget = u64::try_from(MASTERS_RESPONSE_BUDGET_BYTES).unwrap();
    for kind in SIZED_KINDS {
        assert_eq!(limit_for(kind, Some(4_321)), 4_321, "{kind:?}");
    }
    // Voucher types were not sized before the read, so they have no estimate
    // to be held to, and are held to the budget whatever they are given.
    assert_eq!(limit_for(NativeMasterKind::VoucherTypes, None), budget);
    assert_eq!(
        limit_for(NativeMasterKind::VoucherTypes, Some(4_321)),
        budget
    );
}

#[tokio::test]
async fn a_book_that_moved_during_the_read_is_reported_as_moved_not_as_what_its_rows_broke() {
    // A godown created mid-read: its AlterID (213) is above the mark the read
    // opened at (150), and the closing extent shows the mark moved to 260.
    let plans = through_closing_extent(extent_with_mark(150), godowns(), extent_with_mark(260));
    let (result, dispatched) = run(plans, GODOWNS_KIND).await;

    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(changed, PairedReadValidationError::MastersExtent));
    assert_eq!(changed.safe_code(), "masters_extent_changed");
    assert!(cause::<MastersReadError>(&error).is_none());
    assert_eq!(dispatched, 15);
}

#[tokio::test]
async fn a_zero_row_capture_reads_as_an_empty_collection_through_every_bracket() {
    // The READS LAB capture: a book without inventory answers with a present,
    // empty collection. Its GUID is replaced by the test double's company.
    let empty = decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/masters_units_reads_lab_live.utf16le.xml"
    ))
    .replace("de2e15f2-6d42-4715-b6e7-b7a95a68abe8", GUID);
    let (result, dispatched) = run(
        complete(extents(), empty, extents()),
        MastersKind::Native(NativeMasterKind::Units),
    )
    .await;

    let MastersRows::Native(masters) = result.unwrap().0.rows else {
        panic!("a native kind reads a native collection");
    };
    assert!(masters.rows.is_empty());
    assert_eq!(dispatched, 18);
}
