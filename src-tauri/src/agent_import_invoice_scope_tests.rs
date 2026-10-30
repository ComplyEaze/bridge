//! The invoice's ledger scope (#1331): which ledgers a Sales invoice's rate read
//! is restricted to, by GUID, and the proof that an answer to it holds exactly
//! those ledgers.
//!
//! The real-bytes cases use the pilot lab's own answers of 31 Oct 2026
//! (fixtures/agent/pilot-lab/PROVENANCE.md, "The ledger-filter round"); the
//! rule cases use small synthetic catalogues and say so.
use super::*;
use crate::agent::agent_import::invoice::wire;
use bridge_tally_protocol::StandardLedgerCatalogV2;

const PILOT: &str = "BRIDGE PILOT LAB";
const PILOT_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

/// The five ledgers of the lab's Goods 18% invoice: the customer, the sales
/// ledger, the two tax ledgers and the round off.
const FIVE: [&str; 5] = [
    "BRIDGE Walk-in",
    "BRIDGE Goods 18%",
    "BRIDGE CGST 9%",
    "BRIDGE SGST 9%",
    "BRIDGE Round Off",
];

const CATALOGUE: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-ledger-catalogue-v2.utf16le.xml");
const GUID_REQUEST: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-guid.request.utf16le.xml");
const GUID_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-guid.utf16le.xml");
const ONE_ABSENT_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-guid-one-absent.utf16le.xml");
const NONE_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-guid-none.utf16le.xml");
const PARENTS_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-scoped-hsn.utf16le.xml");

/// The window the lab's rate requests carried: the financial year's start and
/// the invoice date.
fn window() -> (bridge_tally_core::TallyDate, bridge_tally_core::TallyDate) {
    (
        bridge_tally_core::TallyDate::parse("20260401").unwrap(),
        bridge_tally_core::TallyDate::parse("20260801").unwrap(),
    )
}

/// A captured file as text, without its byte order mark.
fn captured(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
    .trim_start_matches('\u{feff}')
    .to_string()
}

fn pilot_catalogue() -> StandardLedgerCatalogV2 {
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(
        &captured(CATALOGUE),
        PILOT,
        PILOT_GUID,
    )
    .unwrap()
}

fn listed(answer: &[u8]) -> Vec<ListedLedger> {
    wire::parse_ledger_rates_and_rows(&captured(answer), PILOT_GUID, &FIVE)
        .unwrap()
        .1
}

/// SYNTHETIC: a catalogue of `(name, parent)` rows, each with a GUID of its own.
fn synthetic(rows: &[(&str, &str)]) -> StandardLedgerCatalogV2 {
    let rows = rows
        .iter()
        .enumerate()
        .map(|(index, (name, parent))| {
            format!(
                "<LEDGER NAME=\"{}\" RESERVEDNAME=\"\"><GUID TYPE=\"String\">{}</GUID>\
                 <PARENT TYPE=\"String\">{}</PARENT><ISBILLWISEON TYPE=\"Logical\">No</ISBILLWISEON>\
                 <BRIDGECOMPANYGUID TYPE=\"String\">{PILOT_GUID}</BRIDGECOMPANYGUID>\
                 <BRIDGECOMPANYNAME TYPE=\"String\">{PILOT}</BRIDGECOMPANYNAME></LEDGER>",
                name.replace('&', "&amp;").replace('"', "&quot;"),
                guid(index),
                parent.replace('&', "&amp;").replace('"', "&quot;"),
            )
        })
        .collect::<String>();
    bridge_tally_protocol::parse_standard_ledger_catalog_v2_with_identities(
        &format!("<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION>{rows}</COLLECTION></DATA></BODY></ENVELOPE>"),
        PILOT,
        PILOT_GUID,
    )
    .unwrap()
}

fn guid(index: usize) -> String {
    format!("{PILOT_GUID}-{index:08x}")
}

fn row(index: usize, name: &str, parent: &str) -> ListedLedger {
    ListedLedger {
        name: name.to_string(),
        guid: Some(guid(index)),
        parent: Some(parent.to_string()),
    }
}

fn scope_of(
    catalogue: &StandardLedgerCatalogV2,
    named: &[&str],
) -> Result<InvoiceLedgerScope, InvoiceRefusal> {
    InvoiceLedgerScope::from_catalogue(catalogue.catalog(), named)
}

fn rendered(scope: &InvoiceLedgerScope) -> String {
    wire::render_ledger_rates_request_for_ledgers(PILOT, (&window().0, &window().1), scope.part())
}

// ---- The pilot lab's own bytes ----

