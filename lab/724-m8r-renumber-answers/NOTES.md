# What "Renumber Vouchers" does to numbers already in the book (#724, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 2 (its GUID is in every marks answer), the Receipt
voucher type only. TallyPrime 7.1, Silver Rental, on Windows. One run. No ComplyEaze Bridge build: every
voucher was entered by hand on Tally's Receipt screen (Dr Cash / Cr Suspense, no number typed) and every
read is a raw HTTP request by curl, one at a time. A backup was taken first. Times in send-log.txt are UTC.

## Method

- Marks: `requests/n2-r02-marks.xml` (ALTVCHID, ALTMSTID; one COMPANY row, the company's GUID, every time).
- Series: `requests/n2-series-vt-request.xml` (24 voucher types every time).
- Vouchers: `requests/m8r-vch-request.xml`, ComplyEaze Bridge's own voucher export text for 1 to 31 Jul
  2026.
- Before anything was entered the voucher read held the six known vouchers, one of them a Receipt.
- After each act: marks, vouchers, and the table in `tables/`. `tools/read-answers.ps1` parses the answers;
  its output is `derived/analysis-output.txt`. Nothing is counted by text search.

## Result, one line per step

Receipts as number (date); M = MASTERID. Each Receipt's ALTERID equals its MASTERID at every step.

| step | act | Receipts after it | ALTVCHID / ALTMSTID |
|---|---|---|---|
| x0 | start (type on Retain Original Voucher No.) | 1 (1 Jul, M1) | 6 / 231 |
| x1 | Receipt A, 10 Jul | A 2 (M7) | 7 / 231 |
| x2 | Receipt B, 20 Jul | B 3 (M8) | 8 / 231 |
| x3 | Receipt C, 5 Jul, back-dated | C 4 (M9); others unchanged | 9 / 231 |
| x4 | type set to Renumber Vouchers | seed 1; C 4→2; A 2→3; B 3→4 | 9 / 233 |
| x5 | Receipt D, 3 Jul, back-dated | D 2 (M10); C 2→3; A 3→4; B 4→5 | 10 / 233 |
| x6 | Receipt E, 25 Jul | E 6 (M11); others unchanged | 11 / 233 |
| x7 | type set back to Retain | seed 1; A 4→2; B 5→3; C 3→4; D 2→5; E 6 | 11 / 235 |
| x8 | Receipt F, 2 Jul, back-dated | F 7 (M12); others unchanged | 12 / 235 |
| End | reads only | as x8, row for row | 12 / 235 |

- At x4 the numbers became date order; at x7 they became entry order. On every renumbered Receipt the
  ALTERID did not change, and MASTERID and GUID stayed.
- ALTVCHID moved only by the one voucher entered at a step, and not at all at a switch.
- ALTMSTID moved by 2 at each save of the type; the type's own ALTERID read 38, then 233, then 235.
- Series for Receipt: NUMBERINGSUBMETHOD `Auto Retain` (x0), `Auto Renumber` (x4), `Auto Retain` (x7, End);
  NUMBERINGMETHOD `Automatic` and PREVENTDUPLICATES `No` throughout. The other 23 types equal x0 at x4, x7
  and End. No voucher that is not a Receipt changed at any step.
- Before accepting, the entry screen showed the number the Receipt then took (2, 3, 4, 2, 6, 7).
- Entries as read: Suspense +amount, ISDEEMEDPOSITIVE No; Cash -amount, ISDEEMEDPOSITIVE Yes.
- At the end Tally's Statistics screen reads Receipt 7; the Day Book for July after x4 shows the x4 numbers.

## The two confirmations

Tally asked twice at each switch (x4 and x7) on this type, which holds vouchers.

1. On choosing the behaviour in the field: "Changing the method of voucher numbering will affect voucher
   numbers of existing transactions. Continue? Yes or No".
2. On accepting the screen: "Are you sure you want to update the voucher numbers of existing vouchers?
   Y : Accept Master & Update Vouchers / N : Back".

The steps as written allowed for one confirmation at a switch. At x4 the run paused with the second one
on screen, unanswered, until answering it was approved. Each of the two was then answered Yes once, and
the same was done at x7.

## What it decides

On this book, this release and this one voucher type:

- "Renumber Vouchers" changed numbers already in the book: at the switch, and on a back-dated entry. An
  entry dated after every other Receipt changed nothing else.
- Setting the type back to Retain renumbered again, into entry order: A, B and C read as before x4 (2, 3,
  4), and D, which took 2 under Renumber, read 5.
- No renumbering showed in a voucher's ALTERID or in ALTVCHID, so a program reading only those would not
  have seen one. What moved was ALTMSTID and the voucher type's ALTERID, and the series'
  NUMBERINGSUBMETHOD said which behaviour was set.
- Under Retain a back-dated Receipt took the next number and left the others alone (x3, x8).

## Not covered

A voucher sent by import; a deletion; Journal, Payment, Contra; a type with more than a handful of
vouchers; Gold; other releases; "Set/Alter additional numbering details"; a type holding no voucher.

## Files

`requests/`, `answers/` (every answer with its headers), `tables/`, `derived/` (analysis-output.txt,
step-lines.txt), `send-log.txt`, `tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab PC, listed by
hash in `SHA256-kept-on-lab-pc.txt`: 21 screenshots, the backup, and the process and port listings.
