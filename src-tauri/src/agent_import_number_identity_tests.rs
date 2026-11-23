//! How an invoice is recognised in the book: by its number (owner decision of
//! 7 Oct 2026, ADR 0004), where every other voucher is recognised by its date,
//! type and entries. Pure matching cases: the rows are written here and do not
//! claim a live Tally capture. A bank batch's whole result and the duplicate
//! report's hashes over a window that holds same-figures invoices are pinned,
//! so how an invoice is matched cannot move anything a Payment, Receipt,
//! Contra or Journal batch reports.
use super::*;

fn row(
    suffix: u32,
    voucher_type: &str,
    number: Option<&str>,
    entries: &[(&str, &str, &str)],
) -> ReadVoucher {
    ReadVoucher {
        remote_id: None,
        guid: Some(format!("{GUID}-{suffix:08x}")),
        alter_id: Some(u64::from(suffix)),
        date: bridge_tally_core::TallyDate::parse("20260311").unwrap(),
        voucher_type: Some(voucher_type.into()),
        narration: None,
        voucher_number: number.map(str::to_string),
        master_id: Some(suffix.to_string()),
        cancelled: Some(false),
        optional: Some(false),
        effective_date: Some(bridge_tally_core::TallyDate::parse("20260311").unwrap()),
        entries: entries
            .iter()
            .map(|(ledger, amount, positive)| ReadEntry {
                ledger: (*ledger).into(),
                amount: (*amount).into(),
                is_deemed_positive: (*positive).into(),
            })
            .collect(),
    }
}

const PAYMENT: &[(&str, &str, &str)] = &[("Rent", "-500.00", "Yes"), ("Bank", "500.00", "No")];
const INVOICE: &[(&str, &str, &str)] = &[
    ("Customer Regular RJ", "-1181.00", "Yes"),
    ("Sales - Goods", "1000.40", "No"),
    ("Output CGST", "90.04", "No"),
    ("Output SGST", "90.04", "No"),
    ("Round Off", "0.52", "No"),
];

/// Two Payments with the same figures, and two invoices of one type with the
/// same figures and different numbers.
fn window() -> ImportReadSource {
    ImportReadSource::admit(vec![
        row(0x31, "Payment", Some("7"), PAYMENT),
        row(0x32, "Payment", Some("8"), PAYMENT),
        row(0x3d, "Sales Invoice", Some("INV/26-27/011"), INVOICE),
        row(0x3e, "Sales Invoice", Some("INV/26-27/002"), INVOICE),
    ])
    .unwrap()
}

/// A saved batch of one Payment dated 11-Mar-2026.
fn payment_batch(debit: &str, amount: &str) -> ImportLedgerLine {
    serde_json::from_value(json!({
        "batch_id": sentinel_batch_id(0xa),
        "identity_scheme":"batch_v1",
        "company_guid":GUID,
        "company":{"name":"Lab", "guid":GUID, "company_number":"100001", "books_from":"20250401"},
        "txn_ids":["p-1"],
        "date_from":"20260311", "date_to":"20260311",
        "sha256":"0000000000000000000000000000000000000000000000000000000000000000",
        "built_at":"2026-03-11T00:00:00.000Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water", "value":40, "master_value":10},
        "vouchers":[{"bridge_txn_id":"p-1", "date":"20260311", "voucher_type":"Payment",
            "narration":null, "reference":null, "voucher_number":null,
            "entries":[{"ledger":debit, "amount":amount, "side":"Dr"},
                {"ledger":"Bank", "amount":amount, "side":"Cr"}]}]
    }))
    .unwrap()
}

/// The duplicate report's hash of the two Payments, and of the two invoices as
/// a bank batch's verification reports them: by date, type and entries.
const PAYMENT_PAIR: &str = "b02463e73435fbf5d99596cbfab38688c8d529a2afe49a7191ef3800e40ba1b6";
const INVOICE_PAIR: &str = "c5fae948c9774d5890275af7150ba97f101de124cad5205c779f86e5d102265b";

fn pair(hash: &str, first: u32, second: u32) -> Value {
    json!({"fingerprint_sha256":hash, "kind":"accounting_fingerprint", "remote_ids":[],
        "voucher_ids":[format!("guid:{GUID}-{first:08x}"), format!("guid:{GUID}-{second:08x}")]})
}

