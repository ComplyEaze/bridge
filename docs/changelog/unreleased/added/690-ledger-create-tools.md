- `prepare_ledger_creates` and `post_ledger_creates` create ONE new ledger in a company, the first write a published
  build makes to Tally's masters (part of #690). They are listed only when `BRIDGE_AGENT_ENABLE_MASTER_WRITES` and posting
  (`BRIDGE_AGENT_ENABLE_WRITES`) are both set to true; the desktop extension has no setting for the first, so a default
  install does not offer them. The ledger is a party under Sundry Debtors or Sundry Creditors, a Suspense ledger, or a
  GST duty ledger whose head has been measured, from typed fields; ComplyEaze Bridge builds the request itself, accepts
  no XML, and can set no opening balance or parent. `prepare_ledger_creates` reads the company's ledger and group names,
  refuses a name that already exists as a ledger, a group or an alias or that differs from one only in case, spaces or
  symbols (Tally can take such a Create for the existing ledger and change it), writes nothing to Tally, and returns a
  `batch_id` and the preview the person will be shown (its ledger name masked under `mask_parties`). `post_ledger_creates` asks the local user in a native dialog and sends
  the create once; a batch already sent is never sent again. After the send the book is read back and one state is
  returned, `posted_verified` only when Tally's counters show one create and no alteration, the company's own master
  mark moved, exactly one new ledger of the name exists with the parent, tax fields and GSTIN as sent, no ledger or
  group that existed before changed, and no other loaded company's master mark moved (bill-wise is not read back). Measured: one ledger created and read back as `posted_verified` through the tools on a synthetic
  company (TallyPrime 7.1 Silver, 14 Oct 2026), and the approval box shown whole, declined with nothing sent, accepted
  and then answered from the journal with no second box (TallyPrime 7.2, a continuous-integration package of an earlier
  head of the change, 239896f8, 28 Oct 2026). Not run: a create whose answer is lost, other TallyPrime releases and editions, and the build
  of this release. A name Tally resolves to an existing ledger is reported loudly and is not retried.
