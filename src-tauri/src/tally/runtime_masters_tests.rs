//! Masters reads replayed from captures, with one admission or stability fault
//! at a time. The sequence is the Trial Balance's without its currency pair:
//! mode probe and identity (3), opening extent (4), the collection pair (4),
//! closing extent (4), identity and mode again (3). Voucher types read a
//! second collection, the series-level numbering, as a second pair between the
//! first and the closing extent.
use super::masters::{
    admit_candidate_stock_item_read, admit_masters_size, admit_stock_groups, admit_stock_units,
    check_masters_premise, check_numbering_premise, limit_for, sized_before_the_read,
    AdmittedCandidateRead, MastersRead, StockItemCountMismatch, StockItemReadKind,
    StockItemReadKindError, StockItemReadSize,
};
use super::trial_balance_tests::{
    companies, config, decode, education, extents, identity, pair, status, xml, GUID,
};
use super::*;
use bridge_tally_protocol::native_masters::{
    masters_worst_row_bytes, parse_native_masters, render_native_masters_request, NativeMasterKind,
    NativeMastersError, MASTERS_RESPONSE_BUDGET_BYTES,
};
use bridge_tally_protocol::native_voucher_type_numbering::{
    parse_voucher_type_numbering, render_voucher_type_numbering_request, DuplicateSetting,
    NumberingJoinError, NumberingSubMethod, SeriesNumberingMethod, VoucherTypeNumbering,
};
use bridge_tally_protocol::{decode_tally_xml_response_bytes_limited, ExpectedTallyTextEncoding};
use tally_protocol_simulator::{
    encode, ObservedRequest, ScenarioPlan, SequenceSimulator, WireEncoding,
};

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
        // The item list with an item that carries two dated HSN and two dated GST entries.
        NativeMasterKind::StockItems => include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/masters_stock_items_shape_lab_two_dates_live.utf16le.xml"
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
        NativeMasterKind::StockItems => 12,
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

/// The real series-level numbering capture (24 voucher types, one synthetic
/// book), with its company GUID replaced by the test double's company.
const NUMBERING_CAPTURE_GUID: &str = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2";

fn numbering_capture() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/voucher_type_numbering_series_live.utf16le.xml"
    ))
    .trim_start_matches('\u{feff}')
    .replace(NUMBERING_CAPTURE_GUID, GUID)
}

/// Each `VOUCHERTYPE` element of `text`, opening tag to closing tag.
fn type_blocks(text: &str) -> Vec<&str> {
    let mut blocks = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("<VOUCHERTYPE ") {
        let end = rest[start..].find("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
        blocks.push(&rest[start..start + end]);
        rest = &rest[start + end..];
    }
    blocks
}

/// The text of the first `<tag ...>` element in `block`.
fn element<'a>(block: &'a str, tag: &str) -> &'a str {
    let open = block.find(&format!("<{tag} ")).expect(tag);
    let text = open + block[open..].find('>').unwrap() + 1;
    &block[text..text + block[text..].find(&format!("</{tag}>")).unwrap()]
}

fn name_of(block: &str) -> &str {
    let at = block.find("NAME=\"").unwrap() + "NAME=\"".len();
    &block[at..at + block[at..].find('"').unwrap()]
}

/// A masters answer for the types of the real numbering capture: each type's
/// name, GUID, parent, type-level method and ids as the capture printed them,
/// in the shape of the captured masters answer.
fn masters_for_numbering_capture() -> String {
    let numbering = numbering_capture();
    let masters = capture(NativeMasterKind::VoucherTypes);
    let rows = type_blocks(&numbering)
        .into_iter()
        .map(|block| {
            let name = name_of(block);
            format!(
                "<VOUCHERTYPE NAME=\"{name}\" RESERVEDNAME=\"{name}\"><GUID TYPE=\"String\">{}</GUID><PARENT TYPE=\"String\">{}</PARENT><NUMBERINGMETHOD TYPE=\"String\">{}</NUMBERINGMETHOD><ISACTIVE TYPE=\"Logical\">Yes</ISACTIVE><ISOPTIONAL TYPE=\"Logical\">No</ISOPTIONAL><ALTERID TYPE=\"Number\">{}</ALTERID><MASTERID TYPE=\"Number\">{}</MASTERID><BRIDGECOMPANYGUID TYPE=\"String\">{GUID}</BRIDGECOMPANYGUID></VOUCHERTYPE>",
                element(block, "GUID"),
                element(block, "PARENT"),
                element(block, "NUMBERINGMETHOD"),
                element(block, "ALTERID"),
                element(block, "MASTERID"),
            )
        })
        .collect::<String>();
    let start = masters.find("<VOUCHERTYPE ").unwrap();
    let end = masters.rfind("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
    format!("{}{}{}", &masters[..start], rows, &masters[end..])
}

/// A numbering answer for any voucher-type masters answer: the real numbering
/// capture's envelope, and for each masters row the capture's row of the same
/// name (or its `Contra` row where the capture has none) with only the row's
/// name, GUID and ids set to the masters row's.
fn numbering_for(masters: &str) -> String {
    let real = numbering_capture();
    let templates = type_blocks(&real);
    let template = |name: &str| {
        templates
            .iter()
            .find(|block| name_of(block) == name)
            .or_else(|| templates.iter().find(|block| name_of(block) == "Contra"))
            .copied()
            .unwrap()
    };
    let set = |block: &str, tag: &str, value: &str| {
        let open = block.find(&format!("<{tag} ")).unwrap();
        let text = open + block[open..].find('>').unwrap() + 1;
        let end = text + block[text..].find(&format!("</{tag}>")).unwrap();
        format!("{}{}{}", &block[..text], value, &block[end..])
    };
    let rows = parse_native_masters(NativeMasterKind::VoucherTypes, masters, GUID)
        .unwrap()
        .rows
        .iter()
        .map(|row| {
            assert!(!row.name.contains(['&', '"', '<']), "{}", row.name);
            let block = template(&row.name);
            let tag_end = block.find('>').unwrap();
            let block = format!(
                "<VOUCHERTYPE NAME=\"{0}\" RESERVEDNAME=\"{0}\"{1}",
                row.name,
                &block[tag_end..]
            );
            let block = set(&block, "GUID", &row.guid);
            let block = set(&block, "ALTERID", &format!(" {}", row.alter_id));
            set(&block, "MASTERID", &format!(" {}", row.master_id))
        })
        .collect::<String>();
    let start = real.find("<VOUCHERTYPE ").unwrap();
    let end = real.rfind("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
    format!("{}{}{}", &real[..start], rows, &real[end..])
}

/// `text` with the first `from` at or after `anchor` replaced by `to`.
fn edit_after(text: &str, anchor: &str, from: &str, to: &str) -> String {
    let anchor = text.find(anchor).expect(anchor);
    let at = anchor + text[anchor..].find(from).expect(from);
    format!("{}{}{}", &text[..at], to, &text[at + from.len()..])
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

/// The opening, then the voucher-type collection pair, the numbering pair and
/// the closing extent (every plan of a voucher-types read up to its brackets).
fn through_closing_extent_numbered(
    opening_extent: String,
    collection: String,
    numbering: String,
    closing_extent: String,
) -> Vec<ScenarioPlan> {
    let mut plans = opening(opening_extent);
    pair(&mut plans, xml(collection));
    pair(&mut plans, xml(numbering));
    pair(&mut plans, xml(closing_extent));
    plans
}

/// The opening, then the voucher-type collection pair and the numbering pair:
/// all a read that is refused at once, on the numbering answer, sends.
fn through_numbering(
    opening_extent: String,
    collection: String,
    numbering: String,
) -> Vec<ScenarioPlan> {
    let mut plans = opening(opening_extent);
    pair(&mut plans, xml(collection));
    pair(&mut plans, xml(numbering));
    plans
}

/// A whole voucher-types read: also the identity and mode brackets after the
/// closing extent.
fn complete_numbered(
    opening_extent: String,
    collection: String,
    numbering: String,
    closing_extent: String,
) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans =
        through_closing_extent_numbered(opening_extent, collection, numbering, closing_extent);
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// The whole voucher-types read of the captured masters answer and the
/// numbering answer made for it, at a mark no kind sized by the mark would be
/// admitted at.
fn voucher_types_plans() -> Vec<ScenarioPlan> {
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    complete_numbered(
        extent_with_mark(50_000_000),
        masters,
        numbering,
        extent_with_mark(50_000_000),
    )
}

/// The largest master mark a collection read of `kind` is admitted at.
fn largest_admitted_mark(kind: NativeMasterKind) -> u64 {
    u64::try_from(MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(kind)).unwrap()
}

/// The requests the Tally double saw, or a message carrying both the double's error and the
/// runtime answer already obtained, so a failed `finish()` never hides what the runtime said.
fn observed_or_message(
    result: &dyn std::fmt::Debug,
    finished: std::io::Result<Vec<ObservedRequest>>,
) -> Result<Vec<ObservedRequest>, String> {
    finished.map_err(|error| {
        format!("the simulator's finish failed: {error:?}; the runtime had answered: {result:?}")
    })
}

fn observed_or_panic(
    result: &dyn std::fmt::Debug,
    finished: std::io::Result<Vec<ObservedRequest>>,
) -> Vec<ObservedRequest> {
    observed_or_message(result, finished).unwrap_or_else(|message| panic!("{message}"))
}

async fn run(
    plans: Vec<ScenarioPlan>,
    kind: MastersKind,
) -> (anyhow::Result<(MastersRead, CompanyBookExtent)>, usize) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let result = TallyRuntime::default()
        .fetch_masters_with_extent(config(&simulator), &identity(), kind)
        .await;
    let finished = observed_or_panic(&result, simulator.finish());
    (result, finished.len())
}

/// As [`run`], with the requests the Tally double received.
async fn run_observed(
    plans: Vec<ScenarioPlan>,
    kind: MastersKind,
) -> (
    anyhow::Result<(MastersRead, CompanyBookExtent)>,
    Vec<ObservedRequest>,
) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let result = TallyRuntime::default()
        .fetch_masters_with_extent(config(&simulator), &identity(), kind)
        .await;
    let finished = observed_or_panic(&result, simulator.finish());
    (result, finished)
}

/// The body hash the Tally double records for `request`.
fn request_hash(request: &str) -> String {
    sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        request,
    ))
}