fn counts(key: &str) -> Value {
    let mut counts = json!({"bound_not_in_window":0, "cancelled_with_effective_copy":0,
        "duplicate_fingerprint":0, "matching_content_observed":0, "not_attributable":0,
        "not_found":0, "posted_divergent":0, "posted_not_effective":0, "posted_verified":0});
    counts[key] = json!(1);
    counts
}

#[test]
fn a_bank_batch_reads_a_window_with_same_figures_invoices_as_it_always_has() {
    // A Payment nothing in the window matches: absent, and both pairs are
    // listed as the window's own duplicates.
    assert_eq!(
        verify_batch(
            &payment_batch("Salary", "750.00"),
            &window(),
            Attribution::Tag
        )
        .unwrap(),
        json!({"ambiguous_within_batch":[], "counts":counts("not_found"), "duplicates":[],
            "unrelated_duplicates_in_window":[pair(PAYMENT_PAIR, 0x31, 0x32), pair(INVOICE_PAIR, 0x3d, 0x3e)],
            "vouchers":[{"bridge_txn_id":"p-1", "status":"not_found"}]})
    );
    // A Payment with the figures of the two in the window matches both, by tag
    // attribution and as an unbound native post alike; the invoices stay the
    // window's own pair.
    let twins = json!({"ambiguous_within_batch":[], "counts":counts("duplicate_fingerprint"),
        "duplicates":[pair(PAYMENT_PAIR, 0x31, 0x32)],
        "unrelated_duplicates_in_window":[pair(INVOICE_PAIR, 0x3d, 0x3e)],
        "vouchers":[{"bridge_txn_id":"p-1", "marker":"accounting_fingerprint", "matches":2,
            "status":"duplicate_fingerprint"}]});
    for attribution in [Attribution::Tag, Attribution::Span(None)] {
        assert_eq!(
            verify_batch(&payment_batch("Rent", "500.00"), &window(), attribution).unwrap(),
            twins
        );
    }
}

const KEYED: u32 = 0x3d;
const OTHER_FIGURES: &[(&str, &str, &str)] = &[
    ("Customer Regular RJ", "-118.00", "Yes"),
    ("Sales - Goods", "100.00", "No"),
    ("Output CGST", "9.00", "No"),
    ("Output SGST", "9.00", "No"),
];

/// A saved batch of one invoice with `INVOICE`'s figures, under `number`.
fn invoice_batch(number: &str) -> ImportLedgerLine {
    let mut line = payment_batch("unused", "1.00");
    line.vouchers[0] = serde_json::from_value(json!({
        "bridge_txn_id":"p-1", "date":"20260311", "voucher_type":"Sales",
        "narration":null, "reference":null, "voucher_number":number,
        "invoice":{"voucher_type_name":"Sales Invoice", "place_of_supply":"Rajasthan",
            "round_off_ledger":"Round Off"},
        "entries":[{"ledger":"Customer Regular RJ", "amount":"1181.00", "side":"Dr"},
            {"ledger":"Sales - Goods", "amount":"1000.40", "side":"Cr"},
            {"ledger":"Output CGST", "amount":"90.04", "side":"Cr"},
            {"ledger":"Output SGST", "amount":"90.04", "side":"Cr"},
            {"ledger":"Round Off", "amount":"0.52", "side":"Cr"}]
    }))
    .unwrap();
    line
}

/// What the check before a post reads of invoice `number` over `rows`: the
/// whole result, by tag attribution, with the given identity.
fn before_a_post(number: &str, rows: Vec<ReadVoucher>, identity: InvoiceIdentity) -> Value {
    verify_batch_as(
        &invoice_batch(number),
        &ImportReadSource::admit(rows).unwrap(),
        Attribution::Tag,
        identity,
    )
    .unwrap()
}

fn absent(result: &Value) -> bool {
    result["counts"]["not_found"] == 1
}

