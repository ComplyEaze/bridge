//! bridge#1198: a record of what this crate's parsers return today for every captured response
//! fixture (and, marked, the synthetic ones), so that a change of XML library must reproduce it
//! exactly. How the records are made and read: `tests/parse_record/README.md`.
//!
//! One row per (fixture, parser). Each row decodes its fixture with the crate's own decoder,
//! parses it with fixed arguments, renders the whole result, and compares the rendering with its
//! committed record. A rendering up to [`FULL_RECORD_CAP`] bytes is committed in full; a longer one
//! as its SHA-256 and length. A mismatch names the fixture, the parser and the first differing line,
//! and no more (AGENTS.md P5). `every_response_fixture_is_recorded_or_named` keeps the table total:
//! a response fixture added later fails until it gets a row or a stated reason.
//! `every_fixture_left_to_the_app_crate_is_in_its_record` holds the two records together: a
//! fixture this table leaves to the app crate's record must be in it, and only those may be.
#![allow(
    clippy::disallowed_methods,
    reason = "the test reads and, in its recording mode, writes its own record files"
)]

use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::native_cash_flow::{parse_native_cash_flow, WholeMonthWindow};
use bridge_tally_protocol::native_company_features::{parse_company_features, ExpectedCompany};
use bridge_tally_protocol::native_masters::{parse_native_masters, NativeMasterKind};
use bridge_tally_protocol::native_outstandings::{
    parse_company_currency, parse_company_currency_name,
    parse_compliance_ledger_snapshot_for_company, parse_currency_master_list,
    parse_native_bill_rows, parse_native_group_snapshot, parse_native_group_snapshot_with_evidence,
    parse_native_ledger_snapshot, parse_native_ledger_snapshot_classified,
    parse_native_ledger_snapshot_classified_for_company, parse_native_ledger_snapshot_for_company,
    BaseCurrencyName,
};
use bridge_tally_protocol::native_statement_reports::{
    parse_native_statement, NativeStatementKind,
};
use bridge_tally_protocol::native_stock_summary::{
    parse_company_inventory_flags, parse_native_stock_items, parse_native_stock_summary_report,
};
use bridge_tally_protocol::native_trial_balance::{
    parse_native_trial_balance, parse_native_trial_balance_with_currency,
};
use bridge_tally_protocol::native_voucher_type_numbering::parse_voucher_type_numbering;
use bridge_tally_protocol::outstandings_shared::{
    parse_company_book_extent, parse_company_book_extent_v2, parse_company_ledger_count,
    CompanyBookExtent, CompanyBookExtentExpectation, DateBoundaryProfile, OutstandingsError,
};
use bridge_tally_protocol::{
    company_list_may_be_in_educational_mode, decode_tally_xml_response_bytes_limited,
    export_status, parse_companies_from_collection, parse_company_gateway_capability_observation,
    parse_import_evidence, parse_import_outcome, parse_import_result, parse_ledger_census_slice,
    parse_ledger_source_records_with_evidence, parse_ledgers, parse_ledgers_with_evidence,
    parse_native_group_source_records_with_evidence,
    parse_native_ledger_source_records_with_evidence,
    parse_native_party_ledger_master_records_with_evidence,
    parse_native_voucher_source_records_with_evidence,
    parse_native_voucher_type_source_records_with_evidence,
    parse_standard_ledger_catalog_v2_with_identities,
    parse_standard_ledger_catalog_with_identities, parse_standard_ledger_identity_observation,
    parse_voucher_source_records_with_evidence, parse_vouchers_with_evidence,
    ExpectedTallyTextEncoding, TallyImportApplicationStatus, TallyImportResult,
};
use sha2::{Digest, Sha256};

/// A rendering longer than this is recorded as its SHA-256 and length, not in full.
/// The maintainers' choice on bridge#1404, so that the heaviest captures (party masters, ledger
/// sources, the dense trial balance) are read in full where a parser change most needs it.
/// Measured at this cap: 256 renderings, 2,033,320 bytes in all; 1 is hashed (529,929 bytes) and
/// 1,503,391 bytes are committed in full.
const FULL_RECORD_CAP: usize = 128 * 1024;

/// Writes every record from today's parsers. Refused where `CI` is set.
const RECORD_VAR: &str = "BRIDGE_RECORD_PARSES";
/// Writes every full rendering into this directory (outside the repository), so two revisions
/// can be compared by hand where a record is only a hash.
const DUMP_VAR: &str = "BRIDGE_PARSE_RECORD_DUMP";
/// Mismatches listed before the rest are only counted.
const MISMATCHES_SHOWN: usize = 20;

/// Whether a fixture is evidence of what Tally sends.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Source {
    Captured,
    /// Authored or derived; recorded because an unusual shape is exactly what a parser change
    /// must be heard on, never as evidence of Tally's behaviour.
    Synthetic,
}

struct Row {
    /// Path from this crate's directory.
    fixture: &'static str,
    source: Source,
    /// The parser and, where it takes them, a short name for its fixed arguments.
    parser: &'static str,
    parse: fn(&[u8], &str) -> String,
}

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn records_dir() -> PathBuf {
    crate_dir().join("tests").join("parse_record")
}

/// The fixture's text, decoded as the crate decodes a response: UTF-16LE when its second byte is
/// zero (with or without a BOM), UTF-8 otherwise.
fn decoded(bytes: &[u8]) -> String {
    let utf16 = bytes.len() >= 2 && (bytes[1] == 0 || bytes.starts_with(&[0xff, 0xfe]));
    let (content_type, expected) = if utf16 {
        (
            "text/xml; charset=utf-16",
            ExpectedTallyTextEncoding::Utf16Le,
        )
    } else {
        ("text/xml; charset=utf-8", ExpectedTallyTextEncoding::Utf8)
    };
    decode_tally_xml_response_bytes_limited(bytes, content_type, expected, bytes.len())
        .expect("every recorded fixture decodes")
        .text
}

/// A typed result, whole: the value's pretty `Debug`, or the error's typed variant.
fn typed<T: Debug, E: Debug>(result: Result<T, E>) -> String {
    match result {
        Ok(value) => format!("Ok({value:#?})"),
        Err(error) => format!("Err({error:?})"),
    }
}

/// An `anyhow` result. An error whose cause chain holds the XML library's own error is recorded as
/// one stable class of ours, never by the library's name or text, which a new version renames; any
/// other error by its own text, which is this crate's.
fn untyped<T: Debug>(result: anyhow::Result<T>) -> String {
    match result {
        Ok(value) => format!("Ok({value:#?})"),
        Err(error) if from_xml_library(&error) => "Err(xml library error)".to_string(),
        Err(error) => format!("Err({error:#})"),
    }
}

fn from_xml_library(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause.downcast_ref::<quick_xml::Error>().is_some()
            || cause
                .downcast_ref::<quick_xml::escape::EscapeError>()
                .is_some()
            || cause
                .downcast_ref::<quick_xml::encoding::EncodingError>()
                .is_some()
            || cause.downcast_ref::<quick_xml::DeError>().is_some()
    })
}

fn date(yyyymmdd: &str) -> TallyDate {
    TallyDate::parse(yyyymmdd).expect("a date copied from a test")
}

/// Another fixture's decoded text, for an argument a test derives from a capture rather than types.
fn fixture_text(path: &str) -> String {
    decoded(&fs::read(crate_dir().join(path)).unwrap_or_else(|error| panic!("{path}: {error}")))
}

/// A whole result whose value is shown through its accessors, as `view` renders it, because its
/// own `Debug` redacts or omits fields.
fn viewed<T, V: Debug, E: Debug>(result: Result<T, E>, view: impl FnOnce(T) -> V) -> String {
    typed(result.map(view))
}

/// `CompanyBookExtent`'s `Debug` prints its company as `PinnedCompany([verified identity])`; every
/// field is shown here through the public accessors instead.
#[derive(Debug)]
#[allow(dead_code, reason = "read only through Debug")]
struct CompanyBookExtentFields {
    company_name: String,
    company_guid: String,
    books_from: TallyDate,
    last_voucher_date: TallyDate,
    voucher_alter_id_high_water: Option<u64>,
    master_alter_id_high_water: Option<u64>,
}

fn extent(result: Result<CompanyBookExtent, OutstandingsError>) -> String {
    viewed(result, |extent| CompanyBookExtentFields {
        company_name: extent.company().name().to_string(),
        company_guid: extent.company().guid().to_string(),
        books_from: extent.books_from().clone(),
        last_voucher_date: extent.last_voucher_date().clone(),
        voucher_alter_id_high_water: extent.voucher_alter_id_high_water().map(|mark| mark.get()),
        master_alter_id_high_water: extent.master_alter_id_high_water().map(|mark| mark.get()),
    })
}

/// A V2 extent read against the expectation production builds from the same company's row of the
/// Company collection read alongside it (`company_list`), selected by the GUID a test names.
fn extent_v2(text: &str, company_list: &str, guid: &str) -> String {
    let company = parse_companies_from_collection(&fixture_text(company_list))
        .expect("the paired company list parses")
        .into_iter()
        .find(|company| company.guid.as_deref() == Some(guid))
        .expect("the paired company list names the company");
    let expectation = CompanyBookExtentExpectation::new(
        company.name,
        guid.to_string(),
        company
            .company_number
            .expect("the list row carries a number"),
        company.books_from.expect("the list row carries BOOKSFROM"),
    )
    .expect("the list row is a valid expectation");
    extent(parse_company_book_extent_v2(text, &expectation))
}

/// `ParsedImportEvidence`'s `Debug` prints only the count of its line-error digests; every field is
/// shown here through the public accessors instead.
#[derive(Debug)]
#[allow(dead_code, reason = "read only through Debug")]
struct ParsedImportEvidenceFields {
    application_status: TallyImportApplicationStatus,
    counters: TallyImportResult,
    exceptions_were_reported: bool,
    response_sha256: String,
    line_error_sha256: Vec<String>,
}

fn import_evidence(text: &str) -> String {
    untyped(
        parse_import_evidence(text).map(|evidence| ParsedImportEvidenceFields {
            application_status: evidence.application_status(),
            counters: evidence.counters().clone(),
            exceptions_were_reported: evidence.exceptions_were_reported(),
            response_sha256: evidence.response_sha256().to_string(),
            line_error_sha256: evidence.line_error_sha256().to_vec(),
        }),
    )
}

