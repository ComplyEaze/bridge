# Which ledger a voucher lands on when names differ by a trailing line break, by case, or by an alias (#626, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE LINE BREAK LAB (GUID 224e7f8c-a0aa-45ce-889a-ad4b7fdbe852). TallyPrime 7.1,
Silver Rental, on Windows. One run. No ComplyEaze Bridge build: raw HTTP requests by curl, one at a time,
each write sent once, plus three acts on Tally's own ledger screens. A backup was taken first and restored
at the end. Times in send-log.txt are UTC.

## Method

- After every step: marks, the ledger list and the August vouchers (`requests/marks`, `ledgers`,
  `vch-aug`); tables per step in `tables/`. One more marks read was sent just before each write.
- "Which ledger" = the debit entry's LEDGERNAME exactly as read, matched on the NAME attribute of exactly
  one LEDGER row of the same step.
- `tools/read-answers.ps1` parses the answers into `derived/analysis-output.txt`; names are printed with
  their length and with a line break shown as \r\n. Nothing is counted by text search.

## Result (marks = ALTVCHID / ALTMSTID)

| step | act | what was read | marks |
|---|---|---|---|
| t0 | as found | 5 ledgers, among them "Lab Line Break Ledger" + CR LF (23 characters, GUID ending ce) and "Lab Plain Ledger" (cf); one August voucher | 2 / 212 |
| t1 | import, Dr "Lab Plain Ledger" (control) | CREATED 1; debits cf | 3 / 212 |
| t2 | Create "Lab Line Break Ledger" without the break | CREATED 1: a second ledger (21 characters, GUID ending d1); the first unchanged | 3 / 213 |
| t3 | import, Dr "LAB LINE BREAK LEDGER" (capitals, no break) | CREATED 1; the debit reads "Lab Line Break Ledger" (21) = d1, the one WITHOUT the break | 4 / 213 |
| t4 | Create "LAB CASE PROBE" | CREATED 1 (GUID ending d2) | 4 / 214 |
| t5 | Create "lab case probe", another group | ALTERED 1, CREATED 0: the same GUID, its group changed; NAME attribute still "LAB CASE PROBE", NAME element "lab case probe" | 4 / 215 |
| t6 | screen: alias "Lab Plain Alias" added to cf | same GUID, a second NAME element | 4 / 216 |
| t7 | import, Dr "Lab Plain Alias" | CREATED 1; the debit reads "Lab Plain Ledger" = cf | 5 / 216 |
| t8 | screen: Create Ledger "LAB PLAIN LEDGER" | refused at the Name field (text below); nothing saved | 5 / 216 |
| t10 | screen: "Lab Screen Break" + CR LF pasted into the Name field, accepted | stored NAME is 16 characters, no line break (GUID ending d3) | 5 / 217 |
| End | restore | ledger list and vouchers as t0; the three answers byte-identical to t0's | 2 / 212 |

The planned import against a case twin made on screen (t9) was not run: t8 made no twin.

## The two pauses at t8

On Enter in the Name field Tally showed a red box: "Error / Oops! / Name already exists. / Enter a
different name." The run paused with the box on screen, as the steps required for a message at that field.
Leaving was then approved: one click in the Name field (the box went), then Ctrl+Q once. Tally asked no
quit question and showed the "Master Creation" list, not the Gateway; the run paused again until one more
Ctrl+Q was approved, which reached the Gateway. Nothing was sent while the box or the form was on screen.
The backup asked "Accept ?"; the restore asked "Accept ?" and then whether to replace the existing company
data folder (Replace). The two ledger accepts asked nothing.

## What it decides

On this book and this release:

- A name that differs from two ledgers only by case, where the two differ by a trailing line break, was
  accepted by import and landed on the ledger without the break.
- Two ledgers differing by case alone could not be made: a Create by request altered the existing ledger,
  and the screen refused the name.
- A voucher naming an alias landed on the aliased ledger and read back under the ledger's own name.
- The ledger screen dropped a pasted trailing line break; on this book only an import has stored one.

## Not covered

Both creation orders of each pair; an alias that only folds equal to another ledger's name; a balance
read by GUID; Gold; other releases; the same through ComplyEaze Bridge's own build and post.

## Files

`requests/` (the nine files sent), `answers/` (every answer with its headers), `tables/`,
`derived/analysis-output.txt`, `send-log.txt`, `tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab PC,
listed by hash in `SHA256-kept-on-lab-pc.txt`: the screenshots, the backup, the process and port listings,
the restore plan, the data-folder listings and the working log. The first three screenshots are of the
whole desktop with another window over the right fifth of Tally's window; the later ones are of Tally's
window alone.
