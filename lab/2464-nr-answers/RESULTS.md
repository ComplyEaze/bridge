# Row 2464-NR: on the one-registration book, the import that names `Rajasthan Registration` was created (`CREATED` 1) and stored the registration as `BP/26-27/0010` holds it; the read-back returns it; B' was not sent by step 4's rule

Run on 10 Nov 2026, 04:18:46Z to 04:27:40Z (09:48 to 09:57 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as row 2464-GN read it from the program's file; the Gateway shows no release number), by the sheet at `lab/2464-nr` (de14c060), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no Bridge build. No master change was keyed. One import (`I-0085.xml`, sent once); `I-0087.xml` was not sent (step 4). Then the restore of this sitting's own backup. Every value below is M, measured by this run, in one run. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

The GSTIN in this folder is the synthetic family's, as the sheet names it: the company's own `08ZZZZZ0000Z1ZQ`.

## Result

**For the write, by the sheet's Decides (PARTIAL: one import, one sitting, one book, one release): "A' is created and stores `Rajasthan Registration`, with `CMPGSTIN`, the state and the type as `BP/26-27/0010` holds them".** The sheet's line for this case: the element, in this place and form, is taken on an import where it agrees with the invoice and the book. Row 2464-GN's refusal then came from the unit disagreeing with the invoice or the book (its causes 2 and 3), not from the element.

| counter | A', `I-0085.xml` (s3) | GN's A, `I-0095.xml` |
|---|---|---|
| `CREATED` | 1 | 0 |
| `ALTERED` | 0 | 0 |
| `DELETED` | 0 | 0 |
| `LASTVCHID` | 45 | 0 |
| `LASTMID` | 0 | 0 |
| `COMBINED` | 0 | 0 |
| `IGNORED` | 0 | 0 |
| `ERRORS` | 0 | 0 |
| `CANCELLED` | 0 | 0 |
| `EXCEPTIONS` | 0 | 1 |

- The answer is 540 B, sha256 `fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea`; it is not byte-equal to GN's import A answer (538 B, sha256 `09a713e2...`). It carries no LINEERROR. The window after the send was the Gateway, no dialog (`s3-after`).
- **What A' stored** (s3r), V-0085's six fields as `count19.py` prints them, beside V-0010's:

  | field | V-0085 | V-0010 |
  |---|---|---|
  | `GSTREGISTRATION` | `Rajasthan Registration` (no attribute) | `Rajasthan Registration` (no attribute) |
  | `CMPGSTIN` | `08ZZZZZ0000Z1ZQ` | `08ZZZZZ0000Z1ZQ` |
  | `CMPGSTSTATE` | `Rajasthan` | `Rajasthan` |
  | `CMPGSTREGISTRATIONTYPE` | `Regular` | `Regular` |
  | `PLACEOFSUPPLY` | `Rajasthan` | `Rajasthan` |
  | `STATENAME` | `Rajasthan` | `Rajasthan` |

- `--diff` of V-0010 against V-0085 (beyond the sheet): only `ALTERID`, `GUID`, `MASTERID`, `REFERENCE`, `REMOTEGUID` (its last group, from 0010's tail to the tail the sheet gave A'), `VOUCHERKEY`, `VOUCHERNUMBER` and `VOUCHERRETAINKEY` differ; lists compared 2.
- **N-0085:** one row, `BP/26-27/0085`, DATE 20260801, GUID the book's with suffix `-0000002d`, MASTERID 45.
- **R-0085:** `BRIDGE Walk-in`; included `Yes`, uncertain `No`, excluded `No`, `ISGSTOVERRIDDEN` `No`; six legs: `BRIDGE Walk-in` -1051.00, `BRIDGE Svc 998313 5%` 600.40, `BRIDGE Svc 998314 5%` 400.40, `BRIDGE CGST 2.5%` 25.02, `BRIDGE SGST 2.5%` 25.02, `BRIDGE Round Off` 0.16. `--diff` of s2's R-0010 against R-0085 (beyond the sheet): only the voucher's own identity fields and number differ, legs 6 -> 6. So the status flags are those of `BP/26-27/0010`, as the sheet estimated.
- `V-0010` did not move: `--diff` against s2 shows no field moved (lists compared 2, legs 0 -> 0).
- M: ALTVCHID 68 -> 69, ALTMSTID 245 -> 245.

**For the read-back: "RC returns `GSTREGISTRATION` and `CMPGSTIN` equal to V's, with every other field equal to R's".** RC-0085 returned `GSTREGISTRATION` `Rajasthan Registration` and `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, equal to V-0085's. `--diff` of R-0085 against RC-0085: those two fields the only ones that differ, each from `<absent>`; legs 6 -> 6. As in row 2464-GN's RC-0010, `GSTREGISTRATION` carries the attribute `TYPE="String"` in RC, where V's carries none; recorded and not judged.

**Step 4: B' was not sent, by the sheet's rule.** Before B', s4m's `--unit-diff` against s2 read 0 leaves differ in both units, and ALTMSTID was 245, equal to s3m's. `--type-diff` against s2 read one leaf: `BRIDGE Sales` `/LASTNUMBER[1]` `'BP/26-27/0044'` -> `'BP/26-27/0085'`. The sheet: "If any differs ... Send nothing more. The difference is recorded as a result of A'." So `I-0087.xml` was not sent, and the row went to step 5. This is the sheet's rule applied, not a fault. The run lead's word after the run: B' gets no sitting of its own; it is folded into the next #2464 row as a step of its own.

- **s1:** one COMPANY row, the book's: ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **BK:** taken after s1, before anything was sent but reads; restored at step 5.
- **s2, the book before:** every answer byte-equal to row 2464-GN's s2 answer of the same kind (`evidence/compare-s2.txt`): M, U, T, N-0085 and N-0087 (each against GN's `a-s2-N-0095.xml`), R-0010, RC-0010 and V-0010. EXPECT met. So two units, `Rajasthan Registration` the one GST unit; `BRIDGE Sales` unbound; 0 rows for each number.
- **Step 5, the restore:** by C6, with `restore-plan.txt` written first; the two expected prompts. s5: `--same-row` against s1 equal; `--unit-diff` 0 leaves differ in each of the two units; `--type-diff` `BRIDGE Sales` 0 leaves differ; N-0085 and N-0087 0 rows each; `--diff` of R-0010 and of RC-0010 against s2 only `lists compared: 1` and `legs: 6 -> 6`; every answer's sha256 equal to s2's (`evidence/compare-s5.txt`). EXPECT met. The book is as it was found: ALTVCHID 68, ALTMSTID 245.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** The sheet says "Close every other Bridge process first." Three Bridge processes were running at the start; on the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **The start-up company.** Tally's start-up company loaded by itself, as the install is set to. It was shut first by the K: Company label, Shut, Enter on its one row, no prompt, on the run lead's standing practice for these sittings, with no change to it.
3. **Files removed by `rm` in a scratch copy.** To rerun `gen.py`, the sheet's files were copied into a scratch folder off the lab folders, and the copied request files that `gen.py` writes were then removed there by `rm` before the rerun, a moment after they were copied. Nothing in this row's folder or any lab folder was touched. The run lead's rule is the Recycle Bin, or leaving the files and listing them, even in a scratch copy. It was reported to the run lead, who asked for it to be recorded here.

**Beyond the sheet, recorded and not judged:**
- The Gateway's company line carried "(e)" and "(Schedule Backup)", with the footnote "(e) Data exceptions exist", at step 0 and after the reload at step 5 alike.
- The DESC/CMPINFO count lines (LAB-COMMON C4: these are not rows) were 0 in every answer of s1, s2, s3m and s5. In every answer of s3r and s4m, after A', they read LEDGER 11, CURRENCY 1, TAXUNIT 1, VOUCHERNUMBERSERIES 1 and VOUCHER 2, the others 0 (`evidence/cmpinfo.txt`). GN's s5r, after its refused import A, read LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2 and VOUCHERNUMBERSERIES 1 there, on its two-registration book.
- V-0010's answer at s3r is 2 B longer than at s2 (52,786 B against 52,784 B), with no field moved by `--diff`. The only lines of the two answers that differ are the five CMPINFO count lines above; LEDGER's 0 to 11 is the one extra character (2 B in UTF-16).

## Step 1: load, check and back up
- Sheet files: all 25 equal to `SHA256.txt` at de14c060. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count19.py`, `M-marks.xml`, `U-taxunits.xml`, `T-types.xml`, `R-0010.xml`, `RC-0010.xml`, `V-0010.xml` and the three templates are byte-equal to row 2464-GN's. `gen.py`, rerun in a scratch copy, writes its 12 requests byte-equal to the sheet's (and asserts GN's `I-0095.xml` byte for byte).
- Row 2464-GN's s2 answers were checked against the sheet's table (size and sha256), and five were copied beside this row's answers as `a-gns2-U-taxunits.xml`, `a-gns2-RC-0010.xml`, `a-gns2-V-0010.xml`, `a-gns2-T-types.xml` and `a-gns2-R-0010.xml`, each byte-equal to its source (`evidence/gns2-copies.txt`). T and R were copied from the lab PC's copies of GN's committed answers, which carry the table's sha256. As no s2 answer differed, none was used for a compare.
- Before the start, the book's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 04:18:46Z, from its desktop shortcut, after Get-Process showed none. Its start-up company was shut first (deviation 2).
- C2 at 04:19:48Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE`, a space, `PILOT`, a space and `LAB` typed into Select Company's name filter (checked by zoom), one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; date of last entry 15-Sep-26. As GN read them.
- `s1` (04:20:40Z): as in the Result.
- **BK** (C5), by GN's path, to a new folder:
  - Before the backup, `C:\lab\2464-nr\backup-1` was made new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times, checked), Enter, then `C:\lab\2464-nr\backup-1` typed in three chunks (Tally showed it as `C:\lab\2464-Nr\backup-1`, its own capital, checked by zoom), and its row taken.
  - `BRIDGE`, a space and `PILOT` typed: one row, BRIDGE PILOT LAB, Enter; End of List (Up 3 times, checked), Enter. "Accept ?" was captured (`bk-accept`, 04:22:15Z) and answered y; no other message; the Gateway again.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `a057e749f18085b288cef3f5df097cccb29d0a31b27d76d798db810e14bba05f`, written at 09:52 IST.

## Step 2: the book before
`s2` (04:22:43Z): `U-taxunits.xml T-types.xml N-0085.xml N-0087.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml`, each request checked against `SHA256.txt` just before its send. As in the Result.

## Step 3: import A'
- `s3m` (04:23:00Z): M 68/245; the before-write check against s1 passed (one COMPANY row, the same GUID). No key was pressed in Tally since BK's y.
- `s3` (04:23:09Z): `I-0085.xml` sent once; answered as in the Result; the window after it the Gateway, no dialog (`s3-after`, 04:23:13Z). The wait rule was not needed: the answer came at once.
- `s3r` (04:23:26Z): `N-0085.xml R-0085.xml RC-0085.xml V-0085.xml V-0010.xml M-marks.xml`. As in the Result. A' is created by the sheet's test (`CREATED` 1, `ERRORS` 0, `EXCEPTIONS` 0, N-0085 one row): to step 4.

## Step 4: before B'
- `s4m` (04:23:55Z): `T-types.xml U-taxunits.xml M-marks.xml`; the three checks as in the Result. The type check found one leaf, so nothing more was sent and `I-0087.xml` was not sent; to step 5.

## Step 5: restore (C6) and compare
- 04:24:13Z, before the shut: `prep-c6.sh 5` re-hashed BK equal to `evidence/bk.txt`, found its folder holding only that file, wrote `restore-plan.txt` (kept on the lab PC, as it names the company number) and listed the Tally data folder (kept on the lab PC).
- The book shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded.
- The Data label, Backup & Restore, Restore: Restore Companies, Restore Data Path `<Tally data folder>`; Specify Path (Up 3, checked); `C:\lab\2464-nr\backup-1` typed in three chunks (shown `C:\lab\2464-Nr\backup-1`, checked by zoom); Enter: one company row, BRIDGE PILOT LAB, its number, 10-Nov-26 (9:52 am), as the plan says (`restore-list`, 04:25:17Z); Enter on it; Enter on End of List.
- "Accept ?" (`restore-accept`, 04:25:40Z): `y`. "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip (`restore-replace`, 04:25:56Z): `r`. The restore ran and Tally returned to Select Company with no other prompt.
- The book reloaded alone by the filter, as at step 1 (`s5-gateway`): the Gateway as at step 0, the same period, date and date of last entry.
- `s5` (04:26:40Z): `M-marks.xml U-taxunits.xml T-types.xml N-0085.xml N-0087.xml R-0010.xml RC-0010.xml V-0010.xml`. Every compare equal, as in the Result.

## Step 6: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 04:27:21Z).
- C2 at 04:27:21Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`. The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count19.py` output and compare, the answers' sizes and sha256. Process and port listings, the backup-folder listing and the data-folder listings stay on the lab PC.
- `logs/`: `run.py`'s log of each send.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count19.py outputs, whole

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
== a-s2-N-0085.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0087.xml: 3022 bytes
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s2-RC-0010.xml: 94744 bytes
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
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   GSTREGISTRATION attributes {'TYPE': 'String'}
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s2-T-types.xml: 306946 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]@TYPE 'String'
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]/LASTSERIESBYGSTTAXUNIT[1] 'Default_BRIDGE Sales'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]@TYPE 'String'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]/LASTGSTTAXUNITBYSERIES[1] 'Rajasthan Registration'
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 0
== a-s2-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
== a-s2-V-0010.xml: 52784 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   MASTERID '11'
   ALTERID '17'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   BASICBUYERNAME 'BRIDGE Walk-in'
   PARTYMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEEMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s3m
```
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s3
```
== a-s3-I-0085.xml: 540 bytes
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

### s3r
```
== a-s3r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3r-N-0085.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0085' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s3r-R-0085.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0085'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0085'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s3r-RC-0085.xml: 94748 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0085'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0085'
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
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   GSTREGISTRATION attributes {'TYPE': 'String'}
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s3r-V-0010.xml: 52786 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   MASTERID '11'
   ALTERID '17'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   BASICBUYERNAME 'BRIDGE Walk-in'
   PARTYMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEEMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
== a-s3r-V-0085.xml: 52788 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0085'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   BASICBUYERNAME 'BRIDGE Walk-in'
   PARTYMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEEMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0085'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s4m
```
== a-s4m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4m-T-types.xml: 306948 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]@TYPE 'String'
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]/LASTSERIESBYGSTTAXUNIT[1] 'Default_BRIDGE Sales'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]@TYPE 'String'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]/LASTGSTTAXUNITBYSERIES[1] 'Rajasthan Registration'
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 0
== a-s4m-U-taxunits.xml: 22638 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

### s5
```
== a-s5-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5-N-0085.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s5-N-0087.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s5-R-0010.xml: 94474 bytes
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5-RC-0010.xml: 94744 bytes
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
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   GSTREGISTRATION attributes {'TYPE': 'String'}
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5-T-types.xml: 306946 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]@TYPE 'String'
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]/LASTSERIESBYGSTTAXUNIT[1] 'Default_BRIDGE Sales'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]@TYPE 'String'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]/LASTGSTTAXUNITBYSERIES[1] 'Rajasthan Registration'
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 0
== a-s5-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
== a-s5-V-0010.xml: 52784 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   MASTERID '11'
   ALTERID '17'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   BASICBUYERNAME 'BRIDGE Walk-in'
   PARTYMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEEMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

