- `stock_by_batch` returns Tally's own Stock Query for 1 to 20 stock items by GUID: each item's quantity per godown
  and batch at a month's last day, quantities only (closes #2017 on quantity, part of #2018). Each item's lines are tied as a whole to its
  closing quantity in Tally's stock item list for the same period, and its state says how (`tied`, `differs`,
  `not_comparable`, `no_stock` or `no_movement`); Tally's empty answer is never read as zero on its own. Nothing is
  compared with a voucher. A failed send stops the reads and the answer says it is partial; an answer that cannot be
  read refuses that item only. Item, godown and batch names are masked under `mask_parties`. Measured on TallyPrime
  7.2 on one synthetic book, through the tool at three month ends; a month in which
  a closing moves is not measured.
