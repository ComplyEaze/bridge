#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::*;
use crate::commands::VerifiedCompanyIdentity;
use crate::tally::connection::TallyProduct;
use crate::tally::ConnectionStatus;
use anyhow::Context;
use bridge_tally_core::CapabilityProfile;
use bridge_tally_protocol::native_outstandings::{
    compute_native_outstandings, parse_native_bill_rows, parse_native_ledger_snapshot,
    parse_native_ledger_snapshot_for_company,
};
use std::collections::BTreeMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn verified_identity(name: &str, guid: &str) -> VerifiedCompanyIdentity {
    VerifiedCompanyIdentity::test_fixture(name, guid)
}

fn captured_aarav_company_list_and_identity() -> (String, VerifiedCompanyIdentity) {
    let bytes = include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml");
    let xml = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("captured Company collection decodes")
    .text;
    let rows = parse_companies_from_collection(&xml).unwrap();
    let row = rows
        .iter()
        .find(|row| row.name == "Aarav Trading Company Demo")
        .unwrap();
    let identity = VerifiedCompanyIdentity::from_observed_companies(
        row.name.clone(),
        crate::tally::test_company_guid(&row.guid.clone().unwrap()),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &rows,
    )
    .unwrap();
    (xml, identity)
}

#[test]
fn agent_company_list_evidence_hashes_the_encoded_utf16_response_bytes() {
    let response = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY NAME=\"Book\"><GUID>g-1</GUID><COMPANYNUMBER>1</COMPANYNUMBER><BOOKSFROM>20260401</BOOKSFROM></COMPANY></COLLECTION></DATA></BODY></ENVELOPE>".to_string();
    let encoded = bridge_tally_protocol::encode_tally_xml_request_utf16le(&response);
    let expected_bytes = encoded.len();
    let expected_sha = Sha256::digest(&encoded)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let listed = agent_company_list_from_response(response, expected_bytes, expected_sha.clone())
        .expect("company list");
    assert_eq!(listed.response_bytes, expected_bytes);
    assert_eq!(listed.response_sha256, expected_sha);
    assert_eq!(listed.companies.len(), 1);
}

#[test]
fn party_ledger_master_evidence_includes_currency_probe_and_source_responses() {
    let source = |master_response_sha256: &str| PartyLedgerMasterSource {
        company: "Synthetic Books".to_string(),
        company_guid: "company-guid".to_string(),
        currency_assertion: OutstandingsCurrencyAssertion::Inr,
        currency_decimal_places: 2,
        from: TallyDate::parse("20260401").expect("source from"),
        to: TallyDate::parse("20260731").expect("source to"),
        balance_date_basis: BalanceDateBasis::HostToday(
            TallyDate::parse("20260731").expect("source to"),
        ),
        last_voucher_date: TallyDate::parse("20260731").expect("source to"),
        rows: Vec::new(),
        request_sha256: "0".repeat(64),
        master_response_sha256: master_response_sha256.to_string(),
        balance_response_sha256: "b".repeat(64),
        group_response_sha256: "c".repeat(64),
        master_response_bytes: 11,
        balance_response_bytes: 13,
        group_response_bytes: 17,
        groups: Vec::new(),
        foreign_currency_ledgers_excluded: Vec::new(),
        mixed_currency_ledgers_excluded: Vec::new(),
        count_cross_check: None,
        mailing_details_read_failed: None,
    };
    let currency = RuntimeReadEvidence::paired("<currency/>", sha256_hex(b"currency-response"), 19);
    let baseline = TallyRuntime::party_ledger_master_source_evidence(
        &source(&"a".repeat(64)),
        currency.clone(),
    );
    let changed = TallyRuntime::party_ledger_master_source_evidence(
        &source(&"d".repeat(64)),
        currency.clone(),
    );
    let changed_currency = TallyRuntime::party_ledger_master_source_evidence(
        &source(&"a".repeat(64)),
        RuntimeReadEvidence::paired("<currency/>", sha256_hex(b"changed-currency-response"), 23),
    );
    let source_scope = "0".repeat(64);
    let source_responses =
        sha256_hex(format!("{}:{}:{}", "a".repeat(64), "b".repeat(64), "c".repeat(64)).as_bytes());
    assert_eq!(
        baseline.request_sha256,
        sha256_hex(format!("{}:{source_scope}", currency.request_sha256).as_bytes())
    );
    assert_eq!(
        baseline.response_sha256,
        sha256_hex(format!("{}:{source_responses}", currency.response_sha256).as_bytes())
    );
    assert_eq!(baseline.request_sha256, changed_currency.request_sha256);
    assert_ne!(baseline.response_sha256, changed_currency.response_sha256);
    assert_eq!(changed_currency.bytes, (11 + 13 + 17 + 23) * 2);

    assert_eq!(baseline.request_sha256, changed.request_sha256);
    assert_ne!(baseline.response_sha256, changed.response_sha256);
    assert_eq!(baseline.bytes, (11 + 13 + 17 + 19) * 2);
}

#[test]
fn paired_evidence_commits_to_utf16le_request_body_not_utf8_source() {
    let request = "<ENVELOPE>₹ &amp; Co</ENVELOPE>";
    let mut encoded = vec![0xff, 0xfe];
    encoded.extend(request.encode_utf16().flat_map(u16::to_le_bytes));
    let evidence = RuntimeReadEvidence::paired(request, "r".repeat(64), 7);
    assert_eq!(evidence.request_sha256, sha256_hex(&encoded));
    assert_ne!(evidence.request_sha256, sha256_hex(request.as_bytes()));
    assert_eq!(evidence.bytes, 14);
}

#[test]
fn outstandings_wire_evidence_changes_with_any_native_report_response() {
    let receivable = RuntimeReadEvidence::paired("<receivable/>", sha256_hex(b"r1"), 3);
    let payable = RuntimeReadEvidence::paired("<payable/>", sha256_hex(b"p1"), 5);
    let baseline = receivable.clone().combine(payable.clone());
    let changed = receivable.combine(RuntimeReadEvidence::paired(
        "<payable/>",
        sha256_hex(b"p2"),
        5,
    ));

    assert_eq!(baseline.bytes, 16, "both paired responses are counted");
    assert_ne!(baseline.response_sha256, changed.response_sha256);
    assert_ne!(baseline.request_sha256, String::new());
}

fn utf16_xml_response(body: impl AsRef<str>) -> Vec<u8> {
    let body = bridge_tally_protocol::encode_tally_xml_request_utf16le(body.as_ref());
    utf16_xml_response_bytes(&body)
}

fn utf16_xml_response_bytes(body: &[u8]) -> Vec<u8> {
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/xml; charset=utf-16\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    [headers.as_bytes(), body].concat()
}

fn utf8_status_response(body: impl AsRef<str>) -> Vec<u8> {
    let body = body.as_ref().as_bytes();
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/xml; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    [headers.as_bytes(), body].concat()
}

async fn read_http_request(socket: &mut tokio::net::TcpStream) -> Vec<u8> {
    let mut request = Vec::new();
    let mut buffer = [0_u8; 4096];
    let (header_end, content_length) = loop {
        let read = socket.read(&mut buffer).await.expect("read request");
        assert!(read > 0, "request ended before headers completed");
        request.extend_from_slice(&buffer[..read]);
        let Some(header_end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") else {
            continue;
        };
        let header =
            std::str::from_utf8(&request[..header_end]).expect("request headers are valid UTF-8");
        let content_length = header
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then_some(value.trim())
            })
            .map(|value| {
                value
                    .parse::<usize>()
                    .expect("request Content-Length is numeric")
            })
            .unwrap_or(0);
        break (header_end + 4, content_length);
    };
    while request.len() < header_end + content_length {
        let read = socket.read(&mut buffer).await.expect("read request body");
        assert!(read > 0, "request ended before declared body completed");
        request.extend_from_slice(&buffer[..read]);
    }
    request
}

