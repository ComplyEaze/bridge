//! Voucher search over captured rows (#1230). The rows are the captured three-voucher window
//! (`native-three-vouchers`) parsed by the production parser; nothing here is a hand-written row.
use super::*;

const CAPTURED_COMPANY_GUID: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn captured_xml() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ))
}

fn captured_rows() -> Vec<Value> {
    parse_agent_rows(&captured_xml(), CAPTURED_COMPANY_GUID).unwrap()
}

fn search(args: Value) -> VoucherSearch {
    VoucherSearch::from_args(&args, Redaction::None)
        .unwrap_or_else(|failure| panic!("refused: {}", failure.code))
        .expect("a criterion was given")
}

fn numbers(rows: &[Value]) -> Vec<&str> {
    rows.iter()
        .map(|row| row["voucher_number"].as_str().unwrap())
        .collect()
}

fn refusal(args: Value) -> String {
    VoucherSearch::from_args(&args, Redaction::None)
        .expect_err("the search is refused")
        .code
}

#[test]
fn no_criterion_is_no_search() {
    assert_eq!(
        VoucherSearch::from_args(&json!({"company_guid": "g"}), Redaction::None).unwrap(),
        None
    );
}

#[test]
fn a_blank_criterion_is_refused_with_its_own_code() {
    for key in [
        "voucher_number",
        "reference",
        "narration_contains",
        "amount",
    ] {
        assert_eq!(refusal(json!({ key: "  \t" })), "search_criterion_empty");
    }
}

#[test]
fn a_narration_term_under_three_characters_is_refused() {
    assert_eq!(
        refusal(json!({"narration_contains": "ab"})),
        "search_narration_too_short"
    );
    // The length is counted after the fold, so padding does not lengthen a term.
    assert_eq!(
        refusal(json!({"narration_contains": "   a      "})),
        "search_narration_too_short"
    );
    assert!(
        VoucherSearch::from_args(&json!({"narration_contains": "abc"}), Redaction::None).is_ok()
    );
}

#[test]
fn an_amount_that_is_not_a_plain_positive_decimal_is_refused() {
    for amount in [
        "-5", "+5", "0", "0.00", "1,00,000", "5.", ".5", "1e3", "abc", "5.5.5",
    ] {
        assert_eq!(
            refusal(json!({ "amount": amount })),
            "search_amount_invalid",
            "{amount}"
        );
    }
    assert!(VoucherSearch::from_args(&json!({"amount": "101.01"}), Redaction::None).is_ok());
}

#[test]
fn an_over_long_term_is_refused() {
    let long = "x".repeat(MAX_SEARCH_TERM_CHARS + 1);
    assert_eq!(
        refusal(json!({"voucher_number": long})),
        "search_criterion_too_long"
    );
    let at_the_bound = "x".repeat(MAX_SEARCH_TERM_CHARS);
    assert!(
        VoucherSearch::from_args(&json!({"voucher_number": at_the_bound}), Redaction::None).is_ok()
    );
}

#[test]
fn a_non_string_criterion_is_an_argument_error_not_a_search() {
    assert_eq!(
        refusal(json!({"voucher_number": 2})),
        "argument_invalid:voucher_number"
    );
}

#[test]
fn a_voucher_number_finds_exactly_that_voucher() {
    let found = search(json!({"voucher_number": " 2 "})).apply(captured_rows());
    assert_eq!(numbers(&found), ["2"]);
    assert_eq!(found[0]["matched"], json!({"voucher_number": true}));
}

#[test]
fn a_voucher_number_is_not_a_prefix_match() {
    assert!(search(json!({"voucher_number": "1"}))
        .apply(captured_rows())
        .iter()
        .all(|row| row["voucher_number"] == "1"));
    assert!(search(json!({"voucher_number": "10"}))
        .apply(captured_rows())
        .is_empty());
}

#[test]
fn a_narration_phrase_matches_ignoring_case_inside_the_narration() {
    let found = search(json!({"narration_contains": "n3-nfc"})).apply(captured_rows());
    assert_eq!(numbers(&found), ["2"]);
    let found = search(json!({"narration_contains": "WR2-N"})).apply(captured_rows());
    assert_eq!(numbers(&found), ["1", "2", "3"]);
    assert_eq!(found[0]["matched"], json!({"narration": true}));
}

