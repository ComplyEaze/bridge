- A Sales invoice build now admits a sales ledger whose GST rate rows carry a per-unit rate of 0 and a cess rate
  of 0. On a lab book Tally answered so for two sales ledgers after their HSN/SAC codes' voucher lists were
  opened from GSTR-1's HSN summary, until the company was shut and loaded again; before, the build refused such
  a ledger with `invoice_sales_ledger_rate_shape_unmeasured`. The re-read before a post applies the same rule. A
  per-unit rate other than 0, or one on some rate rows only, is still refused, and so is a tax ledger whose rows
  carry the field (#2162).
