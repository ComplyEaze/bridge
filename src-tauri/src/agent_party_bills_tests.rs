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
fn a_name_the_scope_type_refuses_is_refused_before_anything_is_sent() {
    for party in ["L5 Trail ", " L5 Trail", "L5  Trail", "L5 Amp & Co", ""] {
        let Err(failure) = admitted_party(party) else {
            panic!("{party:?} was admitted");
        };
        assert_eq!(failure.code, "party_name_not_admitted");
    }
    assert_eq!(
        admitted_party("Lab Party P1").unwrap().as_str(),
        "Lab Party P1"
    );
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
    assert_eq!(admitted_party(&party).unwrap().as_str(), "Lab Party P1");
    for redaction in [Redaction::None, Redaction::MaskParties] {
        assert_eq!(
            ledger_match_json(&found, &party, redaction)["matched"],
            "case_or_spacing"
        );
    }
}
