# Windows item 1038-R v1.2: what an IMPORTED voucher takes after a backup is restored, and in which order a batch is numbered (13 Oct 2026; for issue #1038, and it adds to #1050). Plain HTTP plus Tally's own Backup and Restore screens: NO ComplyEaze Bridge build, no Terms entry. SYNTHETIC ONLY: THE COMPANY = BRIDGE IDENTITY LAB (number 100056; its GUID is the one in the answers on branch lab/1050-identity-answers). LAB-COMMON v3 applies in full. Writes: two raw imports (3 Receipts in all), each file sent ONCE; two restores by the operator. It runs before item 1050-D or after it, never inside it. Needs the run coordinator's go-ahead naming the two import hashes.

Why: #1038 would re-post, by import and with new REMOTEIDs, rows whose vouchers a restore removed. On 12 Oct a voucher keyed BY HAND after a restore took the lost voucher's MasterID, AlterID, GUID and number (twice). Whether an IMPORTED voucher does the same is not measured, and it decides the design: if it does, the re-posted voucher answers to the GUID the old batch is bound to. One act per step.

## Requests (each ONE line written as UTF-16LE with a BOM, no line break at the end)
- MARKS: 982 bytes, sha256 2ce259da099e9d3664ed432e1d5a1c532cc241a1a562e81cd9f3f20ba069ceae (the marks read of the earlier items, only the company name changed).
  File on this branch: requests/1038-R/marks.utf16le.xml
- VCH: lab/1050-identity-requests at e0d61b813704129894b7c8533e3449d5b49006cf, file b1-window.xml, UNCHANGED: 1,782 bytes, sha256 aba85b51370f86aadc6c1c81697fe670a488ce7e03067abd61a13acfee34a439 (every voucher of 1 to 31 Jul 2026; used three times on this company on 12 Oct). The same file is on this branch as requests/1038-R/b1-window.xml: check its hash, whichever copy is used. MARKS answers are bounded at 20,000 bytes. About 32 KB of answer per voucher; an answer over 400,000 bytes = STOP.
- Two IMPORT files, in the shape ComplyEaze Bridge's own builder sends (the envelope and Receipt of the committed fixture post-span-import.xml, sent raw on 2 Oct): no [BRIDGE:...] tag in the narration, no voucher number, a new REMOTEID on each voucher.
  - IMP1, one Receipt A (10 Jul 2026, 301.00, "Lab identity probe 1": the content of the voucher posted on 12 Oct): 2,006 bytes, sha256 a05176369a4b63a97ac77ed47730b8c7d60e736020c18cc7670c79ef1c019973 (UTF-8: 1,002 bytes, 4389fa9102d059badfbddd0deae22122fbb5993660f1e1de9b977a4217d0ec39)
    File on this branch: requests/1038-R/imp1.utf16le.xml
  - IMP2, two Receipts with the LATER date FIRST in the file: C (12 Jul 2026, 303.00, "1038-R probe C"), then B (11 Jul 2026, 302.00, "1038-R probe B"): 3,322 bytes, sha256 4aeccf3812ee1da1aca2444405e1bc2d2f5c043d30e584a1e00fd9ceaee9b9dc (UTF-8: 1,660 bytes, 4d06e7a33d34ab2fa93e35872f1e599db42414b65ec0097cec5143a07ac9aa43)
    File on this branch: requests/1038-R/imp2.utf16le.xml
  Each Receipt is Dr Synthetic Bank Ledger / Cr Party A.

## Backups this item uses
- I0 (the seed only; 12 Oct): C:\bridge-lab\1050-B\backups\I0\TDBK1800_100056.001, 25,111 bytes, sha256 49ed631d851d7611c7bc2aa86bd7b403928c03adfa21a6fad84eb38ccd681470; in the Restore list it read "12-Oct-26 (3:43 pm)".
- Z0: taken at r0 into the NEW folder C:\bridge-lab\1038-R\backups\Z0\; record its local time to the minute, since the Restore list at End must show it.
- If Z0 cannot be restored at End, the run coordinator's fallback is I3 = C:\bridge-lab\1050-C\backups\I3\TDBK1800_100056.001, 26,492 bytes, sha256 ed0937c22c24300e7a878f4797dcde9af774cdc051bae920f1a59f4b3b665149 (the same book as found). It is restored only on the run coordinator's word.

