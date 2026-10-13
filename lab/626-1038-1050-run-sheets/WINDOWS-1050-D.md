# Windows item 1050-D v1.2: does anything on the COMPANY object show that a backup was restored? (13 Oct 2026; for issue #1050, whose last comment planned no further probe: this is one more, of the company, not of the voucher.) Plain HTTP reads plus Tally's own Backup and Restore screens: NO ComplyEaze Bridge build, no Terms entry, NO import. SYNTHETIC ONLY: THE COMPANY = BRIDGE IDENTITY LAB (number 100056; its GUID is the one in the answers on branch lab/1050-identity-answers). LAB-COMMON v3 applies in full; the STATE READS are MARKS and VCH. It runs when item 1038-R is finished or not yet started, never inside it. Three restores by the operator. Needs the run coordinator's go-ahead.

Why: #1050 found that no field of a VOUCHER tells a posted voucher from one keyed again after a restore, and the voucher mark stops showing the restore once the book is keyed past it. Nobody has read the COMPANY object across a restore. If a company value moves on a restore and on nothing else, ComplyEaze Bridge could record it with a post and compare it later; if none does, that way is closed too.

## Requests (each ONE line written as UTF-16LE with a BOM, no line break at the end)
- MARKS: 982 bytes, sha256 2ce259da099e9d3664ed432e1d5a1c532cc241a1a562e81cd9f3f20ba069ceae.
  File on this branch: requests/1050-D/marks.utf16le.xml
- VCH: lab/1050-identity-requests at e0d61b813704129894b7c8533e3449d5b49006cf, b1-window.xml, unchanged: 1,782 bytes, sha256 aba85b51370f86aadc6c1c81697fe670a488ce7e03067abd61a13acfee34a439 (also on this branch: requests/1050-D/b1-window.xml).
- CMP, the whole Company object: a NEW SHAPE (LAB-COMMON C3: screenshot Tally after each send of it). Two candidates; each defines the collection its header names. CMP-A is tried first.
  - CMP-A: 952 bytes, sha256 bd9459c86a6df8bc8ff06c0f50fe53a9f486dce0075b6b4185135415d4a5dfb5 (UTF-8: 475 bytes, bf76563caff9782ba6beaa454d836210a79008aabf42080fcf8175d8eb4f3cbe).
    File on this branch: requests/1050-D/cmp-a.utf16le.xml
  - CMP-B: 924 bytes, sha256 d0809910f5f5b2ddd7806c67e87abf81649d545ae3db950e4aeb6dfc04a532b3 (UTF-8: 461 bytes, 8936dd3a28fbb4bb2ad3536d3d190680736a53e4ccfe13cbc4a4babb60a91ce0).
    File on this branch: requests/1050-D/cmp-b.utf16le.xml
  - A CMP send that fails as a request (C3: no answer, non-200, short, a dialog in Tally) is a STOP like any other; it is never "failed its control", and CMP-B is not sent after it.
  - CONTROL, for an answer that arrived whole and parses: exactly one COMPANY row (C4); its GUID is THE COMPANY's; ALTVCHID and ALTMSTID, where the row carries them, equal the MARKS answer read just before; the row carries at least 20 different element names. A CMP answer over 1,000,000 bytes = STOP before it is used (MARKS answers: 20,000; VCH: 400,000). If CMP-A's answer fails this control, screenshot Tally, and if no dialog is up send CMP-B once; if BOTH fail the control the item ends as UNMEASURED: no backup, no restore; go to Close. The shape that passes at d0 is used for every later read; a later read that fails the control = STOP (after d5: "the book holds the seed only; D0 not restored").
  - Reading a CMP answer: flatten the COMPANY row to lines "path = text" for every attribute and every element with text, in document order with an index for repeated names (as compare.py on lab/1050-identity-answers does for a voucher). Keep each as cmp-N.flat.txt with its count of paths. A DIFF of two reads is the list of paths whose text differs or that exist in one only.

## Backups this item uses
- I0 (the seed only; 12 Oct): C:\bridge-lab\1050-B\backups\I0\TDBK1800_100056.001, 25,111 bytes, sha256 49ed631d851d7611c7bc2aa86bd7b403928c03adfa21a6fad84eb38ccd681470; in the Restore list it read "12-Oct-26 (3:43 pm)".
- D0: taken at d3 into the NEW folder C:\bridge-lab\1050-D\backups\D0\; record its time to the minute.

