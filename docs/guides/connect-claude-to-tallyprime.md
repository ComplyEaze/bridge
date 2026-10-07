# How to connect Claude to TallyPrime

ComplyEaze Bridge is a TallyPrime MCP server for Claude Desktop. It runs on
your own computer and lets Claude read the TallyPrime books open there. Posting
vouchers is off by default. This page is the short path; the
[installation guide](../agent/INSTALL.md) has every setting and its limits.

## What you need

- **Claude Desktop on Windows x64 or an Apple Silicon Mac.** Intel Macs are not
  supported.
- **TallyPrime on the same computer, with its HTTP gateway turned on.** On a
  Mac, TallyPrime must run on that same Mac, in a local Windows virtual machine
  or through approved local forwarding. ComplyEaze Bridge talks only to
  TallyPrime on your own computer, so a separate PC or a TallyPrime elsewhere
  on your network cannot be reached by typing its address.
- **A test company and current backups for your first run.** ComplyEaze Bridge
  is still being developed, so a release may contain errors.

## Steps

1. **Turn on TallyPrime's HTTP gateway.** In TallyPrime's own connectivity
   settings, set it to act as a server ("acts as Both") and note its HTTP
   gateway port, `9000` by default. Open `http://localhost:9000/status`
   (with your port) in a browser: a running gateway answers with a short XML
   response. On a Mac with TallyPrime in a Windows virtual machine, run this
   check inside the virtual machine.
2. **Download the extension.** On the
   [GitHub Releases](https://github.com/ComplyEaze/bridge/releases) page, take
   the `.mcpb` file for your computer from the newest release (normally marked
   Latest; its tag starts with `mcp-`). It is not
   yet code-signed or notarized, so your computer may warn you before opening
   it. Each file has a same-named `.sha256` file to check the download against.
3. **Remove an older ComplyEaze Bridge first, if you have one.** Release 0.4.2
   installs beside an older release instead of replacing it (seen on a Mac;
   not tried on Windows). In Claude Desktop, open Settings, Extensions and
   remove the older one. Do not delete its data folder, which both versions
   use.
4. **Install it.** In Claude Desktop, use **Settings → Extensions → Advanced
   settings → Install Extension…** and choose the `.mcpb` file.
5. **Fill in its settings.** Keep **Tally host** as `localhost`. Set **Tally
   port** to the gateway port from step 1. Read the Terms of Use linked in the
   settings, then turn on **I accept the ComplyEaze Bridge Terms of Use (version
   2026-10.1)**:
   until you do, every tool call is refused and nothing is read from
   TallyPrime.
6. **Restart Claude Desktop.** Save the settings, quit Claude Desktop
   completely and reopen it. In a new chat, use **Connectors** to confirm
   ComplyEaze Bridge is connected.
7. **Ask a first question, on a test company.** For example: "List the loaded
   companies." Then: "Show the trial balance for 1 April to 31 March."

## Before you use it with client data

- **What Claude reads goes to your AI provider**, as part of the conversation:
  company, party and ledger names, amounts, dates, narrations, references,
  GSTINs and, when it reads ledger details, PAN, bank account numbers, IFSC,
  email, phone and address. The **Response redaction** setting takes one value,
  not both: shorten party and ledger names and bank account numbers to their
  first two and last two characters (`mask_parties`; four or fewer become “…”),
  or drop narrations (`drop_narration`). Neither hides amounts, company names,
  PAN, GSTIN, IFSC, contact details or references, and a shortened name can
  still be identified, not least from the GSTIN or PAN sent beside it. It starts
  at `none`.
- **Posting is off by default** for a new install; an earlier version may
  have saved it on, so check the setting. If you turn it on, each voucher waits
  for your approval in a separate ComplyEaze Bridge window. No ComplyEaze
  Bridge tool can approve it for you, but software that controls your screen
  could click the window, so do not let it. ComplyEaze Bridge cannot
  undo a posted voucher; you correct it in TallyPrime. Read *Before you turn on
  posting* in the [README](../../README.md#before-you-turn-on-posting) first.
- **Every tool call leaves a receipt** in a log on your computer.

More: [Security and privacy](../security-and-privacy.md) ·
[How to choose a TallyPrime MCP server safely](./choose-a-tallyprime-mcp-server-safely.md).
