# ComplyEaze Bridge

<!-- llms:begin -->
ComplyEaze Bridge is an MCP server that connects Claude Desktop to the
TallyPrime running on your own computer. It is built for CA firms and
accountants. You can ask about outstanding receivables and payables with
ageing, the trial balance, ledger movement and vouchers. It checks ledger
names before you post, and it prepares Journal, Payment, Receipt and Contra
vouchers as a file. If you turn posting on in the extension, it posts them one
at a time, after you approve each one.

**Current release:**
<!-- managed:current-release -->[`mcp-preview-0.3.0`](https://github.com/ComplyEaze/bridge/releases/latest) (26 September 2026)<!-- /managed:current-release -->,
for Windows x64 and Apple Silicon Macs. We check each release before we
publish it: the release check confirms that each package launches, lists its
tools and parses a synthetic encrypted bank statement. It does not run against
TallyPrime, and nothing we can run covers every
Tally edition, set of books or setting. What has been run against a real
TallyPrime, and what has not, is listed below.
[Install it](./docs/agent/INSTALL.md) · [What changed](./CHANGELOG.md) ·
[Security and privacy](./docs/security-and-privacy.md)

Not yet code-signed; your computer may warn you before opening it.

**Try asking** (on a test company first):

- "List the loaded companies."
- "Show the outstanding receivables and payables, with ageing, as of 31 March."
- "Show the trial balance for 1 April to 31 March."
- "Check these ledger names against the book before I post: …"

## How it handles your books

- **Local connection only.** It talks to Tally's own XML gateway on
  a loopback address, so it cannot be pointed at a remote Tally host. It cannot
  tell whether that local port is forwarded to another machine; do not forward
  one across the internet. The Tally connection sends nothing to a ComplyEaze
  server.
- **Your AI provider sees what the assistant reads**, just as it sees the
  rest of the conversation: company names, party names and amounts. You can
  mask party names or drop narration. Amounts are always sent. See
  *Before you use it with client data* below.
- **Posting is off by default in the extension.** If you installed an
  earlier version, check the setting: an earlier default may still be saved as
  on. When you turn posting on, each voucher waits for your approval in a
  separate ComplyEaze Bridge window. No Bridge tool lets the assistant approve
  it, and an approval counts only when that window returns a fresh one-time
  token. After posting, the voucher is read back from Tally so you can see what
  landed. Three known limits remain: ComplyEaze Bridge cannot undo a posted
  voucher (you correct it in Tally); a company renamed to, or loaded under, the
  target company's name (or one differing only in case or spacing) just after
  its last check could still receive the voucher, if it has the voucher's
  ledgers, and ComplyEaze Bridge cannot always say where it went or prevent it;
  and a ledger renamed and replaced in that same moment could receive the
  entry, and not every such change is noticed. Read *Before you turn on
  posting* first.
- **Tool calls leave receipts** in a log on your computer: the company's Tally
  identifier, and a fingerprint of what was asked and of what came back,
  written whether the call succeeds or is refused.
- **You accept the Terms of Use first.** The extension asks you to accept the
  ComplyEaze Bridge Terms of Use (version 2026-10) in its settings, and every
  tool refuses with `terms_not_accepted` until you do.
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
- The published 0.3.0 package on Windows x64, in Claude Desktop (on a free plan;
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
  tools loaded in a chat. The record is the maintainer's dated notes and
  screenshots, kept privately. This was one run, not a controlled test of each
  key of the window.

Not yet run by us in a controlled test: posting with a published package
against a live TallyPrime; each way of declining in the Windows approval window
(one was tried); the tools answering through Claude Desktop on macOS after the
Terms are accepted; posting on TallyPrime Education; posting on TallyPrime Gold
with its approval step recorded. Each release package is built and launched,
its tool list checked and a synthetic encrypted bank statement parsed, on
hosted CI runners for Windows x64 and Apple Silicon Mac.

## Not in the latest release

- Profit and Loss and Balance Sheet (in source, not yet released)
- Stock or inventory reads; sales, purchase or tax posting; creating masters;
  bill-wise allocation
- Deleting or undoing a posted voucher (correct it in Tally)
- Reads on very large books can fail or take longer than the assistant waits
  (#485, #703)
- Books with more than one currency, or a base currency other than INR
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

- **The Claude Desktop extension turns voucher posting off by default.** Three
  known limits in posting remain. Tally aims an import at a company by its name and cannot bind it to a company's GUID. Bridge's last request before the post checks that exactly one loaded company has the target's GUID and name, and that no other loaded company has the same name ignoring case and spacing; otherwise it refuses the post ([#607](https://github.com/ComplyEaze/bridge/pull/607)). A company renamed to, or loaded under, the target's name (or one differing only in case or spacing) in the moment after that check could still receive the voucher, if it has the voucher's ledgers. Bridge may flag afterwards that the loaded companies changed, but cannot always say where the voucher went, and cannot prevent it (accepted residual, [#574](https://github.com/ComplyEaze/bridge/issues/574)). A ledger renamed and replaced in that same moment means the post can land in the replacement ledger. Bridge marks the result as needing reconciliation when it sees that the ledger now resolves to a different master; a change that leaves the company's master mark unmoved, or is reverted before that check, is not seen, and a regroup in that moment is not detected ([#623](https://github.com/ComplyEaze/bridge/pull/623)). And Bridge has no tool to delete or undo a voucher it has posted, so a wrong post must be corrected by hand in Tally. It records the REMOTEID each post sends, but no delete tool exists yet ([#579](https://github.com/ComplyEaze/bridge/issues/579), [#582](https://github.com/ComplyEaze/bridge/pull/582)). Turning on
  **Allow voucher posting (Journal, Payment, Receipt, Contra)** in the extension
  settings adds `post_import`, which posts one saved voucher of those types; every
  posting still waits for your approval in a separate Bridge dialog. Leave it
  off unless you accept those risks. Voucher file preparation and bank-statement
  parsing, which write nothing to Tally, stay available with the setting off.
  If you installed an earlier version, check the setting: an earlier default
  may still be saved as on.
- **A source build turns writing off by default.** Preparing a file needs
  `BRIDGE_AGENT_ENABLE_IMPORT`; posting additionally needs
  `BRIDGE_AGENT_ENABLE_WRITES`, which grants both.
- **A source build also needs `BRIDGE_TERMS_ACCEPTED=true`.** The extension asks
  for that as its "I accept" setting; without it every tool refuses.

With writing on:

- **Prepares vouchers as a local file** — Journal, Payment, Receipt and
  Contra. Bridge writes the file; it does not send it.
- **Posts one saved Journal, Payment, Receipt or Contra per approval**, and
  only after you approve that exact voucher in a dialog on your own machine.
  The assistant cannot approve it. Bridge then reads the voucher back so you
  can see what actually landed. You can also import a prepared file through
  Tally yourself; `verify_import` then reads that back.
- **Posting has limits.** It creates no masters, posts no sales, purchase, tax
  or inventory entries, and never alters or deletes a voucher. A company with
  more than one currency defined is refused.

**What it does not do**

- **The Tally path uploads nothing to ComplyEaze.** Bridge reads it over a local
  connection and hands it to the assistant you are talking to; nothing in the
  Tally path sends it to a server of ours.
- It will not post anything without a separate, explicit step after the file is
  prepared.
- It is not a Tally replacement, a reporting suite, or a filing tool.

## The desktop app

**The extension is built from the same source library as the desktop app.**
The published 0.3.0 package still contains an unfinished document-upload
feature and an AXAL sign-in. In that package no tool of the extension reaches
them. The source after 0.3.0 no longer has them
([#914](https://github.com/ComplyEaze/bridge/pull/914)), so the next release
will not contain them; in that source the only network client in ComplyEaze
Bridge's own code connects to Tally on your own computer. No desktop installer
is published. See [Security and privacy](./docs/security-and-privacy.md).

## Before you use it with client data

**One thing to understand before you use it.** When you ask an AI assistant for
financial data through Bridge, the assistant's provider sees what it reads —
company names, party names and amounts. That is a property of using a hosted
assistant, not of Bridge. Bridge can mask party names or drop narration first
(`BRIDGE_AGENT_REDACTION`), but **neither setting removes amounts** — figures
always go with the answer. Decide this deliberately for client data.

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
  your Tally, or in conversation inside Claude Desktop. The owner reports one
  read-only session of the published 0.3.0 package in Claude Desktop on Windows
  x64 against a live licensed TallyPrime Gold 7.1, with no logs kept; posting
  and the approval window have not been run on Windows.
- **Windows x64 and Apple Silicon Macs only.** Intel Macs are not supported.
- **On a Mac, Tally must run on that same Mac**, in a local Windows virtual
  machine or through approved local forwarding. Bridge only talks to Tally on
  your own computer, so a separate PC or a Tally elsewhere on your network
  cannot be reached by typing its address.
- **It does not update itself.** To upgrade, install a newer release from
  Claude Desktop's Extensions settings.

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

Before requesting financial data through an MCP client, the client may send the selected
Tally result to its AI provider, including company
identity, party or open-bill details, and amounts. Source installations default
to `BRIDGE_AGENT_REDACTION=none`; set it to `mask_parties` or `drop_narration`
before launch when that better fits the workflow. These settings mask party
names or drop narration; they do not remove amounts. The package installation
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
The historical `v0.1.0` release remains under the MIT license shipped with
that tag; current development source is version `0.3.0` under Apache-2.0.