#[test]
fn an_amount_matches_either_side_of_the_entry_pair_and_names_the_entries() {
    let found = search(json!({"amount": "102.02"})).apply(captured_rows());
    assert_eq!(numbers(&found), ["2"]);
    assert_eq!(found[0]["matched"], json!({"amount_entries": [0, 1]}));
    // A scale-only difference is the same amount.
    let padded = search(json!({"amount": "102.020"})).apply(captured_rows());
    assert_eq!(numbers(&padded), ["2"]);
    // 102.2 is not 102.02.
    assert!(search(json!({"amount": "102.2"}))
        .apply(captured_rows())
        .is_empty());
}

#[test]
fn criteria_combine_with_and() {
    let both = search(json!({"voucher_number": "2", "amount": "102.02"})).apply(captured_rows());
    assert_eq!(numbers(&both), ["2"]);
    assert_eq!(
        both[0]["matched"],
        json!({"voucher_number": true, "amount_entries": [0, 1]})
    );
    assert!(search(json!({"voucher_number": "2", "amount": "101.01"}))
        .apply(captured_rows())
        .is_empty());
}

#[test]
fn a_reference_matches_only_a_voucher_that_carries_one() {
    // The captured window carries no reference; one is added to the second voucher so the
    // criterion has something to find. This is a derived row, not a live capture.
    let xml = captured_xml().replacen(
        "<VOUCHERNUMBER>2</VOUCHERNUMBER>",
        "<VOUCHERNUMBER>2</VOUCHERNUMBER>\n     <REFERENCE TYPE=\"String\">INV/2026/77</REFERENCE>",
        1,
    );
    assert_ne!(xml, captured_xml());
    let rows = parse_agent_rows(&xml, CAPTURED_COMPANY_GUID).unwrap();
    let found = search(json!({"reference": "inv/2026/77"})).apply(rows.clone());
    assert_eq!(numbers(&found), ["2"]);
    assert_eq!(found[0]["matched"], json!({"reference": true}));
    assert!(search(json!({"reference": "INV/2026/7"}))
        .apply(rows)
        .is_empty());
}

#[test]
fn an_amount_search_keeps_a_withheld_voucher_it_cannot_judge() {
    let xml = window_with_composite_vouchers(1);
    let rows = parse_agent_rows_withholding(&xml, CAPTURED_COMPANY_GUID)
        .unwrap()
        .into_iter()
        .map(VoucherRow::into_filter_row)
        .collect::<Vec<_>>();
    assert!(rows[0].get(WITHHELD_MARKER).is_some());
    let found = search(json!({"amount": "102.02"})).apply(rows.clone());
    // The withheld first voucher stays, undecided; the second matches on its amount.
    assert_eq!(numbers(&found), ["1", "2"]);
    assert_eq!(found[0]["matched"], json!({"amount_undetermined": true}));
    assert_eq!(found[1]["matched"], json!({"amount_entries": [0, 1]}));
    // Its other fields still decide: a number that is not its own removes it.
    let by_number = search(json!({"voucher_number": "3", "amount": "103.03"})).apply(rows);
    assert_eq!(numbers(&by_number), ["3"]);
}

#[test]
fn a_narration_phrase_is_refused_where_narrations_are_withheld_from_the_assistant() {
    let args = json!({"narration_contains": "n3-nfc"});
    assert_eq!(
        VoucherSearch::from_args(&args, Redaction::DropNarration)
            .expect_err("refused")
            .code,
        "search_narration_redacted"
    );
    for redaction in [Redaction::None, Redaction::MaskParties] {
        assert!(VoucherSearch::from_args(&args, redaction)
            .unwrap()
            .is_some());
    }
    // Every other criterion reads no narration, so redaction does not touch it.
    let by_number = json!({"voucher_number": "2", "amount": "102.02"});
    assert!(
        VoucherSearch::from_args(&by_number, Redaction::DropNarration)
            .unwrap()
            .is_some()
    );
}

#[test]
fn a_voucher_number_and_a_reference_match_ignoring_ascii_case_only() {
    let mut rows = captured_rows();
    rows[1]["voucher_number"] = json!("Inv-7");
    rows[1]["reference"] = json!("Ref/AB");
    let by_number = search(json!({"voucher_number": "INV-7"})).apply(rows.clone());
    assert_eq!(numbers(&by_number), ["Inv-7"]);
    let by_reference = search(json!({"reference": "ref/ab"})).apply(rows.clone());
    assert_eq!(numbers(&by_reference), ["Inv-7"]);
    // Only ASCII case folds: a different spelling is a different number.
    assert!(search(json!({"voucher_number": "inv 7"}))
        .apply(rows)
        .is_empty());
}
