//! `outstandings` with `party` and `detail: open_bills` (#2377): one party's open
//! bills from Tally's `Ledger Outstandings`, without the whole-book Bills reports.
use super::*;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::ledger_scope::LedgerScopeName;
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
        let (catalogue, read) = self
            .read_resolvable_ledgers(&identity, &company.name)
            .await
            .map_err(|failure| late(failure, &evidence))?;
        evidence = combine_evidence(evidence, read);
        let (_, party) =
            resolve_catalogue_ledger_or_refuse(&catalogue, party_argument, self.settings.redaction)
                .map_err(|failure| late(failure, &evidence))?;
        // The book's own name is what is sent, and only a name `LedgerScopeName`
        // admits (DESIGN deviation of 8 Nov 2026). A trailing space, which Tally
        // trims on create, is among its refusals.
        let scope = LedgerScopeName::new(&party).map_err(|_| {
            late(
                ToolFailure::from("party_name_not_admitted".to_string()),
                &evidence,
            )
        })?;
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
            "as_of": to.as_str(),
            "books_from": load.books_from.as_str(),
            "no_open_bills": load.read.has_no_open_bills(),
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
                "Measured on one synthetic book on TallyPrime 7.2: a receivable party at two dates and parties with one bill each; a payable party, a party with bill-wise off, a foreign-currency ledger and a party with no open bills were not measured",
                "The party name sent is the book's own ledger name, and only one of ASCII letters, digits and single inner spaces; any other name is refused before anything is sent",
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
