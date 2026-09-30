//! Tally's own count of a company's ledgers (bridge#938): the request and its
//! parser, against a live capture of the request (30 Sep 2026, synthetic lab
//! companies only; three of the 31 rows kept, see the sidecar). Each refusal is
//! exercised by damaging the capture in one place.

use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited,
    outstandings_shared::{
        parse_company_ledger_count, render_company_ledger_count_request, OutstandingsError,
    },
    ExpectedTallyTextEncoding,
};
use sha2::{Digest, Sha256};

const CAPTURE: &[u8] = include_bytes!("fixtures/agent/company-ledger-count.utf16le.xml");
const SIZE_BOOK: &str = "BRIDGE SIZE 2K";
const SIZE_BOOK_GUID: &str = "f8dab51e-a5d9-49d5-9232-54a16c8b95bb";

fn capture() -> String {
    decode_tally_xml_response_bytes_limited(
        CAPTURE,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        CAPTURE.len(),
    )
    .expect("a captured BOM-less UTF-16LE response decodes")
    .text
}

/// `body` with `from` replaced by `to`, which must occur once: a damaged capture
/// that did not change would let its test pass for nothing.
fn damaged(body: &str, from: &str, to: &str) -> String {
    assert_eq!(
        body.matches(from).count(),
        1,
        "capture no longer holds {from:?}"
    );
    body.replace(from, to)
}

fn parse(body: &str) -> Result<Option<u64>, OutstandingsError> {
    parse_company_ledger_count(body, SIZE_BOOK, SIZE_BOOK_GUID)
        .map(|count| count.map(|count| count.get()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The bytes Bridge sends are the bytes that were sent live (the sidecar of the
/// capture carries the same hash as `source_request_sha256`).
#[test]
fn the_request_is_the_request_that_was_sent_live() {
    let sent = bridge_tally_protocol::encode_tally_xml_request_utf16le(
        &render_company_ledger_count_request(SIZE_BOOK),
    );
    assert_eq!(sent.len(), 992);
    assert_eq!(
        sha256_hex(&sent),
        "33c0144f052827ca8359e157f119147b5e7612f561639e27617d065dd90ac721"
    );
}

#[test]
fn the_request_asks_the_company_collection_for_name_guid_and_ledger_count_only() {
    let request = render_company_ledger_count_request(SIZE_BOOK);
    assert!(request.contains("<ID>BridgeCompanyLedgerCountV1</ID>"));
    assert!(request.contains("<TYPE>Company</TYPE>"));
    assert!(request.contains("<FETCH>Name, GUID, NUMLEDGERS</FETCH>"));
    assert!(!request.contains("ALTMSTID") && !request.contains("ALTVCHID"));
    assert_eq!(request.matches("$$").count(), 1, "only the export format");
    let escaped = render_company_ledger_count_request("A & B <LAB>");
    assert!(escaped.contains("<SVCURRENTCOMPANY>A &amp; B &lt;LAB&gt;</SVCURRENTCOMPANY>"));
}

#[test]
fn the_capture_yields_each_companys_count_and_the_size_books_equals_its_catalogue() {
    let body = capture();
    // 4,339 is also the ledger catalogue's and the AlterID census's count of the same book.
    assert_eq!(parse(&body), Ok(Some(4339)));
    let count = |name: &str, guid: &str| {
        parse_company_ledger_count(&body, name, guid).map(|count| count.map(|count| count.get()))
    };
    assert_eq!(
        count("Bridge Ageing Lab", "eebb9a9f-1679-4468-9e8f-814c729674cb"),
        Ok(Some(6))
    );
    assert_eq!(
        count(
            "BRIDGE CORPUS DENSE",
            "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3"
        ),
        Ok(Some(123))
    );
}

#[test]
fn a_row_without_the_field_is_unavailable_not_failed() {
    let body = damaged(
        &capture(),
        "     <NUMLEDGERS TYPE=\"Number\"> 4339</NUMLEDGERS>\r\n",
        "",
    );
    assert_eq!(parse(&body), Ok(None));
}

#[test]
fn a_count_that_is_not_a_plain_integer_is_refused() {
    for bad in [
        "",
        " ",
        "-1",
        "+5",
        "1,234",
        "12.0",
        "abc",
        "1e3",
        "0x10",
        "1234567890123",
    ] {
        let body = damaged(
            &capture(),
            "<NUMLEDGERS TYPE=\"Number\"> 4339</NUMLEDGERS>",
            &format!("<NUMLEDGERS TYPE=\"Number\">{bad}</NUMLEDGERS>"),
        );
        assert_eq!(
            parse(&body),
            Err(OutstandingsError::InvalidResponse(
                "company_ledger_count_invalid"
            )),
            "{bad:?}"
        );
    }
}

#[test]
fn an_answer_for_another_company_is_refused() {
    let other = "00000000-0000-0000-0000-000000000000";
    assert_eq!(
        parse_company_ledger_count(&capture(), SIZE_BOOK, other).map(|c| c.map(|c| c.get())),
        Err(OutstandingsError::CompanyIdentityMismatch)
    );
    assert_eq!(
        parse_company_ledger_count(&capture(), "Another Name", SIZE_BOOK_GUID)
            .map(|c| c.map(|c| c.get())),
        Err(OutstandingsError::CompanyIdentityMismatch)
    );
}

#[test]
fn a_row_whose_name_attribute_and_name_differ_is_refused() {
    let body = damaged(
        &capture(),
        "<COMPANY NAME=\"BRIDGE SIZE 2K\"",
        "<COMPANY NAME=\"Something Else\"",
    );
    assert_eq!(
        parse(&body),
        Err(OutstandingsError::CompanyIdentityMismatch)
    );
}

#[test]
fn two_rows_for_one_company_are_ambiguous() {
    let body = capture();
    let start = body.find("<COMPANY NAME=\"BRIDGE SIZE 2K\"").unwrap();
    let end = start + body[start..].find("</COMPANY>").unwrap() + "</COMPANY>".len();
    let row = body[start..end].to_owned();
    let doubled = format!("{}{}{}", &body[..end], row, &body[end..]);
    assert_eq!(
        parse(&doubled),
        Err(OutstandingsError::InvalidResponse(
            "company_identity_ambiguous"
        ))
    );
}

#[test]
fn a_failed_or_damaged_answer_is_never_read_as_unavailable() {
    let body = capture();
    assert!(parse(&damaged(&body, "<STATUS>1</STATUS>", "<STATUS>0</STATUS>")).is_err());
    assert!(parse(&body[..body.len() / 2]).is_err());
    assert!(parse("").is_err());
    assert!(parse(&damaged(&body, "</ENVELOPE>", "")).is_err());
}
