use super::*;
use crate::{
    native_outstandings::NativeLedgerSnapshotPeriod,
    outstandings_shared::DateBoundaryProfile,
    text_encoding::{
        decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
    },
};
use bridge_tally_primitives::{ExactDecimal, TallyDate};

/// The non-empty answer of 9 Oct 2026 (one synthetic company): two ledgers listed.
const LIVE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_ledgers_bridge_lab_win_fy_live.utf16le.xml");
const REQUEST: &[u8] = include_bytes!(
    "../tests/fixtures/builtin_negative_ledgers_bridge_lab_win_fy_request.utf16le.xml"
);
/// The answer of 21 Oct 2026 on a book with eleven listed ledgers, both sides and several groups.
const SHAPE_LAB: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_ledgers_shape_lab_fy_live.utf16le.xml");
const SHAPE_LAB_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_ledgers_shape_lab_fy_request.utf16le.xml");
/// The answer of 6 Oct 2026 on a book with no negative ledger: an empty envelope.
const EMPTY_ENVELOPE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_ledgers_probe_b_fy_empty_live.utf16le.xml");
/// Tally's refusal of an unknown report name on 7.1: a failure signal, not a bare `RESPONSE`.
const UNKNOWN_REPORT: &[u8] =
    include_bytes!("../tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml");

const COMPANY: &str = "Bridge Lab Win 0.5.1";

/// A captured response, decoded as production decodes it: BOM-less UTF-16LE under
/// the `charset=utf-16` content type Tally sent.
fn response(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text
}

fn live() -> String {
    response(LIVE)
}

fn dec(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value).unwrap()
}

fn invalid_response(
    code: &'static str,
) -> Result<NativeNegativeLedgers, NativeNegativeLedgersError> {
    Err(NativeNegativeLedgersError::InvalidResponse(code))
}

/// The captured answer compacted to one line, so a mutation matches one string.
fn compact(xml: &str) -> String {
    xml.split('\n').map(str::trim).collect::<String>()
}

fn mutate(from: &str, to: &str) -> String {
    let compact = compact(&live());
    assert!(compact.contains(from), "mutation target not found: {from}");
    compact.replacen(from, to, 1)
}

// ---- the request ----

#[test]
fn the_request_is_byte_equal_to_its_committed_capture() {
    let period = NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20260401").unwrap(),
        TallyDate::parse("20270331").unwrap(),
    )
    .unwrap();
    assert_eq!(
        render_native_negative_ledgers_request(COMPANY, &period),
        decode_xml_bytes(REQUEST).unwrap()
    );
}

#[test]
fn the_second_requests_bytes_are_those_of_the_committed_capture() {
    let period = NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20250401").unwrap(),
        TallyDate::parse("20260331").unwrap(),
    )
    .unwrap();
    assert_eq!(
        render_native_negative_ledgers_request("BRIDGE SHAPE LAB", &period),
        decode_xml_bytes(SHAPE_LAB_REQUEST).unwrap()
    );
}

// ---- the captured responses ----

#[test]
fn a_list_of_eleven_on_a_book_with_both_sides_keeps_each_side_and_tallys_order() {
    // A debit is a negative number on the wire and the unsigned amount here (the side names it).
    let parsed = parse_native_negative_ledgers(&response(SHAPE_LAB)).unwrap();
    let side = |entry: &NativeNegativeLedger| match &entry.balance {
        NegativeLedgerBalance::Debit(amount) => ("debit", amount.as_str().to_string()),
        NegativeLedgerBalance::Credit(amount) => ("credit", amount.as_str().to_string()),
    };
    let listed: Vec<(&str, (&str, String))> = parsed
        .entries
        .iter()
        .map(|entry| (entry.name.as_str(), side(entry)))
        .collect();
    let expected = [
        ("Bank of Baroda CA", "credit", "24270.50"),
        ("Capital A/c", "debit", "1000000.00"),
        ("Cess", "debit", "200.00"),
        ("CGST", "debit", "3600.25"),
        ("HDFC CC", "debit", "252600.00"),
        ("IGST", "debit", "1800.00"),
        ("Shape Buyer 1", "credit", "1180.00"),
        ("Shape Buyer 2", "credit", "8875.00"),
        ("Shape Buyer 3", "credit", "9090.00"),
        ("State Tax", "debit", "3600.25"),
        ("Working Capital Loan", "debit", "500000.00"),
    ];
    assert_eq!(listed.len(), expected.len());
    for ((name, (got_side, amount)), (want_name, want_side, want_amount)) in
        listed.iter().zip(expected)
    {
        assert_eq!(*name, want_name);
        assert_eq!(*got_side, want_side, "{want_name}");
        assert_eq!(amount, want_amount, "{want_name}");
    }
}

