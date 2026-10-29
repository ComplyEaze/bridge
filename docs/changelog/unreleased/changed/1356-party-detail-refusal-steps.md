- When the `outstandings` read of Tally's Bills Receivable or Payable report is refused because one of its requests took longer than the
  20-second limit, the refusal now names a next step: not to repeat the call in a loop, and that an earlier `as_of` reads fewer open bills.
  On one synthetic book of about 30,000 vouchers this happened at the end of its first year. A party detail refused by a step after its
  party was read by its ledger (the closing marks read, a book that moved, or the detail's checks) now says so in `window_scope`, and the
  next step for `party_ledger_row_without_line` says that only a bill trail can be narrowed by a `reference` (part of #1356).
