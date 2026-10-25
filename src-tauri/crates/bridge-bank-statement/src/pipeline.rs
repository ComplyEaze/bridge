//! The fail-closed order of operations, in one place.
//!
//! Every stage refuses the whole run; nothing downstream sees a statement an
//! earlier stage could not prove. The order is the one the removed Python tool used: bind the
//! account before trusting the arithmetic, prove the arithmetic before mapping,
//! and re-read what was built.
//!
//! Why that order: the running-balance proof validates the statement's own arithmetic and
//! nothing else, so it is equally happy to certify the wrong account's statement. The
//! account check is the one thing that ties the document in hand to the ledger it will be
//! posted against, so it runs before the arithmetic is trusted and before any proposal
//! exists. The balance replay runs before mapping because a misread amount or a dropped
//! row must stop the run before it is turned into vouchers. The design rule throughout: a
//! malformed input, an ambiguous mapping or an unproven extent stops the run, because
//! every proposal is posted to a real book.

use crate::bank::Bank;
use crate::cash::CashAnswers;
use crate::date::Date;
use crate::geometry::Page;
use crate::mapping::Mapping;
use crate::money::{reconcile, statement_totals, verify_against_statement, Controls, Totals};
use crate::parse::{parse_statement, require_account_match};
use crate::pdf::{no_readable_text, StatementText};
use crate::proposals::{
    build, coverage, group_counterparties, selfcheck, Build, BuildOptions, CounterpartyGroup,
    Selfcheck,
};
use crate::refusal::Refusal;
use bridge_tally_primitives::ExactDecimal;

pub struct StatementRequest<'a> {
    pub bank: Bank,
    /// The operator's account label; its digits bind the statement.
    pub account_label: &'a str,
    pub controls: &'a Controls,
    pub bank_ledger: &'a str,
    pub suspense_ledger: &'a str,
    pub mapping: &'a Mapping,
    /// A person's answer per cash line.
    pub cash_answers: &'a CashAnswers,
    pub date_from: Option<Date>,
    pub date_to: Option<Date>,
}

#[derive(Debug)]
pub struct ParsedStatement {
    /// The account number printed on the statement's account-number line.
    pub account_number: String,
    /// Every row the statement prints, before the date window.
    pub statement_rows: usize,
    pub closing: ExactDecimal,
    pub totals: Totals,
    pub build: Build,
    pub check: Selfcheck,
    pub counterparties: Vec<CounterpartyGroup>,
    /// The days whose every row the statement read (see [`coverage`]).
    pub coverage: Option<(Date, Date)>,
}

/// [`prepare`] over the words read from a statement PDF.
///
/// A refusal is reported as `no_readable_text` when most of the PDF's own text
/// is in render mode 3 (a scan's OCR layer, which is not read) and the rest of
/// its text yields no statement row (its parse finds none, or refuses): a scan
/// that also draws a stamp or page numbers gets the right next step, a text
/// PDF, instead of a name that points at the layout. A PDF whose text that is
/// read has rows keeps its refusal, so a request fault on a statement with a
/// scanned page appended is named as itself; and a statement that prepares is
/// never refused here.
pub fn prepare_text(
    text: &StatementText,
    request: &StatementRequest<'_>,
) -> Result<ParsedStatement, Refusal> {
    prepare(text.pages(), request).map_err(|refusal| {
        name_hidden_text(refusal, text.pages(), text.mostly_hidden(), request.bank)
    })
}

fn name_hidden_text(refusal: Refusal, pages: &[Page], mostly_hidden: bool, bank: Bank) -> Refusal {
    // the second parse runs only for a mostly hidden PDF
    if mostly_hidden && !parse_statement(pages, bank).is_ok_and(|rows| !rows.is_empty()) {
        no_readable_text()
    } else {
        refusal
    }
}

/// Pages → proven proposals, or the first refusal.
///
/// The balance and totals are proven over **every** row the statement prints;
/// the date window only narrows which rows become proposals afterwards.
pub fn prepare(pages: &[Page], request: &StatementRequest<'_>) -> Result<ParsedStatement, Refusal> {
    if request.bank.prints_totals() && request.controls.debits.is_none() {
        return Err(Refusal::new(
            "control_totals_required",
            format!(
                "a {} statement prints its debit and credit totals; supply both",
                request.bank.name().to_uppercase()
            ),
        ));
    }
    let rows = parse_statement(pages, request.bank)?;
    let account_number = require_account_match(pages, request.bank, request.account_label)?;
    let closing = reconcile(&rows, &request.controls.opening, &request.controls.closing)?;
    // `Controls` holds both totals or neither
    let totals = match (&request.controls.debits, &request.controls.credits) {
        (Some(debits), Some(credits)) => verify_against_statement(&rows, debits, credits)?,
        _ => statement_totals(&rows)?,
    };
    let build = build(
        &rows,
        request.bank,
        request.mapping,
        &BuildOptions {
            bank_ledger: request.bank_ledger,
            suspense_ledger: request.suspense_ledger,
            account_label: request.account_label,
            account_number: &account_number,
            date_from: request.date_from,
            date_to: request.date_to,
            cash_answers: request.cash_answers,
        },
    )?;
    let check = selfcheck(&build, request.bank_ledger)?;
    let counterparties = group_counterparties(&build.records)?;
    let coverage = coverage(&build.dates, request.date_from, request.date_to);
    Ok(ParsedStatement {
        account_number,
        statement_rows: rows.len(),
        closing,
        totals,
        build,
        check,
        counterparties,
        coverage,
    })
}

#[cfg(test)]
#[path = "pipeline_tests.rs"]
mod tests;
