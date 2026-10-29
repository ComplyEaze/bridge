- `vouchers` takes `goods_lines: true` to add, on each ledger entry of an invoice voucher, its goods lines: the stock
  item, actual and billed quantity, rate and amount, and each godown and batch row, each field saying whether Tally sent
  it absent, empty or with a value. An entry with a line whose batch amounts do not add up to its amount shows no lines, and says
  why. A Stock Journal showed no lines in the one read of it (TallyPrime 7.2, not committed); a Delivery Note or Receipt Note has not been captured. Without the argument, answers
  are as before. Measured on Sales and Purchase invoices of two synthetic books (TallyPrime 7.1, one also on 7.2); no
  request changed (#2015).