#[test]
fn the_captured_answer_lists_a_debit_creditor_and_a_credit_debtor_in_tallys_order() {
    let parsed = parse_native_negative_ledgers(&live()).unwrap();
    assert_eq!(
        parsed.entries,
        vec![
            NativeNegativeLedger {
                name: "Synthetic Creditor".to_string(),
                balance: NegativeLedgerBalance::Debit(dec("2500.50")),
            },
            NativeNegativeLedger {
                name: "Synthetic Debtor".to_string(),
                balance: NegativeLedgerBalance::Credit(dec("10000.00")),
            },
        ]
    );
}

#[test]
fn an_empty_envelope_is_an_empty_list_not_a_refusal_and_not_a_finding() {
    // The captured answer of a book with no negative ledger, a self-closed root and an
    // empty pair are all the same empty list. What an empty list means is the caller's:
    // it cannot be told from a report that was not rendered.
    for xml in [response(EMPTY_ENVELOPE), "<ENVELOPE/>".to_string()] {
        assert_eq!(
            parse_native_negative_ledgers(&xml),
            Ok(NativeNegativeLedgers {
                entries: Vec::new()
            })
        );
    }
}

#[test]
fn an_unknown_report_name_is_a_reported_failure_by_structure_and_a_bare_response_is_unknown() {
    assert_eq!(
        parse_native_negative_ledgers(&response(UNKNOWN_REPORT)),
        Err(NativeNegativeLedgersError::TallyReportedFailure)
    );
    assert_eq!(
        parse_native_negative_ledgers("<RESPONSE>Unknown Request</RESPONSE>"),
        Err(NativeNegativeLedgersError::UnknownReport)
    );
}

#[test]
fn a_failure_signal_beside_the_entries_beats_them_and_one_inside_an_entry_is_a_shape_error() {
    for tail in ["<LINEERROR>x</LINEERROR>", "<STATUS/>", "<ERROR/>"] {
        let xml = mutate("</ENVELOPE>", &format!("{tail}</ENVELOPE>"));
        assert_eq!(
            parse_native_negative_ledgers(&xml),
            Err(NativeNegativeLedgersError::TallyReportedFailure),
            "{tail}"
        );
    }
    let in_entry = mutate("<DSPCLDRAMT>", "<DSPCLDRAMT><LINEERROR>x</LINEERROR>");
    assert_eq!(
        parse_native_negative_ledgers(&in_entry),
        invalid_response("negative_ledgers_column_shape")
    );
}

#[test]
fn a_wrapper_is_admitted_only_to_reach_a_failure_signal() {
    for xml in [
        "<ENVELOPE><HEADER><VERSION>1</VERSION></HEADER></ENVELOPE>",
        "<ENVELOPE><BODY><DATA></DATA></BODY></ENVELOPE>",
    ] {
        assert_eq!(
            parse_native_negative_ledgers(xml),
            invalid_response("negative_ledgers_wrapper_without_failure_signal"),
            "{xml}"
        );
    }
    assert_eq!(
        parse_native_negative_ledgers("<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER></ENVELOPE>"),
        Err(NativeNegativeLedgersError::TallyReportedFailure)
    );
}

// ---- the sign and the sides ----

