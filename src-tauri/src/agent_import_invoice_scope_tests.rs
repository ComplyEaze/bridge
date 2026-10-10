//! The invoice's ledger scope (#1331): which ledgers and parents a Sales
//! invoice's rate read is restricted to, and the proof that an answer to it
//! holds exactly those ledgers.
//!
//! The real-bytes cases use the pilot lab's own answers of 10 Oct 2026
//! (fixtures/agent/pilot-lab/PROVENANCE.md, "The parent-filter round"); the
//! rule cases use small synthetic catalogues and say so.
use super::*;
use crate::agent::agent_import::invoice::wire;
use bridge_tally_protocol::StandardLedgerCatalogV2;

const PILOT: &str = "BRIDGE PILOT LAB";
const PILOT_GUID: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

/// The ledgers of the lab invoice `BP/26-27/0010`: the customer, two sales
/// ledgers, the two tax ledgers and the round off.
const LEGS: [&str; 6] = [
    "BRIDGE Walk-in",
    "BRIDGE Svc 998313 5%",
    "BRIDGE Svc 998314 5%",
    "BRIDGE CGST 2.5%",
    "BRIDGE SGST 2.5%",
    "BRIDGE Round Off",
];

const CATALOGUE: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-catalogue-v2-10oct.utf16le.xml");
const SCOPED_REQUEST: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-scoped.request.utf16le.xml");
const SCOPED_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-scoped.utf16le.xml");
const WHOLE_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-whole-10oct.utf16le.xml");
const CASE_CHANGED_REQUEST: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-scoped-case-changed.request.utf16le.xml");
const CASE_CHANGED_ANSWER: &[u8] = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-scoped-case-changed.utf16le.xml");

const WINDOW: (&str, &str) = ("20260401", "20260801");

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
    wire::parse_ledger_rates_and_rows(&captured(answer), PILOT_GUID, &LEGS)
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

fn parents_of(scope: &InvoiceLedgerScope) -> Vec<String> {
    scope
        .part()
        .parents()
        .iter()
        .map(|parent| parent.as_catalogue_text().to_string())
        .collect()
}

// ---- The pilot lab's own bytes ----

/// The scope the lab invoice's legs make is the four parents, and the request
/// it renders is, byte for byte, the one the lab answered (the file starts
/// with a byte order mark).
#[test]
fn the_lab_invoices_legs_make_the_request_the_lab_answered() {
    let scope = scope_of(&pilot_catalogue(), &LEGS).unwrap();
    assert_eq!(
        parents_of(&scope),
        [
            "Duties & Taxes",
            "Indirect Expenses",
            "Sales Accounts",
            "Sundry Debtors"
        ]
    );
    let rendered =
        wire::render_ledger_rates_request_for_parents(PILOT, WINDOW, scope.part()).unwrap();
    assert_eq!(captured(SCOPED_REQUEST), rendered);
    // Naming a ledger twice, or in another order, is the same scope.
    let mut shuffled = LEGS.to_vec();
    shuffled.reverse();
    shuffled.push("BRIDGE Walk-in");
    let again = scope_of(&pilot_catalogue(), &shuffled).unwrap();
    assert_eq!(
        wire::render_ledger_rates_request_for_parents(PILOT, WINDOW, again.part()).unwrap(),
        rendered
    );
}

/// Tally's answer to that request holds exactly the catalogue's 11 ledgers
/// under the four parents, and the scope accepts it.
#[test]
fn the_answer_the_lab_gave_is_exactly_the_scopes_ledgers() {
    let scope = scope_of(&pilot_catalogue(), &LEGS).unwrap();
    let rows = listed(SCOPED_ANSWER);
    assert_eq!(rows.len(), 11);
    assert_eq!(scope.part().ledger_count(), 11);
    assert_eq!(scope.prove(&rows), Ok(()));
    let (rates, _) =
        wire::parse_ledger_rates_and_rows(&captured(SCOPED_ANSWER), PILOT_GUID, &LEGS).unwrap();
    assert_eq!(
        rates.len(),
        LEGS.len(),
        "every named ledger has its rate row"
    );
}

/// The whole book's answer (15 rows) to a scoped request is a filter that was
/// not applied: refused on the count, as is the answer to the case-changed
/// filter (4 rows). The scope never accepts an answer for a request it did not
/// make.
#[test]
fn an_answer_that_is_not_the_scopes_is_refused_on_its_rows() {
    let scope = scope_of(&pilot_catalogue(), &LEGS).unwrap();
    for (answer, label) in [
        (WHOLE_ANSWER, "whole book"),
        (CASE_CHANGED_ANSWER, "one parent"),
    ] {
        assert_eq!(
            scope.prove(&listed(answer)),
            Err(refuse_value(
                "invoice_ledger_rates_rows_differ",
                "parent_part_row_count_differs"
            )),
            "{label}"
        );
    }
}

/// `$Parent` folds case on this book (reference 11e): the filter spelled
/// `SALES ACCOUNTS` returned the four `Sales Accounts` ledgers, each row
/// carrying the book's own spelling. That is why a scope refuses a book with a
/// parent that differs from a scoped one only in case, and why the proof
/// compares each row's parent exactly.
#[test]
fn a_filter_in_another_case_returned_rows_in_the_books_spelling() {
    let request = captured(CASE_CHANGED_REQUEST);
    assert!(request.contains("$Parent = \"SALES ACCOUNTS\""));
    let rows = listed(CASE_CHANGED_ANSWER);
    assert_eq!(rows.len(), 4);
    assert!(rows
        .iter()
        .all(|row| row.parent.as_deref() == Some("Sales Accounts")));
}

