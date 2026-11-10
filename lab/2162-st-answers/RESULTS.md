# Row 2162-ST: a second Manual Sales type, keyed with "Prevent duplicates", accepted the invoice number that another Sales type already held; the import created it, and the read-back by the first type's GUID still found only its own invoice

Run on 10 Nov 2026, 05:11:03Z to 05:25:24Z (10:41 to 10:55 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as row 2162-KD read it from the program's file; the Gateway shows no release number), by the sheet at `lab/2162-st-sheet` (b6636e0b), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. One voucher type keyed on Tally's screen; one import, `I-0010k.xml`, sent once; read-only reads around it, one at a time through `run.py`; no Bridge build. Then the restore of this sitting's own backup. Every value below is M, measured by this run, in one run. Every read was answered, each with STATUS 1. The window after the import was the Gateway, no dialog.

This sitting ran after row 683-VD (Tally quit before it), with a Tally start of its own, so step 1's "otherwise" applied: step 0 of C2, then the book loaded alone.

## Result

PARTIAL by the sheet's Decides: one book, one release, one sitting.

| import | step | `CREATED` | `ALTERED` | `EXCEPTIONS` | `ERRORS` | `LASTVCHID` | LINEERROR |
|---|---|---|---|---|---|---|---|
| `I-0010k.xml`, `BP/26-27/0010` into `BRIDGE Sales Keyed` | s4 | 1 | 0 | 0 | 0 | 45 | none |

`DELETED`, `LASTMID`, `COMBINED`, `IGNORED` and `CANCELLED` were 0.

**The answer is "created", with N two rows and R one, as the sheet's first Decides case names.**
- s4r N (the build's number read): two rows, both `BP/26-27/0010`, date 20260801: one on `BRIDGE Sales`, one on `BRIDGE Sales Keyed`; each `BRIDGEVCHISSALES` Yes.
- s4r V: two rows. `BRIDGE Sales`: GUID tail `0000000b`, MASTERID 11, ALTERID 17, as at s2. `BRIDGE Sales Keyed`: GUID tail `0000002d`, MASTERID 45 (the import's `LASTVCHID`), ALTERID 69. Every other field V prints is equal between the two rows (party, the buyer and consignee block, the GST registration and status fields, place of supply, reference and its date, entry mode and view); each row has 729 direct children.
- s4r R (the build's read-back with `BRIDGE Sales`' GUID): one row, the `BRIDGE Sales` invoice, every field and leg `count22.py` prints as at s2. The answer's bytes differ from s2's only in six DESC/CMPINFO count lines (LAB-COMMON C4: not rows): `LEDGER`, `VOUCHERTYPE`, `CURRENCY`, `TAXUNIT`, `VOUCHERNUMBERSERIES` and `VOUCHER` (`evidence/cmpinfo-s4r-r.txt`); `count22.py --diff` of the two: legs 6 -> 6.
- The marks after the import: ALTVCHID 69 (68 before), ALTMSTID 247.

By the sheet's Decides for this case, quoted:
- "Tally's duplicate guard is per type on Sales too."
- "The build's read after the post does not see the second invoice, because it reads the posted type only."
- "So an invoice whose number another Sales type takes between the queue's read and the import would read `posted_verified`."
- "#2162 needs the number read sent again after the post. `posted_verified` would then need exactly one Sales-class row, the posted invoice."

**The key (step 3).** `BRIDGE Sales Keyed` was keyed on screen by row 2162-KD's path, the four fields only. s3r: T 29 rows; the new type one row, `PARENT` `Sales`, `NUMBERINGMETHOD` `None`, one series `Default`, `Manual`, `NUMBERINGSUBMETHOD` `Default`, `PREVENTDUPLICATES` Yes, GUID tail `000000e8`. The whole s3r T answer is byte-equal to row 2162-KD's s3r T answer (sha256 b1d08c99...), and `count22.py --type-diff` against s2: `BRIDGE Sales` 0 leaves differ, the new type in the second only. M: ALTVCHID 68, ALTMSTID 247.

**The way back (step 5).** Restored by C6 with only the two expected prompts. s9, every file of s2 in s2's order: `--same-row` against s1 equal (ALTVCHID 68, ALTMSTID 245); T 28 rows, `BRIDGE Sales Keyed` 0; each of the five answers' sha256 equal to s2's (`evidence/compare-s9.txt`).

**The sheet's estimates:** s3r as row 2162-KD's s3r (Manual, `PREVENTDUPLICATES` Yes): as estimated, and byte-equal. s4 created: as estimated.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **The start-up company.** Tally's start-up company loaded by itself, as the install is set to. It was shut by the K: Company label, Shut, Enter on it, before step 0, on the run lead's standing practice for these sittings, with no change to it. One Enter there was refused by the desktop-control tool because the desktop client had the keyboard focus; no key reached Tally. A click on Tally's title bar and a capture showed the screen unchanged before the Enter was pressed.

**Beyond the sheet, recorded and not judged:**
- Tally showed the backup path as `C:\lab\2162-St\backup-1` at BK and at the restore (the `s` after the hyphen raised). Windows paths ignore case; the folder is the one made empty at step 1, and it held one file.
- The Gateway's company line carried "(e)" and "(Schedule Backup)" at every reading.
- s2's T answer is byte-equal to rows 2162-KD's and 2346-MN's s2 T answers (sha256 cfd36272...).

## Step 1: load, check and back up
- Sheet files: all 14 equal to `SHA256.txt` at b6636e0b (sizes too). `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml`, `run.py`, `T-types.xml` and `N-0010s.xml` are byte-equal to row 2346-MN's; `R-0010.xml` and `template-import-0010.utf16le.xml` to row 2346-AR's; `template-full-1.utf16le.xml` to row 2162-KD's. `gen.py` was run again in a new folder with copies of its three inputs: `I-0010k.xml` and `V-0010.xml` byte-equal to the sheet's. `count22.py` differs from row 2346-MN's `count21.py` in its two ST lines and two header lines only.
- Before the start, the book's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`). The backup folder was made new and empty.
- Tally was started once, at 05:11:03Z, from its desktop shortcut, after Get-Process showed none. Its start-up company was shut first (deviation 2).
- C2 at 05:12:20Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE`, ` PILOT` and ` LAB` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s1-select`), Enter, no prompt. The Gateway (`s1-gateway`): current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (not before 1-Aug-2026); date of last entry 15-Sep-26.
- `s1`: one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245. EXPECT met.
- BK by C5 (05:13Z to 05:15Z) to `C:\lab\2162-st\backup-1`: Data label, Backup & Restore, Backup; Backspace to the path; Specify Path; the path typed in two chunks; Enter, Enter; `BRIDGE`, ` PILOT` and ` LAB` typed, the one row, Enter; End of List, Enter; "Accept ?" (`bk-accept`, 05:14:46Z) = y. One file, 90,051 B; the Restore list showed it as 10-Nov-26 (10:44 am).

## Step 2: the book before
- `s2` (05:15:04Z): T 28 rows; `BRIDGE Sales` one series `Default`, `Manual`, `PREVENTDUPLICATES` Yes, GUID tail `000000ce`; `BRIDGE Sales Keyed` 0 rows. N one row, `BP/26-27/0010` on `BRIDGE Sales`, `BRIDGEVCHISSALES` Yes. V one row, `BRIDGE Sales`, GUID tail `0000000b`, MASTERID 11. R one row. M as s1. EXPECT met.

## Step 3: the type keyed
- `s3m` and the before-write check against s1 passed (`evidence/bw-s3m.txt`).
- From the Gateway: Create, Enter; List of Masters, Down 3 to Voucher Type, Enter; Voucher Type Creation. `BRIDGE`, space, `Sales`, space, `Keyed` typed, read by zoom, Enter; (alias) Enter; `Sales` typed in the List of Voucher Types (two rows, Sales and Sales Order), Enter; Abbreviation as Tally filled it (`Sale`), Enter; Activate (Yes), Enter; Method of Voucher Numbering from "Methods of Numbering" (Automatic, Automatic (Manual Override), Manual, Multi-user Auto, None), Down 2 to Manual, read by zoom, Enter. Once Manual was chosen Tally offered "Prevent creating duplicate Voucher Nos." showing No: y, Enter: Yes. No other field touched.
- Every field and value as text: `evidence/s3-screens.txt` (each field as row 2162-KD's); the screen before the save: `s3-vt-before-save`.
- The save: Ctrl+A, no prompt; a new blank Voucher Type Creation screen (`s3-after-save-blank`); Ctrl+Q to Master Creation's List of Masters (`s3-master-creation`); Ctrl+Q to the Gateway, no dialog (`s3-gateway`).
- `s3r` (05:19:36Z): as in the Result (`evidence/typediff-s3r.txt`). EXPECT met.

## Step 4: the number another Sales type holds
- `s4m` and the before-write check against s1 passed (`evidence/bw-s4m.txt`).
- `s4` (05:20:22Z): `I-0010k.xml` sent once; the window captured at once (`s4-after-import`, 05:20:23Z): the Gateway, no dialog.
- `s4r` (05:20:40Z): `N-0010s.xml V-0010.xml R-0010.xml M-marks.xml`, as in the Result.

## Step 5: restore (C6) and compare
- `prep-c6.sh 5` first: BK re-hashed equal to `evidence/bk.txt`, its folder holding only that file; `restore-plan.txt` written; the data folder listed (kept on the lab PC).
- K: Company label, Shut, Enter on BRIDGE PILOT LAB, no prompt; Data label, Backup & Restore, Restore; Specify Path; the path typed in two chunks; the list showed one row, BRIDGE PILOT LAB, 10-Nov-26 (10:44 am), as the plan; Enter on it; End of List, Enter; "Accept ?" (`c6-accept`, 05:22:53Z) = y; "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (`c6-replace`, 05:23:13Z) = r. No other message.
- The book loaded alone (`c6-gateway`): period, date and last entry as at step 1.
- `s9` (05:24Z): as in the Result (`evidence/sr-s9.txt`, `evidence/compare-s9.txt`).

## Step 6: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 05:24:58Z).
- C2 at 05:24:58Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe` (05:25:24Z). The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count22.py` output and compare, the screen's fields as text, the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listings stay on the lab PC; the BK record (`bk.txt`) goes with the company number replaced.
- `logs/`: `run.py`'s log of each send.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count22.py outputs, whole

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
== a-s2-N-0010s.xml: 5604 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Sales' BRIDGEVCHISSALES 'Yes'
  rows with no class element: 0
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
   bills on 'BRIDGE Walk-in': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
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
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]@TYPE 'String'
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]/LASTSERIESBYGSTTAXUNIT[1] 'Default_BRIDGE Sales'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]@TYPE 'String'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]/LASTGSTTAXUNITBYSERIES[1] 'Rajasthan Registration'
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 0
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

