- The `vouchers` tool description and the agent guide say that a row's `party` is Tally's own
  `PARTYLEDGERNAME`, which on a Receipt, Payment or Contra can name the bank or cash ledger rather than the
  counterparty, and that the counterparty of those vouchers is read from `amounts`. The parse is unchanged
  (fixes #1986).