#[test]
fn ageing_anchor_serializes_as_an_explicit_wire_contract() {
    assert_eq!(
        serde_json::to_value(OutstandingsAgeingAnchor::DueDate).unwrap(),
        serde_json::json!("due_date")
    );
    assert_eq!(
        serde_json::to_value(OutstandingsAgeingAnchor::BillDate).unwrap(),
        serde_json::json!("bill_date")
    );
}

#[test]
fn outstandings_read_strategy_serializes_as_an_explicit_wire_contract() {
    assert_eq!(
        serde_json::to_value(OutstandingsReadStrategy::NativeBills).unwrap(),
        serde_json::json!("native_bills")
    );
}

#[test]
fn single_company_forex_ledger_capture_returns_a_typed_partial() {
    const FOREX_LEDGER_CAPTURE: &[u8] = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/ledgers_forex_composite_live.utf16le.xml"
    );
    let ledger_body = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        FOREX_LEDGER_CAPTURE,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        FOREX_LEDGER_CAPTURE.len(),
    )
    .expect("captured forex ledger response decodes")
    .text;
    let admitted = admit_native_ledger_snapshot(
        parse_native_ledger_snapshot(&ledger_body)
            .map(|base| ClassifiedLedgerSnapshot {
                base,
                foreign: Vec::new(),
                mixed: Vec::new(),
                unobserved: 0,
            })
            .map_err(anyhow::Error::from)
            .context("stable native ledger snapshot"),
    )
    .expect("a foreign-currency ledger is an in-band partial");

    assert!(matches!(
        admitted,
        NativeLedgerSnapshotAdmission::Partial(OutstandingsLoadResult::Partial { reason, .. })
            if reason.reason_code == "company_foreign_currency_ledger_balance"
                && reason.foreign_currency_ledger_name.as_deref() == Some("FX USD Debtor 02")
    ));
}

/// The single-company outstandings read of the captured book whose ledger
/// snapshot holds a composite, under the currency witness `witness` builds
/// from the scripted extent.
async fn forex_capture_read(
    witness: impl FnOnce(&str, &VerifiedCompanyIdentity) -> OutstandingsCurrencyWitness,
) -> OutstandingsLoadResult {
    const EXTENT: &str = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    );
    const FOREX_LEDGER_CAPTURE: &[u8] = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/ledgers_forex_composite_live.utf16le.xml"
    );
    const RECEIVABLE: &str = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/bills_receivable_aarav.xml"
    );
    const PAYABLE: &str = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/bills_payable_aarav.xml"
    );
    const STATUS: &str = "<RESPONSE>TallyPrime Server is Running</RESPONSE>";
    let (company_list, identity) = captured_aarav_company_list_and_identity();

    let extent = EXTENT;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind synthetic outstandings server");
    let address = listener.local_addr().expect("synthetic server address");
    let server = tokio::spawn(async move {
        let mut source_post_index = 0;
        let mut company_post_index = 0;
        for index in 0..30 {
            let (mut socket, _) =
                tokio::time::timeout(std::time::Duration::from_secs(2), listener.accept())
                    .await
                    .expect("outstandings request timed out")
                    .expect("accept outstandings request");
            let request = read_http_request(&mut socket).await;
            let response = if request.starts_with(b"GET /status") {
                utf8_status_response(STATUS)
            } else {
                assert!(request.starts_with(b"POST /"), "request {index}");
                let body_start = request
                    .windows(4)
                    .position(|bytes| bytes == b"\r\n\r\n")
                    .expect("complete request headers")
                    + 4;
                let xml = bridge_tally_protocol::decode_tally_text_bytes_limited(
                    &request[body_start..],
                    request.len() - body_start,
                )
                .expect("decode request")
                .text;
                if xml.contains("<ID>BridgeCompanyExtent</ID>")
                    && !xml.contains("<SVCURRENTCOMPANY>")
                {
                    company_post_index += 1;
                    utf16_xml_response(&company_list)
                } else {
                    let response = match source_post_index {
                        0 | 1 | 10 | 11 => utf16_xml_response(extent),
                        2 | 3 => utf16_xml_response(RECEIVABLE),
                        6 | 7 => utf16_xml_response(PAYABLE),
                        _ => utf16_xml_response_bytes(FOREX_LEDGER_CAPTURE),
                    };
                    source_post_index += 1;
                    response
                }
            };
            socket.write_all(&response).await.expect("write response");
        }
        assert_eq!(company_post_index, 4);
        assert_eq!(source_post_index, 12);
    });

    let (result, _) = TallyRuntime::default()
        .fetch_agent_outstandings_with_evidence(
            TallyConfig {
                host: address.ip().to_string(),
                port: address.port(),
            },
            &identity,
            TallyDate::parse("20260401").expect("captured book as-of"),
            witness(extent, &identity),
            OutstandingsAgeingAnchor::DueDate,
        )
        .await
        .expect("foreign-currency capture returns an in-band partial");
    server.await.expect("synthetic outstandings server task");
    result
}

fn assert_forex_capture_refused(result: &OutstandingsLoadResult) {
    assert!(
        matches!(
            result,
            OutstandingsLoadResult::Partial { reason, .. }
                if reason.reason_code == "company_foreign_currency_ledger_balance"
                    && reason.foreign_currency_ledger_name.as_deref() == Some("FX USD Debtor 02")
        ),
        "{result:?}"
    );
}

#[tokio::test]
async fn single_company_read_returns_the_forex_capture_partial() {
    let result =
        forex_capture_read(|extent, identity| inr_witness_for_tests(extent, identity).into()).await;
    assert_forex_capture_refused(&result);
}

/// bridge#642: every production outstandings read admits its currency through
/// `admit_inr_classified`, which also admits a book with one Currency master.
/// Such a book keeps its refusal of a composite value, as before, under that
/// witness: the one-master INR currency capture, identified as production
/// identifies it.
#[tokio::test]
async fn a_one_master_book_under_the_production_witness_keeps_its_composite_refusal() {
    const CURRENCY: &[u8] = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    );
    let currency = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        CURRENCY,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        CURRENCY.len(),
    )
    .expect("captured currency response decodes")
    .text;
    let masters =
        bridge_tally_protocol::native_outstandings::parse_currency_master_list(&currency).unwrap();
    assert_eq!(masters.count(), 1);
    let result = forex_capture_read(|extent, identity| {
        ClassifiedCompanyCurrencyRead {
            currency_count: masters.count(),
            identified: masters.identify_base(None),
            extent: inr_witness_for_tests(extent, identity).currency_read_extent,
            evidence: RuntimeReadEvidence::empty(),
        }
        .admit_inr_classified()
        .expect("one INR master admits")
        .into()
    })
    .await;
    assert_forex_capture_refused(&result);
}

