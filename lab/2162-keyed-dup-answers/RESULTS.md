# Row 2162-KD answers: a Sales voucher type keyed on screen today, Manual with Prevent duplicates Yes

**Decides: the flag holds on a second Sales type, keyed in this sitting, in the answer. PARTIAL (one book, one release; with S4, two types and two sittings).** Keying "Prevent creating duplicate Voucher Nos." Yes under Manual on Tally's screen gave the series `PREVENTDUPLICATES` Yes that the build reads (`s3r`). The fresh number `BP/26-27/0061` was stored as sent, on `BRIDGE Sales Keyed` (`s4`, `s4r`). The same number sent again was declined in the answer, CREATED 0 and EXCEPTIONS 1 with no LINEERROR, and no dialog came up (`s5`, `s5r`). Every EXPECT line was met.

Two things the sheet does not name, recorded as found:
- **Field label.** This release labels the field "Prevent creating duplicate Voucher Nos.", not "Prevent duplicates". It appeared under Method of Voucher Numbering once Manual was chosen, showing No, and was set to Yes.
- **The save.** Ctrl+A saved the voucher type with no `Accept ?` prompt and opened a new blank Voucher Type Creation screen. Two Ctrl+Q presses, each after a capture of the screen's title, went back to the Gateway by way of the Master Creation list.

Sheet `lab/2162-keyed-dup-sheet` at c5b8e06e, run 9 Nov 2026, 10:46Z to 10:56Z, on `BRIDGE PILOT LAB` (synthetic), TallyPrime 7.2 Silver (tally.exe 1.1.7.2). The Gateway's current date was Tuesday 15-Sep-2026. Every number below is read from its file in this folder. The book was restored from the step 1 backup. `s9`'s marks are byte-equal to `s1`'s, and its T is byte-equal to `s2`'s.

| step | result |
|---|---|
| 1. Load, `s1`, BK | Loaded alone; period 1-Apr-26 to 31-Mar-27, current date 15-Sep-2026 (not before 1-Aug-2026). M: one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245. BK by C5 to a new empty folder: `TDBK1800_<company number>.001`, 90051 B, sha256 949b20d376b0a4fdda40dd434182bd387b4dc8cbdc5966b6de718ee0a5fe1662, 10:49:12Z. `s1b` equal to `s1`. |
| 2. The book as found | T: 28 VOUCHERTYPE rows; `BRIDGE Sales` 1 (PARENT `Sales`, NUMBERINGMETHOD `None`, one series `Default`, `Manual`, `Yes`); `BRIDGE Sales Keyed` 0. N 0 rows, V 0 rows. EXPECT met. The committed answer the sheet counted held 25 rows; this book holds 28. |
| 3. The type on screen | `s3m` passed `count6.py`'s before-write check. Create > Voucher Type. Name `BRIDGE Sales Keyed`; Select type of voucher `Sales`; Method of Voucher Numbering `Manual`; Prevent creating duplicate Voucher Nos. `Yes`. Every other field was left as Tally filled it, including Abbreviation `Sale` (`evidence/s3-screens.txt`). Saved by Ctrl+A with no prompt (above). `s3r`: T 29 rows; `BRIDGE Sales Keyed` 1, PARENT `Sales`, NUMBERINGMETHOD `None`, ISACTIVE `Yes`, one series `Default`, NUMBERINGMETHOD `Manual`, PREVENTDUPLICATES `Yes`; `BRIDGE Sales` as at `s2`. M: ALTVCHID 68, ALTMSTID 247. EXPECT met. |
| 4. The fresh number | `s4m` passed. `s4` `I-0061.xml`: CREATED 1, LASTVCHID 45, every other counter 0, no LINEERROR; the Gateway, no dialog. `s4r`: N one row, `BP/26-27/0061`, BRIDGEVCHISSALES `Yes`; V one row, VOUCHERNUMBER `BP/26-27/0061`, VOUCHERTYPENAME `BRIDGE Sales Keyed`, PARTYLEDGERNAME `BRIDGE Walk-in`, DATE 20260801, MASTERID 45; M 69, 247. EXPECT met: the number and the type were stored as sent. |
| 5. The same number again | `s5m` passed. `s5` `I-0061b.xml`: CREATED 0, EXCEPTIONS 1, every other counter 0 (LASTVCHID 0), no LINEERROR; the Gateway, no dialog. `s5r`: N and V one row each, the voucher of step 4, VOUCHER rows equal to `s4r`'s by parse; M equal to `s4r`'s (69, 247). EXPECT met. |
| 6. Restore (C6) | Company > Shut; Restore of `backup-1`, listed as 9-Nov-26 (4:19 pm); `Accept ?` y; `Company data folder (<company number>) already exists ...` r (Replace); no other prompt. Loaded alone, 15-Sep-2026, no dialog. `s9`: M byte-equal to `s1`'s (68, 245), `--same-row` equal; T byte-equal to `s2`'s, `BRIDGE Sales Keyed` 0. EXPECT met. |
| 7. Close (C10) | Company > Shut: Select Company, none loaded; one tally.exe owning 9001; `GET /status` 200. Every read was answered (20 sends, curl exit 0 each). |

## Recorded, not judged
- The `s5r` N and V answers differ from `s4r`'s in three CMPINFO count lines only: LEDGER 11 to 22, TAXUNIT 1 to 2, VOUCHERNUMBERSERIES 1 to 2. LAB-COMMON C4 says those lines are not counts of anything read. The VOUCHER rows are equal (`evidence/s5r-vs-s4r.txt`).
- ALTMSTID rose by 2, not 1, for the one voucher type created (245 to 247).
- The Gateway shows `(e) (Schedule Backup)` beside the company name and `(e) Data exceptions exist` below it, before and after the run. Neither is a dialog.

## Not measured
A number held by another Sales type sent to this one; a number that differs by case or a leading zero; a type with more than one series; a book with no invoice; an invoice built and posted through Bridge; any other book or release.

## Files
- `answers/`: every answer of the row, byte for byte, where it holds no company number, no GSTIN shape and no long digit run outside a VOUCHERKEY value; any other is kept on the lab PC (`kept-local.txt`). `evidence/answers.txt` lists all 20 by sha256 and bytes.
- `evidence/`: the notes (one line per step, UTC), each step's `count6.py` output, the before-write and same-row checks, the step 3 screen as text, the `s5r` comparison and the sends' times.
- `logs/`: `run.py`'s logs. `scripts/`: `step0.py`, `shot.sh`. `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
