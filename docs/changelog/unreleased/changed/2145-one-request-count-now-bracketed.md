- The second look at an empty window, widened by a day each side, now checks the book's voucher mark after a count
  that took one request, as it already did after a count of several (#2145). Every count of a window's vouchers asks
  Tally only for the vouchers at or below the voucher mark read before it, so a voucher saved while `vouchers`,
  `voucher_presence`, `ledger_movement` or a register is reading would otherwise go unseen. When the voucher mark has
  moved, the call is refused as `voucher_window_changed_during_read`, whose next step is to ask the person whether
  anything is being saved or exported in Tally and to run the same request again once; before, a count of one request
  answered `empty_uncorroborated`. A master saved in that time, such as a ledger created or renamed, no longer refuses
  any count, of one request or several: a count compares the voucher mark alone. The mark read after the count is also
  the one that decides an empty window, so it is not read again. Where the widened window is empty, a count of one
  request sends the same number of requests as before and a count of several sends six fewer; where a neighbouring day
  holds vouchers, a count of one request sends six more and a count of several the same.
