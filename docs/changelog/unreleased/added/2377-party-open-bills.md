- `outstandings` takes `detail` `open_bills` with `party`: that party's open bills at `as_of`, read through Tally's
  Ledger Outstandings for that one ledger, with no voucher read and no whole-book Bills report (part of #2377). The
  bills are told only when they sum, On Account rows included, to the ledger's closing balance read from the books'
  first day to `as_of`; otherwise the call refuses (`party_bills_untied`). Tally sends no bytes for a party it reports
  nothing for, both for a name no ledger carries and for a bill-wise party with no balance at the date (each seen on
  one synthetic book and one party on TallyPrime 7.2), so the ledger
  list decides: a name it does not carry refuses as `party_not_in_book`, a ledger that keeps no bills as
  `party_not_billwise`, a bill-wise party with no balance as `party_bills_none_reported` and one with a balance as
  `party_bills_answer_missing`, never as "no bills"; only that exact empty answer, twice, is read so. The party is resolved
  from the ledger catalogue as the other party details resolve it, and the book's own name is sent only if it is ASCII
  letters, digits and single inner spaces; any other name is refused before anything is sent
  (`party_name_not_admitted`). The company's marks are read before and after, and must be unchanged. Measured with
  curl on TallyPrime 7.2 on one synthetic book: a receivable party at two dates, and parties with one bill each;
  Bridge's own build has not yet been run against a live Tally. Not measured: a payable party, a party with bill-wise
  off, a foreign-currency ledger, a party with no open bills. Until that last answer is measured, an answer with no
  rows refuses (`party_bills_no_rows_not_established`). The answer carries `ledger_match`, which says when the party
  asked for was bound by case or spacing.
