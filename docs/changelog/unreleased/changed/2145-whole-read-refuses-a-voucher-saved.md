- A window read whole on a book counted in one census request (by `vouchers`, `voucher_presence`, `ledger_movement` or
  a register) now asks Tally to count only the vouchers at or below the voucher mark read before it (#2145). So a
  voucher saved in the window between that mark and the count is not counted, the read returns it all the same, and
  the call is refused as `voucher_window_part_not_admitted` with the cause `part_census_mismatch`, as a read in several
  parts already was. Its next step is to run the same request again once, while the book is quiet. In 0.6.1 the count
  included such a voucher and the read came back `complete`.
