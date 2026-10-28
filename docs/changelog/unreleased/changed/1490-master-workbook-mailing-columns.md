- The party/ledger master workbook, a local file written by the desktop app, gains five columns at its end: the mailing
  address, PIN code and state in force on the Balances-as-of date, the date they apply from, and a status that says why
  they are blank (no mailing details, none in force on that date, not observed, or unreadable). The workbook asks Tally
  for the mailing details only for a book whose master-alteration mark (an upper bound on its ledgers) is at most 3,404;
  for a larger book it sends the same request as before, every ledger's status reads not observed, and a note in the
  workbook's header says so, so a book that exported before is never refused for the larger per-ledger estimate. The
  choice is made on the mark, which counts every master alteration, so a book with few ledgers and many alterations is
  read without the mailing details too (#1490).
