- A repeated `vouchers` call for the same company, dates and book now reuses the count of the window's vouchers (#2399)
  from the call before it, for up to ten minutes, while Tally's two change marks, read afresh by the new call, have not
  moved. It skips the census requests, 6 for a book counted in one request, and the answer's `window.census_reused`
  says when the count was taken. If the vouchers read are not the ones that count described, the call is refused as
  `census_reused_mismatch`, the count is dropped, and the next call counts afresh. The saving is counted from
  replayed answers; it has not yet been run against a live Tally or measured on a real book.
