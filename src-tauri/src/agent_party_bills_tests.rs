//! The `open_bills` detail's own refusals, each asserted by the code the caller
//! is given. No Tally answer is written here: the runtime's typed errors are
//! passed through the tool's own mapping as the read would raise them.
use super::*;
use crate::tally::connection::PairedReadValidationError;
use crate::tally::runtime::PartyBillsReadError;
use bridge_tally_core::ExactDecimal;
use bridge_tally_protocol::native_outstandings::{PartyBillsAnswerError, PartyBillsRefusal};
use bridge_tally_transport::TallyTransportError;

fn tool_failure(error: anyhow::Error) -> ToolFailure {
    ToolFailure::from_runtime("party_bills_read_failed", error.context("party bills read"))
}

#[test]
fn each_refusal_of_the_read_reaches_the_caller_by_its_own_code() {
    let untied = PartyBillsRefusal::Untied {
        bills_total: ExactDecimal::parse("-26000.00").unwrap(),
        ledger_closing: ExactDecimal::parse("-25000.00").unwrap(),
    };
    for (error, code) in [
        (PartyBillsReadError::NotInSnapshot, "party_not_in_book"),
        (PartyBillsReadError::NotBillWise, "party_not_billwise"),
        (
            PartyBillsReadError::NoneReported,
            "party_bills_none_reported",
        ),
        (
            PartyBillsReadError::AnswerMissing,
            "party_bills_answer_missing",
        ),
        (
            PartyBillsReadError::EducationUnqualified,
            "party_bills_education_unqualified",
        ),
        (
            PartyBillsReadError::Answer(PartyBillsAnswerError::TallyReportedFailure),
            "party_bills_tally_failure",
        ),
        (
            PartyBillsReadError::Answer(PartyBillsAnswerError::Malformed {
                rule: "party_bills_closing_empty",
                row: Some(1),
            }),
            "party_bills_answer_malformed",
        ),
        (
            PartyBillsReadError::Refused(PartyBillsRefusal::LedgerClosingNotRead),
            "party_ledger_closing_not_read",
        ),
        (PartyBillsReadError::Refused(untied), "party_bills_untied"),
        (
            PartyBillsReadError::Refused(PartyBillsRefusal::NoRowsNotEstablished),
            "party_bills_no_rows_not_established",
        ),
    ] {
        assert_eq!(tool_failure(anyhow::Error::new(error)).code, code);
    }
}

/// Two answers to one request that differ are the read's failure, with the
/// drift named as its cause.
#[test]
fn a_drifted_pair_is_the_read_s_failure_with_the_drift_as_its_cause() {
    let failure = tool_failure(anyhow::Error::new(PairedReadValidationError::PartyBills));
    assert_eq!(failure.code, "party_bills_read_failed");
    assert_eq!(failure.cause, Some("party_bills_drifted"));
}

/// An empty head that is not the captured one stays the transport's refusal
/// (`runtime_party_bills_tests.rs`), and reaches the caller as the read's failure; the empty
/// answer itself, should any read meet it unasked, is reported with the same cause.
#[test]
fn the_empty_answer_s_transport_refusal_is_the_read_s_failure() {
    for transport in [
        TallyTransportError::InvalidEncoding {
            code: "response_content_type_unsupported",
        },
        TallyTransportError::EmptyAnswer,
    ] {
        let failure = tool_failure(anyhow::Error::new(transport));
        assert_eq!(failure.code, "party_bills_read_failed");
        assert_eq!(
            shown_cause(&failure.code, failure.cause, failure.unanswered),
            Some("response_content_type_unsupported")
        );
    }
}

#[test]
fn the_none_reported_next_step_does_not_treat_an_empty_closing_as_zero() {
    assert_eq!(
        crate::agent::remediation_for("party_bills_read_failed", Some("party_bills_none_reported")),
        Some(
            "The ledger list carries this party, bill-wise, with an empty closing at as_of, and \
             Tally's bill report sent nothing for it. ComplyEaze Bridge does not state that the \
             party has no open bills: an empty closing is not established to be zero, and even a \
             zero balance may hold an open bill set against an on-account advance; that answer is \
             not measured. For the party's balance, read ledger_masters or trial_balance. The same \
             call refuses again."
        )
    );
}

/// One ledger per entry, each as the names it answers to.
fn ledgers(names: &[&[&str]]) -> Vec<Vec<String>> {
    names
        .iter()
        .map(|names| names.iter().map(|name| name.to_string()).collect())
        .collect()
}

