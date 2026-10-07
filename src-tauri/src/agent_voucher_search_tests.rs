//! Voucher search over captured rows (#1230). The rows are the captured three-voucher window
//! (`native-three-vouchers`) parsed by the production parser. A test that needs a field the capture
//! lacks (a reference, a mixed-case number, a trailing-zero amount) changes or adds it on a parsed
//! row, or in the captured text; those are derived rows, not live captures.
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
    assert_eq!(found[0]["matched"], json!({}));
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

#[test]
fn a_voucher_number_is_matched_whole_not_as_a_prefix() {
    let mut rows = captured_rows();
    rows[1]["voucher_number"] = json!("Inv-12");
    assert!(search(json!({"voucher_number": "Inv-1"}))
        .apply(rows.clone())
        .is_empty());
    assert!(search(json!({"voucher_number": "inv"}))
        .apply(rows.clone())
        .is_empty());
    assert_eq!(
        numbers(&search(json!({"voucher_number": "INV-12"})).apply(rows)),
        ["Inv-12"]
    );
}

#[test]
fn an_amount_matches_by_value_not_by_the_spelling_tally_stored() {
    let mut rows = captured_rows();
    // Tally wrote 102.20 with its trailing zero; the search 102.2 is the same amount.
    rows[1]["amounts"][0]["amount"] = json!("-102.20");
    rows[1]["amounts"][1]["amount"] = json!("102.20");
    for term in ["102.2", "102.20", "102.200"] {
        let found = search(json!({ "amount": term })).apply(rows.clone());
        assert_eq!(numbers(&found), ["2"], "{term}");
    }
}

#[test]
fn a_voucher_number_the_book_stored_with_spaces_still_matches() {
    let mut rows = captured_rows();
    rows[2]["voucher_number"] = json!(" 7 ");
    assert_eq!(
        numbers(&search(json!({"voucher_number": "7"})).apply(rows)),
        [" 7 "]
    );
}

// ---- The live rows (#1230): the same 67 synthetic-book vouchers and the searches the tool answered
// over them on 6 Oct 2026 (a debug build of master at 4c30f3f9f; see the PROVENANCE note beside the
// fixtures).
#[test]
fn each_live_search_over_the_live_rows_returns_the_vouchers_the_tool_returned_live() {
    let rows: Vec<Value> = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.rows.json"
    ))
    .unwrap();
    let answers: Value = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.live-answers.json"
    ))
    .unwrap();
    let searches = answers["searches"].as_array().unwrap();
    assert_eq!(searches.len(), 5);
    for live in searches {
        let found = search(live["args"].clone()).apply(rows.clone());
        let guids: Vec<&Value> = found.iter().map(|row| &row["guid"]).collect();
        assert_eq!(
            guids,
            live["guids"].as_array().unwrap().iter().collect::<Vec<_>>(),
            "{}",
            live["step"]
        );
        let matched: Vec<&Value> = found.iter().map(|row| &row["matched"]).collect();
        assert_eq!(
            matched,
            live["matched"]
                .as_array()
                .unwrap()
                .iter()
                .collect::<Vec<_>>(),
            "{}",
            live["step"]
        );
        assert_eq!(json!(found.len()), live["total"], "{}", live["step"]);
    }
    // The live run's counts, from the capture: 9 with the reference, 4 numbered 7, 57 with the narration
    // phrase, 1 holding the amount, none for a number no voucher has.
    let totals: Vec<u64> = searches
        .iter()
        .map(|s| s["total"].as_u64().unwrap())
        .collect();
    assert_eq!(totals, vec![9, 4, 57, 1, 0]);
}

#[test]
fn a_live_narration_phrase_is_found_whatever_its_letter_case() {
    // Every live search term was given in the case it is stored in, so this one asks for the live
    // narration phrase in other cases and requires the vouchers the live run returned.
    let rows: Vec<Value> = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.rows.json"
    ))
    .unwrap();
    let answers: Value = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.live-answers.json"
    ))
    .unwrap();
    let live = answers["searches"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["step"] == "s10_narration")
        .unwrap();
    let phrase = live["args"]["narration_contains"].as_str().unwrap();
    assert_eq!(phrase, "SHAPELAB");
    for variant in [
        phrase.to_lowercase(),
        "ShapeLab".to_string(),
        format!("  {phrase}  "),
    ] {
        let found = search(json!({ "narration_contains": variant })).apply(rows.clone());
        let guids: Vec<&Value> = found.iter().map(|row| &row["guid"]).collect();
        assert_eq!(
            guids,
            live["guids"].as_array().unwrap().iter().collect::<Vec<_>>(),
            "{variant:?}"
        );
    }
}

