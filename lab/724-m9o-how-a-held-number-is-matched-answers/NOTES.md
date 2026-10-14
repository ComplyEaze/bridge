# Manual numbering with duplicates prevented: a number held by a Journal does not block a Payment; the same number in small letters is refused; "01" is kept beside "1" (#724, Windows lab, 14 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 1 (its GUID is in every marks answer),
the book of the run on branch `lab/724-m9n-held-number-duplicates-prevented-answers`. Its Journal and
Payment types were already on Manual with "Prevent creating duplicate Voucher Nos." Yes. TallyPrime 7.1,
Silver Rental, on Windows. One run. No ComplyEaze Bridge build and no screen act on any voucher type: six
raw imports of one numbered voucher each, each file sent once; raw reads by curl, one at a time. A backup
was taken first and restored at the end. Times in send-log.txt are UTC.

## Method

- Reads: marks, series (24 types), the vouchers of July 2026. After every import: marks, vouchers with
  the table in `tables/`, then series, compared by program, row by row, with the read before it; a marks
  read just before each import. Both comparers were first checked on kept answers of two earlier runs.
- A Journal file is Dr Lab Charges / Cr Suspense; a Payment file is Dr Lab Charges / Cr Lab Bank. Each
  imported voucher was found by type and date and read as its file (no narration is returned).
- As found: a Receipt, a Payment and a Journal, each numbered "1".

## Result

Marks = ALTVCHID / ALTMSTID. Every number below is copied exactly as the read returned it.

| step | import | answer | the book after | marks |
|---|---|---|---|---|
| o0 | as found | | Receipt "1", Payment "1", Journal "1" | 3 / 215 |
| o1 | Journal "M9O-A1", 20 Jul | CREATED 1 | reads "M9O-A1", MASTERID 4 | 4 / 215 |
| o2 | Payment "M9O-P1", 21 Jul | CREATED 1 | reads "M9O-P1", MASTERID 5 | 5 / 215 |
| o3 | Payment "M9O-A1", 22 Jul | CREATED 1 | reads "M9O-A1", MASTERID 6; the Journal "M9O-A1" unchanged | 6 / 215 |
| o4 | Journal "m9o-a1", 23 Jul | CREATED 0, EXCEPTIONS 1 | unchanged, 6 vouchers | 6 / 215 |
| o5 | Journal "01", 24 Jul | CREATED 1 | reads "01", two characters, MASTERID 7 | 7 / 215 |
| o6 | Journal "M9O-B1", 10 Jul | CREATED 1 | reads "M9O-B1", MASTERID 8 | 8 / 215 |
| End | restore | | 3 vouchers | 3 / 215 |

- o3: a Payment took a number that only a Journal held. Both vouchers read "M9O-A1" afterwards.
- o4: the refusal is byte-identical to the earlier run's (538 bytes, sha256 09a713e2...f874): CREATED 0,
  EXCEPTIONS 1, LASTVCHID 0, no LINEERROR, no other text. Everything under COLLECTION of the voucher
  read was equal to the read before it. No small-letter number WITHOUT a held twin was sent, so this
  refusal alone does not show that Tally matches numbers without regard to letter case.
- o5: "01" was created and reads "01"; the three vouchers as found still read "1".
- o6: a back-dated Journal with a new number was created; no other voucher changed.
- A created voucher's answer read CREATED 1 with the seven other counters 0 and LASTVCHID equal to the
  new MASTERID; it is 538 bytes and differs from the refusal in three bytes.
- No Journal or Payment other than the one just imported changed in any field at any step; the Receipt
  read as at o0 throughout. All five created vouchers read NUMBERINGSTYLE Manual; VOUCHERRETAINKEY 5, 5,
  9, 9, 13 in the order created.
- Series: after each of the six imports, the refused one too, all 24 rows equalled the read before it.
- After the restore the marks, voucher and series answers are each byte-identical to o0's.

## Screen

No import raised anything on Tally's screen: the whole-window capture after each import is byte-identical
to the one taken at o0. The Gateway line "(e) Data exceptions exist" was there at o0, after every import
and after the restore, unchanged; it was not opened. The backup asked "Accept ?"; the restore asked
"Accept ?" and then whether to replace the company data folder (Replace).

## One slip

At o1 the lab's own helper miscounted and halted after the marks and voucher reads. It was corrected,
the two kept answers were judged, and the series read was sent once. Nothing was sent twice.

## What it decides

On this book and this release, under Manual numbering with duplicates prevented: a number held only
by a Journal does not block a Payment that carries it; "01" does not meet a held "1"; a back-dated voucher
with a new number is kept; a Journal numbered like a held Journal but in small letters is refused.

## Not covered

A small-letter number, or one with a leading zero, with no held twin; a Journal whose number only a
Payment holds; Receipt and Contra; spaces in a number; "Automatic (Manual Override)"; a file of several
vouchers; a deletion; Gold; other releases; ComplyEaze Bridge's own post.

## Files

`requests/` (9), `answers/` (each with its headers), `tables/`, `derived/`, `tools/`, `send-log.txt`,
`SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots, the backup,
listings, the helper, the working log.