fn cause<T: std::error::Error + 'static>(error: &anyhow::Error) -> Option<&T> {
    error.chain().find_map(|cause| cause.downcast_ref::<T>())
}

/// The kinds sized by the master mark before their request. Voucher types are
/// not: they are held by the checks after the read only.
const SIZED_KINDS: [NativeMasterKind; 6] = [
    NativeMasterKind::Godowns,
    NativeMasterKind::Units,
    NativeMasterKind::StockGroups,
    NativeMasterKind::StockItems,
    NativeMasterKind::CostCentres,
    NativeMasterKind::CostCategories,
];

#[test]
fn the_admission_edge_is_the_largest_mark_that_fits_and_the_next_refuses() {
    for (kind, largest) in [
        (NativeMasterKind::Godowns, 1_152),
        (NativeMasterKind::Units, 1_168),
        (NativeMasterKind::StockGroups, 1_160),
        // The item row is allowed for several dated entries (its fixed allowance is
        // 10,200 UTF-16 units against 4,373 measured), so the admitted mark is
        // the lowest of the kinds. The largest AlterID in the capture is 294.
        (NativeMasterKind::StockItems, 489),
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
        let plans = if kind == NativeMasterKind::VoucherTypes {
            let masters = capture(kind);
            complete_numbered(
                extent_with_mark(mark),
                masters.clone(),
                numbering_for(&masters),
                extent_with_mark(mark),
            )
        } else {
            complete(
                extent_with_mark(mark),
                capture(kind),
                extent_with_mark(mark),
            )
        };
        let (result, dispatched) = run(plans, MastersKind::Native(kind)).await;

        let (read, extent) = result.unwrap();
        // Voucher types read a second collection, the numbering, and each row
        // is paired with its result; no other kind reads or carries it.
        let rows = match (kind, read.rows) {
            (NativeMasterKind::VoucherTypes, MastersRows::VoucherTypes(types)) => {
                assert_eq!(dispatched, 18 + 4, "{kind:?}");
                types.len()
            }
            (_, MastersRows::Native(masters)) if kind != NativeMasterKind::VoucherTypes => {
                assert_eq!(dispatched, 18, "{kind:?}");
                masters.rows.len()
            }
            _ => panic!("{kind:?} is read as its own collection"),
        };
        assert_eq!(rows, captured_rows(kind), "{kind:?}");
        assert_eq!(
            extent.master_alter_id_high_water().map(|mark| mark.get()),
            Some(mark)
        );
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

        let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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
    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

    let error = result.expect_err("refused");
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

const VOUCHER_TYPES_KIND: MastersKind = MastersKind::Native(NativeMasterKind::VoucherTypes);

/// The whole sequence of a voucher-types read: the mode probe and identity (3),
/// the opening extent (4), the masters pair (4), the numbering pair (4), the
/// closing extent (4), identity and mode again (3).
const VOUCHER_TYPES_SEQUENCE: usize = 3 + 4 + 4 + 4 + 4 + 3;

#[tokio::test]
async fn voucher_types_send_the_masters_request_then_the_numbering_request_between_the_extents() {
    let (result, observed) = run_observed(voucher_types_plans(), VOUCHER_TYPES_KIND).await;
    result.unwrap();

    let name = identity().display_name().to_string();
    let masters = request_hash(&render_native_masters_request(
        NativeMasterKind::VoucherTypes,
        &name,
    ));
    let numbering = request_hash(&render_voucher_type_numbering_request(
        identity().company_name(),
    ));
    assert_ne!(masters, numbering);
    assert_eq!(observed.len(), VOUCHER_TYPES_SEQUENCE);
    let hash = |at: usize| observed[at].request_body_sha256.as_str();
    // Opening extent (twice), masters (twice), numbering (twice), closing
    // extent (twice): each a POST, with a health check between and after.
    let extent = hash(3);
    for at in [3, 5, 7, 9, 11, 13, 15, 17] {
        assert_eq!(observed[at].method, "POST", "{at}");
        assert_eq!(observed[at + 1].method, "GET", "{at}");
    }
    assert_eq!([hash(5), hash(15), hash(17)], [extent; 3]);
    assert_ne!(extent, masters);
    assert_ne!(extent, numbering);
    assert_eq!([hash(7), hash(9)], [masters.as_str(); 2]);
    assert_eq!([hash(11), hash(13)], [numbering.as_str(); 2]);
}

#[tokio::test]
async fn every_other_kind_sends_no_numbering_request() {
    let name = identity().display_name().to_string();
    let numbering = request_hash(&render_voucher_type_numbering_request(
        identity().company_name(),
    ));
    for kind in NativeMasterKind::ALL {
        if kind == NativeMasterKind::VoucherTypes {
            continue;
        }
        let mark = largest_admitted_mark(kind);
        let (result, observed) = run_observed(
            complete(
                extent_with_mark(mark),
                capture(kind),
                extent_with_mark(mark),
            ),
            MastersKind::Native(kind),
        )
        .await;
        result.unwrap();
        // The sequence of one collection pair, as before the numbering read.
        assert_eq!(observed.len(), 18, "{kind:?}");
        let masters = request_hash(&render_native_masters_request(kind, &name));
        assert_eq!(observed[7].request_body_sha256, masters, "{kind:?}");
        assert_eq!(observed[9].request_body_sha256, masters, "{kind:?}");
        assert!(
            observed
                .iter()
                .all(|request| request.request_body_sha256 != numbering),
            "{kind:?}"
        );
    }
    // The group snapshot neither.
    let (result, observed) = run_observed(
        complete(
            extent_with_mark(50_000_000),
            groups(),
            extent_with_mark(50_000_000),
        ),
        MastersKind::Groups,
    )
    .await;
    result.unwrap();
    assert_eq!(observed.len(), 18);
    assert!(observed
        .iter()
        .all(|request| request.request_body_sha256 != numbering));
}

#[tokio::test]
async fn each_type_gets_the_numbering_of_its_own_series() {
    // The real numbering capture, joined to a masters answer made from the same
    // capture's own identities: Journal, Payment and Receipt were set on the
    // screen, the other 21 types were not altered.
    let (result, dispatched) = run(
        complete_numbered(
            extent_with_mark(50_000_000),
            masters_for_numbering_capture(),
            numbering_capture(),
            extent_with_mark(50_000_000),
        ),
        VOUCHER_TYPES_KIND,
    )
    .await;
    assert_eq!(dispatched, VOUCHER_TYPES_SEQUENCE);
    let MastersRows::VoucherTypes(types) = result.unwrap().0.rows else {
        panic!("voucher types carry their numbering");
    };
    assert_eq!(types.len(), 24);
    let numbering = types
        .iter()
        .map(|(_, numbering)| numbering.clone())
        .collect::<Vec<_>>();
    let of = |name: &str| {
        let at = types.iter().position(|(row, _)| row.name == name).unwrap();
        &numbering[at]
    };
    let observed = |method, duplicates| VoucherTypeNumbering::Observed { method, duplicates };
    assert_eq!(
        of("Journal"),
        &observed(
            SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRetain),
            DuplicateSetting::NotPrevented
        )
    );
    assert_eq!(
        of("Payment"),
        &observed(SeriesNumberingMethod::Manual, DuplicateSetting::Prevented)
    );
    assert_eq!(
        of("Receipt"),
        &observed(
            SeriesNumberingMethod::Manual,
            DuplicateSetting::NotPrevented
        )
    );
    assert_eq!(
        numbering
            .iter()
            .filter(|result| matches!(
                result,
                VoucherTypeNumbering::Observed {
                    method: SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRetain),
                    duplicates: DuplicateSetting::NotPrevented
                }
            ))
            .count(),
        22
    );
}

