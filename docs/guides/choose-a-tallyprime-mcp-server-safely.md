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

## How ComplyEaze Bridge answers them

ComplyEaze Bridge is our TallyPrime MCP server for Claude Desktop. In short:

- ComplyEaze does not receive your TallyPrime data through ComplyEaze Bridge;
  what Claude reads goes to your AI provider. Response redaction can mask party
  names or drop narration; nothing hides amounts.
- Posting is off by default. When it is on, each voucher waits for your
  approval in a separate ComplyEaze Bridge window, and no ComplyEaze Bridge
  tool lets the assistant approve it. It cannot undo a posted voucher; you
  correct it in TallyPrime.
- It connects only to a loopback address on your own computer, and it cannot
  tell whether that port is forwarded elsewhere.
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
