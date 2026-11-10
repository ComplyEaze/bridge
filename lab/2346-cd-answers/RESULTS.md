# Row 2346-CD: both notes were created from the build's invoice request with only the type and the sides changed; Tally numbered each `1`, and the Sales read-back with the note type's GUID returns each with its six legs as sent

Run on 10 Nov 2026, 01:58:46Z to 02:08:37Z (07:28 to 07:38 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/2346-cd-sheet` (9c2229cc), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no Bridge build. Nothing was keyed in Tally but BK (C5) and the restore (C6). The two writes, `I-CN.xml` and `I-DN.xml`, were each sent once. Every value below is M, measured by this run, in one run. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

The GSTINs in this folder are the synthetic family's: the company GSTIN `08ZZZZZ0000Z1ZQ`, and `08ZZZZZ0000Z2ZP`, the Purchase's supplier GSTIN in the V reads, as the book holds it. Both are on master already.

**Result, by the sheet's Decides (PARTIAL: one import of each type, one sitting, one book, one release).** Two lines apply:
- **"Each note is created, and its R row holds the six legs as sent."** `I-CN.xml`: `CREATED` 1, `ERRORS` 0, no LINEERROR. `I-DN.xml`: `CREATED` 1, `ERRORS` 0, no LINEERROR. R-CN1 and R-DN1 each return one row, with the six legs as the requests sent them:

| leg | Credit Note (R-CN1) | Debit Note (R-DN1) |
|---|---|---|
| `BRIDGE Walk-in` | No, 1051.00 | Yes, -1051.00 |
| `BRIDGE Svc 998313 5%` | Yes, -600.40 | No, 600.40 |
| `BRIDGE Svc 998314 5%` | Yes, -400.40 | No, 400.40 |
| `BRIDGE CGST 2.5%` | Yes, -25.02 | No, 25.02 |
| `BRIDGE SGST 2.5%` | Yes, -25.02 | No, 25.02 |
| `BRIDGE Round Off` | Yes, -0.16 | No, 0.16 |

  The sheet asks for the status flags and what Tally filled. On each note: included `Yes`, uncertain `No`, excluded `No`. `GSTNATUREOFRETURN`, `VATPARTYTRANSRETURNNUMBER` and `VATPARTYTRANSRETURNDATE` are empty. Tally filled the registration fields: `GSTREGISTRATION` `Rajasthan Registration` (no attributes), `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, `CMPGSTSTATE` `Rajasthan`, `CMPGSTREGISTRATIONTYPE` `Regular`. Each note also reads `PLACEOFSUPPLY` and `STATENAME` `Rajasthan`, `REFERENCE` `BP/26-27/0010`, `REFERENCEDATE` 20260801, `ISINVOICE` `Yes`, `VCHENTRYMODE` `Accounting Invoice`, `PERSISTEDVIEW` `Invoice Voucher View`, and BRIDGEVCHISSALES (the Sales class) `No`.
- **"Tally numbers the note itself."** Neither request sent a number. Tally gave the Credit Note `1` (GUID suffix `-0000002d`, MASTERID 45) and the Debit Note `1` (`-0000002e`, MASTERID 46), both dated 20260801. R-CN1 and R-DN1 read each one back by that number, which was known here only because the sheet chose `1` from the book's state before the send.

The other lines do not apply here. No note was refused, and none was stored other than as sent. R-0010 did not move after either note (`--diff`: lists compared 1, legs 6 -> 6, at s3r and at s4r), and R-CN1 did not move from s3r to s4r. Each note was returned by the read-back under its number with the note type's GUID.

- **s1:** one COMPANY row, the book's: ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **BK:** taken before anything was sent; restored at step 5.
- **s2, the book before:** every EXPECT met. W 40 rows, none of type `Credit Note` or `Debit Note`. V-1 2 rows: the Receipt numbered `1` (20260803) and the Purchase numbered `1` (20260804). R-CN1 0 rows, R-DN1 0 rows. R-0010 one row: `BRIDGE Walk-in`, included `Yes`, every other status flag `No`, 6 legs. M 68/245.
- **Step 3, the Credit Note:** s3m passed the before-write check. s3: as above. s3r: W 41 rows, the one new row the Credit Note `1`; V-1 3 rows; R-CN1 one row. ALTVCHID 68 to 69, ALTMSTID 245 unchanged.
- **Step 4, the Debit Note:** s4m passed the before-write check. s4: as above. s4r: W 42 rows, the new rows the Credit Note `1` and the Debit Note `1`; V-1 4 rows; R-DN1 one row; R-CN1 one row. ALTVCHID 69 to 70, ALTMSTID 245 unchanged.
- **Step 5, the restore:** by C6, with `restore-plan.txt` written first; the two expected prompts. s5: every answer byte-equal to s2's (M, W, V-1, R-CN1, R-DN1, R-0010); M equal to s1 by `--same-row`; R-0010 `--diff`: lists compared 1, legs 6 -> 6. The book is as it was found: ALTVCHID 68, ALTMSTID 245, no note.

**Deviation from the sheet:** the sheet says "Close every other Bridge process first." Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.

**Beyond the sheet, recorded and not judged:**
- **V-1 holds no ledger-entry lists.** `V-1.xml` asks for `NATIVEMETHOD *`. Its answer holds 729 plain fields for each voucher and no `LEDGERENTRIES.LIST`, `ALLLEDGERENTRIES.LIST`, `INVENTORYENTRIES.LIST` or `ACCOUNTINGALLOCATIONS.LIST`, for any row, the Receipt and the Purchase included. So `count17.py`'s new lines print `LEDGERENTRIES.LIST elements: 0` and `ALLLEDGERENTRIES.LIST elements: 0` for every row. The sheet's trial was on the fixture's voucher, which carries both lists. The notes' legs are measured here by R only.
- **R's bytes beyond its rows.** R-0010 at s3r, s4r and s5, and R-CN1 at s4r, differ from the earlier answers only in the CMPINFO count lines (LEDGER, CURRENCY, TAXUNIT, VOUCHERNUMBERSERIES, VOUCHER). These are not rows (LAB-COMMON C4).
- The Gateway's company line carried "(e)" with the footnote "(e) Data exceptions exist", at step 0 and after the reload at step 5 alike.

## Step 1: load, check and back up
- Sheet files: all 16 equal to `SHA256.txt` at 9c2229cc. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` and the two templates are byte-equal to row 2464-GR's. `W-2026-08.xml` is byte-equal to row 2162-KC's (604bd4e0). `count17.py` differs from `count16.py` only by the lines marked CD.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 01:58:46Z, from its desktop shortcut, after Get-Process showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter, no prompt).
- C2 at 01:59:43Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded.
- The book was loaded alone: `BRIDGE` and ` PILOT` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; date of last entry 15-Sep-26.
- `s1`: as in the Result.
- **BK** (C5), by row 2162-GY's step 1 path:
  - Before the backup, `C:\lab\2346-cd\backup-1` was made new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times, checked), then `C:\lab\2346-cd\backup-1` typed in three chunks (Tally showed it as `C:\lab\2346-Cd\backup-1`, its own capital), and its row taken.
  - BRIDGE PILOT LAB alone, End of List (Up 3 times, checked). "Accept ?" was captured (`bk-accept`, 02:02:16Z) and answered y; no other message; the Gateway again.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `21d761f261310b27d78e6f3df185d2e3aadd2ff1b625302004d29820edb82913`, written at 07:32 IST.

## Step 2: the book before
`s2`: `W-2026-08.xml V-1.xml R-CN1.xml R-DN1.xml R-0010.xml M-marks.xml`, each request checked against `SHA256.txt` just before its send. Every EXPECT met, as in the Result.

## Step 3: the Credit Note
`s3m` (02:03:02Z): M 68/245; the before-write check against s1 passed. No key was pressed in Tally after BK's y. `s3` (02:03:10Z): `I-CN.xml` sent once, answered within a second: `CREATED` 1, `ALTERED` 0, `DELETED` 0, `LASTVCHID` 45, `LASTMID` 0, `COMBINED` 0, `IGNORED` 0, `ERRORS` 0, `CANCELLED` 0, `EXCEPTIONS` 0. The window after it was the Gateway with no dialog (`s3-after-send`). `s3r` (02:03:27Z): `W-2026-08.xml V-1.xml R-CN1.xml R-0010.xml M-marks.xml`.

## Step 4: the Debit Note
`s4m` (02:03:56Z): M 69/245; the before-write check against s1 passed (one COMPANY row, the same GUID). `s4` (02:04:04Z): `I-DN.xml` sent once, answered within a second: `CREATED` 1, `ALTERED` 0, `DELETED` 0, `LASTVCHID` 46, `LASTMID` 0, `COMBINED` 0, `IGNORED` 0, `ERRORS` 0, `CANCELLED` 0, `EXCEPTIONS` 0. The window after it was the Gateway with no dialog (`s4-after-send`). `s4r` (02:04:22Z): `W-2026-08.xml V-1.xml R-DN1.xml R-CN1.xml R-0010.xml M-marks.xml`.

## Step 5: restore (C6) and compare
- 02:04:45Z, before the shut: `prep-c6.sh 4` re-hashed BK equal to `evidence/bk.txt`, found its folder holding only that file, wrote `restore-plan.txt` (kept on the lab PC, as it names the company number) and listed the Tally data folder (kept on the lab PC).
- The book shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter: Select Company, no company loaded.
- The Data label, Backup & Restore, Restore: Restore Companies; Specify Path (Up 3, checked); `C:\lab\2346-cd\backup-1` typed in three chunks (shown `C:\lab\2346-Cd\backup-1`); Enter: one company row, BRIDGE PILOT LAB, its number, 10-Nov-26 (7:32 am), as the plan says (`s5-restore-list`); Enter on it; Enter on End of List.
- "Accept ?" (`s5-accept`): `y`. "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip (`s5-folder-exists`): `r`. The restore ran and Tally returned to Select Company with no other prompt.
- The book reloaded alone by the filter (`s5-reloaded`): the Gateway as at step 0, the same period, date and date of last entry.
- `s5` (02:07:23Z): `M-marks.xml W-2026-08.xml V-1.xml R-CN1.xml R-DN1.xml R-0010.xml`. Every compare equal, and every answer's sha256 equal to s2's.

## Step 6: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 02:07:59Z).
- C2 at 02:08:03Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`. The three Bridge processes were left running (the deviation).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count17.py` output and compare, the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listing stay on the lab PC.
- `logs/`: each send's console output.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count17.py outputs, whole

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
== a-s2-R-CN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-DN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-V-1.xml: 101684 bytes
  STATUS '1'
  VOUCHER rows: 2
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   MASTERID '12'
   ALTERID '19'
   DATE '20260803'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Receipt'
   PARTYLEDGERNAME 'Cash'
   BASICBUYERNAME ''
   PARTYMAILINGNAME ''
   CONSIGNEEMAILINGNAME ''
   CONSIGNEESTATENAME ''
   CONSIGNEECOUNTRYNAME ''
   VATDEALERTYPE ''
   COUNTRYOFRESIDENCE ''
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY ''
   STATENAME ''
   REFERENCE ''
   REFERENCEDATE ''
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'No'
   VCHENTRYMODE ''
   PERSISTEDVIEW 'Accounting Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   MASTERID '23'
   ALTERID '31'
   DATE '20260804'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Purchase'
   PARTYLEDGERNAME 'BRIDGE Supplier RJ'
   BASICBUYERNAME 'BRIDGE PILOT LAB'
   PARTYMAILINGNAME 'BRIDGE Supplier RJ'
   CONSIGNEEMAILINGNAME 'BRIDGE PILOT LAB'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN '08ZZZZZ0000Z2ZP'
   GSTREGISTRATIONTYPE 'Regular'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'SUP/0001'
   REFERENCEDATE '20260803'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
== a-s2-W-2026-08.xml: 115168 bytes
  STATUS '1'
  VOUCHER rows: 40
   DATE '20260803'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   DATE '20260804'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   DATE '20260807'; VOUCHERNUMBER '10'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000020'
   DATE '20260807'; VOUCHERNUMBER '11'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000021'
   DATE '20260807'; VOUCHERNUMBER '12'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000022'
   DATE '20260807'; VOUCHERNUMBER '13'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000023'
   DATE '20260807'; VOUCHERNUMBER '14'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000024'
   DATE '20260803'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000d'
   DATE '20260804'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000018'
   DATE '20260803'; VOUCHERNUMBER '3'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000e'
   DATE '20260803'; VOUCHERNUMBER '4'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000f'
   DATE '20260807'; VOUCHERNUMBER '5'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001b'
   DATE '20260807'; VOUCHERNUMBER '6'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001c'
   DATE '20260807'; VOUCHERNUMBER '7'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001d'
   DATE '20260807'; VOUCHERNUMBER '8'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001e'
   DATE '20260807'; VOUCHERNUMBER '9'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0001'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0002'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0003'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0004'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0005'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0006'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0007'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0008'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0009'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0010'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0011'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0012'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000010'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0013'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000011'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0014'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000012'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0015'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000013'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0016'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000014'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0019'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000016'
   DATE '20260805'; VOUCHERNUMBER 'BP/26-27/0021'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001a'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0041'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'Yes'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0042'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000029'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0043'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002b'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0044'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002c'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
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
== a-s3-I-CN.xml: 540 bytes
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
== a-s3r-R-0010.xml: 94476 bytes
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
== a-s3r-R-CN1.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
== a-s3r-V-1.xml: 151418 bytes
  STATUS '1'
  VOUCHER rows: 3
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
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
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   MASTERID '12'
   ALTERID '19'
   DATE '20260803'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Receipt'
   PARTYLEDGERNAME 'Cash'
   BASICBUYERNAME ''
   PARTYMAILINGNAME ''
   CONSIGNEEMAILINGNAME ''
   CONSIGNEESTATENAME ''
   CONSIGNEECOUNTRYNAME ''
   VATDEALERTYPE ''
   COUNTRYOFRESIDENCE ''
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY ''
   STATENAME ''
   REFERENCE ''
   REFERENCEDATE ''
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'No'
   VCHENTRYMODE ''
   PERSISTEDVIEW 'Accounting Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   MASTERID '23'
   ALTERID '31'
   DATE '20260804'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Purchase'
   PARTYLEDGERNAME 'BRIDGE Supplier RJ'
   BASICBUYERNAME 'BRIDGE PILOT LAB'
   PARTYMAILINGNAME 'BRIDGE Supplier RJ'
   CONSIGNEEMAILINGNAME 'BRIDGE PILOT LAB'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN '08ZZZZZ0000Z2ZP'
   GSTREGISTRATIONTYPE 'Regular'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'SUP/0001'
   REFERENCEDATE '20260803'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
== a-s3r-W-2026-08.xml: 117962 bytes
  STATUS '1'
  VOUCHER rows: 41
   DATE '20260803'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   DATE '20260804'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   DATE '20260801'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Credit Note'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   DATE '20260807'; VOUCHERNUMBER '10'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000020'
   DATE '20260807'; VOUCHERNUMBER '11'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000021'
   DATE '20260807'; VOUCHERNUMBER '12'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000022'
   DATE '20260807'; VOUCHERNUMBER '13'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000023'
   DATE '20260807'; VOUCHERNUMBER '14'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000024'
   DATE '20260803'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000d'
   DATE '20260804'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000018'
   DATE '20260803'; VOUCHERNUMBER '3'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000e'
   DATE '20260803'; VOUCHERNUMBER '4'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000f'
   DATE '20260807'; VOUCHERNUMBER '5'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001b'
   DATE '20260807'; VOUCHERNUMBER '6'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001c'
   DATE '20260807'; VOUCHERNUMBER '7'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001d'
   DATE '20260807'; VOUCHERNUMBER '8'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001e'
   DATE '20260807'; VOUCHERNUMBER '9'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0001'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0002'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0003'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0004'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0005'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0006'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0007'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0008'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0009'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0010'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0011'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0012'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000010'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0013'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000011'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0014'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000012'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0015'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000013'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0016'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000014'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0019'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000016'
   DATE '20260805'; VOUCHERNUMBER 'BP/26-27/0021'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001a'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0041'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'Yes'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0042'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000029'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0043'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002b'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0044'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002c'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s4m
```
== a-s4m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s4
```
== a-s4-I-DN.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '46'
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
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
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
== a-s4r-R-CN1.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
== a-s4r-R-DN1.xml: 94452 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Debit Note'
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
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
== a-s4r-V-1.xml: 201144 bytes
  STATUS '1'
  VOUCHER rows: 4
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
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
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID '46'
   ALTERID '70'
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Debit Note'
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
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   MASTERID '12'
   ALTERID '19'
   DATE '20260803'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Receipt'
   PARTYLEDGERNAME 'Cash'
   BASICBUYERNAME ''
   PARTYMAILINGNAME ''
   CONSIGNEEMAILINGNAME ''
   CONSIGNEESTATENAME ''
   CONSIGNEECOUNTRYNAME ''
   VATDEALERTYPE ''
   COUNTRYOFRESIDENCE ''
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY ''
   STATENAME ''
   REFERENCE ''
   REFERENCEDATE ''
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'No'
   VCHENTRYMODE ''
   PERSISTEDVIEW 'Accounting Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   MASTERID '23'
   ALTERID '31'
   DATE '20260804'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Purchase'
   PARTYLEDGERNAME 'BRIDGE Supplier RJ'
   BASICBUYERNAME 'BRIDGE PILOT LAB'
   PARTYMAILINGNAME 'BRIDGE Supplier RJ'
   CONSIGNEEMAILINGNAME 'BRIDGE PILOT LAB'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN '08ZZZZZ0000Z2ZP'
   GSTREGISTRATIONTYPE 'Regular'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'SUP/0001'
   REFERENCEDATE '20260803'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
== a-s4r-W-2026-08.xml: 120750 bytes
  STATUS '1'
  VOUCHER rows: 42
   DATE '20260803'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   DATE '20260804'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   DATE '20260801'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Credit Note'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   DATE '20260801'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Debit Note'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   DATE '20260807'; VOUCHERNUMBER '10'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000020'
   DATE '20260807'; VOUCHERNUMBER '11'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000021'
   DATE '20260807'; VOUCHERNUMBER '12'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000022'
   DATE '20260807'; VOUCHERNUMBER '13'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000023'
   DATE '20260807'; VOUCHERNUMBER '14'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000024'
   DATE '20260803'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000d'
   DATE '20260804'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000018'
   DATE '20260803'; VOUCHERNUMBER '3'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000e'
   DATE '20260803'; VOUCHERNUMBER '4'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000f'
   DATE '20260807'; VOUCHERNUMBER '5'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001b'
   DATE '20260807'; VOUCHERNUMBER '6'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001c'
   DATE '20260807'; VOUCHERNUMBER '7'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001d'
   DATE '20260807'; VOUCHERNUMBER '8'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001e'
   DATE '20260807'; VOUCHERNUMBER '9'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0001'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0002'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0003'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0004'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0005'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0006'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0007'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0008'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0009'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0010'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0011'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0012'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000010'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0013'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000011'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0014'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000012'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0015'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000013'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0016'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000014'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0019'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000016'
   DATE '20260805'; VOUCHERNUMBER 'BP/26-27/0021'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001a'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0041'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'Yes'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0042'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000029'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0043'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002b'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0044'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002c'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s5
```
== a-s5-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
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
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s5-R-CN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s5-R-DN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s5-V-1.xml: 101684 bytes
  STATUS '1'
  VOUCHER rows: 2
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   MASTERID '12'
   ALTERID '19'
   DATE '20260803'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Receipt'
   PARTYLEDGERNAME 'Cash'
   BASICBUYERNAME ''
   PARTYMAILINGNAME ''
   CONSIGNEEMAILINGNAME ''
   CONSIGNEESTATENAME ''
   CONSIGNEECOUNTRYNAME ''
   VATDEALERTYPE ''
   COUNTRYOFRESIDENCE ''
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY ''
   STATENAME ''
   REFERENCE ''
   REFERENCEDATE ''
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'No'
   VCHENTRYMODE ''
   PERSISTEDVIEW 'Accounting Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   MASTERID '23'
   ALTERID '31'
   DATE '20260804'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Purchase'
   PARTYLEDGERNAME 'BRIDGE Supplier RJ'
   BASICBUYERNAME 'BRIDGE PILOT LAB'
   PARTYMAILINGNAME 'BRIDGE Supplier RJ'
   CONSIGNEEMAILINGNAME 'BRIDGE PILOT LAB'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN '08ZZZZZ0000Z2ZP'
   GSTREGISTRATIONTYPE 'Regular'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'SUP/0001'
   REFERENCEDATE '20260803'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   direct children: 729
== a-s5-W-2026-08.xml: 115168 bytes
  STATUS '1'
  VOUCHER rows: 40
   DATE '20260803'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   DATE '20260804'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   DATE '20260807'; VOUCHERNUMBER '10'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000020'
   DATE '20260807'; VOUCHERNUMBER '11'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000021'
   DATE '20260807'; VOUCHERNUMBER '12'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000022'
   DATE '20260807'; VOUCHERNUMBER '13'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000023'
   DATE '20260807'; VOUCHERNUMBER '14'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000024'
   DATE '20260803'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000d'
   DATE '20260804'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000018'
   DATE '20260803'; VOUCHERNUMBER '3'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000e'
   DATE '20260803'; VOUCHERNUMBER '4'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000f'
   DATE '20260807'; VOUCHERNUMBER '5'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001b'
   DATE '20260807'; VOUCHERNUMBER '6'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001c'
   DATE '20260807'; VOUCHERNUMBER '7'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001d'
   DATE '20260807'; VOUCHERNUMBER '8'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001e'
   DATE '20260807'; VOUCHERNUMBER '9'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0001'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0002'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0003'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0004'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0005'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0006'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0007'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0008'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0009'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0010'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0011'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0012'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000010'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0013'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000011'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0014'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000012'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0015'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000013'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0016'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000014'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0019'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000016'
   DATE '20260805'; VOUCHERNUMBER 'BP/26-27/0021'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001a'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0041'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'Yes'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0042'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000029'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0043'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002b'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0044'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002c'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

## Compares and checks, whole

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3r: R-0010 against s2 (--diff)
```
== --diff a-s2-R-0010.xml a-s3r-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s4m: M against s1 (the before-write check)
```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4r: R-0010 against s2 (--diff)
```
== --diff a-s2-R-0010.xml a-s4r-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s4r: R-CN1 against s3r (--diff)
```
== --diff a-s3r-R-CN1.xml a-s4r-R-CN1.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s5: M against s1 (--same-row)
```
== a-s5-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### s5: R-0010 against s2 (--diff)
```
== --diff a-s2-R-0010.xml a-s5-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-W-2026-08.xml 115168 B f67cf46f57e7f4ae65d864f7fc73ed3b20db7027d7fbf048f1730cc691387e97
a-s2-V-1.xml 101684 B 73317fdb3898be0d6dcd1a1f1d86dcc9165cf5e3d6d885baedf64a5b89d6b5f0
a-s2-R-CN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-DN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3-I-CN.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s3r-W-2026-08.xml 117962 B 97e4637cca39a88b4ef520dcd48e132f2fba02c2aa63edb57cbb7ec43c65b55d
a-s3r-V-1.xml 151418 B 6d0519bcd31711abc1d06c6d29c85db708d2290e069310cfa24ae781c3aa1708
a-s3r-R-CN1.xml 94478 B 2da5809167a5852c5c41fb6e5fdd1932fe1c1deaca27c7fa23b2d11e268aa721
a-s3r-R-0010.xml 94476 B 4f91c6409825fee304b5ae6533c8462c53abd2f19d4252dd41842c34abcfacb7
a-s3r-M-marks.xml 3396 B 8feeba3e89f1c0940a4781d39da6b9ea59e8885d0259302ae03e64ca2d06b8b5
a-s4m-M-marks.xml 3396 B 8feeba3e89f1c0940a4781d39da6b9ea59e8885d0259302ae03e64ca2d06b8b5
a-s4-I-DN.xml 540 B 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
a-s4r-W-2026-08.xml 120750 B 5cdb9b3536e40b3c4afa9d9399a051d42df2b91114db2ca823e037d86310d574
a-s4r-V-1.xml 201144 B 5edb5f3e42d6fc6ed72d9ad60047935dcf0aeee6ba461392fb5b3a450a4b41dd
a-s4r-R-DN1.xml 94452 B e24a10d6956df78db45537b34df9ee466219b89aa0a292ad3831d2e07aa8f02f
a-s4r-R-CN1.xml 94478 B 32eac9a41367ac368b4a039924caea0c52283822389904756cebab77e99ac241
a-s4r-R-0010.xml 94476 B f173644c06d12634c984a18a03796ead8a9ce881d61747f2dfdbed5dcfc3596a
a-s4r-M-marks.xml 3396 B 7a5fe6a0e25913f4272a6edfe2b051f98ea4685d6d4fd9643a3ccf3e09a92723
a-s5-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s5-W-2026-08.xml 115168 B f67cf46f57e7f4ae65d864f7fc73ed3b20db7027d7fbf048f1730cc691387e97
a-s5-V-1.xml 101684 B 73317fdb3898be0d6dcd1a1f1d86dcc9165cf5e3d6d885baedf64a5b89d6b5f0
a-s5-R-CN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s5-R-DN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s5-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
```
