- `vouchers` takes `bank_allocations: true` to add, on each ledger entry, the bank date, instrument number and date,
  transaction type and status of its bank allocations, each field saying whether Tally sent it absent, empty or with a
  value; a transfer reference typed in a Contra's bank allocation is its instrument number. `status` is Tally's own
  `STATUS` element, not a reconciliation state, and nothing returned says whether an entry is reconciled. On two
  entries reconciled on Tally's screen with a bank date of 6 April 2026, Tally's raw answer filled the bank date and set
  `BANKMANUALSTATUS` and `BANKRECONSTATUS` to Reconciled, while `STATUS` stayed No; this read returns the bank date but
  neither of those two elements, and it has not been run on a reconciled entry. Without the argument, answers are as
  before. Measured on one synthetic book of five hand-keyed Contras (TallyPrime 7.2, keyed in double entry), and, for the
  reconciled entries, by a raw read of five Contras of a second synthetic book (TallyPrime 7.2); no request changed (#1231,
  #1392).