#[tokio::test]
async fn the_numbering_read_adds_its_own_evidence() {
    let (result, _) = run(voucher_types_plans(), VOUCHER_TYPES_KIND).await;
    let (read, _) = result.unwrap();
    // Two collection pairs: both responses' bytes are counted, each read twice
    // (UTF-16, two bytes a unit), beside the brackets' own.
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    let collections =
        ((masters.encode_utf16().count() + numbering.encode_utf16().count()) * 2 * 2) as u64;
    let counted = u64::try_from(read.evidence.bytes).unwrap();
    assert!(counted >= collections, "{counted} < {collections}");
}

#[tokio::test]
async fn the_numbering_request_is_folded_into_the_evidence_after_the_masters_request() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    // A read that ends after the masters read: its evidence is the mode probe's
    // and the masters pair's (the premise breaks, and the closing extent is read).
    let (result, _) = run(
        through_closing_extent(extent_with_mark(25), masters.clone(), extent_with_mark(25)),
        VOUCHER_TYPES_KIND,
    )
    .await;
    let masters_only = result.expect_err("refused");
    let masters_only = &masters_only
        .downcast_ref::<RuntimeReadFailure>()
        .unwrap()
        .evidence;
    // A read that ends after the numbering read (its answer does not parse):
    // the same, and the numbering pair's.
    let (result, _) = run(
        through_numbering(
            extent_with_mark(50_000_000),
            masters,
            numbering.replacen("<STATUS>1</STATUS>", "<STATUS>0</STATUS>", 1),
        ),
        VOUCHER_TYPES_KIND,
    )
    .await;
    let with_numbering = result.expect_err("refused");
    let with_numbering = &with_numbering
        .downcast_ref::<RuntimeReadFailure>()
        .unwrap()
        .evidence;
    assert_ne!(with_numbering.request_sha256, masters_only.request_sha256);
    // The chain is `<earlier>:<next request>`: the numbering request's own hash,
    // the bytes `render_voucher_type_numbering_request` renders, is the next link.
    let numbering_request = request_hash(&render_voucher_type_numbering_request(
        identity().company_name(),
    ));
    assert_eq!(
        with_numbering.request_sha256,
        sha256_hex(format!("{}:{numbering_request}", masters_only.request_sha256).as_bytes())
    );
}

#[tokio::test]
async fn a_numbering_pair_that_changed_between_its_reads_refuses_the_whole_read() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    let mut plans = opening(extent_with_mark(50_000_000));
    pair(&mut plans, xml(masters));
    plans.extend([
        xml(numbering.clone()),
        status(),
        xml(format!("{numbering}\n")),
        status(),
    ]);
    let (result, dispatched) = run(plans, VOUCHER_TYPES_KIND).await;

    let error = result.expect_err("refused: no rows come back");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::MastersNumberingCollection
    ));
    // Not the masters read's refusal: it names which of the two drifted.
    assert_eq!(changed.safe_code(), "masters_numbering_collection_changed");
    assert_ne!(changed.safe_code(), "masters_collection_changed");
    // Refused at once: no closing extent.
    assert_eq!(dispatched, 3 + 4 + 4 + 4);
}

/// The voucher types as the lab's first read after a company load answered
/// them, and as its second read did (#2160): the first leaves one element out
/// of two rows. Both are captures of one book, the book `capture` came from.
fn first_read_after_load() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/masters_voucher_types_shape_lab_first_read_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
}

fn second_read_after_load() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/masters_voucher_types_shape_lab_second_read_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
}

/// The opening, then a voucher-types pair that disagrees (the first read, then
/// the second), then `rest`.
fn after_a_disagreeing_pair(
    opening_extent: String,
    rest: impl FnOnce(&mut Vec<ScenarioPlan>),
) -> Vec<ScenarioPlan> {
    let mut plans = opening(opening_extent);
    plans.extend([
        xml(first_read_after_load()),
        status(),
        xml(second_read_after_load()),
        status(),
    ]);
    rest(&mut plans);
    plans
}

fn digest(text: &str) -> String {
    sha256_hex(&encode(text, WireEncoding::Utf16Le))
}

fn joined(left: &str, right: &str) -> String {
    sha256_hex(format!("{left}:{right}").as_bytes())
}

/// The voucher-types request's own hash, beside the pair's two reads.
fn voucher_types_request_hash() -> String {
    request_hash(&render_native_masters_request(
        NativeMasterKind::VoucherTypes,
        identity().display_name(),
    ))
}

#[tokio::test]
async fn a_voucher_types_pair_that_disagreed_on_the_first_read_is_read_once_more() {
    let numbering = numbering_for(&second_read_after_load());
    let companies = xml(companies());
    let mut plans = after_a_disagreeing_pair(extent_with_mark(50_000_000), |plans| {
        pair(plans, xml(second_read_after_load()));
        pair(plans, xml(numbering));
        pair(plans, xml(extent_with_mark(50_000_000)));
    });
    plans.extend([companies.clone(), status(), companies]);
    let (result, observed) = run_observed(plans, VOUCHER_TYPES_KIND).await;
    let (read, _) = result.unwrap();

    // The 22 requests of an agreeing read, and one more pair.
    assert_eq!(observed.len(), VOUCHER_TYPES_SEQUENCE + 4);
    let masters = voucher_types_request_hash();
    let hash = |at: usize| observed[at].request_body_sha256.as_str();
    // Opening extent (twice), then the masters request four times: the pair
    // that disagreed and the pair that agreed.
    assert_eq!(
        [hash(7), hash(9), hash(11), hash(13)],
        [masters.as_str(); 4]
    );
    for at in [7, 9, 11, 13, 15, 17, 19, 21] {
        assert_eq!(observed[at].method, "POST", "{at}");
        assert_eq!(observed[at + 1].method, "GET", "{at}");
    }
    let MastersRows::VoucherTypes(types) = &read.rows else {
        panic!("not voucher types");
    };
    assert_eq!(types.len(), captured_rows(NativeMasterKind::VoucherTypes));

    // The answer records the first disagreement: the digests of that pair, the
    // two reads the lab captured, whose bytes differ.
    let first = read
        .first_pair_differed
        .expect("the first pair is recorded");
    let (first_body, second_body) = (first_read_after_load(), second_read_after_load());
    assert_ne!(digest(&first_body), digest(&second_body));
    assert_eq!(first.request_sha256, joined(&masters, &masters));
    assert_eq!(
        first.response_sha256,
        joined(&digest(&first_body), &digest(&second_body))
    );
    assert_eq!(
        first.bytes,
        encode(&first_body, WireEncoding::Utf16Le).len()
            + encode(&second_body, WireEncoding::Utf16Le).len()
    );
    // The read's own evidence counts the first pair and the pair that agreed.
    assert!(
        read.evidence.bytes >= first.bytes + 2 * encode(&second_body, WireEncoding::Utf16Le).len(),
        "{} < {}",
        read.evidence.bytes,
        first.bytes
    );
}

#[tokio::test]
async fn a_voucher_types_pair_that_agrees_records_no_first_disagreement() {
    let (result, observed) = run_observed(voucher_types_plans(), VOUCHER_TYPES_KIND).await;
    let (read, _) = result.unwrap();
    assert!(read.first_pair_differed.is_none());
    assert_eq!(observed.len(), VOUCHER_TYPES_SEQUENCE);
}

#[tokio::test]
async fn a_voucher_types_pair_that_disagrees_twice_is_refused_after_one_more_pair() {
    // The second pair plays the same two captured reads the other way round.
    let plans = after_a_disagreeing_pair(extent_with_mark(50_000_000), |plans| {
        plans.extend([
            xml(second_read_after_load()),
            status(),
            xml(first_read_after_load()),
            status(),
        ]);
    });
    let (result, dispatched) = run(plans, VOUCHER_TYPES_KIND).await;

    let error = result.expect_err("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::MastersCollection
    ));
    assert_eq!(changed.safe_code(), "masters_collection_changed");
    // Refused after the second pair, never a third: the mode probe and
    // identity (3), the opening extent (4), and two pairs (8). No numbering
    // pair, no closing extent.
    assert_eq!(dispatched, 3 + 4 + 4 + 4);
    // All four reads are kept as evidence, not only the last pair's.
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    let (first_body, second_body) = (first_read_after_load(), second_read_after_load());
    let reads = 2
        * (encode(&first_body, WireEncoding::Utf16Le).len()
            + encode(&second_body, WireEncoding::Utf16Le).len());
    assert!(evidence.bytes >= reads, "{} < {reads}", evidence.bytes);
}

#[tokio::test]
async fn a_failure_of_the_second_voucher_types_pair_stays_a_transport_failure() {
    let plans = after_a_disagreeing_pair(extent_with_mark(50_000_000), |plans| {
        plans.push(xml(second_read_after_load()).with_http_status(500));
    });
    let (result, dispatched) = run(plans, VOUCHER_TYPES_KIND).await;

    let error = result.expect_err("refused");
    assert!(matches!(
        cause::<bridge_tally_transport::TallyTransportError>(&error),
        Some(bridge_tally_transport::TallyTransportError::HttpStatus { status: 500 })
    ));
    assert!(cause::<MastersReadError>(&error).is_none());
    assert!(cause::<PairedReadValidationError>(&error).is_none());
    assert_eq!(dispatched, 3 + 4 + 4 + 1);
}