/// The vouchers a criterion selects, written separately from `VoucherSearch::apply`: a number or a
/// reference equal whole ignoring ASCII case and the spaces around it, a narration phrase contained
/// ignoring ASCII case and runs of spaces, an amount equal to the absolute value of any entry. (The live
/// terms are ASCII, so this folding is enough here.)
fn independent_selection<'a>(rows: &'a [Value], args: &Value) -> Vec<&'a Value> {
    let fold = |text: &str| {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase()
    };
    rows.iter()
        .filter(|row| {
            let whole = |key: &str, wanted: &Value| match (row[key].as_str(), wanted.as_str()) {
                (Some(have), Some(want)) => have.trim().eq_ignore_ascii_case(want.trim()),
                (None, Some(_)) => false,
                _ => true,
            };
            let number = whole("voucher_number", &args["voucher_number"]);
            let reference = whole("reference", &args["reference"]);
            let narration = args["narration_contains"].as_str().is_none_or(|phrase| {
                row["narration"]
                    .as_str()
                    .is_some_and(|text| fold(text).contains(&fold(phrase)))
            });
            let amount = args["amount"].as_str().is_none_or(|wanted| {
                let wanted = bridge_tally_core::ExactDecimal::parse(wanted.to_string()).unwrap();
                row["amounts"].as_array().unwrap().iter().any(|entry| {
                    let text = entry["amount"].as_str().unwrap().trim_start_matches('-');
                    bridge_tally_core::ExactDecimal::parse(text.to_string())
                        .unwrap()
                        .numeric_eq(&wanted)
                })
            });
            number && reference && narration && amount
        })
        .collect()
}

#[test]
fn each_live_search_equals_an_independent_selection_over_the_rows() {
    let rows: Vec<Value> = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.rows.json"
    ))
    .unwrap();
    let answers: Value = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.live-answers.json"
    ))
    .unwrap();
    for live in answers["searches"].as_array().unwrap() {
        let wanted: Vec<&Value> = independent_selection(&rows, &live["args"])
            .iter()
            .map(|row| &row["guid"])
            .collect();
        let found = search(live["args"].clone()).apply(rows.clone());
        let got: Vec<&Value> = found.iter().map(|row| &row["guid"]).collect();
        assert_eq!(
            got, wanted,
            "{}: the code under test against the independent selection",
            live["step"]
        );
        assert_eq!(
            wanted,
            live["guids"].as_array().unwrap().iter().collect::<Vec<_>>(),
            "{}: the live answer",
            live["step"]
        );
    }
}

#[test]
fn searches_the_live_answers_do_not_cover_still_equal_the_independent_selection() {
    // Terms chosen so that a wrong rule gives a different answer: a reference that is only a prefix of the
    // live one, the live one in other letter cases, a number written with a leading zero, a narration phrase
    // that fits a single voucher, an amount that is a whole number of the live decimals, and two criteria
    // together. The expected vouchers are the independent selection's, not the code under test's.
    let rows: Vec<Value> = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-shape-lab-fy.rows.json"
    ))
    .unwrap();
    let cases = [
        (json!({"reference": "SHAPELAB-MANUAL"}), 0),
        (json!({"reference": "shapelab-manual-1"}), 9),
        (
            json!({"reference": "SHAPELAB-MANUAL-1", "narration_contains": "SHAPELAB"}),
            8,
        ),
        (json!({"reference": "  SHAPELAB-MANUAL-1 "}), 9),
        (json!({"voucher_number": "07"}), 0),
        (
            json!({"voucher_number": "7", "narration_contains": "SHAPELAB"}),
            3,
        ),
        (
            json!({"reference": "SHAPELAB-MANUAL-1", "narration_contains": "SHAPELAB"}),
            8,
        ),
        (json!({"narration_contains": "T-0401"}), 1),
        (json!({"narration_contains": "t-0401"}), 1),
        (json!({"amount": "10000"}), 7),
        (
            json!({"amount": "10000.00", "narration_contains": "SHAPELAB"}),
            7,
        ),
    ];
    for (args, expected) in cases {
        let wanted = independent_selection(&rows, &args);
        let found = search(args.clone()).apply(rows.clone());
        assert_eq!(
            found.iter().map(|row| &row["guid"]).collect::<Vec<_>>(),
            wanted.iter().map(|row| &row["guid"]).collect::<Vec<_>>(),
            "{args}"
        );
        assert_eq!(found.len(), expected, "{args}: the count on the live rows");
    }
}

// -- #810 slice 2: the suspense tag. The rows are the captured three vouchers; a narration is
// replaced on a parsed row to carry a tag, so the tagged rows are derived, not live captures. The
// marker's UUID has the shape of the captured `[BRIDGE:...]` read-backs (not a version 4 UUID).

const PURPOSE: &str = "Bridge: purpose not confirmed; reclassify";
const MARKER: &str = "[BRIDGE:547cb4ad-9457-8ee7-a1b2-0123456789ab]";

