- `voucher_presence` now reads a window that holds a foreign-currency amount, as `vouchers` does, instead of
  refusing it: proposals are matched on the rupee amount. A voucher it cannot read that way refuses the call as
  `foreign_currency_amount_unparsed`. Registers and `ledger_movement` still refuse such a window (#683).