#[test]
fn two_invoices_that_differ_only_by_their_number_are_two_documents() {
    let keyed = || vec![row(KEYED, "Sales Invoice", Some("INV/26-27/011"), INVOICE)];
    // The acceptance case: the keyed twin does not stop another number.
    let result = before_a_post("INV/26-27/002", keyed(), InvoiceIdentity::ByNumber);
    assert!(absent(&result), "{result}");
    // It is listed beside the verdict, attributed to nothing.
    assert_eq!(
        result["vouchers"][0]["same_figures_other_number"],
        json!({"count":1, "attribution":"not_established", "entries":[{
            "guid":format!("{GUID}-{KEYED:08x}"), "master_id":KEYED.to_string(),
            "alter_id":KEYED, "voucher_number":"INV/26-27/011"}]})
    );
    // Its own number is refused: the invoice is in the book.
    let same = before_a_post("INV/26-27/011", keyed(), InvoiceIdentity::ByNumber);
    assert!(!absent(&same), "{same}");
    assert_eq!(same["vouchers"][0]["voucher_number"], "INV/26-27/011");
    // Beside an unsettled batch of this machine the figures are enough.
    let strict = before_a_post("INV/26-27/002", keyed(), InvoiceIdentity::ByNumberOrFigures);
    assert!(!absent(&strict), "{strict}");
    assert!(strict["vouchers"][0]
        .get("same_figures_other_number")
        .is_none());
    // A voucher of another type with the figures and the number is not it.
    let other_type = before_a_post(
        "INV/26-27/002",
        vec![row(KEYED, "Sales", Some("INV/26-27/002"), INVOICE)],
        InvoiceIdentity::ByNumberOrFigures,
    );
    assert!(absent(&other_type), "{other_type}");
}

