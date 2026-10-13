# Manual numbering with duplicates prevented: a supplied number is kept, a number already held is refused whole, and the refusal gives no reason (#724, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 1 (its GUID is in every marks answer), the other of the
two numbering-lab books. Its Journal type was already on Manual with "Prevent creating duplicate Voucher
Nos." Yes. TallyPrime 7.1, Silver Rental, on Windows. One run, after the one on branch
`lab/724-m9m-numberless-under-manual-answers`. No ComplyEaze Bridge build and no screen act on any
voucher type: four raw imports of one numbered Journal each, each file sent once; raw reads by curl, one
at a time. A backup was taken first and restored at the end. Times in send-log.txt are UTC.

## Method

- Reads: marks, series (24 types), the vouchers of July 2026. After every step: marks, vouchers with the
  table in `tables/`, then series, compared by program, row by row, with the read before it; a marks
  read just before each import.
- JA 20 Jul 91.00, number "M9N-J1", held by no voucher. JB 21 Jul 92.00, "M9N-J1" again. JC 22 Jul
  93.00, "7", digits only, held by no voucher. JD 23 Jul 94.00, "1", digits only, the number the Journal
  of the book as found holds.
- Each imported voucher was found by type and date and read as its file (no narration is returned).

## Result

Marks = ALTVCHID / ALTMSTID.

| step | act | answer | the book after | marks |
|---|---|---|---|---|
| n0 | as found | | Receipt "1", Payment "1", Journal "1" | 3 / 215 |
| n1 | import JA | CREATED 1 | the Journal reads "M9N-J1", MASTERID 4 | 4 / 215 |
| n2 | import JB | CREATED 0, EXCEPTIONS 1 | unchanged, 4 vouchers | 4 / 215 |
| n3 | import JC | CREATED 1 | the Journal reads "7", MASTERID 5 | 5 / 215 |
| n4 | import JD | CREATED 0, EXCEPTIONS 1 | unchanged, 5 vouchers | 5 / 215 |
| End | restore | | 3 vouchers; answers byte-identical to n0's | 3 / 215 |

- A created voucher's answer read CREATED 1 with the seven other counters 0. A refusal read CREATED 0,
  ALTERED 0, DELETED 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 1, LASTVCHID 0, with no
  LINEERROR and no other text.
- The two refusals are byte-identical (538 bytes, sha256 09a713e2...f874): the same answer for a text
  number this run had just created and for a digits-only number the book held as found. The protocol
  reference gives that size and hash prefix for the refusal of a second Journal with no number on this
  book (12 Oct). The answer does not say why.
- After a refusal the vouchers read exactly as before it: everything under COLLECTION was equal.
- "7" came back as the one character "7": no padding, no space. Both new Journals read
  NUMBERINGSTYLE Manual; VOUCHERRETAINKEY 5 and 9.
- Series: after each of the four imports, the refused ones too, all 24 rows equalled the read before
  it; the Journal row read `Manual`, `Auto Retain`, PREVENTDUPLICATES `Yes` throughout, with no
  last-number entry.
- Two marks answers with equal marks still differ in bytes: counters in the answer's `CMPINFO` block
  (LEDGER, TAXUNIT, VOUCHERNUMBERSERIES; CURRENCY and VOUCHER after a created voucher) rise from one
  request to the next. Nothing under COLLECTION differs. The earlier run's notes left a difference of
  this kind unexplained.

## Prompts and one pause

No import raised anything on Tally's screen. The backup asked "Accept ?"; the restore asked "Accept ?"
and then whether to replace the company data folder (Replace). The run paused once before the backup:
the plan asked for the date and time of an older fallback backup from a paper that does not hold them;
they were taken from that file's own last-write time. The fallback was not used.

## What it decides

On this book and this release, under Manual numbering with duplicates prevented: an imported Journal
keeps a number it carries, text or digits only, if no Journal holds it; if one does, the import is
refused whole, nothing changes, and the answer is the bare counter block with EXCEPTIONS 1.

## Not covered

Duplicates prevented under "Automatic (Manual Override)"; a back-dated numbered voucher; a number with
leading zeros or spaces; Payment, Receipt and Contra; a file of several vouchers; Multi-user Auto; a
deletion; Gold; other releases; the same through ComplyEaze Bridge's own post.

## Files

`requests/` (7), `answers/` (each with its headers), `tables/`, `derived/`, `tools/`, `send-log.txt`,
`SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots, the backup,
listings, the restore plan, the working log.
