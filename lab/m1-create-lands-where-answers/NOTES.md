# Where does a ledger Create land when the company it names is not the active one? (#690, Windows lab, 13 Oct 2026)

Synthetic data only: two companies made for this item on Tally's own screen, BRIDGE M1 LAB A and
BRIDGE M1 LAB B (Rajasthan, books from 1 Apr 2026, no GST, no inventory, no password; their GUIDs are
in every marks answer). TallyPrime 7.1, Silver Rental, on Windows. One run. No ComplyEaze Bridge build:
five raw imports of one ledger each (`Import Data`, `All Masters`, `SVCURRENTCOMPANY` naming
`BRIDGE M1 LAB A` in all five), each file sent once; raw reads by curl, one at a time. Each company was
backed up before the first request and again at the end; nothing was restored. Times in send-log.txt
are UTC.

## Method

- Before each import: a screenshot showing which company the Gateway held as active, then a marks read
  that had to list exactly the companies the step names, by GUID. After it: a screenshot of the whole
  window, marks, and the ledger list of every loaded company, compared by GUID with the list before
  (`tables/`). "A" is one company throughout (one GUID), also while it was named BRIDGE M1 LAB A2.
- Loading, shutting and renaming were done on Tally's screen (Select, Shut and Alter Company).

## Result

marks = ALTMSTID of A / of B.

| step | loaded; active | answer to the Create | ledger lists after | marks |
|---|---|---|---|---|
| 3 control | A, B; A | CREATED 1, other counters 0 | `ZZ M1 PROBE 00` new in A; B unchanged | 208 to 209 / 208 |
| 4 | A, B; B | CREATED 1, other counters 0 | `ZZ M1 PROBE 01` new in A; B unchanged | 209 to 210 / 208 |
| 6 | B only (A shut) | one LINEERROR, no counters | B unchanged; A unchanged once loaded again | 210 / 208 |
| 7 | A only, renamed A2 | the same | A unchanged | 211 / not loaded |
| 8 | A (as A2), B; B | the same | A and B unchanged | 211 / 208 |

- The LINEERROR in steps 6, 7 and 8 read `Could not set 'SVCurrentCompany' to 'BRIDGE M1 LAB A'`. Each
  of those answers was a RESPONSE of 248 bytes holding that line and none of CREATED, ALTERED and the
  other counters, LASTVCHID or LASTMID; HTTP status 200.
- No answer read ALTERED above 0. No ledger other than the two probes appeared or changed in either
  company. No request raised anything on Tally's screen.
- A ledger-list READ naming a company that was not loaded (step 6), or the old name of the renamed
  company (step 7), answered HTTP 200 with STATUS 1 and an empty COLLECTION: no rows, no error line, not
  the loaded company's rows. The two answers are byte-identical.
- Shutting A and loading it again moved neither mark (step 5). Each rename moved A's ALTMSTID by one
  (210 to 211; back, 211 to 212); A's GUID and ledger rows did not change, and Tally itself made Mailing
  Name follow the name. The four backups moved no mark.

## One stop, and what was done by hand

- The run stopped at step 2, before any write: the first marks answer was 3,674 bytes and the run
  sheet's bound for a marks answer was 2,048. The answer was kept unread. The bound was then corrected
  to 8,192 bytes for every marks read of the item (a one-company marks answer on this Tally is about
  3,300 bytes); the kept answer was checked for exactly two COMPANY rows and used, and the run went on
  after a fresh check of the process, the port and the screen. The other bounds held.
- Two keying slips, neither of which reached a book: the first typing of A's name on the creation screen
  lost its spaces and was retyped before the screen was accepted; at step 6 a click on the already
  highlighted row of the Shut Company list shut A at once, where the shut before had taken a click and
  Enter.
- One helper loop checked nothing (a variable name used twice); the check of the ten request files was
  redone. It sent nothing.
- One marks read more than the sheet lists was sent after the last two backups.
- Prompts answered: "Accept ?" on each company creation, on its features screen and on each backup. Each
  rename was accepted with the screen's own accept and raised no prompt.

## What it decides

On this release, for a ledger Create in this envelope and two new companies: the Create landed in the
company the request named whenever that company was loaded, whichever was active. When the named company
was not loaded, or carried another name by then, nothing was created anywhere and the answer said so
with an error line and no counters. It never landed in the active company instead.

## Not covered

Gold; other releases; Tally security on; a company named in another letter case or with trailing
spaces; a Create over an existing name; a voucher import; another transport encoding; a second run.

## Files

`requests/` (10), `answers/` (every answer with its headers), `tables/`,
`derived/analysis-output.txt`, `send-log.txt`, `tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab
PC, listed by hash in `SHA256-kept-on-lab-pc.txt`: the screenshots, the four backups, the listings and
the working log.