#[test]
fn validation_lab_future_bill_stays_unaged_in_open_bill_output() {
    let receivable = parse_native_bill_rows(
        include_str!(
            "../../crates/bridge-tally-protocol/tests/fixtures/native/bills_receivable_validation_lab.xml"
        ),
        &TallyDate::parse("20250401").expect("captured BooksFrom"),
        &TallyDate::parse("20260817").expect("capture as-of"),
    )
    .expect("captured validation-book rows parse");
    let rows = all_open_bill_rows(
        &receivable,
        &[],
        OutstandingsAgeingAnchor::DueDate,
        &TallyDate::parse("20260817").expect("capture as-of"),
    );
    let future = rows
        .iter()
        .find(|row| row.reference == "ALPHA-FUTURE")
        .expect("captured future-due bill remains present");
    assert_eq!(future.amount.as_str(), "22222.00");
    assert_eq!(future.age_days, None);

    let bill_date_rows = all_open_bill_rows(
        &receivable,
        &[],
        OutstandingsAgeingAnchor::BillDate,
        &TallyDate::parse("20260817").expect("capture as-of"),
    );
    let bill_date_future = bill_date_rows
        .iter()
        .find(|row| row.reference == "ALPHA-FUTURE")
        .expect("captured future-due bill remains present for bill-date ageing");
    assert!(
        bill_date_future.age_days.is_some(),
        "the selected bill-date basis must not reuse the future due date"
    );
}

#[test]
fn settled_native_bill_rows_do_not_reach_statement_or_export_sources() {
    let books_from = TallyDate::parse("20250401").expect("synthetic BooksFrom");
    let as_of = TallyDate::parse("20260817").expect("synthetic as-of");
    let parsed = parse_native_bill_rows(
        "<ENVELOPE><BILLFIXED><BILLDATE>1-Aug-26</BILLDATE><BILLREF>SETTLED</BILLREF>\
             <BILLPARTY>Synthetic Customer</BILLPARTY></BILLFIXED><BILLCL>0.00</BILLCL>\
             <BILLDUE>1-Aug-26</BILLDUE><BILLOVERDUE>16</BILLOVERDUE></ENVELOPE>",
        &books_from,
        &as_of,
    )
    .expect("synthetic settled row parses at the wire boundary");

    assert_eq!(parsed.len(), 1, "the fixture proves the native row existed");
    assert!(
        all_open_bill_rows(&parsed, &[], OutstandingsAgeingAnchor::DueDate, &as_of).is_empty(),
        "a zero closing balance is not an open bill"
    );
}

#[test]
fn captured_validation_lab_bytes_render_a_complete_working_paper_end_to_end() {
    const COMPANY: &str = "Bridge Validation Lab";
    const COMPANY_GUID: &str = "c6afd306-00e1-4f51-802a-babe44daddd3";
    let books_from = TallyDate::parse("20250401").expect("captured BooksFrom");
    let as_of = TallyDate::parse("20260801").expect("captured as-of date");
    let receivable_xml = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/bills_receivable_validation_lab.xml"
    );
    let payable_xml = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/bills_payable_validation_lab.xml"
    );
    let groups_xml = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/group_snapshot_validation_lab.xml"
    );
    let ledgers_xml = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/native/ledger_snapshot_validation_lab.xml"
    );

    let receivable = parse_native_bill_rows(receivable_xml, &books_from, &as_of)
        .expect("captured receivable rows parse");
    let payable = parse_native_bill_rows(payable_xml, &books_from, &as_of)
        .expect("captured payable rows parse");
    // This captured Group body predates the request's response-bound
    // compute. Add only that collection context for the parser contract;
    // source-byte accounting below remains the exact captured response.
    let groups_with_response_company_guid = groups_xml.replace(
        "</GUID>",
        "</GUID><BRIDGECOMPANYGUID>c6afd306-00e1-4f51-802a-babe44daddd3</BRIDGECOMPANYGUID>",
    );
    let groups = parse_native_group_snapshot(&groups_with_response_company_guid, COMPANY_GUID)
        .expect("captured group identity and ancestry parse");
    let ledgers =
        parse_native_ledger_snapshot(ledgers_xml).expect("captured ledger controls parse");
    let source_bytes = receivable_xml
        .len()
        .checked_add(payable_xml.len())
        .and_then(|value| value.checked_add(groups_xml.len()))
        .and_then(|value| value.checked_add(ledgers_xml.len()))
        .expect("captured source byte count fits usize");
    let computed = compute_native_outstandings(
        COMPANY,
        &receivable,
        &payable,
        NativeMasterSnapshot {
            ledgers: &ledgers,
            groups: NativeGroupSnapshot::Complete(&groups),
        },
        NativeAgeingAnchor::DueDate,
        &as_of,
        source_bytes,
    )
    .expect("captured native controls compute");
    assert_eq!(
        native_crosscheck_partial_reason(&computed, &as_of),
        None,
        "captured overdue counters must positively prove the requested date"
    );

    let statement_open_bills = all_open_bill_rows(
        &receivable,
        &payable,
        OutstandingsAgeingAnchor::DueDate,
        &as_of,
    );
    assert_eq!(statement_open_bills.len(), 6);
    assert!(statement_open_bills.iter().all(|row| !row.amount.is_zero()));
    let statement_unallocated_by_party = all_unallocated_parties(&computed.residuals);
    let result = OutstandingsLoadResult::Complete {
        report: Box::new(computed.report),
        read_strategy: OutstandingsReadStrategy::NativeBills,
        currency_assertion: OutstandingsCurrencyAssertion::Inr,
        ageing_anchor: OutstandingsAgeingAnchor::DueDate,
        synced_at_unix_ms: 1_777_000_000_000,
        unallocated_total: Some(computed.residual_total),
        statement_unallocated_by_party,
        statement_open_bills,
    };
    let source = crate::reports::outstandings_working_paper_store::source_from_complete_result(
        &result,
        COMPANY_GUID,
    )
    .expect("captured source stays inside export budgets")
    .expect("captured complete result substantiates a source");
    assert_eq!(source.source_bytes, 33_575);
    assert_eq!(source.open_bills.len(), 6);
    let paper =
        crate::reports::outstandings_working_paper::build_outstandings_working_paper(source)
            .expect("captured exact controls reconcile into a working paper");
    let workbook =
        crate::reports::outstandings_working_paper_xlsx::render_outstandings_working_paper_xlsx(
            &paper,
        )
        .expect("captured working paper renders");
    assert!(workbook.len() > 200);
    assert_eq!(&workbook[0..2], b"PK");
}

#[test]
fn raw_overdue_disagreement_withholds_native_complete() {
    let rows = parse_native_bill_rows(
        "<ENVELOPE><BILLFIXED><BILLDATE>1-Jul-26</BILLDATE><BILLREF>MISMATCH</BILLREF>\
             <BILLPARTY>Synthetic Customer</BILLPARTY></BILLFIXED><BILLCL>-100.00</BILLCL>\
             <BILLDUE>1-Jul-26</BILLDUE><BILLOVERDUE>31</BILLOVERDUE></ENVELOPE>",
        &TallyDate::parse("20250401").expect("synthetic BooksFrom"),
        &TallyDate::parse("20260817").expect("synthetic as-of"),
    )
    .expect("raw bill parses");
    let computed = compute_native_outstandings(
        "Synthetic Company",
        &rows,
        &[],
        NativeMasterSnapshot {
            ledgers: &[],
            groups: NativeGroupSnapshot::LegacyFixtureWithoutGroups,
        },
        NativeAgeingAnchor::DueDate,
        &TallyDate::parse("20260817").expect("synthetic as-of"),
        0,
    )
    .expect("arithmetic still computes for diagnostic comparison");
    assert_eq!(
        computed.overdue_crosscheck,
        NativeOverdueCrosscheck::Inconsistent
    );
    assert_eq!(
        native_crosscheck_partial_reason(&computed, &TallyDate::parse("20260817").unwrap()),
        Some(OutstandingsPartialReason::code(
            "native_overdue_crosscheck_mismatch"
        ))
    );
}

