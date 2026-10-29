- The `vouchers` snapshot shows `voucher_alter_id`, and the audit read's manifest `alter_voucher_id`, as null,
  not 0, when Tally's answer carries no voucher mark, which it omits for a company that has never held a
  voucher; `ledger_masters` already showed null for the same book. A mark Tally sends, 0 included, is shown as
  sent. Every comparison of the marks still reads an absent voucher mark as 0, so no read or post decides
  differently (fixes #1984).
