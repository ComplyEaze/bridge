First line: **Neither attempt changed the parent. c1: in Voucher Type Alteration of `IX EMPTY` (a type with no voucher), Return and Shift+Tab both passed over "Select type of voucher", as in Part 1's k8; the screen was left by Escape and Tally's "Quit ?" Yes. c2: the one All Masters Alter of `IX SALES` with `PARENT` Journal was answered ERRORS 1 with LINEERROR "Cannot change Type of Voucher!", every other counter 0, no dialog. Neither moved V or M; `IX SALES` read under Sales after c2, and its voucher's BRIDGEVCHISSALES did not change. A class change can be made by neither route on this release; the row stays open. The restore was checked. No stop, no skip.**

Sheet `lab/2162-class-sheet` at `8d6c7ab4` (row 2162-L2b), kit 9 of 9 OK by its `SHA256.txt`; `gen_c.py`'s two lines equal it (`evidence/s1-gen.txt`). Book BRIDGE BILLTRAIL LAB (synthetic), GUID `9ab5002c...` (the lab's own record of this book), loaded alone, at `bm0`'s state (V 7, M 212). TallyPrime 7.2 Silver (ProductVersion 1.1.7.2), Part 1's release. All times are UTC on 9 Nov 2026. Every number is read from its file; `evidence/notes.txt` has one line per step, `evidence/send-log.txt` every send with bytes, sha256 and curl's first-byte and total seconds, and `evidence/s3-check.txt`, `s5-check.txt` and `s6-check.txt` check_c.py's whole output.

The answers are on `lab/2162-class-answers`, the name the queue gives this item; the sheet's Keep section names `lab/2162-cx-answers`. The queue's name was used on the run lead's word.

## What it reports
| step | result |
|---|---|
| S0 | step 0 met (`s0` line in the notes); the one company loaded alone; current date Monday 1-Sep-2025. cm0: one COMPANY row, V 7, M 212 (EXPECT met) |
| S1 | kit 9 of 9 equal by sha256 and size; gen_c.py's lines for `vt-read.xml` and `c2-alter.xml` equal SHA256.txt's |
| S2 backup (C5) | `TDBK1800_<company number>.001`, 33,477 B, sha256 `6b355839ae1d45e859fca71d92dca79210ebc199c3d227ac2ab8290130de358e`, the only file in the new empty folder `C:\lab\2162-cx\backup-1`, written 06:23:36Z. cm0b byte-equal to cm0 (EXPECT met) |
| S3 type | `IX SALES` created under Sales, Manual numbering, duplicates prevented Yes. cm1: V 7, M 214 (M +2, V +0: EXPECT met) |
| S3 voucher | `IX-0003` keyed in IX SALES on 1-Oct-25: Lab Party P1, Lab Sales 100.00, bill New Ref IX-0003. cm2: V 8, M 214 (V +1: EXPECT met) |
| S3 reads | vt1 46,900 B, 25 VOUCHERTYPE rows; `IX SALES` PARENT Sales. y1 5,808 B, one VOUCHER row `IX-0003`, type IX SALES, BRIDGEVCHISSALES `Yes` (EXPECT met) |
| S4 create | `IX EMPTY` created under Sales, Manual numbering, duplicates at Tally's default No; no voucher keyed in it. cm3: V 8, M 216 |
| S4 c1 | Alter, Voucher Type, IX EMPTY: the alteration screen showed Select type of voucher Sales. Return from Name: the cursor to the alias line; Return: the cursor to Abbreviation, passing over "Select type of voucher". Shift+Tab from Abbreviation: back to the alias line, passing over it again. No mouse. The field was not reached, with no message from Tally. Escape: Tally's "Quit ?"; Yes. cm4 byte-equal to cm3 (EXPECT met). vt2 not sent |
| S5 c2 | cm5 byte-equal to cm4 (EXPECT met). `c2-alter.xml` sent once at 06:31:16.901Z, 938 B, sha256 `9486755e...`; answer 650 B in 0.065 s: CREATED 0, ALTERED 0, DELETED 0, COMBINED 0, IGNORED 0, ERRORS 1, CANCELLED 0, EXCEPTIONS 0, LASTVCHID 0, LASTMID 0, LINEERROR "Cannot change Type of Voucher!". The capture after the answer: the Gateway, no dialog, no prompt |
| S5 reads | cm6 byte-equal to cm5 (V 8, M 216). vt3 48,610 B, 26 rows: `IX SALES` PARENT Sales, ALTERID unchanged at 214; `IX EMPTY` PARENT Sales. `diff vt1 vt3`: IX EMPTY added (S4), 25 rows unchanged. y3: the row `IX-0003` as in y1, BRIDGEVCHISSALES `Yes`; y1 and y3 differ only in the CMPINFO count line VOUCHERTYPE (1 to 2), which is not a row (`evidence/s5-y1-y3-lines.txt`) |
| S6 restore (C6) | the plan written; the backup's sha256 checked; THE COMPANY shut; the list showed it with its number and 9-Nov-26 (11:53 am); "Accept ?" Yes; Tally's "Company data folder (<company number>) already exists ..." Replace; no other prompt. Loaded alone: the Gateway, current date Monday 1-Sep-2025 again, no dialog. cm7 byte-equal to cm0 (V 7, M 212). vt4 45,192 B, 24 rows, no IX SALES and no IX EMPTY. y4 no VOUCHER row and no `IX-` text; byte-equal to Part 1's read after its restore (`947fbf36...`). EXPECT met |
| S7 (C10) | Company, Shut; Tally at Select Company, none loaded; one tally.exe owning 9001; /status 200. No XML request after the shut |

Every request was answered in under 0.09 s by curl's total (`evidence/send-log.txt`).

## What it decides
**Neither changes the parent.** On this release a voucher type's class cannot be changed by either route tried: on the screen the parent field is not reachable even in a type that holds no voucher, and through the Gateway Tally refuses the Alter with "Cannot change Type of Voucher!" and changes nothing. Neither mark moved in either attempt. The row stays open for another release, as the sheet says.

## Observations
- vt1 and vt3 read `NUMBERINGMETHOD` as `None` for both IX types, though Manual was chosen on the creation screen each time. Recorded as received; the sheet asks nothing of it.
- F2 in the voucher screen (S3) set Tally's current date to 1-Oct-2025, as in Part 1; the restore returned it to 1-Sep-2025.
- check_c.py's `diff` matches rows by GUID and counted no VOUCHER rows in the Y0 answers, so y1 and y3 were compared by `vouchers` and by text (S5 line in the notes).
- The C10 notes line first said its capture equalled row 1381-R1's C10 capture; it does not (`7ba902a0...`). The capture was looked at (Select Company, no company loaded) and the line corrected in place, with the first wording recorded in it.

## Kept on the lab PC
The marks, voucher-type and Y0 reads hold the company's GUID or number; they are listed by bytes and sha256 in `kept-local.txt`, as the sheet's Keep section says while the GUID is not in the synthetic-uuid registry. Also listed there: the backup, the restore plan, the data-folder listings, the process and port listings and this row's stage script. The import answer (c2) carries neither and is here whole, and every answer's headers are here. The screenshots stay on the lab PC and are listed in `SCREENSHOTS.txt`.

## Limits
One sitting, one book, one release, one attempt by each route. c1 was tried with Return and Shift+Tab only; the sheet forbids the mouse.