### s3r
```
== a-s3r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3r-T-types.xml: 319886 bytes
  STATUS '1'
  VOUCHERTYPE rows: 29
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]@TYPE 'String'
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]/LASTSERIESBYGSTTAXUNIT[1] 'Default_BRIDGE Sales'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]@TYPE 'String'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]/LASTGSTTAXUNITBYSERIES[1] 'Rajasthan Registration'
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
```

### s4m
```
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s4
```
== a-s4-I-0010k.xml: 540 bytes
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
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4r-N-0010s.xml: 8206 bytes
  STATUS '1'
  VOUCHER rows: 2 of which with a VOUCHERNUMBER: 2 without one: 0
   'BP/26-27/0010' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Sales' BRIDGEVCHISSALES 'Yes'
   'BP/26-27/0010' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Sales Keyed' BRIDGEVCHISSALES 'Yes'
  rows with no class element: 0
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s4r-V-0010.xml: 102590 bytes
  STATUS '1'
  VOUCHER rows: 2
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales Keyed'
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

### s9
```
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s9-N-0010s.xml: 5604 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Sales' BRIDGEVCHISSALES 'Yes'
  rows with no class element: 0
== a-s9-R-0010.xml: 94474 bytes
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
   bills on 'BRIDGE Walk-in': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s9-T-types.xml: 306946 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]@TYPE 'String'
   unit leaf /LASTSERIESBYGSTTAXUNIT.LIST[1]/LASTSERIESBYGSTTAXUNIT[1] 'Default_BRIDGE Sales'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]@TYPE 'String'
   unit leaf /LASTGSTTAXUNITBYSERIES.LIST[1]/LASTGSTTAXUNITBYSERIES[1] 'Rajasthan Registration'
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 0
== a-s9-V-0010.xml: 52784 bytes
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

## Compares, checks and screens, whole

### step 3: the Voucher Type Creation screen before the save, as text
```
Step 3, Voucher Type Creation (BRIDGE PILOT LAB), before saving; capture shots/s3-vt-before-save.png. Every label and value as on the screen:
Name : BRIDGE Sales Keyed
(alias) : (blank)
General
  Select type of voucher : Sales
  Abbreviation : Sale  (filled by Tally when Sales was chosen; kept)
  Activate this Voucher Type : Yes
  Method of Voucher Numbering : Manual
    Prevent creating duplicate Voucher Nos. : Yes  (the field appeared under Manual once Manual was chosen; Tally showed No, set to Yes)
  Show unused vch nos. in transactions for Retain Original Voucher No. behaviour : No  (greyed)
  Use effective dates for vouchers : No
  Allow zero-valued transactions : No
  Make this voucher type as 'Optional' by default : No
  Allow narration in voucher : Yes
  Provide narrations for each ledger in voucher : No
  Enable default accounting allocations : No
  WhatsApp voucher after saving : No