#[test]
fn a_number_in_use_is_this_invoice_whatever_the_figures_and_however_it_is_spelt() {
    // The number under other figures: in the book, and the difference shows.
    let other = before_a_post(
        "INV/26-27/002",
        vec![row(
            KEYED,
            "Sales Invoice",
            Some("INV/26-27/002"),
            OTHER_FIGURES,
        )],
        InvoiceIdentity::ByNumber,
    );
    assert!(!absent(&other), "{other}");
    assert!(
        other["vouchers"][0]["diffs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diff| diff.get("entries").is_some()),
        "{other}"
    );
    // Case and surrounding space do not make another number before a post;
    // the stored spelling is still reported as a difference.
    for stored in ["inv/26-27/002", " INV/26-27/002 "] {
        let result = before_a_post(
            "INV/26-27/002",
            vec![row(KEYED, "Sales Invoice", Some(stored), INVOICE)],
            InvoiceIdentity::ByNumber,
        );
        assert!(!absent(&result), "{stored:?} {result}");
        assert_eq!(
            result["vouchers"][0]["diffs"],
            // The batch attributes by its tag, so it writes its narration with the tag (#1608):
            // a keyed twin that holds none reads as a narration difference beside the number.
            json!(["voucher_number", "narration"]),
            "{stored:?}"
        );
    }
}

#[test]
fn a_voucher_with_no_number_is_this_invoice_when_its_figures_are() {
    for missing in [None, Some(""), Some("  ")] {
        let same = before_a_post(
            "INV/26-27/002",
            vec![row(KEYED, "Sales Invoice", missing, INVOICE)],
            InvoiceIdentity::ByNumber,
        );
        assert!(!absent(&same), "{missing:?} {same}");
        let other = before_a_post(
            "INV/26-27/002",
            vec![row(KEYED, "Sales Invoice", missing, OTHER_FIGURES)],
            InvoiceIdentity::ByNumber,
        );
        assert!(absent(&other), "{missing:?} {other}");
    }
}

#[test]
fn a_posted_invoice_beside_a_keyed_twin_is_no_duplicate_and_beside_its_own_number_is_one() {
    let posted = row(0x3e, "Sales Invoice", Some("INV/26-27/002"), INVOICE);
    let twin = row(KEYED, "Sales Invoice", Some("INV/26-27/011"), INVOICE);
    let read = |rows: Vec<ReadVoucher>| {
        verify_batch_as(
            &invoice_batch("INV/26-27/002"),
            &ImportReadSource::admit(rows).unwrap(),
            Attribution::Span(None),
            InvoiceIdentity::ByNumber,
        )
        .unwrap()
    };
    let beside_a_twin = read(vec![twin, posted.clone()]);
    assert_eq!(beside_a_twin["counts"]["matching_content_observed"], 1);
    assert_eq!(
        beside_a_twin["vouchers"][0]["voucher_number"],
        "INV/26-27/002"
    );
    assert_eq!(beside_a_twin["vouchers"][0]["diffs"], json!([]));
    assert_eq!(beside_a_twin["duplicates"], json!([]));
    assert_eq!(beside_a_twin["unrelated_duplicates_in_window"], json!([]));
    // A second voucher under the same number is a duplicate of the batch.
    let twice = read(vec![
        posted,
        row(0x3f, "Sales Invoice", Some("INV/26-27/002"), INVOICE),
    ]);
    assert_eq!(twice["counts"]["duplicate_fingerprint"], 1, "{twice}");
    assert_eq!(twice["duplicates"].as_array().unwrap().len(), 1, "{twice}");
}

/// In an invoice batch's verification the rows of every other voucher type are
/// still matched by their content: two Payments with the same figures and
/// different numbers stay the window's own duplicate pair, under the hash a
/// bank batch reports for them.
#[test]
fn an_invoice_batch_still_reads_vouchers_of_other_types_by_their_content() {
    let result = verify_batch_as(
        &invoice_batch("INV/26-27/002"),
        &window(),
        Attribution::Tag,
        InvoiceIdentity::ByNumber,
    )
    .unwrap();
    assert_eq!(
        result["unrelated_duplicates_in_window"],
        json!([pair(PAYMENT_PAIR, 0x31, 0x32)]),
        "{result}"
    );
    // The posted invoice is found by its number; its keyed twin is neither a
    // duplicate of the batch nor of the window.
    assert_eq!(result["duplicates"], json!([]), "{result}");
    assert_eq!(result["counts"]["not_found"], 0, "{result}");
}

/// A Journal may carry a number under a type that numbers automatically, where
/// Tally discards it: it is still matched by its content.
#[test]
fn a_journal_that_carries_a_number_is_still_matched_by_its_content() {
    let mut line = payment_batch("Rent", "500.00");
    line.vouchers[0].voucher_type = VoucherType::Journal;
    line.vouchers[0].voucher_number = Some("J-1".into());
    let rows = vec![row(0x31, "Journal", Some("5"), PAYMENT)];
    for identity in [
        InvoiceIdentity::ByNumber,
        InvoiceIdentity::ByNumberOrFigures,
    ] {
        let result = verify_batch_as(
            &line,
            &ImportReadSource::admit(rows.clone()).unwrap(),
            Attribution::Tag,
            identity,
        )
        .unwrap();
        assert_eq!(result["counts"]["matching_content_observed"], 1, "{result}");
        // The batch attributes by its tag, so it writes its narration with the tag (#1608): the
        // row holds none, which reads as a narration difference beside the number.
        assert_eq!(
            result["vouchers"][0]["diffs"],
            json!(["voucher_number", "narration"])
        );
    }
}

#[test]
fn the_identity_beside_an_unsettled_batch_is_the_stricter_one() {
    assert_eq!(InvoiceIdentity::beside(None), InvoiceIdentity::ByNumber);
    assert_eq!(
        InvoiceIdentity::beside(Some("bridge-earlier")),
        InvoiceIdentity::ByNumberOrFigures
    );
}

/// The wiring the post, the check before it and every later verification share
/// (`import_invoice_identity`): a batch with no invoice never takes the
/// journal's lock or reads the journal, so a Payment, Receipt, Contra or
/// Journal post and its readback cannot be refused as busy by this change; an
/// invoice batch is by number alone until this machine holds a sent, unsettled
/// batch with its figures, and then by number or figures.
#[test]
fn the_identity_is_read_from_the_journal_only_for_an_invoice_batch() {
    let directory = tempfile::tempdir().expect("temporary data directory");
    let server = Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let bank = payment_batch("Rent", "500.00");
    let mut earlier = invoice_batch("INV/1");
    earlier.batch_id = sentinel_batch_id(0xb);
    let invoice = invoice_batch("INV/2");
    server.append_import_ledger(&earlier).expect("saved batch");
    // The exclusive lock held by another writer.
    let held = server.lock_import_admission().expect("admission lock");
    assert_eq!(
        server.import_invoice_identity(&bank),
        Ok(InvoiceIdentity::ByNumber)
    );
    assert_eq!(
        server.import_invoice_identity(&invoice),
        Err("import_admission_busy".to_string())
    );
    // The refusal's own read: nothing for a bank batch, the busy lock for an
    // invoice batch (not read as "no unsettled batch").
    assert_eq!(
        server.import_unsettled_invoice_twin_for_refusal(&bank),
        None
    );
    assert_eq!(
        server.import_unsettled_invoice_twin_for_refusal(&invoice),
        Some(Err("import_admission_busy".to_string()))
    );
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&earlier))
        .expect("dispatch intent");
    drop(held);
    assert_eq!(
        server.import_invoice_identity(&bank),
        Ok(InvoiceIdentity::ByNumber)
    );
    assert_eq!(
        server.import_invoice_identity(&invoice),
        Ok(InvoiceIdentity::ByNumberOrFigures)
    );
    assert_eq!(
        server.import_unsettled_invoice_twin_for_refusal(&invoice),
        Some(Ok(Some(earlier.batch_id.clone())))
    );
}

