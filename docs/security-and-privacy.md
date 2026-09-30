# Security and privacy

This page answers, in one place, what a CA or a firm's IT person asks before
installing ComplyEaze Bridge next to client books. It describes the latest
published release (version 0.3.0). It was written from a reading of that
release's source code on 30 September 2026. It was not tested on a running
system. Unless a line says otherwise, each answer rests on reading that source.
Anything not measured on a running system is marked **Not measured**.

To report a vulnerability, see [SECURITY.md](../SECURITY.md).

## 1. What does it read?

It reads from the TallyPrime running on your own computer, over Tally's own
local gateway: the loaded companies, ledger masters, the trial balance,
vouchers in a date window, outstanding receivables and payables with ageing,
and ledger movement. It also checks ledger names against the book, and can read
a password-protected bank statement PDF you name (SBI, HDFC or Union Bank of
India) to propose vouchers. The full tool list is in the
[MCP guide](./agent/README.md).

It writes to Tally only if you turn posting on, and then only one voucher at a
time, after you approve that voucher in a separate ComplyEaze Bridge window. No
ComplyEaze Bridge tool can approve it.

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
- **To ComplyEaze: nothing we found in the published extension's code.** We
  found no analytics, telemetry, crash reporting or automatic update check in
  it. The extension's tools reach only the Tally transport. The extension is
  built from the same source library as the desktop app, which still contains a
  document-upload feature and an AXAL sign-in. No published build exposes them,
  and no tool of the extension reaches them (see section 3). They are being
  removed ([#914](https://github.com/lamemustafa/bridge/pull/914)).

## 3. Which network destinations can it contact?

- The only network client the extension's tools can reach is the Tally
  transport. It accepts only a loopback address (any `127.x.x.x` address, or
  `::1`) or the name `localhost`, which it maps to `127.0.0.1` without a DNS
  lookup. It refuses anything else, uses no proxy and follows no redirects.
- The extension is built from the same source library as the desktop app. That
  library also contains two HTTPS clients for the desktop app, an AXAL sign-in
  and a document upload, whose default destination is `complyeaze.com`. No tool
  of the extension calls them. That rests on reading how the code is wired; no
  CI check enforces it.
- The [egress check](../scripts/check-tally-egress-boundary.mjs) in CI limits
  which source files may create an HTTP client. Its header lists what it does
  not prove, including network use by native libraries such as PDFium and
  whether the loopback check is still wired up. The changelog adds that it does
  not by itself prove that no data leaves the machine.

**Not measured:** a network capture of the running extension on Windows or Mac.

## 4. What is stored on my computer, and for how long?

| Where | What |
| --- | --- |
| Windows: `%LOCALAPPDATA%\Bridge\agent` | the files below |
| macOS: `~/Library/Application Support/Bridge` | the files below |

- **A receipt log** (`agent-egress.jsonl`): records for each tool call, with
  the tool name, the time, the company's Tally GUID, fingerprints of the
  request and the response, the names (not the values) of the fields returned,
  and its size. It holds no row values.
- **An import journal** (`agent-import-ledger.jsonl`) and saved batch files
  (`imports/`): the vouchers ComplyEaze Bridge prepared or posted, with their dates,
  narrations, amounts and ledgers, and the proof-of-post files that hold what
  Tally read back.
- **Bank-statement proposals** (`bank-statements/`): every row of a statement
  you asked it to read, with date, amount, bank reference and narration.
- **Lock files** with no content, and a small record written when you record a
  review of a posted voucher, including your operating-system account name.

None of these files is encrypted. On macOS the folder is private to your user
(mode 0700) and files are 0600. On Windows the folder inherits its parent's
permissions. **Not measured:** the resulting Windows permissions.

There is no size limit, expiry or deletion command: the files grow until you
delete them. You can delete them by hand when Claude Desktop is not running. Do
not delete the import journal (`agent-import-ledger.jsonl`) or `imports/` while
any batch in them has been posted or is waiting to be verified: they are the
record ComplyEaze Bridge uses to reconcile a post, and without them
`verify_import` cannot find the batch (see the
[installation guide](./agent/INSTALL.md)). **Not measured:** what else breaks if
only some files are deleted.

## 5. Is it signed, and how do I check what I downloaded?

It is not yet code-signed or notarized. Each
package on GitHub Releases has a SHA-256 file and a provenance record naming the
source commit it was built from. The package includes an unsigned third-party
PDF library (PDFium, from a pinned release, checked by SHA-256 at build time),
used only to read the bank statement PDF you name. **Not measured:** whether
PDFium itself opens any network connection or file other than the named PDF.

## 6. Has it been independently reviewed?

**No independent security audit has been done.** The project runs its own checks
in CI, and everything is open source, but that is not an external review.

## 7. Does it need administrator rights, or install anything else?

The package declares only a command to run and its settings. In its source we
found no service, driver, scheduled task, registry key, launch agent or
listening port. It writes files only in its data folder. It also reads files
named in a tool call (often by the assistant), such as a bank statement and its
password file. Its one extra process is a second copy of itself that shows the
approval window.

**Not measured:** what Claude Desktop itself needs when installing an extension,
and a check of a machine before and after installing and running it.

## 8. How do I turn it off, or remove it?

- **Turn posting off:** switch "Allow voucher posting" off in the extension
  settings (off is the default for a new install; an earlier version may have
  saved it on), then restart Claude Desktop. `post_import` and
  `acknowledge_post_review` are then absent from the tool list and refuse if
  called. Preparing voucher files and reading bank statements stay available,
  and still write to the folder in section 4. **Not measured:** whether Claude
  Desktop applies the change without a restart.
- **Remove it:** uninstall it from Claude Desktop's Extensions settings. This
  does not change Tally's gateway setting. **Not measured:** what the uninstall
  removes. ComplyEaze Bridge has no uninstall step of its own and nothing in it
  deletes the files in section 4; delete them yourself.
- **Vouchers already posted stay in Tally.** ComplyEaze Bridge has no tool to delete or
  undo one; correct it in Tally.
