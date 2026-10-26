- `negative_ledgers` (new tool; part of #1358): Tally's own Negative Ledgers list for a window, as
  `ledger`, `side` and `amount` per listed ledger, returned as Tally lists it and compared with no other report
  (`checks` says so). An empty list is `not_established`, never "no negative ledger": Tally's empty answer cannot be told
  from a report that was not rendered. Measured on three synthetic books, each for one whole financial year (on the first, a creditor with a
  debit balance and a debtor with a credit balance were listed and a Suspense ledger with a credit balance was not; on the
  other two, the one ledger listed matched the trial balance); any
  other window is named as unmeasured, and the cost on a large book is not known. Not yet run against Tally.
