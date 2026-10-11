//! bridge#1198, slice 2: a record of what this crate's own XML parsers return today for the
//! response fixtures they read, so that a change of XML library must reproduce it exactly. The
//! protocol crate's record (`crates/bridge-tally-protocol/tests/captured_parse_record.rs`) leaves
//! these fixtures to this one. How the records are made and read: `tests/parse_record/README.md`.
//!
//! One row per (fixture, parser). Each row decodes its fixture as the protocol crate's record
//! does, parses it with fixed arguments copied from the tests that read it, renders the whole
//! result, and compares the rendering with its committed record. A rendering up to
//! [`FULL_RECORD_CAP`] bytes is committed in full; a longer one as its SHA-256 and length. A
//! mismatch names the fixture, the parser and the first differing line, and no more (AGENTS.md
//! P5).
#![allow(
    clippy::disallowed_methods,
    reason = "the test reads and, in its recording mode, writes its own record files"
)]

use std::collections::BTreeSet;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

use bridge_tally_protocol::{decode_tally_xml_response_bytes_limited, ExpectedTallyTextEncoding};
use chrono::NaiveDate;
use sha2::{Digest, Sha256};

use super::{
    parse_agent_rows, parse_agent_rows_withholding, parse_all_company_marks,
    parse_company_high_water, parse_company_marks, parse_import_verification_rows,
    parse_voucher_census,
};
use crate::source_draft_xml::parse_source_xml;

/// A rendering longer than this is recorded as its SHA-256 and length, not in full: the
/// protocol crate's record's cap, as the maintainers chose it on bridge#1404.
const FULL_RECORD_CAP: usize = 128 * 1024;
/// Writes every record from today's parsers. Refused where `CI` is set.
const RECORD_VAR: &str = "BRIDGE_RECORD_PARSES";
/// Writes every full rendering into this directory (outside the repository).
const DUMP_VAR: &str = "BRIDGE_PARSE_RECORD_DUMP";
/// Mismatches listed before the rest are only counted.
const MISMATCHES_SHOWN: usize = 20;

/// The companies the reading tests name, each the GUID its capture carries.
const WR2_LAB: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
const REGISTER_LAB: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";
const SHAPE_LAB: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const OUTSTANDINGS_LAB: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";
const STOCK_LAB: &str = "1edb9b05-8d35-4c0c-9959-dce655730463";
const READS_LAB: &str = "de2e15f2-6d42-4715-b6e7-b7a95a68abe8";
/// The import lab of the D3, L1, WA1 and post-span captures (`agent_import_ack_tests.rs`'
/// `D3_GUID`, `agent_import_span_identity_tests.rs`' `COMPANY_GUID`).
const IMPORT_LAB: &str = "17a10910-773c-42c6-bd66-7bba9a392536";
const FOREX_LAB: &str = "b14e9b2d-8a63-4779-804d-25d59eb787eb";
const AARAV: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";
const BILL_ALLOCATION_LAB: &str = "74d7e825-396a-4667-90b2-83f593f06a36";
const GST_CREDIT_PERIODS_LAB: &str = "46faa869-1208-4119-8961-f28db4df3b8e";
const REOPEN_LAB: &str = "ec4454ae-5c4c-4bfa-b3b0-68182a749689";

/// Whether a fixture is evidence of what Tally sends.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Source {
    Captured,
    /// Authored or derived: recorded because an unusual shape is exactly what a parser change
    /// must be heard on, never as evidence of Tally's behaviour.
    Synthetic,
}

struct Row {
    /// Path from the protocol crate's `tests/fixtures/`.
    fixture: &'static str,
    source: Source,
    /// The parser and a short name for its fixed arguments.
    parser: &'static str,
    parse: fn(&[u8], &str) -> String,
}

fn app_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixtures_dir() -> PathBuf {
    app_dir().join("crates/bridge-tally-protocol/tests/fixtures")
}

fn records_dir() -> PathBuf {
    app_dir().join("tests").join("parse_record")
}

/// The fixture's text, decoded as the protocol crate decodes a response (and as its record does):
/// UTF-16LE when its second byte is zero (with or without a BOM), UTF-8 otherwise.
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

