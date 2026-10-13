# Under "Renumber Vouchers": an import of two Receipts in one file, and an imported Receipt that carries a number (#724, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 2 (its GUID is in every marks answer), the Receipt
voucher type only. TallyPrime 7.1, Silver Rental, on Windows. One run, a follow-up to the run on branch
`lab/724-m9i-import-renumber-answers`. No ComplyEaze Bridge build: three raw imports in the shape its
builder sends, each file sent once; raw reads by curl, one at a time; the two type switches made on
Tally's own Receipt type screen. A backup was taken first and restored at the end. Times in
send-log.txt are UTC.

## Method

- The three reads are that earlier run's request files: marks, series (24 types), the vouchers of July
  2026. After every step: marks, vouchers and the table in `tables/`; a series read at the start, after
  each switch and at the end; one more marks read just before each import.
- The imports: C, one Receipt of 7 Jul carrying the number "M9J-C1"; B, one file of two Receipts without
  numbers, 8 Jul first and 6 Jul second; N, one Receipt of 4 Jul carrying "M9J-N1". Each imported Receipt
  was found by its date and read as its file; the read returns no narration.

## Result

Receipts as date:number; marks = ALTVCHID / ALTMSTID. Each Receipt's ALTERID equalled its MASTERID at
every step.

| step | act | Receipts after it | marks |
|---|---|---|---|
| j0 | as found, type on Retain Original Voucher No. | 1 Jul:1, 2 Jul:7, 3 Jul:5, 5 Jul:4, 10 Jul:2, 20 Jul:3, 25 Jul:6 | 12 / 235 |
| j1 | import C (numbered, 7 Jul) | it read "8"; nothing else moved | 13 / 235 |
| j2 | type set to Renumber Vouchers | date order: 1, 2, 3, 5, 7, 10, 20, 25 Jul = 1 to 8 | 13 / 237 |
| j3 | import B (two, no numbers) | 6 Jul took 5, 8 Jul took 7; 7 Jul went 5 to 6; 10, 20 and 25 Jul each went up by two | 15 / 237 |
| j4 | import N (numbered, 4 Jul) | it read "4"; the seven later-dated Receipts each went up by one | 16 / 237 |
| j5 | type set back to Retain | entry order again: the seven as at j0, then 7 Jul 8, 8 Jul 9, 6 Jul 10, 4 Jul 11 | 16 / 239 |
| End | restore | 12 vouchers; the marks, voucher and series answers byte-identical to j0's | 12 / 235 |

- Neither supplied number was kept: no Receipt read "M9J-C1" or "M9J-N1" at any step. Under Retain the
  numbered import took the next number; under Renumber it was numbered by its date.
- At j3 both Receipts of the one file were numbered by their dates and four Receipts already in the book
  changed number; at j4 seven did. No ALTERID changed at any step, ALTVCHID rose only by the vouchers
  created, and the three import answers read CREATED 1, 2 and 1 with ALTERED 0 and every other counter 0.
- In B the first voucher of the file (8 Jul) took MASTERID 14 and the second (6 Jul) 15.
- Series for Receipt: `Auto Retain` (j0, j5, End), `Auto Renumber` (j2); the other 23 types' whole rows
  equal j0's at j2, j5 and End. The five vouchers that are not Receipts read as at j0 at every step.

## Prompts and what was done by hand

At each switch the cursor passed "Method of Voucher Numbering", whose list showed two rows containing
the word Automatic; the row reading exactly "Automatic" (the value already set) was taken and the
field read "Automatic" both times. Tally then asked two questions, each answered Yes once:
"Changing the method of voucher numbering will affect voucher numbers of
existing transactions. Continue? Yes or No", then on accepting "Are you sure you want to update the
voucher numbers of existing vouchers? Y : Accept Master & Update Vouchers / N : Back". The backup asked
"Accept ?"; the restore asked "Accept ?" and then whether to replace the company data folder (Replace).
No import raised anything on Tally's screen.

## Two slips

Neither touched the book or sent a request twice. The working notes were opened only after the
company-list filter had been typed, so the first kept screenshot shows the filtered list, no company
loaded. One command for j4 did not parse and ran nothing; it was corrected and run once.

## What it decides

On this book, this release and this one voucher type: a voucher number supplied in an import was
discarded under both settings. Under "Renumber Vouchers" a numbered import and a two-voucher batch, dated
before existing vouchers, each changed the numbers of later-dated vouchers, and nothing in ALTERID, in
ALTVCHID or in the import answer showed it.

## Not covered

A batch that mixes numbered and numberless vouchers; a number an existing voucher already holds;
Journal, Payment, Contra; a deletion; a type with many vouchers; Gold; other releases; the same through
ComplyEaze Bridge's own post.

## Files

`requests/` (6), `answers/` (each with its headers), `tables/`, `derived/` (the answers parsed by
`tools/read-answers.ps1`), `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in
`SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the restore plan, the working log.
