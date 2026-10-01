# Security and privacy

This page answers, in one place, what a CA or a firm's IT person asks before
installing ComplyEaze Bridge next to client books. It describes the latest
published release (version 0.4.0). It was written from a reading of that
release's source code, at the tag `mcp-v0.4.0`, on 2 October 2026. It was not
tested on a running system (see the README's list of what has been run).
Unless a line says otherwise, each answer rests on reading that source.
Anything not measured on a running system is marked **Not measured**.

To report a vulnerability, see [SECURITY.md](../SECURITY.md).

## 1. What does it read?

It reads from the TallyPrime running on your own computer, over Tally's own
local gateway: the loaded companies, ledger masters, other masters as lists
(voucher types, godowns, units, stock groups, ledger groups), the trial
balance, Profit and Loss and Balance Sheet, vouchers in a date window,
outstanding receivables and payables with ageing, ledger movement, a purchase
register of the tax the books record, and closing stock values per item. It
also checks ledger names against the book, says which of a list of proposed
vouchers are already in the book, and can read a password-protected bank
statement PDF you name (SBI, HDFC or Union Bank of India) to propose vouchers.
Several of these reads refuse some books rather than answer in part; the
[changelog](../CHANGELOG.md) lists the limits. The full tool list is in the
[MCP guide](./agent/README.md).

Four tools read nothing from Tally. They read ComplyEaze Bridge's own files on
this computer: its receipt log, the evidence it kept for a read, the voucher
format it accepts, and a report of what it has stored (section 4).

It writes to Tally only if you turn posting on, and then only one voucher at a
time (a Journal, Payment, Receipt or Contra it prepared and saved), after you
approve that voucher in a separate ComplyEaze Bridge window. No ComplyEaze
Bridge tool can approve it. The server program also has a setting for posting 2
to 50 vouchers of one saved batch after one approval; the published extension
neither offers nor sets it, so it applies only if someone sets it by hand.

Every tool refuses until the Terms of Use are accepted in the extension
settings (section 4).

## 2. Does anything leave my computer?

- **Between ComplyEaze Bridge and Tally:** only to a loopback address on this
  computer. If you forward that port to a virtual machine or another machine,
  the traffic follows your forward.
- **To your AI provider: yes, whatever the assistant reads.** Claude Desktop
  sends the results a tool returns, including company names, party names and
  amounts, to the AI provider you use, as it sends the rest of the
  conversation. ComplyEaze Bridge can mask party names or drop narration
  (`BRIDGE_AGENT_REDACTION`, or the Response redaction setting), but neither
  removes amounts. The default is `none`: nothing is masked unless you choose it.
  - `mask_parties` shortens the names it treats as party names (the party and
    ledgers of a voucher, ledger and other master names and their parents,
    stock item names, and the name on a PAN, a bank account holder name and
    bank details) to their first two and last two characters; a name of four
    characters or fewer is removed entirely. It does not touch company names,
    narrations, GSTINs or amounts.
  - `drop_narration` removes narrations and nothing else.
  - Either setting also leaves out the text of an error Tally returned for a
    line. Any other value stops the extension from starting.
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
- In the source of release 0.4.0 no `complyeaze.com` address remains, and the
  Tally transport is the only code that depends on an HTTP client library
  outside tests.
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
- The check's own header lists what it does not prove: that the loopback
  restriction is correct or still wired up (that has its own tests), and
  network use through a method the lints do not name, such as a native
  library. PDFium, the PDF library in the package, is such a library. The
  changelog adds that the check does not by itself prove that no data leaves
  the machine.

**Not measured:** a network capture of the running extension on Windows or Mac.

## 4. What is stored on my computer, and for how long?

| Where | What |
| --- | --- |
| Windows: `%LOCALAPPDATA%\Bridge\agent` | the files below |
| macOS: `~/Library/Application Support/Bridge` | the files below |

- **A receipt log** (`agent-egress.jsonl`): a record for each tool call, with
  the tool name, the time, the company's Tally GUID, fingerprints of the
  request and the response, the names (not the values) of the fields returned,
  how many rows and bytes, whether the response was cut short, the redaction
  setting in force, an error code if there was one, and for each request the
  call sent to Tally its kind, size, outcome and time. It holds no row values.
- **A terms record** (`terms-acceptance.jsonl`): the extension asks you to
  accept the ComplyEaze Bridge Terms of Use (version 2026-10) in its settings,
  and every tool refuses with `terms_not_accepted` until you do. When the
  server starts with the setting on, it appends a line with the terms version,
  the time, and that the acceptance came through the setting (once per
  version; two servers starting together can each add a line). If that line
  cannot be written, every tool refuses with `terms_record_unavailable`. The
  line is a local record that the setting was on, not proof of who accepted or
  of when the box was ticked; it is not sent anywhere.
- **An import journal** (`agent-import-ledger.jsonl`) and saved batch files
  (`imports/`): the vouchers ComplyEaze Bridge prepared or posted, with their
  dates, narrations, amounts and ledgers, the proof-of-post files that hold
  what Tally read back, and a short note when an approval you gave ran out
  before it was used.
- **Bank-statement proposals** (`bank-statements/`): every row of a statement
  you asked it to read, with date, amount, bank reference and narration.
- **Lock files** with no content, a lock folder that keeps two copies of the
  program from sending to Tally at the same time, and a small record written
  when you record a review of a posted voucher, including your
  operating-system account name.

None of these files is encrypted. On macOS the folder is private to your user
(mode 0700) and files are 0600. On Windows the folder inherits its parent's
permissions. **Not measured:** the resulting Windows permissions.

You can ask what is stored. The `local_data_report` tool (and
`bridge_mcp --local-data-report` on the command line) counts the files, their
size and the age of the oldest by kind, says how many saved batches are not
settled, and says so when it could not read something. It changes and deletes
nothing, names no file path unless you ask on the command line, and covers
this folder only, not files kept by the desktop app.

There is no size limit, expiry or deletion command: the files grow until you
delete them. You can delete them by hand when Claude Desktop is not running. Do
not delete the import journal (`agent-import-ledger.jsonl`) or `imports/` while
any batch in them has been posted or is waiting to be verified: they are the
record ComplyEaze Bridge uses to reconcile a post, and without them
`verify_import` cannot find the batch (see the
[installation guide](./agent/INSTALL.md)). **Not measured:** what else breaks if
only some files are deleted.

## 5. Is it signed, and how do I check what I downloaded?

It is not yet code-signed or notarized. Each package on GitHub Releases has a
SHA-256 file and a provenance record naming the source commit it was built
from. From release 0.4.0 the release workflow also records a build attestation
for each package and checks it before publishing; an attestation says where a
file was built, and is not a code signature. The package includes an unsigned
third-party PDF library (PDFium, from a pinned release, checked by SHA-256 at
build time), used only to read the bank statement PDF you name. **Not
measured:** whether PDFium itself opens any network connection or file other
than the named PDF.

## 6. Has it been independently reviewed?

**No independent security audit has been done.** The project runs its own checks
in CI, and everything is open source, but that is not an external review.

## 7. Does it need administrator rights, or install anything else?

The package declares only a command to run and its settings. In its source we
found no service, driver, scheduled task, registry key, launch agent or
listening port. It writes files only in its data folder and in the lock folder
named in section 4. It also reads files named in a tool call (often by the
assistant), such as a bank statement and its password file; on macOS it
refuses a password file that other users can read. Its one extra process is a
second copy of itself that shows the approval window.

The approval window is a system dialog. On macOS it is titled "ComplyEaze
Bridge — approve one voucher" with the buttons Cancel and Post voucher. On
Windows it is titled "ComplyEaze Bridge — post this voucher?" with Yes, No and
Cancel, and No is the default. Only the positive button approves. The window
closes by itself after two minutes without an answer, and an approval that is
not used within fifteen minutes, or before the program restarts, is dropped.

**Not measured:** what Claude Desktop itself needs when installing an extension,
and a check of a machine before and after installing and running it.

## 8. How do I turn it off, or remove it?

- **Turn posting off:** switch "Allow voucher posting" off in the extension
  settings (off is the default for a new install; an earlier version may have
  saved it on), then quit Claude Desktop completely and reopen it.
  `post_import` and `acknowledge_post_review` are then absent from the tool
  list and refuse if called. Preparing voucher files and reading bank
  statements stay available, and still write to the folder in section 4.
  **Not measured:** whether Claude Desktop applies the change without a
  restart.
- **Remove it:** uninstall it from Claude Desktop's Extensions settings. This
  does not change Tally's gateway setting. **Not measured:** what the uninstall
  removes. ComplyEaze Bridge has no uninstall step of its own and nothing in it
  deletes the files in section 4; delete them yourself.
- **Vouchers already posted stay in Tally.** ComplyEaze Bridge has no tool to delete or
  undo one; correct it in Tally.
