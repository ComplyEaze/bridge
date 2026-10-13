# The two ledger listings with the to-dates a Sales invoice build sends, on W18's book state (#1342 W18c, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE PILOT LAB (its GUID is in every marks answer). TallyPrime 7.1, Silver
Rental, on Windows. One run. No ComplyEaze Bridge build: six raw reads by curl, one at a time, each sent
once; the only writes were two restores made on Tally's own screen. Nothing was keyed, altered or
deleted in the book and Tally's date was not changed. Times in send-log.txt are UTC.

## What was done

1. The book as found was backed up (90,051 bytes) and its marks read (`aw18c-marks-now`: 68 / 245).
2. The company was shut and the backup the W19 run took before its first write was restored over it. It
   was identified by file name, 85,468 bytes and SHA-256 `d4d6e200...5052`, checked on the file before
   the restore and again with the Restore screen up.
3. Marks (`-marks-0`), the compliance listing with the to-date 15 Sep 2026, the paired listing with the
   to-date 1 Oct 2026, marks again (`-marks-1`).
4. The company was shut and the first backup restored; marks (`-marks-2`); the company was shut again,
   as it had been found.

## Result against the four expectations written before the run

marks = ALTVCHID / ALTMSTID.

| | expectation | result |
|---|---|---|
| a | marks-0 and marks-1 byte-identical to W18's first marks answer | met (60 / 243) |
| b | compliance answer byte-identical to W18's raw compliance answer | met: 38,278 bytes, 18 ledgers |
| c | paired answer byte-identical to W18's | NOT met: 25,518 bytes against 25,480 |
| d | marks-2 equal to marks-now | met, byte-identical |

- (c): both answers hold the same 18 ledgers; CLOSINGBALANCE differed on 13 of them and no other field
  differed (`derived/paired-diff-against-w18.txt`). W18 had asked the to-date 1 Aug 2026. The paired
  listing's closing balances followed the to-date; the compliance listing, which asks for no closing
  balance, did not change with it.
- Every answer was HTTP 200, a complete envelope with one COLLECTION and no LINEERROR, in 0.3 s.

## Prompts, and what was pressed

- The Backup screen and each Restore screen asked "Accept ?" (Yes or No): Yes each time. The item's
  list does not name this prompt; it is the screen's own accept, as in the earlier runs.
- Each restore then showed "Company data folder (100033) already exists. Selecting Skip will cancel the
  Restore process for this company. Selecting Replace will overwrite the existing folder.": Replace each
  time. The Restore list showed one row each time, BRIDGE PILOT LAB (100033), with the date and time of
  the backup meant.
- No other dialog. The Gateway line of this company carried "(e)" and the footnote "(e) Data exceptions
  exist" throughout; it was on this book's Gateway before the run and nothing was pressed for it.
- Tally's current date read 15-Sep-2026 at the start and at the end. One Tally process owned the port
  throughout. No company was loaded before the run or after it.
- Beyond the item's list, kept on the lab PC: screenshots, a listing of the company's data folder around
  each restore, a restore plan. No status request was sent.

## What it decides

A replay built on W18's answers must take the paired listing from this run (to-date 1 Oct 2026); W18's
compliance answer stands for the to-date 15 Sep 2026 as it is. One book, one release, one run.

## Files

`requests/` (3), `answers/` (six, each with headers), `derived/`, `send-log.txt`,
`tools/read-answers.ps1`, `SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`:
screenshots, both backups, listings, the working log.
