- `balance_sheet` and `profit_and_loss` now say when Tally's own Balance Sheet does not balance: on the one book
  measured (a stock item's opening value edited, TallyPrime 7.2) the export left out the `Difference in opening
  balances` line Tally's screen showed, so when its lines do not sum to zero the result carries
  `tally_balance_sheet_sides` (the credit and debit totals), the first line gives both and nothing is established;
  the new reason `tally_balance_sheet_does_not_balance` is given when no other reason applies (#2063).