Printing
  Print voucher after saving : No
  Use for POS invoicing : No
  Default template to print : Not Applicable
  Default title to print : (blank)
  Default bank : Not Applicable
  Default jurisdiction : (blank)
  Set/alter declaration : No
  Set/Alter Terms and Conditions : No
Name of Class : (empty)
Right-hand buttons: F2 Period (greyed), F3 Company, F10 Other Masters, I More Details, F12 Configure. Bottom: Q Quit, A Accept, D Delete (greyed).
Before the change: with Abbreviation filled, "Show unused vch nos. ..." read Yes (greyed) while the method was empty; after Manual it reads No.
Path and keys: at the Gateway, Create (highlighted) Enter; List of Masters, Down 3 to Voucher Type, Enter; BRIDGE, space, Sales, space, Keyed typed
(zoomed and checked), Enter; (alias) Enter; "Sales" typed in List of Voucher Types (two rows: Sales, Sales Order), Enter; Abbreviation Enter;
Activate Enter; Methods of Numbering list (Automatic, Automatic (Manual Override), Manual, Multi-user Auto, None): Down 2 to Manual (checked), Enter;
Prevent creating duplicate Voucher Nos.: y, Enter. Every field and value is as row 2162-KD's s3-screens.txt.
```

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3r: T against s2 (--type-diff)
```
== --type-diff a-s2-T-types.xml a-s3r-T-types.xml: VOUCHERTYPE rows 28 -> 29
   type 'BRIDGE Sales': 0 leaves differ
   type 'BRIDGE Sales Keyed': in the second only