## Steps (LAB-COMMON start and step 0 first; every step ends with MARKS, VCH and its table, then the CMP read named)
- EXPECT AT EVERY STEP except d5: the VCH table is the d0 table row for row (2 vouchers). A miss = STOP. The two marks are RECORDED beside d0's at every step: they are part of what is measured, and a mark that differs after a backup, a reload or a restore is a result, not a STOP.
- d0 (as found): MARKS. VCH: KNOWN ANSWER, exactly 2 vouchers: the seed Receipt "1", 1 Jul 2026, MASTERID 1, ALTERID 1, GUID ending -00000001; and Receipt "2", 10 Jul 2026, 301.00, MASTERID 2, ALTERID 2, GUID ending -00000002. Anything else = STOP. CMP read 1 with its control.
- d1 (nothing done): wait at least 5 minutes after read 1, touching nothing, then CMP read 2. DIFF 1-2 = the NOISE set (values that move by themselves, such as a clock).
- d2 (a reload, no restore): shut THE COMPANY and load it again, alone. CMP read 3. DIFF 2-3 less NOISE = the RELOAD set.
- d3 (a backup, no restore): BACKUP D0 (C5). CMP read 4. DIFF 3-4 less NOISE = the BACKUP set.
- d4 (a restore of the SAME content): restore D0 over THE COMPANY (C6: the Replace prompt and the data-folder listings before and after are the proof that the restore reached the live folder; no Replace prompt = STOP). CMP read 5. DIFF 4-5 less NOISE, RELOAD and BACKUP = the RESTORE-SAME set.
- d5 (a restore of an OLDER backup): restore I0. MARKS: RECORD both marks. VCH: EXPECT exactly 1 voucher, the seed (else STOP; the way back is D0). CMP read 6. DIFF 5-6 less NOISE and RELOAD = the FOLLOWS-CONTENT set (it will hold the voucher mark and whatever else is copied from the backup: list all of it; these are not restore markers).
- d6 (back to the book as found): restore D0 again. MARKS: RECORD. VCH: the d0 table, and its answer equal to d0's under COLLECTION, compared by program; else STOP: "the book is not as found". CMP read 7. DIFF 5-7 less NOISE and RELOAD = the EVERY-RESTORE set (reads 5 and 7 are the same backup restored twice, two loads apart).
- Close (C10).
- THE RESULT is one table of the paths that are in ANY of the sets, no others: path; its text at reads 1 to 7; which of NOISE, RELOAD, BACKUP, RESTORE-SAME, FOLLOWS-CONTENT, EVERY-RESTORE it is in. Then the count of paths per read, and two lines: "A. Paths that changed at the restore of the same content and not by themselves, on a reload or at a backup (RESTORE-SAME): <list, or none among N paths>" and "B. Paths that differ between two restores of the same backup and not by themselves or on a reload (EVERY-RESTORE): <list, or none among N paths>".
- SAFE PLACES TO STOP: after d0, d1, d2, d3, d4, and after d6. NOT after d5: the book then holds the seed only; if a pause is called there, do d6 first. A STOP after d5 reports "the book holds the seed only; D0 not restored".

## Stop rules (beyond LAB-COMMON)
The known answer at d0 failing; the VCH table differing from d0's at any step but d5; a restore list that does not show the backup meant.
WHAT THE BOOK HOLDS IF THE ITEM STOPS (say which in the report): after d0 to d2, as found, no new backup; after d3 and d4, as found, and D0 exists; after d5, the seed only; after d6, as found; inside a restore, not known. The way back is D0 once it exists (before d3 the book is untouched), else I3 = C:\bridge-lab\1050-C\backups\I3\TDBK1800_100056.001 (26,492 bytes, sha256 ed0937c22c24300e7a878f4797dcde9af774cdc051bae920f1a59f4b3b665149), on the run coordinator's word, then Close.

## What it decides
- Whether a read of the company returns any value that changes when a backup is restored and at no other time. A path in line A or line B is a candidate only, to be measured again on another book before anything is built on it. "None" closes this way for the paths this shape returned, on this release with security off.
- Whether taking a backup or re-loading the company changes any company value: a control for every later reading of this object.
Not covered: a restore made on another machine or into a new folder; Tally with security on; Gold; other releases; what a company value does when the book is keyed after the restore.
PRIVACY (this overrides C11's "every answer in full" for the branch): a whole-company answer may carry licence, account or contact values (serial number, e-mail, user names). The raw CMP answers and flat files STAY on the lab PC, listed by hash. Only the result table goes to the evidence branch, and any such value in it is replaced by "<withheld>" with its path kept; the notes say which paths were withheld.
End state: BRIDGE IDENTITY LAB as found; one new backup D0 on the lab PC; Tally on 9001 with no company loaded.

Evidence branch: lab/1050-company-object-answers. Report after d1 (which shape passed, the size and path count of a CMP answer) and at the end, without waiting in between, or at a STOP.