/// Each refusal of the name is `party_name_not_admitted`, with its reason as the cause.
#[test]
fn a_name_the_party_type_refuses_is_refused_before_anything_is_sent() {
    let book = ledgers(&[&["Lab Party P1"]]);
    for (party, cause) in [
        ("L5 Trail ", "party_name_spacing"),
        (" L5 Trail", "party_name_spacing"),
        ("L5  Trail", "party_name_spacing"),
        ("", "party_name_empty"),
        ("L5 Amp ; Co", "party_name_mark_not_measured"),
        ("L5 Amp \u{2019}s", "party_name_mark_not_measured"),
        ("Salary Not Paid", "party_name_formula_word_not_measured"),
    ] {
        let Err(failure) = admitted_party(party, &book) else {
            panic!("{party:?} was admitted");
        };
        assert_eq!(failure.code, "party_name_not_admitted", "{party:?}");
        assert_eq!(failure.cause, Some(cause), "{party:?}");
    }
    assert_eq!(
        admitted_party("Lab Party P1", &book).unwrap().as_str(),
        "Lab Party P1"
    );
}

/// Before #2480 the ampersand name was refused here; it holds a measured mark,
/// and its key is the book's alone, so it is admitted now.
#[test]
fn a_name_with_measured_marks_is_admitted_when_its_key_is_its_own() {
    let book = ledgers(&[
        &["L5 Amp & Co"],
        &["Cash"],
        &["BRIDGE Mx Ak, Am & Co"],
        &["Printing and Stationery"],
    ]);
    for party in [
        "L5 Amp & Co",
        "BRIDGE Mx Ak, Am & Co",
        "Printing and Stationery",
    ] {
        assert_eq!(admitted_party(party, &book).unwrap().as_str(), party);
    }
}

/// The lab read every marked name where no other ledger had its letters and
/// digits. Another ledger that has them, by its row spelling, its stored name
/// or an alias, refuses the name; a name with no mark keeps its old admission.
#[test]
fn a_marked_name_whose_key_another_ledger_shares_is_refused() {
    for book in [
        ledgers(&[&["Ak & Am"], &["Ak Am"]]),
        ledgers(&[&["Ak & Am"], &["Old Name", "AK-AM"]]),
        ledgers(&[&["Ak & Am"], &["Other", "Other Stored", "ak.am"]]),
        // A catalogue that does not hold the party at all.
        ledgers(&[&["Cash"]]),
    ] {
        let Err(failure) = admitted_party("Ak & Am", &book) else {
            panic!("admitted against {book:?}");
        };
        assert_eq!(failure.code, "party_name_not_admitted");
        assert_eq!(failure.cause, Some("party_name_key_shared"));
    }
    // A ledger that answers to two names with the key is still one ledger.
    let book = ledgers(&[&["Ak & Am", "Ak, Am"], &["Cash"]]);
    assert!(admitted_party("Ak & Am", &book).is_ok());
    // A plain name is admitted as before #2480, whatever shares its key.
    let book = ledgers(&[&["Ak Am"], &["Ak & Am"]]);
    assert_eq!(admitted_party("Ak Am", &book).unwrap().as_str(), "Ak Am");
}

/// Review P2-1: a padded, lower-case request binds by the case-and-spaces fold,
/// and the answer's `ledger_match` says so, masked or not.
#[test]
fn a_folded_party_name_is_told_as_a_case_or_spacing_match() {
    let catalogue = [
        super::super::ledger_candidates::CatalogueLedger::new("Lab Party P1", None),
        super::super::ledger_candidates::CatalogueLedger::new("Cash", None),
    ];
    let (found, party) =
        resolve_catalogue_ledger_or_refuse(&catalogue, " lab party p1 ", Redaction::None).unwrap();
    assert!(matches!(found, LedgerMatch::CaseOrSpacing { .. }));
    assert_eq!(party, "Lab Party P1");
    let book = ledgers(&[&["Lab Party P1"], &["Cash"]]);
    assert_eq!(
        admitted_party(&party, &book).unwrap().as_str(),
        "Lab Party P1"
    );
    for redaction in [Redaction::None, Redaction::MaskParties] {
        assert_eq!(
            ledger_match_json(&found, &party, redaction)["matched"],
            "case_or_spacing"
        );
    }
}

/// Each reason a party's name is not sent has its own next step under
/// `party_name_not_admitted`, and none repeats the name refused.
#[test]
fn each_party_name_refusal_has_its_own_next_step() {
    let book = ledgers(&[&["Ak & Am"], &["Ak Am"], &["Cash"]]);
    let mut steps = std::collections::BTreeSet::new();
    for party in [
        "Ak ; Am",
        "Ak &Am",
        "Ak Or Am",
        &format!("Ak{}", " & Ak".repeat(15)),
        "Ak & Am",
        "Ak  Am",
        "",
    ] {
        let Err(failure) = admitted_party(party, &book) else {
            panic!("{party:?} was admitted");
        };
        let step = super::remediation_for(&failure.code, failure.cause)
            .unwrap_or_else(|| panic!("a step for {:?}", failure.cause));
        assert!(party.is_empty() || !step.contains(party), "{party:?}");
        assert!(!step.contains("Ak"), "{party:?}");
        steps.insert(step);
    }
    assert_eq!(steps.len(), 7, "each cause has its own step");
    assert!(super::remediation_for("party_name_not_admitted", None).is_some());
}
