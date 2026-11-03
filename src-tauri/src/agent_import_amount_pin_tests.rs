//! What the import path does with the amount text, pinned before the amount is typed (#1872).
//!
//! The renderers write an entry's amount exactly as the batch holds it; the verification
//! fingerprint and the span identity compare it in canonical form. These tests hold each of
//! those to its form today, so the pull request that types the amount shows by leaving them
//! untouched that no request byte and no comparison moved. The three captured batches are
//! re-rendered from their journal line and compared with the file that run saved.
use super::*;

const D3_JOURNAL: &str =
    include_str!("../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-journal.jsonl");
const D3_IMPORT: &[u8] =
    include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/d3-batch-import.xml");
const WA1_JOURNAL: &str =
    include_str!("../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-journal.jsonl");
const WA1_IMPORT: &[u8] =
    include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/wa1-payment-import.xml");
const L1_JOURNAL: &str =
    include_str!("../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-journal.jsonl");
const L1_IMPORT: &[u8] =
    include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-import.xml");

/// The amount forms every build that wrote batch lines could write: the rule of
/// `validate_payload` is the same text in every tagged build from `mcp-preview-0.2.0` to
/// `mcp-v0.7.0` (two decimals, digits only, nonzero, at most 256 bytes, leading zeros kept).
fn released_forms() -> Vec<String> {
    let longest = format!("{}.99", "9".repeat(253));
    assert_eq!(longest.len(), 256);
    [
        "0.01",
        "1.00",
        "10.10",
        "12.50",
        "0012.50",
        "000.01",
        "1234567.89",
    ]
    .into_iter()
    .map(String::from)
    .chain([longest])
    .collect()
}

/// A one-voucher batch whose two entries carry `amount`.
fn batch_with_amount(amount: &str) -> ImportPayload<String> {
    serde_json::from_value(json!({"company_guid":GUID,"vouchers":[
        {"bridge_txn_id":"txn-001","date":"2026-09-01","voucher_type":"Journal",
         "entries":[{"ledger":"Expense","amount":amount,"side":"Dr"},
                    {"ledger":"Bank","amount":amount,"side":"Cr"}]}]}))
    .expect("a batch")
}

fn voucher_with_amount(voucher_type: &str, amount: &str) -> ImportVoucher {
    let voucher: ImportVoucher<String> = serde_json::from_value(json!({
        "bridge_txn_id":"txn-001","date":"2026-09-01","voucher_type":voucher_type,
        "entries":[{"ledger":"Expense","amount":amount,"side":"Dr"},
                   {"ledger":"Bank","amount":amount,"side":"Cr"}]}))
    .expect("voucher");
    let date = normalized_date(&voucher.date).unwrap();
    let entries = voucher.parsed_entries().expect("an import amount");
    voucher.admitted(date, entries)
}

/// The first byte at which two texts differ, or `None` when equal.
fn first_difference(left: &[u8], right: &[u8]) -> Option<usize> {
    left.iter()
        .zip(right)
        .position(|(a, b)| a != b)
        .or((left.len() != right.len()).then(|| left.len().min(right.len())))
}

/// The batch line of a captured journal, re-rendered by today's renderer, is the file that run
/// saved: same bytes, and the `sha256` the line recorded.
fn assert_reproduces_its_saved_file(journal: &str, saved: &[u8], batch_id: &str) {
    let first = journal.lines().next().expect("a journal line");
    let line: ImportLedgerLine = serde_json::from_str(first).expect("the captured batch line");
    assert_eq!(line.batch_id, batch_id);
    for entry in line.vouchers.iter().flat_map(|voucher| &voucher.entries) {
        // The captured runs carried only whole amounts with two zero decimals, which is why the
        // edge forms below are pinned on their own.
        assert!(
            entry.amount.as_str().ends_with(".00")
                && entry.amount.as_str()[..entry.amount.as_str().len() - 3]
                    .bytes()
                    .all(|byte| byte.is_ascii_digit()),
            "{}",
            entry.amount
        );
    }
    let company = line.company.as_ref().expect("the captured company");
    let rendered = render_import_xml(&company.name, &line.vouchers, line.identity_batch_id());
    assert_eq!(
        first_difference(rendered.as_bytes(), saved),
        None,
        "rendered {} bytes, saved {} bytes",
        rendered.len(),
        saved.len()
    );
    assert_eq!(sha256_hex(rendered.as_bytes()), line.sha256);
}

#[test]
fn the_d3_batch_re_renders_to_its_saved_file() {
    assert_eq!(D3_IMPORT.len(), 31_052);
    assert_reproduces_its_saved_file(
        D3_JOURNAL,
        D3_IMPORT,
        "bridge-1e5b2cd7-f5c6-4d51-adc9-53a4dfa370bb",
    );
}