#[tokio::test]
async fn a_book_that_moved_during_a_repeated_voucher_types_read_is_still_reported_as_moved() {
    let numbering = numbering_for(&second_read_after_load());
    let plans = after_a_disagreeing_pair(extent_with_mark(50_000_000), |plans| {
        pair(plans, xml(second_read_after_load()));
        pair(plans, xml(numbering));
        pair(plans, xml(extent_with_mark(50_000_001)));
    });
    let (result, dispatched) = run(plans, VOUCHER_TYPES_KIND).await;

    assert_reported_as_moved(&result.expect_err("refused"));
    // Refused at the closing extent: 3 + 4, two collection pairs, the numbering
    // pair and the closing extent.
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4 + 4);
}

#[tokio::test]
async fn a_type_saved_during_a_repeated_numbering_read_is_still_refused_by_the_mark() {
    let second = second_read_after_load();
    let highest = parse_native_masters(NativeMasterKind::VoucherTypes, &second, GUID)
        .unwrap()
        .rows
        .iter()
        .map(|row| row.alter_id)
        .max()
        .unwrap();
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    plans.extend([
        xml(first_read_after_load()),
        status(),
        xml(second.clone()),
        status(),
    ]);
    pair(&mut plans, xml(second));
    // A mark below the highest AlterID: a type was saved after the mark was read.
    let (result, observed) = run_numbering(plans, highest - 1).await;
    assert_numbering_premise_refusal(
        &result.expect_err("refused"),
        "masters_alter_id_above_master_mark",
    );
    assert_eq!(observed.len(), 3 + 4 + 4);
}

#[tokio::test]
async fn the_numbering_read_repeats_a_voucher_types_pair_that_disagreed() {
    let second = second_read_after_load();
    let numbering = numbering_for(&second);
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies.clone()];
    plans.extend([
        xml(first_read_after_load()),
        status(),
        xml(second.clone()),
        status(),
    ]);
    pair(&mut plans, xml(second));
    pair(&mut plans, xml(numbering));
    plans.extend([companies.clone(), status(), companies]);
    let (result, observed) = run_numbering(plans, 50_000_000).await;
    let read = result.unwrap();
    assert_eq!(observed.len(), NUMBERING_READ_SEQUENCE + 4);
    assert_eq!(
        read.types().len(),
        captured_rows(NativeMasterKind::VoucherTypes)
    );
    // The numbering pair follows the repeated masters pair.
    let masters = voucher_types_request_hash();
    let posts = observed
        .iter()
        .filter(|request| request.method == "POST")
        .map(|request| request.request_body_sha256.as_str())
        .collect::<Vec<_>>();
    assert_eq!(&posts[2..6], [masters.as_str(); 4]);
}

#[tokio::test]
async fn the_numbering_read_refuses_a_voucher_types_pair_that_disagrees_twice() {
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    plans.extend([
        xml(first_read_after_load()),
        status(),
        xml(second_read_after_load()),
        status(),
        xml(second_read_after_load()),
        status(),
        xml(first_read_after_load()),
        status(),
    ]);
    let (result, observed) = run_numbering(plans, 50_000_000).await;
    assert!(matches!(
        cause::<PairedReadValidationError>(&result.expect_err("refused")),
        Some(PairedReadValidationError::MastersCollection)
    ));
    assert_eq!(observed.len(), 3 + 4 + 4);
}

#[tokio::test]
async fn a_numbering_answer_that_does_not_parse_refuses_at_once_with_the_parsers_cause() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    for (refused, expected) in [
        (
            numbering.replacen("<STATUS>1</STATUS>", "<STATUS>0</STATUS>", 1),
            NativeMastersError::TallyReportedFailure,
        ),
        (
            numbering.replacen("</ENVELOPE>", "", 1),
            NativeMastersError::Malformed("masters_envelope_unterminated"),
        ),
    ] {
        let (result, dispatched) = run(
            through_numbering(extent_with_mark(50_000_000), masters.clone(), refused),
            VOUCHER_TYPES_KIND,
        )
        .await;
        let error = result.expect_err("refused: no rows come back");
        assert_eq!(cause::<NativeMastersError>(&error), Some(&expected));
        // The masters pair and the numbering pair were read, nothing after.
        assert_eq!(dispatched, 3 + 4 + 4 + 4);
        // Both completed reads are kept as evidence.
        let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
        assert!(!evidence.response_sha256.is_empty());
    }
    // A type with no GUID in the numbering answer: a typed parse error, not a default.
    let without_guid = {
        let at = numbering.find("<VOUCHERTYPE NAME=\"Payment\"").unwrap();
        let from = at + numbering[at..].find("<GUID ").unwrap();
        let to = from + numbering[from..].find("</GUID>").unwrap() + "</GUID>".len();
        format!("{}{}", &numbering[..from], &numbering[to..])
    };
    let (result, _) = run(
        through_numbering(extent_with_mark(50_000_000), masters, without_guid),
        VOUCHER_TYPES_KIND,
    )
    .await;
    assert_eq!(
        cause::<NativeMastersError>(&result.expect_err("refused")),
        Some(&NativeMastersError::Malformed(
            "masters_numbering_guid_missing"
        ))
    );
}

#[tokio::test]
async fn a_numbering_answer_that_does_not_join_refuses_the_whole_read_after_the_closing_extent() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    let without_payroll = {
        let at = numbering.find("<VOUCHERTYPE NAME=\"Payroll\"").unwrap();
        let end = at + numbering[at..].find("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
        format!("{}{}", &numbering[..at], &numbering[end..])
    };
    // A type in the masters answer and not in the numbering answer.
    let (result, dispatched) = run(
        through_closing_extent_numbered(
            extent_with_mark(50_000_000),
            masters.clone(),
            without_payroll.clone(),
            extent_with_mark(50_000_000),
        ),
        VOUCHER_TYPES_KIND,
    )
    .await;
    let error = result.expect_err("refused: no rows come back");
    let refused = cause::<MastersReadError>(&error).expect("a join refusal");
    assert!(matches!(
        refused,
        MastersReadError::NumberingJoin(NumberingJoinError::TypeOnOneSideOnly)
    ));
    assert_eq!(refused.safe_code(), "masters_numbering_names_differ");
    // Held until the closing extent was read, and it had not moved.
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4);
    // The completed reads are kept as evidence.
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    assert!(!evidence.response_sha256.is_empty());

    // The same GUID under another name.
    let renamed = edit_after(
        &numbering,
        "<VOUCHERTYPE NAME=\"Payroll\"",
        "<VOUCHERTYPE NAME=\"Payroll\" RESERVEDNAME=\"Payroll\">",
        "<VOUCHERTYPE NAME=\"Payrolls\" RESERVEDNAME=\"Payrolls\">",
    );
    let (result, _) = run(
        through_closing_extent_numbered(
            extent_with_mark(50_000_000),
            masters.clone(),
            renamed,
            extent_with_mark(50_000_000),
        ),
        VOUCHER_TYPES_KIND,
    )
    .await;
    let error = result.expect_err("refused");
    let refused = cause::<MastersReadError>(&error).expect("a join refusal");
    assert!(matches!(
        refused,
        MastersReadError::NumberingJoin(NumberingJoinError::SameGuidDifferentNames)
    ));
    assert_eq!(refused.safe_code(), "masters_numbering_identity_differs");

    // The book moved between the two reads: reported as moved, not as the
    // join its new type broke.
    let (result, dispatched) = run(
        through_closing_extent_numbered(
            extent_with_mark(50_000_000),
            masters,
            without_payroll,
            extent_with_mark(50_000_001),
        ),
        VOUCHER_TYPES_KIND,
    )
    .await;
    let error = result.expect_err("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(changed, PairedReadValidationError::MastersExtent));
    assert!(cause::<MastersReadError>(&error).is_none());
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4);
}

#[tokio::test]
async fn a_book_that_moved_between_the_two_reads_is_reported_as_moved() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    let (result, dispatched) = run(
        through_closing_extent_numbered(
            extent_with_mark(50_000_000),
            masters,
            numbering,
            extent_with_mark(50_000_010),
        ),
        VOUCHER_TYPES_KIND,
    )
    .await;
    let error = result.expect_err("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(changed, PairedReadValidationError::MastersExtent));
    assert_eq!(changed.safe_code(), "masters_extent_changed");
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4);
}

