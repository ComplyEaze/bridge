//! The `outstandings` party detail's own handler, `outstandings_detail`
//! (#945), on a scripted transport: the verified company, the ledger catalogue
//! and the window read are served from the captured import cycle and the
//! captured three-voucher window. These tests call the detail's handler, not
//! the `outstandings` tool: the native figures the detail is tied against are
//! typed values, because no company in the tree has a captured outstandings
//! read and a captured catalogue and window together. Each voucher of that window carries one On Account allocation on its
//! party's ledger (`Café Naïve Traders` holds -102.02); none names a bill.
use super::*;
use crate::agent::bill_trail::{DetailKind, DetailLimits};
use crate::agent::voucher_window::{VoucherReadShape, WindowReadLimits};
use crate::tally::{ExposureDirection, OpenBillRow, UnallocatedComposition, UnallocatedParty};

const PARTY: &str = "Café Naïve Traders";
const AS_OF: &str = "20260802";

fn captured_window() -> ScenarioPlan {
    plan_of(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ))
}

fn empty_window() -> ScenarioPlan {
    plan_of(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-empty-collection.utf16le.xml"
    ))
}

fn plan_of(bytes: &[u8]) -> ScenarioPlan {
    let words = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    ScenarioPlan::new(Fixture::SyntheticXml(String::from_utf16(&words).unwrap()))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

/// The verified company, then the detail's ledger catalogue read.
fn company_and_catalogue() -> Vec<ScenarioPlan> {
    import_cycle_plans()[..10].to_vec()
}

/// [`company_and_catalogue`], then the window's high-water pre-flight and the
/// paired window read serving `window` (the `vouchers` tool's sequence).
fn detail_plans(window: ScenarioPlan) -> Vec<ScenarioPlan> {
    let cycle = import_cycle_plans();
    let mut plans = company_and_catalogue();
    plans.extend(cycle[10..16].iter().cloned());
    plans.extend([
        cycle[0].clone(),
        window.clone(),
        cycle[1].clone(),
        window,
        cycle[1].clone(),
        cycle[0].clone(),
    ]);
    plans
}

fn native_bill(reference: &str) -> OpenBillRow {
    OpenBillRow {
        party: PARTY.into(),
        reference: reference.into(),
        bill_date: "20260801".into(),
        due_date: "20260801".into(),
        amount: bridge_tally_core::ExactDecimal::parse("250").unwrap(),
        age_days: Some(1),
        kind: ExposureDirection::Receivable,
    }
}

fn residual(amount: &str) -> UnallocatedParty {
    UnallocatedParty {
        party: PARTY.into(),
        amount: bridge_tally_core::ExactDecimal::parse(amount).unwrap(),
        direction: ExposureDirection::Receivable,
        opening_balance: Some(bridge_tally_core::ExactDecimal::parse("0.00").unwrap()),
        composition: Some(UnallocatedComposition::BillWiseLedgerComponentsNotSeparated),
    }
}

/// What one call of the handler needs besides the transport.
struct Call {
    kind: DetailKind,
    as_of: &'static str,
    open_bills: Vec<OpenBillRow>,
    unallocated: Vec<UnallocatedParty>,
    redaction: Redaction,
    /// `None` calls the production entry, `outstandings_detail`, with its own
    /// limits; `Some` calls `outstandings_detail_within` with these.
    limits: Option<DetailLimits>,
    books_from_missing: bool,
    /// The `party` argument as the caller typed it.
    party: &'static str,
    reference: Option<&'static str>,
}

impl Call {
    fn new(kind: DetailKind) -> Self {
        Self {
            kind,
            as_of: AS_OF,
            open_bills: Vec::new(),
            unallocated: Vec::new(),
            redaction: Redaction::None,
            limits: None,
            books_from_missing: false,
            party: PARTY,
            reference: None,
        }
    }
}

/// Runs the detail's handler over `plans` and returns its result and how many
/// requests the simulator served.
async fn run(
    plans: Vec<ScenarioPlan>,
    call: Call,
) -> (Result<(Value, Evidence), ToolFailure>, usize) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: call.redaction,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let (mut company, identity, _) = server.verified_company(CAPTURED_GUID).await.unwrap();
    if call.books_from_missing {
        company.books_from = None;
    }
    let result = match call.limits {
        None => {
            server
                .outstandings_detail(
                    &identity,
                    &company,
                    call.as_of,
                    call.party,
                    call.kind,
                    call.reference,
                    &call.open_bills,
                    &call.unallocated,
                )
                .await
        }
        Some(limits) => {
            server
                .outstandings_detail_within(
                    &identity,
                    &company,
                    call.as_of,
                    call.party,
                    call.kind,
                    call.reference,
                    &call.open_bills,
                    &call.unallocated,
                    limits,
                )
                .await
        }
    };
    simulator.cancel();
    let served = simulator
        .finish()
        .unwrap()
        .iter()
        // `cancel` wakes the simulator with a connection of its own, recorded
        // as a cancelled entry when it was waiting: that is not a request.
        .filter(|request| !request.cancelled)
        .count();
    (result, served)
}

