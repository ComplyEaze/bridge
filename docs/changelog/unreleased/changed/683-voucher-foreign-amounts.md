- `vouchers` now reads a foreign-currency amount instead of withholding its voucher, on TallyPrime 7.1 and 7.2: the
  rupee amount is the entry's `amount`, and `foreign` holds the currency, foreign amount and rate. A voucher it cannot
  read that way stays in `withheld_vouchers`. Registers and `ledger_movement` still refuse such a window (#683).
