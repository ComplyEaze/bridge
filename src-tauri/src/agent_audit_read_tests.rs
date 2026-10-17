#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
//! The audit read, replayed from the tax-audit crate's synthetic read: its
//! parts are served by the simulator in the order the reader asks for them,
//! and what the crate admits is compared with what it admits from a directory
//! holding the same bytes.
use super::*;
use crate::agent::voucher_window::{
    AuditWindowFailure, RetainedWindowRead, SmallBooks, WindowPart,
};
use crate::tally::runtime::{AuditPartFailureKind, RuntimeReadEvidence};
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;
use tally_protocol_simulator::{
    Fixture, ObservedRequest, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const FIXTURE_GUID: &str = "6f1c2a3e-8b4d-4c5e-9a7f-0d1e2f3a4b5c";
const FIXTURE_NAME: &str = "Lakeview Hardware Demo";

/// A fixture part as text: UTF-16LE parts are decoded, UTF-8 ones kept.
fn text_of(bytes: &[u8]) -> String {
    let bytes = bytes.strip_prefix(&[0xFF, 0xFE]).unwrap_or(bytes);
    if bytes.get(1) == Some(&0) {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    } else {
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

fn plan(xml: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(xml))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn companies_xml(guid: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC><CMPINFO>\
         <COMPANY>0</COMPANY></CMPINFO></DESC><DATA><COLLECTION>\
         <COMPANY NAME=\"{FIXTURE_NAME}\" RESERVEDNAME=\"\">\
         <BOOKSFROM TYPE=\"Date\">20250401</BOOKSFROM><NAME TYPE=\"String\">{FIXTURE_NAME}</NAME>\
         <GUID TYPE=\"String\">{guid}</GUID><COMPANYNUMBER TYPE=\"Number\"> 100000</COMPANYNUMBER>\
         <PRODUCTNAME TYPE=\"String\">TallyPrime</PRODUCTNAME><EDUMODE TYPE=\"Logical\">No</EDUMODE>\
         <SILVER TYPE=\"Logical\">Yes</SILVER><GOLD TYPE=\"Logical\">No</GOLD>\
         </COMPANY></COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

fn identity() -> VerifiedCompanyIdentity {
    let companies =
        bridge_tally_protocol::parse_companies_from_collection(&companies_xml(FIXTURE_GUID))
            .unwrap();
    let observed = &companies[0];
    VerifiedCompanyIdentity::from_observed_companies(
        observed.name.clone(),
        crate::tally::test_company_guid(observed.guid.as_deref().unwrap()),
        observed.company_number.clone().unwrap(),
        observed.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

/// The company object a real Tally serves for an `Object` export: the GUID and
/// books-from the crate's company part reads.
fn company_object(guid: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS>\
         <PRODMAJORVER>1</PRODMAJORVER><PRODMINORVER>1</PRODMINORVER>\
         <PRODMAJORREL>7</PRODMAJORREL><PRODMINORREL>1</PRODMINORREL><PRODTYPE>5</PRODTYPE>\
         </HEADER><BODY><DATA>\
         <TALLYMESSAGE><COMPANY NAME=\"{FIXTURE_NAME}\" RESERVEDNAME=\"\">\
         <GUID TYPE=\"String\">{guid}</GUID><BOOKSFROM TYPE=\"Date\">20250401</BOOKSFROM>\
         <ISINTEGRATED TYPE=\"Logical\">No</ISINTEGRATED></COMPANY></TALLYMESSAGE>\
         </DATA></BODY></ENVELOPE>"
    )
}

/// The three legs of one audit part: identity bracket, the part, identity bracket.
fn legs(body: String) -> Vec<ScenarioPlan> {
    vec![
        plan(companies_xml(FIXTURE_GUID)),
        plan(body),
        plan(companies_xml(FIXTURE_GUID)),
    ]
}

/// The fixture's first voucher window as the one window of the whole period.
fn period_vouchers() -> String {
    text_of(include_bytes!(
        "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/vouchers_h1.xml"
    ))
}

struct Replay {
    marks_before: String,
    company: String,
    marks_in_window: (String, String),
}

impl Replay {
    fn clean() -> Self {
        let marks = text_of(include_bytes!(
            "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/high_water_before.xml"
        ));
        Self {
            marks_before: marks.clone(),
            company: company_object(FIXTURE_GUID),
            marks_in_window: (marks.clone(), marks),
        }
    }

    fn plans(&self) -> Vec<ScenarioPlan> {
        [
            self.marks_before.clone(),
            self.company.clone(),
            text_of(include_bytes!(
                "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/groups.xml"
            )),
            text_of(include_bytes!(
                "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/ledgers.xml"
            )),
            text_of(include_bytes!(
                "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/tb_fy.xml"
            )),
            text_of(include_bytes!(
                "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/vouchertypes.xml"
            )),
            self.marks_in_window.0.clone(),
            period_vouchers(),
            self.marks_in_window.1.clone(),
        ]
        .into_iter()
        .flat_map(legs)
        .collect()
    }
}

/// The whole period fits one request, so the window plans no census.
fn one_request_window() -> WindowReadLimits {
    WindowReadLimits {
        budget_bytes: 1 << 40,
        default_bytes_per_voucher: 1024,
        max_reads: 128,
        small_books: SmallBooks::Skip,
    }
}

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

fn roles() -> AuditRoles {
    AuditRoles {
        cash_groups: vec!["Cash-in-Hand".to_string()],
        bank_groups: vec!["Bank Accounts".to_string(), "Bank OD A/c".to_string()],
    }
}

async fn read_replay(
    replay: &Replay,
) -> (Result<AuditBook, AuditReadFailure>, Vec<ObservedRequest>) {
    let simulator = SequenceSimulator::spawn(replay.plans()).unwrap();
    let config = TallyConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    };
    let book = read_audit_book(
        &TallyRuntime::default(),
        &config,
        &identity(),
        (&date("20250401"), &date("20260331")),
        &roles(),
        one_request_window(),
    )
    .await;
    (book, simulator.finish().unwrap_or_default())
}

fn failure_of(book: Result<AuditBook, AuditReadFailure>) -> AuditReadFailure {
    match book {
        Ok(_) => panic!("the read was admitted"),
        Err(failure) => failure,
    }
}

/// The cash test's canonical dump over a book, whichever way it was loaded.
fn cash_dump(engagement: &Engagement, book: &Book) -> serde_json::Value {
    bridge_tax_audit::cash_44ab_on(
        engagement,
        book,
        &bridge_tax_audit::rules_for(engagement).unwrap(),
    )
    .unwrap()
}

#[tokio::test]
async fn a_clean_replay_is_admitted_and_sends_only_exports() {
    let (book, observed) = read_replay(&Replay::clean()).await;
    let audit = book.unwrap_or_else(|failure| panic!("refused: {failure:?}"));
    assert_eq!(audit.book.company_guid, FIXTURE_GUID);
    assert_eq!(audit.book.vouchers.len(), 43);
    assert_eq!(audit.engagement.period.from.as_str(), "20250401");
    assert_eq!(audit.engagement.cash_groups, roles().cash_groups);
    // Nine parts, each between two company-list brackets, one at a time.
    assert_eq!(observed.len(), 27);
    for (index, request) in observed.iter().enumerate() {
        let body = text_of(&request.request_body);
        let is_part = index % 3 == 1;
        assert!(
            body.contains("<TALLYREQUEST>Export</TALLYREQUEST>"),
            "request {index} is an export"
        );
        for write in ["<TALLYREQUEST>Import", "<TALLYREQUEST>Create", "ACTION="] {
            assert!(!body.contains(write), "request {index} writes ({is_part})");
        }
    }
}

/// The crate admits the same book from a directory holding the bytes the store
/// holds, and the cash test says the same of both.
#[tokio::test]
async fn the_book_equals_the_one_a_directory_of_the_same_bytes_gives() {
    let replay = Replay::clean();
    let (book, _) = read_replay(&replay).await;
    let audit = book.unwrap();
    let identity = identity();
    // Rebuild the store the reader built, from the same parts, and write it out.
    let simulator = SequenceSimulator::spawn(replay.plans()).unwrap();
    let config = TallyConfig {
        host: simulator.address().ip().to_string(),
        port: simulator.address().port(),
    };
    let period = (&date("20250401"), &date("20260331"));
    let (parts, marks) = collect_parts(
        &TallyRuntime::default(),
        &config,
        &identity,
        period,
        one_request_window(),
    )
    .await
    .unwrap();
    simulator.finish().unwrap();
    let (store, _, _) = memory_read(&identity, period, marks, &parts).unwrap();
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("manifest.json"), &store.manifest).unwrap();
    for (path, bytes) in &store.blobs {
        let target = directory.path().join(path);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::write(target, bytes).unwrap();
    }
    let from_directory = Read::open(directory.path()).unwrap();
    from_directory
        .check(
            &audit.engagement.period,
            false,
            audit.engagement.company_pin.as_ref(),
        )
        .unwrap();
    let directory_book =
        bridge_tax_audit::book::load_book(&from_directory, identity.display_name()).unwrap();
    assert_eq!(audit.book.vouchers.len(), directory_book.vouchers.len());
    assert_eq!(
        cash_dump(&audit.engagement, &audit.book),
        cash_dump(&audit.engagement, &directory_book)
    );
}

/// Compared with a book the crate loads from its own fixture, which this
/// module did not produce: the masters are the same bytes, and the vouchers
/// are the fixture's first window.
#[tokio::test]
async fn the_book_holds_what_the_crates_own_fixture_holds_of_the_same_bytes() {
    let (book, _) = read_replay(&Replay::clean()).await;
    let audit = book.unwrap();
    let fixture = Read::open(std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/crates/bridge-tax-audit/tests/fixtures/synthetic-read"
    )))
    .unwrap();
    let theirs = bridge_tax_audit::book::load_book(&fixture, FIXTURE_NAME).unwrap();
    let keys = |book: &Book| {
        (
            book.groups.keys().cloned().collect::<Vec<_>>(),
            book.ledgers.keys().cloned().collect::<Vec<_>>(),
            book.tb.keys().cloned().collect::<Vec<_>>(),
        )
    };
    assert_eq!(keys(&audit.book), keys(&theirs));
    assert!(!audit.book.ledgers.is_empty());
    let guids = |book: &Book| {
        book.vouchers
            .iter()
            .map(|v| v.guid.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(guids(&audit.book), guids(&theirs)[..43]);
}

#[tokio::test]
async fn a_company_object_of_another_company_is_refused_by_code() {
    let mut replay = Replay::clean();
    replay.company = company_object(&FIXTURE_GUID.replacen("6f1c2a3e", "7f1c2a3e", 1));
    let failure = failure_of(read_replay(&replay).await.0);
    assert_eq!(
        failure,
        AuditReadFailure::new("audit_company_part_guid_mismatch", false)
    );
}

#[tokio::test]
async fn marks_that_moved_before_the_window_refuse_the_read_and_may_be_retried() {
    let mut replay = Replay::clean();
    replay.marks_before = replay.marks_before.replace("> 153<", "> 152<");
    let failure = failure_of(read_replay(&replay).await.0);
    assert_eq!(failure, AuditReadFailure::new("C6-moved", true));
}

#[tokio::test]
async fn marks_that_moved_inside_the_window_refuse_it_whole() {
    let mut replay = Replay::clean();
    replay.marks_in_window.1 = replay.marks_in_window.1.replace("> 153<", "> 154<");
    let failure = failure_of(read_replay(&replay).await.0);
    assert_eq!(
        failure,
        AuditReadFailure::new(WINDOW_CHANGED_DURING_READ, true)
    );
}

#[tokio::test]
async fn a_part_the_gateway_refused_carries_its_typed_kind() {
    let mut replay = Replay::clean();
    // Tally answers the marks read with a failed export status.
    replay.marks_before = "<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER></ENVELOPE>".to_string();
    let failure = failure_of(read_replay(&replay).await.0);
    assert_eq!(
        failure.code,
        AuditPartFailureKind::ResponseRejected.code().to_string()
    );
}

fn retained(kind: WindowReadKind, bytes: &str) -> RetainedWindowRead {
    RetainedWindowRead {
        kind,
        request_sha256: "0".repeat(64),
        part: AuditPart {
            encoded_body: bytes.as_bytes().to_vec(),
            encoded_sha256: sha256_hex(bytes.as_bytes()),
            body: bytes.to_string(),
            elapsed: std::time::Duration::ZERO,
            evidence: RuntimeReadEvidence {
                request_sha256: "0".repeat(64),
                response_sha256: sha256_hex(bytes.as_bytes()),
                bytes: bytes.len(),
            },
            boundary_profile: DateBoundaryProfile::ModeAgnostic,
        },
    }
}

#[test]
fn a_window_divided_by_alterid_cannot_become_a_read() {
    let marks = text_of(include_bytes!(
        "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/high_water_before.xml"
    ));
    let day = |span| {
        WindowReadKind::Part(WindowPart {
            from: date("20250401"),
            to: date("20250401"),
            span,
        })
    };
    let spanned = Some(crate::agent::voucher_window::AlterIdSpan {
        after: 0,
        through: 2,
    });
    let mut parts = Vec::new();
    let failure = collect_window(
        &[
            retained(WindowReadKind::Marks, &marks),
            retained(day(spanned), "<ENVELOPE/>"),
            retained(WindowReadKind::Marks, &marks),
        ],
        FIXTURE_GUID,
        &mut parts,
    )
    .unwrap_err();
    assert_eq!(failure, refused(WINDOW_DIVIDED_BY_ALTERID));
}

#[test]
fn a_window_whose_last_read_is_not_the_marks_is_refused() {
    let marks = text_of(include_bytes!(
        "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/high_water_before.xml"
    ));
    let part = WindowReadKind::Part(WindowPart {
        from: date("20250401"),
        to: date("20260331"),
        span: None,
    });
    let mut parts = Vec::new();
    let failure = collect_window(
        &[
            retained(WindowReadKind::Marks, &marks),
            retained(part, "<ENVELOPE/>"),
        ],
        FIXTURE_GUID,
        &mut parts,
    )
    .unwrap_err();
    assert_eq!(failure, refused(MARKS_MISSING));
}

#[test]
fn a_window_that_admitted_other_rows_than_its_parts_hold_is_refused() {
    let part = |rows| Collected {
        id: "vouchers".to_string(),
        kind: "vouchers",
        name: "vouchers.xml".to_string(),
        window: None,
        voucher_counts: Some((rows, None)),
        request_sha256: "0".repeat(64),
        bytes: Vec::new(),
    };
    assert!(check_row_count(5, &[part(2), part(3)]).is_ok());
    assert_eq!(
        check_row_count(5, &[part(2), part(2)]).unwrap_err(),
        refused(ROW_COUNT_DIFFERS)
    );
}

#[test]
fn a_part_that_ran_past_its_deadline_is_not_retryable_wherever_it_was_read() {
    let failure = AuditReadFailure::from(AuditPartFailure {
        kind: AuditPartFailureKind::Deadline,
        evidence: None,
    });
    assert!(!failure.retryable);
}

#[test]
fn a_window_without_its_closing_marks_is_refused() {
    let mut parts = Vec::new();
    let failure = collect_window(&[], FIXTURE_GUID, &mut parts).unwrap_err();
    assert_eq!(failure, refused(MARKS_MISSING));
}

#[test]
fn the_failure_of_a_window_keeps_its_code_and_whether_it_may_repeat() {
    let changed = AuditReadFailure::from(AuditWindowFailure::WindowChanged);
    assert_eq!(
        (changed.code.as_str(), changed.retryable),
        (WINDOW_CHANGED_DURING_READ, true)
    );
    let deadline = AuditReadFailure::from(AuditWindowFailure::Part(AuditPartFailureKind::Deadline));
    assert_eq!(deadline.code, "audit_part_deadline_exceeded");
    // A part that ran past its deadline would send the same request again.
    assert!(!deadline.retryable);
}

#[test]
fn voucher_counts_are_the_ones_the_crate_declares_for_its_own_fixture() {
    let manifest: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/manifest.json"
    ))
    .unwrap();
    let declared = manifest["parts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|part| part["id"] == "vouchers-2025-04-01")
        .unwrap();
    assert_eq!(
        count_vouchers(
            include_bytes!(
                "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/vouchers_h1.xml"
            ),
            "vouchers"
        )
        .unwrap(),
        (
            declared["rows"].as_u64().unwrap(),
            declared["alter_id_max"].as_u64()
        )
    );
}

#[test]
fn group_names_that_need_escaping_reach_the_engagement_whole() {
    let awkward = AuditRoles {
        cash_groups: vec!["Cash \"in\" Hand".to_string(), "Back\\slash".to_string()],
        bank_groups: vec!["Bank\tAccounts".to_string()],
    };
    let toml = engagement_toml(
        &identity(),
        (&date("20250401"), &date("20260331")),
        &awkward,
    )
    .unwrap();
    let engagement = Engagement::from_toml_for_read(&toml).unwrap();
    assert_eq!(engagement.cash_groups, awkward.cash_groups);
    assert_eq!(engagement.bank_groups, awkward.bank_groups);
    assert_eq!(
        engagement.company_pin.unwrap().guid,
        FIXTURE_GUID.to_string()
    );
}
