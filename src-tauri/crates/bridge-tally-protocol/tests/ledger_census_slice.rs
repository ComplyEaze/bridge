//! The ledger census reads a slice's rows as GUIDs and nothing else, against
//! two live captures of one synthetic book (30 Sep 2026): an empty slice and
//! an eight-ledger slice. Every refusal is exercised by damaging the capture
//! in one place, so a refusal that does not fire fails a test that names it.

use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited, parse_ledger_census_slice, ExpectedTallyTextEncoding,
    StandardLedgerCatalogError,
};

const COMPANY_GUID: &str = "f8dab51e-a5d9-49d5-9232-54a16c8b95bb";
const EMPTY_SLICE: &[u8] = include_bytes!("fixtures/agent/ledger-census-slice-empty.utf16le.xml");
const EIGHT_ROWS: &[u8] =
    include_bytes!("fixtures/agent/ledger-census-slice-eight-rows.utf16le.xml");

fn decode(captured: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        captured,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        captured.len(),
    )
    .expect("a captured BOM-less UTF-16LE response decodes")
    .text
}

fn eight_rows() -> String {
    decode(EIGHT_ROWS)
}

/// `body` with `from` replaced by `to`, which must occur `times` times: a
/// damaged capture that did not change would let its test pass for nothing.
fn damaged(body: &str, from: &str, to: &str, times: usize) -> String {
    assert_eq!(
        body.matches(from).count(),
        times,
        "capture no longer holds {from:?}"
    );
    body.replace(from, to)
}

/// The byte range of the first `<GUID ...>...</GUID>` field in `body`.
fn first_guid_field(body: &str) -> std::ops::Range<usize> {
    let start = body.find("<GUID TYPE").unwrap();
    let end = start + body[start..].find("</GUID>").unwrap() + "</GUID>".len();
    start..end
}

#[test]
fn an_empty_slice_is_a_well_formed_answer_with_no_rows() {
    let guids = parse_ledger_census_slice(&decode(EMPTY_SLICE), COMPANY_GUID, 8)
        .expect("Tally answers an empty slice with a status 1 envelope and an empty collection");
    assert!(guids.is_empty());
}

#[test]
fn a_slice_of_eight_ledgers_yields_eight_distinct_guids() {
    let guids = parse_ledger_census_slice(&eight_rows(), COMPANY_GUID, 8).expect("captured slice");
    assert_eq!(guids.len(), 8);
    assert_eq!(
        guids.iter().collect::<std::collections::HashSet<_>>().len(),
        8
    );
    // A ledger's GUID is the company's, a dash and the master's eight-digit id.
    for guid in &guids {
        assert!(guid.starts_with(&format!("{COMPANY_GUID}-")), "{guid}");
        assert_eq!(guid.len(), COMPANY_GUID.len() + 9);
    }
    assert_eq!(guids[0], format!("{COMPANY_GUID}-000001f3"));
}

#[test]
fn a_slice_holding_more_rows_than_the_bound_given_is_refused() {
    assert_eq!(
        parse_ledger_census_slice(&eight_rows(), COMPANY_GUID, 8).map(|rows| rows.len()),
        Ok(8)
    );
    assert_eq!(
        parse_ledger_census_slice(&eight_rows(), COMPANY_GUID, 7),
        Err(StandardLedgerCatalogError::BoundsViolation)
    );
    // An empty response cannot exceed any width.
    assert_eq!(
        parse_ledger_census_slice(&decode(EMPTY_SLICE), COMPANY_GUID, 0),
        Ok(Vec::new())
    );
}

#[test]
fn a_row_of_another_company_is_refused() {
    let other = "00000000-0000-0000-0000-000000000000";
    assert_eq!(
        parse_ledger_census_slice(&eight_rows(), other, 8),
        Err(StandardLedgerCatalogError::CompanyIdentityMismatch)
    );
    // The company GUID compares without case.
    assert!(
        parse_ledger_census_slice(&eight_rows(), &COMPANY_GUID.to_ascii_uppercase(), 8).is_ok()
    );
}

