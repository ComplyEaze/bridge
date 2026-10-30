# 690-R1: an Alter whose GUID and name point at different ledgers, and what a refused Alter leaves (Part of #690)

Sheet `lab/690-ledger-rename-r1` at 523df687. BRIDGE LEDGERBOX CI LAB (GUID 5848d0ca-ed85-4c97-ab20-e045893a9803), synthetic, loaded alone, TallyPrime 7.2 Silver. Raw requests by curl (LAB-COMMON C3). BK0 23,613 B, sha256 866a052b44abf8584405ececa26869f740a68160ad50fee2b6e858fe5e4e3194, restored at Z1 and Z3; at Z2 and Z4 the book read exactly as at S1. 30 Oct 2026, 07:44 to 07:58 UTC. Confidence of every answer: one sitting, one synthetic book: partial.

| question | result |
|---|---|
| Does a `GUID` element in an `Alter` leave a matched rename working? (M1, M2) | **Yes.** `ALTERED 1`; X, by its GUID, is renamed `R1 Guard X2`; Y is unchanged |
| Does Tally refuse an `Alter` whose `GUID` names one ledger and whose `NAME` names another, existing one? (A1, A2) | **No: outcome (ii).** `ALTERED 1`, no `LINEERROR`; **Y**, the named ledger, is renamed `R1 Guard Y2` and keeps its own GUID; X, whose GUID was carried, is unchanged; no master added; `ALTMSTID` 211 to 212. The GUID is ignored and the name decides |
| Does a restart alone, with no write, mark the company with data exceptions? (B0) | **No.** After a quit and a restart with no write since the restore, the Gateway showed no "(e)" and no "Data exceptions exist" |
| Does a refused `Alter` to a missing name leave the company marked? (B1, B2, B4, B5) | **Yes, after a restart.** B1 answered `EXCEPTIONS 1` with `LINEERROR` "Group Name not given!", and nothing moved (B2). Before the restart the Gateway showed no mark (B2). After the restart (B4) the company's Gateway line showed "(e)" and the Gateway read "(e) Data exceptions exist"; Select Company's line showed no mark. Tally's exception reports (B5): "Verification of Chart of Accounts" (All Exceptions) lists "Not Used (2 Ledgers, Not Verified 2)"; "Verification of Stock Items" has no entries. Nothing in them names the refused Alter. The BK0 restore removed the mark (Z4) |

What the B5 lists show:
- The Go To list for "Exception" offered Payroll's Passport, Visa and Contract Exceptions, "Exception Reports" (a list of Overdue Receivables, Overdue Payables and Post-Dated Summary), Chart of Accounts Exceptions and Stock Item Exceptions.
- No report entry was opened.
- The Payroll reports were not opened: payroll is off in this book, and they report employee documents.

For #690 slice 2, by the sheet's table: outcome (ii) means a GUID cannot guard an `Alter` in-band. Closing the window needs the owner's acceptance in ADR 0004's words. Separately, a refused rename to a missing name leaves the company marked "Data exceptions exist" after the next restart, with nothing in Tally's exception reports naming it.

Tally was quit and restarted twice (B0, B3), each by one Escape at the Gateway and Yes at "Quit ?". It was opened again from its program path, the demo company it loaded by itself was shut, and the company was loaded alone.

Not covered: Gold, other releases, other master types, an `Alter` carrying any field other than a name list and a `GUID`.
