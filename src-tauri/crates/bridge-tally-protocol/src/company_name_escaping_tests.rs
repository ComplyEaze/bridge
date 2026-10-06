//! Cross-renderer proof that a company name carrying every reserved XML
//! character (bridge#832) survives `escape_text` and comes back out exactly
//! as it went in: the rendered request parses as well-formed XML, and its
//! `SVCURRENTCOMPANY` element decodes to the raw name, byte for byte.
//!
//! One table-driven test per always-compiled renderer family, plus one small
//! test for the non-default `voucher-scan` family, instead of a near-duplicate
//! escaping test in every renderer file. The qualification-only Bills probe may
//! not be referenced outside its own module (the live-read boundary gate), so
//! its escaping stays tested there
//! (`dynamic_values_are_escaped_and_cannot_inject_requests_or_variables`).

use quick_xml::events::Event;

use bridge_tally_primitives::TallyDate;

use crate::native_masters::{render_native_masters_request, NativeMasterKind};
use crate::native_outstandings::{
    render_native_group_snapshot_request, NativeLedgerSnapshotPeriod,
};
use crate::native_statement_reports::{render_native_statement_request, NativeStatementKind};
use crate::native_stock_summary::{
    render_company_inventory_flags_request, render_native_stock_summary_request,
};
use crate::native_trial_balance::render_native_trial_balance_request;
use crate::outstandings_shared::{render_company_book_extent, DateBoundaryProfile};
use crate::xml_read_profiles::compatibility::ledgers_request;
use crate::xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName};

const NAME: &str = "BRIDGE ESCAPE & <LAB> \"Q\" 'A'";

/// The decoded, unescaped text of the first `SVCURRENTCOMPANY` element in
/// `xml`. Panics if `xml` does not parse as well-formed XML, or has no such
/// element -- neither should ever happen for a request built from
/// [`crate::xml_text::escape_text`].
fn decoded_svcurrentcompany(xml: &str) -> String {
    let mut reader = quick_xml::Reader::from_str(xml);
    loop {
        match reader
            .read_event()
            .expect("request must be well-formed XML")
        {
            Event::Start(event) if event.name().as_ref() == b"SVCURRENTCOMPANY" => {
                let raw = reader
                    .read_text(event.name())
                    .expect("SVCURRENTCOMPANY must have a matching close tag");
                let decoded = raw.decode().expect("text must decode as UTF-8");
                return quick_xml::escape::unescape(&decoded)
                    .expect("text must unescape")
                    .into_owned();
            }
            Event::Eof => panic!("request has no SVCURRENTCOMPANY element:\n{xml}"),
            _ => {}
        }
    }
}

#[test]
fn company_name_round_trips_through_xml_parsing_across_renderer_families() {
    let from = TallyDate::parse("20260401").unwrap();
    let to = TallyDate::parse("20260430").unwrap();
    let period =
        NativeLedgerSnapshotPeriod::new(DateBoundaryProfile::ModeAgnostic, from, to).unwrap();
    let company = ValidatedCompanyName::new(NAME).unwrap();
    let requests: Vec<(&str, String)> = vec![
        (
            // The renderer bridge#832 was first filed about (wrongly: it
            // already escaped). Kept here so that claim stays checked.
            "xml_read_profiles::ReadOnlyProfile::StandardLedgerCatalogV1",
            ReadOnlyProfile::StandardLedgerCatalogV1 { company: &company }.render(),
        ),
        (
            "xml_read_profiles::ReadOnlyProfile::StandardLedgerCatalogV2",
            ReadOnlyProfile::StandardLedgerCatalogV2 { company: &company }.render(),
        ),
        (
            "xml_read_profiles::compatibility::ledgers_request",
            ledgers_request(NAME),
        ),
        (
            "native_masters::render_native_masters_request",
            render_native_masters_request(NativeMasterKind::VoucherTypes, NAME),
        ),
        (
            "native_outstandings::render_native_group_snapshot_request",
            render_native_group_snapshot_request(NAME),
        ),
        (
            "native_statement_reports::render_native_statement_request",
            render_native_statement_request(NativeStatementKind::BalanceSheet, NAME, &period),
        ),
        (
            "native_stock_summary::render_company_inventory_flags_request",
            render_company_inventory_flags_request(NAME, "3a6bd6e1-b835-4bff-89dd-8a6af138c346")
                .unwrap(),
        ),
        (
            "native_stock_summary::render_native_stock_summary_request",
            render_native_stock_summary_request(NAME, &period),
        ),
        (
            "native_trial_balance::render_native_trial_balance_request",
            render_native_trial_balance_request(NAME, &period),
        ),
        (
            "outstandings_shared::render_company_book_extent",
            render_company_book_extent(NAME),
        ),
    ];
    for (label, request) in requests {
        assert_eq!(
            decoded_svcurrentcompany(&request),
            NAME,
            "{label} did not round-trip the company name byte for byte"
        );
    }
}

/// The `voucher-scan` outstandings path (`outstandings/request.rs`) is
/// compiled only under its own feature, so it gets its own small test rather
/// than joining the always-on table above.
#[cfg(feature = "voucher-scan")]
#[test]
fn outstandings_request_company_name_round_trips() {
    let request = crate::outstandings::render_ledger_opening_coverage(NAME);
    assert_eq!(decoded_svcurrentcompany(&request), NAME);
}
