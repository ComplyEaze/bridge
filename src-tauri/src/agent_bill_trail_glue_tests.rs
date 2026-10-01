//! The `outstandings` party detail's handler glue (#945), on a scripted
//! transport: the verified company, the ledger catalogue and the window read
//! are served from the captured import cycle and the captured three-voucher
//! window; the native figures the detail is tied against are typed values,
//! because the outstandings read that would produce them is not part of this
//! glue. Each voucher of that window carries one On Account allocation on its
//! party's ledger (`Café Naïve Traders` holds -102.02); none names a bill.
use super::*;
use crate::agent::bill_trail::DetailKind;
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
    cap: usize,
    books_from_missing: bool,
}

impl Call {
    fn new(kind: DetailKind) -> Self {
        Self {
            kind,
            as_of: AS_OF,
            open_bills: Vec::new(),
            unallocated: Vec::new(),
            redaction: Redaction::None,
            cap: 500,
            books_from_missing: false,
        }
    }
}

/// Runs `outstandings_detail_within` over `plans` and returns its result and
/// how many requests the simulator served.
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
    let result = server
        .outstandings_detail_within(
            &identity,
            &company,
            call.as_of,
            PARTY,
            call.kind,
            None,
            &call.open_bills,
            &call.unallocated,
            call.cap,
        )
        .await;
    simulator.cancel();
    let served = simulator
        .finish()
        .unwrap()
        .iter()
        .filter(|request| !request.request_body_sha256.is_empty())
        .count();
    (result, served)
}

/// The handler end to end: catalogue, window from the books' start to the
/// as-of, the party's own allocation, the residual it ties to, and the window
/// it says it read.
#[tokio::test]
async fn the_unadjusted_detail_runs_end_to_end_on_a_captured_window() {
    let mut call = Call::new(DetailKind::Unadjusted);
    call.unallocated = vec![residual("102.02")];
    let (result, served) = run(detail_plans(captured_window()), call).await;
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
    // The evidence covers the catalogue and the window, not just one of them.
    assert!(evidence.bytes > 0);
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
    let mut call = Call::new(DetailKind::Unadjusted);
    call.cap = 0;
    let (result, _) = run(detail_plans(captured_window()), call).await;
    let failure = result.unwrap_err();
    assert_eq!(failure.code, "unadjusted_detail_too_large");
    assert!(failure.evidence.is_some_and(|evidence| evidence.bytes > 0));
    // At the limit the same answer is given whole.
    let mut call = Call::new(DetailKind::Unadjusted);
    call.cap = 1;
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
