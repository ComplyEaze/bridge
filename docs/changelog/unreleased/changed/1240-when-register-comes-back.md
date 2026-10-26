- When `vouchers`, `voucher_presence`, `ledger_movement` or a register comes back empty for a window on a large
  book, the second look at the window widened by a day each side now counts it instead of reading its vouchers,
  so it usually sends fewer requests (#1240; the same number where the count takes several requests and the
  widened window is small). The labels do not change: an empty window with vouchers on a neighbouring day is
  still `partial` (`empty_window_wider_read_nonempty`), and a voucher inside the window still refuses the read.
  What differs: a count that took several requests is refused (`voucher_window_changed_during_read`) when the
  company's marks differ from the ones the first read opened on, as a divided read is; a count that took one
  request is not bracketed, where a widened window that used to be read in several parts was; and because no
  vouchers are read, a refusal that came from reading them (a part that failed or was not admitted, a part
  Education cannot serve, a read plan that could not be made or went over its allowance, a disagreement between
  the count and the read) no longer happens. A count of several requests can still be refused when it passes
  its own allowance. A book that fits one request is read as before. The request counts come from test doubles,
  not from a live Tally.
