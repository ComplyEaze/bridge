//! Company features reads replayed from the capture, with one fault at a time.
//! The sequence is the stock summary read's with one paired read between its
//! extents: mode probe and identity (3), opening extent (4), Company collection
//! (4), closing extent (4), identity and mode again (3).
use super::company_features::CompanyFeaturesRead;
use super::trial_balance_tests::{
    companies, config, decode, education, extents, pair, status, xml, GUID,
};
use super::*;
use bridge_tally_protocol::native_company_features::{
    render_company_features_request, NativeCompanyFeaturesError, NativeCurrencySymbol,
    NativeSetting,
};
use tally_protocol_simulator::{ObservedRequest, ScenarioPlan, SequenceSimulator};

/// The synthetic book the capture came from. Its identity is replaced by the
/// test company's, so the captured row binds to it.
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";

/// The test company as its identity bracket verifies it.
fn identity() -> VerifiedCompanyIdentity {
    let companies = parse_companies_from_collection(&companies()).unwrap();
    let row = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(GUID))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        row.name.clone(),
        GUID.into(),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

/// The captured Company collection, its identity rewritten to the test company's.
fn features() -> String {
    let identity = identity();
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/company_features_shape_lab_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
    .replace("BRIDGE SHAPE LAB", identity.display_name())
    .replace(
        "> 100021<",
        &format!("> {}<", identity.company_number()),
    )
    .replace(
        "<BOOKSFROM TYPE=\"Date\">20250401<",
        &format!(
            "<BOOKSFROM TYPE=\"Date\">{}<",
            identity.books_from_yyyymmdd()
        ),
    )
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
fn opening() -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    pair(&mut plans, xml(extents()));
    plans
}

/// A whole read: also the identity and mode brackets after the closing extent.
fn complete(features: String, closing_extent: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = opening();
    pair(&mut plans, xml(features));
    pair(&mut plans, xml(closing_extent));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

const WHOLE_READ: usize = 18;
const AT_OPENING_EXTENT: usize = 7;
const AT_COLLECTION: usize = 11;
const AT_CLOSING_EXTENT: usize = 15;

type Outcome = (anyhow::Result<CompanyFeaturesRead>, Vec<ObservedRequest>);

async fn run(plans: Vec<ScenarioPlan>) -> Outcome {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let result = TallyRuntime::default()
        .fetch_company_features(config(&simulator), &identity())
        .await;
    (result, simulator.finish().unwrap())
}

fn cause<T: std::error::Error + 'static>(error: &anyhow::Error) -> Option<&T> {
    error.chain().find_map(|cause| cause.downcast_ref::<T>())
}

fn body_hash(request: &str) -> String {
    sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        request,
    ))
}

#[tokio::test]
async fn a_whole_read_sends_the_builders_request_twice_between_the_extents() {
    let (result, observed) = run(complete(features(), extents())).await;

    let read = result.unwrap();
    assert_eq!(observed.len(), WHOLE_READ);
    assert_eq!(read.features.cost_centres, NativeSetting::No);
    assert_eq!(read.features.gst, NativeSetting::Yes);
    assert_eq!(read.features.batch_wise, NativeSetting::Yes);
    assert_eq!(
        read.features.base_currency,
        NativeCurrencySymbol::Reported("\u{20b9}".to_string())
    );
    assert!(!read.evidence.request_sha256.is_empty());
    let request =
        render_company_features_request(identity().display_name(), identity().company_guid())
            .unwrap();
    let hash = body_hash(&request);
    assert_eq!(observed[AT_OPENING_EXTENT].request_body_sha256, hash);
    assert_eq!(observed[AT_OPENING_EXTENT + 2].request_body_sha256, hash);
    assert_eq!(observed[AT_OPENING_EXTENT + 1].method, "GET");
}

#[tokio::test]
async fn education_is_refused_before_identity_or_extent_dispatch() {
    let (result, observed) = run(vec![status(), xml(education(&companies()))]).await;

    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<CompanyFeaturesReadError>(&error),
        Some(CompanyFeaturesReadError::EducationUnqualified)
    ));
    assert_eq!(observed.len(), 2);
}

#[tokio::test]
async fn an_answer_that_is_not_this_companys_is_refused_at_once_with_its_typed_cause() {
    let other = features().replace(identity().display_name(), "Another Company");
    let mut plans = opening();
    pair(&mut plans, xml(other));
    let (result, observed) = run(plans).await;

    let error = result.err().expect("refused");
    assert_eq!(
        cause::<NativeCompanyFeaturesError>(&error),
        Some(&NativeCompanyFeaturesError::CompanyMismatch("name"))
    );
    assert_eq!(observed.len(), AT_COLLECTION);
}

#[tokio::test]
async fn a_collection_that_changed_between_its_paired_reads_is_refused_at_that_read() {
    let text = features();
    let mut plans = opening();
    plans.extend([
        xml(text.clone()),
        status(),
        xml(format!("{text}\n")),
        status(),
    ]);
    let (result, observed) = run(plans).await;

    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::CompanyFeatures
    ));
    assert_eq!(changed.safe_code(), "company_features_changed");
    assert_eq!(observed.len(), AT_COLLECTION);
}

#[tokio::test]
async fn a_book_that_moved_during_the_read_is_refused_and_keeps_the_completed_source() {
    let mut plans = opening();
    pair(&mut plans, xml(features()));
    pair(&mut plans, xml(extent_with_mark(225)));
    let (result, observed) = run(plans).await;

    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::CompanyFeaturesExtent
    ));
    assert_eq!(changed.safe_code(), "company_features_extent_changed");
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    assert!(!evidence.request_sha256.is_empty());
    assert_eq!(observed.len(), AT_CLOSING_EXTENT);
}