#[test]
fn zero_bill_rows_with_a_ledger_residual_withhold_native_complete() {
    let groups = parse_native_group_snapshot(
        r#"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>
            <GROUP NAME="Sundry Debtors" RESERVEDNAME="Sundry Debtors"><GUID>11111111-1111-1111-1111-111111111111-00000001</GUID><BRIDGECOMPANYGUID>11111111-1111-1111-1111-111111111111</BRIDGECOMPANYGUID><PARENT>Primary</PARENT></GROUP>
            </COLLECTION></DATA></BODY></ENVELOPE>"#,
        "11111111-1111-1111-1111-111111111111",
    )
    .expect("synthetic group snapshot parses");
    let ledgers = parse_native_ledger_snapshot(
        r#"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>
            <LEDGER NAME="Synthetic Customer"><PARENT>Sundry Debtors</PARENT><CLOSINGBALANCE>-100.00</CLOSINGBALANCE><OPENINGBALANCE>0</OPENINGBALANCE><ISBILLWISEON>No</ISBILLWISEON></LEDGER>
            </COLLECTION></DATA></BODY></ENVELOPE>"#,
    )
    .expect("synthetic ledger snapshot parses");
    let requested_as_of = TallyDate::parse("20260817").expect("synthetic as-of");
    let computed = compute_native_outstandings(
        "Synthetic Company",
        &[],
        &[],
        NativeMasterSnapshot {
            ledgers: &ledgers,
            groups: NativeGroupSnapshot::Complete(&groups),
        },
        NativeAgeingAnchor::DueDate,
        &requested_as_of,
        0,
    )
    .expect("the residual is a partial-result diagnostic, not a parser failure");

    assert_eq!(
        computed.overdue_crosscheck,
        NativeOverdueCrosscheck::UnconfirmedAsOfWithoutBillReferences
    );
    assert_eq!(
        native_crosscheck_partial_reason(&computed, &requested_as_of),
        Some(OutstandingsPartialReason::code(
            WarningCode::NativeOutstandingsAsOfUnconfirmedWithoutBillReferences.as_str(),
        ))
    );
}

#[test]
fn raw_empty_bill_references_preserve_each_amount_and_get_explicit_label() {
    let raw_bills = "<ENVELOPE>\
            <BILLFIXED><BILLDATE>1-Jul-26</BILLDATE><BILLREF></BILLREF><BILLPARTY>Synthetic Customer</BILLPARTY></BILLFIXED>\
            <BILLCL>-40.00</BILLCL><BILLDUE>1-Jul-26</BILLDUE><BILLOVERDUE>30</BILLOVERDUE>\
            <BILLFIXED><BILLDATE>2-Jul-26</BILLDATE><BILLREF> \n\t </BILLREF><BILLPARTY>Synthetic Customer</BILLPARTY></BILLFIXED>\
            <BILLCL>-60.00</BILLCL><BILLDUE>2-Jul-26</BILLDUE><BILLOVERDUE>29</BILLOVERDUE>\
            </ENVELOPE>";
    let as_of = TallyDate::parse("20260731").expect("synthetic as-of");
    let parsed = parse_native_bill_rows(
        raw_bills,
        &TallyDate::parse("20250401").expect("synthetic BooksFrom"),
        &as_of,
    )
    .expect("paired empty BILLREF values remain parseable");
    let rows = all_open_bill_rows(&parsed, &[], OutstandingsAgeingAnchor::DueDate, &as_of);

    assert_eq!(rows.len(), 2, "empty identities must not collapse rows");
    let total = rows
        .iter()
        .try_fold(ExactDecimal::zero(), |sum, row| {
            sum.checked_add(&row.amount)
        })
        .expect("synthetic bill total remains exact");
    assert_eq!(total.as_str(), "100", "neither amount may be lost");
    assert_eq!(
        rows.iter()
            .map(|row| row.reference.as_str())
            .collect::<Vec<_>>(),
        vec!["No reference reported", "No reference reported"],
        "client-facing rows must disclose the missing identity",
    );
}

#[test]
fn party_master_currency_assertion_rejects_a_changed_company_extent() {
    const EXTENT: &str = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    );
    let read_extent_xml = EXTENT;
    let changed_extent_xml = read_extent_xml.replacen(
        "<LASTVOUCHERDATE TYPE=\"Date\">20260401</LASTVOUCHERDATE>",
        "<LASTVOUCHERDATE TYPE=\"Date\">20260402</LASTVOUCHERDATE>",
        1,
    );
    assert_ne!(
        changed_extent_xml, read_extent_xml,
        "the mutation must apply"
    );
    let expectation =
        bridge_tally_protocol::outstandings_shared::CompanyBookExtentExpectation::new(
            "Aarav Trading Company Demo".to_string(),
            "bb8ad19e-6aef-4239-a917-87fec0c6215e".to_string(),
            "100000".to_string(),
            "20240401".to_string(),
        )
        .expect("captured full tuple is a valid expectation");
    let read_extent = bridge_tally_protocol::outstandings_shared::parse_company_book_extent_v2(
        read_extent_xml,
        &expectation,
    )
    .expect("captured extent parses");
    let changed_extent = bridge_tally_protocol::outstandings_shared::parse_company_book_extent_v2(
        &changed_extent_xml,
        &expectation,
    )
    .expect("changed synthetic extent parses");
    let assertion = CompanyCurrencyRead {
        currency: CompanyCurrency {
            symbol: "₹".to_string(),
            mailing_name: "INR".to_string(),
            currency_count: 1,
            decimal_places: 2,
            is_inr: true,
            names: Vec::new(),
        },
        extent: read_extent.clone(),
    }
    .bind_party_ledger_master_assertion(OutstandingsCurrencyAssertion::Inr);

    assert_eq!(
        assertion
            .require_opening_extent(&read_extent)
            .expect("the observed extent releases INR"),
        PartyLedgerMasterCurrency {
            assertion: OutstandingsCurrencyAssertion::Inr,
            decimal_places: 2,
        }
    );
    let error = assertion
        .require_opening_extent(&changed_extent)
        .expect_err("a changed extent must not release INR");
    assert!(matches!(
        error.downcast_ref::<PairedReadValidationError>(),
        Some(PairedReadValidationError::CurrencyToMasterExtent)
    ));
}

#[tokio::test]
async fn detect_base_currency_rejects_book_drift_after_the_currency_read() {
    const EXTENT: &str = include_str!(
        "../../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    );
    const CURRENCY: &[u8] = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    );
    const STATUS: &str = "<RESPONSE>TallyPrime Server is Running</RESPONSE>";
    let (company_list, identity) = captured_aarav_company_list_and_identity();

    let currency = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        CURRENCY,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        CURRENCY.len(),
    )
    .expect("captured currency response decodes")
    .text;

    let opening_extent = EXTENT;
    let closing_extent = opening_extent.replace(
        "<LASTVOUCHERDATE TYPE=\"Date\">20260401</LASTVOUCHERDATE>",
        "<LASTVOUCHERDATE TYPE=\"Date\">20260402</LASTVOUCHERDATE>",
    );
    assert_ne!(
        closing_extent, opening_extent,
        "the drift mutation must apply"
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind synthetic currency server");
    let address = listener.local_addr().expect("synthetic server address");
    let server = tokio::spawn(async move {
        let mut source_post_index = 0;
        for index in 0..13 {
            let (mut socket, _) =
                tokio::time::timeout(std::time::Duration::from_secs(2), listener.accept())
                    .await
                    .expect("currency request timed out")
                    .expect("accept currency request");
            let request = read_http_request(&mut socket).await;
            let response = if request.starts_with(b"GET /status") {
                utf8_status_response(STATUS)
            } else {
                assert!(request.starts_with(b"POST /"), "request {index}");
                let body_start = request
                    .windows(4)
                    .position(|bytes| bytes == b"\r\n\r\n")
                    .expect("complete request headers")
                    + 4;
                let xml = bridge_tally_protocol::decode_tally_text_bytes_limited(
                    &request[body_start..],
                    request.len() - body_start,
                )
                .expect("decode request")
                .text;
                if xml.contains("<ID>BridgeCompanyExtent</ID>")
                    && !xml.contains("<SVCURRENTCOMPANY>")
                {
                    utf16_xml_response(&company_list)
                } else {
                    let response = match source_post_index {
                        0 | 1 => utf16_xml_response(opening_extent),
                        2 | 3 => utf16_xml_response(&currency),
                        _ => utf16_xml_response(&closing_extent),
                    };
                    source_post_index += 1;
                    response
                }
            };
            socket.write_all(&response).await.expect("write response");
        }
    });

    let runtime = TallyRuntime::default();
    let result = runtime
        .detect_base_currency(
            TallyConfig {
                host: address.ip().to_string(),
                port: address.port(),
            },
            &identity,
        )
        .await;

    let error = result.expect_err("closing extent drift must reject the currency");
    assert!(
        error.chain().any(|cause| matches!(
            cause.downcast_ref::<PairedReadValidationError>(),
            Some(PairedReadValidationError::CurrencyExtent)
        )),
        "unexpected error: {error:#}"
    );
    server.await.expect("synthetic currency server task");
}

