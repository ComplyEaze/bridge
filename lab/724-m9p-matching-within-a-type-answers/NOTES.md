# Manual numbering with duplicates prevented: a Payment is refused for a number a Payment holds, a Journal is not refused for a number only a Payment holds, small letters are not refused in themselves, and a number's end space does not come back (#724, Windows lab, 15 Oct 2026)

Synthetic data only: BRIDGE MANUAL NUMBERING LAB 1 (its GUID is in every marks answer), the book of the
runs on branches `lab/724-m9n-held-number-duplicates-prevented-answers` and
`lab/724-m9o-how-a-held-number-is-matched-answers`. Its Journal and Payment types were already on Manual
with "Prevent creating duplicate Voucher Nos." Yes. TallyPrime 7.1, Silver, on Windows. One run. No
ComplyEaze Bridge build and no screen act on any voucher type: six raw imports of one numbered voucher
each, each file sent once; raw reads by curl, one at a time. Item description and requests: branch
`lab/724-m9p-run-sheet` at 827e0f52. A backup was taken first and restored at the end. Times in
send-log.txt are UTC.

## Method

- Reads: marks, series (24 types), the vouchers of July 2026. After every import: marks, vouchers with
  the table in `tables/`, then series, compared by program, row by row, with the read before it; a marks
  read just before each import.
- Before the first read the comparers were checked on kept answers: the series comparer found 24 equal
  rows for the 724-M9O run's o0 against o1 (different bytes) and exactly one differing row, Journal, for
  the 724-M9L run's l5 against l8; the voucher comparer found everything under COLLECTION equal for the
  724-M9O run's o3 against o4 and not equal for its o0 against o1. The table program printed every kept
  voucher number untrimmed with its length, and a parsed "M9P-S1 " as "M9P-S1[space]" (7).
- A voucher number is copied exactly as the read returned it, with its length; a space is written
  [space]. A Journal file is Dr Lab Charges / Cr Suspense; a Payment file is Dr Lab Charges / Cr Lab Bank.
  Each imported voucher was found by type and date and read as its file (no narration is returned).
- As found (p0): a Receipt, a Payment and a Journal, each numbered "1"; marks 3 / 215; every series row
  and everything under COLLECTION of the voucher read equal to (and byte-identical with) the 724-M9O
  run's o0 answers.

## Result

Marks = ALTVCHID / ALTMSTID. "(n)" after a number is its length in characters.

| step | import | answer | the book after | marks |
|---|---|---|---|---|
| p0 | as found | | Receipt "1", Payment "1", Journal "1" | 3 / 215 |
| p1 | Payment "M9P-P1", 20 Jul, 61.00 | CREATED 1 | Payment reads "M9P-P1" (6), MASTERID 4 | 4 / 215 |
| p2 | Payment "M9P-P1", 21 Jul, 62.00 | CREATED 0, EXCEPTIONS 1 | unchanged, 4 vouchers | 4 / 215 |
| p3 | Journal "M9P-P1", 22 Jul, 63.00 | CREATED 1 | Journal reads "M9P-P1" (6), MASTERID 5; the Payment unchanged | 5 / 215 |
| p4 | Journal "m9p-x1", 23 Jul, 64.00 | CREATED 1 | Journal reads "m9p-x1" (6), MASTERID 6 | 6 / 215 |
| p5 | Journal "M9P-S1 " (7, a space at the end), 25 Jul, 66.00 | CREATED 1 | Journal reads "M9P-S1" (6), no space, MASTERID 7 | 7 / 215 |
| p6 | Journal "M9P-S1" (6), 26 Jul, 67.00 | CREATED 0, EXCEPTIONS 1 | unchanged, 7 vouchers | 7 / 215 |
| End | restore | | 3 vouchers | 3 / 215 |

- p2 (Payment against Payment): refused, no voucher changed. The answer is byte-identical to the
  refusal of the 724-M9N and 724-M9O runs (538 bytes, sha256 09a713e2...f874): CREATED 0, EXCEPTIONS 1,
  LASTVCHID 0, no LINEERROR. Everything under COLLECTION of the voucher read equalled the read before it.
- p3 (Journal against Payment): created. The Journal reads "M9P-P1" while the 20 Jul Payment holds
  "M9P-P1"; every Payment is unchanged. With the 724-M9O run's o3 (a Payment took a number only a Journal
  held), the check is not across these two types in either direction on this book.
- p4 (small letters, no held twin): created, and it reads "m9p-x1" in small letters as sent. Small letters
  are not refused in themselves here, so the 724-M9O run's refusal of "m9o-a1" beside a held "M9O-A1"
  is a match made without regard to letter case.
- p5 (a space at the end): created. The voucher read returns "M9P-S1", six characters, UTF-16 code units
  U+004D U+0039 U+0050 U+002D U+0053 U+0031: no space. The raw answer's element text reads the same.
  Whether Tally dropped the space on import or on export cannot be told apart from these reads.
- p6 (the same number without the space): refused, no voucher changed, the same 538-byte refusal.
  Because p5's Journal read back without its space, the two numbers read the same, and this is the
  known refusal of a held number.
- A created voucher's answer read CREATED 1 with the seven other counters 0 and LASTVCHID equal to the
  new MASTERID (4, 5, 6, 7); it is 538 bytes and differs from the refusal in three bytes.
- All four created vouchers read NUMBERINGSTYLE Manual, ISOPTIONAL No, ISCANCELLED No;
  VOUCHERRETAINKEY 5, 5, 9, 13 in the order created; each MASTERID one above the highest in the book.
- No Journal or Payment other than the one just imported changed in any field at any step; the Receipt
  read as at p0 throughout.
- Series: after each of the six imports, the two refused ones too, all 24 rows equalled the read before
  it; the Journal and Payment series read Manual, Auto Retain, PREVENTDUPLICATES Yes throughout.
- After the restore the marks, voucher and series answers are each byte-identical to p0's.

## Screen

No import raised anything on Tally's screen: the whole-window capture after each import is
byte-identical to the Gateway capture taken at p0. The Gateway line "(e) Data exceptions exist" was
there at p0, after every import and after the restore, unchanged; it was not opened. The backup asked
"Accept ?"; the restore asked "Accept ?" and then whether to replace the company data folder (Replace).
The backup went to a new empty local folder; before the restore was accepted, the Restore list showed
that one backup, by its date and time.

## Before the run

The desktop's text-input panel held the keyboard focus when the item began; Tally's window was made
active by one click on its title band before the company was loaded. Nothing was keyed in any other
window.

## What it decides

On this book and this release, under Manual numbering with duplicates prevented: a Payment is refused
for a number another Payment holds; a Journal is not refused for a number only a Payment holds; a number
in small letters with no held twin is kept in small letters, so a refusal of small letters beside a held
capital twin is a case-blind match; a space at the end of a sent number does not come back in the
voucher read, and the same number without the space is then refused.

## Not covered

Whether the end space was dropped on import or on export; a space at the start or inside a number; a
number with a leading zero and no held twin; Receipt and Contra; "Automatic (Manual Override)"; a file of
several vouchers; Multi-user Auto; a deletion; whether a restore undoes an entry in Tally's
data-exceptions store; Gold; other releases; ComplyEaze Bridge's own post.

## Files

`requests/` (9), `answers/` (each with its headers; the /status bodies as `.body.txt`), `tables/`,
`send-log.txt`, `SHA256.txt`. Kept on the lab PC, by hash in `SHA256-kept-on-lab-pc.txt`: screenshots,
the backup, folder listings and step-0 observations, the restore plan, the helper, the working log.
