- The `outstandings` party detail can now answer for a small party on a book too large to read whole in one request. Where the book's voucher
  mark is above 42, a party whose vouchers carry its ledger's name as stored, whose name holds only ASCII letters, digits and single spaces
  and no TDL operator word, and that no other ledger shares but for case or spaces, is counted by its own ledger; when its vouchers fit one
  request they are read by that ledger and checked against the count voucher by voucher, instead of reading the whole company's vouchers.
  Such a detail's `window` says `party_vouchers_read`, the party's vouchers read; every other detail keeps `company_vouchers_read`, the
  company's, and is read as before. If that read is refused after the party was counted, the refusal also says `party_vouchers_at_least`
  (part of #1356).