fn synthetic_probe_result() -> TallyProbeResult {
    TallyProbeResult {
        connection: ConnectionStatus {
            reachable: true,
            compatible: false,
            server_text: "Synthetic status".to_string(),
            product: TallyProduct::Unknown,
            error: None,
        },
        companies: vec![TallyCompany {
            name: "Synthetic Company".to_string(),
            guid: Some("synthetic-guid".to_string()),
            company_number: None,
            books_from: None,
        }],
        profile: CapabilityProfile {
            profile_version: 2,
            product: "Unknown".to_string(),
            release: None,
            license_tier: None,
            mode: None,
            transports: BTreeMap::new(),
            features: BTreeMap::new(),
            packs: BTreeMap::new(),
        },
        passport_snapshot_id: None,
    }
}

#[test]
fn future_due_bill_from_raw_native_bytes_reaches_the_statement_source() {
    let books_from = TallyDate::parse("20260401").expect("synthetic book start");
    let as_of = TallyDate::parse("20260731").expect("synthetic as-of date");
    let bills_xml = "<ENVELOPE>\
            <BILLFIXED><BILLDATE>1-Jul-26</BILLDATE><BILLREF>FUTURE-1</BILLREF><BILLPARTY>Synthetic Party</BILLPARTY></BILLFIXED>\
            <BILLCL>-100.00</BILLCL><BILLDUE>1-Aug-26</BILLDUE><BILLOVERDUE>0</BILLOVERDUE>\
            </ENVELOPE>";
    let ledger_xml = "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION></COLLECTION></DATA></BODY></ENVELOPE>";
    let receivable = parse_native_bill_rows(bills_xml, &books_from, &as_of)
        .expect("a future due date in raw native bytes must parse");
    let ledgers = parse_native_ledger_snapshot(ledger_xml)
        .expect("the synthetic raw ledger response must parse");
    let computed = compute_native_outstandings(
        "Synthetic Company",
        &receivable,
        &[],
        NativeMasterSnapshot {
            ledgers: &ledgers,
            groups: NativeGroupSnapshot::LegacyFixtureWithoutGroups,
        },
        NativeAgeingAnchor::DueDate,
        &as_of,
        bills_xml.len() + ledger_xml.len(),
    )
    .expect("a future-due bill must not abort the native computation");
    assert_eq!(computed.report.receivable_total.as_str(), "100");
    assert_eq!(computed.report.top_parties[0].oldest_bill_age_days, None);
    assert_eq!(computed.report.ageing.days_0_30.as_str(), "100");
    assert_eq!(computed.report.ageing.days_31_60, ExactDecimal::zero());
    assert_eq!(computed.report.ageing.days_61_90, ExactDecimal::zero());
    assert_eq!(computed.report.ageing.days_90_plus, ExactDecimal::zero());
    assert_eq!(computed.report.open_receivable_bill_count, 1);

    let statement_rows =
        all_open_bill_rows(&receivable, &[], OutstandingsAgeingAnchor::DueDate, &as_of);
    assert_eq!(
        statement_rows.len(),
        1,
        "a future-due bill must remain available to the statement source"
    );
    assert_eq!(statement_rows[0].amount.as_str(), "100.00");
    assert_eq!(statement_rows[0].age_days, None);
    let statement = crate::reports::party_statement::build_party_statement_with_ageing_anchor(
        "Synthetic Company",
        as_of.as_str(),
        "Synthetic Party",
        &statement_rows,
        &[],
        OutstandingsAgeingAnchor::DueDate,
    )
    .expect("the future-due bill must build a party statement");
    assert_eq!(statement.bills.len(), 1);
    assert_eq!(statement.bills[0].reference, "FUTURE-1");
    assert_eq!(statement.bills[0].age_days, None);
    assert_eq!(statement.bills[0].bucket, None);
    assert_eq!(statement.bill_total.as_str(), "100");

    let destination = tempfile::tempdir().expect("synthetic destination");
    let approvals =
        crate::reports::bulk_party_statement::PartyStatementDestinationApprovals::default();
    let approval_id = approvals
        .issue(destination.path().to_path_buf())
        .expect("approve synthetic destination");
    let approved_destination = approvals
        .consume(&approval_id, destination.path())
        .expect("consume synthetic approval");
    let bulk =
        crate::reports::bulk_party_statement::write_bulk_party_statements_with_ageing_anchor(
            crate::reports::bulk_party_statement::BulkPartyStatementRequest {
                destination: &approved_destination,
                company: "Synthetic Company",
                as_of_yyyymmdd: as_of.as_str(),
                format: "xlsx",
                open_bills: &statement_rows,
                unallocated_by_party: &[],
                ageing_anchor: OutstandingsAgeingAnchor::DueDate,
                render: |party_statement: &crate::reports::party_statement::PartyStatement| {
                    crate::reports::party_statement_xlsx::render_party_statement_xlsx(
                        party_statement,
                    )
                    .map_err(|error| error.to_string())
                },
            },
        )
        .expect("a bulk run must write the future-due party statement");
    assert_eq!(bulk.written.len(), 1);
    let workbook = std::fs::File::open(destination.path().join(&bulk.written[0].file_name))
        .expect("the bulk statement file exists");
    let mut archive = zip::ZipArchive::new(workbook).expect("bulk output is an XLSX archive");
    let mut workbook_text = String::new();
    for entry_name in ["xl/worksheets/sheet1.xml", "xl/sharedStrings.xml"] {
        let mut entry = archive
            .by_name(entry_name)
            .expect("the XLSX statement entry exists");
        std::io::Read::read_to_string(&mut entry, &mut workbook_text)
            .expect("the workbook XML is readable");
    }
    assert!(workbook_text.contains("FUTURE-1"));
    assert!(workbook_text.contains("Not due"));
    assert!(workbook_text.contains("Unaged"));
}

#[test]
fn education_snapshot_period_refusal_is_an_in_band_partial() {
    let result = match admit_native_ledger_snapshot_period(
        DateBoundaryProfile::EducationRestricted,
        TallyDate::parse("20260401").expect("Education-valid book start"),
        TallyDate::parse("20260415").expect("ordinary calendar as-of"),
    ) {
        NativeLedgerSnapshotPeriodAdmission::Period(_) => {
            panic!("an Education-refused as-of must not construct a snapshot period")
        }
        NativeLedgerSnapshotPeriodAdmission::Partial(partial) => partial,
    };
    assert!(matches!(
        result,
        OutstandingsLoadResult::Partial { reason, .. }
            if reason.reason_code == "as_of_has_no_valid_window_boundary"
    ));
}