/// The verification of a saved invoice batch (the check before the dialog and
/// every later verification) reads the identity from the journal: a keyed
/// voucher with the invoice's figures and another number leaves the invoice
/// absent until this machine holds a sent, unsettled batch with those figures,
/// and then it does not.
#[test]
fn a_verification_beside_an_unsettled_batch_matches_the_invoice_by_its_figures() {
    let directory = tempfile::tempdir().expect("temporary data directory");
    let server = Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let mut earlier = invoice_batch("INV/26-27/011");
    earlier.batch_id = sentinel_batch_id(0xb);
    let invoice = invoice_batch("INV/26-27/002");
    let keyed = || {
        ImportReadSource::admit(vec![row(
            KEYED,
            "Sales Invoice",
            Some("INV/26-27/011"),
            INVOICE,
        )])
        .unwrap()
    };
    server.append_import_ledger(&earlier).expect("saved batch");
    let verdict = |server: &Server| {
        server
            .verify_batch_by_journal_identity(&invoice, &keyed(), Attribution::Tag)
            .unwrap()
    };
    assert!(absent(&verdict(&server)));
    let held = server.lock_import_admission().expect("admission lock");
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&earlier))
        .expect("dispatch intent");
    drop(held);
    assert!(!absent(&verdict(&server)));
}

/// Sign-rule site 3 at its canonical caller (#1872): an unsettled batch is the twin of an
/// invoice by its figures, whatever lexeme each was written in. A batch saved with `0001181.00`
/// is found by an invoice holding `1181.00`, and a different figure is not found. The figures are
/// compared in canonical form, where the renderers use the lexeme.
#[test]
fn an_unsettled_twin_is_found_whatever_lexeme_its_figures_were_written_in() {
    let directory = tempfile::tempdir().expect("temporary data directory");
    let server = Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let mut earlier = invoice_batch("INV/26-27/011");
    earlier.batch_id = sentinel_batch_id(0xb);
    for entry in &mut earlier.vouchers[0].entries {
        entry.amount = ImportAmount::parse(&format!("000{}", entry.amount)).unwrap();
    }
    let invoice = invoice_batch("INV/26-27/002");
    let mut other = invoice_batch("INV/26-27/003");
    other.vouchers[0].entries[0].amount = ImportAmount::parse("1181.01").unwrap();
    server.append_import_ledger(&earlier).expect("saved batch");
    let held = server.lock_import_admission().expect("admission lock");
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&earlier))
        .expect("dispatch intent");
    drop(held);
    assert_eq!(
        server.import_unsettled_invoice_twin_for_refusal(&invoice),
        Some(Ok(Some(earlier.batch_id.clone())))
    );
    assert_eq!(
        server.import_unsettled_invoice_twin_for_refusal(&other),
        Some(Ok(None))
    );
}

fn verified_post(line: &ImportLedgerLine, observed: ReadVoucher) -> Value {
    let bindings = [span_identity::PostedVoucherIdentity {
        bridge_txn_id: line.vouchers[0].bridge_txn_id.clone(),
        guid: observed.guid.clone().expect("observed GUID"),
        master_id: observed
            .master_id
            .as_deref()
            .expect("observed MasterID")
            .parse()
            .expect("numeric MasterID"),
    }];
    let result = verify_batch(
        line,
        &ImportReadSource::admit(vec![observed]).unwrap(),
        Attribution::Span(Some(&bindings)),
    )
    .unwrap();
    assert_eq!(result["vouchers"][0]["status"], "posted_verified");
    result
}

