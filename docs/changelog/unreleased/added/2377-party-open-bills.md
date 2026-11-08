- `outstandings` takes `detail` `open_bills` with `party`: that party's open bills at `as_of`, read through Tally's
  Ledger Outstandings for that one ledger, with no voucher read and no whole-book Bills report (part of #2377). The
  bills are told only when they sum, On Account rows included, to the ledger's closing balance read from the books'
  first day to `as_of`; otherwise the call refuses (`party_bills_untied`), and an answer with no bytes, which is how
  Tally answered a name it did not match, refuses as a transport failure, never as "no bills". The party is resolved from the ledger catalogue as the other party
  details resolve it, and the book's own name is sent only if it is ASCII letters, digits and single inner spaces;
  any other name is refused before anything is sent (`party_name_not_admitted`). The company's marks are read before
  and after, and must be unchanged. Measured with curl on TallyPrime 7.2 on one synthetic book: a receivable party at
  two dates, and parties with one bill each; Bridge's own build has not yet been run against a live Tally. Not
  measured: a payable party, a party with bill-wise off, a foreign-currency ledger, a party with no open bills.