## Compares and checks, whole

### step 1: row 2464-GN's s2 answers, checked against the sheet's table before the copies
```
GN a-s2-U-taxunits.xml 22636 B sha256 equal to the sheet table
GN a-s2-RC-0010.xml 94744 B sha256 equal to the sheet table
GN a-s2-V-0010.xml 52784 B sha256 equal to the sheet table
GN a-s2-T-types.xml 306946 B sha256 equal to the sheet table
GN a-s2-R-0010.xml 94474 B sha256 equal to the sheet table
GN a-s2-M-marks.xml 3394 B sha256 equal to the sheet table
GN a-s2-N-0095.xml 3022 B sha256 equal to the sheet table
GN a-s2-N-0097.xml 3022 B sha256 equal to the sheet table
```

### s2: each answer against row 2464-GN's s2
```
$ step 2: each s2 answer against row 2464-GN's s2 answer of the same kind, by size, sha256 and bytes
a-s2-U-taxunits.xml 22636 B sha256 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 | GN a-s2-U-taxunits.xml 22636 B | byte-equal: yes
a-s2-T-types.xml 306946 B sha256 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed | GN a-s2-T-types.xml 306946 B | byte-equal: yes
a-s2-N-0085.xml 3022 B sha256 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | GN a-s2-N-0095.xml 3022 B | byte-equal: yes
a-s2-N-0087.xml 3022 B sha256 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | GN a-s2-N-0095.xml 3022 B | byte-equal: yes
a-s2-R-0010.xml 94474 B sha256 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 | GN a-s2-R-0010.xml 94474 B | byte-equal: yes
a-s2-RC-0010.xml 94744 B sha256 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399 | GN a-s2-RC-0010.xml 94744 B | byte-equal: yes
a-s2-V-0010.xml 52784 B sha256 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456 | GN a-s2-V-0010.xml 52784 B | byte-equal: yes
a-s2-M-marks.xml 3394 B sha256 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | GN a-s2-M-marks.xml 3394 B | byte-equal: yes
```

