- A Sales invoice now reads the ledger listing for its own ledgers too, so a company with
  a long history of master changes, or with up to 20,000 ledgers, is no longer refused for an
  invoice: the master-mark and ledger-count limits and the refusals `invoice_book_too_large`
  and `invoice_book_size_unreadable` are gone. A company past 20,000 ledgers is refused as
  `master_catalog_too_large`, as for every voucher, and the 25,000 voucher-mark limit stays.
  An answer holding a ledger the invoice does not name is refused as
  `invoice_ledger_listing_rows_differ` (#1331).
