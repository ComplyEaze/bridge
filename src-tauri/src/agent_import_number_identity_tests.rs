//! How an invoice is recognised in the book: by its number (owner decision of
//! 7 Oct 2026, ADR 0004), where every other voucher is recognised by its date,
//! type and entries. Pure matching cases: the rows are written here and do not
//! claim a live Tally capture. A bank batch's whole result and the duplicate
//! report's hashes over a window that holds same-figures invoices are pinned,
//! so how an invoice is matched cannot move anything a Payment, Receipt,
//! Contra or Journal batch reports.
use super::*;

const GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";

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
        date: Some("20260311".into()),
        voucher_type: Some(voucher_type.into()),
        narration: None,
        voucher_number: number.map(str::to_string),
        master_id: Some(suffix.to_string()),
        cancelled: Some(false),
        optional: Some(false),
        effective_date: Some("20260311".into()),
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
    ("TG Buyer Regular RJ", "-1457.00", "Yes"),
    ("Sales - Goods", "1234.50", "No"),
    ("Output CGST", "111.11", "No"),
    ("Output SGST", "111.11", "No"),
    ("Round Off", "0.28", "No"),
];

/// Two Payments with the same figures, and two invoices of one type with the
/// same figures and different numbers.
fn window() -> ImportReadSource {
    ImportReadSource::admit(vec![
        row(0x31, "Payment", Some("7"), PAYMENT),
        row(0x32, "Payment", Some("8"), PAYMENT),
        row(0x3d, "Sales Manual", Some("TG/25-26/011"), INVOICE),
        row(0x3e, "Sales Manual", Some("TG/25-26/002"), INVOICE),
    ])
    .unwrap()
}

/// A saved batch of one Payment dated 11-Mar-2026.
fn payment_batch(debit: &str, amount: &str) -> ImportLedgerLine {
    serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-00000000000a",
        "identity_scheme":"batch_v1",
        "company_guid":GUID,
        "company":{"name":"Lab", "guid":GUID, "company_number":"100017", "books_from":"20250401"},
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
const INVOICE_PAIR: &str = "9c90d86d02b9080592f000dd1fa4c2030f90da3e4c3fcb7ccb27b152bc61bf0a";

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
            "unrelated_duplicates_in_window":[pair(INVOICE_PAIR, 0x3d, 0x3e), pair(PAYMENT_PAIR, 0x31, 0x32)],
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
    ("TG Buyer Regular RJ", "-118.00", "Yes"),
    ("Sales - Goods", "100.00", "No"),
    ("Output CGST", "9.00", "No"),
    ("Output SGST", "9.00", "No"),
];