#[test]
fn a_posted_verified_sales_invoice_carries_the_e_invoice_sent_note() {
    let result = verified_post(
        &invoice_batch("INV/26-27/002"),
        row(KEYED, "Sales Invoice", Some("INV/26-27/002"), INVOICE),
    );
    assert_eq!(
        result["vouchers"][0]["e_invoice_sent"],
        json!({"details":"none","note":"ComplyEaze Bridge sent this invoice without e-invoice details, so it carries no IRN from this post. Where e-invoicing applies to this company and customer, generating the IRN is the CA's act; ComplyEaze Bridge does not generate one."})
    );
}

#[test]
fn a_posted_verified_sales_invoice_sent_with_e_invoice_details_has_no_e_invoice_sent_field() {
    let mut line = invoice_batch("INV/26-27/002");
    line.vouchers[0]
        .invoice
        .as_mut()
        .expect("invoice detail")
        .e_invoice = Some(
        serde_json::from_value(json!({
            "irn": "c3d4e5f6a7b8c9d0c3d4e5f6a7b8c9d0c3d4e5f6a7b8c9d0c3d4e5f6a7b8c9d0",
            "ack_no": "445566",
            "ack_date": "2025-04-03"
        }))
        .unwrap(),
    );
    let result = verified_post(
        &line,
        row(KEYED, "Sales Invoice", Some("INV/26-27/002"), INVOICE),
    );
    assert!(
        result["vouchers"][0].get("e_invoice_sent").is_none(),
        "{result}"
    );
}

#[test]
fn a_sales_invoice_without_native_dispatch_has_no_e_invoice_sent_field() {
    let result = verified_post(
        &invoice_batch("INV/26-27/002"),
        row(KEYED, "Sales Invoice", Some("INV/26-27/002"), INVOICE),
    );
    let mut found_not_sent = result.clone();
    verification::mark_found_not_sent(&mut found_not_sent);
    assert!(
        found_not_sent["vouchers"][0]
            .get("e_invoice_sent")
            .is_none(),
        "mark_found_not_sent alone removes the field"
    );
    verification::mark_verification_names(&mut found_not_sent);
    assert!(found_not_sent["vouchers"][0]
        .get("e_invoice_sent")
        .is_none());

    let mut partly_found = result;
    verification::mark_verification_names(&mut partly_found);
    assert!(partly_found["vouchers"][0].get("e_invoice_sent").is_none());
}

#[test]
fn e_invoice_sent_requires_a_named_sent_dispatch_state() {
    let result = verified_post(
        &invoice_batch("INV/26-27/002"),
        row(KEYED, "Sales Invoice", Some("INV/26-27/002"), INVOICE),
    );
    for state in [
        "posted_verified",
        "reconciliation_required",
        "previous_attempt_reconciled",
    ] {
        let mut dispatched = result.clone();
        dispatched["dispatch"] = json!({"state": state});
        verification::mark_verification_names(&mut dispatched);
        assert!(
            dispatched["vouchers"][0].get("e_invoice_sent").is_some(),
            "{state}"
        );
    }
    for state in ["not_dispatched", "some_later_state"] {
        let mut not_sent = result.clone();
        not_sent["dispatch"] = json!({"state": state});
        verification::mark_verification_names(&mut not_sent);
        assert!(
            not_sent["vouchers"][0].get("e_invoice_sent").is_none(),
            "{state}"
        );
    }
    let mut no_dispatch = result;
    verification::mark_verification_names(&mut no_dispatch);
    assert!(no_dispatch["vouchers"][0].get("e_invoice_sent").is_none());
}

#[test]
fn a_posted_verified_batch_without_sales_has_no_e_invoice_sent_field() {
    let result = verified_post(
        &payment_batch("Rent", "500.00"),
        row(0x31, "Payment", Some("7"), PAYMENT),
    );
    assert!(result["vouchers"]
        .as_array()
        .unwrap()
        .iter()
        .all(|row| { row["status"] == "posted_verified" && row.get("e_invoice_sent").is_none() }));
}

