- A Sales invoice build now refuses a customer whose ledger holds the company's own GSTIN
  (`invoice_party_gstin_is_company_own`), before anything is written, and the re-read before a post refuses it the
  same way: Tally lists such an invoice as uncertain in GSTR-1 (#2162).