/// The evidence one detail read carries: both bodies of each paired read it
/// made, the catalogue, the high-water mark and the window, in that order.
fn detail_evidence_bytes(plans: &[ScenarioPlan]) -> u64 {
    2 * [5, 11, 17]
        .iter()
        .map(|index| response_bytes(&plans[*index]).len() as u64)
        .sum::<u64>()
}

/// The detail's handler through its production limits: the catalogue, the
/// window from the books' start to the as-of, the party's own allocation, the
/// residual it ties to, and the window it says it read.
#[tokio::test]
async fn the_handler_reads_and_ties_an_unadjusted_detail_on_a_captured_window() {
    let mut call = Call::new(DetailKind::Unadjusted);
    call.unallocated = vec![residual("102.02")];
    let plans = detail_plans(captured_window());
    let expected_bytes = detail_evidence_bytes(&plans);
    let (result, served) = run(plans, call).await;
    let (detail, evidence) = result.unwrap();
    assert_eq!(served, 22);
    assert_eq!(detail["kind"], "unadjusted");
    assert_eq!(detail["party"], PARTY);
    assert_eq!(detail["as_of"], AS_OF);
    assert_eq!(
        detail["window"],
        json!({"from": "20260401", "to": AS_OF, "company_vouchers_read": 3})
    );
    assert_eq!(detail["state"], "tied", "{detail}");
    assert_eq!(detail["residual"], "-102.02");
    assert_eq!(detail["on_account_sum"], "-102.02");
    let rows = detail["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 1, "{detail}");
    assert_eq!(rows[0]["class"], "on_account");
    assert_eq!(rows[0]["date"], "20260801");
    // The evidence is the catalogue's, the mark's and the window's, each read
    // in full: not just one of them.
    assert_eq!(evidence.bytes as u64, expected_bytes);
    assert_eq!(evidence.state, "complete");
}

/// Without a residual row the same answer ties nothing, through the handler.
#[tokio::test]
async fn the_handler_reports_an_absent_residual_row_as_its_own_state() {
    let (result, _) = run(
        detail_plans(captured_window()),
        Call::new(DetailKind::Unadjusted),
    )
    .await;
    let (detail, _) = result.unwrap();
    assert_eq!(detail["state"], "no_residual_row_for_party", "{detail}");
    assert!(detail["residual"].is_null(), "{detail}");
    assert_eq!(detail["rows"].as_array().unwrap().len(), 1);
}

/// A bill trail through the handler with party names masked: the detail's own
/// `party` and every bill's `party` are masked, and the name appears nowhere in
/// the serialized answer.
#[tokio::test]
async fn a_masked_bill_trail_masks_the_party_everywhere() {
    let mut call = Call::new(DetailKind::BillTrail);
    call.open_bills = vec![native_bill("GLUE-1"), native_bill("GLUE-2")];
    call.redaction = Redaction::MaskParties;
    let (result, _) = run(detail_plans(captured_window()), call).await;
    let (detail, _) = result.unwrap();
    assert_eq!(detail["state"], "bills_listed", "{detail}");
    let masked = json!(crate::agent::mask(PARTY));
    assert_eq!(detail["party"], masked);
    let bills = detail["bills"].as_array().unwrap();
    assert_eq!(bills.len(), 2, "{detail}");
    for bill in bills {
        assert_eq!(bill["party"], masked, "{bill}");
        // A listed bill no voucher in the window allocates to cannot tie.
        assert_eq!(bill["state"], "trail_does_not_tie", "{bill}");
    }
    let text = detail.to_string();
    for name in [PARTY, "Café", "Naïve"] {
        assert!(!text.contains(name), "{name} unmasked: {text}");
    }
}

/// An empty bill list from the handler says why it is empty.
#[tokio::test]
async fn the_handler_says_why_a_bill_trail_is_empty() {
    let (result, _) = run(
        detail_plans(captured_window()),
        Call::new(DetailKind::BillTrail),
    )
    .await;
    let (detail, _) = result.unwrap();
    assert_eq!(detail["bills"], json!([]));
    assert_eq!(detail["state"], "no_named_bill_for_party", "{detail}");
}

/// A window read that returned no voucher is reported as such through the
/// handler, for either kind, with nothing tied.
#[tokio::test]
async fn the_handler_reports_an_empty_window_read_and_ties_nothing() {
    for kind in [DetailKind::BillTrail, DetailKind::Unadjusted] {
        let mut call = Call::new(kind);
        call.open_bills = vec![native_bill("GLUE-1")];
        call.unallocated = vec![residual("102.02")];
        let (result, _) = run(detail_plans(empty_window()), call).await;
        let (detail, _) = result.unwrap();
        assert_eq!(detail["state"], "window_returned_no_vouchers", "{detail}");
        assert_eq!(detail["window"]["company_vouchers_read"], 0);
        assert!(detail.get("bills").is_none(), "{detail}");
        assert!(detail.get("residual").is_none(), "{detail}");
    }
}

/// The row limit through the handler: refused with the kind's code, never cut,
/// and the refusal keeps the evidence of the reads before it.
#[tokio::test]
async fn the_handler_refuses_an_answer_over_its_row_limit_with_the_reads_evidence() {
    let limits = |rows| DetailLimits {
        rows,
        window: WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard),
    };
    let mut call = Call::new(DetailKind::Unadjusted);
    call.limits = Some(limits(0));
    let plans = detail_plans(captured_window());
    let expected_bytes = detail_evidence_bytes(&plans);
    let (result, _) = run(plans, call).await;
    let failure = result.unwrap_err();
    assert_eq!(failure.code, "unadjusted_detail_too_large");
    assert_eq!(
        failure.evidence.map(|evidence| evidence.bytes as u64),
        Some(expected_bytes),
        "the refusal keeps every read before it"
    );
    // At the limit the same answer is given whole.
    let mut call = Call::new(DetailKind::Unadjusted);
    call.limits = Some(limits(1));
    let (result, _) = run(detail_plans(captured_window()), call).await;
    assert_eq!(result.unwrap().0["rows"].as_array().unwrap().len(), 1);
}

