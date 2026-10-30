- A Sales invoice's tax-rate read now asks Tally for the invoice's own ledgers, by the
  identifier Tally gave each, instead of every ledger under their groups, so a customer
  among thousands under Sundry Debtors no longer makes that read the whole book. A ledger
  moved to another group between the approval and the post still refuses the post. The
  refusals `invoice_scope_parent_folds` and `invoice_ledger_scope_too_large` are gone, and
  `invoice_ledger_guid_unnameable` is new. The ledger listing beside it and its book-size
  limits are unchanged for now (#1331).
