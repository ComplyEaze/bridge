- `ledger_movement` now reads a book with several Currency masters when `from` is the first day of the books (part of
  #683), through the INR base Tally identifies; until now every such book was refused. Only its plain base-currency
  ledgers are read. A ledger is set aside, by name and with no figure, when the currency-scoped Trial Balance names it
  as kept in or showing a value in another currency; a composite opening at the first day or a composite entry in the
  window on a ledger the Trial Balance reads as plain refuses the read with `ledger_currency_scope_inconclusive`. The result is always `partial` (`partial_reason` `base_currency_ledgers_only`)
  and carries `trial_balance`'s `ledgers_scope`, `foreign_currency_ledgers_excluded` and
  `base_currency_ledgers_mixed_excluded`. A `ledger` that is set aside is refused with `ledger_currency_set_aside`, and a
  Trial Balance and catalogue that name different ledgers with `ledger_currency_scope_unmatched`. A later `from` is
  still refused, because from a later day a dollar balance can read as a plain rupee figure. Measured on one synthetic
  book on TallyPrime 7.2.