#[test]
fn a_sales_row_that_is_not_posted_verified_has_no_e_invoice_sent_field() {
    // A Sales invoice seen in Tally but not attributed to this post.
    let unattributed = before_a_post(
        "INV/26-27/002",
        vec![row(
            KEYED,
            "Sales Invoice",
            Some("INV/26-27/002"),
            OTHER_FIGURES,
        )],
        InvoiceIdentity::ByNumber,
    );
    assert_eq!(
        unattributed["vouchers"][0]["status"],
        "matching_content_observed"
    );
    assert!(unattributed["vouchers"][0].get("e_invoice_sent").is_none());

    // A confirmed invoice whose later read-back differs loses the field.

    let mut downgraded = verified_post(
        &invoice_batch("INV/26-27/002"),
        row(KEYED, "Sales Invoice", Some("INV/26-27/002"), INVOICE),
    );
    verification::mark_invoice_readback(
        &mut downgraded,
        "p-1",
        &["invoice_fields_differ".to_string()],
        &[],
    );
    assert_eq!(downgraded["vouchers"][0]["status"], "posted_divergent");
    assert!(downgraded["vouchers"][0].get("e_invoice_sent").is_none());
}

#[test]
fn the_e_invoice_sent_note_has_no_threshold_date_or_tally_listing_claim() {
    let note = verification::NO_E_INVOICE_DETAILS_NOTE;
    assert!(!note.chars().any(|character| character.is_ascii_digit()));
    let lower = note.to_ascii_lowercase();
    assert!(!lower.contains("must"));
    assert!(!lower.contains("list"));
    assert!(!lower.contains("gstr"));
}

// #2058: which rows of the window are listed as the invoice's same-figures
// others, and which batches count as holding one invoice.

fn other_entry(suffix: u32, number: &str) -> Value {
    json!({"guid":format!("{GUID}-{suffix:08x}"), "master_id":suffix.to_string(),
        "alter_id":suffix, "voucher_number":number})
}

fn invoice_row(suffix: u32, number: &str) -> ReadVoucher {
    row(suffix, "Sales Invoice", Some(number), INVOICE)
}

fn tagged_invoice_row(number: &str) -> ReadVoucher {
    let line = invoice_batch(number);
    let mut tagged = invoice_row(0x50, number);
    tagged.narration = Some(format!(
        "[BRIDGE:{}]",
        line.attribution_tag(&line.vouchers[0])
    ));
    tagged
}

#[test]
fn a_figures_match_beside_an_unsettled_twin_needs_the_figures() {
    let result = before_a_post(
        "INV/26-27/002",
        vec![row(
            KEYED,
            "Sales Invoice",
            Some("INV/26-27/011"),
            OTHER_FIGURES,
        )],
        InvoiceIdentity::ByNumberOrFigures,
    );
    assert!(absent(&result), "{result}");
}

#[test]
fn the_plain_check_recognises_an_invoice_by_its_number_alone() {
    let result = verify_batch(
        &invoice_batch("INV/26-27/002"),
        &ImportReadSource::admit(vec![invoice_row(KEYED, "INV/26-27/011")]).unwrap(),
        Attribution::Tag,
    )
    .unwrap();
    assert!(absent(&result), "{result}");
}

#[test]
fn a_batch_of_two_invoices_is_matched_by_content_like_any_other() {
    let mut line = invoice_batch("INV/26-27/001");
    let mut second = line.vouchers[0].clone();
    second.bridge_txn_id = "p-2".into();
    second.voucher_number = Some("INV/26-27/002".into());
    line.vouchers.push(second);
    let result = verify_batch(
        &line,
        &ImportReadSource::admit(vec![invoice_row(KEYED, "INV/26-27/002")]).unwrap(),
        Attribution::Tag,
    )
    .unwrap();
    // The one row is the first voucher's, whatever number it carries.
    assert_eq!(
        result["vouchers"][0]["status"], "matching_content_observed",
        "{result}"
    );
    assert_eq!(result["vouchers"][1]["status"], "not_found", "{result}");
}

#[test]
fn the_same_figures_under_other_numbers_are_listed_by_alter_id_and_capped() {
    let rows = vec![
        invoice_row(0x45, "INV/26-27/045"),
        invoice_row(0x43, "INV/26-27/043"),
    ];
    let result = before_a_post("INV/26-27/002", rows, InvoiceIdentity::ByNumber);
    assert_eq!(
        result["vouchers"][0]["same_figures_other_number"],
        json!({"count":2, "attribution":"not_established", "entries":[
            other_entry(0x43, "INV/26-27/043"), other_entry(0x45, "INV/26-27/045")]})
    );
    let many = (0x41..=0x46)
        .map(|suffix| invoice_row(suffix, &format!("INV/26-27/{suffix:03}")))
        .collect::<Vec<_>>();
    let capped = before_a_post("INV/26-27/002", many, InvoiceIdentity::ByNumber);
    let listed = &capped["vouchers"][0]["same_figures_other_number"];
    assert_eq!(listed["count"], 6);
    assert_eq!(listed["entries"].as_array().unwrap().len(), 5);
    assert_eq!(listed["entries"][0], other_entry(0x41, "INV/26-27/065"));
}