/// An as-of before the books begin is refused after the catalogue and before
/// any voucher request: by the window read's own range check, which the
/// handler relies on rather than repeating.
#[tokio::test]
async fn an_as_of_before_the_books_is_refused_before_the_window_read() {
    let mut call = Call::new(DetailKind::Unadjusted);
    call.as_of = "20260301";
    let (result, served) = run(detail_plans(captured_window()), call).await;
    assert_eq!(result.unwrap_err().code, "invalid_date_range");
    assert_eq!(served, 10, "only the company and the catalogue were read");
}

/// A company with no books-from date is refused, not read from a guessed
/// start, before any voucher request.
#[tokio::test]
async fn a_company_without_a_books_from_date_is_refused_before_the_window_read() {
    let mut call = Call::new(DetailKind::BillTrail);
    call.books_from_missing = true;
    let (result, served) = run(detail_plans(captured_window()), call).await;
    assert_eq!(result.unwrap_err().code, "trail_books_from_missing");
    assert_eq!(served, 10, "only the company and the catalogue were read");
}

/// Tally refusing the window read fails the detail with the transport's own
/// typed cause, never as an empty or partial detail.
#[tokio::test]
async fn a_refused_window_read_fails_the_detail_with_its_transport_cause() {
    let cycle = import_cycle_plans();
    let mut plans = company_and_catalogue();
    plans.extend(cycle[10..16].iter().cloned());
    plans.extend([cycle[0].clone(), captured_window().with_http_status(500)]);
    let (result, _) = run(plans, Call::new(DetailKind::Unadjusted)).await;
    let failure = result.unwrap_err();
    let expected = bridge_tally_transport::TallyTransportError::HttpStatus { status: 500 };
    assert_eq!(
        failure.unanswered,
        Some(crate::agent::Unanswered(expected.safe_code()))
    );
    assert_eq!(failure.code, crate::agent::GENERIC_RUNTIME_READ_FAILURE);
    assert!(
        failure.window_timings.is_some(),
        "the window read's own failure"
    );
}