#[test]
fn a_debit_that_is_not_negative_and_a_credit_that_is_not_positive_are_refused() {
    // The sign convention was measured on one book (debit negative, credit positive); an
    // answer that breaks it is refused, not reinterpreted.
    for (from, to) in [
        (
            "<DSPCLDRAMTA>-2500.50</DSPCLDRAMTA>",
            "<DSPCLDRAMTA>2500.50</DSPCLDRAMTA>",
        ),
        (
            "<DSPCLCRAMTA>10000.00</DSPCLCRAMTA>",
            "<DSPCLCRAMTA>-10000.00</DSPCLCRAMTA>",
        ),
        (
            "<DSPCLDRAMTA>-2500.50</DSPCLDRAMTA>",
            "<DSPCLDRAMTA>0</DSPCLDRAMTA>",
        ),
        (
            "<DSPCLCRAMTA>10000.00</DSPCLCRAMTA>",
            "<DSPCLCRAMTA>0.00</DSPCLCRAMTA>",
        ),
    ] {
        assert_eq!(
            parse_native_negative_ledgers(&mutate(from, to)),
            Err(NativeNegativeLedgersError::BalanceUnexpected),
            "{from} -> {to}"
        );
    }
}

#[test]
fn an_entry_with_both_columns_or_neither_is_refused() {
    // Both columns filled on the creditor.
    let both = mutate(
        "<DSPCLCRAMTA></DSPCLCRAMTA>",
        "<DSPCLCRAMTA>5.00</DSPCLCRAMTA>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&both),
        Err(NativeNegativeLedgersError::BalanceUnexpected)
    );
    // Neither column filled on the creditor.
    let neither = mutate(
        "<DSPCLDRAMTA>-2500.50</DSPCLDRAMTA>",
        "<DSPCLDRAMTA></DSPCLDRAMTA>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&neither),
        Err(NativeNegativeLedgersError::BalanceUnexpected)
    );
}

#[test]
fn a_grouped_or_suffixed_amount_is_refused() {
    for text in ["-2,500.50", "2500.50 Dr", "-2500.50 ", "₹-2500.50"] {
        let xml = mutate(
            "<DSPCLDRAMTA>-2500.50</DSPCLDRAMTA>",
            &format!("<DSPCLDRAMTA>{text}</DSPCLDRAMTA>"),
        );
        assert_eq!(
            parse_native_negative_ledgers(&xml),
            Err(NativeNegativeLedgersError::InvalidAmount),
            "{text:?}"
        );
    }
}

// ---- the names ----

#[test]
fn a_ledger_named_twice_is_refused() {
    let xml = mutate(
        "<DSPDISPNAME>Synthetic Debtor</DSPDISPNAME>",
        "<DSPDISPNAME>Synthetic Creditor</DSPDISPNAME>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&xml),
        Err(NativeNegativeLedgersError::LedgerRepeated)
    );
}

#[test]
fn a_name_with_no_entry_and_an_entry_with_no_name_are_refused() {
    // A name followed by another name.
    let two_names = mutate(
        "<DSPACCINFO>",
        "<DSPACCNAME><DSPDISPNAME>Extra</DSPDISPNAME></DSPACCNAME><DSPACCINFO>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&two_names),
        invalid_response("negative_ledgers_name_without_entry")
    );
    // A name left at the end.
    let trailing_name = mutate(
        "</ENVELOPE>",
        "<DSPACCNAME><DSPDISPNAME>Extra</DSPDISPNAME></DSPACCNAME></ENVELOPE>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&trailing_name),
        invalid_response("negative_ledgers_name_without_entry")
    );
    // An entry first.
    let entry_first =
        compact(&live()).replacen("<DSPACCNAME>", "<DSPACCINFO></DSPACCINFO><DSPACCNAME>", 1);
    assert_eq!(
        parse_native_negative_ledgers(&entry_first),
        invalid_response("negative_ledgers_entry_without_name")
    );
}

#[test]
fn an_empty_or_missing_display_name_is_refused() {
    let empty = mutate(
        "<DSPDISPNAME>Synthetic Debtor</DSPDISPNAME>",
        "<DSPDISPNAME>  </DSPDISPNAME>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&empty),
        invalid_response("negative_ledgers_name_empty")
    );
    let missing = mutate("<DSPDISPNAME>Synthetic Debtor</DSPDISPNAME>", "");
    assert_eq!(
        parse_native_negative_ledgers(&missing),
        invalid_response("negative_ledgers_name_missing")
    );
}