fn bills(text: &str, books_from: &str, as_of: &str) -> String {
    typed(parse_native_bill_rows(
        text,
        &date(books_from),
        &date(as_of),
    ))
}

fn cash_flow(text: &str, from: &str, to: &str) -> String {
    let window = WholeMonthWindow::new(DateBoundaryProfile::ModeAgnostic, date(from), date(to))
        .expect("a whole-month window copied from a test");
    typed(parse_native_cash_flow(text, &window))
}

fn company_features(text: &str, guid: &str, name: &str, number: &str) -> String {
    typed(parse_company_features(
        text,
        &ExpectedCompany {
            guid,
            name,
            number,
            books_from: "20250401",
        },
    ))
}

const FOREX_GUID: &str = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
const SHAPE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
/// The parity lab book's company GUID, as scrubbed in its captures (`native_masters_tests.rs`).
const PARITY_GUID: &str = "7c0de000-0000-4000-8000-0000000000a1";
const WR2_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const AARAV_GUID: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";
const AARAV_NAME: &str = "Aarav Trading Company Demo";
const OUTSTANDINGS_LAB_GUID: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";
const REGISTER_LAB_GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";
const READS_LAB_GUID: &str = "de2e15f2-6d42-4715-b6e7-b7a95a68abe8";
const STOCK_LAB_GUID: &str = "1edb9b05-8d35-4c0c-9959-dce655730463";
const MASTER_FIELDS_LAB_GUID: &str = "56359347-3976-4d01-b44e-56fa0f6a422c";
const COMPANY_CURRENCY_NAMES: &str = "tests/fixtures/company_currencyname_live.utf16le.xml";

/// The base of a book with several masters, built the way
/// `forex_identifies_its_rupee_base_and_sets_its_dollar_ledgers_aside` builds FOREX's (the unit
/// tests' `among_several_for_tests` is not public): the currency read with `ORIGINALNAME` and the
/// company's `CURRENCYNAME`. `expected` is the base the tests name.
fn base_among_several(currencies: &str, guid: &str, expected: &str) -> BaseCurrencyName {
    let company = parse_company_currency_name(&fixture_text(COMPANY_CURRENCY_NAMES), guid)
        .expect("the captured company currency name");
    let base = parse_currency_master_list(&fixture_text(currencies))
        .expect("the captured currency masters")
        .identify_base(Some(&company))
        .expect("the base is identified");
    assert_eq!(base.base().name(), expected);
    base.base().clone()
}

/// `I₹`, FOREX's base (`ledger_currency_tests::forex_base`).
fn forex_base() -> BaseCurrencyName {
    base_among_several(
        "tests/fixtures/currency_originalname_forex_live.utf16le.xml",
        FOREX_GUID,
        "I\u{20b9}",
    )
}

/// `I₹`, SHAPE LAB's base (`runtime_outstandings_currency_tests::shape_plans`).
fn shape_base() -> BaseCurrencyName {
    base_among_several(
        "tests/fixtures/currency_originalname_shape_live.utf16le.xml",
        SHAPE_GUID,
        "I\u{20b9}",
    )
}

/// `Rs.`, the one master of the legacy single-currency book (`ledger_currency_tests::rs_base`).
fn rs_base() -> BaseCurrencyName {
    let currency = parse_company_currency(&fixture_text(
        "tests/fixtures/currency_inr_legacy_live.utf16le.xml",
    ))
    .expect("the captured single-master currency read");
    BaseCurrencyName::of_single_master(&currency).expect("one master")
}

fn companies(text: &str) -> String {
    untyped(parse_companies_from_collection(text))
}

fn gateway(text: &str) -> String {
    untyped(parse_company_gateway_capability_observation(text))
}

fn groups(text: &str, guid: &str) -> String {
    typed(parse_native_group_snapshot(text, guid))
}

fn group_sources(text: &str, guid: &str) -> String {
    typed(parse_native_group_source_records_with_evidence(text, guid))
}

fn ledger_sources(text: &str, guid: &str) -> String {
    untyped(parse_native_ledger_source_records_with_evidence(text, guid))
}

fn party_masters(text: &str, guid: &str) -> String {
    untyped(parse_native_party_ledger_master_records_with_evidence(
        text, guid,
    ))
}

fn ledger_snapshot_for(text: &str, guid: &str) -> String {
    typed(parse_native_ledger_snapshot_for_company(text, guid))
}

fn catalogue(text: &str, name: &str, guid: &str) -> String {
    typed(parse_standard_ledger_catalog_with_identities(
        text, name, guid,
    ))
}

fn catalogue_v2(text: &str, name: &str, guid: &str) -> String {
    typed(parse_standard_ledger_catalog_v2_with_identities(
        text, name, guid,
    ))
}

fn currency(text: &str) -> String {
    typed(parse_company_currency(text))
}

fn currency_masters(text: &str) -> String {
    typed(parse_currency_master_list(text))
}

fn masters(kind: NativeMasterKind, text: &str, guid: &str) -> String {
    typed(parse_native_masters(kind, text, guid))
}