/// A whole-books window that needs more data requests than one call may spend
/// is refused before any data request, as the detail kind's own refusal with
/// the window's code as its cause, the planned size beside it, and the window
/// read's timings, so the next call can be chosen from the refusal alone.
#[tokio::test]
async fn a_window_needing_more_requests_than_allowed_is_the_details_own_refusal() {
    // Planner limits under which the captured book needs one request per
    // voucher and may spend two: the mark (10) is over one request, so the
    // window is counted (one census, served the captured three vouchers) and
    // planned at three requests, which the planner refuses before any is sent.
    let window = WindowReadLimits {
        budget_bytes: crate::agent::WINDOW_READ_BUDGET_BYTES,
        default_bytes_per_voucher: crate::agent::WINDOW_READ_BUDGET_BYTES,
        max_reads: 2,
    };
    // (kind, named reference, code, requests the plan needs): a named bill's
    // window starts at its date, so the empty day before it is not read.
    for (kind, reference, code, needed) in [
        (DetailKind::BillTrail, None, "trail_window_too_large", 5),
        (
            DetailKind::BillTrail,
            Some("GLUE-1"),
            "named_bill_window_too_large",
            4,
        ),
        (
            DetailKind::Unadjusted,
            None,
            "unadjusted_window_too_large",
            5,
        ),
    ] {
        let cycle = import_cycle_plans();
        let mut plans = company_and_catalogue();
        plans.extend(cycle[10..16].iter().cloned());
        plans.extend([
            cycle[0].clone(),
            captured_window(),
            cycle[1].clone(),
            captured_window(),
            cycle[1].clone(),
            cycle[0].clone(),
        ]);
        let expected_bytes = detail_evidence_bytes(&plans);
        let mut call = Call::new(kind);
        call.limits = Some(DetailLimits { rows: 500, window });
        call.reference = reference;
        call.open_bills = vec![native_bill("GLUE-1")];
        let (result, served) = run(plans, call).await;
        let failure = result.unwrap_err();
        assert_eq!(failure.code, code);
        assert_eq!(failure.cause, Some("voucher_window_too_many_reads"));
        assert_eq!(
            failure.planned_reads(),
            // The day's three vouchers at one a request, and one request each
            // for the empty days before and after it: the plan tiles the window.
            Some(&crate::agent::PlannedReads {
                needed_at_least: needed,
                allowed: 2,
            }),
            "{code}"
        );
        assert!(failure.window_timings.is_some());
        // The catalogue's evidence is kept before the window read's own: both
        // bodies of the catalogue, the mark and the census.
        assert_eq!(
            failure
                .evidence
                .as_ref()
                .map(|evidence| evidence.bytes as u64),
            Some(expected_bytes),
            "{code}"
        );
        // The company, the catalogue, the marks and the census: no data part.
        assert_eq!(served, 22, "{code}");
        assert!(
            crate::agent::refusal_remediation(code).is_some(),
            "{code} names its next step"
        );
    }
}

/// Any other refusal of the window read is passed through unchanged.
#[tokio::test]
async fn other_window_refusals_keep_their_own_code() {
    let mut call = Call::new(DetailKind::BillTrail);
    call.as_of = "20260301";
    let (result, _) = run(detail_plans(captured_window()), call).await;
    let failure = result.unwrap_err();
    assert_eq!(failure.code, "invalid_date_range");
    assert_eq!(failure.cause, None);
}