#[tokio::test]
async fn a_numbering_answer_that_repeats_a_guid_is_refused_at_once_by_the_parser() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    let numbering = numbering_for(&masters);
    let guid_of = |name: &str| {
        parse_voucher_type_numbering(&numbering)
            .unwrap()
            .into_iter()
            .find(|row| row.name == name)
            .unwrap()
            .guid
    };
    let repeated = numbering.replacen(&guid_of("Payment"), &guid_of("Journal"), 1);
    assert_ne!(repeated, numbering);
    let (result, dispatched) = run(
        through_numbering(extent_with_mark(50_000_000), masters, repeated),
        VOUCHER_TYPES_KIND,
    )
    .await;
    let error = result.expect_err("refused: no rows come back");
    assert_eq!(
        cause::<NativeMastersError>(&error),
        Some(&NativeMastersError::DuplicateGuid)
    );
    assert_eq!(dispatched, 3 + 4 + 4 + 4);
}

#[test]
fn the_numbering_premise_checks_refuse_each_way_its_answer_can_outgrow_its_mark() {
    let rows = parse_voucher_type_numbering(&numbering_capture()).unwrap();
    assert_eq!(rows.len(), 24);
    let violated = |result: Result<(), MastersReadError>| match result {
        Err(error @ MastersReadError::PremiseViolated(reason)) => {
            assert_eq!(error.safe_code(), "masters_bound_premise_violated");
            reason
        }
        other => panic!("{other:?}"),
    };
    // 24 rows, a mark of 24, a response that just fits.
    assert!(check_numbering_premise(&rows, 24, 1_000, 1_000).is_ok());
    // One row more than the mark.
    assert_eq!(
        violated(check_numbering_premise(&rows, 23, 0, u64::MAX)),
        "masters_numbering_rows_exceed_master_mark"
    );
    // A response one byte over the budget it is held to.
    assert_eq!(
        violated(check_numbering_premise(&rows, 24, 1_001, 1_000)),
        "masters_numbering_response_over_admitted_bytes"
    );
    // The numbering answer is held to the whole budget, as voucher types are.
    let budget = u64::try_from(MASTERS_RESPONSE_BUDGET_BYTES).unwrap();
    assert!(check_numbering_premise(&rows, 24, MASTERS_RESPONSE_BUDGET_BYTES, budget).is_ok());
    assert!(check_numbering_premise(&rows, 24, MASTERS_RESPONSE_BUDGET_BYTES + 1, budget).is_err());
}

/// The numbering answer with `count` more types than `numbering` holds, each a
/// copy of its `Contra` row under a new name and GUID.
fn numbering_with_extra_types(numbering: &str, count: usize) -> String {
    let contra = type_blocks(numbering)
        .into_iter()
        .find(|block| name_of(block) == "Contra")
        .unwrap()
        .to_string();
    let guid = element(&contra, "GUID").to_string();
    let extra = (0..count)
        .map(|at| {
            contra
                .replacen("NAME=\"Contra\"", &format!("NAME=\"Extra {at}\""), 1)
                .replacen(&guid, &format!("{GUID}-{:08x}", 0x1000_0000 + at), 1)
        })
        .collect::<String>();
    let at = numbering.rfind("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
    format!("{}{extra}{}", &numbering[..at], &numbering[at..])
}

/// A voucher-types read whose numbering answer is `numbering`, at the master
/// mark `mark` and with the closing extent's mark `closing`: held through the
/// closing extent, which is all such a read sends.
async fn read_with_numbering(
    numbering: String,
    mark: u64,
    closing: u64,
) -> (anyhow::Result<(MastersRead, CompanyBookExtent)>, usize) {
    run(
        through_closing_extent_numbered(
            extent_with_mark(mark),
            capture(NativeMasterKind::VoucherTypes),
            numbering,
            extent_with_mark(closing),
        ),
        VOUCHER_TYPES_KIND,
    )
    .await
}

fn assert_numbering_premise_refusal(error: &anyhow::Error, reason: &'static str) {
    let violated = cause::<MastersReadError>(error).expect("a premise refusal");
    assert!(
        matches!(violated, MastersReadError::PremiseViolated(found) if *found == reason),
        "{violated:?}"
    );
    assert_eq!(violated.safe_code(), "masters_bound_premise_violated");
    assert!(cause::<PairedReadValidationError>(error).is_none());
}

fn assert_reported_as_moved(error: &anyhow::Error) {
    let changed = cause::<PairedReadValidationError>(error).expect("a stability refusal");
    assert!(matches!(changed, PairedReadValidationError::MastersExtent));
    assert_eq!(changed.safe_code(), "masters_extent_changed");
    assert!(cause::<MastersReadError>(error).is_none());
}

#[tokio::test]
async fn a_numbering_answer_with_more_rows_than_the_mark_is_a_premise_refusal() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    // The mark every masters row sits at or under, with one numbering row more
    // than it: the masters answer's own premise holds (26 rows, none above it).
    let rows = parse_native_masters(NativeMasterKind::VoucherTypes, &masters, GUID)
        .unwrap()
        .rows;
    let mark = rows.iter().map(|row| row.alter_id).max().unwrap();
    let extra = usize::try_from(mark).unwrap() + 1 - rows.len();
    let numbering = numbering_with_extra_types(&numbering_for(&masters), extra);
    assert_eq!(
        parse_voucher_type_numbering(&numbering).unwrap().len() as u64,
        mark + 1
    );

    let (result, dispatched) = read_with_numbering(numbering.clone(), mark, mark).await;
    let error = result.expect_err("refused: no rows come back");
    assert_numbering_premise_refusal(&error, "masters_numbering_rows_exceed_master_mark");
    // Held until the closing extent was read, and it had not moved.
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4);

    // The book moved between the reads: reported as moved.
    let (result, dispatched) = read_with_numbering(numbering, mark, mark + 1).await;
    assert_reported_as_moved(&result.expect_err("refused"));
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4);

    // One row fewer is within the premise: the read goes on to the join, which
    // refuses the extra types as a numbering with no masters row.
    let within = numbering_with_extra_types(&numbering_for(&masters), extra - 1);
    let (result, _) = read_with_numbering(within, mark, mark).await;
    let error = result.expect_err("refused by the join");
    assert!(matches!(
        cause::<MastersReadError>(&error),
        Some(MastersReadError::NumberingJoin(
            NumberingJoinError::TypeOnOneSideOnly
        ))
    ));
}

#[tokio::test]
async fn a_numbering_answer_over_the_admitted_bytes_is_a_premise_refusal() {
    let masters = capture(NativeMasterKind::VoucherTypes);
    // A filler the parser does not read, past the whole budget in UTF-16.
    let filler = MASTERS_RESPONSE_BUDGET_BYTES / 2 + 100_000;
    let numbering = numbering_for(&masters).replacen(
        "<DATA>",
        &format!("<PAD>{}</PAD><DATA>", "x".repeat(filler)),
        1,
    );

    let (result, dispatched) = read_with_numbering(numbering.clone(), 50_000_000, 50_000_000).await;
    let error = result.expect_err("refused: no rows come back");
    assert_numbering_premise_refusal(&error, "masters_numbering_response_over_admitted_bytes");
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4);

    let (result, dispatched) = read_with_numbering(numbering, 50_000_000, 50_000_001).await;
    assert_reported_as_moved(&result.expect_err("refused"));
    assert_eq!(dispatched, 3 + 4 + 4 + 4 + 4);
}

// The numbering read a voucher sent without a number is admitted on
// (bridge#724): bounded by a master mark read before it, with no book extent.

/// The whole sequence: the mode probe and identity (3), the masters pair (4),
/// the numbering pair (4), identity and mode again (3).
const NUMBERING_READ_SEQUENCE: usize = 3 + 4 + 4 + 3;

/// A whole numbering read of `masters` and `numbering`.
fn numbering_read_plans(masters: String, numbering: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies.clone()];
    pair(&mut plans, xml(masters));
    pair(&mut plans, xml(numbering));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

async fn run_numbering(
    plans: Vec<ScenarioPlan>,
    master_mark: u64,
) -> (anyhow::Result<VoucherNumberingRead>, Vec<ObservedRequest>) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let result = TallyRuntime::default()
        .fetch_voucher_type_numbering(config(&simulator), &identity(), master_mark)
        .await;
    let finished = observed_or_panic(&result, simulator.finish());
    (result, finished)
}

/// The largest AlterID among the voucher types of the real numbering capture.
fn captured_highest_alter_id() -> u64 {
    parse_native_masters(
        NativeMasterKind::VoucherTypes,
        &masters_for_numbering_capture(),
        GUID,
    )
    .unwrap()
    .rows
    .iter()
    .map(|row| row.alter_id)
    .max()
    .unwrap()
}

// Waits out the simulator's fixed 30-second plan deadline: it has no shorter option, and none is
// added to production code for this test.
#[tokio::test]
async fn a_failed_finish_reports_the_runtime_answer_as_well_as_the_simulators_error() {
    let mut plans = numbering_read_plans(masters_for_numbering_capture(), numbering_capture());
    plans.push(status());
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let result = TallyRuntime::default()
        .fetch_voucher_type_numbering(config(&simulator), &identity(), captured_highest_alter_id())
        .await;
    assert!(result.is_ok(), "the runtime read must succeed: {result:?}");
    let message =
        observed_or_message(&result, simulator.finish()).expect_err("plan 11 never asked");
    assert!(message.contains("NoRequestForPlan"), "{message}");
    assert!(message.contains(&format!("{result:?}")), "{message}");
}