### s3m: M against s1 (<the before-write option>)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s3r: R-0085 against RC-0085 (--diff)
```
== --diff a-s3r-R-0085.xml a-s3r-RC-0085.xml
   CMPGSTIN: <absent> -> '08ZZZZZ0000Z1ZQ'
   GSTREGISTRATION: <absent> -> 'Rajasthan Registration'
   lists compared: 1
   legs: 6 -> 6
```

### s3r: V-0010 against s2 (--diff)
```
== --diff a-s2-V-0010.xml a-s3r-V-0010.xml
   lists compared: 2
   legs: 0 -> 0
```

### s3r: V-0010 against V-0085 (--diff), beyond the sheet
```
== --diff a-s3r-V-0010.xml a-s3r-V-0085.xml
   ALTERID: '17' -> '69'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0085'
   REMOTEGUID: '<REMOTEGUID, the sheet value>' -> '<REMOTEGUID, the sheet value>'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0085'
   VOUCHERRETAINKEY: '41' -> '129'
   lists compared: 2
   legs: 0 -> 0
```

### s3r: s2's R-0010 against R-0085 (--diff), beyond the sheet
```
== --diff a-s2-R-0010.xml a-s3r-R-0085.xml
   ALTERID: '17' -> '69'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0085'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0085'
   VOUCHERRETAINKEY: '41' -> '129'
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s4m: T against s2 (--type-diff)
```
== --type-diff a-s2-T-types.xml a-s4m-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 1 leaves differ
     /LASTNUMBER[1]: 'BP/26-27/0044' -> 'BP/26-27/0085'