/// Whether an error string is one of this crate's codes (`agent_read_protocol_invalid`, ...). A
/// few parser paths pass an error's own text on; that text may be the XML library's, which a new
/// version rewords, so it is recorded as one stable class instead.
fn is_code(error: &str) -> bool {
    error.starts_with(|c: char| c.is_ascii_lowercase())
        && error
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// A result whose error is a string: the value's pretty `Debug`, the error's code, or the stable
/// class for any other error text.
fn coded<T: Debug>(result: Result<T, String>) -> String {
    match result {
        Ok(value) => format!("Ok({value:#?})"),
        Err(code) if is_code(&code) => format!("Err({code:?})"),
        Err(_) => "Err(xml library error)".to_string(),
    }
}

fn day(yyyymmdd: &str) -> NaiveDate {
    NaiveDate::parse_from_str(yyyymmdd, "%Y%m%d").expect("a date copied from a capture")
}

fn agent_rows(text: &str, guid: &str) -> String {
    coded(parse_agent_rows(text, guid))
}

fn withholding(text: &str, guid: &str) -> String {
    coded(parse_agent_rows_withholding(text, guid))
}

fn verification(text: &str, guid: &str) -> String {
    coded(parse_import_verification_rows(text, guid))
}

/// A one-day census, as the window and census reads ask for it, with no ALTERID span.
fn census(text: &str, on: &str) -> String {
    coded(parse_voucher_census(text, (day(on), day(on)), None))
}

fn high_water(text: &str, guid: &str) -> String {
    coded(parse_company_high_water(text, guid))
}

fn marks(text: &str, guid: &str) -> String {
    coded(parse_company_marks(text, guid))
}

fn all_marks(text: &str) -> String {
    coded(parse_all_company_marks(text))
}

/// A company high-water capture: its three parsers, with the company its test reads.
macro_rules! high_water_rows {
    ($fixture:literal, $guid:ident) => {
        [
            Row {
                fixture: $fixture,
                source: Source::Captured,
                parser: concat!("parse_company_high_water(", stringify!($guid), ")"),
                parse: |_, t| high_water(t, $guid),
            },
            Row {
                fixture: $fixture,
                source: Source::Captured,
                parser: concat!("parse_company_marks(", stringify!($guid), ")"),
                parse: |_, t| marks(t, $guid),
            },
            Row {
                fixture: $fixture,
                source: Source::Captured,
                parser: "parse_all_company_marks",
                parse: |_, t| all_marks(t),
            },
        ]
    };
}

macro_rules! row {
    ($fixture:literal, $parser:literal, $parse:expr) => {
        Row {
            fixture: $fixture,
            source: Source::Captured,
            parser: $parser,
            parse: $parse,
        }
    };
}

fn rows() -> Vec<Row> {
    let mut rows = Vec::new();
    // Company high water and marks (`agent_import_ack_tests.rs`,
    // `agent_import_span_identity_tests.rs`, `agent_register_server_tests.rs`; the outstandings,
    // note-day and stock-lab captures by the company each carries).
    rows.extend(high_water_rows!(
        "agent/d3-batch-company-high-water.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/d3-cancelled-company-high-water.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/l1-reentry-company-high-water.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/wa1-payment-company-high-water.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/post-span-company-high-water-after.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/post-span-company-high-water-before.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/post-span-journal-company-high-water-after.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/post-span-journal-company-high-water-before.utf16le.xml",
        IMPORT_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/native-outstandings-detail-high-water.utf16le.xml",
        OUTSTANDINGS_LAB
    ));
    rows.extend(high_water_rows!(
        "agent/register-e2e/native-register-e2e-marks.utf16le.xml",
        REGISTER_LAB
    ));
    rows.extend(high_water_rows!(
        "note-days/shape_lab_company_marks_response.utf16le.xml",
        SHAPE_LAB
    ));
    rows.extend(high_water_rows!(
        "stock-lab-day/stock_lab_taxed_day_company_marks_response.utf16le.xml",
        STOCK_LAB
    ));
    // Every loaded company's marks (`agent_import_post_location_tests.rs`).
    rows.push(row!(
        "agent/native-company-book-extents.utf16le.xml",
        "parse_all_company_marks",
        |_, t| all_marks(t)
    ));
    // Voucher censuses, each over the day its capture asked for: the voucher day its provenance
    // gives (D3, L1, WA1, the register end-to-end read), or its paired request's
    // SVFROMDATE/SVTODATE (the note days).
    rows.push(row!(
        "agent/d3-batch-voucher-census.utf16le.xml",
        "parse_voucher_census(20260401)",
        |_, t| census(t, "20260401")
    ));
    rows.push(row!(
        "agent/d3-cancelled-voucher-census.utf16le.xml",
        "parse_voucher_census(20260401)",
        |_, t| census(t, "20260401")
    ));
    rows.push(row!(
        "agent/l1-reentry-voucher-census.utf16le.xml",
        "parse_voucher_census(20260617)",
        |_, t| census(t, "20260617")
    ));
    rows.push(row!(
        "agent/wa1-payment-voucher-census.utf16le.xml",
        "parse_voucher_census(20260403)",
        |_, t| census(t, "20260403")
    ));
    rows.push(row!(
        "agent/register-e2e/native-register-e2e-census.utf16le.xml",
        "parse_voucher_census(20250903)",
        |_, t| census(t, "20250903")
    ));
    rows.push(row!(
        "note-days/cancelled_purchase_day_voucher_census_response.utf16le.xml",
        "parse_voucher_census(20250703)",
        |_, t| census(t, "20250703")
    ));
    rows.push(row!(
        "note-days/credit_note_day_voucher_census_response.utf16le.xml",
        "parse_voucher_census(20250429)",
        |_, t| census(t, "20250429")
    ));
    rows.push(row!(
        "note-days/debit_note_day_voucher_census_response.utf16le.xml",
        "parse_voucher_census(20250428)",
        |_, t| census(t, "20250428")
    ));
    // Import verification and span reads (`agent_import_ack_tests.rs`,
    // `agent_import_index_tests.rs`, `agent_import_span_identity_tests.rs`).
    rows.push(row!(
        "agent/d3-batch-import-verification.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    rows.push(row!(
        "agent/d3-cancelled-import-verification.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    rows.push(row!(
        "agent/l1-reentry-import-verification.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    rows.push(row!(
        "agent/wa1-payment-import-verification.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    rows.push(row!(
        "agent/post-span-alterid-span-read.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    rows.push(row!(
        "agent/post-span-journal-alterid-span-read.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    rows.push(row!(
        "agent/post-span-guid-absent.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    rows.push(row!(
        "agent/post-span-guid-present.utf16le.xml",
        "parse_import_verification_rows(IMPORT_LAB)",
        |_, t| verification(t, IMPORT_LAB)
    ));
    // Agent voucher reads (`agent_voucher_parse_tests.rs`, `agent_register_tests.rs`,
    // `agent_voucher_window_tests.rs`, `agent_bill_trail_tests.rs` and the others that read them).
    rows.push(row!(
        "agent/native-billallocations-wildcard.utf16le.xml",
        "parse_agent_rows(REGISTER_LAB)",
        |_, t| agent_rows(t, REGISTER_LAB)
    ));
    rows.push(row!(
        "agent/native-entry-wildcard-allocations.utf16le.xml",
        "parse_agent_rows(REGISTER_LAB)",
        |_, t| agent_rows(t, REGISTER_LAB)
    ));
    rows.push(row!(
        "agent/native-education-empty-movement-part.utf16le.xml",
        "parse_agent_rows(WR2_LAB)",
        |_, t| agent_rows(t, WR2_LAB)
    ));
    rows.push(row!(
        "agent/native-namespaced-journal.utf16le.xml",
        "parse_agent_rows(WR2_LAB)",
        |_, t| agent_rows(t, WR2_LAB)
    ));
    rows.push(row!(
        "agent/native-three-vouchers.utf16le.xml",
        "parse_agent_rows(WR2_LAB)",
        |_, t| agent_rows(t, WR2_LAB)
    ));
    rows.push(row!(
        "agent/native-register-lab-vouchers-class.utf16le.xml",
        "parse_agent_rows(REGISTER_LAB)",
        |_, t| agent_rows(t, REGISTER_LAB)
    ));
    rows.push(row!(
        "agent/native-vouchers-renamed-purchase-class.utf16le.xml",
        "parse_agent_rows(READS_LAB)",
        |_, t| agent_rows(t, READS_LAB)
    ));
    rows.push(row!(
        "agent/vouchers-reference-date-year-rows.utf16le.xml",
        "parse_agent_rows(SHAPE_LAB)",
        |_, t| agent_rows(t, SHAPE_LAB)
    ));
    rows.push(row!(
        "agent/native-outstandings-detail-vouchers-window-from-20250420.utf16le.xml",
        "parse_agent_rows(OUTSTANDINGS_LAB)",
        |_, t| agent_rows(t, OUTSTANDINGS_LAB)
    ));
    rows.push(row!(
        "agent/native-outstandings-lab-vouchers-window.utf16le.xml",
        "parse_agent_rows(OUTSTANDINGS_LAB)",
        |_, t| agent_rows(t, OUTSTANDINGS_LAB)
    ));
    rows.push(row!(
        "agent/register-e2e/native-register-e2e-window.utf16le.xml",
        "parse_agent_rows(REGISTER_LAB)",
        |_, t| agent_rows(t, REGISTER_LAB)
    ));
    rows.push(row!(
        "note-days/cancelled_purchase_day_voucher_window_response.utf16le.xml",
        "parse_agent_rows(SHAPE_LAB)",
        |_, t| agent_rows(t, SHAPE_LAB)
    ));
    rows.push(row!(
        "note-days/credit_note_day_voucher_window_response.utf16le.xml",
        "parse_agent_rows(SHAPE_LAB)",
        |_, t| agent_rows(t, SHAPE_LAB)
    ));
    rows.push(row!(
        "note-days/debit_note_day_voucher_window_response.utf16le.xml",
        "parse_agent_rows(SHAPE_LAB)",
        |_, t| agent_rows(t, SHAPE_LAB)
    ));
    rows.push(row!(
        "sales-day/register_window_sales_day_live.utf16le.xml",
        "parse_agent_rows(STOCK_LAB)",
        |_, t| agent_rows(t, STOCK_LAB)
    ));
    rows.push(row!(
        "sales-day/register_window_taxed_sales_day_live.utf16le.xml",
        "parse_agent_rows(STOCK_LAB)",
        |_, t| agent_rows(t, STOCK_LAB)
    ));
    rows.push(row!(
        "stock-lab-day/stock_lab_taxed_day_voucher_window_response.utf16le.xml",
        "parse_agent_rows(STOCK_LAB)",
        |_, t| agent_rows(t, STOCK_LAB)
    ));
    rows.push(row!(
        "unit_a_optional_voucher_live.xml",
        "parse_agent_rows(AARAV)",
        |_, t| agent_rows(t, AARAV)
    ));
    rows.push(row!(
        "vouchers_agst_ref_reopen_live.utf16le.xml",
        "parse_agent_rows(BILL_ALLOCATION_LAB)",
        |_, t| agent_rows(t, BILL_ALLOCATION_LAB)
    ));
    rows.push(row!(
        "vouchers_gst_credit_periods_live.utf16le.xml",
        "parse_agent_rows(GST_CREDIT_PERIODS_LAB)",
        |_, t| agent_rows(t, GST_CREDIT_PERIODS_LAB)
    ));
    rows.push(row!(
        "vouchers_settle_then_reopen_live.utf16le.xml",
        "parse_agent_rows(REOPEN_LAB)",
        |_, t| agent_rows(t, REOPEN_LAB)
    ));
    // Foreign-currency reads, which withhold what they cannot read (`agent_voucher_parse_tests.rs`).
    rows.push(row!(
        "agent/vouchers-forex-bill-allocation-20260915.utf16le.xml",
        "parse_agent_rows_withholding(FOREX_LAB)",
        |_, t| withholding(t, FOREX_LAB)
    ));
    rows.push(row!(
        "agent/vouchers-forex-composite-20260915.utf16le.xml",
        "parse_agent_rows_withholding(FOREX_LAB)",
        |_, t| withholding(t, FOREX_LAB)
    ));
    // A synthetic voucher-import file, read as bytes (`source_draft_xml_tests.rs`).
    rows.push(Row {
        fixture: "voucher_import_candidate_structure_derived.xml",
        source: Source::Synthetic,
        parser: "parse_source_xml",
        parse: |bytes, _| match parse_source_xml(bytes, "voucher-import-candidate.xml".into()) {
            Ok(value) => format!("Ok({value:#?})"),
            Err(error) => format!("Err({error:?})"),
        },
    });
    rows
}

fn record_stem(row: &Row) -> String {
    let fixture = row.fixture.replace('/', "__");
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
    let bytes = fs::read(fixtures_dir().join(row.fixture))
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

/// The first line at which `want` and `got` differ, each side cut to 160 characters.
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
    let repository = app_dir()
        .join("..")
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
    for row in &rows() {
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
    // The directory is exactly the table: a file no row writes is removed when recording and
    // reported otherwise.
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

/// An error string is a code only when it is a lower-case snake-case word; any other text, which
/// a library may have written, is the stable class. A rendering of exactly the cap is kept in
/// full and one byte more is hashed.
#[test]
fn codes_the_class_and_the_cap_are_exact() {
    assert_eq!(
        coded::<()>(Err("agent_read_protocol_invalid".to_string())),
        "Err(\"agent_read_protocol_invalid\")"
    );
    for text in [
        "ill-formed document",
        "Agent",
        "agent_Read",
        "agent read",
        "",
    ] {
        assert_eq!(
            coded::<()>(Err(text.to_string())),
            "Err(xml library error)",
            "{text:?}"
        );
    }
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
        first_difference(&"y".repeat(200), "z"),
        format!(r#"line 1: want "{}" / got "z""#, "y".repeat(160))
    );
}
