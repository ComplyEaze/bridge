//! Every answer here is a capture from one synthetic book on TallyPrime 7.2
//! (`tests/fixtures/agent/party-bills/PROVENANCE.md`).

use super::*;
use crate::ledger_scope::LedgerScopeNameError;
use crate::native_outstandings::parse_native_ledger_snapshot;

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/agent/party-bills/"
);

fn capture(name: &str) -> String {
    let bytes = std::fs::read(format!("{FIXTURES}{name}")).expect("capture is present");
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    assert_eq!(bytes.len() % 2, 0, "{name} holds whole UTF-16 units");
    String::from_utf16(&units).expect("capture decodes")
}

fn date(value: &str) -> TallyDate {
    TallyDate::parse(value).expect("date")
}

fn amount(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value).expect("amount")
}

fn books_from() -> TallyDate {
    date("20250401")
}

fn answer(name: &str, to: &str) -> PartyBillsAnswer {
    parse_ledger_outstandings_answer(&capture(name), &books_from(), &date(to))
        .expect("answer parses")
}

fn closing(snapshot: &str, ledger: &str) -> Option<ExactDecimal> {
    parse_native_ledger_snapshot(&capture(snapshot))
        .expect("snapshot parses")
        .into_iter()
        .find(|entry| entry.name == ledger)
        .unwrap_or_else(|| panic!("{ledger} is in the snapshot"))
        .closing_balance
}

fn total(bills: &[PartyBill]) -> ExactDecimal {
    bills.iter().fold(ExactDecimal::zero(), |sum, bill| {
        sum.checked_add(&bill.closing).expect("sum")
    })
}

#[test]
fn the_request_is_the_lab_request_without_its_indentation() {
    let party = LedgerScopeName::new("Lab Party P1").expect("admitted");
    assert_eq!(
        render_ledger_outstandings_request("BRIDGE BILLTRAIL LAB", &party, &date("20260331")),
        concat!(
            "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST>",
            "<TYPE>Data</TYPE><ID>Ledger Outstandings</ID></HEADER><BODY><DESC><STATICVARIABLES>",
            "<SVCURRENTCOMPANY>BRIDGE BILLTRAIL LAB</SVCURRENTCOMPANY>",
            "<LedgerName>Lab Party P1</LedgerName>",
            "<SVTODATE TYPE=\"Date\">20260331</SVTODATE>",
            "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><EXPLODEFLAG>Yes</EXPLODEFLAG>",
            "</STATICVARIABLES></DESC></BODY></ENVELOPE>"
        )
    );
}

#[test]
fn a_party_name_with_a_trailing_space_is_never_sent() {
    // Tally holds such a party without the space, and a tie by the spaced name
    // finds no ledger; the book's own name is what is sent.
    assert_eq!(
        LedgerScopeName::new("L5 Trail "),
        Err(LedgerScopeNameError::Spacing)
    );
    assert!(LedgerScopeName::new("L5 Trail").is_ok());
}

#[test]
fn a_party_s_bills_tie_to_its_ledger_closing_at_two_dates() {
    for (lo, snapshot, to, expected) in [
        (
            "ledger-outstandings-p1-20260331.utf16le.xml",
            "ledger-snapshot-p1-20260331.utf16le.xml",
            "20260331",
            "-26000.00",
        ),
        (
            "ledger-outstandings-p1-20260731.utf16le.xml",
            "ledger-snapshot-p1-20260731.utf16le.xml",
            "20260731",
            "-37000.00",
        ),
    ] {
        let parsed = answer(lo, to);
        assert!(total(parsed.rows()).numeric_eq(&amount(expected)), "{lo}");
        let read = parsed
            .tie(closing(snapshot, "Lab Party P1").as_ref())
            .expect("ties");
        assert!(!read.has_no_open_bills());
        let both = [read.bills(), read.on_account()].concat();
        assert!(total(&both).numeric_eq(&amount(expected)), "{lo}");
    }
}

#[test]
fn the_march_answer_holds_five_bills_and_one_on_account_row() {
    let read = answer("ledger-outstandings-p1-20260331.utf16le.xml", "20260331")
        .tie(closing("ledger-snapshot-p1-20260331.utf16le.xml", "Lab Party P1").as_ref())
        .expect("ties");
    assert_eq!(read.bills().len(), 5);
    assert_eq!(read.on_account().len(), 1);
    let first = &read.bills()[0];
    assert_eq!(
        first.reference,
        PartyBillReference::Bill("OB-1".to_string())
    );
    assert_eq!(first.bill_date, date("20250331"));
    assert!(first.closing.numeric_eq(&amount("-10000.00")));
    assert_eq!(first.due_date, Some(date("20250331")));
    assert_eq!(first.overdue_days, Some(365));
}

#[test]
fn bills_against_another_date_s_closing_do_not_tie() {
    let refusal = answer("ledger-outstandings-p1-20260331.utf16le.xml", "20260331")
        .tie(closing("ledger-snapshot-p1-20260731.utf16le.xml", "Lab Party P1").as_ref())
        .expect_err("a March answer against a July closing");
    assert_eq!(refusal.code(), "party_bills_untied");
    let PartyBillsRefusal::Untied {
        bills_total,
        ledger_closing,
    } = refusal
    else {
        panic!("untied: {refusal:?}");
    };
    assert!(bills_total.numeric_eq(&amount("-26000.00")));
    assert!(ledger_closing.numeric_eq(&amount("-37000.00")));
}

