- The `outstandings` party detail can now answer for a party on a book whose window is too large to read. Where the read of the whole
  company's vouchers is refused as needing too many requests, a party whose vouchers carry its ledger's name as stored, whose name holds only
  ASCII letters, digits and single spaces and no TDL operator word, and that no other ledger shares but for case or spaces, is counted by
  its own ledger and its vouchers are read by that ledger in planned parts, each checked against the count voucher by voucher. Such a
  detail's `window` says `party_vouchers_read`, the party's vouchers read; every other detail is read as before and keeps
  `company_vouchers_read`, and a party whose ledger read finds nothing, is not admitted, or would need a day divided by AlterID keeps the
  original refusal (part of #1356).