## Steps (every step ends with MARKS, then VCH and its table)
- EXPECT AT EVERY STEP (a miss is a STOP, never a result): the MARKS answer has one COMPANY row with this GUID; the VCH answer parses and holds the count given for the step; the seed row reads Receipt "1", 1 Jul 2026, MASTERID 1, ALTERID 1, GUID ending -00000001, Lab Seed Income 1000.00 / Cash -1000.00; every voucher is a Receipt dated in July 2026; each voucher this item imported carries the date, narration, ledgers and amounts of its file.
- r0 (as found): LAB-COMMON start and step 0 (its MARKS read is this step's first). VCH: KNOWN ANSWER, exactly 2 vouchers: the seed, and Receipt "2" of 10 Jul 2026, MASTERID 2, ALTERID 2, GUID ending -00000002, "Lab identity probe 1", Party A 301.00 then Synthetic Bank Ledger -301.00 (the hand-keyed voucher of 12 Oct). Anything else = STOP: nothing is backed up or restored. Keep the answer file whole: End compares against it. BACKUP Z0 (C5).
- r1 (the restore): restore I0 (C6). MARKS: RECORD ALTVCHID and ALTMSTID (1 and 211 were read on this company in this state on 12 Oct); the marks are measured here, not required. VCH: EXPECT exactly 1 voucher, the seed; anything else = STOP (the way back: Z0, else I3).
- r2 (ONE imported voucher after a restore): send IMP1. EXPECT CREATED 1 (C4). MARKS: RECORD both marks. VCH: EXPECT 2 vouchers. RECORD for A: VOUCHERNUMBER, MASTERID, ALTERID, GUID. QUESTION 1: does A read "2", 2, 2, GUID ending -00000002, what the posted voucher and the hand-keyed one each held? One line, yes or no per field. Either answer is a result.
- r3 (a batch whose file order is the reverse of its date order): send IMP2. EXPECT CREATED 2. MARKS. VCH: EXPECT 4 vouchers; the seed and A as at r2. RECORD for C and for B: number, MASTERID, ALTERID, GUID. QUESTION 2: which of the two took the LOWER MasterID and AlterID, C (first in the file) or B (earlier date)? One line: "numbered in file order" or "numbered in date order" or what was seen.
- End (put the book back): re-check Z0's hash and that its folder holds that one file; restore Z0 (C6). MARKS: RECORD both marks beside r0's. VCH: EXPECT 2 vouchers and, compared by program with r0's answer file, everything under COLLECTION equal (the count lines under CMPINFO are not compared). Else STOP and report "the book is not as found". Then Close (C10).
- SAFE PLACES TO STOP: after r0, and after any step's reads are complete (r1 to r3 leave a book that only this item uses). The End step is owed before any other item uses this company. Report after r2 and at the end, without waiting in between.

## Stop rules (beyond LAB-COMMON)
The known answer at r0 failing; the seed row changing; a voucher this item did not import appearing; a voucher read after a restore that is not what the step says.
WHAT THE BOOK HOLDS IF THE ITEM STOPS (say which in the report): after r0, as found, and Z0 exists; after r1, the seed only; after r2, the seed and A; after r3, the seed, A, C and B; inside a restore, not known: say "restore of <backup> did not complete". The way back in every case is Z0, else I3, on the run coordinator's word, then Close.

## What it decides
- QUESTION 1: whether a voucher that arrives by import after a restore takes the lost voucher's MasterID, AlterID, GUID and number, as a hand-keyed one did. If yes: #1038's re-post cannot leave the old batch's binding in place (both batches would name one GUID), and "re-posted" cannot be told from "keyed again" by any identifier. If no: say what it took instead; the design question changes.
- QUESTION 2: whether a batch is numbered in the order of the file or of its dates. The 2 Oct capture had ten vouchers of one date, so it could not tell. With question 1 it says which row of a re-posted batch inherits which lost identity, and it is the ground ComplyEaze Bridge's position check stands on.
- What the two marks read after a restore that removes vouchers only.
Not covered: ComplyEaze Bridge's own verify_import over these states (a later item does that through the product); a REMOTEID sent a second time after a restore (never sent here); other voucher types; a restore that also removes masters; security on; Gold; other releases.
End state: BRIDGE IDENTITY LAB as found (the seed and the hand-keyed Receipt "2"); one new backup Z0 on the lab PC; Tally on 9001 with no company loaded.

Evidence branch: lab/1038-restore-import-answers.
