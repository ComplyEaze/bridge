//! What a bank batch's verification reads over a window that also holds two
//! invoices with the same figures. Pure matching cases: the rows are written
//! here and do not claim a live Tally capture. The whole result and the
//! duplicate report's hash are pinned, so a change to how an invoice is matched
//! cannot move anything a Payment, Receipt, Contra or Journal batch reports.
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