```

### s4m: U against s2 (--unit-diff)
```
== --unit-diff a-s2-U-taxunits.xml a-s4m-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### s5: M against s1 (--same-row)
```
== a-s5-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### s5: U against s2 (--unit-diff)
```
== --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
```

### s5: T against s2 (--type-diff)
```
== --type-diff a-s2-T-types.xml a-s5-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
```

### s5: R-0010 and RC-0010 against s2 (--diff)
```
== --diff a-s2-R-0010.xml a-s5-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
== --diff a-s2-RC-0010.xml a-s5-RC-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s5: each answer against s2's by sha256
```
$ step 5: each s5 answer against s2's of the same name, by size, sha256 and bytes
a-s5-M-marks.xml 3394 B sha256 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 | s2 3394 B | byte-equal: yes
a-s5-U-taxunits.xml 22636 B sha256 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 | s2 22636 B | byte-equal: yes
a-s5-T-types.xml 306946 B sha256 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed | s2 306946 B | byte-equal: yes
a-s5-N-0085.xml 3022 B sha256 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | s2 3022 B | byte-equal: yes
a-s5-N-0087.xml 3022 B sha256 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | s2 3022 B | byte-equal: yes
a-s5-R-0010.xml 94474 B sha256 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 | s2 94474 B | byte-equal: yes
a-s5-RC-0010.xml 94744 B sha256 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399 | s2 94744 B | byte-equal: yes
a-s5-V-0010.xml 52784 B sha256 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456 | s2 52784 B | byte-equal: yes
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every read answer that has them (LAB-COMMON C4: not rows); the names with a count other than 0, then the rest as "others 0"
a-s1-M-marks.xml: all 0 (names in all: 35)
a-s2-U-taxunits.xml: all 0 (names in all: 35)
a-s2-T-types.xml: all 0 (names in all: 35)
a-s2-N-0085.xml: all 0 (names in all: 35)
a-s2-N-0087.xml: all 0 (names in all: 35)
a-s2-R-0010.xml: all 0 (names in all: 35)
a-s2-RC-0010.xml: all 0 (names in all: 35)
a-s2-V-0010.xml: all 0 (names in all: 35)
a-s2-M-marks.xml: all 0 (names in all: 35)
a-s3m-M-marks.xml: all 0 (names in all: 35)
a-s3r-N-0085.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s3r-R-0085.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s3r-RC-0085.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s3r-V-0085.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s3r-V-0010.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s3r-M-marks.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4m-T-types.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4m-U-taxunits.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4m-M-marks.xml: LEDGER 11; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s5-M-marks.xml: all 0 (names in all: 35)
a-s5-U-taxunits.xml: all 0 (names in all: 35)
a-s5-T-types.xml: all 0 (names in all: 35)
a-s5-N-0085.xml: all 0 (names in all: 35)
a-s5-N-0087.xml: all 0 (names in all: 35)
a-s5-R-0010.xml: all 0 (names in all: 35)
a-s5-RC-0010.xml: all 0 (names in all: 35)
a-s5-V-0010.xml: all 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s2-N-0085.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0087.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s2-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
a-s2-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3-I-0085.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s3r-N-0085.xml 5608 B 55fb08c004abd22cce75a49049dfb72b025481ee14b90fc5a0456be8813c7d4a
a-s3r-R-0085.xml 94478 B 33a17e1a213b66fcb3eed35b7503b5296d0b52ba428abcf21d781a6599b1c1fa
a-s3r-RC-0085.xml 94748 B d47e47fab561a26a5db4c31df388da3043f51fa5a3197e30bb3fdd687846fc69
a-s3r-V-0085.xml 52788 B 977e683446efa3a5d8de39d681a8227e6d01448f89cef920e789082d44fef354
a-s3r-V-0010.xml 52786 B aa0d777c8cc8c3cf96b009d129b692b03e5a51967d9aa53f0aa78d011807d4cf
a-s3r-M-marks.xml 3396 B 8feeba3e89f1c0940a4781d39da6b9ea59e8885d0259302ae03e64ca2d06b8b5
a-s4m-T-types.xml 306948 B f50a05f693d1c1fcc3ca17ea0df87556dcdc6fdf766a4c488650c025e7ef19ec
a-s4m-U-taxunits.xml 22638 B 562447315e81ea1e1bf9da7e3c618b465ba6e0a2b7c2113a88067c0fbe3b980c
a-s4m-M-marks.xml 3396 B 8feeba3e89f1c0940a4781d39da6b9ea59e8885d0259302ae03e64ca2d06b8b5
a-s5-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s5-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s5-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s5-N-0085.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s5-N-0087.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s5-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s5-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
a-s5-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
```

