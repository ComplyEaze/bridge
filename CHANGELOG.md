# Changelog

All notable changes to ComplyEaze Bridge are documented here. The project follows
[Semantic Versioning](https://semver.org/).

## [Unreleased]

Published builds are MCPB packages that are not yet code-signed (tags
`mcp-preview-*` and, from 0.4.0, `mcp-v*`): so far
`mcp-preview-0.2.0`, `mcp-preview-0.3.0`, `mcp-v0.4.0` and `mcp-v0.4.1`. The
number of the next build is chosen when it is released.
The version boundary between the published MIT-licensed `v0.1.0` release and
Apache-2.0 builds from current source stays unambiguous.

### In plain words: the next build, since `mcp-v0.4.1`

These changes are in source and not yet in a published build.

**What the next build adds**

- Read a register of the tax in the books for sales: the Sales and Credit Note
  vouchers of a date window that touch a Duties & Taxes ledger, with each
  entry's tax taken only from the GST duty head on that ledger's master, never
  from a name or an amount. It reads as the purchase register does. It was run
  against a live Tally for one taxed Sales item invoice, one untaxed one (read
  once by an earlier build; only its voucher window is committed) and one Credit
  Note, on two synthetic companies. A Credit Note keeps Tally's signs
  (nothing is netted or flipped, so add signed amounts), and the state-side tax
  head has two recognised forms, `state_tax` and `sgst_utgst`, for the same
  side. An inter-state line, a cancelled or optional sale, an unrecognised or
  missing duty head, more than one voucher in a window and paging were not
  shown. A sale of a kind a row can show as unmeasured is marked
  `not_measured_live`; the tool's text says what cannot be marked. The response
  `state` is `complete` when the company marks and the ledger masters read the
  same before and after the window, not when the window was counted as
  `vouchers` now requires (#1009).

**Safer or fixed**

- `vouchers` no longer re-reads a whole window for every page. A later page of a
  `complete` window is served from the first page's read while the company's
  books have not changed in a way that moves their marks, and a page that names
  the first page's `snapshot_id` is refused if they did, instead of continuing
  from a different read. Without the name, a page that reads afresh (the held
  window moved on, expired or was dropped) has offsets that may not continue the
  earlier pages, which the result says when it knows (`earlier_snapshot`). Before,
  a book that changed between pages could skip or repeat vouchers while every
  page said `complete`. A window that is only `partial` is read again for each
  page, as before (#485).
- `outstandings` no longer refuses because one bill's due date in the Bills
  Receivable report is printed with a four-digit year (a bill with a credit period
  of about 83 years, `1-Dec-2108`): that one date made the whole read fail with no
  reason. Not measured: that the rest of the read then completes on such a book. A
  due date is now read as written when Tally prints its year in full and the year
  is 2100 or later. A bill row whose dates ComplyEaze Bridge cannot read still
  refuses the read; the refusal now carries a typed `cause`, the report and the
  row number when it is a row that fails, and a next step, never the bill's party
  or reference. A due date that far out gives no overdue days in the ageing (#1091).
- In the desktop app, the button on an export notice that shows the file
  (Show in Finder, Show in Explorer or Open folder) now opens only a file
  ComplyEaze Bridge exported since it started. Any other path is refused
  before the file manager opens, so a compromised app window cannot use it to
  point at other files on the computer. Exports work as before (#915, #1051).

## [0.4.1] - 2026-10-02

### In plain words: ComplyEaze Bridge 0.4.1, since 0.4.0 (2 Oct 2026)

These changes are in ComplyEaze Bridge 0.4.1. Each line names the pull
requests it comes from, except where it names an issue.

**Should I upgrade?**

- **If you use 0.3.0 or 0.4.0 on Windows: yes.** 0.4.1 closes the network-path
  forms (a share, its WebDAV form, and the long-path prefixes) of a
  medium-severity security issue in how the bank-statement tool opens file
  paths, published as
  [advisory GHSA-vm5g-r3p7-wxx7](https://github.com/ComplyEaze/bridge/security/advisories/GHSA-vm5g-r3p7-wxx7).
  Some paths still pass; the advisory lists which. If you cannot upgrade yet,
  follow the advisory's steps.
- **On a Mac: recommended, though not urgent.** The advisory rates the issue
  low there, and the other changes below apply there too.
- **How:** ComplyEaze Bridge does not update itself. Follow the
  [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md):
  (1) close any other program that runs ComplyEaze Bridge; (2) in Claude
  Desktop, install the newer file from Settings, Extensions; (3) keep
  ComplyEaze Bridge's data folder, which holds its record of what it has sent
  to Tally; (4) check that the extension shows 0.4.1 and that "Allow voucher
  posting" is as you want it, since an earlier default may still be saved as
  on; (5) quit Claude Desktop completely and reopen it.
- **What was tried** is under "Known limits" below. No one on our side
  installed the Windows package of this build in Claude Desktop on a Windows PC.

**What you can do now**

- No new tool. On a company large enough to be counted first,
  `voucher_presence` can now answer `absent` for a voucher it finds nowhere in
  a date range that holds vouchers and was checked against that count. An empty
  range is called absent only on a company that has never held a voucher, as
  before (#985, #1020).

**Safer or fixed**

- `parse_bank_statement` accepts a statement file or password file path only
  when its text names a local disk. A path that starts with two separators
  (two slashes, two backslashes or one of each), such as the Windows long-path
  form (starting with `\\?\`) or a network share (`\\computer\folder`), is
  now refused: copy the file to this computer and give its path there (#1024).
- `vouchers` and `voucher_presence` now call a date range with vouchers in it
  complete by one rule: only when every voucher read was checked against a
  separate count of that range. Before, `vouchers` called any such range
  complete (unless it withheld a voucher), and `voucher_presence` called none
  complete. On a small or new company, which ComplyEaze Bridge does not count
  first, both now say `partial` (#985, #1020).
- A voucher changed in Tally between the count and the read now refuses the
  read (`voucher_window_part_not_admitted`, cause `part_census_mismatch`),
  also when the range is read in one request, where it used to return without
  that check. Call again once while the book is quiet; if it repeats, read the
  range in Tally. `ledger_movement`, `purchase_register`, `build_import_xml`,
  verifying an import and the party detail of `outstandings` can give the same
  refusal. After a post was sent, this refusal on the read-back means the
  voucher may already be in Tally: use `verify_import` on the same batch and
  never post it again. In some cases, verifying an import for one day that
  Tally cannot serve in one request is now read in parts instead of refused,
  which sends more requests to Tally (#985, #1020).
- When `post_import` refuses a batch before it is sent because some of its
  rows may already be in the book (`import_preexisting_identity`), the answer
  now names those rows by their transaction ids and says what to check in Tally
  next, instead of returning a bare code, unless the response size limit
  leaves no room for the list. The same batches are refused as before (#901,
  #908).
- `vouchers` returns a voucher's `master_id` (Tally's internal voucher id) as
  the plain number (`"1"`), not as Tally sends it with a leading space
  (`" 1"`), so it matches the same id returned by `verify_import` (#989, #1021).
- If voucher posting is turned on, you approve a batch in its approval dialog,
  and the post is then refused before it is sent (for example because the
  import journal is busy, the batch is not found, or the call names a
  different company from the batch's), your approval is withdrawn, and the next
  post that gets past those checks asks you again. Before, in these cases the
  approval stayed held for up to 15 minutes, and posting any other batch was
  refused until it was used or lapsed (#857, #904).

**Known limits**

- A date range is called complete when the vouchers read match a separate
  count of the range. If Tally's own date selection leaves a voucher out, the
  count and the read both miss it, and the range is still called complete. A
  small or new company is not counted, so it cannot get `absent`; adding a
  count for it is tracked in #1029, with the multi-day and after-post cases
  (#985).
- `parse_bank_statement` checks the text of the path. A mapped drive letter
  still passes, and so, on a Mac, does a path under a mounted network volume
  (#1024).
- A batch refused just before it is sent can still return no rows and no next
  step. The rows named before that can be more than the vouchers the book
  holds, when several rows look the same; count the vouchers in Tally before
  leaving any row out. A row is still refused when a voucher that an earlier
  batch or a hand entry put in the book has the same date, type, ledgers,
  amounts and sides (#865, #901).
- Tried against TallyPrime 7.1 Silver in a lab on 2 October 2026, before
  #1020 merged: one-day ranges on two synthetic books, and the read-back of one
  posted Journal. Not tried: a range of several days read in one request, the
  new refusal itself (seen only in tests), how often it appears on a busy book
  with several users, and other Tally editions. The other changes have not been
  run against TallyPrime; each is covered by automated tests. No one on our
  side installed the Windows package of this build in Claude Desktop on a
  Windows PC. The release check starts each package, lists its tools and reads
  a sample bank statement, and does not run against TallyPrime.

## [0.4.0] - 2026-10-02

### In plain words: ComplyEaze Bridge 0.4.0, since `mcp-preview-0.3.0` (26 Sep 2026)

These changes are in ComplyEaze Bridge 0.4.0. Each line names the pull
requests it comes from, except where it names an issue.

**What you can do now**

- Read a company's masters as a list: voucher types (with their numbering
  method), godowns, units, stock groups, or ledger groups, in pages. The book is
  checked before and after the read. For godowns, units and stock groups, a book
  with very many masters is refused, with the size named, rather than answered
  in part (#952).
- See what a party's unallocated amount is made of, from data the tool already
  reads, and what that data cannot tell apart. Each bill now carries its own
  date and credit period, and payable and receivable follow the sign of the
  bill's balance (#945, #946, #957, #959, #961).
- See what ComplyEaze Bridge keeps on this computer. `local_data_report` (and
  `bridge_mcp --local-data-report` on the command line) counts files, bytes and
  the age of the oldest file by kind, and gives the state of the import journal
  and how many saved batches are not settled. It changes no book and deletes
  nothing; like every call, it records its own receipt line in the local log. It
  names no file path unless you ask on the command line (#925).
- Read the purchase register of tax in the books: the Purchase and Debit Note
  vouchers of a date window that touch a ledger under Duties & Taxes, with the
  tax amount each entry records under the ledger's GST duty head. Other voucher
  types that touch those ledgers (Sales, Journal, Payment) are listed apart with
  exact counts (at most 100 are listed), and an entry whose ledger has no recognised GST head is listed and
  never given one. Nothing is posted and nothing is inferred: whether an entry
  belongs in a return is for you to decide (#971).
- Ask `outstandings` for one party, with `detail`, to see why a bill is open and
  what an unallocated amount holds, party by party. A bill trail lists the
  allocations of each of the party's bills, oldest first, leaving out those in
  cancelled and optional vouchers, and says whether they
  add up to Tally's own balance for the bill. The unadjusted view lists the
  party's on-account, advance and pending credit or debit note allocations and
  compares the on-account total with its unallocated amount; a note is
  recognised only when its voucher type is named exactly Credit Note or Debit
  Note. A bill that does
  not tie, or whose identity is ambiguous, is shown as such and nothing is
  merged. The detail reads the company's vouchers from the start of the books,
  so a large book can be refused, with a next step (#981).
- Read Profit and Loss and Balance Sheet. A figure is shown only when it ties
  line for line to Tally's own statement. Otherwise ComplyEaze Bridge's own
  statement is withheld; Tally's own amount for each line is still returned,
  with the derived amount on each line that differs, and the lines that differ
  are named. A book with stock items is expected to be
  refused, because no such book has been measured, and the tie-out itself has
  been measured on two synthetic books only (#774).
- Read closing stock values per item. An item is returned only when the values
  add up to Tally's own Stock Summary and ComplyEaze Bridge read exactly as many
  items as Tally's own count; otherwise no item is returned, with the reason and
  the next step. A company with no stock items is told so. A value keeps
  Tally's sign, as in the Trial Balance: stock held is a negative number.
  Quantities are not returned, because nothing checks them, and names, parents
  and units are returned but not checked. Only a 31 March and small books are
  read (#980, #979, #1001).
- Read books that define more than one currency. Outstandings set aside
  foreign-currency ledgers, and a rupee ledger with a foreign-currency
  balance, and name them. Compliance ledgers and the Trial Balance are read
  through the book's base currency. The `vouchers` tool withholds a voucher
  whose amount Tally stored in a foreign currency instead of refusing the
  whole date window. The desktop app follows the same read (#642, #647, #649, #715,
  #781, #824, #825).
- Read a book too large for one read, in parts, including ledgers under parents
  that cannot be named, and have the ledger count cross-checked against the
  company's own count. A movement read names an oversized ledger catalogue and
  refuses an unknown ledger before it reads any voucher (#679, #885, #891, #936,
  #938, #939, #960).
- Recognise SGST/UTGST as a GST duty head (#968).
- The app and the extension carry the ComplyEaze Bridge tick logo as their icon
  (#950, #978).
- A ledger read on a several-currency book, or one whose base currency is not
  INR, is refused before any request to Tally, instead of returning bare
  numbers (#751).
- A recognised bank-statement cash line is now asked its purpose. An
  unanswered line blocks the import file; "don't know" posts to a suspense
  ledger, tagged and listed. Other unmapped cash lines still go to suspense,
  tagged and counted (#817).

**Safer or fixed**

- `profit_and_loss` and `balance_sheet` no longer open with `"state": "observed"`
  when nothing was established. The top-level `state` is `observed` only when
  the tool's result is established; otherwise it is `not_established`, with
  the same `reason` as the nested result. This changes the tool's output; no
  figure, check or withheld line changes (#984).
- Every tool is refused until the Terms of Use are accepted, and the extension
  carries a privacy policy (#943).
- The party-master balance snapshot ends at today's date, not at a stray voucher
  date (#896).
- The Markdown proof shows the verification status and any duplicate vouchers in
  the batch (#811).
- ComplyEaze Bridge sends one request to Tally at a time, across all its
  running processes (#883).
- Each send to Tally is recorded in the local log with its place, kind, size,
  outcome and time, and no book content (#918, #941).
- ComplyEaze Bridge refuses to build or post a row that another batch already
  sent to Tally (#876, #898).
- A bank statement's result returns no amount except the figures you supplied
  and an open cash line's amount, and names the saved proposals file by its id
  alone (#848, #850).
- The configured redaction also applies to ledger names in posting and
  verification results (#851).
- The native posting windows say ComplyEaze Bridge (#970).
- Each package's build is attested, and the attestation is checked before it is
  published (#923).
- If you are slow at the approval window, the agent's call no longer waits
  on it. The agent is told the approval is pending and asks again. A click
  made while no call is waiting is posted by the next call, not one call later
  (#792, #854).
- The approval window for a batch names how many vouchers it covers, in its
  title and its button. The review window shows a debit the way the post
  window does (#757, #762).
- A voucher you cancel in Tally after ComplyEaze Bridge posted it reads back
  as posted but not effective, not as changed. Cancelled vouchers are no
  longer reported as duplicates of each other (#771, #789).
- A recorded review can no longer answer for a different doubt than the one it
  covers (#755, #769, #809, #813, #831).
- Tally's own error text on a rejected line is read safely, including text
  with an `&` in it (#763).
- Each tool now says whether it changes anything. Reads are marked read-only
  and say they only record local receipt lines for the call, never book
  content. Some assistants may now run the read tools without asking each
  time (#909, #921).
- The tools that save a file on this computer are marked as writes: preparing
  an import file and reading a bank statement add new files, while
  verification and acknowledgement replace the batch's saved proof and are
  marked destructive. Posting stays marked destructive, and no tool is marked
  as reaching outside this computer. `parse_bank_statement` had been marked
  read-only by mistake (#909, #921).

**Known limits**

- Stock: the closing-value total is checked against Tally's own Stock Summary on
  one synthetic book, and the item count against the rows on one synthetic
  company, and the sign of a value was measured on one synthetic company. A
  book with many stock items is refused: the read is for small books, and
  typical stock-heavy books refuse today. A book in which no item carries a value
  returns nothing. A book with
  stock items is expected to be refused by Profit and Loss and Balance Sheet,
  because no such book has been measured (#774, #980, #1001).
- A book with very many masters is refused by `masters` for godowns, units and
  stock groups, and how common that is across real books has not been measured
  (#952).
- `local_data_report` has not been run on a Windows host, and how it reports a
  Windows junction is unverified. It does not cover the desktop app's other
  settings, its database or its logs (#925).
- The purchase register has been read from a disposable synthetic book only. A
  purchase typed on screen, item invoices whose purchase ledger sits in an
  inventory allocation, and books with several currencies are not covered, and
  it does not decide whether a Debit Note is a purchase return or a debit note to
  a customer (#971).
- The party detail of `outstandings` has been measured on one synthetic book.
  Its cost on a large book is not measured, and a window that needs too many
  requests is refused (#981).

**Also in source**

- The tax-audit engine gained its stock and party-monthly tests. The engine is
  still not in the extension, and its accuracy is not yet proven publicly (#738,
  #744, #788).
- The desktop app moved to a Tauri release that fixes GHSA-w28w-mhc8-qvjv
  (#805).

**Removed**

- The unfinished document-sync feature is gone from the source, and so from
  both the desktop app and the binary the Claude Desktop extension runs: the
  AXAL sign-in, the code that scanned a folder and uploaded the files you chose
  to ComplyEaze cloud storage, and the two hidden screens for them. No ComplyEaze Bridge
  tool could reach them, and no navigation led to them. After this change the
  only network client in ComplyEaze Bridge's own code connects to Tally on your own
  computer. The CI egress gate fails if a first-party crate other than the Tally
  transport depends on reqwest in its shipped dependencies (the app crate keeps
  it only as a dev-dependency) or if any first-party crate depends on hyper. It
  also counts the calls that send, connect or start a process in library and
  binary code, and refuses any outside the reviewed Tally files; test code is
  not counted. The unused delivery types in the portable core crate (no network
  code) went with it. The bulk party-statement export no longer records a hash
  for each file it writes and no longer refuses to write a file it cannot
  record; a record file left by an earlier development build is never read. The
  feature can be rebuilt from the git history if it is needed
  again (#914).

## [0.3.0] - 2026-09-26

### In plain words: `mcp-preview-0.3.0` (26 Sep 2026), since `mcp-preview-0.2.0` (16 Sep 2026)

These changes are in the published `mcp-preview-0.3.0` build. Each line names
the pull requests it comes from, except where it names an issue.

**What you can do now**

- Post a Payment, Receipt or Contra voucher, as well as a Journal. Each one
  still waits for your approval in a separate ComplyEaze Bridge window
  (#585, #600).
- Read password-protected bank statement PDFs into voucher proposals, including
  Union Bank of India statements (#444, #481).
- See a party's GSTIN as it stood on a date you choose, from its dated
  registration history (#657, #700).
- See each ledger's group ancestry, and the date its opening balance is as of
  (#498, #499, #569).
- See the post-dated flag, invoice and reference details, and the party GSTIN
  on voucher reads (#498).
- Pick vouchers by voucher class the way Tally classifies them (#663).
- Work with books of more than 1,000 ledgers in the ledger catalogue (#643).
- Get a whole `verify_import` verdict at once, with the verified vouchers in
  pages (#673).
- Source builds only: with `BRIDGE_AGENT_ENABLE_BATCH_POST` on as well as
  posting, post 2 to 50 vouchers of one saved batch after one approval, which
  shows a summary by ledger. It is off by default and not in the extension
  until a live batch post is proven. A batch counts as clean only when Tally's
  counts match exactly; otherwise you review it (#712, #721).

**Safer posting**

- Voucher posting is off by default in the Claude Desktop extension. Turn it
  on in the extension settings (#577).
- An approval counts only when the approval window returns a fresh one-time
  token. How the window closes no longer decides it (#665, #704).
- ComplyEaze Bridge confirms the company as its last step before posting, and
  reports where the voucher landed. A post can still land in a company that was
  renamed after that check (#607, #574).
- It refuses to post if a ledger changed since the voucher was prepared, if
  the company's masters changed between its final checks and the post, or if
  the saved file no longer matches its record (#578, #615, #616).
- If the company's masters change while a post is landing, the post is
  reported as not verified and needing reconciliation, so you check it in
  Tally (#623).
- It records each voucher's import identity before sending it, and never sends
  one twice: it refuses an import identity its own journal already records
  (#582, #678).
- It refuses to post into a book with more than one currency defined (#613).
- When Tally rejects a line, you see Tally's own error text, kept short and
  safe to display (#695).
- When you correct a voucher it built, ComplyEaze Bridge checks the
  correction against the book as it prepares the file. It does not post
  corrections itself; you import the file in Tally (#439, #620, #639, #660).

**Clearer answers when it cannot answer**

- A refusal now names its cause instead of a generic failure. Causes include:
  - a read that ran out of time (#493);
  - an empty book (#489);
  - a missing company identity (#514);
  - no reply from Tally (#659);
  - a foreign-currency opening balance, or a book with more than one currency
    (#606, #706, #720);
  - a voucher type that doesn't exist (#677);
  - a list Tally would not serve (#719);
  - two ledgers whose names differ only by a hidden line break (#708).
- Education-mode Tally: ComplyEaze Bridge refuses a read that Education would
  return empty without saying so (#588, #598).

**Big books**

- ComplyEaze Bridge splits a long voucher window when Tally cannot serve it in
  one request, and sizes each request by expected response size, not date span
  (#494, #506).
- The `vouchers` tool reports how long each request to Tally took (#608).

**Tax-audit engine (in development, not in the extension)**

- A Rust tax-audit engine, `bridge-tax-audit`, now covers:
  - cash payments and receipts (s.40A(3), s.269ST, s.269SS/T);
  - depreciation;
  - s.44AB applicability;
  - financial statements;
  - the trial balance;
  - stale balances;
  - ledger scrutiny;
  - the cash book;
  - TDS payees;
  - s.43B and s.43B(h);
  - book-keeping quality;
  - loans and interest;
  - partners (s.40(b), s.194T);
  - bank reconciliation;
  - a high-value register.
  (#501, #504, #508, #560, #561, #571, #592, #593, #618, #636, #710, #713)
- Its tests are mutation-tested: a check limited to the paths a change
  touches runs in the pull request, and the recorded list is re-run nightly
  (#646, #682). Coverage of every module is not claimed.
- Its accuracy is **not yet proven publicly**. Issue #738 proposes how to
  prove it.

**Security and upkeep**

- The tools workspace moved off a yanked `chacha20` and a vulnerable `rustls`,
  and CI audits its lockfile (#455, #458).
- CI now fails if an HTTP call appears outside the files allowed to make
  one. This checks where network code may live; it does not by itself prove
  that no data leaves the machine (#701).
- We deleted legacy code that nothing reached (#473, #495).

**Known limits of 0.3.0**

- The package is unsigned. It is for evaluation, not production.
- Native Windows validation with Tally and Claude Desktop is outstanding
  (the Tauri desktop app's own Windows catalog flow is #293). The posting
  approval's decision also has no behavioural test on Windows (#702). Intel
  Mac is not qualified.
- How long Claude Desktop waits on one tool call is not measured (#703).
- There is no tool to delete or undo a posted voucher, and a ledger that was
  replaced is not always noticed (#623).

## [0.2.0] - 2026-09-16

These are the detailed entries that were in this file when `mcp-preview-0.2.0`
was published. They were written before the plain-words summaries began, and
there is no summary for that build.

### Removed

- DSC (digital-signature certificate) hardware-token detection, certificate
  extraction, and their AXAL sync path have been withdrawn from Bridge's
  scope, along with the `pkcs11` and `cryptoki` dependencies that reached the
  PKCS#11 driver. `pkcs11` 0.5.0 was unsound (RUSTSEC-2022-0034) and
  unmaintained; the capability may be rebuilt properly later if needed. AXAL's
  Tally and Documents integrations are unaffected.

### Changed

- `build_import_xml` now reports `live_evidence` as an array of
  `{observation, report, voucher_types}` records rather than a single string,
  and no longer emits `live_evidence_report`. The previous shape could name
  only one source for a whole batch, so a Payment build cited a report that
  records Payment being refused. A client branching on the old string value
  needs updating; the accompanying voucher types make the provenance readable
  without one.
- Relicensed future Bridge distributions from the MIT License to the Apache
  License, Version 2.0. The previously published `v0.1.0` release remains
  available under the MIT License that accompanied that release.
- Core snapshot canaries now authorize attempts through stable observed
  sealed-profile execution evidence without claiming field support from an
  incidental first-day dataset; snapshot rows are always re-fetched after the
  durable run starts.
- A probe that no longer returns the selected company now clears and
  invalidates every company-scoped evidence, proof, mirror, diagnostic, and
  snapshot view before installing the replacement probe, so its fresh review
  remains usable without displaying stale company data.
- Snapshot lifecycle probes no longer replace interactive setup-review state;
  restart admission uses the exact sealed Core receipt, and ambiguous duplicate
  live company identities fail before any snapshot read with their concrete
  terminal proof reason preserved.
- Snapshot recovery now durably replays backward-clock abandonment evidence,
  enforces a 100,000-record aggregate hydration ceiling, and recovers an exact
  already-committed receipt from compact hash-bound proof authority without
  rehydrating canonical membership.

### Added

- Local import files may now carry Payment, Receipt and Contra vouchers as well
  as Journals, so a bank statement can be expressed in the voucher types Tally
  files it under. Each of the three is admitted only as two entries over two
  distinct ledgers carrying neither a voucher number nor a reference. The side
  that must hold money is refused unless that ledger's live group ancestry
  reaches a reserved Bank Accounts or Cash-in-Hand identity — the two where a
  captured ledger is observed sitting under a captured group. The counterparty
  side must be established as holding no money: any money group there means the
  voucher is really a Contra, and a ledger whose group ancestry cannot be
  resolved is refused as well, because neither leg is admitted on an absence of
  evidence. A build that names a
  counterparty warns that its amount lands On Account. Native posting is
  unchanged and still accepts only one unnumbered Journal.
- A local-first Tally Truth Layer with capability passports, explicit truth
  states, encrypted mirror evidence, resumable/adaptive snapshots, Proof of
  Sync and Gap Map output, and a safer operator console. The migrations are
  additive; rollback requires restoring the prior application and retaining
  the encrypted database for forward recovery rather than deleting evidence.
- Portable, bounded Tally protocol, canonicalization, transport, runtime,
  compatibility, incremental-policy, qualification, observability, and
  write-safety crates backed by a synthetic loopback protocol simulator.
- Reviewed single-use setup authority, exact selected-read qualification, and
  fail-closed compatibility manifests/runbooks. Live Education behavior and
  every write capability remain unknown or disabled until exact reviewed
  evidence exists.
- Native Windows and macOS CI coverage for formatting, tests, builds, and
  Clippy.
- Repository-local Windows and macOS application icons.
- Open-source contribution, security, review, and rectification guidance.
- Reproducible Node and Rust toolchain baselines, installer smoke builds, and
  complete lockfile-to-license-inventory checks.
- Automated legal-resource inspection for Windows MSI/NSIS installers and the
  staged and DMG-packaged macOS app bundles.

### Security

- SQLCipher/keyring-backed local Tally state, immutable proof/checkpoint
  receipts, loopback-only HTTP without a proxy, bounded incremental decoding,
  cancellation and lease enforcement, idempotent crash replay, and sealed
  no-write qualification boundaries.
- SQLCipher pool replacement connections now receive raw key bytes from
  zeroizing storage without retaining a key-derived pragma string. Proof
  contract v3 binds detailed record counts, and historical crash recovery no
  longer depends on current checkpoint ownership.
- File-backed snapshot ownership now uses per-run kernel advisory locks, so a
  crash can be reclaimed after wall-clock rollback without allowing a live
  owner to be stolen. Persisted/live company profiles correlate through an
  opaque endpoint-scoped identity key, and macOS qualification reports
  `ru_maxrss` in its native byte units.
- Losing checkpoint compare-and-swap decisions terminalize as durable failed
  proofs and close staging attempts instead of remaining falsely resumable;
  unrelated checkpoint advances do not rewrite Failed or Cancelled outcomes.
- Compatibility claims now require verified synthetic-fixture identity before
  an explicit parsed Tally application rejection can establish `Unsupported`;
  fixture, context, sentinel, parser, malformed-response, and transport
  failures remain fail-closed observations rather than incompatibility claims.
- Updated the XML parsing graph and removed unused Linux-only dialog
  dependencies from the supported Windows and macOS build graph.
- Updated the Tauri runtime to 2.11.5 and tauri-runtime-wry to 2.11.4.
- HTTPS-only AXAL endpoints with redirect blocking, bounded responses, and
  credential validation.
- Safer DSC PIN transport and PKCS#11 library discovery without exposing
  arbitrary native-library loading to the webview.
- Bounded Tally and document responses with endpoint and upload validation.

## [0.1.0] - 2026-07-12

### Added

- Initial open-source Bridge application with React, Rust, and Tauri support
  for Tally, GST, DSC, document, sync, and local database workflows.

[Unreleased]: https://github.com/ComplyEaze/bridge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ComplyEaze/bridge/releases/tag/v0.1.0