#[test]
fn only_an_effective_row_of_the_same_figures_and_another_number_is_listed() {
    let mut cancelled = invoice_row(0x44, "INV/26-27/044");
    cancelled.cancelled = Some(true);
    let mut optional = invoice_row(0x45, "INV/26-27/045");
    optional.optional = Some(true);
    let rows = vec![
        invoice_row(0x43, "INV/26-27/043"),
        cancelled,
        optional,
        row(0x46, "Sales Invoice", Some("INV/26-27/046"), OTHER_FIGURES),
    ];
    let result = before_a_post("INV/26-27/002", rows, InvoiceIdentity::ByNumber);
    assert_eq!(
        result["vouchers"][0]["same_figures_other_number"],
        json!({"count":1, "attribution":"not_established",
            "entries":[other_entry(0x43, "INV/26-27/043")]})
    );
}

#[test]
fn an_invoice_that_is_found_lists_no_others_and_one_with_none_to_list_says_nothing() {
    let others = vec![invoice_row(0x43, "INV/26-27/043")];
    // Found by its tag: verified, so nothing is listed beside it.
    let mut found = others.clone();
    found.push(tagged_invoice_row("INV/26-27/002"));
    let result = before_a_post("INV/26-27/002", found, InvoiceIdentity::ByNumber);
    assert_eq!(
        result["vouchers"][0]["status"], "posted_verified",
        "{result}"
    );
    assert!(result["vouchers"][0]
        .get("same_figures_other_number")
        .is_none());
    // Not found and nothing else with its figures: no empty list.
    let none = before_a_post("INV/26-27/002", vec![], InvoiceIdentity::ByNumber);
    assert!(none["vouchers"][0]
        .get("same_figures_other_number")
        .is_none());
    // Read as optional, not as cancelled: nothing is listed either.
    let mut optional = tagged_invoice_row("INV/26-27/002");
    optional.optional = Some(true);
    let mut with_others = others;
    with_others.push(optional);
    let result = before_a_post("INV/26-27/002", with_others, InvoiceIdentity::ByNumber);
    assert_eq!(
        result["vouchers"][0]["reason"], "voucher_optional",
        "{result}"
    );
    assert!(result["vouchers"][0]
        .get("same_figures_other_number")
        .is_none());
}

#[test]
fn a_cancelled_invoice_lists_the_other_numbers_but_not_a_row_under_its_own_number() {
    let mut cancelled = tagged_invoice_row("INV/26-27/002");
    cancelled.cancelled = Some(true);
    let rows = vec![
        invoice_row(0x43, "INV/26-27/043"),
        invoice_row(0x44, "INV/26-27/002"),
        cancelled,
    ];
    let result = before_a_post("INV/26-27/002", rows, InvoiceIdentity::ByNumber);
    assert_eq!(
        result["vouchers"][0]["reason"], "voucher_cancelled",
        "{result}"
    );
    assert_eq!(
        result["vouchers"][0]["same_figures_other_number"],
        json!({"count":1, "attribution":"not_established",
            "entries":[other_entry(0x43, "INV/26-27/043")]})
    );
}

#[test]
fn a_row_bound_to_another_voucher_is_not_listed_as_an_invoices_other_number() {
    let mut line = invoice_batch("INV/26-27/002");
    let mut second = line.vouchers[0].clone();
    second.bridge_txn_id = "p-2".into();
    second.voucher_number = None;
    line.vouchers.push(second);
    let bindings = vec![span_identity::PostedVoucherIdentity {
        bridge_txn_id: "p-2".into(),
        guid: format!("{GUID}-{:08x}", 0x43),
        master_id: 0x43,
    }];
    let result = verify_batch(
        &line,
        &ImportReadSource::admit(vec![invoice_row(0x43, "INV/26-27/043")]).unwrap(),
        Attribution::Span(Some(&bindings)),
    )
    .unwrap();
    assert!(
        result["vouchers"][0]
            .get("same_figures_other_number")
            .is_none(),
        "{result}"
    );
}
