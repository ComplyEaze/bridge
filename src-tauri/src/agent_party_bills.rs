//! `outstandings` with `party` and `detail: open_bills` (#2377): one party's open
//! bills from Tally's `Ledger Outstandings`, without the whole-book Bills reports.
use super::*;
use crate::tally::runtime::OpenBillsParty;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::native_outstandings::{PartyBill, PartyBillReference};

pub(super) const OPEN_BILLS_DETAIL: &str = "open_bills";

const VERIFICATION: &str = "party_bills_tied_to_ledger_closing_between_unchanged_marks";

fn bill_json(bill: &PartyBill) -> Value {
    json!({
        "reference": match &bill.reference {
            PartyBillReference::OnAccount => Value::Null,
            PartyBillReference::Bill(reference) => json!(reference),
        },
        "bill_date": bill.bill_date.as_str(),
        "closing": bill.closing.as_str(),
        "due_date": bill.due_date.as_ref().map(TallyDate::as_str),
        "overdue_days": bill.overdue_days,
    })
}

/// The resolved spelling as the name sent, or `party_name_not_admitted` with the reason as its
/// cause: a mark or formula word not measured in this read, the spacing, or another ledger of
/// `ledgers` (every name of every catalogue ledger) sharing its letters-and-digits key (#2480).
fn admitted_party(party: &str, ledgers: &[Vec<String>]) -> Result<OpenBillsParty, ToolFailure> {
    OpenBillsParty::admit(
        party,
        ledgers.iter().map(|names| names.iter().map(String::as_str)),
    )
    .map_err(|refusal| {
        let mut failure = ToolFailure::from("party_name_not_admitted".to_string());
        failure.cause = Some(refusal.code());
        failure
    })
}

impl Server {
    /// The `open_bills` detail. Its arguments are checked before any read; then the
    /// company's marks, the party resolved from the ledger catalogue, the paired
    /// snapshot and report, and the marks again, which must be unchanged.
    pub(super) async fn outstandings_open_bills(
        &self,
        args: &Value,
        guid: &str,
        party_argument: Option<&str>,
        reference: Option<&str>,
    ) -> Result<ToolOutcome, ToolFailure> {
        let party_argument =
            party_argument.ok_or_else(|| ToolFailure::from("detail_requires_party".to_string()))?;
        if reference.is_some() {
            return Err("reference_requires_bill_trail".to_string().into());
        }
        let to = match optional_string(args, "as_of")?
            .as_deref()
            .map(normalized_date)
        {
            Some(as_of) => as_of?,
            None => {
                TallyDate::parse(tally_host_today()).map_err(|_| "invalid_as_of".to_string())?
            }
        };
        let (company, identity, mut evidence) = self.verified_company(guid).await?;
        let late = |failure: ToolFailure, evidence: &Evidence| {
            super::bill_trail::with_evidence(failure, evidence)
        };
        let (opening, read) = self
            .read_company_marks_once(&identity, &company.name)
            .await
            .map_err(|failure| late(failure, &evidence))?;
        evidence = combine_evidence(evidence, read);
        let (catalogue, names, read) = self
            .read_resolvable_ledgers_with_names(&identity, &company.name)
            .await
            .map_err(|failure| late(failure, &evidence))?;
        evidence = combine_evidence(evidence, read);
        let (resolved, party) =
            resolve_catalogue_ledger_or_refuse(&catalogue, party_argument, self.settings.redaction)
                .map_err(|failure| late(failure, &evidence))?;
        // What is sent is the spelling the ledger's voucher rows carry, which the
        // resolver returns, and only a name `OpenBillsParty` admits: the marks
        // measured in this report's `LedgerName` (protocol reference §12a.34),
        // and for a name with any of them, no other ledger sharing its key. A
        // trailing space, which Tally trims on create, is among its refusals. A
        // row spelling the snapshot does not carry fails closed as
        // `party_not_in_book`.
        let scope = admitted_party(&party, &names).map_err(|failure| late(failure, &evidence))?;
        let load = self
            .runtime
            .fetch_party_bills(self.tally_config(), &identity, scope, to.clone())
            .await
            .map_err(|error| {
                ToolFailure::from_runtime("party_bills_read_failed", error)
                    .with_prior_evidence(evidence.clone())
            })?;
        evidence = combine_evidence(evidence, evidence_from_runtime_read(load.evidence));
        let (closing, read) = self
            .read_company_marks_once(&identity, &company.name)
            .await
            .map_err(|failure| late(failure, &evidence))?;
        evidence = combine_evidence(evidence, read);
        if let Some(code) = super::register::window_drift(opening, closing) {
            return Err(late(ToolFailure::from(code.to_string()), &evidence));
        }
        let bills = load.read.bills().iter().map(bill_json).collect::<Vec<_>>();
        let on_account = load
            .read
            .on_account()
            .iter()
            .map(bill_json)
            .collect::<Vec<_>>();
        let result = json!({
            "state": "observed",
            "basis": "tally_ledger_outstandings",
            "detail": OPEN_BILLS_DETAIL,
            "party": redact_value(party_name_value(party.clone()), self.settings.redaction),
            "ledger_match": ledger_match_json(&resolved, &party, self.settings.redaction),
            "as_of": to.as_str(),
            "books_from": load.books_from.as_str(),
            "bills": bills,
            "on_account": on_account,
            "checks": {
                "bills_sum_to_ledger_closing": "tied",
                "company_marks_unchanged": "checked",
            },
            "verification": VERIFICATION,
            "limitations": [
                "Read through Tally's Ledger Outstandings for one party: the bills Tally lists for that ledger at as_of, each with its balance at that date, a debit as a negative amount",
                "Told only when the bills, On Account rows included, sum to the ledger's closing balance at as_of, read from the books' first day; a party whose bills do not is refused, never answered in part",
                "An On Account row is told by its reference text, which Tally writes as On Account; a real bill given that reference would be read as On Account (not measured)",
                "Measured on one synthetic book on TallyPrime 7.2: a receivable party at two dates and parties with one bill each; a payable party, a party with bill-wise off and a foreign-currency ledger were not measured",
                "What Tally answers for a party with no open bills is not measured, so an answer with no rows is refused (party_bills_no_rows_not_established), never told as no open bills",
                "The party name sent is the book's own ledger name. One of ASCII letters, digits and single inner spaces with no formula word is sent as before. Any other is sent only in a shape built from the names the lab read: it starts with a letter; its marks are among - . & / ( ) ' , _ \" + @ # : % * !, each beside the neighbours it was read beside, no more often than one read name holds it; a quote or a spaced bracket is paired within one word, and a bracket inside a word stands only in a name with no other mark, Devanagari or and; the word and appears once, in lower case, between two other words; Devanagari appears once, only as the one word read, not first; neighbouring words are only of kinds read side by side, so no two marks standing alone touch; and it is at most 72 characters. Anything else is refused before anything is sent (party_name_not_admitted, with the reason as its cause)",
                "A name holding any of those marks, the word and or Devanagari is sent only when no other ledger of the catalogue, by any of its names, has the same letters and digits; otherwise it is refused (party_name_not_admitted, cause party_name_key_shared). Groups and other masters are not checked, and look-alike characters in other ledgers' names are not folded",
                "Those shapes are the names of 23 kept captures from one synthetic book on TallyPrime 7.2, single-user edition (tally.exe 1.1.7.2), each read where no other ledger shared its letters and digits; they are not every shape a party name takes",
            ],
        });
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_party_bills_tests.rs"]
mod tests;