#[test]
fn an_empty_closing_is_not_zero() {
    let refusal = answer("ledger-outstandings-p1-20260331.utf16le.xml", "20260331")
        .tie(None)
        .expect_err("no closing");
    assert_eq!(refusal, PartyBillsRefusal::LedgerClosingNotRead);
}

#[test]
fn an_unknown_party_s_empty_body_never_parses_as_zero_bills() {
    // The transport refuses this answer by its missing content type first
    // (`tally/runtime_party_bills_tests.rs`); the parser refuses it as well.
    let headers = std::fs::read_to_string(format!(
        "{FIXTURES}ledger-outstandings-unknown-party.headers.txt"
    ))
    .expect("headers");
    assert!(headers.starts_with("HTTP/1.1 200 OK"));
    assert!(headers.contains("CONTENT-LENGTH: 0"));
    assert!(!headers.to_ascii_uppercase().contains("CONTENT-TYPE"));
    let body = capture("ledger-outstandings-unknown-party.utf16le.xml");
    assert!(body.is_empty());
    assert_eq!(
        parse_ledger_outstandings_answer(&body, &books_from(), &date("20260331")),
        Err(PartyBillsAnswerError::Malformed {
            rule: "party_bills_envelope_unclosed",
            row: None,
        })
    );
}

#[test]
fn an_envelope_with_no_rows_is_no_open_bills_only_for_a_zero_closing() {
    let empty = parse_ledger_outstandings_answer(
        "<ENVELOPE>\n</ENVELOPE>",
        &books_from(),
        &date("20260331"),
    )
    .expect("an envelope parses");
    assert!(empty
        .clone()
        .tie(Some(&ExactDecimal::zero()))
        .expect("zero closing")
        .has_no_open_bills());
    assert_eq!(
        empty.tie(Some(&amount("-100.00"))),
        Err(PartyBillsRefusal::Untied {
            bills_total: ExactDecimal::zero(),
            ledger_closing: amount("-100.00"),
        })
    );
}

#[test]
fn an_ampersand_party_s_bill_ties_to_its_ledger() {
    let read = answer("ledger-outstandings-ampersand.utf16le.xml", "20260331")
        .tie(closing("ledger-snapshot-ampersand.utf16le.xml", "L5 Amp & Co").as_ref())
        .expect("ties");
    assert_eq!(read.bills().len(), 1);
    assert!(read.bills()[0].closing.numeric_eq(&amount("-100.00")));
}

#[test]
fn the_trimmed_party_s_bill_ties_by_the_name_tally_kept() {
    let read = answer("ledger-outstandings-trailing-space.utf16le.xml", "20260331")
        .tie(closing("ledger-snapshot-trailing-space.utf16le.xml", "L5 Trail").as_ref())
        .expect("ties");
    assert_eq!(
        read.bills()[0].reference,
        PartyBillReference::Bill("L5-trail".to_string())
    );
}

#[test]
fn two_on_account_rows_on_one_party_are_both_kept() {
    let parsed = answer("ledger-outstandings-two-on-account.utf16le.xml", "20260331");
    assert!(total(parsed.rows()).numeric_eq(&amount("-150.00")));
    let rows = parsed.rows();
    assert!(rows
        .iter()
        .all(|bill| bill.reference == PartyBillReference::OnAccount));
    assert_eq!(rows.len(), 2);
    // Tally leaves the second row's due date and overdue days empty.
    assert_eq!(
        (rows[1].due_date.clone(), rows[1].overdue_days),
        (None, None)
    );
}

#[test]
fn a_status_element_is_tally_s_failure() {
    assert_eq!(
        parse_ledger_outstandings_answer(
            "<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER></ENVELOPE>",
            &books_from(),
            &date("20260331"),
        ),
        Err(PartyBillsAnswerError::TallyReportedFailure)
    );
}

#[test]
fn a_cut_answer_does_not_parse() {
    let whole = capture("ledger-outstandings-p1-20260331.utf16le.xml");
    let cut = &whole[..whole.len() / 2];
    let error =
        parse_ledger_outstandings_answer(cut, &books_from(), &date("20260331")).expect_err("cut");
    assert!(
        matches!(error, PartyBillsAnswerError::Malformed { .. }),
        "{error:?}"
    );
}

#[test]
fn an_empty_bill_closing_refuses_the_answer() {
    let whole = capture("ledger-outstandings-ampersand.utf16le.xml");
    let emptied = whole.replacen("<BILLCL>-100.00</BILLCL>", "<BILLCL></BILLCL>", 1);
    assert_ne!(emptied, whole);
    assert_eq!(
        parse_ledger_outstandings_answer(&emptied, &books_from(), &date("20260331")),
        Err(PartyBillsAnswerError::Malformed {
            rule: "party_bills_closing_empty",
            row: Some(1),
        })
    );
}