#[tokio::test]
async fn the_numbering_read_sends_the_masters_pair_then_the_numbering_pair_and_no_extent() {
    let mark = captured_highest_alter_id();
    let (result, observed) = run_numbering(
        numbering_read_plans(masters_for_numbering_capture(), numbering_capture()),
        mark,
    )
    .await;
    let read = result.unwrap();
    assert_eq!(observed.len(), NUMBERING_READ_SEQUENCE);
    let name = identity().display_name().to_string();
    let masters = request_hash(&render_native_masters_request(
        NativeMasterKind::VoucherTypes,
        &name,
    ));
    let numbering = request_hash(&render_voucher_type_numbering_request(
        identity().company_name(),
    ));
    let posts = observed
        .iter()
        .filter(|request| request.method == "POST")
        .map(|request| request.request_body_sha256.as_str())
        .collect::<Vec<_>>();
    // Six company-list POSTs of the brackets, and the two pairs in order.
    assert_eq!(
        &posts[2..6],
        [
            masters.as_str(),
            masters.as_str(),
            numbering.as_str(),
            numbering.as_str()
        ]
    );
    assert_eq!(posts.len(), 2 + 4 + 2);
    assert_eq!(read.master_mark(), mark);
    assert_eq!(read.company_guid(), GUID);
    assert_eq!(read.types().len(), 24);
    let of = |name: &str| {
        read.types()
            .iter()
            .find(|(type_name, _)| type_name == name)
            .map(|(_, numbering)| numbering.clone())
            .unwrap()
    };
    assert_eq!(
        of("Payment"),
        VoucherTypeNumbering::Observed {
            method: SeriesNumberingMethod::Manual,
            duplicates: DuplicateSetting::Prevented
        }
    );
    assert_eq!(
        of("Journal"),
        VoucherTypeNumbering::Observed {
            method: SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRetain),
            duplicates: DuplicateSetting::NotPrevented
        }
    );
    // Both pairs are in the read's evidence.
    let collections = ((masters_for_numbering_capture().encode_utf16().count()
        + numbering_capture().encode_utf16().count())
        * 2
        * 2) as u64;
    assert!(u64::try_from(read.evidence().bytes).unwrap() >= collections);
}

/// The mark is read before this call, so a voucher type saved after it carries
/// an AlterID above the mark: the read refuses rather than admit an observation
/// newer than the mark the post is compared by later.
#[tokio::test]
async fn a_voucher_type_saved_after_the_mark_refuses_the_read() {
    let mark = captured_highest_alter_id() - 1;
    let mut plans = numbering_read_plans(masters_for_numbering_capture(), numbering_capture());
    plans.truncate(3 + 4);
    let (result, observed) = run_numbering(plans, mark).await;
    let error = result.expect_err("refused");
    assert!(matches!(
        cause::<MastersReadError>(&error),
        Some(MastersReadError::PremiseViolated(
            "masters_alter_id_above_master_mark"
        ))
    ));
    // Refused on the masters answer: the numbering was never requested.
    assert_eq!(observed.len(), 3 + 4);
}

#[tokio::test]
async fn the_numbering_read_refuses_education_before_any_collection() {
    let (result, observed) =
        run_numbering(vec![status(), xml(education(&companies()))], 50_000_000).await;
    let error = result.expect_err("refused");
    assert!(matches!(
        cause::<MastersReadError>(&error),
        Some(MastersReadError::EducationUnqualified)
    ));
    assert_eq!(observed.len(), 2);
}

#[tokio::test]
async fn a_numbering_pair_that_changed_refuses_the_numbering_read_by_its_own_code() {
    let masters = masters_for_numbering_capture();
    let numbering = numbering_capture();
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    pair(&mut plans, xml(masters));
    plans.extend([
        xml(numbering.clone()),
        status(),
        xml(format!("{numbering}\n")),
        status(),
    ]);
    let (result, observed) = run_numbering(plans, 50_000_000).await;
    let error = result.expect_err("refused");
    assert!(matches!(
        cause::<PairedReadValidationError>(&error),
        Some(PairedReadValidationError::MastersNumberingCollection)
    ));
    assert_eq!(observed.len(), 3 + 4 + 4);
}

#[tokio::test]
async fn a_numbering_answer_that_does_not_join_keeps_the_joins_code() {
    // The numbering answer drops the masters answer's Journal.
    let masters = masters_for_numbering_capture();
    let numbering = numbering_capture();
    let journal = type_blocks(&numbering)
        .into_iter()
        .find(|block| name_of(block) == "Journal")
        .unwrap()
        .to_string();
    let mut plans = numbering_read_plans(masters, numbering.replacen(&journal, "", 1));
    plans.truncate(3 + 4 + 4);
    let (result, observed) = run_numbering(plans, 50_000_000).await;
    let error = result.expect_err("refused");
    assert!(matches!(
        cause::<MastersReadError>(&error),
        Some(MastersReadError::NumberingJoin(_))
    ));
    assert_eq!(observed.len(), 3 + 4 + 4);
}

#[tokio::test]
async fn a_transport_failure_during_the_numbering_read_stays_a_transport_failure() {
    let masters = masters_for_numbering_capture();
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    pair(&mut plans, xml(masters));
    plans.push(xml(numbering_capture()).with_http_status(500));
    let (result, observed) = run_numbering(plans, 50_000_000).await;
    let error = result.expect_err("refused");
    assert!(matches!(
        cause::<bridge_tally_transport::TallyTransportError>(&error),
        Some(bridge_tally_transport::TallyTransportError::HttpStatus { status: 500 })
    ));
    assert!(cause::<MastersReadError>(&error).is_none());
    assert!(cause::<PairedReadValidationError>(&error).is_none());
    assert_eq!(observed.len(), 3 + 4 + 1);
}

#[tokio::test]
async fn a_tally_error_inside_a_stock_item_list_is_refused_at_once() {
    // `ERROR` inside a rate row of a list the parser now reads: the read is
    // refused with Tally's reported failure, never skipped as unmodelled
    // content and never held behind the closing extent read.
    let captured = capture(NativeMasterKind::StockItems);
    let at = captured.find("<RATEDETAILS.LIST>").unwrap() + "<RATEDETAILS.LIST>".len();
    let failed = format!(
        "{}<LINEERROR>x</LINEERROR>{}",
        &captured[..at],
        &captured[at..]
    );
    let mut plans = opening(extents());
    pair(&mut plans, xml(failed));
    let (result, dispatched) = run(plans, MastersKind::Native(NativeMasterKind::StockItems)).await;

    let error = result.expect_err("refused");
    assert_eq!(
        cause::<NativeMastersError>(&error),
        Some(&NativeMastersError::TallyReportedFailure)
    );
    // Mode probe and identity (3), opening extent (4), the collection pair (4).
    assert_eq!(dispatched, 11);
}

#[tokio::test]
async fn a_stock_item_row_with_too_many_real_entries_is_the_parsers_typed_refusal() {
    // Eight more copies of the item's real 22 September 2025 GST entry, each with
    // a new valid date: the row exceeds the bound (16,344 UTF-16 units) and the
    // parser refuses it, at once and by its own code.
    let captured = capture(NativeMasterKind::StockItems);
    let marker =
        "<GSTDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>";
    let start = captured.find(marker).unwrap();
    let end =
        start + captured[start..].find("</GSTDETAILS.LIST>").unwrap() + "</GSTDETAILS.LIST>".len();
    let copies = (1..=8)
        .map(|month| captured[start..end].replace("20250922", &format!("2026{month:02}01")))
        .collect::<String>();
    let long_row = format!("{}{copies}{}", &captured[..end], &captured[end..]);
    let mut plans = opening(extents());
    pair(&mut plans, xml(long_row));
    let (result, dispatched) = run(plans, MastersKind::Native(NativeMasterKind::StockItems)).await;

    let error = result.expect_err("refused");
    assert_eq!(
        cause::<NativeMastersError>(&error),
        Some(&NativeMastersError::RowExceedsBound),
        "{error:?}"
    );
    // Mode probe and identity (3), opening extent (4), the collection pair (4).
    assert_eq!(dispatched, 11);
}

#[test]
fn stock_items_are_named_by_their_kind_and_only_by_it() {
    assert_eq!(
        MastersKind::parse("stock_items"),
        Some(MastersKind::Native(NativeMasterKind::StockItems))
    );
    for kind in NativeMasterKind::ALL {
        let name = MastersKind::Native(kind).as_str();
        assert_eq!(MastersKind::parse(name), Some(MastersKind::Native(kind)));
    }
    assert_eq!(MastersKind::parse("stock_item"), None);
    assert_eq!(MastersKind::parse("Stock_Items"), None);
}