### each send's start, UTC
```
2026-11-10T04:20:40.172Z s1 M-marks.xml (hashes equal to the lists)
2026-11-10T04:22:43.703Z s2 U-taxunits.xml T-types.xml N-0085.xml N-0087.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:23:00.934Z s3m M-marks.xml (hashes equal to the lists)
2026-11-10T04:23:09.361Z s3 I-0085.xml (hashes equal to the lists)
2026-11-10T04:23:26.056Z s3r N-0085.xml R-0085.xml RC-0085.xml V-0085.xml V-0010.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:23:55.569Z s4m T-types.xml U-taxunits.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:26:40.599Z s5 M-marks.xml U-taxunits.xml T-types.xml N-0085.xml N-0087.xml R-0010.xml RC-0010.xml V-0010.xml (hashes equal to the lists)
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
09:50:40	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
09:52:44	U-taxunits.xml	a-s2-U-taxunits.xml	curl 0	22636 bytes
09:52:44	T-types.xml	a-s2-T-types.xml	curl 0	306946 bytes
09:52:44	N-0085.xml	a-s2-N-0085.xml	curl 0	3022 bytes
09:52:44	N-0087.xml	a-s2-N-0087.xml	curl 0	3022 bytes
09:52:44	R-0010.xml	a-s2-R-0010.xml	curl 0	94474 bytes
09:52:44	RC-0010.xml	a-s2-RC-0010.xml	curl 0	94744 bytes
09:52:45	V-0010.xml	a-s2-V-0010.xml	curl 0	52784 bytes
09:52:45	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3m
```
09:53:01	M-marks.xml	a-s3m-M-marks.xml	curl 0	3394 bytes
```

### s3
```
09:53:09	I-0085.xml	a-s3-I-0085.xml	curl 0	540 bytes
```

### s3r
```
09:53:26	N-0085.xml	a-s3r-N-0085.xml	curl 0	5608 bytes
09:53:26	R-0085.xml	a-s3r-R-0085.xml	curl 0	94478 bytes
09:53:26	RC-0085.xml	a-s3r-RC-0085.xml	curl 0	94748 bytes
09:53:27	V-0085.xml	a-s3r-V-0085.xml	curl 0	52788 bytes
09:53:27	V-0010.xml	a-s3r-V-0010.xml	curl 0	52786 bytes
09:53:27	M-marks.xml	a-s3r-M-marks.xml	curl 0	3396 bytes
```

### s4m
```
09:53:55	T-types.xml	a-s4m-T-types.xml	curl 0	306948 bytes
09:53:55	U-taxunits.xml	a-s4m-U-taxunits.xml	curl 0	22638 bytes
09:53:55	M-marks.xml	a-s4m-M-marks.xml	curl 0	3396 bytes
```

### s5
```
09:56:40	M-marks.xml	a-s5-M-marks.xml	curl 0	3394 bytes
09:56:40	U-taxunits.xml	a-s5-U-taxunits.xml	curl 0	22636 bytes
09:56:41	T-types.xml	a-s5-T-types.xml	curl 0	306946 bytes
09:56:41	N-0085.xml	a-s5-N-0085.xml	curl 0	3022 bytes
09:56:41	N-0087.xml	a-s5-N-0087.xml	curl 0	3022 bytes
09:56:41	R-0010.xml	a-s5-R-0010.xml	curl 0	94474 bytes
09:56:41	RC-0010.xml	a-s5-RC-0010.xml	curl 0	94744 bytes
09:56:41	V-0010.xml	a-s5-V-0010.xml	curl 0	52784 bytes
```
