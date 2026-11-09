# Row 2162-HS: GSTR-1's HSN summary gives each sales line of a two-line invoice to its own ledger's code; each code's line rose by exactly that code's sales line

Run on 9 Nov 2026, 20:11:56Z to 20:29:12Z (01:41 to 01:59 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-hs-sheet` (b6767b87), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw requests only, one at a time through `run.py`; one import, and no Bridge build. GSTR-1 was read on Tally's screen only, and nothing was keyed that saves. The book was then restored from the row's own backup. Every value below is M, measured by this run, in one run: **partial** (one invoice, one sitting, one book, one release, a screen read). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran after row 2162-GF (Tally quit at 20:06:45Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The book was restored after step 6, so the marks it leaves are those it found (68/245).

The one GSTIN in this folder is the synthetic family's, as the sheet names it: the company's own `08ZZZZZ0000Z1ZQ`.

**Result.**
- **s2, before:** N-0072 0 rows (no voucher holds the number). R-0010 `BRIDGE Walk-in`, included Yes, every other status flag No, six legs, byte-equal to row 2162-GF's s2 R-0010. M 68/245. Every EXPECT met.
- **Step 3, the HSN summary before:** GSTR-1 for 2026-08 (1-Aug-26 to 31-Aug-26, no period key pressed), with no list of registrations offered. Total Vouchers 39: Included 27, Not Relevant 7, Uncertain 5; B2C (Small) 27 (2). `HSN Summary - 12 (B2B - B2C Supplies)` opened one list, headed B2C Supplies, with nine lines. It is row 2162-M1's list exactly: 998313 at 11,137.40 / 10,607.00 / 265.21 / 265.19 / 530.40, 998314 at 1,366.70 / 1,301.60 / 32.55 / 32.55 / 65.10, and Sub Total and Total 14,275.20 / 13,508.70 / 383.26 / 383.24 / 766.50.
- **Step 3.5 (NEW PATH): Enter on a code's line opens a list of vouchers.** Its title is "GSTR-1 - Voucher Register", headed "Vouchers of <code>". Under 998313: 19 lines, `BP/26-27/0010` among them at total 630.42, taxable 600.40, CGST 15.01, SGST 15.01. Under 998314: 7 lines, `BP/26-27/0010` among them at 420.42 / 400.40 / 10.01 / 10.01. Each list's Total equals its code's line. So before the import, `0010` already shows under both codes, each with its own sales line.
- **Step 4, the import:** `I-0072.xml` sent once, after the before-write check passed (68/245, the book's GUID). CREATED 1, every other count 0, LASTVCHID 45, no LINEERROR. The read-back: `BP/26-27/0072` on 2026-08-01, `BRIDGE Walk-in`, the six legs as sent (-1,051.00; 600.40 on `BRIDGE Svc 998313 5%`; 400.40 on `BRIDGE Svc 998314 5%`; 25.02; 25.02; 0.16 on `BRIDGE Round Off`), included Yes, uncertain and excluded No, GUID the book's plus `-0000002d`, ALTERID 69. M 69/245. Against `0010`'s read-back, only ALTERID, GUID, MASTERID, REFERENCE, VOUCHERKEY, VOUCHERNUMBER and VOUCHERRETAINKEY differ; legs 6 -> 6.
- **Step 5, the HSN summary after:** Total Vouchers 40: Included 28, Not Relevant 7, Uncertain 5; B2C (Small) 28 (2). The same list, the same nine lines, no new line. **998313 rose by exactly 630.42 / 600.40 / 15.01 / 15.01 / 30.02, and 998314 by exactly 420.42 / 400.40 / 10.01 / 10.01 / 20.02.** Each is that code's own sales line of `0072`. The seven other lines are unchanged. The Total rose by 1,050.84 / 1,000.80 / 25.02 / 25.02 / 50.04. The invoice is 1,051.00, so the 0.16 round off is on no line.
- **Step 5.5, the lists of vouchers after:** under 998313, 20 lines: the 19 of step 3.5, unchanged, and `BP/26-27/0072` after `0010` at 630.42 / 600.40 / 15.01 / 15.01. Under 998314, 8 lines: the 7 of step 3.5, unchanged, and `0072` after `0010` at 420.42 / 400.40 / 10.01 / 10.01. Each list's Total equals its code's line after.
- **s6, after the look:** M 69/245 and R-0072 are each byte-equal to s4r's. The look changed neither.
- **The restore, then s7:** M is byte-equal to s1's (68/245). N-0072 has 0 rows and is byte-equal to s2's. R-0010 shows `lists compared: 1` and `legs: 6 -> 6` only, and is byte-equal to s2's (94,474 B). Every step 7 EXPECT met.

**The difference, per HSN line** (step 3.4 before, step 5.4 after, read off the screen; amounts in rupees). Every line that holds anything in either reading is here.

| HSN/SAC | | Total amount | Taxable | CGST | SGST/UTGST | Tax amount |
|---|---|--:|--:|--:|--:|--:|
| 847130 | before | 826.00 | 700.00 | 63.00 | 63.00 | 126.00 |
|  | after | 826.00 | 700.00 | 63.00 | 63.00 | 126.00 |
|  | after minus before | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| 9983 | before | 105.00 | 100.00 | 2.50 | 2.50 | 5.00 |
|  | after | 105.00 | 100.00 | 2.50 | 2.50 | 5.00 |
|  | after minus before | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| 998313 | before | 11,137.40 | 10,607.00 | 265.21 | 265.19 | 530.40 |
|  | after | 11,767.82 | 11,207.40 | 280.22 | 280.20 | 560.42 |
|  | after minus before | 630.42 | 600.40 | 15.01 | 15.01 | 30.02 |
| 998314 | before | 1,366.70 | 1,301.60 | 32.55 | 32.55 | 65.10 |
|  | after | 1,787.12 | 1,702.00 | 42.56 | 42.56 | 85.12 |
|  | after minus before | 420.42 | 400.40 | 10.01 | 10.01 | 20.02 |
| 998315 | before | 105.10 | 100.10 | 2.50 | 2.50 | 5.00 |
|  | after | 105.10 | 100.10 | 2.50 | 2.50 | 5.00 |
|  | after minus before | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| 998341 | before | 210.00 | 200.00 | 5.00 | 5.00 | 10.00 |
|  | after | 210.00 | 200.00 | 5.00 | 5.00 | 10.00 |
|  | after minus before | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| 998342 | before | 210.00 | 200.00 | 5.00 | 5.00 | 10.00 |
|  | after | 210.00 | 200.00 | 5.00 | 5.00 | 10.00 |
|  | after minus before | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| 998343 | before | 105.00 | 100.00 | 2.50 | 2.50 | 5.00 |
|  | after | 105.00 | 100.00 | 2.50 | 2.50 | 5.00 |
|  | after minus before | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| 998344 | before | 210.00 | 200.00 | 5.00 | 5.00 | 10.00 |
|  | after | 210.00 | 200.00 | 5.00 | 5.00 | 10.00 |
|  | after minus before | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Sub Total | before | 14,275.20 | 13,508.70 | 383.26 | 383.24 | 766.50 |
|  | after | 15,326.04 | 14,509.50 | 408.28 | 408.26 | 816.54 |
|  | after minus before | 1,050.84 | 1,000.80 | 25.02 | 25.02 | 50.04 |
| Total | before | 14,275.20 | 13,508.70 | 383.26 | 383.24 | 766.50 |
|  | after | 15,326.04 | 14,509.50 | 408.28 | 408.26 | 816.54 |
|  | after minus before | 1,050.84 | 1,000.80 | 25.02 | 25.02 | 50.04 |

The nine lines add up to the Total in both readings. The differences were taken by a script from the two screens as written down, not by hand.

**By the sheet's Decides:**
- **Each code's line rises by exactly that code's sales line:** measured. 600.40 went under 998313 and 400.40 under 998314, with each line's own tax. Tally gives each sales line of a two-line invoice to its own ledger's code. Item 10's split is measured: PARTIAL (one invoice, one book, one release, a screen read).
- **The whole 1,000.80 on one code:** not seen.
- **Any other difference:** none. No line changed that `0072` does not use, and no line was added.
- **Step 3.5 opens a list of vouchers:** measured. The share is read per invoice as well. `0010` (before and after) and `0072` (after) each show 600.40 taxable under 998313 and 400.40 under 998314, each with its own total and tax. This is the stronger evidence, and it matches the difference.
- **The round off on an HSN line:** not seen. The 0.16 of `0072` is on no line, and neither invoice's line in either list includes it (630.42 + 420.42 = 1,050.84, against 1,051.00 invoiced). The month's HSN Total leaves out round offs, as the sheet estimated.

**A finding on R-0010's bytes (the run lead's word: the voucher row met the EXPECT; record the counters apart).**
- At s4r, R-0010's voucher row was unchanged: `--diff` against s2 showed `lists compared: 1` and `legs: 6 -> 6` and nothing more. The answer, though, was 94,476 B against s2's 94,474 B.
- The only bytes that differ are in the answer's header, `BODY/DESC/CMPINFO`. Five counters moved: LEDGER 0 -> 11, CURRENCY 0 -> 1, TAXUNIT 0 -> 1, VOUCHERNUMBERSERIES 0 -> 1 and VOUCHER 0 -> 2. Every other counter stayed 0.
- At s7, after the restore, every counter is 0 again and the answer is byte-equal to s2's. The whole table, s2 / s4r / s7, is in `evidence/cmpinfo-s2-s4r-s7-R-0010.txt`.
- What the counters count was not measured. They moved between s2 and s4r, a span with one import and the screen reads of step 3 in it.

## Step 1: load, check and back up
- Sheet files: all 13 equal to `SHA256.txt` at b6767b87. `gen.py` was run on copies in a folder of its own (not staged), and `I-0072.xml`, `N-0072.xml` and `R-0072.xml` came out equal to `SHA256.txt`. `count15.py` is byte-equal to row 2162-GF's.
- C2 step 0 with no `tally.exe` running. Tally was started once, at 20:11:56Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2). Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt).
- C2 at 20:12:51Z: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID, and not the excluded GUID. No Bridge process was ended and no Bridge tool was called (a standing rule of the lab).
- The book was loaded alone from Select Company: the name typed, one row, Enter, no prompt. The Gateway showed (`s1-gateway`):
  - BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title;
  - current period 1-Apr-26 to 31-Mar-27;
  - current date Tuesday 15-Sep-2026;
  - date of last entry 15-Sep-26.
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5):
  - Before the backup, `C:\lab\2162-hs\backup-1` was new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`, not cloud storage. Backspace moved it back to Backup Data Path.
  - Specify Path (Up 3 times), then `C:\lab\2162-hs\backup-1` typed. Tally shows it as `C:\lab\2162-Hs\backup-1`; Windows treats the two spellings as the same folder. Its row was taken.
  - BRIDGE PILOT LAB alone, End of List. "Accept ?" was captured (`s1-bk-accept`, 20:14:44Z) and answered y; no other message.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `586708162136ab2821ea8ecaebb8af1875925ce33810bd558194d4425522ed73`, written at 01:44 IST. The Restore list showed it as 10-Nov-26 (1:44 am).

## Step 2: the book before
`s2`: N-0072, R-0010 and M, as in the Result. Every EXPECT met. The outputs are below, whole.

## Step 3: the HSN summary before, read only
Every screen, line and key is in `evidence/s3-screens.txt`, word for word. In short:
1. At the Gateway (`s3-gateway`), the G: Go To label clicked, GSTR-1 typed, Down once to the GSTR-1 line (zoomed), Enter. No list of registrations; the report opened at once.
2. "GSTR-1", BRIDGE PILOT LAB, 1-Aug-26 to 31-Aug-26, registration `08ZZZZZ0000Z1ZQ`, Status Not Filed (`s3-gstr1`).
3. The counts, as in the Result.
4. Down 24 times, one line at a time (10, 12, 1 and 1, captured between), to `HSN Summary - 12 (B2B - B2C Supplies)` (zoomed), Enter. Row 2162-M1 used this path on this book. One list, no choice offered: "GSTR-1 - HSN/SAC Summary (B2B - B2C Supplies)", heading B2C Supplies, nine lines, each with its Description line (`s3-hsn`).
5. **NEW PATH.** Down 2 times to 998313 (the Description lines are passed over; zoomed), Enter: "Vouchers of 998313", 19 lines (`s3-vch-998313`). Its button bar offers Enter: Alter, so no key but Esc was pressed on it. Esc once to the HSN list, Down once to 998314 (zoomed), Enter: "Vouchers of 998314", 7 lines (`s3-vch-998314`). Every line of both lists is in `evidence/s3-screens.txt`.
6. Esc 3 times: the HSN list, GSTR-1, the Gateway (`s3-gateway-after`). No prompt at any point.

Keys pressed in step 3, in all: the G: Go To label; GSTR-1 typed; Down; Enter; Down 24 times; Enter; Down 2 times; Enter; Esc; Down; Enter; Esc 3 times.

## Step 4: the import
`s4m`, then the before-write check against s1: one COMPANY row, the same GUID (68/245); passed. No key was pressed in Tally between it and the send. `s4`: `I-0072.xml` sent once; the window after showed the Gateway with no dialog (`s4-after`). `s4r`: N-0072, R-0072, R-0010 and M, as in the Result. The outputs are below, whole.

## Step 5: the HSN summary after, read only
Step 3 again, with the same keys, all in `evidence/s5-screens.txt`. The counts, the whole HSN list and both lists of vouchers are as in the Result. The seven other HSN lines and every Description line read as at step 3, word for word. Every line of the lists of vouchers that was there at step 3 is there again, unchanged; the one new line in each is `0072`'s.

Keys pressed in step 5, in all: the G: Go To label; GSTR-1 typed; Down; Enter; Down 24 times; Enter; Down 2 times; Enter; Esc; Down; Enter; Esc 3 times. No Enter in either list of vouchers.

## Step 6: the reads after the look
`s6`: M and R-0072. `--same-row` against s4r: equal (69/245). R-0072 against s4r: `lists compared: 1` and `legs: 6 -> 6` only. Both answers are byte-equal to s4r's.

## Step 7: restore (C6), then the book after
- **Before the shut:**
  - BK re-hashed, equal to step 1's, its folder holding only that file.
  - `restore-plan.txt` written.
  - The data folder listed: 24 entries.
- **The restore:**
  - Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt.
  - The Data menu (by its label), Backup & Restore, Restore (Down 2 times). Restore Data Path read Tally's data folder.
  - Specify Path (Up 3 times) to the BK's folder. The list showed BRIDGE PILOT LAB, its number and 10-Nov-26 (1:44 am), the BK's time.
  - The company line, then End of List.
- **Two prompts, the only ones answered, each captured first:**
  - "Accept ?": y (`s7-restore-accept`).
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", with R Replace and S Skip: r (`s7-restore-replace`).
- **After the restore:**
  - Select Company came back with no other message.
  - The data folder held 17 entries. Its `Company.1800` held the book's GUID and name, and not the excluded GUID.
  - The book was loaded alone (one row, Enter, no prompt). The Gateway showed the book as the one company, with the period and date as before and no dialog (`s7-gateway`).
- **`s7`:** as in the Result. Each answer's sha256 equals s1's or s2's (`evidence/s2-s7-sha.txt`).

## Step 8: close (C10)
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`).
- At 20:28:18Z: one `tally.exe`, which alone owns 9001, and `GET /status` 200. No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **R-0010's header counters** moved at s4r while its voucher row did not, as in the Result. The run lead was told at s4r and said to go on, with the voucher row as the EXPECT and the counters recorded apart.
- **The Tally window came out of maximize once,** at step 7. Two clicks on its title bar in quick succession (the focus click before each key) read as a double-click, seen on the Data menu. One double-click on the title bar maximized it again. No key reached Tally in between, and the Restore line stayed highlighted.
- **At step 7's load,** the name went in key by key and its spaces did not register: the field read `BRIDGEPILOTLAB`. The list still held one row, BRIDGE PILOT LAB, highlighted (`s7-select-company-desktop`), and Enter loaded it.
- **Two records in the notes were set right from the tool's own key record.** The step 3 key line first listed four Escs at its end; the record shows three, as 3.6 says. A C9 line had a placeholder time; it is now 20:12:39Z. Each change has a line of its own in `evidence/notes.txt`. Also, at step 3.5 the Particulars of `BP/26-27/0019` were first written as `BRIDGE Walk-in`; both window captures read `Bridge Walk-in`, and the text was corrected at step 5.5.
- **Three invoices of the month split tax unevenly between codes or heads,** as found at both readings and unchanged by this row. `BP/26-27/0016` shows 105.20 (CGST 2.50) under 998313 and 105.22 (CGST 2.51) under 998314, each on 100.20 taxable. `BP/26-27/0043` and `0044` each show CGST 25.03 and SGST 25.02 under 998313. Neither was asked about here.
- **Captures.** `shot.sh`'s capture reaches only the window's left part, so the desktop tool's own captures of each list (`*-desktop.jpg`) and of each highlighted line (`*-zoom.jpg`) were copied into `shots/` too.
- **Go To and the menus** were opened by clicking their labels (G: Go To, K: Company, Y: Data), as the lab's standing rule asks. Before each key, Tally's own title bar was clicked (its empty part) so that the key reached Tally.
- **The Gateway's right-hand panel** showed a red line "N: Manage Rental License" at every Gateway of this sitting. It was not pressed.
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.

## Not measured
Every item the sheet lists as not measured. Also not measured:
- What R-0010's header counters count, and which act moved them.
- The second part of the lists of vouchers (a "2" mark at their top right says the columns go on); it was not opened.
- Whether a Bridge build's own post of such an invoice files the same way (this was the raw request).

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC. Those are listed by size and sha256 in `kept-local.txt`, with the reason.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), each count, the before-write, `--same-row` and `--diff` outputs, the send times, the screens written down, the BK record, the s2/s7 hash pairs and the R-0010 counter table. The process, port and data-folder listings and the restore plan stay local.
- `scripts/`: the step 0 helper, the window capture helper, the send helper and the C6 prep helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## count15.py outputs, whole

### s1
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s2
```
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s2-N-0072.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-R-0010.xml: 94474 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s4m
```
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s4
```
== a-s4-I-0072.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '45'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### s4r
```
== a-s4r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4r-N-0072.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0072' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s4r-R-0010.xml: 94476 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s4r-R-0072.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0072'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0072'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s6
```
== a-s6-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6-R-0072.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0072'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0072'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s7
```
== a-s7-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7-N-0072.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s7-R-0010.xml: 94474 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

## Checks and diffs, whole

### s4m against s1 (before-write)
```
$ python -I count15.py <the before-write option> a-s1-M-marks.xml a-s4m-M-marks.xml
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### R-0010, s2 against s4r
```
$ python -I count15.py --diff a-s2-R-0010.xml a-s4r-R-0010.xml
== --diff a-s2-R-0010.xml a-s4r-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### R-0010 against R-0072, both at s4r
```
$ python -I count15.py --diff a-s4r-R-0010.xml a-s4r-R-0072.xml
== --diff a-s4r-R-0010.xml a-s4r-R-0072.xml
   ALTERID: '17' -> '69'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0072'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0072'
   VOUCHERRETAINKEY: '41' -> '129'
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### M, s6 against s4r (--same-row)
```
$ python -I count15.py --same-row a-s4r-M-marks.xml a-s6-M-marks.xml
== a-s6-M-marks.xml against a-s4r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'245'")
  equal
exit 0
```

### R-0072, s4r against s6
```
$ python -I count15.py --diff a-s4r-R-0072.xml a-s6-R-0072.xml
== --diff a-s4r-R-0072.xml a-s6-R-0072.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### M, s7 against s1 (--same-row)
```
$ python -I count15.py --same-row a-s1-M-marks.xml a-s7-M-marks.xml
== a-s7-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### R-0010, s2 against s7
```
$ python -I count15.py --diff a-s2-R-0010.xml a-s7-R-0010.xml
== --diff a-s2-R-0010.xml a-s7-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### sha256, s1 or s2 then s7, per answer
```
sha256 pairs, s2 or s1 against s7 (sha256sum)
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 *a-s1-M-marks.xml
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 *a-s7-M-marks.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0072.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s7-N-0072.xml
b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 *a-s2-R-0010.xml
b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 *a-s7-R-0010.xml
```

### R-0010 header counters, s2 / s4r / s7
```
$ python -I (inline) BODY/DESC/CMPINFO counters of R-0010 at s2, s4r and s7 (answers decoded UTF-16-LE)
counter | a-s2-R-0010.xml | a-s4r-R-0010.xml | a-s7-R-0010.xml
COMPANY | 0 | 0 | 0
GROUP | 0 | 0 | 0
LEDGER | 0 | 11 | 0
COSTCATEGORY | 0 | 0 | 0
COSTCENTRE | 0 | 0 | 0
GODOWN | 0 | 0 | 0
STOCKGROUP | 0 | 0 | 0
STOCKCATEGORY | 0 | 0 | 0
STOCKITEM | 0 | 0 | 0
VOUCHERTYPE | 0 | 0 | 0
CURRENCY | 0 | 1 | 0
UNIT | 0 | 0 | 0
BUDGET | 0 | 0 | 0
CLIENTRULE | 0 | 0 | 0
SERVERRULE | 0 | 0 | 0
STATE | 0 | 0 | 0
TDSRATE | 0 | 0 | 0
TAXCLASSIFICATION | 0 | 0 | 0
STCATEGORY | 0 | 0 | 0
DEDUCTEETYPE | 0 | 0 | 0
ATTENDANCETYPE | 0 | 0 | 0
FBTCATEGORY | 0 | 0 | 0
FBTASSESSEETYPE | 0 | 0 | 0
TARIFFCLASSIFICATION | 0 | 0 | 0
EXCISEDUTYCLASSIFICATION | 0 | 0 | 0
SERIALNUMBER | 0 | 0 | 0
ADJUSTMENTCLASSIFICATION | 0 | 0 | 0
INCOMETAXSLAB | 0 | 0 | 0
INCOMETAXCLASSIFICATION | 0 | 0 | 0
LBTCLASSIFICATION | 0 | 0 | 0
TAXUNIT | 0 | 1 | 0
RETURNMASTER | 0 | 0 | 0
GSTCLASSIFICATION | 0 | 0 | 0
VOUCHERNUMBERSERIES | 0 | 1 | 0
VOUCHER | 0 | 2 | 0
```
