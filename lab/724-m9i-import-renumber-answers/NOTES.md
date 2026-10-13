# Does an imported voucher renumber the vouchers already in the book under "Renumber Vouchers"? (#724, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 2 (its GUID is in every marks answer), the Receipt
voucher type only. TallyPrime 7.1, Silver Rental, on Windows. One run. No ComplyEaze Bridge build: three
raw imports of one Receipt each (the shape ComplyEaze Bridge's builder sends: no number, a new REMOTEID),
each file sent once; raw reads by curl, one at a time; the two type switches made on Tally's own Receipt
type screen. A backup was taken first and restored at the end. Times in send-log.txt are UTC.

## Method

- The three reads are the request files of this day's earlier run on the same book (branch
  `lab/724-m8r-renumber-answers`): marks, series (24 types) and the vouchers of July 2026.
- After every step: marks, vouchers, and the table in `tables/` (with NUMBERINGSTYLE and VOUCHERRETAINKEY);
  a series read at the start, after each switch and at the end. One more marks read was sent just before
  each import. `tools/read-answers.ps1` parses the answers into `derived/analysis-output.txt`.

## Result

Receipts as date:number; marks = ALTVCHID / ALTMSTID. Each Receipt's ALTERID equalled its MASTERID at
every step.

| step | act | Receipts after it | marks |
|---|---|---|---|
| i0 | as found, type on Retain Original Voucher No. | 1 Jul:1, 2 Jul:7, 3 Jul:5, 5 Jul:4, 10 Jul:2, 20 Jul:3, 25 Jul:6 | 12 / 235 |
| i1 | import H, 6 Jul | H took 8; nothing else moved | 13 / 235 |
| i2 | type set to Renumber Vouchers | date order: 1, 2, 3 Jul = 1, 2, 3; 5 Jul 4; 6 Jul (H) 5; 10 Jul 6; 20 Jul 7; 25 Jul 8 | 13 / 237 |
| i3 | import G, 4 Jul | G took 4; the Receipts of 5, 6, 10, 20 and 25 Jul each went up by one | 14 / 237 |
| i4 | import I, 26 Jul | I took 10; nothing else moved | 15 / 237 |
| i5 | type set back to Retain | entry order again: the seven as at i0, then H 8, G 9, I 10 | 15 / 239 |
| End | restore | 12 vouchers; the voucher, series and marks answers byte-identical to i0's | 12 / 235 |

- At i3 the import moved the numbers of five later-dated Receipts, H (itself imported) among them. No
  ALTERID changed on any of them and ALTVCHID rose by one only. The import answer read CREATED 1, ALTERED 0.
- Every import answered CREATED 1 with every other counter 0. The imported Receipts read as their files
  (Cash, ISDEEMEDPOSITIVE Yes, negative; Suspense, No, positive); the read returns no narration.
- At i2 six numbers changed and at i5 eight, with no ALTERID change either time. After i5 the numbers
  followed VOUCHERRETAINKEY (1, 5, 9, ... 37), which is the order of entry.
- Series for Receipt: `Auto Retain` (i0, i5, End), `Auto Renumber` (i2); the other 23 types' whole rows
  equal i0's at i2, i5 and End. The five vouchers that are not Receipts read as at i0 at every step.

## Prompts and what was done by hand

At each switch Tally asked the same two questions as in the earlier run, each answered Yes once:
"Changing the method of voucher numbering will affect voucher numbers of existing transactions. Continue?
Yes or No", then on accepting "Are you sure you want to update the voucher numbers of existing vouchers?
Y : Accept Master & Update Vouchers / N : Back". On the way to the behaviour field the cursor passes
"Method of Voucher Numbering", whose list showed two rows matching the text "Automatic"; the row
"Automatic", the value already set, was clicked and taken at both switches (the series reads show
`Automatic` throughout). The backup asked "Accept ?"; the restore asked "Accept ?" and then whether to
replace the existing company data folder (Replace). No import raised anything on Tally's screen. The run
had no pause.

## Two slips of the reading script

Neither sent or re-sent a request. At i1 the script's comparison with i0 did not run (one variable was
used for two things) and it printed the seven old Receipts as new; it was corrected and the comparison
redone from the two saved answers. At End it compared with i5 and reported the three imported Receipts as
gone, which is what the restore is for; End was then compared with i0. The working log records both.

## What it decides

On this book, this release and this one voucher type: a numberless import dated before existing vouchers,
into a type on "Renumber Vouchers", changed the numbers of the later-dated vouchers, hand-entered and
imported alike, as a hand entry did earlier the same day; nothing in ALTERID, in ALTVCHID or in the import
answer showed it. Under Retain an import took the next number and moved nothing.

## Not covered

A batch of several vouchers under "Renumber Vouchers"; Journal, Payment, Contra; a deletion; a type with
many vouchers; Gold; other releases; the same through ComplyEaze Bridge's own post.

## Files

`requests/` (6), `answers/` (every answer with its headers), `tables/`, `derived/analysis-output.txt`,
`send-log.txt`, `tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab PC, listed by hash in
`SHA256-kept-on-lab-pc.txt`: the screenshots, the backup, the listings, the restore plan and the working log.
