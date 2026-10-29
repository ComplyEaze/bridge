- Four checks that accepted any eight digits as a Tally date now refuse a value that names no calendar day (for example
  `20251340` or `20250229`): a mirror batch's requested date range, which HSN item is in force on an invoice date,
  whether a sales ledger's GST rate is in force on an invoice date, and a company context's from and to dates. Before,
  such a value passed and, in the HSN and rate checks, could be chosen as the item or rate in force. A batch, an invoice
  build or a parsed export that carried an impossible date is now refused where it was accepted. No captured answer
  holds one, and whether Tally ever sends one in these fields is not measured; tested on invented facts (fixes #2009).