#[test]
fn a_name_keeps_its_xml_escapes() {
    let xml = mutate(
        "<DSPDISPNAME>Synthetic Debtor</DSPDISPNAME>",
        "<DSPDISPNAME>A &amp; B &lt;Co&gt;</DSPDISPNAME>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&xml).unwrap().entries[1].name,
        "A & B <Co>"
    );
}

// ---- the shape ----

#[test]
fn any_element_outside_the_expected_entries_is_refused() {
    for (from, to, code) in [
        (
            "<DSPCLCRAMT>",
            "<DSPCLXAMT></DSPCLXAMT><DSPCLCRAMT>",
            "negative_ledgers_unexpected_element",
        ),
        (
            "</ENVELOPE>",
            "<SOMETHING/></ENVELOPE>",
            "negative_ledgers_unexpected_empty_element",
        ),
        (
            "</ENVELOPE>",
            "<SOMETHING></SOMETHING></ENVELOPE>",
            "negative_ledgers_unexpected_element",
        ),
        (
            "</ENVELOPE>",
            "stray</ENVELOPE>",
            "negative_ledgers_stray_text",
        ),
    ] {
        assert_eq!(
            parse_native_negative_ledgers(&mutate(from, to)),
            invalid_response(code),
            "{to}"
        );
    }
}

#[test]
fn a_column_twice_or_missing_is_refused() {
    let twice = mutate(
        "</DSPCLCRAMT>",
        "</DSPCLCRAMT><DSPCLCRAMT><DSPCLCRAMTA></DSPCLCRAMTA></DSPCLCRAMT>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&twice),
        invalid_response("negative_ledgers_duplicate_column")
    );
    let missing = compact(&live()).replacen(
        "<DSPCLCRAMT><DSPCLCRAMTA></DSPCLCRAMTA></DSPCLCRAMT>",
        "",
        1,
    );
    assert_eq!(
        parse_native_negative_ledgers(&missing),
        invalid_response("negative_ledgers_column_missing")
    );
    let amount_missing = mutate(
        "<DSPCLCRAMT><DSPCLCRAMTA></DSPCLCRAMTA></DSPCLCRAMT>",
        "<DSPCLCRAMT></DSPCLCRAMT>",
    );
    assert_eq!(
        parse_native_negative_ledgers(&amount_missing),
        invalid_response("negative_ledgers_amount_missing")
    );
}

#[test]
fn a_cut_off_or_wrongly_rooted_answer_is_refused() {
    let cut = compact(&live()).replace("</ENVELOPE>", "");
    assert_eq!(
        parse_native_negative_ledgers(&cut),
        invalid_response("negative_ledgers_envelope_unterminated")
    );
    assert_eq!(
        parse_native_negative_ledgers("<REPORT></REPORT>"),
        invalid_response("negative_ledgers_root_not_envelope")
    );
    assert_eq!(
        parse_native_negative_ledgers("<REPORT/>"),
        invalid_response("negative_ledgers_root_not_envelope")
    );
    assert_eq!(
        parse_native_negative_ledgers(&format!("{}<ENVELOPE></ENVELOPE>", compact(&live()))),
        invalid_response("negative_ledgers_trailing_content")
    );
}

#[test]
fn a_self_closed_root_is_a_closed_envelope_and_nothing_may_follow_it() {
    assert_eq!(
        parse_native_negative_ledgers("<ENVELOPE/>"),
        Ok(NativeNegativeLedgers {
            entries: Vec::new()
        })
    );
    assert_eq!(
        parse_native_negative_ledgers("<ENVELOPE/>junk"),
        invalid_response("negative_ledgers_stray_text")
    );
    assert_eq!(
        parse_native_negative_ledgers("<ENVELOPE/><ENVELOPE></ENVELOPE>"),
        invalid_response("negative_ledgers_trailing_content")
    );
}

