- `post_ledger_creates` can now settle a ledger create whose answer from Tally was lost even after another master change
  moved the company's master mark: a later call whose read finds the name absent, no ledger or group changed, missing or
  new since the send, and the mark moved forward, answers the new state `confirmed_absent_after_change`. The batch is
  settled and its name released; whether the create ran and its ledger was later deleted, or never ran, is not known.
  Before, such a batch stayed `reconciliation_required` for good and held its name in every company. Both absent states
  now say what they rest on (`verification.basis` `read_after_send_order`): a read sent after the create's send ended
  was served after the create, seen for requests on new connections and with the create, the read or both on a reused
  one (#1713, #2176). That was seen on TallyPrime 7.2 Silver, licensed, only, so both absent states, `confirmed_absent`
  included, are now given only there, with the product exactly TallyPrime: on another product, Gold, another release,
  Education or a Tally whose release is not known, a later call whose read finds the name absent leaves the batch open
  and its name held, with a reason naming what is not covered. Before, `confirmed_absent` was given on any Tally. **This
  changes behaviour on TallyPrime 7.1, Gold and Education:** a batch there whose answer was lost and whose name a later
  read finds absent no longer settles, and holds its ledger name in every company until the company is read back on
  TallyPrime 7.2 Silver; calling again on the same Tally will not close it. A build from before this change cannot read
  a masters journal that holds the new state: it refuses the whole journal, which stops every master write in that build
  until a later build is installed again.
