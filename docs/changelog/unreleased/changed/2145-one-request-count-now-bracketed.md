- When `vouchers`, `voucher_presence`, `ledger_movement` or a register comes back empty for a window on a book counted
  in one census request, the second look at the window widened by a day each side now asks Tally only for the
  vouchers at or below the voucher mark read before it, and checks that mark again after its count, as it already did
  after a count of several requests (#2145). So a voucher saved while the call is reading refuses the call as
  `voucher_window_changed_during_read`, whose next step is to ask the person whether anything is being saved or
  exported in Tally and to run the same request again once. In 0.6.1 such a voucher was counted: one saved inside the
  window refused the call as `window_contradicted`, one saved on a neighbouring day left the window `partial`
  (`empty_window_wider_read_nonempty`), and one saved anywhere else left it `partial` (`empty_uncorroborated`). A count
  of several requests now compares the voucher mark alone, so a master saved during it, such as a ledger created or
  renamed, no longer refuses it; a count of one request never compared the marks. The mark read after the count is
  also the one that decides an empty window, so it is not read again. Where the widened window is empty, a count of
  one request sends the same number of requests as in 0.6.1 and a count of several sends six fewer; where a
  neighbouring day holds vouchers, a count of one request sends six more and a count of several the same.