#[test]
fn an_empty_input_is_an_unterminated_answer_not_an_empty_list() {
    assert_eq!(
        parse_native_negative_ledgers(""),
        invalid_response("negative_ledgers_envelope_unterminated")
    );
}

#[test]
fn a_name_repeated_in_another_case_or_with_padding_is_refused() {
    for repeated in ["synthetic creditor", "Synthetic Creditor "] {
        let xml = mutate(
            "<DSPDISPNAME>Synthetic Debtor</DSPDISPNAME>",
            &format!("<DSPDISPNAME>{repeated}</DSPDISPNAME>"),
        );
        assert_eq!(
            parse_native_negative_ledgers(&xml),
            Err(NativeNegativeLedgersError::LedgerRepeated),
            "{repeated:?}"
        );
    }
}

#[test]
fn a_negative_zero_is_neither_side() {
    for from_to in [
        (
            "<DSPCLDRAMTA>-2500.50</DSPCLDRAMTA>",
            "<DSPCLDRAMTA>-0.00</DSPCLDRAMTA>",
        ),
        (
            "<DSPCLCRAMTA>10000.00</DSPCLCRAMTA>",
            "<DSPCLCRAMTA>-0</DSPCLCRAMTA>",
        ),
    ] {
        assert_eq!(
            parse_native_negative_ledgers(&mutate(from_to.0, from_to.1)),
            Err(NativeNegativeLedgersError::BalanceUnexpected),
            "{from_to:?}"
        );
    }
}

#[test]
fn a_failure_signal_first_in_an_entry_is_a_reported_failure() {
    assert_eq!(
        parse_native_negative_ledgers(&mutate(
            "<DSPACCINFO>",
            "<DSPACCINFO><LINEERROR>x</LINEERROR>"
        )),
        Err(NativeNegativeLedgersError::TallyReportedFailure)
    );
}

#[test]
fn a_self_closed_amount_is_an_empty_column_and_a_second_name_element_is_refused() {
    assert_eq!(
        parse_native_negative_ledgers(&mutate("<DSPCLCRAMTA></DSPCLCRAMTA>", "<DSPCLCRAMTA/>")),
        parse_native_negative_ledgers(&live())
    );
    assert_eq!(
        parse_native_negative_ledgers(&mutate(
            "<DSPDISPNAME>Synthetic Debtor</DSPDISPNAME>",
            "<DSPDISPNAME>Synthetic Debtor</DSPDISPNAME><DSPDISPNAME>x</DSPDISPNAME>"
        )),
        invalid_response("negative_ledgers_name_shape")
    );
}

#[test]
fn every_error_has_its_own_stable_code() {
    let errors = [
        (
            NativeNegativeLedgersError::TallyReportedFailure,
            "negative_ledgers_tally_reported_failure",
        ),
        (
            NativeNegativeLedgersError::UnknownReport,
            "negative_ledgers_report_unknown",
        ),
        (
            NativeNegativeLedgersError::InvalidAmount,
            "negative_ledgers_amount_invalid",
        ),
        (
            NativeNegativeLedgersError::LedgerRepeated,
            "negative_ledgers_ledger_repeated",
        ),
        (
            NativeNegativeLedgersError::BalanceUnexpected,
            "negative_ledgers_balance_unexpected",
        ),
        (
            NativeNegativeLedgersError::InvalidResponse("x_code"),
            "x_code",
        ),
    ];
    for (error, code) in errors {
        assert_eq!(error.code(), code);
    }
    let mut codes: Vec<_> = errors.iter().map(|(error, _)| error.code()).collect();
    codes.sort_unstable();
    codes.dedup();
    assert_eq!(codes.len(), errors.len());
}

#[test]
fn the_serialised_balance_names_its_side_and_carries_no_sign() {
    let parsed = parse_native_negative_ledgers(&live()).unwrap();
    assert_eq!(
        serde_json::to_value(&parsed.entries[0].balance).unwrap(),
        serde_json::json!({"side": "debit", "amount": "2500.50"})
    );
    assert_eq!(
        serde_json::to_value(&parsed.entries[1].balance).unwrap(),
        serde_json::json!({"side": "credit", "amount": "10000.00"})
    );
}