#[test]
fn outstandings_date_boundaries_follow_detected_mode_and_fallback_to_i12() {
    let mut profile = synthetic_probe_result().profile;
    profile.product = "TallyPrime Edit Log".to_string();
    profile.mode = Some("Education".to_string());
    assert_eq!(
        select_date_boundary_profile(Some(&profile)),
        DateBoundaryProfile::EducationRestricted
    );

    profile.mode = Some("Licensed".to_string());
    assert_eq!(
        select_date_boundary_profile(Some(&profile)),
        DateBoundaryProfile::ModeAgnostic
    );
    assert_eq!(
        select_date_boundary_profile(None),
        DateBoundaryProfile::ModeAgnostic
    );

    profile.product = "Unknown".to_string();
    profile.mode = Some("Education".to_string());
    assert_eq!(
        select_date_boundary_profile(Some(&profile)),
        DateBoundaryProfile::ModeAgnostic,
        "inconsistent or incomplete detection must rely on I12 rather than inventing compatibility evidence"
    );
}

/// A non-loopback endpoint is refused by the outstandings read before any
/// Tally request. The loopback guard is the first guard the read reaches,
/// before its currency read, and this asserts it directly.
#[tokio::test]
async fn the_outstandings_read_refuses_a_non_loopback_endpoint() {
    let runtime = TallyRuntime::default();

    let config = TallyConfig {
        host: "not-a-loopback-endpoint".to_string(),
        port: 9000,
    };
    let identity = verified_identity("Synthetic Company", "9d4e7b13-2a58-4c61-b0f9-6e1c3a8d5f27");
    let as_of = TallyDate::parse("20260731").unwrap();
    let error = runtime
        .fetch_operator_outstandings(config, &identity, as_of, OutstandingsAgeingAnchor::DueDate)
        .await
        .expect_err("a non-loopback endpoint must never be contacted");
    assert!(
        error.to_string().contains("non_loopback_forbidden"),
        "loopback-only admission must fail closed before any Tally read, got: {error}"
    );
}

#[test]
fn local_rejections_and_client_http_statuses_do_not_poison_endpoint_health() {
    assert_eq!(
        classify_failure(&anyhow::Error::new(
            TallyRuntimeReadError::ApplicationResponseRejected,
        )),
        ReadFailureClass::Application
    );
    for error in [
        TallyTransportError::RequestTooLarge { limit: 1024 },
        TallyTransportError::PolicyInvalid { code: "test" },
        TallyTransportError::EndpointInvalid { code: "test" },
        TallyTransportError::ClientInitializationFailed,
        TallyTransportError::HttpStatus { status: 400 },
    ] {
        assert_eq!(
            classify_error(&anyhow::Error::new(error)),
            HealthOutcome::ApplicationRejected
        );
    }
    for error in [
        TallyTransportError::ConnectionFailed,
        TallyTransportError::RequestTimedOut,
        TallyTransportError::HttpStatus { status: 503 },
    ] {
        assert_eq!(
            classify_error(&anyhow::Error::new(error)),
            HealthOutcome::TransportFailure
        );
    }
}

#[test]
fn a_failed_response_body_is_its_own_unretried_class_but_counts_as_a_transport_failure() {
    for error in [
        TallyTransportError::ResponseTruncated,
        TallyTransportError::ResponseReadFailed,
    ] {
        let error = anyhow::Error::new(error);
        let class = classify_failure(&error);
        assert_eq!(class, ReadFailureClass::ResponseBodyFailed);
        assert!(!class.retryable());
        assert_eq!(classify_error(&error), HealthOutcome::TransportFailure);
    }
    // A request that failed before any response is still the retryable class.
    let class = classify_failure(&anyhow::Error::new(TallyTransportError::RequestFailed));
    assert_eq!(class, ReadFailureClass::RequestFailed);
    assert!(class.retryable());
}

// The response head had already arrived, so Tally had begun answering; a client cannot cancel
// that work and the gateway serialises, so a resend makes Tally produce the answer again
// (IMPLEMENTATION_GUIDE rule I7, section 5.2).
#[tokio::test]
async fn a_read_whose_response_body_fails_is_sent_once() {
    for (port, failure) in [
        (9130, TallyTransportError::ResponseTruncated),
        (9131, TallyTransportError::ResponseReadFailed),
    ] {
        let runtime = TallyRuntime::default();
        let attempts = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let observed = Arc::clone(&attempts);
        let sent_failure = failure.clone();
        let error = runtime
            .execute(
                TallyConfig {
                    host: "localhost".to_string(),
                    port,
                },
                ReadRetryPolicy::new(3, std::time::Duration::ZERO, std::time::Duration::ZERO, 0)
                    .unwrap(),
                move |_client| {
                    let observed = Arc::clone(&observed);
                    let sent_failure = sent_failure.clone();
                    async move {
                        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        Err::<(), _>(anyhow::Error::new(sent_failure))
                    }
                },
            )
            .await
            .unwrap_err();
        assert_eq!(
            attempts.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "{failure:?}"
        );
        assert_eq!(
            error.downcast_ref::<TallyTransportError>(),
            Some(&failure),
            "the typed transport error is returned"
        );
    }
}

#[test]
fn endpoint_identity_aliases_only_localhost_to_ipv4_loopback() {
    let runtime = TallyRuntime::default();
    let localhost_identity = EndpointKey::from_config(&TallyConfig {
        host: "localhost".to_string(),
        port: 9000,
    })
    .expect("localhost identity");
    let ipv4_identity = EndpointKey::from_config(&TallyConfig {
        host: "127.0.0.1".to_string(),
        port: 9000,
    })
    .expect("IPv4 loopback identity");
    assert_eq!(localhost_identity, ipv4_identity);
    let first = runtime
        .session(TallyConfig {
            host: "localhost".to_string(),
            port: 9000,
        })
        .expect("localhost session");
    let second = runtime
        .session(TallyConfig {
            host: "127.0.0.1".to_string(),
            port: 9000,
        })
        .expect("IPv4 loopback session");
    let third = runtime
        .session(TallyConfig {
            host: "::1".to_string(),
            port: 9000,
        })
        .expect("IPv6 loopback session");
    let fourth = runtime
        .session(TallyConfig {
            host: "127.0.0.2".to_string(),
            port: 9000,
        })
        .expect("alternate IPv4 loopback session");
    assert!(Arc::ptr_eq(&first, &second));
    assert!(!Arc::ptr_eq(&first, &third));
    assert!(!Arc::ptr_eq(&first, &fourth));
    assert!(!Arc::ptr_eq(&third, &fourth));
    let snapshots = runtime.snapshots().expect("runtime snapshots");
    assert_eq!(snapshots.len(), 3);
    assert_eq!(snapshots[0].canonical_endpoint, "http://127.0.0.1:9000");
    assert_eq!(snapshots[1].canonical_endpoint, "http://127.0.0.2:9000");
    assert_eq!(snapshots[2].canonical_endpoint, "http://[::1]:9000");
}

