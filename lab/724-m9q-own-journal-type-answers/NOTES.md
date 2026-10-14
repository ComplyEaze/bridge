# A raw voucher-type Create under Journal gives a series numbered Manual with duplicates prevented; numbers held in a parent and its child do not block each other; a Create on an existing name alters it (#724, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 1 (its GUID is in every marks answer), the book of the runs
on branches `lab/724-m9n-held-number-duplicates-prevented-answers` and
`lab/724-m9o-how-a-held-number-is-matched-answers`; its Journal and Payment types read Manual with duplicate
numbers prevented. TallyPrime 7.1, Silver Rental, on Windows. One run. No ComplyEaze Bridge build and no
screen act on any voucher type: raw imports by curl, each file sent once, raw reads one at a time. A backup
was taken first and restored at the end. Times in send-log.txt are UTC.

## Method

- Reads: marks, series (every voucher type), the vouchers of July 2026. After every import: marks, vouchers
  with the table in `tables/`, then series compared by program row by row (paired by MASTERID) with the read
  before it; a marks read just before each import. The comparers were first checked on kept answers of two
  earlier runs.
- Type files are master imports ("All Masters"); Journal files are Dr Lab Charges / Cr Suspense.
- As found: a Receipt, a Payment and a Journal, each numbered "1"; 24 voucher types.

## Result

| step | import | answer | the book after | marks |
|---|---|---|---|---|
| q0 | as found | | 3 vouchers, 24 types | 3 / 215 |
| q1 | Create "CB Journal A" under Journal, type-level Manual / duplicates Yes only | CREATED 1 | new type, MASTERID 210 | 3 / 217 |
| q2 | Create "CB Journal B" under Journal, with a series list | CREATED 1 | new type, MASTERID 212 | 3 / 219 |
| q3 | Journal "M9Q-P1", 20 Jul | CREATED 1 | reads "M9Q-P1", MASTERID 4 | 4 / 219 |
| q4 | CB Journal B "M9Q-P1", 21 Jul | CREATED 1 | reads "M9Q-P1", MASTERID 5; the Journal's unchanged | 5 / 219 |
| q5 | CB Journal B "M9Q-C1", 22 Jul | CREATED 1 | reads "M9Q-C1", MASTERID 6 | 6 / 219 |
| q6 | Journal "M9Q-C1", 23 Jul | CREATED 1 | reads "M9Q-C1", MASTERID 7; the child's unchanged | 7 / 219 |
| q7 | not sent | | "M9Q-C1" was held by two vouchers, not by the child alone | |
| q8 | Create "CB Journal B" again, duplicates No | CREATED 0, ALTERED 1 | 26 types; CB Journal B altered | 7 / 221 |
| End | restore | | 3 vouchers, 24 types | 3 / 215 |

- q1 and q2: each new type's row holds ONE series list, "Default": NUMBERINGMETHOD Manual,
  NUMBERINGSUBMETHOD Default, PREVENTDUPLICATES Yes; the type-level elements read Manual and Yes. So the
  type-level elements alone (q1) already give a series numbered Manual with duplicates prevented.
- q4: a number held only by a Journal of the parent type was taken by the child type. q6: a number held
  only by the child's Journal was taken by the parent. Every created voucher reads NUMBERINGSTYLE Manual.
- q8: the Create on a name that exists was not refused: CREATED 0, ALTERED 1, no error; CB Journal B's
  ALTERID went 219 to 221 and PREVENTDUPLICATES went Yes to No both at type level and in its series
  (the method stayed Manual); the number of types stayed 26.
- Every import answer is 538 bytes with no LINEERROR. No voucher of the book as found changed at any
  step, no other series row changed, and Journal's series read Manual with duplicates prevented
  throughout; the only other series change was CB Journal B's CANDELETE going Yes to No once a voucher
  used it.
- After the restore the marks, voucher and series answers are each byte-identical to q0's; both new
  types are gone.

## Screen

No import raised anything on Tally's screen: the whole-window capture after each import is byte-identical
to q0's. The Gateway line "(e) Data exceptions exist" was there throughout, unchanged; it was not opened.
The backup asked "Accept ?"; the restore asked "Accept ?" and whether to replace the company data folder
(Replace); each answered by mouse.

## What it decides

On this book and release: ComplyEaze Bridge can give itself a Journal type numbered Manual with duplicate
numbers prevented by one raw Create, with or without a series list; Tally's duplicate check does not look
across a parent type and its child; a Create on a name that already exists alters that type in place,
including its duplicate setting, without a refusal.

## Not covered

A sibling (q7 was not sent); a type under Payment, Receipt or Contra; Automatic (Manual Override); several
series; a native post into the new type; the voucher-type screen; Gold; other releases.

## Files

`requests/` (the reads and the type and Journal files of this run's sheet), `answers/` (each with its
headers), `tables/`, `derived/`, `tools/`, `send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in
`SHA256-kept-on-lab-pc.txt`: screenshots, the backup, listings, the working log and summary.
