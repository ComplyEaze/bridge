# Changelog

All notable changes to ComplyEaze Bridge are documented here. The project follows
[Semantic Versioning](https://semver.org/).

## [Unreleased]

Published builds are MCPB packages that are not yet code-signed (tags
`mcp-preview-*` and, from 0.4.0, `mcp-v*`): so far
`mcp-preview-0.2.0`, `mcp-preview-0.3.0`, `mcp-v0.4.0`, `mcp-v0.4.1` and `mcp-v0.4.2`. The
number of the next build is chosen when it is released.
The version boundary between the published MIT-licensed `v0.1.0` release and
Apache-2.0 builds from current source stays unambiguous.

### In plain words: the next build, since 0.4.2

These changes are in source and not yet in a published build.

**New**

- `vouchers`, `changes` and the sales and purchase registers now return `reference_date`, a voucher's
  `REFERENCEDATE` (`YYYYMMDD`), where the voucher has one (#1257). Checked once against a
  live TallyPrime 7.1 on one synthetic book: the element came back on every voucher, empty on all but one
  and filled on that one (a Purchase) and on a voucher keyed for the check; it was not checked on a sale or on
  another voucher type, and a value that is not a date refuses the read.
- `vouchers` and the `outstandings` party detail now reach a ledger by either
  spelling of its name when Tally's own name for it differs in case or symbols from the
  spelling its vouchers carry (26 of 4,017 ledgers in a separate census of 13 books,
  not reproducible from this repository), and `ledger_match` shows Tally's own name,
  with `ledger_row_spelling` when the vouchers spell it differently. A spelling
  that is two ledgers' is refused as `ledger_ambiguous`. The voucher filter and the
  trail still use the spelling the vouchers carry; `ledger_movement` and imports are
  unchanged. Not measured: that Tally accepts the voucher-row spelling when it
  differs from the ledger's own name, what a voucher row carries for such a ledger
  on a book other than the one measured, and what the outstandings report that the
  trail reads carries for it, so a party detail that finds no bill or no unallocated
  row for such a ledger says `report_spelling: not_established`; that the first name
  list is the primary language (#1085).
- `vouchers` can now find a voucher by its number, reference, a phrase of its
  narration or an amount (`voucher_number`, `reference`, `narration_contains`,
  `amount`), and can add a window up by ledger, month or voucher type
  (`summarise_by`), with debit, credit and voucher counts per bucket and the
  cancelled, optional and entry-less vouchers counted apart. Both run on the
  rows the window read already holds, so they send no new request to Tally; a
  zero from a counted window is a checked zero. A narration phrase is refused
  where narrations are withheld from the assistant. A summary sums post-dated
  vouchers (counted, with the vouchers Tally sent no flag for counted apart) and any
  non-posting voucher type a book uses, and says so in the result. Checked once
  against a live TallyPrime 7.1 on a synthetic book of 67 vouchers: every bucket
  equalled the sums over the listed vouchers, the ledger buckets equalled
  `trial_balance` for the same year, and each search returned what the same
  criterion selects from the listing; it did not cover a large book, a memorandum,
  a reversing journal or a voucher withheld for a foreign-currency amount (#1230).

**Safer or fixed**

- `purchase_register` and `sales_register` now take their `state` from the rule
  `vouchers` uses. A non-empty window is `complete` only when every voucher read
  was checked against a separate count of the window; a window nothing counted,
  which only a book small enough to need no census gets (a few dozen vouchers), is
  `partial` with `reason` `nonempty_window_unqualified`, and its rows are still
  returned. Before, the registers called such a window `complete` on the
  company marks and the ledger masters alone, so the same window had two answers.
  A row's `status` is unchanged. Not measured live: a window admitted against a
  census through the registers on a large book; the change sends no new request,
  it uses the census the read already makes (#1031).
- `profit_and_loss` now compares Tally's `Cost of Sales :` heading with the
  derived cost of sales (Purchase Accounts plus Direct Expenses) even when the
  heading reads zero or empty. Over a non-zero cost of sales such a heading
  refuses gross and net as `tally_profit_and_loss_differs`, naming the heading;
  over a zero one it ties. Before, a zero or empty heading passed whatever the
  cost of sales was. Both committed Profit and Loss captures still tie; whether a
  real Tally ever prints the heading zero or empty over a non-zero cost of sales
  is not measured (#1070).
- `post_import` and `verify_import` proofs now measure `alter_id_delta` from the
  company's voucher mark read just before the POST, for a native post, instead
  of from the mark recorded when the batch was built. Anything posted between
  the build and the POST no longer counts as this post's, so the delta agrees
  with what Tally created. `alter_id_delta` gains `from` (`pre_post_mark`, or
  `build_mark` for a file imported by hand, which has no POST of its own);
  `pre_import_mark` still reports the build-time mark (#1087).
- `build_import_xml` now refuses a batch that names a ledger which keeps bills in
  Tally, as `bill_wise_party_unapproved`, until the person approves each such
  party (the build is repeated with `on_account_approvals`): an entry on it carries no bill allocation, so Tally lands it On Account
  and the person must match it to a bill by hand. Before, the build only printed a
  warning. `post_import` and the queue admission read the flag again, from the
  ledger list they already read, and refuse `import_bill_wise_changed` for a
  ledger that became bill-wise since the build. A batch built before this change
  is refused for posting as `import_batch_predates_bill_wise_record` and is
  rebuilt (check first whether its file was already imported by hand). The
  approval is the assistant's word, not proof that a person said yes, and a hand
  import of the file is not checked. Not measured: a large book, and the bills
  of a ledger whose flag reads No (#1234).
- Versions 0.3.0 to 0.4.2 refuse to post a batch this version builds. They do
  not read the cash-in-hand and bill-wise records a saved batch now carries,
  so after a rollback they could have posted it with neither check; the batch's
  ledger binding is now saved under a name they do not read, and they refuse a
  batch with no binding before any approval window or request to Tally
  (`import_batch_predates_ledger_binding`, nothing posted). Their message for
  that refusal says "Build the batch again"; for a batch this version built,
  reinstall this version (or a newer one) and post the batch from it instead,
  because a batch built again on an older version has neither check. A batch an
  earlier version saved is read as before. Which versions can read the import
  history is unchanged: from the first post attempted with 0.4.2 or later,
  only 0.4.2 and later read it. 0.2.0 and earlier have no such refusal: do not
  run them over a data folder this version has built in (#1234).
- The posting setting is shorter and names its four known limits in plainer
  words, and Terms of Use section 9.2 (version 2026-10.1, effective 7 October
  2026) now lists the same limits and more: a company or ledger renamed or opened
  at the moment of a post, another Tally connector that can change entries
  without the approval window, and that dealing with a voucher that reaches the
  wrong company remains yours. The setting that accepts the Terms is now named
  for version 2026-10.1, so the next build asks you to accept the Terms once
  more; version 2026-10 stays published for the builds that asked for it
  (#1010).
- When a ledger name you gave is not in the book, the candidates now list ahead
  of the others the ledgers that hold every word you typed
  (`rule`: `shared_every_distinctive_token`), then those that share only some;
  a ledger matched by a stronger rule still comes first. The same ledgers are
  found and counted as before and none is chosen for you; when more than 25 are
  found, the ones left off the list are the ledgers holding only some of the
  words before those holding all of them (#1076).
- Five lists of a batch's ledgers in refusals now come in the batch's own
  order, not in name order. `masters` of `masters_not_exact`, `ledger_twins`
  and `ledgers_changed` list a ledger where the batch first names it;
  `refused_ledgers` of `cash_bank_ledger_not_established` lists a ledger where
  one of its entries is first refused, and `refused_ledgers` of the cash-answer
  refusals (in `build_import_xml` and `post_import`) where an answer first
  names it. The order is the same whether or not party names are masked. The
  same ledgers are found and refused as before.
  Where a list is cut (`ledgers_changed` names eight; a small response cap
  leaves rows of `refused_ledgers` out), which ledgers are named, and how many
  `refused_ledgers_omitted` counts, can differ from before. The ledgers a
  `post_import` or `verify_import` answer names in `masters_after_post.ledgers`
  come in the same order, where the batch first names each, in the answer and
  in the proof saved from it (the desktop screen names the first eight of
  them); the recorded verdict, and the review dialog that reads it, keep the
  order they were recorded in. Other lists are unchanged (#1234).

## [0.4.2] - 2026-10-03

### In plain words: ComplyEaze Bridge 0.4.2, since 0.4.1 (2 Oct 2026)

These changes are in ComplyEaze Bridge 0.4.2. Each line names the pull
requests it comes from, except where it names an issue. A "mark" below is a
counter Tally keeps that moves when vouchers or ledgers change.

**Should I upgrade?**

- **Remove the old extension first.** 0.4.2 installs beside 0.4.1 or earlier
  instead of replacing it, because its author line changed from "Bridge
  contributors" to "ComplyEaze contributors" and Claude Desktop builds an
  extension's identity partly from its author name (seen with 0.4.1 on a Mac;
  earlier releases carry the same author line, so the same is expected; we did
  not try Windows). Before installing 0.4.2, in Claude Desktop open Settings,
  Extensions and remove the older ComplyEaze Bridge (release 0.2.0 shows as
  "Bridge Tally"). Keep ComplyEaze Bridge's data folder: both versions use it,
  so do not delete it. After installing, enter your settings again: the Tally
  port, the posting setting (posting starts off), the Terms setting, which every
  tool needs, and Response redaction, which starts at none (set it again if you
  had shortened or masked names). If you already have two, remove the older one:
  once 0.4.2 has tried a post, the older one can no longer prepare, post or
  check vouchers (see the posting points below).
- **The number is a patch number on a larger change.** 0.4.2 adds a tool, asks
  you about ledger names it used to read, and once you post with it you cannot
  go back to 0.4.1.
- **If you use 0.3.0 or 0.4.0 on Windows: yes.** 0.4.2 contains everything in
  0.4.1, including the fix for the network-path forms of the bank-statement path
  issue published as
  [advisory GHSA-vm5g-r3p7-wxx7](https://github.com/ComplyEaze/bridge/security/advisories/GHSA-vm5g-r3p7-wxx7).
  Some paths still pass; the advisory lists which. If you cannot upgrade yet,
  follow the advisory's steps.
- **On a Mac with 0.3.0 or 0.4.0: recommended, though not urgent.** The advisory
  rates the issue low there; some paths still pass, and the advisory lists
  which.
- **If you use 0.4.1: nothing below is urgent.** The reasons to upgrade are that
  a ledger name that differs by a symbol, an accent or run-together words now
  makes the assistant ask you instead of being read, later pages of a long
  voucher list can be served faster, and a post that Tally refuses is now named
  plainly.
- **If you used 0.4.1 or earlier: check answers for a ledger named loosely.** If
  you asked about a ledger by a name that was not its exact spelling, the
  figures may be for a different ledger with the same letters and digits, and an
  earlier `vouchers` answer did not show which ledger was matched. Ask again
  with 0.4.2, which asks you instead (#1092).
- **If you turn posting on, the way a posted voucher is matched has changed.** A
  post no longer adds a ComplyEaze Bridge tag to the narration, and each voucher
  is matched by its place in the run of changes Tally records for that post
  (#1054). From the first post 0.4.2 sends to Tally (its record is written just
  before sending, so this includes a post Tally refuses or never answers), do
  not run 0.4.1 or earlier, of the extension or of the desktop app, on this
  computer: the record of what was sent (the import journal) then holds fields
  they refuse to read, and they stop preparing, posting and checking vouchers.
  Before that first post, going back is harmless. The one-voucher post that the
  extension makes was not run live with this matching (see Known limits).
- **If you turn posting on: `voucher_presence` cannot recognise these
  vouchers.** It cannot identify a voucher this version posted (it matches a
  ComplyEaze Bridge voucher by the tag only for files you imported by hand), so
  a voucher posted with 0.4.2 that someone then edited or re-dated in Tally can
  read `absent` there. Check what ComplyEaze Bridge posted with `verify_import`,
  never with `voucher_presence`, before entering any of it again by hand. In
  Tally's own screens these vouchers can no longer be told from hand-entered
  ones by their narration. Posting is off in a new install, and 0.4.2 installs
  as one: check the setting after you install it.
- **How:** ComplyEaze Bridge does not update itself. Follow the
  [installation guide](https://github.com/ComplyEaze/bridge/blob/master/docs/agent/INSTALL.md):
  (1) close any other program that runs ComplyEaze Bridge; (2) in Claude
  Desktop, remove the older ComplyEaze Bridge in Settings, Extensions, then
  install the new file from the same screen; (3) keep ComplyEaze Bridge's data
  folder, which holds its record of what it has sent to Tally (from 0.4.2 it is
  the only record of which vouchers it posted); (4) enter your settings again,
  including Response redaction (it starts at none), and check that the extension
  shows 0.4.2 and that "Allow voucher posting" is as you want it; (5) quit
  Claude Desktop completely and reopen it.
- **What was tried** is under "Known limits" below. No one on our side installed
  the Windows package of this build in Claude Desktop on a Windows PC.

**What you can do now**

- **A new tool, `sales_register`.** It reads a register of the tax in the books
  for sales: the Sales and Credit Note vouchers of a date window that touch a
  Duties & Taxes ledger, with each entry's tax taken only from the GST duty head
  on that ledger's master, never from a name or an amount. It reads as
  `purchase_register` does, and it is a register of the books, not a GST return.
  A Credit Note keeps Tally's signs (nothing is netted or flipped, so add signed
  amounts), and may be a sales return or a credit note to a supplier: each row
  says the party's group and the tool does not choose. A Debit Note is listed
  apart, without an amount. The state-side tax head has two recognised forms,
  `state_tax` and `sgst_utgst`, for the same side. A row is marked
  `not_measured_live` only for kinds the row itself shows (for example
  inter-state, cancelled, optional or post-dated); an unmarked row is not
  thereby measured, because a sale typed on Tally's screen, a tax Tally
  computed, a duty head such as cess, an invoice with several goods lines and a
  currency other than the book's base cannot be marked. It needs a book whose
  base currency is INR and refuses a book too large to list. It reads the whole
  date window once and the ledger masters twice (before and after), and all of
  it again for every page. One call sent 96 requests to Tally on one small book
  and 118 on another. Use a narrow date range: its cost on a large book was not
  measured. It was run against a live Tally, once each, on synthetic companies:
  one taxed Sales item invoice, one untaxed one (read once by an earlier build;
  its masters and the tool's answer were not kept) and one Credit Note. What was
  not shown is under "Known limits" (#1009).
- **Later pages of a long voucher list can be served from the first page's
  read.** `vouchers` no longer reads the whole window again for every page when
  the window was read `complete`, which means counted first. On one synthetic
  book, one month of 2,542 vouchers read in a release build, the first page took
  about 66 to 68 seconds and a later page about 1 second (one run each). A later
  page that names the first page's `snapshot_id` is refused if the company's
  marks moved or the held read is gone, instead of continuing from a different
  read. A read is held for ten minutes and is also dropped after a write through
  ComplyEaze Bridge, when the size cap that all held windows share (64 MiB)
  evicts it, or when a newer read of the same question replaces it. A small or
  new company is not counted, so its window reads `partial` and every page reads
  it again. A later page that does not name the `snapshot_id` reads afresh when
  the books moved, and says so (`earlier_snapshot`: its offsets do not continue
  the earlier pages, so start again from the first page); when the hold ran out
  it reads afresh without that flag, and only a page whose `snapshot.reused` is
  true continues the earlier ones. A date is the same question as `2026-08-01`
  or `20260801`, while the `ledger` argument must be repeated exactly as typed
  (#1053, #1118).
- **A refused ledger name now tells the assistant to ask you which ledger you
  mean.** When `ledger_movement`, `vouchers` (with a ledger) or the party detail
  of `outstandings` cannot find the name you gave, the refusal can now list the
  ledgers it may mean (not when party names are masked), and the assistant is
  told to ask you, even when there is one candidate, and never to choose. A name
  is now read without asking only when it matches a ledger's spelling exactly,
  or differs only in upper or lower case English letters or in spaces. Before, a
  name that differed by a dropped symbol, an accent or run-together words was
  read if one ledger had the same letters and digits; in a lab list of synthetic
  ledger names that read the truncated name `Input Cess (` as a different
  ledger, `Input Cess`. Each answer for a named ledger now says how the name
  matched. A name containing a no-break, figure, narrow no-break or ideographic
  space asks, as a test now pins (#1057, #1092, #1127; issue #1095).
- **The assistant is told how to pick a company, and to say which one it used.**
  `list_companies` now begins "Start here" and states the rule (use the open
  company only if it is the only one open and you named no client, or if exactly
  one open company matches the client you named; otherwise ask, and never
  guess), and Claude Desktop is sent the same rule at start-up, with the
  instruction to name the company, dates and ledger in the first line of an
  answer, and what to do after a refusal. The start-up text follows your posting
  setting: with posting off or import only, it says this connection cannot post
  to Tally. It also tells the assistant to ask before reading vouchers over more
  than a month unless you gave the period, and before an `outstandings` party
  detail (#1058, #1113).
- **A plain opening line on three reads.** `trial_balance`, `profit_and_loss`
  and `balance_sheet` now begin with a `headline` when they answer: the company,
  the exact period and, in words, whether the figures were read for every
  ledger, partial, or established or not. A refusal has no headline yet. The
  codes and figures stay in `result` unchanged (#1062, #1066).
- **`outstandings` says what it counted and which date it used.** It returns
  `open_bills_total` (every open bill in that direction, counted before paging)
  and `open_bills_shown`, so a cut page no longer reads like the whole list (on
  a partial read these cover base-currency ledgers only). In every answer that
  reads (a refusal carries none) it returns the `as_of` date it used, which is
  today's date on this computer when you gave none; `tally_status` returns that
  date as `today` (#1049, #1079).
- **Pages that are not in the package:** a Questions page on the install site
  with a table of what it can and cannot do, a before-you-start list and plain
  answers, including how far our liability goes and how to reach us (#1061,
  #1084); a "What it costs" section in the README and a line on the Download
  page: ComplyEaze has set no price and sells no licence today (#1059); the
  Privacy Policy (version 2026-10.1, effective 3 October 2026) and every footer
  now name the publisher with its registration number and registered office, the
  policy adds a purpose for publishing written quotes you consent to, and says a
  grievance is answered within one month (it said 30 days) (#1104). The site
  also gained canonical addresses, a sitemap and an `llms.txt` (#1047).

**Safer or fixed**

- **A withdrawn approval is named.** If an approval is withdrawn after the post
  has taken it but before anything is recorded or sent, `post_import` now
  answers `import_approval_revoked`; before, it answered
  `import_dispatch_outcome_unknown` although nothing had been recorded or sent.
  The message ("No posting attempt was recorded. Review the error before
  requesting approval again.") and the `attempt_recorded: false` field are
  unchanged, and no next step is attached. It was tested on a scripted Tally,
  not a live one (#1131; issue #791).
- **A post that Tally refuses is named plainly.** If Tally's answer to a post
  counted nothing created (`CREATED 0`, an exception for each voucher sent and
  every other counter reported as 0) and the voucher is not found, `post_import`
  now says Tally reported it as not created, to check it is not in Tally, and to
  enter that one voucher in Tally's voucher entry screen, never through Tally's
  Import menu; in the answer to that post itself the voucher's status reads
  `tally_reported_not_created`. This covers a single voucher, which is what the
  extension posts, and a saved batch of two or more vouchers rejected whole when
  the company's mark was read before and after and did not move (batch posting
  is an environment setting, `BRIDGE_AGENT_ENABLE_BATCH_POST`, that the
  extension's settings do not offer). Tally's answer was captured live only for
  missing ledgers, on TallyPrime 7.1 Silver (one Education answer to a bad date
  was also used, for its counter shape only), and the reading of it was tested
  on those saved answers, not in a live post; the code applies it to any answer
  of that shape, including on Gold, where it was not measured. After you enter
  the voucher by hand as told, the saved batch stays "reconciliation required"
  and ComplyEaze Bridge cannot close it (#1039 is open; #1116, #1126).
- **Where a posted voucher cannot be matched, the status changed.** In 0.4.1 an
  unfound voucher of a post read `not_found`. In 0.4.2 a voucher of a post made
  with 0.4.2 that cannot be matched reads `sent_not_attributed` ("check in Tally
  before posting it again", for example after an edit), never `not_found`; posts
  made with 0.4.1 or earlier are read as before. That is also the reading for a
  batch that landed in part, and for a later `verify_import`, including of a
  voucher Tally refused. Other new readings name why a voucher is missing:
  `bound_not_in_window` (it may have been deleted, re-dated or restored from a
  backup; whether an edit does this was not tried: check in Tally) and
  `book_rolled_back` (the books look older than the post). A post is refused as
  `post_mark_unrecorded`, with nothing sent and the approval withdrawn, when the
  company's voucher mark cannot be read before it (#1054; #1116, #1126 and #1107
  for the refused and part-landed cases).
- **Clearer next steps on refused batches.** When no attempt was made, a saved
  batch refused because a narration, reference or voucher number cannot be
  posted now says that nothing was sent, and to correct the text, build the
  batch again and post the new one (before, it said to review the error before
  asking for approval again, which fails the same way each time) (#1105). A
  batch that mixes a Journal with Payment, Receipt or Contra vouchers is refused
  as before, and now says to split it into one batch of Journals and one of the
  others (#1094). A date or company ID refused before anything is read now says
  what it should have been: a date as text in the form YYYYMMDD or YYYY-MM-DD,
  and the 36-character company ID from `list_companies`, not its name (#1115).
  The refusal for a changed ledger count names an altered ledger as a possible
  cause, marked as reasoned and not reproduced (#1032).
- **The posting path says what is sent.** Narrations are sent as given, and the
  approval dialogs say "narrations sent as prepared, nothing added" instead of
  saying a batch reference is added. A narration that contains the character
  U+FFFD (a diamond with a question mark) followed by `#`, digits and `;` is now
  refused when the batch is built, because it is compared byte for byte. The
  file you import by hand keeps its tag. A bound voucher whose amount was
  changed in Tally reads `posted_divergent`, and `acknowledge_post_review` finds
  a bound voucher by its Tally ID (#1054).
- **`purchase_register` explains a cancelled Purchase.** A cancelled Purchase or
  Debit Note sits in the list of vouchers without a tax entry, with `cancelled:
  true`, because the cancelled vouchers measured came back from Tally with no
  ledger entries, so being listed there does not mean the voucher was untaxed. A
  cancelled voucher that keeps its entries has not been measured (issue #1013;
  #1027).
- **A statement line that reads 0 and 0 is compared, not skipped.**
  `profit_and_loss` and `balance_sheet` now compare a Tally statement line that
  has `0.00` present in both columns with the line derived from the trial
  balance; a derived amount over such a line is reported as a difference and the
  statement is marked not established, with its derived lines withheld, instead
  of being called established. On four synthetic lab books every zero line came
  back empty in both columns, so this has not been seen in a real answer (issue
  #1067; #1071).
- **`outstandings` now reads one far-future due date.** A due date that Tally
  prints with a four-digit year of 2100 or later (such as `1-Dec-2108`) is read
  as written, and gives no overdue days in the ageing; before, that one bill
  failed the whole read with no reason. A bill row whose dates cannot be read
  still refuses, and the refusal now carries a named cause, the report, the row
  number when a row is what failed (also for an error found while the report is
  read, such as an unreadable amount or a repeated field, but not for a value
  that appears before any bill row), and a next step, never the bill's party,
  reference or date. A whole call on the synthetic book that printed such a date
  has since completed once (see Known limits) (#1098, #1128, #1136; issues
  #1091, #1096).
- **The two log tools now say what they do not show.** `read_evidence` and
  `egress_log` say what each holds and that neither shows what your AI provider
  received, and tell the assistant never to say that no data has left the
  computer. What the assistant reads, amounts included, goes to your AI provider
  (#1045, #1113).
- **Source only, not in the extension: the desktop app's "Show in folder" opens
  only a file it exported since it started.** ComplyEaze Bridge publishes no
  desktop installer, and the extension does not contain the desktop app. Any
  other path is refused before the file manager opens. A file replaced by a link
  after export would still open, the record keeps the 1,000 newest exports, and
  the Windows and Linux paths and the button itself were not tried by hand
  (#1051; issue #915).
- **Wording.** The extension's settings are shorter overall (the redaction
  setting now says more) and the three longest read descriptions lead with the
  outcome; assistant texts say "ComplyEaze Bridge" in full, and so do the
  desktop app's messages (source only); the extension's author line reads
  "ComplyEaze contributors", which is why 0.4.2 installs beside an older version
  (#1042); `acknowledge_post_review` names the refusal for a review recorded for
  a different doubt (#1026, #1028, #1124, #1129, #1130; issues #962, #1010,
  #830).

**Known limits**

- **`sales_register` was not shown** on an invoice-view Credit Note, an
  inter-state (IGST) line, a cancelled or optional sale, an unrecognised or
  missing duty head on a sale, more than one voucher in a window, paging, a
  company with a registration, a tax Tally computes itself, a duty head no sales
  run showed (such as cess), an invoice of another shape than the one run (for
  example several goods lines), a sale typed on Tally's screen,
  accounting-invoice mode, a post-dated sale, a `REFERENCE` or a filled
  `PARTYGSTIN` on a sale, a `REFERENCEDATE` on a sale, or a currency
  other than the book's base. The invoice it was replayed on was imported by
  this project. Its `complete` state rests on the company marks and the ledger
  masters reading the same before and after, not on a separate voucher count
  (#1009).
- **Long voucher lists:** a change that moves neither company mark is not seen
  between pages, a window larger than that cap is not held and is read again for
  each page, and what happens if someone changes the book at Tally's screen
  between two live pages, or a later page of a window with a voucher type or
  ledger filter, was not tried against a live Tally. Not established: whether a
  save by another Tally Gold user, a restored copy of the company, or a company
  setting change moves the marks at once (#1053, #1118).
- **Ledger names:** a spelling that contains the mask mark (`…` or `...`) and is
  not typed exactly as the ledger is spelled is refused as `ledger_name_masked`,
  whatever the setting. When `mask_parties` is on, a spelling that shares a key
  with another ledger's masked form is refused the same way, which can also
  refuse an ordinary name typed in a different case until it is typed exactly,
  and the refusal lists no candidates. The candidates were tested on saved Tally
  answers, not a live book, and the ledger-name changes on a lab list of 144
  synthetic names; a real book was not tried (#1057, #1092).
- **Posting, what was tried:** two live posts through ComplyEaze Bridge's own
  path, of 3 vouchers each (3 Journals; a Payment, a Receipt and a Contra), made
  with the environment batch setting, and raw runs against Tally of 10 untagged
  Payment, Receipt and Contra vouchers and of 5 Journals, all on a synthetic
  company in TallyPrime 7.1 Silver. The extension posts one voucher per
  approval; that one-voucher path was not run live with 0.4.2's untagged
  matching (#1054).
- **Posting, what is not seen or not closed:** if a post's vouchers cannot be
  matched (for example one was edited in Tally before the matching was made, or
  Tally stored a narration differently; only one Hindi-and-rupee-sign narration
  was checked), that is final: the batch stays "reconciliation required",
  `acknowledge_post_review` does not apply, and you check its vouchers in Tally
  yourself (#1039 is open). Another Tally Gold client writing during a post is
  not seen. After a backup is restored, if the books are keyed past the post's
  mark before the next check, the restore is not detected as one (the voucher
  reads `bound_not_in_window`); if Tally reuses the lost vouchers' IDs (not
  measured), a different voucher that took a lost voucher's ID and has the same
  content can read `posted_verified` (#1050). Such vouchers cannot be posted
  again through ComplyEaze Bridge and must be entered by hand (#1038). Not
  tried: whether a voucher keeps its Tally ID when someone edits it in Tally's
  screens (if it does not, `verify_import` reads that voucher as
  `bound_not_in_window`, which does not mean it was deleted), whether deleting a
  voucher can lower the company's mark, a Windows approval dialog, Education
  mode, and more than 3 vouchers through ComplyEaze Bridge's own path (#1054).
- **Statement gate (#1071):** whether a real Tally ever prints `0` in both
  columns is not shown; the `Cost of Sales :` heading is not compared when it
  reads zero or empty (#1070).
- **`outstandings` on a book with such a date:** one whole call on the synthetic
  book that printed `1-Dec-2108` completed in a lab run (state `complete`, 52
  requests, 4.0 seconds, a page of 500 of 1,498 open bills). The bill with that
  date was not on the page returned, later pages were not read, and other books
  were not tried. A bill dated after `as_of` still refuses, and a two-digit due
  year can be read in the wrong century, as before. A bill cannot be left out
  and the rest returned as partial, because that would misstate the party's
  balance (#1136, #1098; #1091 stays open).
- **Assistant text:** whether Claude Desktop passes the start-up instructions to
  the model at all, and whether any of the new wording changes what an assistant
  does, was not measured; the studies were plan-only, without tool calls or real
  users (#1058, #1113, #1115). The new opening lines were tested on saved Tally
  answers, and no live Tally was read for them, for the ledger candidates, for
  the date echo or for the open-bill counts (#1049, #1062, #1066, #1079).
- **Tried against TallyPrime** (7.1 Silver in a lab, synthetic companies, one
  run each unless a line above says otherwise): the `sales_register` reads, the
  paging run, the untagged-post runs, and the rejected-voucher answers. The
  other changes are covered by automated tests, with a saved real answer behind
  the due-date item; the statement fix is tested on hand-built lines, because no
  saved answer has a 0 / 0 line. The release check starts each package, lists
  its tools and reads a sample bank statement, and does not run against
  TallyPrime. No one on our side installed the Windows package of this build in
  Claude Desktop on a Windows PC.
- **Pages:** no lawyer or CA has read the Questions page answers on liability
  and claims or the changed Privacy Policy text; the registration number and
  address were checked against the certificate of incorporation, not the current
  record (#1084, #1104).

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