#[test]
fn reviewed_probe_cache_preserves_observation_time_and_is_single_use() {
    let runtime = TallyRuntime::default();
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9000,
    };
    let session = runtime.session(config.clone()).expect("runtime session");
    let observed_at_unix_ms = chrono::Utc::now().timestamp_millis();
    let review_id = "review-current";
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: review_id.to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: false,
    });

    let mut reservation = runtime
        .reserve_cached_probe_fresh(&config, review_id, 300_000)
        .expect("reserve cache")
        .expect("fresh reviewed probe");
    assert_eq!(reservation.observed_at_unix_ms(), observed_at_unix_ms);
    assert_eq!(reservation.result().companies[0].name, "Synthetic Company");
    assert!(runtime
        .reserve_cached_probe_fresh(&config, review_id, 300_000)
        .expect("second reserve")
        .is_none());
    assert!(reservation.consume().expect("consume reservation"));
    assert!(!reservation
        .consume()
        .expect("consuming an already consumed lease is inert"));
    assert!(runtime
        .reserve_cached_probe_fresh(&config, review_id, 300_000)
        .expect("reserve consumed cache")
        .is_none());
}

#[test]
fn stale_review_id_cannot_consume_or_reserve_a_newer_probe() {
    let runtime = TallyRuntime::default();
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9002,
    };
    let session = runtime.session(config.clone()).expect("runtime session");
    let observed_at_unix_ms = chrono::Utc::now().timestamp_millis();
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: "review-b".to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: false,
    });

    assert!(runtime
        .reserve_cached_probe_fresh(&config, "review-a", 300_000)
        .expect("reject stale review")
        .is_none());
    let mut reservation = runtime
        .reserve_cached_probe_fresh(&config, "review-b", 300_000)
        .expect("reserve current review")
        .expect("current review exists");
    assert!(reservation.release().expect("release current review"));
    assert!(runtime
        .reserve_cached_probe_fresh(&config, "review-b", 300_000)
        .expect("retry current review")
        .is_some());
}

#[test]
fn reviewed_probe_cache_rejects_future_expired_and_invalid_freshness() {
    let runtime = TallyRuntime::default();
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9001,
    };
    let session = runtime.session(config.clone()).expect("runtime session");
    for observed_at_unix_ms in [
        chrono::Utc::now().timestamp_millis() + 1_000,
        chrono::Utc::now().timestamp_millis() - 301_000,
    ] {
        *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
            review_id: "review-expiry".to_string(),
            observed_at_unix_ms,
            freshness_origin_unix_ms: observed_at_unix_ms,
            result: synthetic_probe_result(),
            reserved: false,
        });
        assert!(runtime
            .reserve_cached_probe_fresh(&config, "review-expiry", 300_000)
            .expect("reserve cache")
            .is_none());
    }
    assert!(runtime
        .reserve_cached_probe_fresh(&config, "review-expiry", 0)
        .is_err());
    assert!(runtime
        .reserve_cached_probe_fresh(&config, "review-expiry", 600_001)
        .is_err());
}

#[test]
fn ordinary_read_admission_and_review_reservation_are_mutually_exclusive() {
    let runtime = TallyRuntime::default();
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9004,
    };
    let session = runtime.session(config.clone()).expect("runtime session");
    let observed_at_unix_ms = chrono::Utc::now().timestamp_millis();
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: "review-lease".to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: false,
    });

    let read_lease = runtime
        .begin_ordinary_read(&config)
        .expect("admit ordinary read");
    assert!(runtime
        .reserve_cached_probe_fresh(&config, "review-lease", 300_000)
        .is_err());
    drop(read_lease);

    let reservation = runtime
        .reserve_cached_probe_fresh(&config, "review-lease", 300_000)
        .expect("reserve after read")
        .expect("fresh review");
    let refused = runtime
        .begin_ordinary_read(&config)
        .err()
        .expect("an ordinary read is refused while a review is reserved");
    assert_eq!(
        refused.to_string(),
        "Tally reviewed setup operation is in progress"
    );
    drop(reservation);
}

#[test]
fn dropping_a_review_reservation_restores_the_same_fresh_review() {
    let runtime = TallyRuntime::default();
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9005,
    };
    let session = runtime.session(config.clone()).expect("runtime session");
    let observed_at_unix_ms = chrono::Utc::now().timestamp_millis();
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: "review-drop".to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: false,
    });
    drop(
        runtime
            .reserve_cached_probe_fresh(&config, "review-drop", 300_000)
            .expect("reserve review")
            .expect("fresh review"),
    );
    assert!(runtime
        .reserve_cached_probe_fresh(&config, "review-drop", 300_000)
        .expect("reserve after drop")
        .is_some());
}

#[tokio::test]
async fn aborting_a_task_drops_and_releases_its_review_reservation() {
    let runtime = Arc::new(TallyRuntime::default());
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9006,
    };
    let session = runtime.session(config.clone()).expect("runtime session");
    let observed_at_unix_ms = chrono::Utc::now().timestamp_millis();
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: "review-abort".to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: false,
    });
    let (held_tx, held_rx) = tokio::sync::oneshot::channel();
    let task_runtime = Arc::clone(&runtime);
    let task_config = config.clone();
    let task = tokio::spawn(async move {
        let _reservation = task_runtime
            .reserve_cached_probe_fresh(&task_config, "review-abort", 300_000)
            .expect("reserve review")
            .expect("fresh review");
        held_tx.send(()).expect("announce held reservation");
        std::future::pending::<()>().await;
    });
    held_rx.await.expect("reservation was held");
    task.abort();
    let _ = task.await;
    assert!(runtime
        .reserve_cached_probe_fresh(&config, "review-abort", 300_000)
        .expect("reserve after abort")
        .is_some());
}

#[test]
fn stale_guard_cannot_release_or_consume_a_newer_reserved_review() {
    let runtime = TallyRuntime::default();
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9007,
    };
    let session = runtime.session(config.clone()).expect("runtime session");
    let observed_at_unix_ms = chrono::Utc::now().timestamp_millis();
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: "review-old".to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: false,
    });
    let mut stale = runtime
        .reserve_cached_probe_fresh(&config, "review-old", 300_000)
        .expect("reserve old")
        .expect("old review");
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: "review-new".to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: true,
    });
    assert!(!stale.consume().expect("stale consume is inert"));
    drop(stale);
    let cache = session.cached_probe.read().expect("capability cache");
    let current = cache.as_ref().expect("new review remains");
    assert_eq!(current.review_id, "review-new");
    assert!(current.reserved);
}

#[test]
fn held_review_reservation_prevents_endpoint_session_eviction() {
    let runtime = TallyRuntime::default();
    let reserved_config = TallyConfig {
        host: "127.0.0.1".to_string(),
        port: 9200,
    };
    let reserved_endpoint = EndpointKey::from_config(&reserved_config).unwrap();
    let session = runtime
        .session(reserved_config.clone())
        .expect("reserved session");
    let observed_at_unix_ms = chrono::Utc::now().timestamp_millis();
    *session.cached_probe.write().expect("capability cache") = Some(CachedProbe {
        review_id: "review-capacity".to_string(),
        observed_at_unix_ms,
        freshness_origin_unix_ms: observed_at_unix_ms,
        result: synthetic_probe_result(),
        reserved: false,
    });
    drop(session);
    let _reservation = runtime
        .reserve_cached_probe_fresh(&reserved_config, "review-capacity", 300_000)
        .expect("reserve capacity review")
        .expect("fresh capacity review");
    for host_suffix in 2..=MAX_ENDPOINT_SESSIONS {
        runtime
            .session(TallyConfig {
                host: format!("127.0.0.{host_suffix}"),
                port: 9200,
            })
            .expect("fill endpoint capacity");
    }
    runtime
        .session(TallyConfig {
            host: "127.0.0.254".to_string(),
            port: 9200,
        })
        .expect("evict one unreserved session");
    assert!(runtime
        .sessions
        .lock()
        .expect("session registry")
        .contains_key(&reserved_endpoint));
}

