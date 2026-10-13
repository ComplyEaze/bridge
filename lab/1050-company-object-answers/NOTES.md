# Does anything on the COMPANY object show that a backup was restored? (#1050, Windows lab, 13 Oct 2026)

Synthetic data only: BRIDGE IDENTITY LAB (its GUID: branch `lab/1050-identity-answers`). TallyPrime 7.1,
Silver Rental, security off, on Windows. One run. No ComplyEaze Bridge build and no import: raw HTTP reads
by curl, one at a time; the backup and the three restores were made on Tally's own screens.

## Method

- The whole Company object was read seven times with one request: a collection of type Company with
  `NATIVEMETHOD *` (the request file `cmp-a` of the branch `lab/626-1038-1050-run-sheets`). Each answer
  held one COMPANY row with the company's GUID: 32,986 bytes, 412 different element names.
- Each row was flattened to "path = text" for every attribute and every leaf element, empty ones included:
  414 paths, 199 of them with text. A DIFF of two reads is the list of paths whose text differs or that
  exist in one read only.
- Before each of the seven reads: a marks read and a read of the July vouchers.
- Privacy: a whole-company answer can carry licence, account or contact values, so the seven raw answers
  and the flat files stay on the lab PC, listed by hash. This folder holds the result table, the diffs and
  the hashes only. The one path in the table holds a date; nothing was withheld from it.

## The seven reads

| read | taken after | marks (ALTVCHID / ALTMSTID) | vouchers | against the read before |
|---|---|---|---|---|
| 1 | nothing (as found) | 2 / 211 | 2 | |
| 2 | 5 minutes with nothing touched | 2 / 211 | 2 | byte-identical |
| 3 | the company shut and loaded again | 2 / 211 | 2 | byte-identical |
| 4 | a backup | 2 / 211 | 2 | byte-identical |
| 5 | that backup restored over the company | 2 / 211 | 2 | byte-identical |
| 6 | an older backup restored (one voucher fewer) | 1 / 211 | 1 | one path differs |
| 7 | the first backup restored again | 2 / 211 | 2 | byte-identical to read 5 |

## Result

- A. Paths that changed at the restore of the same content and not by themselves, on a reload or at a
  backup: none among 414.
- B. Paths that differ between two restores of the same backup and not by themselves or on a reload: none
  among 414.
- The only path that moved in the whole run: `COMPANY/AUDITEDUPTO[0]`, '20260710' with the book as found
  and '20260701' with the older backup. Each value is the date of the last voucher the book then held,
  so here it followed the content of the book; it did not mark a restore.
- The row carried no ALTVCHID and no ALTMSTID element in this shape; the two marks come from the separate
  marks request.
- Every restore asked "Accept ?" and then whether to replace the existing company data folder (100056);
  Replace was chosen each time, and a listing of that folder before and after shows it was rewritten.
- After the last restore the voucher answer equals the first one under COLLECTION, and is the same bytes.
- The run had no pause.

## What it decides

On this book and this release, with security off: no value returned by this read of the company changed
when a backup was restored and at no other time. Taking a backup or reloading the company changed no
value either.

## Not covered

A restore made on another machine or into a new folder; security on; Gold; other releases; a company
value after the book is keyed past the restore; fields that `NATIVEMETHOD *` does not return.

## Files

`derived/result-table.md`, `derived/flat-diff.txt` (path counts, hashes and the pairwise diffs),
`SHA256.txt`. Kept on the lab PC, listed by hash in `SHA256-kept-on-lab-pc.txt`: the seven raw company
answers and their flat files, the marks and voucher answers, the screenshots, the backup, the process and
port listings, the restore plan, the data-folder listings and the working log.