/// A saved batch of one invoice with `INVOICE`'s figures, under `number`.
fn invoice_batch(number: &str) -> ImportLedgerLine {
    let mut line = payment_batch("unused", "1");
    line.vouchers[0] = serde_json::from_value(json!({
        "bridge_txn_id":"p-1", "date":"20260311", "voucher_type":"Sales",
        "narration":null, "reference":null, "voucher_number":number,
        "invoice":{"voucher_type_name":"Sales Manual", "place_of_supply":"Rajasthan",
            "round_off_ledger":"Round Off"},
        "entries":[{"ledger":"TG Buyer Regular RJ", "amount":"1457.00", "side":"Dr"},
            {"ledger":"Sales - Goods", "amount":"1234.50", "side":"Cr"},
            {"ledger":"Output CGST", "amount":"111.11", "side":"Cr"},
            {"ledger":"Output SGST", "amount":"111.11", "side":"Cr"},
            {"ledger":"Round Off", "amount":"0.28", "side":"Cr"}]
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
    let keyed = || vec![row(KEYED, "Sales Manual", Some("TG/25-26/011"), INVOICE)];
    // The acceptance case: the keyed twin does not stop another number.
    let result = before_a_post("TG/25-26/002", keyed(), InvoiceIdentity::ByNumber);
    assert!(absent(&result), "{result}");
    // It is listed beside the verdict, attributed to nothing.
    assert_eq!(
        result["vouchers"][0]["same_figures_other_number"],
        json!({"count":1, "attribution":"not_established", "entries":[{
            "guid":format!("{GUID}-{KEYED:08x}"), "master_id":KEYED.to_string(),
            "alter_id":KEYED, "voucher_number":"TG/25-26/011"}]})
    );
    // Its own number is refused: the invoice is in the book.
    let same = before_a_post("TG/25-26/011", keyed(), InvoiceIdentity::ByNumber);
    assert!(!absent(&same), "{same}");
    assert_eq!(same["vouchers"][0]["voucher_number"], "TG/25-26/011");
    // Beside an unsettled batch of this machine the figures are enough.
    let strict = before_a_post("TG/25-26/002", keyed(), InvoiceIdentity::ByNumberOrFigures);
    assert!(!absent(&strict), "{strict}");
    assert!(strict["vouchers"][0]
        .get("same_figures_other_number")
        .is_none());
    // A voucher of another type with the figures and the number is not it.
    let other_type = before_a_post(
        "TG/25-26/002",
        vec![row(KEYED, "Sales", Some("TG/25-26/002"), INVOICE)],
        InvoiceIdentity::ByNumberOrFigures,
    );
    assert!(absent(&other_type), "{other_type}");
}

#[test]
fn a_number_in_use_is_this_invoice_whatever_the_figures_and_however_it_is_spelt() {
    // The number under other figures: in the book, and the difference shows.
    let other = before_a_post(
        "TG/25-26/002",
        vec![row(
            KEYED,
            "Sales Manual",
            Some("TG/25-26/002"),
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
    for stored in ["tg/25-26/002", " TG/25-26/002 "] {
        let result = before_a_post(
            "TG/25-26/002",
            vec![row(KEYED, "Sales Manual", Some(stored), INVOICE)],
            InvoiceIdentity::ByNumber,
        );
        assert!(!absent(&result), "{stored:?} {result}");
        assert_eq!(
            result["vouchers"][0]["diffs"],
            json!(["voucher_number"]),
            "{stored:?}"
        );
    }
}

#[test]
fn a_voucher_with_no_number_is_this_invoice_when_its_figures_are() {
    for missing in [None, Some(""), Some("  ")] {
        let same = before_a_post(
            "TG/25-26/002",
            vec![row(KEYED, "Sales Manual", missing, INVOICE)],
            InvoiceIdentity::ByNumber,
        );
        assert!(!absent(&same), "{missing:?} {same}");
        let other = before_a_post(
            "TG/25-26/002",
            vec![row(KEYED, "Sales Manual", missing, OTHER_FIGURES)],
            InvoiceIdentity::ByNumber,
        );
        assert!(absent(&other), "{missing:?} {other}");
    }
}

#[test]
fn a_posted_invoice_beside_a_keyed_twin_is_no_duplicate_and_beside_its_own_number_is_one() {
    let posted = row(0x3e, "Sales Manual", Some("TG/25-26/002"), INVOICE);
    let twin = row(KEYED, "Sales Manual", Some("TG/25-26/011"), INVOICE);
    let read = |rows: Vec<ReadVoucher>| {
        verify_batch_as(
            &invoice_batch("TG/25-26/002"),
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
        "TG/25-26/002"
    );
    assert_eq!(beside_a_twin["vouchers"][0]["diffs"], json!([]));
    assert_eq!(beside_a_twin["duplicates"], json!([]));
    assert_eq!(beside_a_twin["unrelated_duplicates_in_window"], json!([]));
    // A second voucher under the same number is a duplicate of the batch.
    let twice = read(vec![
        posted,
        row(0x3f, "Sales Manual", Some("TG/25-26/002"), INVOICE),
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
        &invoice_batch("TG/25-26/002"),
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
        assert_eq!(result["vouchers"][0]["diffs"], json!(["voucher_number"]));
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
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let bank = payment_batch("Rent", "500.00");
    let mut earlier = invoice_batch("INV/1");
    earlier.batch_id = "bridge-00000000-0000-4000-8000-00000000000b".into();
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
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let mut earlier = invoice_batch("TG/25-26/011");
    earlier.batch_id = "bridge-00000000-0000-4000-8000-00000000000b".into();
    let invoice = invoice_batch("TG/25-26/002");
    let keyed = || {
        ImportReadSource::admit(vec![row(
            KEYED,
            "Sales Manual",
            Some("TG/25-26/011"),
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