/// One row changed in the lab's real answer is refused under the way it
/// differs: a row lost, a row that is another ledger's, a row under another
/// parent, a row twice, a row with no GUID.
#[test]
fn one_row_of_the_real_answer_changed_is_refused() {
    let scope = scope_of(&pilot_catalogue(), &LEGS).unwrap();
    let differs = |cause: &str| Err(refuse_value("invoice_ledger_rates_rows_differ", cause));
    let rows = listed(SCOPED_ANSWER);

    let mut lost = rows.clone();
    lost.pop();
    assert_eq!(scope.prove(&lost), differs("parent_part_row_count_differs"));

    let mut foreign = rows.clone();
    foreign[0].guid = Some(guid(999));
    assert_eq!(
        scope.prove(&foreign),
        differs("parent_part_row_not_in_catalogue")
    );

    let mut moved = rows.clone();
    moved[0].parent = Some("Sales Accounts".to_string());
    assert_eq!(
        scope.prove(&moved),
        differs("parent_part_row_differs_from_catalogue")
    );

    let mut renamed = rows.clone();
    renamed[0].name = "Renamed".to_string();
    assert_eq!(
        scope.prove(&renamed),
        differs("parent_part_row_differs_from_catalogue")
    );

    let mut doubled = rows.clone();
    doubled[1] = doubled[0].clone();
    assert_eq!(scope.prove(&doubled), differs("parent_part_row_repeated"));

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

/// Only the parents of the named ledgers are read, and the ledgers under them
/// are the count the proof expects: `Other Customer` is read because it shares
/// the customer's parent; `Cash` is not.
#[test]
fn only_the_named_ledgers_parents_are_read() {
    let scope = scope_of(&book(), &["Customer", "Sales", "Output CGST"]).unwrap();
    assert_eq!(
        parents_of(&scope),
        ["Duties & Taxes", "Sales Accounts", "Sundry Debtors"]
    );
    assert_eq!(scope.part().ledger_count(), 4);
    assert_eq!(
        scope.prove(&[
            row(0, "Customer", "Sundry Debtors"),
            row(1, "Other Customer", "Sundry Debtors"),
            row(2, "Sales", "Sales Accounts"),
            row(3, "Output CGST", "Duties & Taxes"),
        ]),
        Ok(())
    );
    // A ledger outside the scope in the answer is not part of it.
    assert_eq!(
        scope.prove(&[
            row(0, "Customer", "Sundry Debtors"),
            row(1, "Other Customer", "Sundry Debtors"),
            row(2, "Sales", "Sales Accounts"),
            row(4, "Cash", "Cash-in-Hand"),
        ]),
        Err(refuse_value(
            "invoice_ledger_rates_rows_differ",
            "parent_part_row_not_in_catalogue"
        ))
    );
}

/// A name is resolved exactly on the catalogue's row spelling: another case, a
/// stray space or a ledger the book does not hold is not a ledger of the book.
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

/// A parent that cannot be written inside a filter (a quote ends the literal)
/// names no read.
#[test]
fn a_parent_that_cannot_be_written_in_a_filter_is_refused() {
    let quoted = synthetic(&[("Sales", "Bad\"Group"), ("Customer", "Sundry Debtors")]);
    assert_eq!(
        scope_of(&quoted, &["Sales"]).err(),
        Some(refuse_ledger("invoice_ledger_parent_unnameable", "Sales"))
    );
    // A parent no named ledger is under does not matter.
    assert!(scope_of(&quoted, &["Customer"]).is_ok());
}

/// `$Parent` folds case, so a filter for `Sales Accounts` would also return
/// the ledgers of a parent spelled `SALES ACCOUNTS`: refused before the read.
/// Naming both spellings is a scope that expects both.
#[test]
fn a_parent_that_differs_only_in_case_from_a_scoped_one_is_refused() {
    let twins = synthetic(&[
        ("Sales", "Sales Accounts"),
        ("Sales Twin", "SALES ACCOUNTS"),
        ("Customer", "Sundry Debtors"),
    ]);
    assert_eq!(
        scope_of(&twins, &["Sales", "Customer"]).err(),
        Some(refuse_value("invoice_scope_parent_folds", "Sales Accounts"))
    );
    assert_eq!(
        scope_of(&twins, &["Sales Twin", "Customer"]).err(),
        Some(refuse_value("invoice_scope_parent_folds", "SALES ACCOUNTS"))
    );
    let both = scope_of(&twins, &["Sales", "Sales Twin"]).unwrap();
    assert_eq!(both.part().ledger_count(), 2);
}

/// More ledgers under the named parents than one read may hold is refused
/// before any request, under the plan's own cause.
#[test]
fn a_scope_too_large_for_one_read_is_refused() {
    let limit = crate::tally::connection::parent_partition_limits().max_ledgers_per_part;
    let names = (0..=limit)
        .map(|index| format!("Ledger {index}"))
        .collect::<Vec<_>>();
    let rows = names
        .iter()
        .map(|name| (name.as_str(), "Sundry Debtors"))
        .collect::<Vec<_>>();
    assert_eq!(
        scope_of(&synthetic(&rows), &["Ledger 0"]).err(),
        Some(refuse_value(
            "invoice_ledger_scope_too_large",
            "parent_over_budget"
        ))
    );
    // One fewer fits.
    assert!(scope_of(&synthetic(&rows[1..]), &["Ledger 1"]).is_ok());
}

/// Every refusal the scope can make has a next step for the caller.
#[test]
fn every_refusal_of_a_scope_has_a_next_step() {
    for code in [
        "invoice_ledger_parent_unnameable",
        "invoice_scope_parent_folds",
        "invoice_ledger_scope_too_large",
        "invoice_ledger_scope_unplannable",
        "invoice_ledger_rates_rows_differ",
    ] {
        assert!(crate::agent::refusal_remediation(code).is_some(), "{code}");
    }
}