fn suspense_search() -> VoucherSearch {
    search(json!({"suspense_tagged": true, "ledger": "WR2 Sales"}))
}

/// The third captured voucher, which touches "WR2 Sales", with this narration.
fn narrated(narration: &str) -> Vec<Value> {
    let mut row = captured_rows().remove(2);
    row["narration"] = json!(narration);
    vec![row]
}

fn tag_of(narration: &str) -> Option<String> {
    let rows = suspense_search().apply(narrated(narration));
    rows.first()
        .map(|row| row["matched"]["suspense_tag"].as_str().unwrap().to_string())
}

#[test]
fn a_suspense_tag_search_needs_the_ledger_and_true_and_a_visible_narration() {
    assert_eq!(
        refusal(json!({"suspense_tagged": true})),
        "search_suspense_tagged_needs_ledger"
    );
    assert_eq!(
        refusal(json!({"suspense_tagged": true, "ledger": "  "})),
        "search_suspense_tagged_needs_ledger"
    );
    for value in [json!(false), json!("true"), json!(1)] {
        assert_eq!(
            refusal(json!({"suspense_tagged": value, "ledger": "WR2 Sales"})),
            "search_suspense_tagged_invalid"
        );
    }
    let redacted = VoucherSearch::from_args(
        &json!({"suspense_tagged": true, "ledger": "WR2 Sales"}),
        Redaction::DropNarration,
    )
    .expect_err("refused where narrations are withheld");
    assert_eq!(redacted.code, "search_narration_redacted");
    // With a neutral criterion beside it, the search differs from the same search without it.
    assert_ne!(
        search(json!({"suspense_tagged": true, "ledger": "WR2 Sales", "amount": "1"})),
        search(json!({"ledger": "WR2 Sales", "amount": "1"}))
    );
}

#[test]
fn a_narration_that_ends_in_either_tag_is_found_with_its_kind() {
    assert_eq!(
        tag_of(&format!("UPI to X | {PURPOSE}")).as_deref(),
        Some("purpose_not_confirmed")
    );
    // Whatever the tag names must be a ledger of the voucher itself.
    assert_eq!(
        tag_of("UPI | UNIDENTIFIED - reallocate from WR2 Sales").as_deref(),
        Some("unidentified")
    );
    // Trailing whitespace is not text after the tag.
    assert_eq!(
        tag_of(&format!("UPI | {PURPOSE}  ")).as_deref(),
        Some("purpose_not_confirmed")
    );
    // Under the fold the build decided by: case and spacing differ.
    assert_eq!(
        tag_of("UPI | UNIDENTIFIED - reallocate from wr2  sales").as_deref(),
        Some("unidentified")
    );
}

#[test]
fn the_one_marker_a_hand_import_adds_after_the_tag_is_ignored() {
    assert_eq!(
        tag_of(&format!("UPI | {PURPOSE} {MARKER}")).as_deref(),
        Some("purpose_not_confirmed")
    );
    assert_eq!(
        tag_of(&format!(
            "UPI | UNIDENTIFIED - reallocate from WR2 Sales {MARKER} "
        ))
        .as_deref(),
        Some("unidentified")
    );
}

#[test]
fn a_narration_that_does_not_end_in_a_tag_is_not_found() {
    for narration in [
        // The tag's text anywhere but the end: a party or account label.
        format!("{PURPOSE} paid to X"),
        format!("UPI | {PURPOSE} (checked)"),
        format!("UPI | {PURPOSE} {MARKER} later note"),
        // Two markers, or a marker that never closes: not stripped, so the tag is not last.
        format!("UPI | {PURPOSE} {MARKER} {MARKER}"),
        format!("{MARKER} UPI | {PURPOSE} {MARKER}"),
        // A marker with no space before it is not the hand-import marker.
        format!("UPI | {PURPOSE}{MARKER}"),
        format!("UPI | {PURPOSE} [BRIDGE:547cb4ad"),
        // No separator before the tag.
        format!("UPI{PURPOSE}"),
        // A ledger the voucher does not touch; a tag with no ledger at all.
        "UPI | UNIDENTIFIED - reallocate from Suspense".to_string(),
        "UPI | UNIDENTIFIED - reallocate from".to_string(),
        String::new(),
    ] {
        assert_eq!(tag_of(&narration), None, "{narration:?}");
    }
}

#[test]
fn none_of_the_captured_narrations_is_a_suspense_tag() {
    let rows = captured_rows();
    assert_eq!(rows.len(), 3);
    assert!(suspense_search().apply(rows).is_empty());
}

#[test]
fn a_voucher_with_no_narration_is_not_found() {
    let mut row = captured_rows().remove(2);
    row.as_object_mut().unwrap().remove("narration");
    assert!(suspense_search().apply(vec![row]).is_empty());
}