#[test]
fn one_row_of_another_company_among_eight_refuses_the_slice() {
    let body = eight_rows();
    let marker = format!(">{COMPANY_GUID}</BRIDGECOMPANYGUID>");
    let last = body.rfind(&marker).unwrap();
    let mut mixed = body.clone();
    mixed.replace_range(
        last..last + marker.len(),
        ">00000000-0000-0000-0000-000000000000</BRIDGECOMPANYGUID>",
    );
    assert_ne!(mixed, body);
    assert_eq!(
        parse_ledger_census_slice(&mixed, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::CompanyIdentityMismatch)
    );
}

#[test]
fn a_repeated_ledger_guid_is_refused() {
    let second = format!("{COMPANY_GUID}-000001f4");
    let first = format!("{COMPANY_GUID}-000001f3");
    let repeated = damaged(
        &eight_rows(),
        &format!(">{second}<"),
        &format!(">{first}<"),
        1,
    );
    assert_eq!(
        parse_ledger_census_slice(&repeated, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::DuplicateIdentity)
    );
    // Differing only in case is the same ledger.
    let upper = damaged(
        &eight_rows(),
        &format!(">{second}<"),
        &format!(">{}<", first.to_ascii_uppercase()),
        1,
    );
    assert_eq!(
        parse_ledger_census_slice(&upper, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::DuplicateIdentity)
    );
}

#[test]
fn a_row_carrying_a_field_the_request_did_not_fetch_is_refused() {
    // A response to some other request must not be counted as a census slice.
    let parent = damaged(
        &eight_rows(),
        "<LANGUAGENAME.LIST>",
        "<PARENT TYPE=\"String\">Sundry Debtors</PARENT><LANGUAGENAME.LIST>",
        8,
    );
    assert_eq!(
        parse_ledger_census_slice(&parent, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
}

#[test]
fn a_row_without_its_guids_is_refused() {
    let no_company = damaged(
        &eight_rows(),
        &format!("<BRIDGECOMPANYGUID TYPE=\"String\">{COMPANY_GUID}</BRIDGECOMPANYGUID>"),
        "",
        8,
    );
    assert_eq!(
        parse_ledger_census_slice(&no_company, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
    let mut no_ledger_guid = eight_rows();
    let field = first_guid_field(&no_ledger_guid);
    no_ledger_guid.replace_range(field, "");
    assert_eq!(
        parse_ledger_census_slice(&no_ledger_guid, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
}

#[test]
fn a_row_repeating_a_guid_field_is_refused() {
    let mut doubled = eight_rows();
    let field = first_guid_field(&doubled);
    let copy = doubled[field.clone()].to_owned();
    doubled.insert_str(field.end, &copy);
    assert_eq!(
        parse_ledger_census_slice(&doubled, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
}

#[test]
fn a_failed_or_damaged_response_is_never_an_empty_census() {
    // A failure status must not read as "no ledgers here".
    let failed = damaged(
        &decode(EMPTY_SLICE),
        "<STATUS>1</STATUS>",
        "<STATUS>0</STATUS>",
        1,
    );
    assert_eq!(
        parse_ledger_census_slice(&failed, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
    // Neither may a response cut short.
    let body = eight_rows();
    assert_eq!(
        parse_ledger_census_slice(&body[..body.len() / 2], COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
    assert_eq!(
        parse_ledger_census_slice("", COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
    // Nor a body that never closes its root.
    let unclosed = damaged(&decode(EMPTY_SLICE), "</ENVELOPE>", "", 1);
    assert_eq!(
        parse_ledger_census_slice(&unclosed, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
}

#[test]
fn a_selfclosed_row_is_refused_not_skipped() {
    let mut emptied = eight_rows();
    let start = emptied.find("<LEDGER NAME").unwrap();
    let end = start + emptied[start..].find("</LEDGER>").unwrap() + "</LEDGER>".len();
    emptied.replace_range(start..end, "<LEDGER NAME=\"x\" RESERVEDNAME=\"\"/>");
    assert_eq!(
        parse_ledger_census_slice(&emptied, COMPANY_GUID, 8),
        Err(StandardLedgerCatalogError::MalformedResponse)
    );
}

#[test]
fn a_malformed_expected_company_guid_is_a_bounds_refusal() {
    assert_eq!(
        parse_ledger_census_slice(&eight_rows(), "", 8),
        Err(StandardLedgerCatalogError::BoundsViolation)
    );
}
