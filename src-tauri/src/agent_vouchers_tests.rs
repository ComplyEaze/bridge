use super::*;

/// Filter rows as `WithheldVoucher::filter_view` shapes them; not Tally output.
fn withheld_rows(count: usize) -> Vec<Value> {
    (0..count)
        .map(|index| {
            json!({
                "guid": format!("guid-{index:04}"), "date": "20260915", "voucher_type": "Sales",
                "voucher_number": index.to_string(), "party": "a party",
                "amounts": [{"ledger": "a ledger"}], WITHHELD_MARKER: WITHHELD_FOREIGN_CURRENCY,
            })
        })
        .collect()
}

#[test]
fn the_withheld_listing_is_capped_and_in_window_order() {
    let listed = listed_withheld(&withheld_rows(MAX_WITHHELD_LISTED + 1), 200_000);
    assert_eq!(listed.len(), MAX_WITHHELD_LISTED);
    assert_eq!(listed[0]["guid"], "guid-0000");
    assert_eq!(
        listed[MAX_WITHHELD_LISTED - 1]["guid"],
        format!("guid-{:04}", MAX_WITHHELD_LISTED - 1)
    );
}

#[test]
fn a_listed_withheld_voucher_names_no_party_ledger_or_amount() {
    let listed = listed_withheld(&withheld_rows(1), 200_000);
    let keys: Vec<&String> = listed[0].as_object().unwrap().keys().collect();
    assert_eq!(
        keys,
        vec!["cause", "date", "guid", "voucher_number", "voucher_type"]
    );
    assert_eq!(listed[0]["cause"], WITHHELD_FOREIGN_CURRENCY);
}

#[test]
fn the_withheld_listing_stays_within_a_quarter_of_the_response_budget() {
    let rows = withheld_rows(MAX_WITHHELD_LISTED);
    let one = withheld_summary(&rows[0]).to_string().len();
    // Room for three entries in a quarter of the budget, and not a fourth.
    let listed = listed_withheld(&rows, 4 * (3 * one + one / 2));
    assert_eq!(listed.len(), 3);
    let serialized: usize = listed.iter().map(|entry| entry.to_string().len()).sum();
    assert!(serialized <= 3 * one + one / 2);
    assert!(listed_withheld(&rows, 0).is_empty());
}

/// The empty book of `COMPANY_MARKS_EMPTY_CAPTURE_PROVENANCE.md`.
const EMPTY_BOOK_GUID: &str = "9b5b6d54-4cdb-4b90-9600-cab45ee70c3a";

/// That book's marks answer, as received from TallyPrime 7.2: `ALTMSTID 210`, no `ALTVCHID`.
fn empty_book_marks_answer() -> String {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/company_marks_empty_company_no_voucher_mark_live.utf16le.xml"
    );
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The snapshot a held window of that book would describe.
fn described(marks: CompanyMarks) -> Value {
    let day = TallyDate::parse("20250401").unwrap();
    VoucherPageSnapshot::new(
        VoucherPageKey {
            company_guid: EMPTY_BOOK_GUID.to_string(),
            from: day.clone(),
            to: day,
            ledger: None,
            selector: None,
            search: None,
            summary: None,
        },
        marks,
        Arc::new(Vec::new()),
        json!({}),
        None,
        None,
        None,
        None,
        None,
    )
    .describe(false)
}

#[test]
fn an_empty_book_has_no_voucher_mark_to_describe() {
    let marks = company_marks(&empty_book_marks_answer(), EMPTY_BOOK_GUID).unwrap();
    // Every comparison still reads the absent mark as 0; only what is shown changes.
    assert_eq!(marks.vouchers(), 0);
    assert_eq!(marks.reported_vouchers(), None);
    assert_eq!(marks.masters(), 210);
    let snapshot = described(marks);
    assert_eq!(snapshot["voucher_alter_id"], Value::Null, "{snapshot}");
    assert_eq!(snapshot["master_alter_id"], 210, "{snapshot}");
}

#[test]
fn a_voucher_mark_of_zero_that_tally_sends_is_described_as_zero() {
    // Edited capture: the same answer with an ALTVCHID of 0 added beside ALTMSTID. Not
    // observed live; it pins that only an absent element is described as null.
    let answer = empty_book_marks_answer();
    let edited = answer.replacen(
        "<ALTMSTID TYPE=\"Number\"> 210</ALTMSTID>",
        "<ALTVCHID TYPE=\"Number\">0</ALTVCHID><ALTMSTID TYPE=\"Number\"> 210</ALTMSTID>",
        1,
    );
    assert_ne!(edited, answer);
    let marks = company_marks(&edited, EMPTY_BOOK_GUID).unwrap();
    assert_eq!(marks.reported_vouchers(), Some(0));
    assert_eq!(marks.vouchers(), 0);
    let snapshot = described(marks);
    assert_eq!(snapshot["voucher_alter_id"], 0, "{snapshot}");
    assert_eq!(snapshot["master_alter_id"], 210, "{snapshot}");
}

#[test]
fn a_voucher_mark_omitted_in_one_read_and_sent_as_zero_in_another_is_the_same_book() {
    // The two reads of one book may disagree only on whether Tally sent the mark. Equality,
    // and so every bracket and held-page check, is over the two marks alone.
    let omitted = CompanyMarks::new(None, 210);
    let zero = CompanyMarks::new(Some(0), 210);
    assert_eq!(omitted, zero);
    assert_ne!(omitted.reported_vouchers(), zero.reported_vouchers());
    assert_ne!(omitted, CompanyMarks::new(Some(1), 210));
    assert_ne!(omitted, CompanyMarks::new(None, 211));
}