exit 0
```

### s4m: M against s1 (the before-write check)
```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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

### s4r: the R-0010 lines that differ from s2
```
$ the R-0010 answer at s4r against s2's: the lines that differ (UTF-16LE decoded; line numbers of the answer)
lines: s2 1311, s4r 1311; CMPINFO is lines 8 to 44 in both
line 11: s2 <LEDGER>0</LEDGER> | s4r <LEDGER>11</LEDGER>
line 18: s2 <VOUCHERTYPE>0</VOUCHERTYPE> | s4r <VOUCHERTYPE>1</VOUCHERTYPE>
line 19: s2 <CURRENCY>0</CURRENCY> | s4r <CURRENCY>1</CURRENCY>
line 39: s2 <TAXUNIT>0</TAXUNIT> | s4r <TAXUNIT>1</TAXUNIT>
line 42: s2 <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES> | s4r <VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>
line 43: s2 <VOUCHER>0</VOUCHER> | s4r <VOUCHER>2</VOUCHER>
every other line equal
```

### s9: M against s1 (--same-row)
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### s9: every answer against s2 by sha256
```
$ each answer of s9 against the same request's answer at s2, by sha256
T-types.xml: s2 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed, s9 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed: equal
N-0010s.xml: s2 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85, s9 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85: equal
V-0010.xml: s2 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456, s9 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456: equal
R-0010.xml: s2 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5, s9 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5: equal
M-marks.xml: s2 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6, s9 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6: equal
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every read answer that has them (LAB-COMMON C4: not rows); the names with a count other than 0, then the rest as "others 0"
a-s1-M-marks.xml: all 0 (names in all: 35)
a-s2-T-types.xml: all 0 (names in all: 35)
a-s2-N-0010s.xml: all 0 (names in all: 35)
a-s2-V-0010.xml: all 0 (names in all: 35)
a-s2-R-0010.xml: all 0 (names in all: 35)
a-s2-M-marks.xml: all 0 (names in all: 35)
a-s3m-M-marks.xml: all 0 (names in all: 35)
a-s3r-T-types.xml: VOUCHERTYPE 1; others 0 (names in all: 35)
a-s3r-M-marks.xml: VOUCHERTYPE 1; others 0 (names in all: 35)
a-s4m-M-marks.xml: VOUCHERTYPE 1; others 0 (names in all: 35)
a-s4r-N-0010s.xml: LEDGER 11; VOUCHERTYPE 1; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4r-V-0010.xml: LEDGER 11; VOUCHERTYPE 1; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4r-R-0010.xml: LEDGER 11; VOUCHERTYPE 1; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4r-M-marks.xml: LEDGER 11; VOUCHERTYPE 1; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s9-T-types.xml: all 0 (names in all: 35)
a-s9-N-0010s.xml: all 0 (names in all: 35)
a-s9-V-0010.xml: all 0 (names in all: 35)
a-s9-R-0010.xml: all 0 (names in all: 35)
a-s9-M-marks.xml: all 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s2-N-0010s.xml 5604 B 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85
a-s2-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3r-T-types.xml 319886 B b1d08c994aa6b5fd9eb54a4ac8a95698fd7de5e1e6c06659d80a6acb801c5e6a
a-s3r-M-marks.xml 3394 B 913425f72bab50b0f29ac48c7c650ecacb0bebab076434ab524ec85521561196
a-s4m-M-marks.xml 3394 B 913425f72bab50b0f29ac48c7c650ecacb0bebab076434ab524ec85521561196
a-s4-I-0010k.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s4r-N-0010s.xml 8206 B 01086f928399b4ed0bc316c453dd4e08f393e123a201967d557076707ba5100d
a-s4r-V-0010.xml 102590 B 9a86111aa2e9366bf2a629c33ca6b4384e6a068c1ac3631455f432317220df5d
a-s4r-R-0010.xml 94476 B c4961c67c8be536617cd4dc7c956a9852a7909719356a5f394be198292a9f4c8
a-s4r-M-marks.xml 3396 B a8e6758b2fadca0c02548a66880d46206695e4174ec76a6b2f68b3b0f3e6a402
a-s9-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s9-N-0010s.xml 5604 B 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85
a-s9-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s9-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s9-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
```