/// The five ledgers make, byte for byte, the request the lab answered on 31 Oct
/// 2026 (its file starts with a byte order mark). Naming a ledger twice, or in
/// another order, is the same request.
#[test]
fn the_lab_invoices_ledgers_make_the_request_the_lab_answered() {
    let scope = scope_of(&pilot_catalogue(), &FIVE).unwrap();
    assert_eq!(captured(GUID_REQUEST), rendered(&scope));
    let mut shuffled = FIVE.to_vec();
    shuffled.reverse();
    shuffled.push("BRIDGE Walk-in");
    assert_eq!(
        rendered(&scope_of(&pilot_catalogue(), &shuffled).unwrap()),
        rendered(&scope)
    );
}

/// Tally's answer to that request holds exactly the five ledgers, and the scope
/// accepts it, with every named ledger's rate row.
#[test]
fn the_answer_the_lab_gave_is_exactly_the_scopes_ledgers() {
    let scope = scope_of(&pilot_catalogue(), &FIVE).unwrap();
    let rows = listed(GUID_ANSWER);
    assert_eq!(rows.len(), 5);
    assert_eq!(scope.prove(&rows), Ok(()));
    let (rates, _) =
        wire::parse_ledger_rates_and_rows(&captured(GUID_ANSWER), PILOT_GUID, &FIVE).unwrap();
    assert_eq!(
        rates.len(),
        FIVE.len(),
        "every named ledger has its rate row"
    );
}

/// An answer that is not the scope's rows is refused on its count: the lab's
/// answer that lacks the round off (the request named four of the five and a
/// GUID the book does not hold), and its answer to the four-parent read, which
/// holds every ledger under those parents. The scope that names exactly those
/// four accepts the first.
#[test]
fn an_answer_that_is_not_the_scopes_is_refused_on_its_rows() {
    let scope = scope_of(&pilot_catalogue(), &FIVE).unwrap();
    for (answer, label) in [
        (ONE_ABSENT_ANSWER, "one lacking"),
        (PARENTS_ANSWER, "parents"),
    ] {
        assert_eq!(
            scope.prove(&listed(answer)),
            Err(refuse_value(
                "invoice_ledger_rates_rows_differ",
                "ledger_scope_row_count_differs"
            )),
            "{label}"
        );
    }
    let four = scope_of(&pilot_catalogue(), &FIVE[..4]).unwrap();
    assert_eq!(four.prove(&listed(ONE_ABSENT_ANSWER)), Ok(()));
}

/// The lab's answer to a filter that names no ledger of the book is a complete
/// envelope with no row. The rate read never takes that for "no ledger has a
/// rate": it is a read that failed.
#[test]
fn an_answer_with_no_ledger_is_a_failed_read() {
    assert_eq!(
        wire::parse_ledger_rates_and_rows(&captured(NONE_ANSWER), PILOT_GUID, &FIVE).err(),
        Some("invoice_ledger_rates_listing_empty")
    );
}

/// One row changed in the lab's real answer is refused under the way it
/// differs: a row lost, a row that is another ledger's, a row under another
/// parent, a renamed row, a row twice, a row with no GUID.
#[test]
fn one_row_of_the_real_answer_changed_is_refused() {
    let scope = scope_of(&pilot_catalogue(), &FIVE).unwrap();
    let differs = |cause: &str| Err(refuse_value("invoice_ledger_rates_rows_differ", cause));
    let rows = listed(GUID_ANSWER);

    let mut lost = rows.clone();
    lost.pop();
    assert_eq!(
        scope.prove(&lost),
        differs("ledger_scope_row_count_differs")
    );

    let mut foreign = rows.clone();
    foreign[0].guid = Some(guid(999));
    assert_eq!(scope.prove(&foreign), differs("ledger_scope_row_not_named"));

    let mut moved = rows.clone();
    moved[0].parent = Some("Sales Accounts".to_string());
    assert_eq!(
        scope.prove(&moved),
        differs("ledger_scope_row_differs_from_catalogue")
    );

    let mut padded = rows.clone();
    padded[0].parent = padded[0].parent.as_ref().map(|parent| format!("{parent} "));
    assert_eq!(
        scope.prove(&padded),
        differs("ledger_scope_row_differs_from_catalogue")
    );

    let mut renamed = rows.clone();
    renamed[0].name = "Renamed".to_string();
    assert_eq!(
        scope.prove(&renamed),
        differs("ledger_scope_row_differs_from_catalogue")
    );

    let mut doubled = rows.clone();
    doubled[1] = doubled[0].clone();
    assert_eq!(scope.prove(&doubled), differs("ledger_scope_row_repeated"));

    let mut unnamed = rows;
    unnamed[0].guid = None;
    assert_eq!(scope.prove(&unnamed), differs("row_without_guid"));
}

// ---- The rules, on small synthetic catalogues ----