const SHAPE_ITEMS_BASELINE: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/masters_stock_items_shape_lab_baseline_live.utf16le.xml"
);
const SHAPE_ITEMS_TWO_DATES: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/masters_stock_items_shape_lab_two_dates_live.utf16le.xml"
);
const SHAPE_COUNT: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_shape_lab_live.utf16le.xml"
);
const STOCK_CREATE_ITEMS_BEFORE: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-items-before.utf16le.xml"
);
const STOCK_CREATE_GROUPS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-groups.utf16le.xml"
);
const STOCK_CREATE_UNITS: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/stock-create-lab-units.utf16le.xml"
);
const STOCK_CREATE_GUID: &str = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1";

fn admission_capture(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("the committed UTF-16 capture decodes")
    .text
}

fn shape_identity() -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture("BRIDGE SHAPE LAB", CAPTURE_GUID)
}

fn stock_create_identity() -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture("BRIDGE STOCK CREATE LAB", STOCK_CREATE_GUID)
}

fn stable_admission_pair(body: String) -> NativePairedRead {
    let encoded_bytes = body.encode_utf16().count() * 2;
    NativePairedRead::Stable {
        body,
        encoded_bytes,
        encoded_sha256: String::new(),
    }
}

fn drifted_admission_pair() -> NativePairedRead {
    NativePairedRead::Drifted(RuntimeReadEvidence::empty())
}

/// The item and count files are real XML from one book but different capture
/// times, and these item answers lack the three candidate scalars added for R1.
/// They exercise admission logic, not a live eleven-field item/count pairing.
fn admit_shape_candidate(
    items: String,
    count: String,
    master_mark: u64,
) -> anyhow::Result<AdmittedCandidateRead> {
    admit_candidate_stock_item_read(
        StockItemReadSize::new(StockItemReadKind::Candidate, master_mark)?,
        &shape_identity(),
        stable_admission_pair(items),
        stable_admission_pair(count),
    )
}

fn with_stock_item_count(count: &str, value: Option<&str>) -> String {
    assert_eq!(count.matches("<NUMSTOCKITEMS ").count(), 1);
    let start = count.find("<NUMSTOCKITEMS ").unwrap();
    let end = start + count[start..].find("</NUMSTOCKITEMS>").unwrap() + "</NUMSTOCKITEMS>".len();
    let replacement = value.map_or_else(String::new, |value| {
        format!("<NUMSTOCKITEMS TYPE=\"Number\">{value}</NUMSTOCKITEMS>")
    });
    format!("{}{}{}", &count[..start], replacement, &count[end..])
}

#[test]
fn candidate_size_admission_keeps_the_489_mark_edge() {
    let admitted = StockItemReadSize::new(StockItemReadKind::Candidate, 489).unwrap();
    assert_eq!(admitted.admitted_bytes(), 15_984_432);
    assert!(matches!(
        StockItemReadSize::new(StockItemReadKind::Candidate, 490),
        Err(MastersReadError::TooLarge {
            master_alter_id: 490,
            estimated_bytes: 16_017_120,
            limit_bytes: 16_000_000,
            limit_master_alter_id: 489,
        })
    ));
}

#[test]
fn candidate_admission_requires_stable_item_and_count_pairs() {
    let items = admission_capture(SHAPE_ITEMS_BASELINE);
    let count = admission_capture(SHAPE_COUNT);
    let candidate_drift = admit_candidate_stock_item_read(
        StockItemReadSize::new(StockItemReadKind::Candidate, 489).unwrap(),
        &shape_identity(),
        drifted_admission_pair(),
        stable_admission_pair(count.clone()),
    )
    .expect_err("a drifted candidate is not admitted");
    assert!(matches!(
        cause::<PairedReadValidationError>(&candidate_drift),
        Some(PairedReadValidationError::MastersCollection)
    ));

    let count_drift = admit_candidate_stock_item_read(
        StockItemReadSize::new(StockItemReadKind::Candidate, 489).unwrap(),
        &shape_identity(),
        stable_admission_pair(items),
        drifted_admission_pair(),
    )
    .expect_err("a drifted count is not admitted");
    assert!(matches!(
        cause::<PairedReadValidationError>(&count_drift),
        Some(PairedReadValidationError::StockSummaryCollection)
    ));
}

#[test]
fn candidate_admission_refuses_an_unavailable_count() {
    let items = admission_capture(SHAPE_ITEMS_BASELINE);
    let count = admission_capture(SHAPE_COUNT);
    for edited in [
        with_stock_item_count(&count, None),
        with_stock_item_count(&count, Some("")),
        with_stock_item_count(&count, Some("eleven")),
    ] {
        let error = admit_shape_candidate(items.clone(), edited, 489)
            .expect_err("an unavailable count is never zero or admitted");
        assert!(matches!(
            cause::<bridge_tally_protocol::native_stock_summary::NativeStockError>(&error),
            Some(
                bridge_tally_protocol::native_stock_summary::NativeStockError::ItemCountUnavailable
            )
        ));
    }
}

#[test]
fn candidate_admission_refuses_count_mismatch_in_either_direction() {
    let items = admission_capture(SHAPE_ITEMS_BASELINE);
    let count = admission_capture(SHAPE_COUNT);
    for (reported, expected) in [(10, 10), (12, 12)] {
        let error = admit_shape_candidate(
            items.clone(),
            with_stock_item_count(&count, Some(&reported.to_string())),
            489,
        )
        .expect_err("a count in either mismatch direction refuses");
        let mismatch = cause::<StockItemCountMismatch>(&error).expect("the typed mismatch");
        assert_eq!(
            mismatch,
            &StockItemCountMismatch {
                rows: 11,
                tally_count: expected,
            }
        );
    }
}

#[test]
fn candidate_admission_binds_the_item_and_count_answers_to_one_company() {
    let r1_items = admission_capture(STOCK_CREATE_ITEMS_BEFORE);
    let shape_count = admission_capture(SHAPE_COUNT);
    let wrong_items = admit_candidate_stock_item_read(
        StockItemReadSize::new(StockItemReadKind::Candidate, 489).unwrap(),
        &shape_identity(),
        stable_admission_pair(r1_items.clone()),
        stable_admission_pair(with_stock_item_count(&shape_count, Some("1"))),
    )
    .expect_err("another company's item rows refuse");
    assert_eq!(
        cause::<NativeMastersError>(&wrong_items),
        Some(&NativeMastersError::RowGuidForeign)
    );

    let wrong_count = admit_candidate_stock_item_read(
        StockItemReadSize::new(StockItemReadKind::Candidate, 489).unwrap(),
        &stock_create_identity(),
        stable_admission_pair(r1_items),
        stable_admission_pair(shape_count),
    )
    .expect_err("a count answer not naming the target refuses");
    assert!(matches!(
        cause::<bridge_tally_protocol::native_stock_summary::NativeStockError>(&wrong_count),
        Some(bridge_tally_protocol::native_stock_summary::NativeStockError::CompanyFlagsNotOneRow)
    ));
}

#[test]
fn candidate_admission_enforces_the_mark_and_encoded_byte_premises() {
    let items = admission_capture(SHAPE_ITEMS_TWO_DATES);
    let count = with_stock_item_count(&admission_capture(SHAPE_COUNT), Some("12"));
    let above_mark = admit_shape_candidate(items.clone(), count.clone(), 293)
        .expect_err("the capture's AlterID 294 is above mark 293");
    assert!(matches!(
        cause::<MastersReadError>(&above_mark),
        Some(MastersReadError::PremiseViolated(
            "masters_alter_id_above_master_mark"
        ))
    ));

    // The count edit and artificial byte limit are logic vectors, not a captured
    // same-sitting count or a Tally limit.
    let pair = stable_admission_pair(items);
    let encoded_bytes = match &pair {
        NativePairedRead::Stable { encoded_bytes, .. } => *encoded_bytes,
        NativePairedRead::Drifted(_) => unreachable!(),
    };
    let over_bytes = admit_candidate_stock_item_read(
        StockItemReadSize::with_limit_for_test(
            StockItemReadKind::Candidate,
            294,
            u64::try_from(encoded_bytes - 1).unwrap(),
        ),
        &shape_identity(),
        pair,
        stable_admission_pair(count),
    )
    .expect_err("one byte over the artificial admitted limit refuses");
    assert!(matches!(
        cause::<MastersReadError>(&over_bytes),
        Some(MastersReadError::PremiseViolated(
            "masters_response_over_admitted_bytes"
        ))
    ));
}

