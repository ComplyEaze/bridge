# Changelog

All notable changes to ComplyEaze Bridge are documented here. The project follows
[Semantic Versioning](https://semver.org/).

## [Unreleased]

Published builds so far are `mcp-preview-0.2.0` and `mcp-preview-0.3.0`, unsigned
MCPB packages. The number of the next build is chosen when it is released.
The version boundary between the published MIT-licensed `v0.1.0` release and
Apache-2.0 builds from current source stays unambiguous.

## [0.4.0] - YYYY-MM-DD [PLACEHOLDER: set the release date when the build is cut]

### In plain words: ComplyEaze Bridge 0.4.0, since `mcp-preview-0.3.0` (26 Sep 2026)

[PLACEHOLDER: this section is drafted ahead of the build. Before it merges, resolve every
line that starts with `[PLACEHOLDER` or `[PENDING`, then delete this paragraph. Heading
and first-part names follow docs/release-process.md.]

Each line names the pull requests it comes from.

**What you can do now**

- Read a company's masters as a list: voucher types (with their numbering
  method), godowns, units, stock groups, or ledger groups, in pages. The book is
  checked before and after the read. A book with very many masters is refused,
  with the size named, rather than answered in part (#952).
- Read closing stock per stock item at a financial year end (a 31 March): the
  quantity and value of each item, whether inventory is integrated with the
  accounts, and how many items have a negative closing quantity. The item
  values are checked against Tally's own Stock Summary. Any other date is
  refused. Only the year ending 31 March 2026 has been measured, on one
  synthetic book (#980).
- See what a party's unallocated amount is made of, from data the tool already
  reads, and what that data cannot tell apart. Each bill now carries its own
  date and credit period, and payable and receivable follow the sign of the
  bill's balance (#945, #946, #957, #959, #961).
- See what ComplyEaze Bridge keeps on this computer. `local_data_report` (and
  `bridge_mcp --local-data-report` on the command line) counts files, bytes and
  the age of the oldest file by kind, and gives the state of the import journal
  and how many saved batches are not settled. It reads only: it changes and
  deletes nothing, and it names no file path unless you ask on the command line
  (#925).
- [PLACEHOLDER: purchase register (#971). Add one plain sentence when it merges,
  or delete this line if it does not make the build.]
- [PLACEHOLDER: bill trail (#981), why a bill is open and what an unallocated
  amount holds, party by party. Add one plain sentence when it merges, or delete
  this line if it does not make the build.]
- Read Profit and Loss and Balance Sheet. A figure is shown only when it ties
  line for line to Tally's own statement; otherwise it is refused, and the
  lines that differ are named. A book with stock items is expected to be
  refused, because no such book has been measured, and the tie-out itself has
  been measured on two synthetic books only (#774).
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
- [PENDING #914: keep this line only once #914 has merged, and then delete the
  line about the documents upload below.] The unfinished document-sync code is
  removed: the sign-in and the code that uploaded files you picked. After this
  change the only network connection ComplyEaze Bridge's own code makes is to
  Tally on the same computer. The published 0.3.0 package still contains that
  code, though no tool of the extension reaches it (#914).
- Every tool is refused until the Terms of Use are accepted, and the extension
  carries a privacy policy (#943).
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
- The desktop app's documents upload skips unchanged copies of the
  party-statement batches it wrote (#847).
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

- Stock: closing stock has been measured for one year end on one synthetic
  book. A book with stock items is expected to be refused by Profit and Loss and
  Balance Sheet, because no such book has been measured (#774, #980).
- A book with very many masters is refused by `masters` for godowns, units and
  stock groups, and how common that is across real books has not been measured
  (#952).
- `local_data_report` has not been run on a Windows host, and how it reports a
  Windows junction is unverified. It does not cover the desktop app's other
  settings, its database or its logs (#925).
- [PLACEHOLDER: add the known limits of the purchase register (#971) and the
  bill trail (#981) when they merge.]

**Also in source**

- The tax-audit engine gained its stock and party-monthly tests. The engine is
  still not in the extension, and its accuracy is not yet proven publicly (#738,
  #744, #788).
- The desktop app moved to a Tauri release that fixes GHSA-w28w-mhc8-qvjv
  (#805).

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

[Unreleased]: https://github.com/lamemustafa/bridge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/lamemustafa/bridge/releases/tag/v0.1.0