#[test]
fn the_wa1_payment_re_renders_to_its_saved_file() {
    assert_eq!(WA1_IMPORT.len(), 1_037);
    assert_reproduces_its_saved_file(
        WA1_JOURNAL,
        WA1_IMPORT,
        "bridge-78ce4328-9c2a-4827-8584-821d6d656b40",
    );
}

#[test]
fn the_l1_reentry_batch_re_renders_to_its_saved_file() {
    assert_reproduces_its_saved_file(
        L1_JOURNAL,
        L1_IMPORT,
        "bridge-4e4af679-b48b-48ac-b9ca-71f3ea6a507d",
    );
}

/// Sign-rule site 1. A debit is the lexeme behind a `-`, a credit is the lexeme: nothing is
/// parsed or normalised on the way, so `0012.50` stays `0012.50` and the longest amount stays
/// 256 bytes.
#[test]
fn the_voucher_renderer_writes_each_edge_amount_as_written() {
    for amount in ["0.01", "0012.50"]
        .map(String::from)
        .into_iter()
        .chain([format!("{}.99", "9".repeat(253))])
    {
        for voucher_type in ["Journal", "Payment"] {
            let xml = render_import_xml(
                "Synthetic Book",
                &[voucher_with_amount(voucher_type, &amount)],
                "batch-render",
            );
            let debit =
                format!("<ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-{amount}</AMOUNT>");
            let credit =
                format!("<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>{amount}</AMOUNT>");
            assert_eq!(xml.matches(&debit).count(), 1, "{voucher_type} {amount}");
            assert_eq!(xml.matches(&credit).count(), 1, "{voucher_type} {amount}");
        }
    }
}

/// Sign-rule site 3, called on a voucher as it is given: the fingerprint keeps that voucher's
/// text. Its callers pass a canonical copy where they compare with the book (see the identity
/// and number-identity tests), which is how `0012.50` and `12.50` come to match.
#[test]
fn the_fingerprint_keeps_the_amount_it_is_given() {
    assert_eq!(
        expected_entry_fingerprint(&voucher_with_amount("Journal", "0012.50")),
        ["Bank|0012.50|No", "Expense|-0012.50|Yes"]
    );
    assert_eq!(
        expected_entry_fingerprint(&voucher_with_amount("Journal", "12.50")),
        ["Bank|12.50|No", "Expense|-12.50|Yes"]
    );
}

/// What a journal line holding every released form does: `validate_payload` admits it, the line
/// reads, writes back the same text, and renders it as written.
#[test]
fn every_amount_form_a_released_build_wrote_loads_and_is_written_back_as_it_was() {
    let first: Value = serde_json::from_str(WA1_JOURNAL.lines().next().unwrap()).unwrap();
    for amount in released_forms() {
        payload_verdict(&batch_with_amount(&amount))
            .unwrap_or_else(|code| panic!("{amount}: {code}"));
        let mut record = first.clone();
        for entry in record["vouchers"][0]["entries"].as_array_mut().unwrap() {
            entry["amount"] = json!(amount);
        }
        let line: ImportLedgerLine = serde_json::from_value(record).expect("the line loads");
        let written = serde_json::to_value(&line).unwrap();
        for entry in written["vouchers"][0]["entries"].as_array().unwrap() {
            assert_eq!(entry["amount"], json!(amount));
        }
        let company = line.company.as_ref().expect("the captured company");
        let xml = render_import_xml(&company.name, &line.vouchers, line.identity_batch_id());
        assert_eq!(
            xml.matches(&format!("<AMOUNT>-{amount}</AMOUNT>")).count(),
            1,
            "{amount}"
        );
        assert_eq!(
            xml.matches(&format!("<AMOUNT>{amount}</AMOUNT>")).count(),
            1,
            "{amount}"
        );
    }
}

/// The refusals that go with them, by the exact code and through the path that stays.
#[test]
fn an_amount_no_released_build_wrote_is_refused_as_voucher_entry_invalid() {
    let too_long = format!("{}.99", "9".repeat(254));
    assert_eq!(too_long.len(), 257);
    let refused = [
        "",
        "0.00",
        "000.00",
        "1",
        "1.0",
        "1.000",
        "-1.00",
        "+1.00",
        ".50",
        "1.",
        "1,000.00",
        " 1.00",
        "1.00 ",
        "1e2.00",
        "1.5e1",
        "０１.００",
        "1..00",
    ]
    .map(String::from)
    .into_iter()
    .chain([too_long]);
    for amount in refused {
        assert_eq!(
            payload_verdict(&batch_with_amount(&amount)),
            Err("voucher_entry_invalid".to_string()),
            "{amount:?}"
        );
    }
}
