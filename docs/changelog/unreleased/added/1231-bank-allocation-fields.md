- `vouchers` takes `bank_allocations: true` to add, on each ledger entry, the bank date, instrument number and date,
  transaction type and status of its bank allocations, each field saying whether Tally sent it absent, empty or with a
  value; a transfer reference typed in a Contra's bank allocation is its instrument number. Nothing in them says
  whether an entry is reconciled: no reconciled entry has been captured yet. Without the argument, answers are as
  before. Measured on one synthetic book of five hand-keyed Contras (TallyPrime 7.2, keyed in double entry); no request changed (#1231,
  #1392).
