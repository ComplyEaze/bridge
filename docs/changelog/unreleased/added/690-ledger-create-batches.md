- `prepare_ledger_creates` takes `ledgers`, two to 10 new ledgers of one kind and side, in place of `ledger`, and
  `post_ledger_creates` creates them together: one native dialog lists every ledger and names their count, and one
  approval sends them to Tally in one request (part of #690). A list that mixes parent groups, or holds two names
  that differ only in case, spaces or symbols, is refused before anything is saved. The batch is judged as one:
  `posted_verified` needs every ledger new and as sent, one create counted per ledger and nothing altered, and the
  company's master mark moved by exactly one per ledger; `verification.guids` gives each ledger's GUID in order. A
  batch whose answer from Tally was lost is never `posted_unattributed`. Measured in one request on TallyPrime 7.2
  (one synthetic company, once each): three and ten ledgers, each counted one create per ledger, the master mark
  stepped by the batch's size, and each new ledger took the next mark value in request order. Not measured: other
  releases and editions, and the box for a batch on screen. On Windows a box that is not wholly on the screen is
  refused with nothing approved; whether a box of ten fits the lab's screen has not been tried.
