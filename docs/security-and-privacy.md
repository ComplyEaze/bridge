# Security and privacy

This page answers, in one place, what a CA or a firm's IT person asks before
installing ComplyEaze Bridge next to client books. It describes the source of
release 0.5.0, read at commit `eb6983ffe` (the tag `mcp-v0.5.0` was made afterwards, and this
page was not read again at it, except as said below). Releases 0.6.0 (tag `mcp-v0.6.0`, 16 October 2026), 0.6.1 (tag `mcp-v0.6.1`, 27 October 2026) and 0.7.0 (tag `mcp-v0.7.0`, 30 October 2026) were
published after the pull requests named here; this page was not read again at any of those tags. Seven
changes were added after that commit, and the page describes each as read in the
pull request that adds it: the cost centre and cost category lists of the
masters read, in sections 1 and 2 (#1398), the note on how long a
`ledger_movement` read took, in section 2 (#1366), where a program built
from this repository loads the PDF library from, in section 5 (#1422), the
statement party written into a bank voucher's narration, in section 2 (#1430), the
file of skipped bank-statement rows, in section 4 (#1625), and the
location shown for an import file on a Microsoft Store install of Claude Desktop, in section 4 (#1569), and the mailing details of a party ledger and the masking of its address, in section 2 (#1490). It was written from a reading
of the source code of release 0.4.0 (tag `mcp-v0.4.0`, 2 October 2026), updated for 0.4.1 in sections 3 and 7, and read
again on 7 October 2026 against the changes between `mcp-v0.4.2` and that
commit for sections 1, 2, 3, 4, 7 and 8. Sections 5 and 6 were not read again
(except the sentences on who opens a bank statement PDF, what PDFium is inferred
to read, and what is built from this repository, which were read at the tag
`mcp-v0.5.0` and again at commit `a3c1bb9d` in the pull request that changed
them, #1416),
and neither were the sentences in the other sections that describe 0.4.0 or
0.4.1 only as history. Earlier releases differ: 0.4.0 to 0.4.2 ask you to
accept Terms version 2026-10, not 2026-10.1. It was not tested on a running
system, except where a sentence says it was seen once (see the README's list
of what has been run). Unless a line says
otherwise, each answer rests on reading that source. Anything not measured on a
running system is marked **Not measured**.

To report a vulnerability, see [SECURITY.md](../SECURITY.md).

## 1. What does it read?

It reads from the TallyPrime running on your own computer, over Tally's own
local gateway: the loaded companies, ledger masters, other masters as lists
(voucher types, godowns, units, stock groups, cost centres, cost categories, ledger groups), the trial balance,
Profit and Loss and Balance Sheet, vouchers in a date window, outstanding
receivables and payables with ageing, ledger movement, a purchase register of
the tax the books record, and closing stock values per item. It also reads
Tally's own Cash Flow for whole months (the month-wise movement of the cash and
bank ledgers), and can search the vouchers it has read and add them up by
ledger, month, voucher type or group; adding up by group also reads the ledger
list and the group list. It also checks ledger names against the book, says
which of a list of proposed vouchers are already in the book, and can read a
password-protected bank statement PDF you name (SBI, HDFC or Union Bank of
India) to propose vouchers. Several of these reads refuse some books rather than
answer in part; the [changelog](../CHANGELOG.md) lists the limits. The full tool
list is in the [MCP guide](./agent/README.md).

Four tools read nothing from Tally. They read ComplyEaze Bridge's own files on
this computer: its receipt log, the evidence it kept for a read, the voucher
format it accepts, and a report of what it has stored (section 4).

It writes to Tally only if you turn posting on, and then only a Journal,
Payment, Receipt or Contra it prepared and saved: one voucher at a time, or a
saved batch of 2 to 50, each post after you approve it in a separate ComplyEaze
Bridge window (for a batch, its summary). No ComplyEaze Bridge tool can approve
it. From 0.6.0, one setting covers both. It is a new setting:
if you turned posting on in an earlier version, turn it on again. In 0.5.1 and
earlier, posting through the extension is one voucher at a time.

Every tool refuses until the Terms of Use (version 2026-10.1 since 0.5.0) are
accepted in the extension settings. In 0.5.0 the setting was marked required, and
Claude Desktop does not start an extension while a required setting has no value.
Seen on one Mac with 0.5.0 installed over 0.4.2, where the new setting had no
value: a build made by our checks was not started before the tick or any quit,
and a request failed with a 400 error; the published file, switched on, was not
started either ("Unable to connect to extension server"). Neither error mentioned
the Terms (section 4). From 0.5.1 the setting is not marked required, so Claude
Desktop starts the extension and every tool answers `terms_not_accepted`, naming
the setting (bridge#1413). The tools the assistant called gave that answer on one
Mac with a test package and on one Windows computer with the build our checks
made for 0.5.1. A server started by hand with `BRIDGE_TERMS_ACCEPTED` set to
`true` or `1` is not asked.

## 2. Does anything leave my computer?

- **Between ComplyEaze Bridge and Tally:** only to a loopback address on this
  computer. If you forward that port to a virtual machine or another machine,
  the traffic follows your forward.
- **A Windows network path, in 0.3.0 and 0.4.0 (its network forms refused from 0.4.1; some paths still pass).** The
  bank statement tool could be given a path that names another computer, and
  Windows then connects to that computer by itself. Section 3 describes it.
- **To your AI provider: yes, whatever the assistant reads.** ComplyEaze Bridge
  hands each tool result, including company, party and ledger names, amounts,
  dates, narrations, references and GSTINs and, when the assistant reads ledger
  details, PAN, bank account numbers, IFSC, MSME or Udyam registration numbers,
  email, phone and address, and, when it builds a voucher file, that file's
  local path (which can include your computer user name), month-by-month cash
  and bank movement, totals by ledger, month, voucher type or group, a voucher's reference
  date, the bill-wise ledgers waiting for your On Account approval with their
  row dates and amounts, and counts and seconds describing how long a voucher
  read or a `ledger_movement` read took (no book data), to Claude Desktop.
  Claude Desktop is the host, and it sends tool results to the AI provider you
  use as part of the conversation; that is the host's behaviour, which this
  repository cannot show. ComplyEaze Bridge can shorten party and ledger names,
  bank account numbers and, from release 0.7.0 (#1490), the address printed for a
  party ledger, to their first two and last two
  characters, or drop
  narrations (`BRIDGE_AGENT_REDACTION`, or the Response redaction setting; one
  value, not both), but neither hides amounts, company names, PAN, GSTIN, IFSC,
  MSME or Udyam registration numbers, contact details or references, or the last
  four digits of a bank statement's account number (`account_last4`), and a
  shortened name can still be identified, not least from the GSTIN or PAN sent
  beside it. The default is `none`: nothing is masked unless you choose it.
  - `mask_parties` shortens, to their first two and last two characters, the
    names of ledgers and parties wherever a tool returns them (in vouchers,
    ledger lists, the trial balance and statements, outstandings, the purchase
    register, the GST totals per counter ledger, bank-statement proposals, voucher totals by ledger, the party ledger of
    each voucher `gst_uncertain_vouchers` lists, the ledgers
    listed under a group total, the ledger named when a group total cannot be
    given, the list of bill-wise ledgers waiting for approval, the suspense ledger
    named in a build warning, the ledger in a refused cash-or-bank row and the
    ledgers on Tally's Negative Ledgers list), stock
    item names and their parents, godown, stock-group, cost-centre and cost-category
    names and their parents, the category a cost centre belongs to,
    and three fields of a ledger's details: the name on the PAN, the bank account
    holder's name and the bank details; from release 0.7.0 (#1490) also the name a
    mailing address is printed under and the address itself (each line of the mailing address, and the flat address field).
    A line is shortened to its first two and last two characters, so a house number
    or a town at either end of a line can stay readable. Every `ledger_masters` call
    with `fields=compliance` reads and returns the older flat fields (the address
    from `LEDADDRESS.LIST`, the PIN code and the state), as before: under mode `none`
    (and under dropping narrations, which masks no name) they are in clear, and under
    `mask_parties` the flat address is shortened and the PIN code and state are not. The dated mailing address (lines, PIN code, state and
    country) is read from Tally only when the assistant asks for it
    (`include_mailing_details`); without that argument `ledger_masters` does not read
    it. The flat PIN code and state, and the mailing entry's PIN code, state and
    country when asked for, are not masked under any mode: for a party that is an
    individual, such as a proprietor, they can point to the locality of a home address. A name of four characters or fewer is
    replaced by “…”. The spelling a ledger's vouchers carry, which is returned
    beside its name when the two differ, is left out. It does not mask amounts, company names, the names of
    groups in a group total (shown as the book has them), a ledger's parent
    group, the names of voucher types, units and account groups, narrations,
    references, GSTINs, PAN numbers, email addresses, phone numbers, postal
    addresses outside a ledger's own address fields, or IFSCs, or the last four digits of a bank statement's account
    number (`account_last4`), and a name written inside a narration or reference
    stays as it is.
  - `drop_narration` removes narrations, and a search of vouchers by a phrase of
    their narration is refused under it. Apart from the error text noted below,
    it removes nothing else.
  - From the release that includes #1430, a voucher built from a bank statement
    whose party is mapped to a ledger carries “Statement party:” and the name as
    the statement prints it at the end of its narration text; a file you import by
    hand adds the `[BRIDGE:...]` tag after it. A cash line the tool recognises, and
    a line it labels itself as bank charges, a card fee or an EMI, get no name; a
    line sent to suspense already carried the name. Vouchers already in Tally
    keep their narration until an amendment of their batch is imported.
    Whenever narrations are read, the name goes to your AI provider, unshortened
    under `mask_parties`; `drop_narration` removes it with the narration.
  - A tax-audit test run (`audit_test_run`, added in release 0.6.1 and listed only when its setting is on) sends the AI
    provider the test's figures and findings: amounts in paise, figure and finding ids, the test's
    fixed wording, and the code and count of any failed book check. It names no ledger, group,
    party or voucher, and it is refused while `mask_parties` or `drop_narration` is on, because
    those settings mask only marked values and would not hide what is returned here. The books it
    reads for the test are held in memory for the call and are not written to disk.
  - A bank-statement check over whole calendar months (`statement_tie_out`,
    added in release 0.6.0) can show, with the book's figures,
    the statement's balance at the month ends and its net over those months,
    under every setting: never a part of a month except the part-months at the
    statement's own start and end (whose net a month-end balance gives together
    with the opening and closing supplied to the parse), but a month in which
    every other row's amount is already known (a single row, or one row beside open
    cash lines, whose amounts the parse returns) gives that row's amount.
  - **What the new reads in 0.5.0 send, under each setting.** Under `none`
    everything below is sent as read.
    - Cash Flow sends month names, each month's amounts and the net total, and
      only when the net total ties to the trial balance. When the two differ it
      sends the two net figures and counts of ledgers instead; in its other
      refusals it sends at most a count of ledgers. It sends no ledger or party
      name on any outcome. The only text from the book is the company name, its
      identifier, company number and books-from date, and the currency.
      `mask_parties` has nothing to shorten in it, and it holds no narrations.
    - Voucher search sends the same voucher fields as the vouchers read, with
      names and narrations treated as that read treats them under each setting.
    - A summary by ledger, month or voucher type sends the totals and counts,
      and the voucher numbers, dates, types and Tally identifiers of up to five
      of the vouchers behind each total. Ledger names in it are shortened under
      `mask_parties`.
    - A summary by group sends group names as the book has them under every
      setting; the ledgers listed under a group total, and the ledger named when
      a group total cannot be given, are shortened under `mask_parties`.
    - A voucher's reference date (the `reference_date` field) is a date, the
      same under every setting.
    - The cost centre and cost category lists (two new kinds of the masters
      read) send each one's name, its parent (a cost centre's parent is another
      cost centre or the top level), its category for a cost centre, three
      yes-or-no allocation flags for a cost category, and the Tally identifier
      and alteration number of each. Names, parents and categories are
      shortened under `mask_parties`; the flags are not names. In the answers
      captured they carry no amounts and no narrations. How a voucher was
      allocated to a cost centre is not returned in 0.5.0, 0.5.1 and 0.6.0 (Tally's
      answer to the vouchers read carries it, and those releases drop it); from
      release 0.6.1 the voucher reads return it (see
      "What the reads added since 0.5.0 send").
    - The list of bill-wise ledgers waiting for your approval sends each
      ledger's name (shortened under `mask_parties`), the date, voucher type,
      side and amount of each of its rows with its transaction label, its totals
      and row count, and a 64-character fingerprint (a short code made from that
      ledger's rows, the batch and the company). The approved-party list in the
      build result is shortened the same way.
    - The note on how long a voucher read or a `ledger_movement` read took holds
      counts and seconds, and no book data. Cash Flow, like the trial balance and the two statements, also
      sends a short summary in words naming the company and the period; the
      trial balance's also counts its rows.
  - Either setting also leaves out the text of an error Tally returned for a
    line. Any other value stops the extension from starting.
  - **What the reads added since 0.5.0 send, under each setting.**
    - The read-cost preview (`voucher_read_cost`) sends the company name,
      its identifier, company number and books-from date, the company's voucher
      mark, and counts and seconds derived from it, with constant text. It holds
      no amounts and no narrations, and `mask_parties` has nothing to shorten in it.
    - The company settings read (`company_features`) sends the company name,
      its identifier, company number and books-from date, the cost-centre, GST
      and batch-wise settings as yes, no or not reported, and the currency
      symbol. It holds no amounts and no narrations, and `mask_parties` has
      nothing to shorten in it.
    - The month-wise register read (`month_register`) sends the company name,
      its identifier, company number and books-from date, the register you
      chose (`kind`) and the report's name, the window's dates, and for each
      month, Tally's debit, credit and closing amounts. It holds no
      party, ledger or narration, and `mask_parties` has nothing to shorten in it.
    - The Negative Stock read (`negative_stock`) sends the company name and
      its identifier and the dates, and for each item on Tally's Negative
      Stock list, the item's name (shortened under `mask_parties`), its closing
      quantity with its unit, and its rate and value. The quantity, rate and
      value are not masked under any setting. It holds no narrations.
    - The GSTR-1 status read (`gst_status`) sends the company name and its
      identifier, the voucher type name you gave and the dates, and for each
      voucher its number, date, voucher type name, Tally's identifier and
      change number, and the voucher's GSTR-1 status (included, uncertain,
      excluded or not in the return, or that it is cancelled, optional or
      post-dated and what its status flags read) and whether it was accepted
      as it stands. It holds no amounts, no party or ledger names and no
      narrations, and `mask_parties` has nothing to shorten in it.
    - The GST-by-ledger read (`gst_by_counter_ledger`) sends the company, the
      dates, each counter ledger's name, Tally GUID, group and parent with its
      voucher count and its GST totals per head, and for each set-aside reason its totals and
      up to 100 vouchers by number, date, type, class and Tally's identifier; a voucher set
      aside as `repeats_a_purchase_bill` also names, under `repeats`, the date, number and
      identifier of the attributed voucher whose bill it is entered against.
      It sends no party names and no narrations; `mask_parties` shortens the
      ledger names and does not mask the GUID, which names no party.
    - The negative ledgers read (`negative_ledgers`) sends the company name and
      its identifier and the dates, and for each ledger on Tally's Negative
      Ledgers list, the ledger's name (shortened under `mask_parties`), the side
      and the closing amount. The side and the amount are not masked under any
      setting. It holds no narrations.
    - The uncertain-voucher read (`gst_uncertain_vouchers`) sends the company,
      the voucher type name and the dates, the counts of the whole window and
      the pre-filing line, and, only for each voucher it lists as uncertain or
      as accepted as it stands, its number, date, party ledger's name,
      invoice total (without its sign) and which side it is on, Tally's two
      identifiers and its status. It sends no voucher type and no status
      flags per item.
      Unlike the status read, its answer therefore carries party names and
      amounts to the assistant. `mask_parties` shortens the party names in
      it and does not shorten the amounts. It holds no narrations and no
      ledger entries of the voucher.
    - Cost-centre allocations of a voucher's ledger entries (added in release
      0.6.1): each entry that carries them sends, for each allocation, the name
      of its cost category, the name of its cost centre and the amount. The reads that return a voucher's entries carry them: the
      vouchers read and its search, and the entries the purchase and sales
      registers list. The two names are shortened under
      `mask_parties`, like the cost centre and cost category lists; the amount
      is not a name and is sent under every setting. An entry whose allocations
      cannot be read whole says so with a reason and sends none of them, an
      allocation in a shape the read does not recognise included. On an item
      invoice the read returns the sales or purchase ledger as a ledger entry
      with the goods line nested under it: an allocation list directly under
      that entry is read and sent like any other, and one inside the goods line
      is flagged as in a shape the read does not recognise. No captured item
      invoice carries allocations, so where Tally prints them there is not
      measured.
    - Bank allocations of a voucher's ledger entries (added in release 0.7.0),
      only when `vouchers` is asked for them with
      `bank_allocations: true`: for each allocation, at most eight per entry, its
      bank date, instrument number and date, transaction type and status. The
      payee, bank, branch and account details an allocation can carry are not
      read or sent. The instrument number is any text typed or sent, so a name typed into it
      stays as it is: it is sent under every setting, as the voucher's own
      reference is. An entry whose allocations cannot be read whole says so with
      a reason and sends none of them.
    - Goods lines of a voucher's ledger entries (added in release 0.7.0),
      only when `vouchers` is asked for them with
      `goods_lines: true`: for each line of an invoice voucher, the stock item,
      quantities, rate and amount, and each godown and batch row with its
      quantities and amount. Item, godown and batch names are masked under
      `mask_parties`. An entry whose lines cannot be read whole says so with a
      reason and sends none of them.
- **To ComplyEaze: nothing we found in the published extension's code.** We
  found no analytics, telemetry, crash reporting or automatic update check in
  it. The extension's tools reach only the Tally transport. Releases up to
  0.3.0 were built from a source library that also held a document-upload
  feature and an AXAL sign-in for the desktop app; that code was removed
  ([#914](https://github.com/ComplyEaze/bridge/pull/914)) and release 0.4.0
  does not contain it.

## 3. Which network destinations can it contact?

- The only network client in ComplyEaze Bridge's own code is the Tally
  transport. It accepts only a loopback address (any `127.x.x.x` address, or
  `::1`) or the name `localhost`, which it maps to `127.0.0.1` without a DNS
  lookup. It refuses anything else, uses no proxy and follows no redirects.
- In the program code of release 0.5.0 no `complyeaze.com` address remains
  outside a test, and the Tally transport is the only code that depends on an
  HTTP client library outside tests. The extension's manifest links to the
  privacy and terms pages and the home page on `bridge.complyeaze.com`, and to
  the project's repository, issues and install guide on `github.com`; the
  program does not contact any of them.
- CI checks this in two ways. The
  [egress check](../scripts/check-tally-egress-boundary.mjs) limits which
  source files and crates may hold network code. A second check runs the
  compiler's lints on Windows and macOS and compares every place that sends a
  network request, opens a socket or starts a process with a
  [reviewed list](../scripts/tally-egress-census.json), failing on any
  difference. At the tag that list names the Tally transport as the only place
  that sends an HTTP request; its other entries are a development simulator's
  local socket and the two places that start a process (the approval window,
  and a desktop-app command).
- Among the limits the check's own
  [header](../scripts/check-tally-egress-boundary.mjs) lists, in plain words:
  - it does not prove that the loopback restriction is correct or still wired
    up (that has its own tests);
  - the lints name specific methods, so network use through a method they do
    not name, a crate on neither list or a native library is outside them.
    PDFium, the PDF library in the package, is such a library;
  - the second check sees only the code its own run compiles (the shipped
    targets with default features, on Windows and on Apple Silicon macOS), not
    code built under other settings or features, Linux-only code, or tests;
  - nine system functions on its list were reported as resolving to nothing
    on Windows, on one run, and six of those are unexplained;
  - a client it does not name, such as the HTTP library's blocking client,
    and the desktop app's window settings are outside it;
  - an entry is a file, a method and a count, so one reviewed call swapped for
    another of the same method in the same file passes;
  - removing or narrowing the check in the CI file, dropping the setting that
    makes its warnings fail, or a build that skips the lints is not detected
    by the check itself;
  - the project's development tools are outside the lints, because they do
    not ship.

  The changelog adds that the check does not by itself prove that no data
  leaves the machine.

- **One exception, in 0.3.0 and 0.4.0 (its network forms refused from 0.4.1; some paths still pass).** The Windows network
  path ([advisory](https://github.com/ComplyEaze/bridge/security/advisories/GHSA-vm5g-r3p7-wxx7)):
  - *What was wrong.* The `parse_bank_statement` tool opened any absolute path
    it was given for the statement or the password file, including a Windows
    network path (a network share or its WebDAV form). That is the operating
    system opening a file, not a network client in ComplyEaze Bridge's own
    code, so the checks above do not see it.
  - *Impact.* On Windows, opening a path that names another computer makes
    Windows connect to it, and may send the signed-in user's network sign-in
    response there. It needs the assistant to be steered into giving such a
    path (for example by text it has read), your approval of the call in the
    assistant or an always-allow setting, and outbound file-sharing or WebDAV
    traffic to that computer. No Tally data is sent. The advisory's severity is
    medium; on macOS it is low.
  - *What 0.4.1 changes.* Both paths are checked as text before anything is
    opened. A path that begins with two slashes or backslashes, in any mix, is
    refused; this includes the Windows long-path form (`\\?\C:\...`), so use the
    plain drive path. On Windows only a path that starts with a drive letter, a
    colon and a slash or backslash is accepted.
  - *What still passes.* A drive letter mapped to a network share, a link or
    junction partway along a path, and on a Mac a mounted network volume.
  - *What to do.* Keep statements and password files on a local disk. From
    0.6.0 the tool refuses, on Windows too, a password file
    owned by neither you nor Administrators, or that anyone but you, SYSTEM
    and Administrators can read or change; with 0.5.1 or earlier, restrict its
    access yourself.

**Not measured:** a network capture of the running extension on Windows or Mac.

## 4. What is stored on my computer, and for how long?

| Where | What |
| --- | --- |
| Windows: `%LOCALAPPDATA%\Bridge\agent` | the files below |
| macOS: `~/Library/Application Support/Bridge` | the files below |

On a Microsoft Store install of Claude Desktop, Windows may keep some of these
files under
`%LOCALAPPDATA%\Packages\<Claude package>\LocalCache\Local\Bridge\agent`, and a
given file can be in either place. Measured on one computer (30 October 2026,
#1584), with ComplyEaze Bridge started from a shell that Claude Desktop runs:
the files it created went to that second folder; a copy started outside Claude
Desktop created `%LOCALAPPDATA%\Bridge\agent` and its files there; and once both
had run, each kept its own import journal. For the import file that the
`build_import_xml` tool reports, ComplyEaze Bridge shows the location Windows
reports for that file, with a `path_state` beside it that says whether Windows
reported it; there it named the second folder, and a process outside Claude
Desktop opened the file there with the same sha256. The command-line report
(`bridge_mcp --local-data-report --show-paths`) prints each folder as built (the
Windows location in the table above, unless you set another data folder); beside
it, where the files directly in that folder are as Windows reports each file
(`folder_file_locations` and `lock_folder_file_locations`, read from the first
64 files by name, with how many were read of how many; not the files in its
subfolders, such as `imports`; the listing stops at 100,000 entries, and
`listing_capped` says so; a lock folder's `native-dispatch-leases` that is
itself a link is not entered and reads `link_not_followed`); the import
journal's own location (`journal_path`; a journal file that is itself a link is
shown as built, its target not followed, with `journal_path_state`
`link_not_followed`); and, on Windows, each copy of the data folder inside a
Store app's package that exists (`package_copies`). It does not print a folder
where Windows reports the folder itself to be: once both folders existed,
Windows named `%LOCALAPPDATA%\Bridge\agent` for a view that also showed the
second folder's files. Seen from inside Claude Desktop, a file both copies held
named the second folder and a file only `%LOCALAPPDATA%\Bridge\agent` held named
that folder; from outside, every file named `%LOCALAPPDATA%\Bridge\agent`. So a
file in both copies is shown only by the copy that side reads, and a file in one
copy by that copy. `package_copies` is what shows that the second copy exists,
and a terminal outside Claude Desktop does not see that copy as part of the
folder. The report does not record which side ran it; run from inside Claude
Desktop on that computer, `package_copies` and the data folder's file locations
would both name the second folder (inferred from the lab's probe, not from a run
of this report), so such a run cannot tell whether `%LOCALAPPDATA%\Bridge\agent`
also exists. Check both folders before you archive or delete. **Not measured:**
ComplyEaze Bridge as Claude Desktop starts the extension (the measured copies
carried no package identity, yet Windows redirected their files); a copy outside
Claude Desktop running first; another Claude Desktop version; Windows 11; this
build, its file locations once both folders exist (the lab read them with a
probe making the same Windows call), and `package_copies`, which lists
`%LOCALAPPDATA%\Packages`, on a real computer. On that
one computer (#1459) the second folder held a receipt log, the terms record,
the import journal, the saved batch files (`imports/`), the bank-statement
proposals, a lock file and the lock folder (`native-dispatch-leases/`).
Microsoft's documentation says that uninstalling or resetting a Store app
(here Claude Desktop itself; this is not the removal of the extension that
section 8 describes) removes that second folder and everything in it.
**Not measured:** any other computer, where the proof files of a verification
go, and what an uninstall or reset really removes.

- **A receipt log** (`agent-egress.jsonl`): two lines for each tool call. The
  first has the tool name, the time, the company's Tally GUID, fingerprints of
  the request and the response, the names (not the values) of the fields
  returned, how many rows and bytes, whether the response was cut short, the
  redaction setting in force, an error code if there was one, and the last 32
  requests the call sent to Tally (their kind, size, outcome and time) with a
  count of the rest. The second records that the response was written out. It
  holds no row values.
- **A terms record** (`terms-acceptance.jsonl`): the extension asks you to
  accept the ComplyEaze Bridge Terms of Use (version 2026-10.1) in its settings,
  and every tool refuses with `terms_not_accepted` until you do (section 1 says
  what 0.5.0 did after an update and what changes in 0.5.1). When the server
  starts with the setting on, it appends a line with the terms version, the
  time, and that the acceptance came through the setting (once per version; two
  servers starting together can each add a line). If that line cannot be
  written, every tool refuses with `terms_record_unavailable`. The line is a
  local record that the setting was on, not proof of who accepted or of when the
  box was ticked; it is not sent anywhere. Accepting 0.5.0 adds a second line,
  for version 2026-10.1, beside any 2026-10 line from 0.4.0 to 0.4.2.
- **An import journal** (`agent-import-ledger.jsonl`) and saved batch files
  (`imports/`): the vouchers ComplyEaze Bridge prepared or posted, with their
  dates, narrations, amounts and ledgers, and for each saved batch: each ledger
  it names with the identifier (GUID) Tally gave it, the ledgers named as
  Cash-in-Hand for a bank cash answer (with the voucher label), and the
  bill-wise ledgers you approved to receive entries On Account (with the
  64-character fingerprint your approval matched when the batch was built, a
  short code made from that party's rows, the batch and the company; it is kept as a record
  and is not checked again when the batch is posted). These records hold ledger
  names and fingerprints, and no new amounts, PAN, bank account numbers or
  contact details. It also holds the proof-of-post files that hold what Tally
  read back, and a short note when an approval you gave was dropped without
  being used.
- **Bank-statement proposals** (`bank-statements/`): every row of a statement
  you asked it to read, with date, amount, bank reference and narration.
- **Skipped statement rows** (`agent-import-statement-skips.jsonl`): written
  when you build a statement in which every row is mapped `skip` (recorded
  from the other bank account's statement), so that there is no voucher batch
  to save. Each record holds an identifier of its own, the company's
  identifier (GUID), the Tally address, the time, the bank ledger's GUID and,
  when the statement covers any day in full, those dates; and for each skipped
  row an identifier (the row's date and a short fingerprint of the row), its
  date, amount, direction, the other ledger's GUID, the UPI, IMPS, NEFT or
  RTGS reference if the row has one, the cheque or instrument number as the
  statement prints it, and a mark when an equal amount goes the other way on
  the same day. It has no field for a narration or a ledger name. A statement
  that also has rows to post keeps the same facts for its skipped rows, and
  the reference and instrument of each of its Contra rows, in its record in
  the import journal: unlike the ledger records described above, these hold
  the dates and amounts of rows that are not posted.
- **Lock files** with no content, a lock folder (`native-dispatch-leases/`,
  kept in ComplyEaze Bridge's per-user folder and, on Windows, also in
  `.complyeaze-bridge\coordination` under your user profile) that keeps two
  copies of the program from sending to Tally at the same time, and a small record written
  when you record a review of a posted voucher, including your
  operating-system account name.

None of these files is encrypted. On macOS the folder is private to your user
(mode 0700) and files are 0600. On Windows the folder inherits its parent's
permissions. **Not measured:** the resulting Windows permissions.

You can ask what is stored. The `local_data_report` tool (and
`bridge_mcp --local-data-report` on the command line) counts the files, their
size and the age of the oldest by kind, says how many saved batches are not
settled, and says so when it could not read something. It changes no book
and deletes nothing; the tool form, like every call, adds its own receipt to
the log. It names no file path unless you ask on the command line, and covers
this folder only, not files kept by the desktop app.

There is no size limit, expiry or deletion command: the files grow until you
delete them. Each verification of a batch adds a new pair of proof files and
keeps the earlier ones, so `imports/` grows with every verification. You can delete them by hand when Claude Desktop is not running. Do
not delete the import journal (`agent-import-ledger.jsonl`) or `imports/` while
any batch in them has been posted or is waiting to be verified: they are the
record ComplyEaze Bridge uses to reconcile a post, and without them
`verify_import` cannot find the batch. The journal is also how it remembers
which batches and which bank-statement rows it has already sent to Tally, so
that it can refuse to send one twice. Do not delete
`agent-import-statement-skips.jsonl` either: a missing file reads as no
skipped rows, so those rows are no longer shown in the `transfers` section,
and a later build that posts one of them as its own voucher is no longer
refused for that reason: a transfer may then be booked twice. Once 0.5.0 has built a batch in this
folder, do not run 0.4.2 or earlier over it: they refuse to post that batch. The
[Privacy Policy](./legal/privacy.md) (section 7) describes archiving the whole
folder instead of deleting it, and what ComplyEaze Bridge no longer remembers
afterwards. **Not measured:** what else breaks if
only some files are deleted.

## 5. Is it signed, and how do I check what I downloaded?

It is not yet code-signed or notarized. Each package on GitHub Releases has a
SHA-256 file and a provenance record naming the source commit it was built
from. From release 0.4.0 the release workflow also records a build attestation
for each package and checks it before publishing. An attestation says which
workflow run and commit produced a file. In the workflow's own words, it is
not a code signature, no client checks it yet, and it does not show the code
is safe. The package includes an unsigned
third-party PDF library (PDFium, from a pinned release, checked by SHA-256 at
build time), used only to read the bank statement PDF you name; a program built
from this repository from #1422 on loads it from the folder that holds its own
executable, and checks no SHA-256 or other digest of it when it loads it. **Not
measured:** which files PDFium opens, and when, or whether it opens any network
connection. What the source does show: ComplyEaze Bridge, not PDFium, opens the
statement file. It reads the file itself, refusing one larger than 32 MiB, and hands
PDFium the bytes; it reads the password file itself as well. PDFium probably
also reads fonts installed on the computer. That is inferred, not observed: on
the crate's synthetic test statements, which use a standard PDF font without
embedding it, the word boxes come out shifted vertically by a constant, and the
crate's note explains this as PDFium substituting an installed font. Which font
file it opened was not observed.

## 6. Has it been independently reviewed?

**No independent security audit has been done.** The project runs its own checks
in CI, but that is not an external review. This repository's source is public,
under the Apache License 2.0, and the release workflow builds the program from
it, with the Rust dependencies its lock file pins. The bundled PDFium is not
built from this repository: it is a third-party build, downloaded from release
`chromium/7881` of `bblanchon/pdfium-binaries` and shipped byte for byte as
that release's archive holds it, and nothing here rebuilds it. It is under its own
licences, not this repository's; the package reproduces its licence files in
`THIRD_PARTY_LICENSES_PDFIUM.txt`.

## 7. Does it need administrator rights, or install anything else?

The package declares only a command to run and its settings. In its source we
found no service, driver, scheduled task, registry key, launch agent or
listening port. It writes files only in its data folder and in the lock
folder described in section 4. It also reads files named in a tool call (often
by the assistant), such as a bank statement and its password file. From 0.4.1 it
accepts such a path only when its text starts with a drive letter, a colon and a
separator (Windows) or a single slash (a Mac) (section 3). It refuses a password
file that someone else can read: on macOS one that is not yours or has any group
or other permission set (it does not read macOS access-control lists), and, from
0.6.0, on Windows one owned by neither you nor Administrators,
or whose access list lets anyone but you, SYSTEM and Administrators read or
change it. Its one extra process is a
second copy of itself that shows the approval window.

The approval window is a system dialog. On macOS it is titled "ComplyEaze Bridge
— approve one voucher" with the buttons Cancel and Post voucher. On Windows it
is titled "ComplyEaze Bridge — post this voucher?" with Yes, No and Cancel, and
No is the default. Only the positive button approves. On Windows the window
also takes the keyboard (#1780). In tests on Windows 10 with TallyPrime 7.1 full
screen and one screen, each tried once, a Down arrow moved its buttons on the
0.6.0 release candidate, and on the published 0.6.0 a typed n declined it and a
typed y approved it, so a key typed for Tally can answer it. When you ask for a post, stop typing in Tally
until you have answered the window with the mouse. The review acknowledgement
window is built with the same flags in the code, so it is expected to behave the
same; that was not measured, and neither was macOS. The window closes by
itself after two minutes without an answer, and an approval that is not used
within fifteen minutes, or before the program restarts, is dropped. From 0.4.1 a
clicked approval is also withdrawn when the post is refused before its checks
begin (for example when the batch is not found or the journal is busy); a
refusal inside the checks already withdrew it. In 0.5.0 a window for one voucher
puts "On Account" before the name of each ledger you approved for it and shows
the voucher's own reference and narration as its last two lines; if they hold
characters it cannot show safely, or do not fit, it refuses. From 0.6.0,
a narration too long for one line is shown instead in numbered parts, each
quoted and split only at a space, under a line that counts them; one that
cannot be split so, and a reference too long for one line, are still refused.
The review window for a posted voucher shows its narration last, under the
same heading (#1535). A window for a
batch of up to ten vouchers lists each on one line (type, date, amount, ledger
and up to 40 characters of its narration, with no reference; from 0.6.0
a Journal's line names its first debit and first credit ledger and up to
20 characters of its narration, fewer (never under 10) when that makes the line
fit, and the line above says a narration may be cut) and marks the
approved ledgers On Account on its totals lines. A batch of more than ten
vouchers, or whose lines cannot be shown safely or do not fit, shows its totals
only, with a line saying why, and still asks for your approval, unless a ledger
or company name cannot be shown safely or the totals do not fit, when it
refuses. An approval also lapses when Tally's port is busy before anything is
sent, and you are asked again.

The window covers only ComplyEaze Bridge's own tools. Another Tally connector in
the same Claude Desktop that can change entries can do so without it.

**Not measured:** what Claude Desktop itself needs when installing an extension,
and a check of a machine before and after installing and running it.

## 8. How do I turn it off, or remove it?

- **Turn posting off:** switch "Allow voucher posting, one voucher or a batch of up to 50" off in the extension
  settings (off is the default; it is a new setting, so check it after you
  update from an earlier version), then quit Claude Desktop completely and reopen it.
  `post_import` and `acknowledge_post_review` are then absent from the tool
  list and refuse if called. Preparing voucher files and reading bank
  statements stay available, and still write to the folder in section 4.
  **Not measured:** whether Claude Desktop applies the change without a
  restart.
- **Remove it:** uninstall it from Claude Desktop's Extensions settings.
  ComplyEaze Bridge never changes Tally's gateway setting, so uninstalling it
  does not either. **Not measured:** what the uninstall
  removes. ComplyEaze Bridge has no uninstall step of its own and nothing in it
  deletes the files in section 4; delete them yourself.
- **Vouchers already posted stay in Tally.** ComplyEaze Bridge has no tool to delete or
  undo one; correct it in Tally.