### each send's start, UTC
```
2026-11-10T05:13:05.965Z s1 M-marks.xml (hashes equal to the lists)
2026-11-10T05:15:04.866Z s2 T-types.xml N-0010s.xml V-0010.xml R-0010.xml M-marks.xml (hashes equal to the lists)
2026-11-10T05:15:32.156Z s3m M-marks.xml (hashes equal to the lists)
2026-11-10T05:19:36.557Z s3r T-types.xml M-marks.xml (hashes equal to the lists)
2026-11-10T05:20:07.292Z s4m M-marks.xml (hashes equal to the lists)
2026-11-10T05:20:17.237Z s4 I-0010k.xml (hashes equal to the lists)
2026-11-10T05:20:34.206Z s4r N-0010s.xml V-0010.xml R-0010.xml M-marks.xml (hashes equal to the lists)
2026-11-10T05:24:09.085Z s9 T-types.xml N-0010s.xml V-0010.xml R-0010.xml M-marks.xml (hashes equal to the lists)
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
10:43:06	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
10:45:05	T-types.xml	a-s2-T-types.xml	curl 0	306946 bytes
10:45:05	N-0010s.xml	a-s2-N-0010s.xml	curl 0	5604 bytes
10:45:05	V-0010.xml	a-s2-V-0010.xml	curl 0	52784 bytes
10:45:05	R-0010.xml	a-s2-R-0010.xml	curl 0	94474 bytes
10:45:06	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3m
```
10:45:32	M-marks.xml	a-s3m-M-marks.xml	curl 0	3394 bytes
```

### s3r
```
10:49:37	T-types.xml	a-s3r-T-types.xml	curl 0	319886 bytes
10:49:37	M-marks.xml	a-s3r-M-marks.xml	curl 0	3394 bytes
```

### s4m
```
10:50:07	M-marks.xml	a-s4m-M-marks.xml	curl 0	3394 bytes
```

### s4
```
10:50:17	I-0010k.xml	a-s4-I-0010k.xml	curl 0	540 bytes
```

### s4r
```
10:50:34	N-0010s.xml	a-s4r-N-0010s.xml	curl 0	8206 bytes
10:50:34	V-0010.xml	a-s4r-V-0010.xml	curl 0	102590 bytes
10:50:35	R-0010.xml	a-s4r-R-0010.xml	curl 0	94476 bytes
10:50:35	M-marks.xml	a-s4r-M-marks.xml	curl 0	3396 bytes
```

### s9
```
10:54:09	T-types.xml	a-s9-T-types.xml	curl 0	306946 bytes
10:54:09	N-0010s.xml	a-s9-N-0010s.xml	curl 0	5604 bytes
10:54:09	V-0010.xml	a-s9-V-0010.xml	curl 0	52784 bytes
10:54:09	R-0010.xml	a-s9-R-0010.xml	curl 0	94474 bytes
10:54:10	M-marks.xml	a-s9-M-marks.xml	curl 0	3394 bytes
```