const ROWS: &[Row] = &[
    // tests/company_ledger_count.rs
    Row {
        fixture: "tests/fixtures/agent/company-ledger-count.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_ledger_count(BRIDGE SIZE 2K)",
        parse: |_, text| {
            typed(parse_company_ledger_count(
                text,
                "BRIDGE SIZE 2K",
                "f8dab51e-a5d9-49d5-9232-54a16c8b95bb",
            ))
        },
    },
    // tests/ledger_census_slice.rs
    Row {
        fixture: "tests/fixtures/agent/ledger-census-slice-eight-rows.utf16le.xml",
        source: Source::Captured,
        parser: "parse_ledger_census_slice(max 8)",
        parse: |_, text| {
            typed(parse_ledger_census_slice(
                text,
                "f8dab51e-a5d9-49d5-9232-54a16c8b95bb",
                8,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/agent/ledger-census-slice-eight-rows.utf16le.xml",
        source: Source::Captured,
        parser: "parse_ledger_census_slice(max 7)",
        parse: |_, text| {
            typed(parse_ledger_census_slice(
                text,
                "f8dab51e-a5d9-49d5-9232-54a16c8b95bb",
                7,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/agent/ledger-census-slice-empty.utf16le.xml",
        source: Source::Captured,
        parser: "parse_ledger_census_slice(max 8)",
        parse: |_, text| {
            typed(parse_ledger_census_slice(
                text,
                "f8dab51e-a5d9-49d5-9232-54a16c8b95bb",
                8,
            ))
        },
    },
    // tests/simulator_corpus.rs, src/agent_tests.rs (an empty collection's export status)
    Row {
        fixture: "tests/fixtures/agent/native-bank-allocation-status.utf16le.xml",
        source: Source::Captured,
        parser: "export_status",
        parse: |_, text| untyped(export_status(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/native-empty-collection.utf16le.xml",
        source: Source::Captured,
        parser: "export_status",
        parse: |_, text| untyped(export_status(text)),
    },
    // Company collections: tests/company_release_observation.rs,
    // tests/native_outstandings_live_fixtures.rs, tests/simulator_corpus.rs; the app's replays
    // (src/agent_*_tests.rs, src/tally/runtime_*_tests.rs) for the rest, read as production reads a
    // company list (parse_companies_from_collection and the gateway observation).
    Row {
        fixture: "tests/fixtures/agent/native-licensed-release-companies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-licensed-release-companies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-licensed-release-companies.utf16le.xml",
        source: Source::Captured,
        parser: "company_list_may_be_in_educational_mode",
        parse: |_, text| format!("{:?}", company_list_may_be_in_educational_mode(text)),
    },
    Row {
        fixture: "tests/fixtures/native/company_collection_live.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/company_identity_fields_after_split.utf8.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/company_list_before_split.utf8.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/company_list_after_split.utf8.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-licensed-companies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-licensed-companies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/company_list_synthetic_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/company_list_synthetic_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/agent/d3-batch-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/d3-batch-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/agent/d3-cancelled-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/d3-cancelled-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/agent/l1-reentry-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/l1-reentry-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/agent/wa1-payment-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/wa1-payment-company-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-company-list.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-company-list.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_company_list_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_company_list_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_company_list_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_companies_from_collection",
        parse: |_, text| companies(text),
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_company_list_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_gateway_capability_observation",
        parse: |_, text| gateway(text),
    },
    // Company book extents: tests/outstandings_shared.rs (V1 and the split V2 tuple); the app's
    // replays for the rest, against the expectation built from the paired company list.
    Row {
        fixture: "tests/fixtures/unit_a_company_extent_live.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent(Aarav)",
        parse: |_, text| extent(parse_company_book_extent(text, AARAV_NAME, AARAV_GUID)),
    },
    Row {
        fixture: "tests/fixtures/native/company_extent_9000.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent(Aarav)",
        parse: |_, text| extent(parse_company_book_extent(text, AARAV_NAME, AARAV_GUID)),
    },
    Row {
        fixture: "tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent_v2(PROBE B SANDBOX 100005)",
        parse: |_, text| {
            let expectation = CompanyBookExtentExpectation::new(
                "BRIDGE PROBE B SANDBOX".to_string(),
                "ec4454ae-5c4c-4bfa-b3b0-68182a749689".to_string(),
                "100005".to_string(),
                "20250401".to_string(),
            )
            .expect("the split tuple of tests/outstandings_shared.rs");
            extent(parse_company_book_extent_v2(text, &expectation))
        },
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-book-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent_v2(OUTSTANDINGS LAB)",
        parse: |_, text| {
            extent_v2(
                text,
                "tests/fixtures/agent/native-outstandings-detail-company-list.utf16le.xml",
                OUTSTANDINGS_LAB_GUID,
            )
        },
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-book-extent.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent_v2(REGISTER LAB)",
        parse: |_, text| {
            extent_v2(
                text,
                "tests/fixtures/agent/register-e2e/native-register-e2e-extent.utf16le.xml",
                REGISTER_LAB_GUID,
            )
        },
    },
    Row {
        fixture: "tests/fixtures/company_extents_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent_v2(FOREX)",
        parse: |_, text| {
            extent_v2(
                text,
                "tests/fixtures/company_list_synthetic_live.utf16le.xml",
                FOREX_GUID,
            )
        },
    },
    Row {
        fixture: "tests/fixtures/company_extents_synthetic_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent_v2(SHAPE LAB)",
        parse: |_, text| {
            extent_v2(
                text,
                "tests/fixtures/company_list_synthetic_live.utf16le.xml",
                SHAPE_GUID,
            )
        },
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_book_extent_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent_v2(SHAPE LAB)",
        parse: |_, text| {
            extent_v2(
                text,
                "tests/fixtures/note-days/shape_lab_company_list_response.utf16le.xml",
                SHAPE_GUID,
            )
        },
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_book_extent_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_book_extent_v2(STOCK LAB)",
        parse: |_, text| {
            extent_v2(
                text,
                "tests/fixtures/stock-lab-day/stock_lab_taxed_day_company_list_response.utf16le.xml",
                STOCK_LAB_GUID,
            )
        },
    },
    // Import responses: tests/import_evidence.rs; the app's import tests
    // (src/agent_import_*_tests.rs) for the rest, read by production's parse_import_outcome.
    Row {
        fixture: "tests/fixtures/live_education_w1_ledger_sanitized.xml",
        source: Source::Synthetic,
        parser: "parse_import_evidence",
        parse: |_, text| import_evidence(text),
    },
    Row {
        fixture: "tests/fixtures/live_education_w1_ledger_sanitized.xml",
        source: Source::Synthetic,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/live_education_w4_voucher_sanitized.xml",
        source: Source::Synthetic,
        parser: "parse_import_evidence",
        parse: |_, text| import_evidence(text),
    },
    Row {
        fixture: "tests/fixtures/live_education_w4_voucher_sanitized.xml",
        source: Source::Synthetic,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/live_education_w7_baddate_sanitized.xml",
        source: Source::Synthetic,
        parser: "parse_import_evidence",
        parse: |_, text| import_evidence(text),
    },
    Row {
        fixture: "tests/fixtures/live_education_w7_baddate_sanitized.xml",
        source: Source::Synthetic,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/import_line_error_partial_commit_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_evidence",
        parse: |_, text| import_evidence(text),
    },
    Row {
        fixture: "tests/fixtures/import_line_error_partial_commit_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/post-span-import-response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/post-span-journal-import-response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/batch-import-all-missing-ledgers.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/batch-import-two-missing-ledgers-in-one-voucher.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/partial-import-missing-ledger.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/single-import-missing-ledger.utf16le.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    // The master-fields lab's create and alter answers (native/PROVENANCE.md, §9.4a): import
    // responses, read by production's parse_import_outcome; no test parses them.
    Row {
        fixture: "tests/fixtures/native/master_fields_lab_partial_alter_create.response.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    Row {
        fixture: "tests/fixtures/native/master_fields_lab_partial_alter.response.xml",
        source: Source::Captured,
        parser: "parse_import_outcome",
        parse: |_, text| untyped(parse_import_outcome(text)),
    },
    // Standard ledger catalogues: tests/standard_ledger_catalogue_rows.rs,
    // tests/decoder_convergence.rs; src/source_draft/catalog_tests.rs and the app's replays for the
    // V1 captures no protocol test reads.
    Row {
        fixture: "tests/fixtures/agent/native-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(WR2)",
        parse: |_, text| catalogue(text, "WR2 Unicode Lab", WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_identity_observation(WR2)",
        parse: |_, text| {
            untyped(parse_standard_ledger_identity_observation(
                text,
                "WR2 Unicode Lab",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/agent/escape-lab-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(ESCAPE LAB)",
        parse: |_, text| {
            catalogue(
                text,
                "BRIDGE ESCAPE & LAB",
                "21463ec7-c236-44fb-a06d-20f0ad8a6df3",
            )
        },
    },
    Row {
        fixture: "tests/fixtures/agent/escape-lab-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(another name)",
        parse: |_, text| {
            catalogue(
                text,
                "BRIDGE SOME OTHER LAB",
                "21463ec7-c236-44fb-a06d-20f0ad8a6df3",
            )
        },
    },
    Row {
        fixture: "tests/fixtures/agent/native-shape-lab-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(SHAPE LAB)",
        parse: |_, text| catalogue(text, "BRIDGE SHAPE LAB", SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_v2_with_identities(OUTSTANDINGS LAB)",
        parse: |_, text| catalogue_v2(text, "BRIDGE OUTSTANDINGS LAB", OUTSTANDINGS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(OUTSTANDINGS LAB)",
        parse: |_, text| catalogue(text, "BRIDGE OUTSTANDINGS LAB", OUTSTANDINGS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/d3-amend-lab-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_v2_with_identities(AMEND LAB)",
        parse: |_, text| {
            catalogue_v2(
                text,
                "BRIDGE AMEND LAB",
                "17a10910-773c-42c6-bd66-7bba9a392536",
            )
        },
    },
    Row {
        fixture: "tests/fixtures/agent/d3-amend-lab-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(AMEND LAB)",
        parse: |_, text| {
            catalogue(
                text,
                "BRIDGE AMEND LAB",
                "17a10910-773c-42c6-bd66-7bba9a392536",
            )
        },
    },
    Row {
        fixture: "tests/fixtures/agent/native-shape-lab-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_v2_with_identities(SHAPE LAB)",
        parse: |_, text| catalogue_v2(text, "BRIDGE SHAPE LAB", SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-shape-lab-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(SHAPE LAB)",
        parse: |_, text| catalogue(text, "BRIDGE SHAPE LAB", SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_v2_with_identities(WR2)",
        parse: |_, text| catalogue_v2(text, "WR2 Unicode Lab", WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ledger-catalogue-v2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(WR2)",
        parse: |_, text| catalogue(text, "WR2 Unicode Lab", WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ledger-catalogue-renamed.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(WR2)",
        parse: |_, text| catalogue(text, "WR2 Unicode Lab", WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/d3-amend-lab-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(AMEND LAB)",
        parse: |_, text| {
            catalogue(
                text,
                "BRIDGE AMEND LAB",
                "17a10910-773c-42c6-bd66-7bba9a392536",
            )
        },
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(OUTSTANDINGS LAB)",
        parse: |_, text| catalogue(text, "BRIDGE OUTSTANDINGS LAB", OUTSTANDINGS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/shape-lab-standard-ledger-catalogue.utf16le.xml",
        source: Source::Captured,
        parser: "parse_standard_ledger_catalog_with_identities(SHAPE LAB)",
        parse: |_, text| catalogue(text, "BRIDGE SHAPE LAB", SHAPE_GUID),
    },
    // Native statements: src/native_statement_reports_tests.rs; src/reports/statements_tests.rs for
    // the dense month's Profit & Loss.
    Row {
        fixture: "tests/fixtures/statement_balance_sheet_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_statement(BalanceSheet)",
        parse: |_, text| {
            typed(parse_native_statement(
                NativeStatementKind::BalanceSheet,
                text,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/statement_profit_and_loss_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_statement(ProfitAndLoss)",
        parse: |_, text| {
            typed(parse_native_statement(
                NativeStatementKind::ProfitAndLoss,
                text,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/statement_balance_sheet_empty_month_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_statement(BalanceSheet)",
        parse: |_, text| {
            typed(parse_native_statement(
                NativeStatementKind::BalanceSheet,
                text,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/statement_balance_sheet_dense_month_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_statement(BalanceSheet)",
        parse: |_, text| {
            typed(parse_native_statement(
                NativeStatementKind::BalanceSheet,
                text,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/statement_profit_and_loss_dense_month_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_statement(ProfitAndLoss)",
        parse: |_, text| {
            typed(parse_native_statement(
                NativeStatementKind::ProfitAndLoss,
                text,
            ))
        },
    },
    // Native trial balances: src/native_trial_balance/tests.rs; src/reports/statements_tests.rs for
    // the statement captures; the FOREX plain request (FOREX_601D_CAPTURE_PROVENANCE.md) by the
    // FOREX company of the same unit tests.
    Row {
        fixture: "tests/fixtures/native/trial_balance_known_lab.xml",
        source: Source::Captured,
        parser: "parse_native_trial_balance",
        parse: |_, text| {
            typed(parse_native_trial_balance(
                text,
                "eebb9a9f-1679-4468-9e8f-814c729674cb",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/native/trial_balance_opening_year.xml",
        source: Source::Captured,
        parser: "parse_native_trial_balance",
        parse: |_, text| {
            typed(parse_native_trial_balance(
                text,
                "915d42f8-42ae-4b03-8291-55f596e3a2ea",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/statement_trial_balance_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_trial_balance",
        parse: |_, text| typed(parse_native_trial_balance(text, READS_LAB_GUID)),
    },
    Row {
        fixture: "tests/fixtures/statement_trial_balance_dense_month_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_trial_balance",
        parse: |_, text| {
            typed(parse_native_trial_balance(
                text,
                "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/trial_balance_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_trial_balance",
        parse: |_, text| typed(parse_native_trial_balance(text, FOREX_GUID)),
    },
    Row {
        fixture: "tests/fixtures/trial_balance_currency_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_trial_balance_with_currency",
        parse: |_, text| {
            typed(parse_native_trial_balance_with_currency(
                text,
                FOREX_GUID,
                &forex_base(),
            ))
        },
    },
    // Built-in Cash Flow reports: src/native_cash_flow_tests.rs.
    Row {
        fixture: "tests/fixtures/builtin_cash_flow_probe_b_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20250401..20260331)",
        parse: |_, text| cash_flow(text, "20250401", "20260331"),
    },
    Row {
        fixture: "tests/fixtures/builtin_cash_flow_probe_b_apr_jun_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20250401..20250630)",
        parse: |_, text| cash_flow(text, "20250401", "20250630"),
    },
    Row {
        fixture: "tests/fixtures/builtin_cash_flow_probe_b_june_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20250601..20250630)",
        parse: |_, text| cash_flow(text, "20250601", "20250630"),
    },
    Row {
        fixture: "tests/fixtures/builtin_cash_flow_amend_lab_apr_sep_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20260401..20260930)",
        parse: |_, text| cash_flow(text, "20260401", "20260930"),
    },
    Row {
        fixture: "tests/fixtures/builtin_cash_flow_shape_lab_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20250401..20260331)",
        parse: |_, text| cash_flow(text, "20250401", "20260331"),
    },
    Row {
        fixture: "tests/fixtures/builtin_cash_flow_corpus_dense_fy_empty_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20250401..20260331)",
        parse: |_, text| cash_flow(text, "20250401", "20260331"),
    },
    Row {
        fixture: "tests/fixtures/builtin_negative_ledgers_probe_b_fy_empty_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20250401..20260331)",
        parse: |_, text| cash_flow(text, "20250401", "20260331"),
    },
    Row {
        fixture: "tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_cash_flow(20250401..20260331)",
        parse: |_, text| cash_flow(text, "20250401", "20260331"),
    },
    // Company settings and currency symbol: src/native_company_features_tests.rs.
    Row {
        fixture: "tests/fixtures/company_features_shape_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_features(BRIDGE SHAPE LAB)",
        parse: |_, text| company_features(text, SHAPE_GUID, "BRIDGE SHAPE LAB", "100021"),
    },
    Row {
        fixture: "tests/fixtures/company_features_corpus_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_features(BRIDGE CORPUS FOREX)",
        parse: |_, text| company_features(text, FOREX_GUID, "BRIDGE CORPUS FOREX", "100011"),
    },
    // Native masters: src/native_masters_tests.rs (SHAPE LAB and READS LAB);
    // src/agent_voucher_type_class_tests.rs for READS LAB's voucher types, read by production
    // (src/agent_vouchers.rs) as a voucher-type collection.
    Row {
        fixture: "tests/fixtures/masters_voucher_types_shape_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(VoucherTypes)",
        parse: |_, text| masters(NativeMasterKind::VoucherTypes, text, SHAPE_GUID),
    },
    // Series-level numbering: src/native_voucher_type_numbering_tests.rs (a Windows synthetic book).
    Row {
        fixture: "tests/fixtures/voucher_type_numbering_series_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_voucher_type_numbering",
        parse: |_, text| typed(parse_voucher_type_numbering(text)),
    },
    Row {
        fixture: "tests/fixtures/masters_godowns_shape_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(Godowns)",
        parse: |_, text| masters(NativeMasterKind::Godowns, text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_units_shape_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(Units)",
        parse: |_, text| masters(NativeMasterKind::Units, text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_stock_groups_shape_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(StockGroups)",
        parse: |_, text| masters(NativeMasterKind::StockGroups, text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_units_reads_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(Units)",
        parse: |_, text| masters(NativeMasterKind::Units, text, READS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_stock_groups_reads_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(StockGroups)",
        parse: |_, text| masters(NativeMasterKind::StockGroups, text, READS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_godowns_reads_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(Godowns)",
        parse: |_, text| masters(NativeMasterKind::Godowns, text, READS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-voucher-types-reads-lab.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_voucher_type_source_records_with_evidence(READS LAB)",
        parse: |_, text| {
            typed(parse_native_voucher_type_source_records_with_evidence(
                text,
                READS_LAB_GUID,
            ))
        },
    },
    // Stock: src/native_stock_summary_tests.rs (SHAPE LAB and the empty book).
    Row {
        fixture: "tests/fixtures/stock_items_shape_lab_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_stock_items",
        parse: |_, text| typed(parse_native_stock_items(text, SHAPE_GUID)),
    },
    Row {
        fixture: "tests/fixtures/stock_summary_report_shape_lab_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_stock_summary_report",
        parse: |_, text| typed(parse_native_stock_summary_report(text)),
    },
    Row {
        fixture: "tests/fixtures/company_inventory_flags_shape_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_inventory_flags",
        parse: |_, text| typed(parse_company_inventory_flags(text, SHAPE_GUID)),
    },
    Row {
        fixture: "tests/fixtures/company_inventory_flags_empty_book_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_inventory_flags",
        parse: |_, text| {
            typed(parse_company_inventory_flags(
                text,
                "c3edf50d-3dca-4213-a1f1-1d9fa331a674",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/stock_items_empty_book_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_stock_items",
        parse: |_, text| {
            typed(parse_native_stock_items(
                text,
                "c3edf50d-3dca-4213-a1f1-1d9fa331a674",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/stock_summary_report_empty_book_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_stock_summary_report",
        parse: |_, text| typed(parse_native_stock_summary_report(text)),
    },
    // Currency reads: src/native_outstandings/wire_currency_tests.rs,
    // wire_company_currency_tests.rs; the app's replays for the rest, read as production reads a
    // currency answer (parse_company_currency and the master list).
    Row {
        fixture: "tests/fixtures/currency_inr_modern_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/currency_inr_legacy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/currency_multi_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/currency_originalname_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/currency_originalname_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/currency_originalname_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/currency_originalname_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/currency_originalname_billwise_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/currency_originalname_validation_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/currency_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/currency_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-currencies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-currencies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-currencies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-currencies.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_currencies_1_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_currencies_1_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_currencies_2_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_currencies_2_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_currencies_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency",
        parse: |_, text| currency(text),
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_currencies_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_currency_master_list",
        parse: |_, text| currency_masters(text),
    },
    // The company's own currency name: src/native_outstandings/wire_company_currency_tests.rs, and
    // the note-day replay's base-currency read for SHAPE LAB.
    Row {
        fixture: "tests/fixtures/company_currencyname_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency_name(FOREX)",
        parse: |_, text| typed(parse_company_currency_name(text, FOREX_GUID)),
    },
    Row {
        fixture: "tests/fixtures/company_currencyname_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency_name(SHAPE LAB)",
        parse: |_, text| typed(parse_company_currency_name(text, SHAPE_GUID)),
    },
    Row {
        fixture: "tests/fixtures/company_currencyname_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency_name(Billwise Lab)",
        parse: |_, text| {
            typed(parse_company_currency_name(
                text,
                "75f7566d-7a4f-431a-9642-e93a9d06d57d",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/company_currencyname_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency_name(Validation Lab)",
        parse: |_, text| {
            typed(parse_company_currency_name(
                text,
                "c6afd306-00e1-4f51-802a-babe44daddd3",
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_base_currency_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_company_currency_name(SHAPE LAB)",
        parse: |_, text| typed(parse_company_currency_name(text, SHAPE_GUID)),
    },
    // Ledger snapshots with currencies: src/native_outstandings/ledger_currency_tests.rs,
    // wire_currency_tests.rs, wire_company_currency_tests.rs;
    // src/tally/runtime_outstandings_currency_tests.rs for SHAPE LAB's.
    Row {
        fixture: "tests/fixtures/ledgers_currency_single_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/ledgers_currency_single_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_classified(Rs.)",
        parse: |_, text| typed(parse_native_ledger_snapshot_classified(text, &rs_base())),
    },
    Row {
        fixture: "tests/fixtures/ledgers_currency_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/ledgers_currency_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_classified(FOREX base)",
        parse: |_, text| {
            typed(parse_native_ledger_snapshot_classified(
                text,
                &forex_base(),
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/ledgers_currency_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_classified_for_company(FOREX)",
        parse: |_, text| {
            typed(parse_native_ledger_snapshot_classified_for_company(
                text,
                FOREX_GUID,
                &forex_base(),
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/ledgers_forex_composite_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/balance_snapshot_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_compliance_ledger_snapshot_for_company(FOREX)",
        parse: |_, text| {
            typed(parse_compliance_ledger_snapshot_for_company(
                text,
                FOREX_GUID,
                &forex_base(),
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/balance_snapshot_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_classified_for_company(FOREX)",
        parse: |_, text| {
            typed(parse_native_ledger_snapshot_classified_for_company(
                text,
                FOREX_GUID,
                &forex_base(),
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/balance_snapshot_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_classified(FOREX base)",
        parse: |_, text| {
            typed(parse_native_ledger_snapshot_classified(
                text,
                &forex_base(),
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/balance_snapshot_forex_post_receipt_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_classified(FOREX base)",
        parse: |_, text| {
            typed(parse_native_ledger_snapshot_classified(
                text,
                &forex_base(),
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/ledgers_currency_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_classified(SHAPE LAB base)",
        parse: |_, text| {
            typed(parse_native_ledger_snapshot_classified(
                text,
                &shape_base(),
            ))
        },
    },
    // Ledger snapshots without currencies: tests/native_outstandings.rs,
    // tests/native_outstandings_live_fixtures.rs, tests/native_outstandings_lab_live.rs;
    // src/tally/party_ledger_master_tests.rs for the master-fields lab; the encoding captures
    // (encoding/PROVENANCE.md: the same List of Ledgers) by the same parser.
    Row {
        fixture: "tests/fixtures/native/ledger_snapshot_billwise_lab.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/native/ledger_snapshot_validation_lab.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/native/ledger_snapshot_aarav.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/native/ledger_snapshot_master_fields_lab.utf8.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/encoding/led-ascii.bin",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/encoding/led-utf16.bin",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot",
        parse: |_, text| typed(parse_native_ledger_snapshot(text)),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-lab-ledgers.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_for_company(OUTSTANDINGS LAB)",
        parse: |_, text| ledger_snapshot_for(text, OUTSTANDINGS_LAB_GUID),
    },
    // The app's replays of the outstandings and compliance reads (src/agent_outstandings_tests.rs,
    // src/agent_ledgers_tests.rs, src/agent_register_server_tests.rs), read by production's
    // company-bound snapshot parser.
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-ledgers.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_for_company(OUTSTANDINGS LAB)",
        parse: |_, text| ledger_snapshot_for(text, OUTSTANDINGS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ageing-ledgers.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_for_company(Ageing Lab)",
        parse: |_, text| ledger_snapshot_for(text, "eebb9a9f-1679-4468-9e8f-814c729674cb"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-party-balances.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_for_company(WR2)",
        parse: |_, text| ledger_snapshot_for(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-ledgers-paired.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_for_company(REGISTER LAB)",
        parse: |_, text| ledger_snapshot_for(text, REGISTER_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_ledgers_2_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_for_company(SHAPE LAB)",
        parse: |_, text| ledger_snapshot_for(text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_ledgers_2_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_snapshot_for_company(STOCK LAB)",
        parse: |_, text| ledger_snapshot_for(text, STOCK_LAB_GUID),
    },
    // Native ledger collections: tests/native_ledgers.rs; src/tally/runtime_ledger_opening_tests.rs
    // for the period opening, read by production's ledger-opening admission.
    Row {
        fixture: "tests/fixtures/native/ledgers_native_aarav.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_source_records_with_evidence(Aarav)",
        parse: |_, text| ledger_sources(text, AARAV_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/ledgers_native_wr2_core_window.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_source_records_with_evidence(WR2)",
        parse: |_, text| ledger_sources(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/ledgers_native_bvl.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_source_records_with_evidence(Validation Lab)",
        parse: |_, text| ledger_sources(text, "c6afd306-00e1-4f51-802a-babe44daddd3"),
    },
    Row {
        fixture: "tests/fixtures/native/ledgers_native_master_fields_lab.utf8.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_source_records_with_evidence(MASTER FIELDS LAB)",
        parse: |_, text| ledger_sources(text, MASTER_FIELDS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/compliance_master_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_source_records_with_evidence(FOREX)",
        parse: |_, text| ledger_sources(text, FOREX_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-period-opening.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_ledger_source_records_with_evidence(WR2)",
        parse: |_, text| ledger_sources(text, WR2_GUID),
    },
    // Party ledger masters: src/agent_ledgers_tests.rs, src/agent_register_tests.rs; the
    // compliance read's other replays (src/tally/runtime_party_evidence_tests.rs,
    // src/agent_register_server_tests.rs) by the same parser.
    Row {
        fixture: "tests/fixtures/agent/native-ledger-masters-duty-heads.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(REGISTER LAB)",
        parse: |_, text| party_masters(text, REGISTER_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ledger-masters-sgst-utgst.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(REGISTER LAB)",
        parse: |_, text| party_masters(text, REGISTER_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-party-masters-gst-registrations.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(READS LAB)",
        parse: |_, text| party_masters(text, READS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-party-masters.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(WR2)",
        parse: |_, text| party_masters(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-register-lab-ledger-masters.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(REGISTER LAB)",
        parse: |_, text| party_masters(text, REGISTER_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-ledgers-compliance.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(REGISTER LAB)",
        parse: |_, text| party_masters(text, REGISTER_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_ledgers_1_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(SHAPE LAB)",
        parse: |_, text| party_masters(text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_ledgers_1_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_party_ledger_master_records_with_evidence(STOCK LAB)",
        parse: |_, text| party_masters(text, STOCK_LAB_GUID),
    },
    // Group snapshots: tests/native_outstandings_live_fixtures.rs, tests/native_core_regressions.rs,
    // tests/decoder_convergence.rs, src/native_outstandings/wire_group_tests.rs; the app's replays
    // (src/reports/statements_tests.rs, src/agent_register_tests.rs, src/agent_import_bank_tests.rs,
    // src/agent_voucher_groups_tests.rs, src/tally/runtime_*_tests.rs) for the rest.
    Row {
        fixture: "tests/fixtures/native/group_snapshot_aarav_with_computed_company_guid.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(Aarav)",
        parse: |_, text| groups(text, AARAV_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_aarav_with_computed_company_guid.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(WR2, another company)",
        parse: |_, text| groups(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_aarav.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(Aarav)",
        parse: |_, text| groups(text, AARAV_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_wr2_with_identity.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_source_records_with_evidence(WR2)",
        parse: |_, text| group_sources(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_wr2.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(WR2)",
        parse: |_, text| groups(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_wr2.xml",
        source: Source::Captured,
        parser: "parse_native_group_source_records_with_evidence(WR2)",
        parse: |_, text| group_sources(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_aarav_with_identity.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(Aarav)",
        parse: |_, text| groups(text, AARAV_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_aarav_with_identity.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_source_records_with_evidence(Aarav)",
        parse: |_, text| group_sources(text, AARAV_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_master_fields_lab.utf8.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot_with_evidence(MASTER FIELDS LAB)",
        parse: |_, text| {
            typed(parse_native_group_snapshot_with_evidence(
                text,
                MASTER_FIELDS_LAB_GUID,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_master_fields_lab.utf8.xml",
        source: Source::Captured,
        parser: "parse_native_group_source_records_with_evidence(MASTER FIELDS LAB)",
        parse: |_, text| group_sources(text, MASTER_FIELDS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_validation_lab.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(Validation Lab)",
        parse: |_, text| groups(text, "c6afd306-00e1-4f51-802a-babe44daddd3"),
    },
    Row {
        fixture: "tests/fixtures/native/group_snapshot_validation_lab.xml",
        source: Source::Captured,
        parser: "parse_native_group_source_records_with_evidence(Validation Lab)",
        parse: |_, text| group_sources(text, "c6afd306-00e1-4f51-802a-babe44daddd3"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-lab-groups.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(OUTSTANDINGS LAB)",
        parse: |_, text| groups(text, OUTSTANDINGS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-detail-groups.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(OUTSTANDINGS LAB)",
        parse: |_, text| groups(text, OUTSTANDINGS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ageing-groups.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(Ageing Lab)",
        parse: |_, text| groups(text, "eebb9a9f-1679-4468-9e8f-814c729674cb"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-party-groups.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(WR2)",
        parse: |_, text| groups(text, WR2_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-register-lab-groups.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(REGISTER LAB)",
        parse: |_, text| groups(text, REGISTER_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/register-e2e/native-register-e2e-groups.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(REGISTER LAB)",
        parse: |_, text| groups(text, REGISTER_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/native-shape-lab-groups.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(SHAPE LAB)",
        parse: |_, text| groups(text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/agent/shape-lab-group-snapshot.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(SHAPE LAB)",
        parse: |_, text| groups(text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/groups_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(SHAPE LAB)",
        parse: |_, text| groups(text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/note-days/shape_lab_groups_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(SHAPE LAB)",
        parse: |_, text| groups(text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/stock-lab-day/stock_lab_taxed_day_groups_response.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(STOCK LAB)",
        parse: |_, text| groups(text, STOCK_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/groups_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(FOREX)",
        parse: |_, text| groups(text, FOREX_GUID),
    },
    Row {
        fixture: "tests/fixtures/group_snapshot_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(FOREX)",
        parse: |_, text| groups(text, FOREX_GUID),
    },
    Row {
        fixture: "tests/fixtures/statement_groups_fy_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(READS LAB)",
        parse: |_, text| groups(text, READS_LAB_GUID),
    },
    Row {
        fixture: "tests/fixtures/statement_groups_dense_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_group_snapshot(CORPUS DENSE)",
        parse: |_, text| groups(text, "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3"),
    },
    // Native voucher and voucher-type collections: tests/native_vouchers.rs,
    // tests/native_core_regressions.rs; src/tally/canonical_window_tests.rs for the dropped bound.
    Row {
        fixture: "tests/fixtures/native/vouchers_native_wr2.xml",
        source: Source::Captured,
        parser: "parse_native_voucher_source_records_with_evidence(WR2)",
        parse: |_, text| {
            typed(parse_native_voucher_source_records_with_evidence(
                text, WR2_GUID,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/native/voucher_types_native_wr2.xml",
        source: Source::Captured,
        parser: "parse_native_voucher_type_source_records_with_evidence(WR2)",
        parse: |_, text| {
            typed(parse_native_voucher_type_source_records_with_evidence(
                text, WR2_GUID,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/native/empty_voucher_window_wr2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_voucher_source_records_with_evidence(WR2)",
        parse: |_, text| {
            typed(parse_native_voucher_source_records_with_evidence(
                text, WR2_GUID,
            ))
        },
    },
    Row {
        fixture: "tests/fixtures/native/response-illegal-svtodate-bound-dropped-wr2.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_voucher_source_records_with_evidence(WR2)",
        parse: |_, text| {
            typed(parse_native_voucher_source_records_with_evidence(
                text, WR2_GUID,
            ))
        },
    },
    // Bills Receivable and Payable: tests/native_outstandings.rs,
    // tests/native_outstandings_live_fixtures.rs, tests/native_outstandings_opening_bills.rs,
    // tests/native_outstandings_lab_live.rs, tests/native_outstandings_long_due_date.rs,
    // tests/tally_xml_encoding_captures.rs, src/native_outstandings/ledger_currency_tests.rs,
    // src/agent_bill_trail_tests.rs; the app's replays (src/agent_outstandings_tests.rs,
    // src/tally/runtime_outstandings_currency_tests.rs) at the books-from of the company's extent and
    // the as-of the replay asks for.
    Row {
        fixture: "tests/fixtures/native/bills_receivable_billwise_lab.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20240401..20260731)",
        parse: |_, text| bills(text, "20240401", "20260731"),
    },
    Row {
        fixture: "tests/fixtures/native/bills_payable_billwise_lab_empty.xml",
        source: Source::Synthetic,
        parser: "parse_native_bill_rows(20240401..20260731)",
        parse: |_, text| bills(text, "20240401", "20260731"),
    },
    Row {
        fixture: "tests/fixtures/native/bills_receivable_ageing_lab.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20260401..20260731)",
        parse: |_, text| bills(text, "20260401", "20260731"),
    },
    Row {
        fixture: "tests/fixtures/native/bills_receivable_validation_lab.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260817)",
        parse: |_, text| bills(text, "20250401", "20260817"),
    },
    Row {
        fixture: "tests/fixtures/native/bills_payable_validation_lab.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260817)",
        parse: |_, text| bills(text, "20250401", "20260817"),
    },
    Row {
        fixture: "tests/fixtures/native/bills_receivable_aarav.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20240401..20260731)",
        parse: |_, text| bills(text, "20240401", "20260731"),
    },
    Row {
        fixture: "tests/fixtures/native/bills_payable_aarav.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20240401..20260731)",
        parse: |_, text| bills(text, "20240401", "20260731"),
    },
    Row {
        fixture: "tests/fixtures/native/bills_receivable_unloaded_company_failure.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20240401..20260731)",
        parse: |_, text| bills(text, "20240401", "20260731"),
    },
    Row {
        fixture: "tests/fixtures/bills_receivable_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20250930)",
        parse: |_, text| bills(text, "20250401", "20250930"),
    },
    Row {
        fixture: "tests/fixtures/bills_receivable_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20350930)",
        parse: |_, text| bills(text, "20250401", "20350930"),
    },
    Row {
        fixture: "tests/fixtures/bills_receivable_forex_post_c1_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260915)",
        parse: |_, text| bills(text, "20250401", "20260915"),
    },
    Row {
        fixture: "tests/fixtures/bills_payable_forex_post_c1_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260915)",
        parse: |_, text| bills(text, "20250401", "20260915"),
    },
    Row {
        fixture: "tests/fixtures/bills_receivable_forex_post_receipt_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260915)",
        parse: |_, text| bills(text, "20250401", "20260915"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-lab-bills-receivable.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260630)",
        parse: |_, text| bills(text, "20250401", "20260630"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-lab-bills-payable.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260630)",
        parse: |_, text| bills(text, "20250401", "20260630"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-lab-bills-receivable-after-journal.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260630)",
        parse: |_, text| bills(text, "20250401", "20260630"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-lab-bills-payable-after-journal.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260630)",
        parse: |_, text| bills(text, "20250401", "20260630"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-probe-b-bills-receivable.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260331)",
        parse: |_, text| bills(text, "20250401", "20260331"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-outstandings-probe-b-bills-payable.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260331)",
        parse: |_, text| bills(text, "20250401", "20260331"),
    },
    Row {
        fixture: "tests/fixtures/encoding/bills-utf16.bin",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20260401..20260801)",
        parse: |_, text| bills(text, "20260401", "20260801"),
    },
    Row {
        fixture: "tests/fixtures/encoding/bills-ascii.bin",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20260401..20260801)",
        parse: |_, text| bills(text, "20260401", "20260801"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ageing-receivable.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20260401..20260801)",
        parse: |_, text| bills(text, "20260401", "20260801"),
    },
    Row {
        fixture: "tests/fixtures/agent/native-ageing-payable.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20260401..20260801)",
        parse: |_, text| bills(text, "20260401", "20260801"),
    },
    Row {
        fixture: "tests/fixtures/bills_payable_forex_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20250930)",
        parse: |_, text| bills(text, "20250401", "20250930"),
    },
    Row {
        fixture: "tests/fixtures/bills_receivable_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260923)",
        parse: |_, text| bills(text, "20250401", "20260923"),
    },
    Row {
        fixture: "tests/fixtures/bills_payable_shape_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_bill_rows(20250401..20260923)",
        parse: |_, text| bills(text, "20250401", "20260923"),
    },
    // The simulator's authored corpus (../tally-protocol-simulator/fixtures/PROVENANCE.md):
    // tests/simulator_corpus.rs. `inconsistent_date_filter.xml` has no parsing test; it is a
    // Bridge-schema voucher export, read here as `voucher_export.xml` is.
    Row {
        fixture: "../tally-protocol-simulator/fixtures/export_status_1.xml",
        source: Source::Synthetic,
        parser: "export_status",
        parse: |_, text| untyped(export_status(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/export_status_0.xml",
        source: Source::Synthetic,
        parser: "export_status",
        parse: |_, text| untyped(export_status(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/export_status_0.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers",
        parse: |_, text| untyped(parse_ledgers(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/export_status_missing.xml",
        source: Source::Synthetic,
        parser: "export_status",
        parse: |_, text| untyped(export_status(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/export_status_invalid.xml",
        source: Source::Synthetic,
        parser: "export_status",
        parse: |_, text| untyped(export_status(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/normal_export.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers",
        parse: |_, text| untyped(parse_ledgers(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/normal_export.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers_with_evidence",
        parse: |_, text| untyped(parse_ledgers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/normal_export.xml",
        source: Source::Synthetic,
        parser: "parse_ledger_source_records_with_evidence",
        parse: |_, text| untyped(parse_ledger_source_records_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/empty_export.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers_with_evidence",
        parse: |_, text| untyped(parse_ledgers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/duplicate_identity.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers_with_evidence",
        parse: |_, text| untyped(parse_ledgers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/wrong_company.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers_with_evidence",
        parse: |_, text| untyped(parse_ledgers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/record_count_mismatch.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers_with_evidence",
        parse: |_, text| untyped(parse_ledgers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/malformed_export_metadata.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers_with_evidence",
        parse: |_, text| untyped(parse_ledgers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/duplicate_export_metadata.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers_with_evidence",
        parse: |_, text| untyped(parse_ledgers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/voucher_export.xml",
        source: Source::Synthetic,
        parser: "parse_vouchers_with_evidence",
        parse: |_, text| untyped(parse_vouchers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/voucher_export.xml",
        source: Source::Synthetic,
        parser: "parse_voucher_source_records_with_evidence",
        parse: |_, text| untyped(parse_voucher_source_records_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/inconsistent_date_filter.xml",
        source: Source::Synthetic,
        parser: "parse_vouchers_with_evidence",
        parse: |_, text| untyped(parse_vouchers_with_evidence(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/import_counters.xml",
        source: Source::Synthetic,
        parser: "parse_import_result",
        parse: |_, text| untyped(parse_import_result(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/import_duplicate.xml",
        source: Source::Synthetic,
        parser: "parse_import_result",
        parse: |_, text| untyped(parse_import_result(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/import_partial.xml",
        source: Source::Synthetic,
        parser: "parse_import_result",
        parse: |_, text| untyped(parse_import_result(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/malformed.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers",
        parse: |_, text| untyped(parse_ledgers(text)),
    },
    Row {
        fixture: "../tally-protocol-simulator/fixtures/truncated.xml",
        source: Source::Synthetic,
        parser: "parse_ledgers",
        parse: |_, text| untyped(parse_ledgers(text)),
    },
    // src/native_masters_tests.rs: cost centres and cost categories (#1398).
    Row {
        fixture: "tests/fixtures/masters_cost_centres_shape_lab_flag_no_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(CostCentres, SHAPE LAB)",
        parse: |_, text| masters(NativeMasterKind::CostCentres, text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_cost_categories_shape_lab_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(CostCategories, SHAPE LAB)",
        parse: |_, text| masters(NativeMasterKind::CostCategories, text, SHAPE_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_cost_centres_corpus_forex_empty_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(CostCentres, FOREX)",
        parse: |_, text| masters(NativeMasterKind::CostCentres, text, FOREX_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_cost_centres_parity_flag_yes_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(CostCentres, PARITY LAB)",
        parse: |_, text| masters(NativeMasterKind::CostCentres, text, PARITY_GUID),
    },
    Row {
        fixture: "tests/fixtures/masters_cost_categories_parity_flag_yes_live.utf16le.xml",
        source: Source::Captured,
        parser: "parse_native_masters(CostCategories, PARITY LAB)",
        parse: |_, text| masters(NativeMasterKind::CostCategories, text, PARITY_GUID),
    },
];

const APP_VOUCHER_ROWS: &str =
    "recorded by the app crate's record (#1198 slice 2): parse_agent_rows (an agent voucher read)";
const APP_VERIFICATION: &str = "recorded by the app crate's record (#1198 slice 2): \
     parse_import_verification_rows (an import's verification or span read)";
const APP_CENSUS: &str =
    "recorded by the app crate's record (#1198 slice 2): parse_voucher_census (a voucher census)";
const APP_HIGH_WATER: &str = "recorded by the app crate's record (#1198 slice 2): \
     parse_company_high_water / parse_company_marks (a company ALTVCHID/ALTMSTID read)";
const APP_FOREX_VOUCHERS: &str = "recorded by the app crate's record (#1198 slice 2): \
     parse_agent_rows_withholding (src/currency_composite_tests.rs only classifies its amounts)";
const VOUCHER_SCAN_AND_APP: &str = "recorded by the app crate's record (#1198 slice 2): \
     parse_agent_rows; this crate parses it only through the voucher-scan feature's pub(super) \
     parse_segment (behind verify_segment_pair), which the default build does not compile";
const VOUCHER_SCAN_ONLY: &str = "parsed only by the voucher-scan feature's pub(super) \
     parse_segment (behind verify_segment_pair), which the default build does not compile; no \
     public parser of the default build reads this voucher export";
const IMPORT_REQUEST: &str =
    "an import request body Bridge sends to Tally (TALLYMESSAGE under REQUESTDATA), not a response";
const NOT_READ_REPORT: &str = "a built-in report no parser of this crate reads \
     (BUILTIN_REPORTS_CAPTURE_PROVENANCE.md: kept as evidence for §12a.16)";
const OBJECT_READBACK: &str = "a master-fields lab object readback (§9.4a): tests lift single \
     fields from it into a collection; no parser of this crate reads the object envelope";

/// Response fixtures with no row, each with its reason. Requests are recognised by name and need
/// no entry.
const NOT_RECORDED: &[(&str, &str)] = &[
    // Import request bodies named without `request`.
    ("tests/fixtures/agent/d3-batch-import.xml", IMPORT_REQUEST),
    ("tests/fixtures/agent/l1-reentry-import.xml", IMPORT_REQUEST),
    ("tests/fixtures/agent/post-span-import.xml", IMPORT_REQUEST),
    (
        "tests/fixtures/agent/post-span-journal-import.xml",
        IMPORT_REQUEST,
    ),
    (
        "tests/fixtures/agent/wa1-payment-import.xml",
        IMPORT_REQUEST,
    ),
    (
        "tests/fixtures/voucher_import_candidate_structure_derived.xml",
        "recorded by the app crate's record (#1198 slice 2): parse_source_xml; a synthetic \
         voucher-import file (a structural derivative, not a Tally response) that only \
         src/source_draft_xml_tests.rs reads",
    ),
    // Company high-water and mark reads.
    (
        "tests/fixtures/agent/d3-batch-company-high-water.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/d3-cancelled-company-high-water.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/l1-reentry-company-high-water.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/wa1-payment-company-high-water.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/post-span-company-high-water-after.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/post-span-company-high-water-before.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/post-span-journal-company-high-water-after.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/post-span-journal-company-high-water-before.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/native-outstandings-detail-high-water.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/register-e2e/native-register-e2e-marks.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/note-days/shape_lab_company_marks_response.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/stock-lab-day/stock_lab_taxed_day_company_marks_response.utf16le.xml",
        APP_HIGH_WATER,
    ),
    (
        "tests/fixtures/agent/native-company-book-extents.utf16le.xml",
        "recorded by the app crate's record (#1198 slice 2): company_high_water_rows \
         (src/agent_company_checkpoint.rs reads this extent collection for its marks)",
    ),
    // Voucher censuses.
    (
        "tests/fixtures/agent/d3-batch-voucher-census.utf16le.xml",
        APP_CENSUS,
    ),
    (
        "tests/fixtures/agent/d3-cancelled-voucher-census.utf16le.xml",
        APP_CENSUS,
    ),
    (
        "tests/fixtures/agent/l1-reentry-voucher-census.utf16le.xml",
        APP_CENSUS,
    ),
    (
        "tests/fixtures/agent/wa1-payment-voucher-census.utf16le.xml",
        APP_CENSUS,
    ),
    (
        "tests/fixtures/agent/register-e2e/native-register-e2e-census.utf16le.xml",
        APP_CENSUS,
    ),
    (
        "tests/fixtures/note-days/cancelled_purchase_day_voucher_census_response.utf16le.xml",
        APP_CENSUS,
    ),
    (
        "tests/fixtures/note-days/credit_note_day_voucher_census_response.utf16le.xml",
        APP_CENSUS,
    ),
    (
        "tests/fixtures/note-days/debit_note_day_voucher_census_response.utf16le.xml",
        APP_CENSUS,
    ),
    // Import verification and span reads.
    (
        "tests/fixtures/agent/d3-batch-import-verification.utf16le.xml",
        APP_VERIFICATION,
    ),
    (
        "tests/fixtures/agent/d3-cancelled-import-verification.utf16le.xml",
        APP_VERIFICATION,
    ),
    (
        "tests/fixtures/agent/l1-reentry-import-verification.utf16le.xml",
        APP_VERIFICATION,
    ),
    (
        "tests/fixtures/agent/wa1-payment-import-verification.utf16le.xml",
        APP_VERIFICATION,
    ),
    (
        "tests/fixtures/agent/post-span-alterid-span-read.utf16le.xml",
        APP_VERIFICATION,
    ),
    (
        "tests/fixtures/agent/post-span-journal-alterid-span-read.utf16le.xml",
        APP_VERIFICATION,
    ),
    (
        "tests/fixtures/agent/post-span-guid-absent.utf16le.xml",
        APP_VERIFICATION,
    ),
    (
        "tests/fixtures/agent/post-span-guid-present.utf16le.xml",
        APP_VERIFICATION,
    ),
    // Agent voucher reads.
    (
        "tests/fixtures/agent/native-billallocations-wildcard.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-entry-wildcard-allocations.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-education-empty-movement-part.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-namespaced-journal.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-three-vouchers.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-register-lab-vouchers-class.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-vouchers-renamed-purchase-class.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/vouchers-reference-date-year-rows.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-outstandings-detail-vouchers-window-from-20250420.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/native-outstandings-lab-vouchers-window.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/register-e2e/native-register-e2e-window.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/note-days/cancelled_purchase_day_voucher_window_response.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/note-days/credit_note_day_voucher_window_response.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/note-days/debit_note_day_voucher_window_response.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/sales-day/register_window_sales_day_live.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/sales-day/register_window_taxed_sales_day_live.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/stock-lab-day/stock_lab_taxed_day_voucher_window_response.utf16le.xml",
        APP_VOUCHER_ROWS,
    ),
    (
        "tests/fixtures/agent/vouchers-forex-bill-allocation-20260915.utf16le.xml",
        APP_FOREX_VOUCHERS,
    ),
    (
        "tests/fixtures/agent/vouchers-forex-composite-20260915.utf16le.xml",
        APP_FOREX_VOUCHERS,
    ),
    // The legacy voucher-scan exports.
    (
        "tests/fixtures/unit_a_optional_voucher_live.xml",
        VOUCHER_SCAN_AND_APP,
    ),
    (
        "tests/fixtures/vouchers_agst_ref_reopen_live.utf16le.xml",
        VOUCHER_SCAN_AND_APP,
    ),
    (
        "tests/fixtures/vouchers_gst_credit_periods_live.utf16le.xml",
        VOUCHER_SCAN_AND_APP,
    ),
    (
        "tests/fixtures/vouchers_settle_then_reopen_live.utf16le.xml",
        VOUCHER_SCAN_AND_APP,
    ),
    (
        "tests/fixtures/unit_a_vouchers_wildcard_live.xml",
        VOUCHER_SCAN_ONLY,
    ),
    (
        "tests/fixtures/vouchers_ageing_corpus_live.utf16le.xml",
        VOUCHER_SCAN_ONLY,
    ),
    // Built-in reports no parser reads.
    (
        "tests/fixtures/builtin_funds_flow_probe_b_fy_live.utf16le.xml",
        NOT_READ_REPORT,
    ),
    (
        "tests/fixtures/builtin_negative_stock_shape_lab_fy_live.utf16le.xml",
        NOT_READ_REPORT,
    ),
    (
        "tests/fixtures/builtin_ratio_analysis_probe_b_fy_live.utf16le.xml",
        NOT_READ_REPORT,
    ),
    (
        "tests/fixtures/builtin_sales_register_probe_b_fy_live.utf16le.xml",
        NOT_READ_REPORT,
    ),
    // Object readbacks and the sanitizer sample.
    (
        "tests/fixtures/native/master_fields_lab_partial_alter_before.response.xml",
        OBJECT_READBACK,
    ),
    (
        "tests/fixtures/native/master_fields_lab_partial_alter_after.response.xml",
        OBJECT_READBACK,
    ),
    (
        "tests/fixtures/unit_a_invalid_char_ref_live.xml",
        "only the pre-parse sanitizer reads it (src/tolerant_xml_tests.rs): a NAME/PARENT-only \
         ledger collection with no GUID, company or balance, which no parser of this crate binds",
    ),
];

/// The file a row's record is kept in: `<fixture path>.<parser>.txt` for a full rendering, or
/// `.sha256` for a hashed one, with `/` written as `__`.
fn record_stem(row: &Row) -> String {
    // A simulator fixture's path starts `../`; its record keeps the rest of the path.
    let fixture = row
        .fixture
        .trim_start_matches("tests/fixtures/")
        .trim_start_matches("../")
        .replace('/', "__");
    let parser: String = row
        .parser
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.') {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("{fixture}.{parser}")
}

fn render(row: &Row) -> String {
    let bytes = fs::read(crate_dir().join(row.fixture))
        .unwrap_or_else(|error| panic!("{}: unreadable: {error}", row.fixture));
    let text = decoded(&bytes);
    let rendering = (row.parse)(&bytes, &text);
    let source = match row.source {
        Source::Captured => "captured",
        Source::Synthetic => "synthetic: not evidence of what Tally sends",
    };
    format!(
        "# fixture: {}\n# parser: {}\n# source: {source}\n{rendering}\n",
        row.fixture, row.parser
    )
}

/// What is committed for a rendering: the rendering itself, or its SHA-256 and length.
fn committed_form(rendering: &str) -> (&'static str, String) {
    if rendering.len() <= FULL_RECORD_CAP {
        ("txt", rendering.to_string())
    } else {
        let digest: String = Sha256::digest(rendering.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        (
            "sha256",
            format!("sha256 {digest}\nlength {}\n", rendering.len()),
        )
    }
}

/// The first line at which `want` and `got` differ, as `line N: want … / got …`, each side cut to
/// 160 characters.
fn first_difference(want: &str, got: &str) -> String {
    let cut = |line: Option<&str>| -> String {
        line.map_or_else(
            || "(end)".to_string(),
            |line| line.chars().take(160).collect(),
        )
    };
    let (mut want_lines, mut got_lines) = (want.lines(), got.lines());
    for number in 1.. {
        let (w, g) = (want_lines.next(), got_lines.next());
        if w != g {
            return format!("line {number}: want {:?} / got {:?}", cut(w), cut(g));
        }
        if w.is_none() {
            break;
        }
    }
    "the files differ only in their final newline".to_string()
}

/// A dump directory outside the repository, or none.
fn dump_dir() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os(DUMP_VAR)?);
    let repository = crate_dir()
        .join("../../..")
        .canonicalize()
        .expect("repository root");
    assert!(
        dir.is_absolute() && !dir.starts_with(&repository),
        "{DUMP_VAR} must be an absolute path outside the repository"
    );
    Some(dir)
}

#[test]
fn every_parse_matches_its_record() {
    let recording = std::env::var_os(RECORD_VAR).is_some();
    assert!(
        !(recording && std::env::var_os("CI").is_some()),
        "{RECORD_VAR} writes the records and must not run in CI"
    );
    let dump = dump_dir();
    if let Some(dir) = &dump {
        fs::create_dir_all(dir).expect("dump directory");
    }
    let dir = records_dir();
    let mut stems = BTreeSet::new();
    let mut written = BTreeSet::from(["README.md".to_string()]);
    let mut mismatches = Vec::new();
    for row in ROWS {
        let stem = record_stem(row);
        assert!(
            stems.insert(stem.clone()),
            "two rows share the record {stem}"
        );
        let rendering = render(row);
        if let Some(dump) = &dump {
            fs::write(dump.join(format!("{stem}.txt")), &rendering).expect("dump written");
        }
        let (extension, committed) = committed_form(&rendering);
        let file = format!("{stem}.{extension}");
        let path = dir.join(&file);
        written.insert(file);
        if recording {
            fs::create_dir_all(&dir).expect("records directory");
            fs::write(&path, &committed).expect("record written");
            continue;
        }
        match fs::read_to_string(&path) {
            Ok(want) if want == committed => {}
            Ok(want) => mismatches.push(format!(
                "{} / {}: {}",
                row.fixture,
                row.parser,
                if extension == "txt" {
                    first_difference(&want, &committed)
                } else {
                    format!("hash differs (set {DUMP_VAR} to compare full renderings)")
                }
            )),
            Err(_) => mismatches.push(format!(
                "{} / {}: no record {}",
                row.fixture,
                row.parser,
                path.display()
            )),
        }
    }
    // The directory is exactly the table: a file no row writes (a removed row, or a record that
    // moved between full and hashed) is removed when recording and reported otherwise.
    for entry in fs::read_dir(&dir).expect("records directory") {
        let path = entry.expect("entry").path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();
        if written.contains(&name) {
            continue;
        }
        if recording {
            fs::remove_file(&path).expect("stale record removed");
        } else {
            mismatches.push(format!("{name}: a record no row writes"));
        }
    }
    if recording {
        return;
    }
    let shown: Vec<&String> = mismatches.iter().take(MISMATCHES_SHOWN).collect();
    assert!(
        mismatches.is_empty(),
        "{} parse(s) differ from their records:\n{}{}",
        mismatches.len(),
        shown
            .iter()
            .map(|line| format!("  {line}\n"))
            .collect::<String>(),
        if mismatches.len() > MISMATCHES_SHOWN {
            format!("  … and {} more\n", mismatches.len() - MISMATCHES_SHOWN)
        } else {
            String::new()
        }
    );
}

/// Whether a fixture's name marks it as a request: `_request` or `.request` just before its
/// extension (`.xml`, `.utf8.xml` or `.utf16le.xml`). A word `request` elsewhere in the name does
/// not, so a response fixture named that way still needs a row or a reason.
fn is_request_name(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".xml") else {
        return false;
    };
    let stem = stem
        .strip_suffix(".utf16le")
        .or_else(|| stem.strip_suffix(".utf8"))
        .unwrap_or(stem);
    stem.ends_with("_request") || stem.ends_with(".request")
}

/// Every response fixture under `tests/fixtures/`, as a path from the crate directory: every file
/// but documentation, generators, line-delimited journals and requests ([`is_request_name`]).
fn response_fixtures(root: &Path) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).expect("fixture directory") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name != "generators") {
                    pending.push(path);
                }
                continue;
            }
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            let skipped = [".md", ".py", ".json", ".jsonl", ".txt"]
                .iter()
                .any(|ext| name.ends_with(ext));
            if !skipped && !is_request_name(name) {
                let relative = path.strip_prefix(crate_dir()).expect("under the crate");
                found.insert(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    found
}

/// The table, with its stated exceptions, is exactly the crate's response fixtures.
fn coverage_problems(
    fixtures: &BTreeSet<String>,
    rows: &[Row],
    not_recorded: &[(&str, &str)],
) -> Vec<String> {
    let recorded: BTreeSet<&str> = rows.iter().map(|row| row.fixture).collect();
    let named: BTreeSet<&str> = not_recorded.iter().map(|(fixture, _)| *fixture).collect();
    let mut problems = Vec::new();
    for fixture in fixtures {
        if !recorded.contains(fixture.as_str()) && !named.contains(fixture.as_str()) {
            problems.push(format!("{fixture} has no row and no stated reason"));
        }
    }
    for fixture in recorded.intersection(&named) {
        problems.push(format!("{fixture} has a row and a reason not to record it"));
    }
    for (fixture, _) in not_recorded {
        if !fixtures.contains(*fixture) {
            problems.push(format!(
                "{fixture} is named as not recorded but is no response fixture"
            ));
        }
    }
    problems
}

#[test]
fn every_response_fixture_is_recorded_or_named() {
    let fixtures = response_fixtures(&crate_dir().join("tests/fixtures"));
    assert!(
        fixtures.len() >= 250,
        "the walk must have found the fixtures ({})",
        fixtures.len()
    );
    assert_eq!(
        coverage_problems(&fixtures, ROWS, NOT_RECORDED),
        Vec::<String>::new()
    );
}

/// The coverage check refuses each way the table can drift from the fixtures: a fixture with
/// neither a row nor a reason, one with both, and a reason for a fixture that is not there.
#[test]
fn coverage_names_each_way_the_table_drifts() {
    fn nothing(_: &[u8], _: &str) -> String {
        String::new()
    }
    let fixtures: BTreeSet<String> = ["tests/fixtures/a.xml", "tests/fixtures/b.xml"]
        .into_iter()
        .map(str::to_string)
        .collect();
    let rows = [Row {
        fixture: "tests/fixtures/a.xml",
        source: Source::Captured,
        parser: "p",
        parse: nothing,
    }];
    assert_eq!(
        coverage_problems(&fixtures, &rows, &[]),
        ["tests/fixtures/b.xml has no row and no stated reason"]
    );
    assert_eq!(
        coverage_problems(
            &fixtures,
            &rows,
            &[
                ("tests/fixtures/a.xml", "r"),
                ("tests/fixtures/b.xml", "r"),
                ("tests/fixtures/c.xml", "r")
            ]
        ),
        [
            "tests/fixtures/a.xml has a row and a reason not to record it",
            "tests/fixtures/c.xml is named as not recorded but is no response fixture",
        ]
    );
}

/// The start of every reason that leaves a fixture to the app crate's record
/// (`src/agent_parse_record_tests.rs`, with its records in `src-tauri/tests/parse_record/`).
const APP_RECORD: &str = "recorded by the app crate's record (#1198 slice 2)";

/// The fixture each of the app crate's records names in its `# fixture:` header (a path from
/// this crate's `tests/fixtures/`), as a path from this crate's directory. A record that names
/// none fails the test rather than counting as nothing.
fn app_record_fixtures() -> BTreeSet<String> {
    let dir = crate_dir().join("../../tests/parse_record");
    let mut found = BTreeSet::new();
    for entry in fs::read_dir(&dir).expect("the app crate's records directory") {
        let path = entry.expect("entry").path();
        if path.file_name().is_some_and(|name| name == "README.md") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("an app record");
        let fixture = text
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("# fixture: "))
            .unwrap_or_else(|| panic!("{}: names no fixture", path.display()));
        found.insert(format!("tests/fixtures/{fixture}"));
    }
    found
}

/// Where this table and the app crate's record disagree, by fixture.
#[derive(Debug, Default, PartialEq, Eq)]
struct AppRecordGaps {
    /// Left to the app crate's record, which has no record of it: in neither record.
    unrecorded: Vec<String>,
    /// In the app crate's record, which this table does not leave it to.
    unclaimed: Vec<String>,
}

fn app_record_gaps(
    not_recorded: &[(&str, &str)],
    app_fixtures: &BTreeSet<String>,
) -> AppRecordGaps {
    let left: BTreeSet<&str> = not_recorded
        .iter()
        .filter(|(_, reason)| reason.starts_with(APP_RECORD))
        .map(|(fixture, _)| *fixture)
        .collect();
    AppRecordGaps {
        unrecorded: left
            .iter()
            .filter(|fixture| !app_fixtures.contains(**fixture))
            .map(|fixture| (*fixture).to_string())
            .collect(),
        unclaimed: app_fixtures
            .iter()
            .filter(|fixture| !left.contains(fixture.as_str()))
            .cloned()
            .collect(),
    }
}

/// Every fixture this table leaves to the app crate's record is in it, and that record holds no
/// other fixture.
#[test]
fn every_fixture_left_to_the_app_crate_is_in_its_record() {
    assert_eq!(
        app_record_gaps(NOT_RECORDED, &app_record_fixtures()),
        AppRecordGaps::default()
    );
}

/// A fixture left to the app crate's record that it lacks is named, and so is one it holds that
/// this table records itself or leaves for another reason.
#[test]
fn app_record_gaps_name_each_fixture() {
    let app_fixtures: BTreeSet<String> = [
        "tests/fixtures/a.xml",
        "tests/fixtures/c.xml",
        "tests/fixtures/d.xml",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let not_recorded = [
        ("tests/fixtures/a.xml", APP_VOUCHER_ROWS),
        ("tests/fixtures/b.xml", APP_CENSUS),
        ("tests/fixtures/c.xml", IMPORT_REQUEST),
    ];
    assert_eq!(
        app_record_gaps(&not_recorded, &app_fixtures),
        AppRecordGaps {
            unrecorded: vec!["tests/fixtures/b.xml".to_string()],
            unclaimed: vec![
                "tests/fixtures/c.xml".to_string(),
                "tests/fixtures/d.xml".to_string()
            ],
        }
    );
}

/// Only the ending of a fixture's name marks it as a request.
#[test]
fn only_the_ending_of_a_name_marks_a_request() {
    for name in [
        "ledgers_request.xml",
        "ledgers.request.xml",
        "ledgers_request.utf16le.xml",
        "ledgers.request.utf8.xml",
    ] {
        assert!(is_request_name(name), "{name}");
    }
    for name in [
        "request_ledgers.xml",
        "ledgers_request_echo.utf16le.xml",
        "ledgers_request.txt",
        "ledgersrequest.xml",
        "ledgers.utf16le.xml",
    ] {
        assert!(!is_request_name(name), "{name}");
    }
}

/// A rendering of exactly the cap is kept in full and one byte more is hashed; a mismatch names the
/// first differing line, each side cut short.
#[test]
fn the_cap_and_the_first_difference_are_exact() {
    let at_cap = "x".repeat(FULL_RECORD_CAP);
    assert_eq!(committed_form(&at_cap), ("txt", at_cap.clone()));
    let over = "x".repeat(FULL_RECORD_CAP + 1);
    let (extension, committed) = committed_form(&over);
    assert_eq!(extension, "sha256");
    assert_eq!(
        committed.lines().nth(1),
        Some(format!("length {}", FULL_RECORD_CAP + 1).as_str())
    );
    assert_eq!(
        first_difference("a\nb\nc\n", "a\nB\nc\n"),
        r#"line 2: want "b" / got "B""#
    );
    assert_eq!(
        first_difference("a\n", "a\nb\n"),
        r#"line 2: want "(end)" / got "b""#
    );
    let long = "y".repeat(500);
    assert_eq!(
        first_difference(&long, "z"),
        format!(r#"line 1: want "{}" / got "z""#, "y".repeat(160))
    );
}