fn book() -> StandardLedgerCatalogV2 {
    synthetic(&[
        ("Customer", "Sundry Debtors"),
        ("Other Customer", "Sundry Debtors"),
        ("Sales", "Sales Accounts"),
        ("Output CGST", "Duties & Taxes"),
        ("Cash", "Cash-in-Hand"),
    ])
}

/// Only the named ledgers are read: `Other Customer` shares the customer's
/// parent and is not, and the proof expects exactly the three named.
#[test]
fn only_the_named_ledgers_are_read() {
    let scope = scope_of(&book(), &["Customer", "Sales", "Output CGST"]).unwrap();
    let request = rendered(&scope);
    for index in [0, 2, 3] {
        assert!(
            request.contains(&format!("$GUID = \"{}\"", guid(index))),
            "{index}"
        );
    }
    for index in [1, 4] {
        assert!(!request.contains(&guid(index)), "{index}");
    }
    assert_eq!(
        scope.prove(&[
            row(0, "Customer", "Sundry Debtors"),
            row(2, "Sales", "Sales Accounts"),
            row(3, "Output CGST", "Duties & Taxes"),
        ]),
        Ok(())
    );
    assert_eq!(
        scope.prove(&[
            row(0, "Customer", "Sundry Debtors"),
            row(1, "Other Customer", "Sundry Debtors"),
            row(2, "Sales", "Sales Accounts"),
            row(3, "Output CGST", "Duties & Taxes"),
        ]),
        Err(refuse_value(
            "invoice_ledger_rates_rows_differ",
            "ledger_scope_row_count_differs"
        ))
    );
}

/// The plan records each scoped ledger's catalogue parent by GUID, so a ledger
/// regrouped under the same GUID changes what the plan carries though its
/// request does not.
#[test]
fn a_regrouped_ledger_changes_the_parents_but_not_the_request() {
    let before = scope_of(&book(), &["Customer", "Sales"]).unwrap();
    let after = scope_of(
        &synthetic(&[
            ("Customer", "BRIDGE Debtors Sub"),
            ("Other Customer", "Sundry Debtors"),
            ("Sales", "Sales Accounts"),
            ("Output CGST", "Duties & Taxes"),
            ("Cash", "Cash-in-Hand"),
        ]),
        &["Customer", "Sales"],
    )
    .unwrap();
    assert_eq!(rendered(&before), rendered(&after));
    assert_eq!(
        before.parents(),
        [
            (guid(0), "Sundry Debtors".to_string()),
            (guid(2), "Sales Accounts".to_string())
        ]
    );
    assert_ne!(before.parents(), after.parents());
}

/// A parent with a space at its end is compared verbatim: the catalogue's
/// spelling is the one the row must carry.
#[test]
fn a_row_parent_with_a_trailing_space_is_read_verbatim() {
    let scope = scope_of(&synthetic(&[("Sales", "Sales Accounts ")]), &["Sales"]).unwrap();
    assert_eq!(scope.prove(&[row(0, "Sales", "Sales Accounts ")]), Ok(()));
    assert_eq!(
        scope.prove(&[row(0, "Sales", "Sales Accounts")]),
        Err(refuse_value(
            "invoice_ledger_rates_rows_differ",
            "ledger_scope_row_differs_from_catalogue"
        ))
    );
}

#[test]
fn a_name_is_resolved_exactly() {
    for name in ["customer", "Customer ", "Nobody"] {
        assert_eq!(
            scope_of(&book(), &["Sales", name]).err(),
            Some(refuse_ledger("invoice_ledger_not_observed", name)),
            "{name:?}"
        );
    }
}

/// A parent a filter literal could not hold (a quote, a space at either end)
/// was refused while the read named parents. The read now names ledgers by
/// GUID, so such a parent is only compared, verbatim, against the row.
/// (SYNTHETIC: the lab book holds no such group.)
#[test]
fn a_parent_no_filter_could_hold_is_read_and_compared_verbatim() {
    for parent in ["Bad\"Group", " Sundry Debtors", "\u{a0}Sundry Debtors"] {
        let scope = scope_of(&synthetic(&[("Customer", parent)]), &["Customer"]).unwrap();
        assert_eq!(
            scope.prove(&[row(0, "Customer", parent)]),
            Ok(()),
            "{parent:?}"
        );
        assert!(!rendered(&scope).contains(parent), "{parent:?}");
    }
}

#[test]
fn every_refusal_of_a_scope_has_a_next_step() {
    for code in [
        "invoice_ledger_parent_unnameable",
        "invoice_ledger_guid_unnameable",
        "invoice_ledger_scope_unplannable",
        "invoice_ledger_rates_rows_differ",
    ] {
        assert!(crate::agent::refusal_remediation(code).is_some(), "{code}");
    }
}
