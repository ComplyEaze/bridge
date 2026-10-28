- The `outstandings` party detail can now answer for a party on a book whose window is too large to read. Where the read of the whole
  company's vouchers is refused as needing too many requests, a party matched to one ledger alone, whose vouchers carry the ledger's name as
  stored and whose name holds only ASCII letters, digits and single spaces and no TDL operator word, is counted by its own ledger and its
  vouchers are read by that ledger in planned parts, each checked against the count voucher by voucher. Such a detail says so in
  `window_scope` (`basis` `ledger_scoped`, with a note on what that list was compared with) and counts `party_vouchers_read`; every other
  detail is read as before and keeps `company_vouchers_read`. Where the ledger is not read, or its read finds nothing or would need a day
  divided by AlterID, the original refusal stands and its `window_scope` gives the reason; a refusal of the ledger read itself (a part not
  admitted, a row with no line on the ledger) keeps its own code (part of #1356).
