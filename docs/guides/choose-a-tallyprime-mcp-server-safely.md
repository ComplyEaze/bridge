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

## Tally's own tools, next to ComplyEaze Bridge

Tally's own tools as described on its help site (the dates are Tally's own
"last updated" dates), beside ComplyEaze Bridge's. ComplyEaze Bridge is not made
by, or affiliated with, Tally Solutions, we have not run the two side by side,
and this page does not rank them.

| Job | Tally Solutions | ComplyEaze Bridge |
| --- | --- | --- |
| Ask Claude about a client's books | TallyPrime MCP plug-in for Claude Desktop: tools in six groups (company, master, accounting, inventory, outstanding and transaction), as described on [Tally's tools page](https://help.tallysolutions.com/tallyprime-mcp-tools/) (last updated 18 September 2026) | Read tools for companies, the trial balance, outstandings with ageing, vouchers, ledger movement, the purchase and sales registers and stock values; it changes nothing in TallyPrime unless you turn posting on |
| Bank statements | TallyPrime's Bank Statement import, in the formats Tally lists for each bank ([Tally's list of supported banks](https://help.tallysolutions.com/list-of-banks-supported-by-tallyprime-for-e-payments-auto-brs-and-cheque-formats/)) | Reads a client's statement in a supported layout (the README lists them) into Payment, Receipt and Contra vouchers as a file you import yourself, and can read the batch back; the whole path has not yet been recorded end to end against TallyPrime |

Where ComplyEaze Bridge is behind: three banks' statement layouts; no stock quantities; not yet
code-signed; and a short list of runs against a real TallyPrime (the README
lists each run).

## How ComplyEaze Bridge answers them

ComplyEaze Bridge is our TallyPrime MCP server for Claude Desktop. In short:

- ComplyEaze does not receive your TallyPrime data through ComplyEaze Bridge;
  what Claude reads goes to your AI provider. Response redaction can shorten
  party and ledger names and bank account numbers or drop narrations; it does
  not hide amounts, company names, PAN, GSTIN, IFSC, MSME or Udyam registration
  numbers, contact details or references.
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