#[tokio::test]
async fn cancellation_registry_cancels_and_releases_requests() {
    let runtime = Arc::new(TallyRuntime::default());
    let config = TallyConfig {
        host: "localhost".to_string(),
        port: 9100,
    };
    let runtime_task = Arc::clone(&runtime);
    let task = tokio::spawn(async move {
        runtime_task
            .execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, |_client| async {
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                Ok::<_, anyhow::Error>(())
            })
            .await
    });
    tokio::task::yield_now().await;
    let snapshot = runtime
        .snapshots()
        .expect("runtime snapshots")
        .pop()
        .expect("active session");
    assert_eq!(snapshot.active_requests, 1);
    let session = runtime
        .sessions
        .lock()
        .expect("sessions lock")
        .values()
        .next()
        .expect("session")
        .session
        .clone();
    let request_id = session
        .active_requests
        .lock()
        .expect("request lock")
        .keys()
        .next()
        .expect("request ID")
        .clone();
    assert!(runtime.cancel_request(&request_id).expect("cancel request"));
    assert!(task.await.expect("request task").is_err());
    assert_eq!(
        runtime.snapshots().expect("runtime snapshots")[0].active_requests,
        0
    );
    assert_eq!(
        runtime.snapshots().expect("runtime snapshots")[0].consecutive_failures,
        0,
        "operator cancellation must not degrade endpoint health"
    );
}

/// What each unallocated row says about its own composition, on captures of a
/// book seeded to hold each case (#945; the hand-worked expectations are in
/// `bridge-tally-protocol/tests/native_outstandings_lab_live.rs`).
#[test]
fn unallocated_rows_carry_the_ledgers_flag_and_opening_and_a_composition_that_is_never_on_account()
{
    fn decoded(bytes: &[u8]) -> String {
        bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
            bytes,
            "text/xml; charset=utf-16",
            bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
            bytes.len(),
        )
        .expect("capture decodes")
        .text
    }
    const GUID: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";
    let books_from = TallyDate::parse("20250401").unwrap();
    let as_of = TallyDate::parse("20260630").unwrap();
    let receivable = parse_native_bill_rows(
        &decoded(include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-lab-bills-receivable.utf16le.xml")),
        &books_from,
        &as_of,
    )
    .unwrap();
    let payable = parse_native_bill_rows(
        &decoded(include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-lab-bills-payable.utf16le.xml")),
        &books_from,
        &as_of,
    )
    .unwrap();
    let groups = parse_native_group_snapshot(
        &decoded(include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-lab-groups.utf16le.xml")),
        GUID,
    )
    .unwrap();
    let ledgers = parse_native_ledger_snapshot_for_company(
        &decoded(include_bytes!("../../crates/bridge-tally-protocol/tests/fixtures/agent/native-outstandings-lab-ledgers.utf16le.xml")),
        GUID,
    )
    .unwrap();
    let computed = compute_native_outstandings(
        "BRIDGE OUTSTANDINGS LAB",
        &receivable,
        &payable,
        NativeMasterSnapshot {
            ledgers: &ledgers,
            groups: NativeGroupSnapshot::Complete(&groups),
        },
        NativeAgeingAnchor::DueDate,
        &as_of,
        0,
    )
    .unwrap();
    let rows = all_unallocated_parties(&computed.residuals);
    // Largest first: P08 20,000, P03 7,500, P02 3,000, P05 1,500, P06 800.
    let summary = rows
        .iter()
        .map(|row| {
            (
                &row.party[3..6],
                row.amount.as_str(),
                row.direction,
                row.opening_balance.as_ref().map(|opening| opening.as_str()),
                row.composition,
            )
        })
        .collect::<Vec<_>>();
    use UnallocatedComposition::{
        BillWiseLedgerComponentsNotSeparated as Mixed, NotBillWiseLedger as Off,
    };
    assert_eq!(
        summary,
        vec![
            (
                "P08",
                "20000",
                ExposureDirection::Receivable,
                Some("-20000.00"),
                Some(Mixed)
            ),
            (
                "P03",
                "7500",
                ExposureDirection::Receivable,
                Some("0.00"),
                Some(Off)
            ),
            (
                "P02",
                "3000",
                ExposureDirection::Payable,
                Some("0.00"),
                Some(Mixed)
            ),
            (
                "P05",
                "1500",
                ExposureDirection::Payable,
                Some("0.00"),
                Some(Mixed)
            ),
            (
                "P06",
                "800",
                ExposureDirection::Receivable,
                Some("0.00"),
                Some(Mixed)
            ),
        ]
    );
    // The serialized row names the composition in words, and nothing in it says
    // on-account.
    let json = serde_json::to_string(&rows).unwrap();
    assert!(json.contains("\"composition\":\"bill_wise_ledger_components_not_separated\""));
    assert!(json.contains("\"composition\":\"not_bill_wise_ledger\""));
    // Everything but the party name (a ledger may be NAMED "On Account Debtor"):
    // the fields Bridge itself writes must not call any amount on-account.
    let mut without_names = serde_json::to_value(&rows).unwrap();
    for row in without_names.as_array_mut().unwrap() {
        row.as_object_mut().unwrap().remove("party");
    }
    let text = without_names.to_string().to_ascii_lowercase();
    assert!(!text.contains("on_account"));
    assert!(!text.contains("on account"));
}

/// Pins Bridge's handling of a read whose connection closes after the whole
/// request was received and before any response: the transport calls it
/// `RequestFailed`, and the read retry policy sends it again, three sends in
/// all. The read retry policy also serves the connector's reads; see #1658.
/// The double is local; this measures Bridge's mapping and retry count, not
/// what a real Tally does when it closes.
#[tokio::test]
async fn a_read_is_sent_three_times_when_its_connection_closes_after_the_request_was_received() {
    const MAX_HOST_ABORT_ATTEMPTS: usize = 3;
    for attempt in 1..=MAX_HOST_ABORT_ATTEMPTS {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind the local double");
        let address = listener.local_addr().expect("double address");
        let received = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counted = std::sync::Arc::clone(&received);
        let double = tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.expect("accept a send");
                let request =
                    crate::tally::connection::tests::read_complete_http_request(&mut socket).await;
                assert!(request.starts_with(b"POST "), "a Tally read is a POST");
                counted.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                // Close with the request fully read and nothing answered.
                drop(socket);
            }
        });
        let runtime = TallyRuntime::default();
        let config = TallyConfig {
            host: address.ip().to_string(),
            port: address.port(),
        };

        let (result, trail) =
            crate::request_trail::with_request_trail(Box::pin(runtime.fetch_companies(config)))
                .await;
        double.abort();
        let received = received.load(std::sync::atomic::Ordering::SeqCst);
        let trail = trail.expect("the sends are on the call's trail");
        // Windows endpoint-security software can abort a new loopback flow
        // before the request reaches the double, as in the transport tests.
        // Only that case, a send the double never received, reruns the whole
        // call on a fresh double; the production retry policy is unchanged.
        if attempt < MAX_HOST_ABORT_ATTEMPTS && trail["sent"].as_u64() != Some(received as u64) {
            continue;
        }

        let error = result.expect_err("a read that is never answered must fail");
        assert!(matches!(
            error
                .chain()
                .find_map(|cause| cause.downcast_ref::<TallyTransportError>()),
            Some(TallyTransportError::RequestFailed)
        ));
        assert_eq!(classify_failure(&error), ReadFailureClass::RequestFailed);
        assert_eq!(received, 3);
        assert_eq!(trail["sent"], 3);
        assert_eq!(trail["failed"], 3);
        let sends = trail["last"].as_array().expect("trail sends");
        assert_eq!(sends.len(), 3);
        for send in sends {
            assert_eq!(send["outcome"], "request_failed");
        }
        return;
    }
    unreachable!("bounded attempts always return")
}