/// One foreign-currency composite voucher anywhere in the window fails the
/// whole detail: the window is read refusing composites, so the voucher is not
/// withheld as `vouchers` withholds it, and nothing is tied without it.
#[tokio::test]
async fn a_composite_voucher_anywhere_in_the_window_fails_the_detail() {
    let composite = ScenarioPlan::new(Fixture::SyntheticXml(
        crate::agent::voucher_parse::window_with_composite_vouchers(1),
    ))
    .with_encoding(WireEncoding::Utf16Le)
    .with_framing(ResponseFraming::ContentLength);
    for kind in [DetailKind::BillTrail, DetailKind::Unadjusted] {
        let (result, _) = run(detail_plans(composite.clone()), Call::new(kind)).await;
        let failure = result.unwrap_err();
        // The capture's composite sits on the party's bill allocation too,
        // which is parsed first.
        assert_eq!(failure.code, "bill_allocation_amount_invalid");
    }
}

/// A named reference reaches the window: it starts at the earliest date
/// Tally lists for that bill, not at the books' start, and only that bill is
/// answered.
#[tokio::test]
async fn a_named_reference_starts_the_window_at_its_bill_date() {
    let mut call = Call::new(DetailKind::BillTrail);
    call.open_bills = vec![native_bill("GLUE-1"), {
        let mut other = native_bill("GLUE-2");
        other.bill_date = "20260501".into();
        other
    }];
    call.reference = Some("GLUE-1");
    let (result, _) = run(detail_plans(captured_window()), call).await;
    let (detail, _) = result.unwrap();
    assert_eq!(
        detail["window"],
        json!({"from": "20260801", "to": AS_OF, "company_vouchers_read": 3})
    );
    let bills = detail["bills"].as_array().unwrap();
    assert_eq!(bills.len(), 1, "{detail}");
    assert_eq!(bills[0]["reference"], "GLUE-1");
}

/// The party is resolved against the ledger catalogue as `vouchers ledger=`
/// resolves it: a name that differs only in ASCII case finds the ledger, and
/// the answer names it as the catalogue spells it and says how it matched.
#[tokio::test]
async fn a_party_named_in_another_case_is_answered_under_the_catalogues_name() {
    let mut call = Call::new(DetailKind::Unadjusted);
    call.party = "CAFé NAïVE TRADERS";
    call.unallocated = vec![residual("102.02")];
    let (result, _) = run(detail_plans(captured_window()), call).await;
    let (detail, _) = result.unwrap();
    assert_eq!(detail["party"], PARTY);
    assert_eq!(
        detail["ledger_match"]["matched"], "case_or_spacing",
        "{detail}"
    );
    assert_eq!(detail["state"], "tied", "{detail}");
}

/// The case of an accented letter is not folded (#1076 decision A, reference
/// 9.4f): `CAFÉ` asks the user rather than reading `Café`.
#[tokio::test]
async fn a_party_whose_accented_letters_differ_in_case_is_asked_about() {
    let mut call = Call::new(DetailKind::Unadjusted);
    call.party = "CAFÉ NAÏVE TRADERS";
    call.unallocated = vec![residual("102.02")];
    let (result, _) = run(detail_plans(captured_window()), call).await;
    assert_eq!(result.unwrap_err().code, "ledger_not_found");
}

/// A party the catalogue does not hold is refused after the catalogue and
/// before any voucher request, never answered as a party with no rows.
#[tokio::test]
async fn an_unknown_party_is_refused_before_any_voucher_request() {
    let mut call = Call::new(DetailKind::Unadjusted);
    call.party = "No Such Synthetic Party";
    let (result, served) = run(detail_plans(captured_window()), call).await;
    let refusal = result.unwrap_err();
    assert_eq!(refusal.code, "ledger_not_found");
    // The same candidates a `vouchers` or `ledger_movement` refusal carries.
    let miss = refusal.candidates.and_then(|candidates| candidates.miss);
    assert_eq!(miss.map(|miss| miss.listing.as_str()), Some("none"));
    assert_eq!(served, 10, "only the company and the catalogue were read");
}