#[test]
fn support_reads_are_paired_sized_company_bound_and_of_their_own_kind() {
    let groups = admission_capture(STOCK_CREATE_GROUPS);
    let units = admission_capture(STOCK_CREATE_UNITS);
    let identity = stock_create_identity();
    let admitted_groups = admit_stock_groups(
        StockItemReadSize::new(StockItemReadKind::StockGroups, 210).unwrap(),
        &identity,
        stable_admission_pair(groups.clone()),
    )
    .unwrap();
    assert_eq!(admitted_groups.rows().len(), 1);
    assert_eq!(admitted_groups.rows()[0].name, "SC Goods Group");
    assert_eq!(admitted_groups.sized_mark(), 210);
    let admitted_units = admit_stock_units(
        StockItemReadSize::new(StockItemReadKind::Units, 210).unwrap(),
        &identity,
        stable_admission_pair(units.clone()),
    )
    .unwrap();
    assert_eq!(admitted_units.rows().len(), 1);
    assert_eq!(admitted_units.rows()[0].name, "Nos");
    assert_eq!(admitted_units.sized_mark(), 210);

    for (kind, edge) in [
        (StockItemReadKind::StockGroups, 1_160),
        (StockItemReadKind::Units, 1_168),
    ] {
        assert!(StockItemReadSize::new(kind, edge).is_ok());
        assert!(matches!(
            StockItemReadSize::new(kind, edge + 1),
            Err(MastersReadError::TooLarge {
                master_alter_id,
                limit_master_alter_id,
                ..
            }) if master_alter_id == edge + 1 && limit_master_alter_id == edge
        ));
    }

    for drifted in [
        admit_stock_groups(
            StockItemReadSize::new(StockItemReadKind::StockGroups, 210).unwrap(),
            &identity,
            drifted_admission_pair(),
        )
        .expect_err("drifted groups refuse"),
        admit_stock_units(
            StockItemReadSize::new(StockItemReadKind::Units, 210).unwrap(),
            &identity,
            drifted_admission_pair(),
        )
        .expect_err("drifted units refuse"),
    ] {
        assert!(matches!(
            cause::<PairedReadValidationError>(&drifted),
            Some(PairedReadValidationError::MastersCollection)
        ));
    }

    let foreign_groups = admit_stock_groups(
        StockItemReadSize::new(StockItemReadKind::StockGroups, 489).unwrap(),
        &shape_identity(),
        stable_admission_pair(groups.clone()),
    )
    .expect_err("foreign groups refuse");
    let foreign_units = admit_stock_units(
        StockItemReadSize::new(StockItemReadKind::Units, 489).unwrap(),
        &shape_identity(),
        stable_admission_pair(units.clone()),
    )
    .expect_err("foreign units refuse");
    for error in [foreign_groups, foreign_units] {
        assert_eq!(
            cause::<NativeMastersError>(&error),
            Some(&NativeMastersError::RowGuidForeign)
        );
    }

    let wrong_group_token = admit_stock_groups(
        StockItemReadSize::new(StockItemReadKind::Units, 210).unwrap(),
        &identity,
        stable_admission_pair(groups.clone()),
    )
    .expect_err("a unit token admits no groups");
    assert!(cause::<StockItemReadKindError>(&wrong_group_token).is_some());
    let wrong_unit_token = admit_stock_units(
        StockItemReadSize::new(StockItemReadKind::StockGroups, 210).unwrap(),
        &identity,
        stable_admission_pair(units.clone()),
    )
    .expect_err("a group token admits no units");
    assert!(cause::<StockItemReadKindError>(&wrong_unit_token).is_some());
    // The candidate read takes only its own token, and neither support read
    // takes the candidate's.
    for kind in [StockItemReadKind::StockGroups, StockItemReadKind::Units] {
        let wrong_candidate_token = admit_candidate_stock_item_read(
            StockItemReadSize::new(kind, 489).unwrap(),
            &shape_identity(),
            stable_admission_pair(admission_capture(SHAPE_ITEMS_BASELINE)),
            stable_admission_pair(admission_capture(SHAPE_COUNT)),
        )
        .expect_err("a support token admits no candidate read");
        assert!(cause::<StockItemReadKindError>(&wrong_candidate_token).is_some());
    }
    let candidate_token_groups = admit_stock_groups(
        StockItemReadSize::new(StockItemReadKind::Candidate, 210).unwrap(),
        &identity,
        stable_admission_pair(groups.clone()),
    )
    .expect_err("a candidate token admits no groups");
    assert!(cause::<StockItemReadKindError>(&candidate_token_groups).is_some());
    let candidate_token_units = admit_stock_units(
        StockItemReadSize::new(StockItemReadKind::Candidate, 210).unwrap(),
        &identity,
        stable_admission_pair(units.clone()),
    )
    .expect_err("a candidate token admits no units");
    assert!(cause::<StockItemReadKindError>(&candidate_token_units).is_some());

    // These AlterID and byte faults are constructed admission vectors, not new
    // captures. Both support reads use the same shared premise reasons.
    let group_mark =
        parse_native_masters(NativeMasterKind::StockGroups, &groups, STOCK_CREATE_GUID)
            .unwrap()
            .rows[0]
            .alter_id
            - 1;
    let unit_mark = parse_native_masters(NativeMasterKind::Units, &units, STOCK_CREATE_GUID)
        .unwrap()
        .rows[0]
        .alter_id
        - 1;
    let group_above = admit_stock_groups(
        StockItemReadSize::new(StockItemReadKind::StockGroups, group_mark).unwrap(),
        &identity,
        stable_admission_pair(groups.clone()),
    )
    .expect_err("a group above its token's mark refuses");
    let unit_above = admit_stock_units(
        StockItemReadSize::new(StockItemReadKind::Units, unit_mark).unwrap(),
        &identity,
        stable_admission_pair(units.clone()),
    )
    .expect_err("a unit above its token's mark refuses");
    for error in [group_above, unit_above] {
        assert!(matches!(
            cause::<MastersReadError>(&error),
            Some(MastersReadError::PremiseViolated(
                "masters_alter_id_above_master_mark"
            ))
        ));
    }

    for (kind, body) in [
        (StockItemReadKind::StockGroups, groups),
        (StockItemReadKind::Units, units),
    ] {
        let pair = stable_admission_pair(body);
        let encoded_bytes = match &pair {
            NativePairedRead::Stable { encoded_bytes, .. } => *encoded_bytes,
            NativePairedRead::Drifted(_) => unreachable!(),
        };
        let size = StockItemReadSize::with_limit_for_test(
            kind,
            210,
            u64::try_from(encoded_bytes - 1).unwrap(),
        );
        let result = match kind {
            StockItemReadKind::StockGroups => admit_stock_groups(size, &identity, pair).map(|_| ()),
            StockItemReadKind::Units => admit_stock_units(size, &identity, pair).map(|_| ()),
            StockItemReadKind::Candidate => unreachable!(),
        };
        let error = result.expect_err("one byte over the support token's limit refuses");
        assert!(matches!(
            cause::<MastersReadError>(&error),
            Some(MastersReadError::PremiseViolated(
                "masters_response_over_admitted_bytes"
            ))
        ));
    }
}

#[test]
fn a_zero_row_candidate_with_zero_count_is_no_items_not_a_complete_catalogue() {
    // Structurally removing the eleven rows and changing the count to zero are
    // constructed vectors; no captured zero-row candidate response is claimed.
    let baseline = admission_capture(SHAPE_ITEMS_BASELINE);
    let mut rest = baseline.as_str();
    let mut without_rows = String::new();
    let mut removed = 0;
    while let Some(start) = rest.find("<STOCKITEM ") {
        let end = rest[start..].find("</STOCKITEM>").unwrap() + "</STOCKITEM>".len();
        without_rows.push_str(&rest[..start]);
        rest = &rest[start + end..];
        removed += 1;
    }
    without_rows.push_str(rest);
    assert_eq!(removed, 11);
    assert!(!without_rows.contains("<STOCKITEM "));
    let admitted = admit_shape_candidate(
        without_rows,
        with_stock_item_count(&admission_capture(SHAPE_COUNT), Some("0")),
        489,
    )
    .expect("zero rows and Tally's reported zero agree at admission");
    let error =
        crate::tally::stock_item_create::CompleteStockItemCatalogue::from_admitted(admitted)
            .expect_err("zero rows never constructs a complete catalogue");
    assert!(matches!(
        error,
        crate::tally::stock_item_create::StockItemCatalogueError::NoItems
    ));

    let start = baseline.find("<COLLECTION").unwrap();
    let end = baseline.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    let absent = format!("{}{}", &baseline[..start], &baseline[end..]);
    let error = admit_shape_candidate(absent, admission_capture(SHAPE_COUNT), 489)
        .expect_err("an absent collection is malformed, not a zero-row answer");
    assert_eq!(
        cause::<NativeMastersError>(&error),
        Some(&NativeMastersError::CollectionAbsent)
    );
}

#[test]
fn an_admitted_candidate_read_carries_its_sized_mark_into_the_complete_catalogue() {
    // The shape lab's item and count files are one book's answers at
    // different capture times (see admit_shape_candidate); the count file
    // reports the eleven items the item file holds.
    let admitted = admit_shape_candidate(
        admission_capture(SHAPE_ITEMS_BASELINE),
        admission_capture(SHAPE_COUNT),
        489,
    )
    .expect("eleven rows and Tally's count of eleven admit");
    assert_eq!(admitted.sized_mark(), 489);
    assert_eq!(admitted.company(), &shape_identity());
    let complete =
        crate::tally::stock_item_create::CompleteStockItemCatalogue::from_admitted(admitted)
            .expect("an admitted non-empty read is a complete catalogue");
    assert_eq!(complete.sized_mark(), 489);
    assert_eq!(complete.catalogue().items().len(), 11);
    assert_eq!(complete.catalogue().company(), &shape_identity());
}
