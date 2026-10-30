# ComplyEaze Bridge: TallyPrime MCP server for Claude Desktop

[![ComplyEaze Bridge MCP server – quality and maintenance score on Glama](https://glama.ai/mcp/servers/ComplyEaze/bridge/badges/score.svg)](https://glama.ai/mcp/servers/ComplyEaze/bridge)

<!-- llms:begin -->
ComplyEaze Bridge is an open-source (Apache-2.0) MCP server that connects
Claude Desktop to the TallyPrime (also written Tally Prime) running on your own
computer. It is built for chartered accountants, CA firms and accountants. It
reads your books; from a password-protected SBI, HDFC, Union Bank of India or
Bank of Baroda statement PDF it proposes Payment, Receipt and Contra vouchers, and once every
ledger name is exact and every question is answered it writes them to a file
that you import in TallyPrime yourself. What Claude reads from your books goes to your AI provider as part of
the chat. You can ask about the trial balance, outstanding receivables and
payables with ageing, ledger movement and vouchers, the purchase and sales
registers, the stock summary, and the Profit and Loss and Balance Sheet (a book
with stock items is read only when Tally's own closing stock closes every gap). It checks ledger
names before you post, and prepares Journal, Payment, Receipt and Contra
vouchers as an import file that you review and import into TallyPrime yourself.
Posting is off by default in a new install, so as installed it never posts to
TallyPrime: it reads from it and prepares files on your computer. If you turn
posting on in the extension, it posts one voucher at a time, or a saved batch
of up to 50, after you approve each post.

Tally Solutions also offers its own TallyPrime MCP plug-in for Claude Desktop;
its help pages (last updated 18 September 2026) list tools in six groups:
company, master, accounting, inventory, outstanding and transaction. ComplyEaze
Bridge is not made by, or affiliated with, Tally Solutions, and we have not run
the two side by side. TallyPrime also has its own Bank Statement import, which
takes statement files in the formats Tally lists for each bank.

**Current release:** <!-- managed:current-release -->[`mcp-v0.7.0`](https://github.com/ComplyEaze/bridge/releases/latest) (30 October 2026)<!-- /managed:current-release -->,
for Windows x64 and Apple Silicon Macs, as a Claude Desktop extension (an .mcpb
file); it has not been run in other MCP clients. We check each release before
we publish it: the release check confirms that each package launches, lists its
tools and parses a synthetic encrypted bank statement. It does not run against
TallyPrime, and nothing we can run covers every
Tally edition, set of books or setting. What has been run against a real
TallyPrime, and what has not, is listed below.
[Install it](./docs/agent/INSTALL.md) · [What changed](./CHANGELOG.md) ·
[Security and privacy](./docs/security-and-privacy.md) ·
[How to connect Claude to TallyPrime](./docs/guides/connect-claude-to-tallyprime.md) ·
[How to choose a TallyPrime MCP server safely](./docs/guides/choose-a-tallyprime-mcp-server-safely.md)

Not yet code-signed; your computer may warn you before opening it.

**Get it:** the [Download page](https://bridge.complyeaze.com/download.html)
offers the file for your computer; compare it with its `.sha256` file as
[the install guide](./docs/agent/INSTALL.md#for-it-staff) says. ComplyEaze Bridge does not
update itself, so look for a newer version on the
[releases page](https://github.com/ComplyEaze/bridge/releases).

**Try asking** (on a test company first):

- "List the loaded companies."
- "Show the outstanding receivables and payables, with ageing, as of 31 March."
- "Show the trial balance for 1 April to 31 March."
- "Check these ledger names against the book before I post: …"

**At a glance, release 0.7.0.**

- **You can ask for** outstanding receivables and payables with ageing, the trial balance, the vouchers in a date range (found by number, reference, narration or amount, and added up by ledger, month, voucher type, group or primary group; checked on one synthetic book, not a large one), Tally's own Cash Flow for whole months (shown only when its net total ties to the trial balance; not a cash flow statement under AS 3; tried on two synthetic books), one ledger's movement, Tally's own month-wise Sales or Purchase Register for whole months (not tied to the vouchers; read on one synthetic book), Tally's own Negative Stock and Negative Ledgers lists for a window (being listed is not a finding; synthetic books only), and the vouchers of one type that Tally marks Uncertain in its GSTR-1 (a Sales type, measured on one synthetic book).
- **Partly:** profit and loss and the balance sheet (a book with stock items is read only when Tally's own closing stock closes every gap, measured on one synthetic book; a book with several currencies is refused); closing stock value at a month's last day, and its closing quantity (compared with Tally's own Stock Summary for an item on a line of its own, the item list's own for an item inside a stock group), on small books, checked on two lab books; and a file of Payment, Receipt and Contra vouchers from a password-protected SBI, HDFC, Union Bank of India or Bank of Baroda PDF statement, which you check and import in Tally yourself. It creates no ledger, it cannot build onto a Cash-in-Hand ledger, and it has not yet been run on a real bank statement.
- **Not in this release:** GST returns or GSTR-2B matching, making sales or purchase invoices or GST entries, tax-audit flags (one test, the cash share of receipts and payments against the s.44AB(a) limit, is off by default and not yet run against a live TallyPrime), and Tally's Funds Flow and Ratio Analysis reports.

"You can ask" means it answered on the books and builds we ran it on; we have not yet run the published 0.7.0 file against TallyPrime (the program inside the published Windows x64 0.6.1 file (checked against its `.sha256` file) had one read-only run against a live TallyPrime 7.2 Silver, on one Windows computer on 27 October 2026, started directly by our test client rather than installed in Claude Desktop; it reported version 0.6.1, read Tally's status, the company list and the ledger masters of one synthetic company; nothing was posted); the macOS file has not been run against TallyPrime; our one recorded run of the published 0.6.0 file against a live TallyPrime posted one Contra to a synthetic company on one Windows computer on 16 October 2026 (#1780) and read it back; the other 0.6.0 posts ran on a release-candidate build and a rehearsal build. Very large books can fail or take longer than the assistant waits, and a book with several currencies is read only in part. Check any figure you rely on against Tally. The [Questions page](https://bridge.complyeaze.com/faq.html#what-can-i-ask) has the full table and its limits.

**Its tools.** In name order; release 0.7.0 installs 34 of them, 33 offered by default (`audit_test_run` appears only when its setting is on). None of them changes your TallyPrime books; with posting on, two more appear, and with posting and master writes both on, two more again that create one ledger, described below the table. Every call adds a receipt line,
with no figures, to a log on this computer.

| Tool | What it gives you | What it writes besides that receipt |
| --- | --- | --- |
| `audit_test_run` | Off by default (setting **Allow tax-audit test runs**). One tax-audit test run on the company's own books for a date range, today only the cash share of receipts and payments against the s.44AB(a) limit: figure and finding ids with their values, confidence and clauses, and the count of any failed book check; no ledger name, no voucher, and refused while a redaction setting is on. You name the cash and bank groups; the period must lie in 2025-26. Books only, not a tax opinion | Nothing |
| `balance_sheet` | The Balance Sheet for a date range, by primary group, from TallyPrime’s Trial Balance (a book with stock items is read only when Tally’s own closing stock closes every gap) | Nothing |
| `build_import_xml` | Checks a Journal, Payment, Receipt or Contra batch and writes an import file | An import file and a ledger record on this computer |
| `cash_flow` | Tally’s own Cash Flow for whole months: the month-wise movement of the cash and bank ledgers, returned only after its net total matches the trial balance; the split into months is Tally’s own and is not checked (not a cash flow statement under AS 3) | Nothing |
| `company_features` | Three settings the company’s record in Tally holds today (cost centres, GST and batch-wise stock; each yes, no or not reported) and the currency symbol; no ledger, voucher, amount or narration. Each of the three settings equalled Tally’s F11 screen on three synthetic books, not on this company; a setting changed inside a sitting has not been measured | Nothing |
| `egress_log` | The receipts ComplyEaze Bridge keeps of its own tool calls, with no figures | Nothing |
| `gst_by_counter_ledger` | For a period, the GST the books record on ledgers under Duties & Taxes, totalled per counter ledger: the one Purchase Accounts, Direct Expenses, Indirect Expenses or Fixed Assets ledger on a purchase, debit note, journal or payment with a supplier on it, named with its GUID, debit and credit side apart, per head; every other voucher with GST is set aside under a named reason with its own totals. Books only, not a return figure or an eligibility decision; measured on fourteen vouchers typed on Tally's screen in one synthetic book | Nothing |
| `gst_status` | Tally’s own GSTR-1 status flags on the vouchers of one voucher type in a period of at most 93 days: for each voucher its number, date, a status (included, uncertain, excluded, not in return, cancelled, optional or post-dated; for the last three also what the three status flags read) and whether a person accepted it as it stands; no amount, party, ledger or narration. A status is Tally’s own flag and not a judgement of ComplyEaze Bridge. Measured on one synthetic book of TallyPrime 7.1 Silver, where `excluded` has not been observed | Nothing |
| `gst_uncertain_vouchers` | The vouchers of one voucher type (a Sales type is what was measured) in a period of at most 93 days that Tally marks Uncertain in its GSTR-1, and the ones a person accepted as they stand: for each its number, date, party ledger, invoice total (without its sign, with the side) and Tally’s identifiers, and the line a CA reads before filing, which names the voucher type it covers and is withheld when no voucher was read, none is uncertain, or a voucher it depends on could not be read. The party and the total are returned to the assistant, and `mask_parties` masks the party and not the total. The reason for uncertainty is not read, and bills missing an HSN/SAC are not split out. Measured on one synthetic book of TallyPrime 7.1 Silver, before and after one Accept As Is; item invoices, a registered buyer, credit notes, foreign currency and several accepts at once are not measured | Nothing |
| `ledger_masters` | Ledgers with their opening balances; optionally GSTIN, PAN and other party details | Nothing |
| `ledger_movement` | A ledger’s opening, debits, credits and closing for a period | Nothing |
| `list_companies` | The companies loaded in TallyPrime (start here) | Nothing |
| `local_data_report` | What ComplyEaze Bridge keeps on this computer | Nothing |
| `masters` | Voucher types, godowns, units, stock groups, stock items, cost centres, cost categories or account groups | Nothing |
| `month_register` | Tally’s own month-wise Sales Register or Purchase Register for whole months: one row per month with Tally’s debit, credit and closing columns exactly as printed; not tied to the vouchers or the trial balance, and not compared with Tally’s register screen | Nothing |
| `negative_ledgers` | Tally’s own Negative Ledgers list for a window, as Tally returns it and compared with no other report; an empty list is not a finding | Nothing |
| `negative_stock` | Tally’s own Negative Stock list for a window, each item with its quantity, rate and value as Tally prints them, compared with no other report; being listed is not a finding, and an empty answer is not one either | Nothing |
| `outstandings` | Receivables and payables with ageing, the top parties and the open bills | Nothing |
| `parse_bank_statement` | Reads a password-protected SBI, HDFC, Union Bank of India or Bank of Baroda statement PDF and proposes Payment, Receipt and Contra vouchers | A private proposals file on this computer |
| `parse_bank_statement_csv` | Reads a bank's own CSV statement export in the one layout measured, and refuses every file with a named reason; it proposes no vouchers yet | Nothing |
| `profit_and_loss` | The Profit and Loss for a date range, by primary group (a book with stock items is read only when Tally’s own closing stock closes every gap) | Nothing |
| `purchase_register` | Purchase and Debit Note vouchers that touch a ledger under Duties & Taxes, as the books record them (not a GST return) | Nothing |
| `read_evidence` | ComplyEaze Bridge’s own recent reads, as fingerprints, with no figures | Nothing |
| `sales_register` | Sales and Credit Note vouchers that touch a ledger under Duties & Taxes, as the books record them (not a GST return) | Nothing |
| `statement_tie_out` | Whether the bank ledger stands where a parsed bank statement says it stood at the start and at the end of the statement's dates, or of whole calendar months of it: three gaps and the dates; the balances themselves are not returned | Nothing |
| `stock_summary` | The closing stock value of each item as of a month's last day, and its closing quantity where Tally's own Stock Summary shows the item on a line of its own; an item inside a stock group returns its own quantity from Tally's stock item list, marked as not compared; small books only (a book with many masters of any kind is refused, and so is a date that is not a month's last day); month ends inside the year were checked on two lab books | Nothing |
| `tally_status` | Whether TallyPrime’s gateway answers, and which companies are loaded | Nothing |
| `trial_balance` | The ledger-wise Trial Balance for a date range | Nothing |
| `validate_masters` | Checks ledger names against the live book before you build an import file | Nothing |
| `verify_import` | Reads back a batch you imported by hand in TallyPrime | Proof files and status records on this computer |
| `voucher_presence` | Which proposed vouchers are already in the book | Nothing |
| `voucher_read_cost` | Before a call to `vouchers`, `ledger_movement` or `verify_import`: the most counting reads (reads that count the period’s vouchers by walking the whole book, before any voucher is fetched) it will make, and the least time the gaps between them take. It follows how many voucher changes the book has seen (Tally’s counter, its voucher mark), not the period, so a narrower period does not lower it and N one-day calls pay it N times. Derived and not measured, and not an estimate of how long the call takes | Nothing |
| `voucher_schema` | The voucher-file format, without asking TallyPrime | Nothing |
| `vouchers` | The vouchers in a period. Search by voucher number, reference, narration or amount, and summaries by ledger, month, voucher type, group or primary group, are in release 0.5.0 and not in 0.4.2 (search and the first three summaries checked once, and the group and primary-group summaries in a second run, on a synthetic book of 67 vouchers; not checked on a large book) | Nothing |

With posting turned on in the extension, two more appear: `post_import` posts one saved voucher, or a saved batch of 2 to 50,
after you approve it in a ComplyEaze Bridge window (for a batch, its summary), and `acknowledge_post_review` asks you, in its own window, to record that you reviewed a posted voucher
whose ledger now points to a different master.

The Claude Desktop extension has no setting for master writes. When posting is on and `BRIDGE_AGENT_ENABLE_MASTER_WRITES=true` is also set, two more appear that create one new ledger: `prepare_ledger_creates` checks the name against the company's ledgers, groups and aliases and writes nothing to Tally, and `post_ledger_creates` sends the create once after you approve it in a ComplyEaze Bridge window and reads the book back. It was run once on a synthetic company on TallyPrime 7.1 Silver, and once on the release-candidate package of 0.7.0 on TallyPrime 7.2 Silver (30 October 2026). The [guide](docs/agent/README.md) has the limits.

## How it handles your books

- **Local connection only.** It talks to Tally's own XML gateway on
  a loopback address, so it cannot be pointed at a remote Tally host. It cannot
  tell whether that local port is forwarded to another machine; do not forward
  one across the internet. The Tally connection sends nothing to a ComplyEaze
  server.
- **Your AI provider sees what the assistant reads**, just as it sees the rest of
  the conversation, including company, party and ledger names, amounts, dates,
  narrations, references, GSTINs and, when it reads ledger details, PAN, bank
  account numbers, IFSC, MSME or Udyam registration numbers, email, phone and
  address, and, when it builds a voucher file, that file's local path (which can
  include your computer user name). The setting takes one value, not both:
  shorten party and ledger names and bank account numbers to their first two and
  last two characters (four or fewer become “…”), or drop narrations. Neither
  hides amounts, company names, PAN, GSTIN, IFSC, MSME or Udyam registration
  numbers, contact details or references, or the last four digits of a bank
  statement's account number, and a shortened name can still be
  identified, not least from the GSTIN or PAN sent beside it. See *Before you
  use it with client data* below.
- **Posting is off by default in the extension.** Note that this is a new
  setting: if you turned posting on in an earlier version, turn it on again.
  When you turn posting on, each post waits for your approval in a separate
  ComplyEaze Bridge window (for a batch, its summary). On Windows that window
  also takes the keyboard: in a test of the published 0.6.0 (a full-screen Tally,
  one screen, #1780) a typed n declined it and a typed y approved it, so when you
  ask for a post, stop typing in Tally until you have answered it with the mouse.
  No Bridge tool lets the assistant approve
  it, and an approval counts only when that window returns a fresh one-time
  token. After posting, the voucher is read back from Tally so you can see what
  landed. Four known limits remain: ComplyEaze Bridge cannot undo a posted
  voucher (you correct it in Tally); a company renamed to, or loaded under, the
  target company's name (or one differing only in case or spacing) just after
  its last check could still receive the voucher, if it has the voucher's
  ledgers, and ComplyEaze Bridge cannot always say where it went or prevent it;
  a ledger renamed and replaced in that same moment could receive the
  entry, and not every such change is noticed; and the approval window covers
  only ComplyEaze Bridge, so another Tally connector in the same Claude Desktop
  that can change entries can do so without it. Read *Before you turn on
  posting* first.
- **Tool calls leave receipts** in a log on your computer: the company's Tally
  identifier, and a fingerprint of what was asked and of what came back,
  written whether the call succeeds or is refused.
- **You accept the Terms of Use first.** The extension asks you to accept the
  ComplyEaze Bridge Terms of Use (version 2026-10.1) in its settings, and every
  tool refuses with `terms_not_accepted` until you do. (In 0.5.0, after an
  update from 0.4.2, Claude Desktop may not start the extension: on a Mac with an
  error that does not mention the Terms, on one Windows computer with no error; #1413.)
- **Open source** under Apache-2.0.

## What has been run against a real TallyPrime

Each line below is recorded in the repository or on the linked issue or pull
request, unless marked as reported by the owner. Each ran on licensed
TallyPrime Silver 7.1 and synthetic companies unless stated. The
[MCP guide](./docs/agent/README.md) and
[ADR 0004](./docs/adr/0004-tally-write-safety.md) hold the full record.

- Reads: 27 checks on an unpublished macOS arm64 build (PR #228), recorded in
  the [6 September 2026 assessment](./docs/agent/ASSESSMENT-2026-09-06.md).
  Some later reads were also run live on development builds, for example the
  trial balance on a debug build with synthetic companies (#246, whose record
  does not name the Tally release or licence tier). Not every read has its own
  recorded live run.
- Posting one Journal, on a development build from 22 September 2026
  ([issue #579](https://github.com/ComplyEaze/bridge/issues/579#issuecomment-5773745569)),
  and a Payment, a Contra and two Receipts (one of three entries) with the
  approval dialog on macOS ([PR #600](https://github.com/ComplyEaze/bridge/pull/600#issuecomment-5781144386)),
  each read back as posted. These builds predate the release published on
  26 September 2026 (version 0.3.0).
- Native posts of ten batches on licensed TallyPrime Gold 7.1, in one session
  on 28 September 2026, on a development build and one client book (the import
  request was captured for nine of them); their approval step was not recorded
  ([protocol reference](./docs/tally/TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md)).
- The published 0.3.0 package on Windows x64, in Claude Desktop (with no paid plan;
  we make no claim about other plans): reads only, against licensed TallyPrime
  Gold 7.1 with one client book, on 28 September 2026, in a session separate
  from the development-build posting above (reported by the owner; no logs were
  kept).
- A candidate build of 0.4.0 on Windows 11, on 1 October 2026 (the build CI
  produced for the version pull request: the same code as the release apart
  from a comment in one test file). The maintainer installed it in Claude
  Desktop with a new Claude account that has no paid plan, against licensed
  TallyPrime Silver 7.1 holding one synthetic company. With the Terms setting
  off, a call was refused and nothing was read. With it on, `tally_status`,
  `vouchers`, `validate_masters`, `purchase_register`, `stock_summary` and
  `local_data_report` answered. One voucher post was declined in the Windows
  approval window and nothing was sent; one was approved, and one Journal was
  posted and then verified by `verify_import`. The same candidate's macOS build
  was started and read the company list, and in Claude Desktop on macOS its
  tools loaded in a chat. This was one run, not a controlled test of each
  key of the window.
- A build of 0.4.1 on a Mac, on 2 October 2026. What was run: the package CI built
  for the version pull request, not the published file. The maintainer installed
  it in Claude Desktop over an installed 0.4.0, as an upgrade; the Terms setting
  and the other settings carried over, and the extension's server process
  started again on its own after the upgrade (its start time was read from the
  process list). `tally_status` and `list_companies` then answered against
  licensed TallyPrime Silver 7.1 holding the lab's own synthetic companies.
  What was not run:
  the published file is built again on another runner, and its program file
  differs from the one tested. We ran both builds without Tally: each reports
  version 0.4.1, lists 21 tools and gives the same answer to `tally_status`. We
  have not installed the published file in Claude Desktop.
- A build of 0.4.2 on a Mac, on 3 October 2026. What was run: the package CI
  built for the release candidate, not the published file. The maintainer
  installed it in Claude Desktop on a Mac that had 0.4.1: it did not replace
  0.4.1 but installed as a second extension (the author line changed in 0.4.2,
  and Claude Desktop builds an extension's identity partly from it); the
  settings did not carry over. `tally_status` and `list_companies` then
  answered against licensed TallyPrime Silver 7.1 holding the lab's own
  companies. What was not run:
  the published file is built again on another runner, and its program file
  differs from the one tested. We ran the published Mac file without Tally: it
  reports version 0.4.2, lists 22 tools with posting off (24 with it on), and
  refuses a call with the Terms setting off. We have not installed the
  published file in Claude Desktop, and nobody on our side has installed the
  Windows package in Claude Desktop on a Windows PC.
- A build of 0.5.0 on a Mac, on 8 October 2026. What was run: the package CI
  built in the merge queue for the version pull request, not the published file. The maintainer
  installed it in Claude Desktop over an installed 0.4.2: it replaced 0.4.2 as an
  update, with the Tally port, Response redaction and the posting setting as they
  were; the old Terms setting was gone and the new one had no value.
  Before the box was ticked, and before any quit, Claude Desktop did not start
  the extension, and a
  request failed with a 400 error that did not mention the Terms. After it was
  ticked and saved the extension started at once and `tally_status` answered,
  though the assistant first called a leftover second entry for the same
  extension, which failed the same way; after Claude Desktop was quit and reopened there was one
  connector and the company list was returned. What was not run: the published file, quitting and
  reopening without ticking the box, installing over a release older than 0.4.2,
  a fresh install, and Windows.
- A package for 0.5.1 on a Mac, on 8 October 2026. What was run: a package built on that Mac from the Terms-setting
  change (#1436), with only its version set to 0.5.1; not a published file and not the CI package. The contributor
  installed it in Claude Desktop 2.26454.2 on macOS 26.6.2 with Tally not running and posting off: as a fresh install; over 0.4.2 (switched
  on with its Terms accepted; left off and then switched on); over 0.5.0 as an update
  leaves it and with the Terms ticked; and after 0.4.2 was removed. Claude Desktop started the program as soon as
  the extension was switched on each time, except once, over a 0.4.2 left switched off, where the assistant had no
  ComplyEaze Bridge tools until Claude Desktop was quit and reopened. With the Terms off, the call was refused and
  named the Terms; with them ticked, it reached the Tally check and reported Tally unreachable. A user-level
  `BRIDGE_TERMS_ACCEPTED` set to true did not change that: the program's own value stayed false. The record is on
  #1436. What was not run: a read of a live Tally with it, the published file, and Windows.
- Builds of 0.5.1 on a Windows computer, on 9 October 2026, India time (#1413). One Windows 10 computer (22H2), Claude Desktop
  2.26454.2 from the Microsoft Store, TallyPrime Release 7.1 Silver, a company made for the test. What was run: the CI
  build of commit `d744d25a` (an earlier commit than the published file; later commits reworded some tool texts and the notes): a fresh install; an update from 0.4.2 and from 0.5.0 (as an update leaves it, and with
  the Terms ticked); a user-level `BRIDGE_TERMS_ACCEPTED` set to true; the company list; one bank-statement import
  file of six vouchers, built under 0.5.0 and under this build and compared, and this build's file imported by hand and read back (the verification found
  all six, the narrations were read back, and an August trial balance agreed with Tally's own screen; a first build under 0.5.0 into the
  company, which then held ledgers but no voucher, was refused as `empty_book_first_import`, so one more ledger and one 1 April voucher were
  imported into Tally outside ComplyEaze Bridge first, and that voucher is in the trial balance); an unchanged
  re-import of that file (matched to the six vouchers, none added); and a switched-off extension through two updates (it stayed off, with no tool and no error in the chat; switching it on was enough).
  Then steps 1 and 2 again on the CI build of the merged commit `b91038a9`, and step 1 on the published 0.5.1 file
  (checked against its `.sha256` file and its build attestation), with Tally closed. The published 0.5.0 file over
  0.4.2 was not started on this computer: the chat had no ComplyEaze Bridge tool and showed no error. What was not
  run: a changed re-import, steps 2 to 11 on the published file, posting, and other Windows versions, Tally editions
  or Claude Desktop builds. The record is the contributor's reports on #1413.
- Builds of 0.6.0 on a Windows computer, on 16 October 2026 (#1148). One Windows 10 Home 22H2 computer, Claude Desktop
  2.37337.1.0 from the Microsoft Store, TallyPrime 7.1 Silver on port 9001, one new synthetic company. What was run: the CI package
  of the release pull request (commit `a3b7316947cdf3c5ce46615c8b89e45d538345f3`, `bridge-tally.mcpb` SHA-256
  `6b83ca74377d7ed86a2b524f4d53bd2e9753b3318bfa6634078d5f5645184022`, `bridge_mcp.exe` SHA-256
  `e9780806594e60afabdb83d35bb751f0aeefba708b2b63c33b11452bbf0e5a84`) with the one posting setting on: a Journal, a Receipt,
  a Contra and a Payment posted once each with no voucher number, and one batch of three (a Payment, a Receipt and a Contra)
  posted after one approval. Every post created what it sent, and every voucher read `posted_verified` in the post's own
  readback and again in `verify_import`. The approval window showed in front of a full-screen Tally and of Claude Desktop (one screen, one run), with No as the
  default, and took keyboard focus (a Down arrow meant for Tally moved the selection inside the window; Enter alone refuses,
  arrow keys then Enter can reach Yes; a typed letter was not tried in that run; later that day, on the published file, a typed n declined the window and a typed y approved it: #1780). Then a package built by the release
  workflow's own steps (rehearsal run 38885882661, `bridge_mcp.exe` SHA-256
  `10e4614f74eab975897834ec1c9750edbe9fc7ad91584b802e5cde57a8180899`) was installed over it, kept its port and posting settings (the same version, not an upgrade), and posted one more
  Journal the same way. The published file (`bridge-tally-mcp-v0.6.0-windows-x64.mcpb` SHA-256
  `7bf81f08e058fc7ae00ccc51b3dceec10dc50d45dca8477a0438fd9b5a2852ca`, `bridge_mcp.exe` SHA-256
  `1e41e039fbf3daaf6be5a3c8e2f5e0bd1245a745abc5e78349fd83ce678224c1`) is neither of those: its exe differs from the rehearsal's in
  77 of 26,503,168 bytes, and on a live TallyPrime it was run only in the probes of 16 October 2026 (#1780) noted above: a typed n declined; a typed y approved a post that came too late in the call to be sent; the call made again 27 minutes later, after that approval had expired, opened a new window; answered with a mouse click on Yes, it posted one Contra, read back `posted_verified`. What was not run on any of these builds, among others: a batch of Journals, a batch of
  more than 3, a refused or partly landed post, amendments, the review window, an On Account approval, the retired batch
  variable, the voucher-numbering rows (#724, #1668), `voucher_read_cost` against a real `vouchers` call, the Store-install
  proof of the lock (#1584, #1459), a second monitor, the batch `headline` on a live post, the Import menu, the desktop app,
  Sales invoice posting (not offered in 0.6.0), Gold, macOS, other Windows or Tally releases, and an upgrade from 0.5.1. The
  record of the pass is in the 0.6.0 entry of the changelog; the comparison with the published file was made from the release
  assets after the release.
- The published 0.6.1 file for Windows, on 27 October 2026. `bridge-tally-mcp-v0.6.1-windows-x64.mcpb` (SHA-256
  `7812d8ad7b0ff6a07eac20828dfedaef4d50104bd5d679db3fc03c26e784c47a`, equal to its `.sha256` file, smoke report and provenance
  record; `bridge_mcp.exe` SHA-256 `bdd428e4806bdbf70925956f90d410df48a0470b1a2f78c746c4c18088471d8d`), unpacked and started
  directly by our test client (not installed in Claude Desktop; the posting and import settings unset, so it listed 29 tools),
  against licensed TallyPrime 7.2 Silver with one synthetic company: it reported version 0.6.1, and `tally_status`,
  `list_companies` and `ledger_masters` answered. Nothing was posted. Not run: the macOS file, Claude Desktop, posting,
  `audit_test_run` with its setting on, any other read. Record:
  [`lab/061-post-publish-answers`](https://github.com/ComplyEaze/bridge/tree/647a64014ce7369acc611b7adc3de63bd80df708/lab/061-post-publish-answers)
  at commit `647a64014ce7369acc611b7adc3de63bd80df708`.
- The release-candidate package of 0.7.0 for Windows, on 30 October 2026: the CI package of master commit
  `026ba163dfcbf75d515a24d0dbbf14c5245534ec`, which holds every code change of 0.7.0 (`bridge-tally.mcpb` SHA-256
  `ec689fd7a664c9d36146648f5839c9d1487cc2d18ba3d5bcd14681e0b8a33bb8`, `bridge_mcp.exe` SHA-256
  `576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c`; it reports version 0.6.1, because the version change
  merges after it), started directly by our test client (not installed in Claude Desktop), against licensed TallyPrime 7.2
  Silver, each row on its own synthetic company. The live pass of writes (one approved post, posted and verified, and a batch
  refused before its approval box), one ledger create (declined with nothing sent, then created and read back as
  `posted_verified`), the cost-centre summary of `vouchers` and a ledger asked for by its alias all passed. Not run: the
  settle dialog on a whole refusal (Tally's period lock needs user security), the approval boxes with Claude Desktop as the
  server's parent, the macOS package, and the published 0.7.0 file. Record:
  [`lab/0.7.0-rc-sitting-answers`](https://github.com/ComplyEaze/bridge/tree/ed05b6ff200e4a8f5390b9482d4e73af1cf3b160/lab/0.7.0-rc-sitting-answers)
  at commit `ed05b6ff200e4a8f5390b9482d4e73af1cf3b160`.
- The bank-statement path through its fourth step, on a Mac, on 4 and 5 October
  2026, with that same CI-built 0.4.2 candidate (installed on 3 October; not the
  published file), against licensed TallyPrime Silver 7.1 holding the lab's own
  synthetic companies. The synthetic HDFC-format statement parsed (six rows, its
  totals matched the figures supplied); the name check found no "Suspense" ledger
  and building the file was refused, with no file written; after "Suspense" was
  created in Tally by a separate one-ledger import, outside this tool, a file of
  six vouchers was built. We have not imported
  that file or read it back, and no real statement has been run.

Not yet run by us in a controlled test: posting with a published package
against a live TallyPrime beyond the one Contra of 16 October 2026 (#1780); each way of declining in the Windows approval window
(one was tried); the tools answering through Claude Desktop on macOS after the
Terms are accepted (`tally_status` and `list_companies` answered on a CI build
of 0.4.1, again on a CI build of 0.4.2 and on one Mac on a CI build of 0.5.0); posting on TallyPrime Education; posting on TallyPrime Gold
with its approval step recorded. Each release package is built and launched,
its tool list checked and a synthetic encrypted bank statement parsed, on
hosted CI runners for Windows x64 and Apple Silicon Mac.

## Not in the latest release

- Stock reads on books with many masters of any kind; sales, purchase or tax posting; creating any master but one
  ledger at a time (which needs a setting the Claude Desktop extension does not offer); bill-wise allocation
- TallyPrime Education mode: the trial balance, Profit and Loss, Balance Sheet, the
  cash flow, the month register, `company_features`, the stock summary and the
  `masters` read are refused; so is a read of vouchers,
  ledger movement, either register or another date-window read whose window starts
  or ends on a day other than the 1st, 2nd or 31st; so is an import file or a post
  with a voucher dated on any other day (posting on Education has not been run)
- The stock summary on a book with inventory turned off, or on a company split by
  year when a sibling year's company is loaded at the same time
- Deleting or undoing a posted voucher (correct it in Tally)
- Reads on very large books can fail or take longer than the assistant waits
  (#485, #703)
- A base currency other than INR. On a book with several currencies: the
  foreign-currency ledgers and vouchers themselves (they are set aside or
  withheld, and named), ledger movement, Profit and Loss and Balance Sheet,
  the purchase and sales registers (each refuses the whole read, with no rows, if
  one voucher in it names a set-aside ledger), and posting
- Tally Cloud Access or any remote Tally host
- Intel Macs, and a code-signed installer
<!-- llms:end -->

## Is this for you

It is aimed at a practising accountant or a CA firm that already keeps client
books in TallyPrime and wants to ask questions of them, or post entries into
them, through an AI assistant such as Claude Desktop.

**What it does today**

- **Reads** the loaded companies, ledger masters, trial balance, vouchers in a
  date window, outstanding receivables and payables, and ledger movement.
- **Checks ledger names before you post.** Give it the names from a bank
  statement or an invoice and it reports which exist in the book and which are
  near-misses needing your decision. Reading the ledger list first is the single
  biggest cause of an import being rejected wholesale when it is skipped.
- **Records what it did.** Every tool call Bridge runs — read or write, and
  whether it succeeds or is refused — appends a receipt to a log on your own
  machine, identifying the company it touched and fingerprinting what was asked and
  what came back. Reads keep those fingerprints as evidence alongside. A
  prepared batch records the local endpoint it was built for, and a native posting
  is refused if that endpoint has changed since; that is a safety check kept in
  Bridge's internal import ledger, not a line in the proof report a reviewer
  opens. A reviewer can read the log rather than take a summary on trust.

## Before you turn on posting

**Whether writing is on depends on how you installed it.** Everything above is
reading. When writing is off, the write tools do not merely refuse — they are
**absent from the tool list entirely**, so an assistant cannot see that they
exist.

- **The Claude Desktop extension turns voucher posting off by default.** Four
  known limits in posting remain. Tally aims an import at a company by its name and cannot bind it to a company's GUID. Bridge's last request before the post checks that exactly one loaded company has the target's GUID and name, and that no other loaded company has the same name ignoring case and spacing; otherwise it refuses the post ([#607](https://github.com/ComplyEaze/bridge/pull/607)). A company renamed to, or loaded under, the target's name (or one differing only in case or spacing) in the moment after that check could still receive the voucher, if it has the voucher's ledgers. Bridge may flag afterwards that the loaded companies changed, but cannot always say where the voucher went, and cannot prevent it (accepted residual, [#574](https://github.com/ComplyEaze/bridge/issues/574)). A ledger renamed and replaced in that same moment means the post can land in the replacement ledger. Bridge marks the result as needing reconciliation when it sees that the ledger now resolves to a different master; a change that leaves the company's master mark unmoved, or is reverted before that check, is not seen, and a regroup in that moment is not detected ([#623](https://github.com/ComplyEaze/bridge/pull/623)). And Bridge has no tool to delete or undo a voucher it has posted, so a wrong post must be corrected by hand in Tally. It records the REMOTEID each post sends, but no delete tool exists yet ([#579](https://github.com/ComplyEaze/bridge/issues/579), [#582](https://github.com/ComplyEaze/bridge/pull/582)). And the approval window covers only ComplyEaze Bridge: another Tally connector in the same Claude Desktop that can change entries can do so without it. Turning on
  **Allow voucher posting, one voucher or a batch of up to 50** in the extension
  settings adds `post_import`, which posts one saved voucher of those types, or a
  saved batch of 2 to 50 Journals, or of 2 to 50 Payments, Receipts and Contras;
  every posting still waits for your approval in a separate Bridge dialog (for a
  batch, its summary). For a batch there are two more limits: if Tally creates
  only some vouchers of a batch, Bridge reports which ones it finds in the book
  and which it does not, and refuses to post the missing ones again (a voucher it does not find may still be in Tally, edited: check
  each in Tally and do not re-enter it before you check); and a batch of more than 50, or one whose summary does not fit
  the window, is refused. Leave the setting
  off unless you accept those risks. Voucher file preparation and bank-statement
  parsing, which write nothing to Tally, stay available with the setting off.
  This is a new setting: if you turned posting on in an earlier version, turn it
  on again.
- **A source build turns writing off by default.** Preparing a file needs
  `BRIDGE_AGENT_ENABLE_IMPORT`; posting additionally needs
  `BRIDGE_AGENT_ENABLE_WRITES`, which grants both and covers one voucher and a
  saved batch of 2 to 50. `BRIDGE_AGENT_ENABLE_BATCH_POST` is retired: while
  posting is on and it is still set, `post_import` refuses as `setting_retired`;
  remove it.
- **A source build also needs `BRIDGE_TERMS_ACCEPTED=true`.** The extension asks
  for that as its "I accept" setting; without it every tool refuses.

With writing on:

- **Prepares vouchers as a local file** — Journal, Payment, Receipt and
  Contra. Bridge writes the file; it does not send it.
- **Posts one saved Journal, Payment, Receipt or Contra per approval, or a saved
  batch of 2 to 50 after one approval of its summary**, and only after you
  approve that exact voucher (or that batch's summary) in a dialog on your own
  machine.
  The assistant cannot approve it. Bridge then reads the voucher back so you
  can see what actually landed. You can also import a prepared file through
  Tally yourself; `verify_import` then reads that back.
- **Posting has limits.** It creates no masters, posts no sales, purchase, tax
  or inventory entries, and never alters or deletes a voucher. A company with
  more than one currency defined is refused.
- **A bank line is posted at most once.** Bridge sends each bank-statement line
  to a company at most once from this computer, and will not send it again as a
  new voucher, even if the post failed, the reply was lost, or the voucher was
  later changed or deleted in Tally. If Tally does not hold it, enter it in
  Tally yourself; do not edit the statement to get it past Bridge. It cannot
  see a post made from another computer or after a reinstall, a hand-typed
  entry that differs in date, type, ledgers or amount, the same statement
  downloaded from another place when the posted voucher was since changed, or a
  hand import of the file Bridge saved. The full rule and its six limits are in
  [the agent guide](docs/agent/README.md#a-bank-line-is-posted-at-most-once).

**What it does not do**

- **The Tally path uploads nothing to ComplyEaze.** Bridge reads it over a local
  connection and hands it to the assistant you are talking to; nothing in the
  Tally path sends it to a server of ours.
- It will not post anything without a separate, explicit step after the file is
  prepared.
- It is not a Tally replacement, a reporting suite, or a filing tool.

## The desktop app

**The extension is built from the same source library as the desktop app.**
Packages up to 0.3.0 contained an unfinished document-upload feature and an
AXAL sign-in, which no tool of the extension reached. That code was removed
([#914](https://github.com/ComplyEaze/bridge/pull/914)) and releases from 0.4.0 on do
not contain it; the only network client in ComplyEaze Bridge's own code
connects to Tally on your own computer. No desktop installer is published.
See [Security and privacy](./docs/security-and-privacy.md).

## Before you use it with client data

**One thing to understand before you use it.** When you ask an AI assistant for
financial data through Bridge, the assistant's provider sees what it reads,
including company, party and ledger names, amounts, dates, narrations,
references, GSTINs and, when it reads ledger details, PAN, bank account numbers,
IFSC, MSME or Udyam registration numbers, email, phone and address, and, when it
builds a voucher file, that file's local path (which can include your computer
user name). That is a property of using a hosted assistant, not of Bridge.
ComplyEaze Bridge can, before sending, shorten party and ledger names and bank
account numbers to their first two and last two characters (four or fewer become
“…”), or drop narrations (`BRIDGE_AGENT_REDACTION` takes one value, not both).
**Neither hides amounts, company names, PAN, GSTIN, IFSC, MSME or Udyam
registration numbers, contact details or references, or the last four digits of
a bank statement's account number, and a shortened name can still be identified, not least from the GSTIN or PAN sent beside it** — figures
always go with the answer. Decide this deliberately for client data.

## What it costs

ComplyEaze has not set a price for ComplyEaze Bridge and does not sell licences
to it; there is no account with us, subscription or licence key. The code of a
release you download stays under the licence it was published with (Apache-2.0
for current releases). We have not decided whether to charge for anything in future.

What you pay or provide today:

- **TallyPrime:** your own licence.
- **Claude Desktop:** Anthropic's plans. On 1 October 2026 we ran a candidate
  build of 0.4.0 (not the published file) on Windows with a Claude account that
  had no paid plan, on one synthetic company; we make no claim about other
  plans or larger books.
- **Your clients' data:** what Claude reads goes to Anthropic, under
  Anthropic's terms for your plan; through ComplyEaze Bridge, ComplyEaze does
  not receive it
  ([Privacy Policy](https://bridge.complyeaze.com/privacy.html), sections 4 to 6).
- **Your checking:** check results in Tally before you rely on them.

Support and updates are not guaranteed
([Terms of Use](https://bridge.complyeaze.com/terms.html), section 4.4). Our
liability is limited as section 14 sets out, including its fallback and
exceptions; read it before client work.

## Installing it

**Before you install, turn on Tally's HTTP gateway.** TallyPrime does not
listen for ComplyEaze Bridge by default. In Tally's own connectivity / client-server
configuration settings, set Tally to act as a server (**"acts as Both"** in
Tally's own words) and note its HTTP gateway port — `9000` by default, but
configurable. To check it is actually on, open `http://localhost:9000/status`
(substitute your port) in a browser: a running gateway answers with a short
Tally XML response, and a browser that cannot connect means the gateway is
still off — **unless Tally is running in a Windows virtual machine on a Mac**,
in which case run this check inside that VM, or only once your local
forwarding is working. A Mac browser that cannot connect may mean the
forwarding described below is missing rather than that the gateway is off.
If instead it hangs without answering, Tally may simply be busy behind
another request — wait and retry rather than changing the setting.

The [latest published release](https://github.com/ComplyEaze/bridge/releases/latest)
of the Claude Desktop extension is the one to install.
Follow the [installation guide](./docs/agent/INSTALL.md) to install and configure
it. Before you do, know what it is and is not:

- **Bridge is still being developed.** A release may contain errors, so try it
  on test data first and keep current backups. It is not yet code-signed or
  notarized, so your operating system may warn before opening it.
  Each package has a `.sha256` file and a provenance record so you can confirm
  exactly which bytes and which source commit you downloaded.
- **Checked only as far as launching.** The release build confirms the package
  starts and lists its tools. It does **not** establish that it works against
  your Tally, or in conversation inside Claude Desktop. What has been run
  against a real TallyPrime, and on which builds, is
  [listed above](#what-has-been-run-against-a-real-tallyprime); the published
  0.6.0 package itself has been run by us against a live TallyPrime on one day, 16 October 2026 (#1780), where it posted one Contra to a synthetic company; the program in the published Windows 0.6.1 file has had one read-only run, on 27 October 2026, and has not posted (see the 27 October 2026 entry above), and the macOS 0.6.1 file has not been run against TallyPrime; two other builds of 0.6.0, made by
  our checks from the same source, were run against TallyPrime 7.1 Silver on one Windows computer
  (see the 16 October 2026 entry above).
- **Windows x64 and Apple Silicon Macs only.** Intel Macs are not supported.
- **On a Mac, Tally must run on that same Mac**, in a local Windows virtual
  machine or through approved local forwarding. Bridge only talks to Tally on
  your own computer, so a separate PC or a Tally elsewhere on your network
  cannot be reached by typing its address.
- **It does not update itself.** To upgrade, install a newer release from
  Claude Desktop's Extensions settings and keep the data folder. Release 0.4.2
  installed beside an older release instead of replacing it (seen on a Mac; not
  tried on Windows). A build of 0.5.0 made by our checks (not the published
  file) replaced 0.4.2 on one Mac and kept its Tally port, Response redaction and
  posting setting: there, tick the new Terms setting, check the posting setting,
  then quit and reopen Claude Desktop. Installing over a release older than 0.4.2
  was not tried: remove it first. If Settings, Extensions lists two ComplyEaze
  Bridge entries (as happened when 0.4.2 was installed beside an older release),
  remove the older
  entry before you use either, enter your settings in the new one, including
  Response redaction, which starts at none, tick the new Terms setting, and quit
  and reopen Claude Desktop.

The Bridge **desktop application** is a separate program and has no published
installer; building it from source is described under *Contributor quick start*
below.

---

The rest of this file is for people working on Bridge. The repository is
self-contained: build and development commands resolve files relative to the
clone, not to a developer-specific directory. It holds a Tauri desktop
application and the MCPB packaging path for Claude Desktop, with
React/TypeScript and Rust components for Tally and local database
operations.

## First useful result

Install the [latest published release](https://github.com/ComplyEaze/bridge/releases/latest)
of the Claude Desktop extension with the [installation guide](./docs/agent/INSTALL.md). For source
use, the contributor quick start below builds the desktop app; to run the MCP
server from source, follow the [source MCP setup](./docs/agent/README.md).

Before requesting financial data through an MCP client, the client may send the
selected Tally result to its AI provider, including company identity, party or
open-bill details, amounts, dates, narrations, references, GSTINs and, when it
reads ledger details, PAN, bank account numbers, IFSC, MSME or Udyam
registration numbers, email, phone and address, and, when it builds a voucher
file, that file's local path (which can include your computer user name). Source
installations default to `BRIDGE_AGENT_REDACTION=none`; set it to `mask_parties`
or `drop_narration` (one value, not both) before launch when that better fits
the workflow. These settings shorten party and ledger names and bank account
numbers or drop narrations; they do not hide amounts, company names, PAN, GSTIN,
IFSC, MSME or Udyam registration numbers, contact details or references, or the
last four digits of a bank statement's account number. The package installation
settings expose the same choices.

For a first result, run `tally_status` to check that TallyPrime and its Licensed
or Education mode are observed, then list the loaded companies. Select a
company with exactly one observed INR currency master and request receivables
or payables. In Education mode, explicitly supply an `as_of` date on day 1, 2,
or 31; an omitted date defaults to today and may be refused. Bridge rechecks
product, mode, dates and currency for the financial read; other or unobserved
products/modes, no currency master, non-INR, or multiple currency masters are
refused.

The desktop also offers [local XML draft preparation](./docs/source-drafts.md) through **Prepare file**. It preserves source observations beside editable proposals and saves a local draft for later review.

For contributors, use the setup and development path below.

## Supported development hosts

Bridge is intended to build and run on Windows and macOS 12.4 or later. Run
platform checks on a native host for each operating system; a successful build
on one operating system does not verify the other.

Shared prerequisites:

- Node.js 24 (>=24.15.0) and Corepack (`.node-version` pins the CI baseline)
- the Rust toolchain pinned by `rust-toolchain.toml`
- Perl 5 with `Locale::Maketext::Simple` for the bundled SQLCipher/OpenSSL build
- LLVM/libclang for SQLCipher binding generation (`LIBCLANG_PATH` may be required)
- the operating-system dependencies listed in the
  [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

On Windows, install the Microsoft C++ build tools, WebView2 components, and a
complete Perl distribution such as Strawberry Perl. If another incomplete
`perl.exe` appears first on `PATH`, set `OPENSSL_SRC_PERL` to the complete Perl
executable. Install LLVM as well; if `libclang.dll` is not discoverable, set
`LIBCLANG_PATH` to its directory (commonly `C:\Program Files\LLVM\bin`). On
macOS, install Xcode Command Line Tools. Bridge's macOS bundles require macOS
12.4 or later.

## Contributor quick start

Run these commands from the repository root in PowerShell, Command Prompt, or a
POSIX-compatible shell:

```text
corepack pnpm install --frozen-lockfile
corepack pnpm exec playwright install chromium webkit
corepack pnpm test
corepack pnpm run build
corepack pnpm run cargo:check
corepack pnpm run tauri:dev
```

`pnpm test` includes Chromium and WebKit evidence-drawer focus suites; installing
the lock-pinned browsers after dependencies is therefore required once for each
developer environment. `tauri:dev` starts the Vite development server and
desktop application. It does not require a fixed checkout location. The first
Rust build can take several minutes.

For a release build, run `corepack pnpm run tauri:build` on each target host.
CI-produced bundles are unsigned smoke artifacts only. Do not redistribute a
desktop installer until the signing, notarization, provenance, and rollback
gates in [the release runbook](./docs/release-process.md) are complete.

## Platform verification

Before claiming support for a platform, run the following on that platform:

```text
corepack pnpm install --frozen-lockfile
corepack pnpm run build
corepack pnpm run cargo:check
corepack pnpm run tauri:build
```

Also manually exercise the affected Tally workflows.
Vendor integrations may require host-specific software even though repository
paths and project commands are portable.

## Integration trust boundaries

Bridge restricts native network and file access even if the renderer is
compromised:

- Tally connections are loopback-only (`localhost`, `127.0.0.0/8`, or `::1`).
  Remote plaintext Tally hosts are intentionally rejected.
- Showing an export in the file manager accepts only a file ComplyEaze Bridge
  exported since it started; any other path the renderer sends is refused
  before anything is launched.

## Privacy and safe diagnostics

Do not commit or attach real customer, company, tax, certificate, credential,
financial, or document data. Before sharing logs, screenshots, fixtures, or
reproduction steps, replace personal and customer data with synthetic values
and remove local usernames and absolute paths. See [SECURITY.md](./SECURITY.md)
for private reporting and handling requirements.

## Repository map

- `src/` - React UI and API bindings
- `src-tauri/` - Rust core and Tauri configuration
- `docs/` - architecture, roadmap, and operational guidance
- `.github/` - issue and pull-request templates plus CI configuration

## Governance

- [Agent responsibilities](./AGENTS.md)
- [Contributor guide](./CONTRIBUTING.md)
- [Review checklist](./review-checklist.md)
- [Security policy](./SECURITY.md)
- [Rectification guidelines](./docs/rectify-guidelines.md)
- [Roadmap](./docs/step-by-step-roadmap.md)
- [Managed Git guidance](./docs/bootstrap/managed-git.md)
- [Source and asset provenance](./docs/provenance.md)
- [Release process](./docs/release-process.md)

## License

Bridge is licensed under the [Apache License, Version 2.0](./LICENSE).
Attribution notices are provided in [NOTICE](./NOTICE).
The ComplyEaze logo and icon files are not licensed under Apache-2.0; see
[NOTICE](./NOTICE) and [TRADEMARKS.md](./TRADEMARKS.md).
Tally and TallyPrime are trade marks of Tally Solutions Pvt. Ltd. ComplyEaze
Bridge is independent and is not affiliated with or endorsed by Tally
Solutions; it uses these names only to describe compatibility.
The historical `v0.1.0` release remains under the MIT license shipped with
that tag; current development source is version `0.7.0` under Apache-2.0.
