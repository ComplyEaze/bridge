- `prepare_stock_item` and `post_stock_item` create one new stock item, a copy of an existing Goods, taxable item
  (`sibling`) under a new `name`, and are listed only when master writes and posting are both on, as the ledger pair
  is (part of #1381). `prepare_stock_item` reads the company's stock items, groups and units, refuses a name an item
  already holds or that differs from one only in case, spaces or symbols, and a sibling it cannot copy, and writes
  nothing to Tally. `post_stock_item` reads them again, asks for approval in a native dialog, sends the create once
  and reads the book back: `posted_verified` only in the call that sent it, otherwise `masters_after_post_unconfirmed`
  or `reconciliation_required`. A cancelled call is judged from the stock journal, so a call whose send is recorded is
  never treated as unsent. The create's request shape was measured once, by a lab script, on one synthetic book on
  TallyPrime 7.2; these tools have not been run against a real TallyPrime.
