# How to choose a TallyPrime MCP server safely

An MCP server lets an AI assistant such as Claude read the books in TallyPrime,
and sometimes change them. Before you connect one to a client's books, put
these questions to it, whichever one it is. Each says how to check the answer
yourself. This page does not rank products.

## The questions

1. **Does the maker see your client's books?** With a hosted assistant such as
   Claude, what the assistant reads goes to your AI provider as part of the
   conversation, whatever server you use.
   The question is whether the server sends anything anywhere else as well.
   Check its privacy policy and, if the source is open, what it connects to.
2. **What does your AI provider see, and can you hide any of it?** Check
   whether names or narrations can be masked, and whether amounts can be.
3. **Do you need your client's permission?** That depends on your engagement
   and the law that applies to you, not on the tool. Check what the tool's
   terms say about your responsibility.
4. **Can it change your books, and is that off by default?** Read the list of
   tools it gives the assistant. A tool that creates, alters or deletes
   vouchers or masters is a write. Check whether writing is off until you turn
   it on.
5. **Who approves a write, and what if it is wrong?** An approval the
   assistant can give on its own is not an approval. Check whether each write
   waits for a person, in a window outside the chat, and whether a posted
   entry can be undone or must be corrected in TallyPrime by hand.
6. **Will it work with your TallyPrime and your computer?** Check the
   operating systems, the TallyPrime editions it has been run on, and whether
   it reaches TallyPrime only on your own computer or can be pointed at
   another machine. Never forward TallyPrime's port across the internet.
7. **What does it cost?** Check the price, the licence of the code, and what
   else you pay for (TallyPrime, your AI plan).
8. **What has been tested?** Check which release was run, against which
   TallyPrime, on real or made-up books, and whether the record is published.
9. **What does it leave on your computer, and how do you remove it?** Logs and
   files it writes can hold names and amounts. Check where they are and how to
   remove them safely.
10. **Has anyone independently checked it, and who answers if it goes wrong?**
    Check for an outside review, a security policy with a way to report a
    problem, and what the terms say about liability.

## Tally's own plug-in, and what ComplyEaze Bridge adds

Tally Solutions offers its own TallyPrime MCP plug-in for Claude Desktop. Its
help pages (last updated 18 September 2026, read on 4 October 2026) list 28
tools, all for reading your books, and a setup that has you refresh your
licence in TallyPrime with your Tally.NET ID, installs Node.js and has you edit
Claude Desktop's configuration file. Put the ten questions to it as well;
Tally's pages are the place to check what it needs. ComplyEaze
Bridge is not a product of Tally Solutions, we have not run the two side by
side, and this page does not rank them. If you only want to ask questions of
your books, check whether the plug-in covers what you need first.

What ComplyEaze Bridge adds, so you can judge whether you need it:

- **A PDF bank statement step.** It reads a password-protected PDF statement
  from State Bank of India, HDFC Bank or Union Bank of India into Payment,
  Receipt and Contra vouchers as a file you import in TallyPrime yourself. It
  checks your ledger names against the book first, and after you import it can
  read the batch back from TallyPrime. TallyPrime's own bank statement import
  takes statement files such as CSV or Excel from the bank's portal, not PDF (Tally's help page
  "Bank statement", read on 4 October 2026), so if your client already sends
  one of those, TallyPrime's own import may be all you need. The whole path,
  from the PDF to the batch read back, has not yet been recorded end to end
  against TallyPrime.
- **Its own install.** One extension file in Claude Desktop, with no Node.js
  and no configuration file to edit. It connects to TallyPrime through
  TallyPrime's own HTTP gateway on your computer, so it does not need a Tally
  plug-in. Most recorded runs used licensed TallyPrime Silver
  and Gold 7.1; see the
  [README](../../README.md#what-has-been-run-against-a-real-tallyprime).

Where it is behind: three banks, where TallyPrime's own import lists many more
(its help page "Bank statement"); no reconciling of a bank statement inside
TallyPrime; no stock quantities (the plug-in has an inventory group); not yet
code-signed; and a short list of runs against a real TallyPrime. The published
latest release has not yet been installed in Claude Desktop by us; the README
lists each run.

## How ComplyEaze Bridge answers them

ComplyEaze Bridge is our TallyPrime MCP server for Claude Desktop. In short:

- ComplyEaze does not receive your TallyPrime data through ComplyEaze Bridge;
  what Claude reads goes to your AI provider. Response redaction can mask party
  names or drop narration; nothing hides amounts.
- Posting is off by default for a new install; an earlier version may have
  saved it on, so check the setting. When it is on, each voucher waits for your
  approval in a separate ComplyEaze Bridge window. No ComplyEaze Bridge tool can
  approve it for you, but software that controls your screen could click the
  window, so do not let it. It cannot undo a posted voucher; you correct it in
  TallyPrime.
- It connects only to a loopback address on your own computer. If you forward
  that port to a virtual machine or another machine, the traffic follows your
  forward, and ComplyEaze Bridge cannot tell.
- ComplyEaze has not set a price and does not sell licences to it; the code of
  current releases is under Apache-2.0.
- What has been run against a real TallyPrime, and what has not, is listed in
  the [README](../../README.md#what-has-been-run-against-a-real-tallyprime).
  No independent security audit has been done, and it is not yet code-signed.

The details: [README](../../README.md) ·
[Security and privacy](../security-and-privacy.md) ·
[Terms of Use](../legal/terms.md) ·
[Security policy](../../SECURITY.md) ·
[How to connect Claude to TallyPrime](./connect-claude-to-tallyprime.md).
