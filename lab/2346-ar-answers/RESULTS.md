# Row 2346-AR: a Credit Note's Agst Ref settled the open invoice bill; the same note against the settled bill was created with no error and stored as sent; a note against a bill the customer does not hold was created with its bill stored as a New Ref

Run on 10 Nov 2026, 03:14:51Z to 03:25:45Z (08:44 to 08:55 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as row 2464-GN read it from the program's file; the Gateway shows no release number), by the sheet at `lab/2346-ar-sheet` (fd0aef60), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no Bridge build. Five imports, each sent once: the set-up ledger and invoice, then the three notes. Then the restore of this sitting's own backup. Nothing was keyed in Tally but the backup and the restore. Every value below is M, measured by this run, in one run. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1 (B's report carries no STATUS element).

## Result

Each is PARTIAL by the sheet's Decides: one import of each shape, one sitting, one book, one release.

| import | sent | `CREATED` | `ERRORS` | `EXCEPTIONS` | `LASTVCHID` | LINEERROR |
|---|---|---|---|---|---|---|
| s3, `L-ledger.xml` (set-up) | once | 1 | 0 | 0 | 0 | none |
| s4, `I-0099.xml` (set-up) | once | 1 | 0 | 0 | 45 | none |
| s5, `I-CN-1.xml` (c1) | once | 1 | 0 | 0 | 46 | none |
| s6, `I-CN-2.xml` (c2) | once | 1 | 0 | 0 | 47 | none |
| s7, `I-CN-3.xml` (c3) | once | 1 | 0 | 0 | 48 | none |

`ALTERED`, `DELETED`, `LASTMID`, `COMBINED`, `IGNORED` and `CANCELLED` were 0 in all five answers. The window after each import was the Gateway, no dialog.

**c1, the note against the open bill: "Stored as sent and the bill settled."**
- R-CN1: Credit Note number `1`, date 20260801, GUID tail `0000002e`, ALTERID 70; included `Yes`, uncertain `No`, excluded `No`, cancelled `No`.
- Its six legs are the invoice's, each turned over. The `BRIDGE Walk-in BC` leg holds one bill with `NAME`: `BP/26-27/0099`, `Agst Ref`, `1051.00`, BILLDATE and BILLCREATIONDATE 20260801, BILLID 2 (the invoice's bill's BILLID). As sent.
- B: no group for `BRIDGE Walk-in BC`, and none in all (46 B, byte-equal to s2's). The bill is gone from Bills Receivable.
- **The invoice did not move.** `--diff` of R-0099 at s4r against s5r: lists compared 1, legs 6 -> 6, no field differs. Its ALTERID stayed 69 and its New Ref `-1051.00`. The two answers differ in DESC/CMPINFO count lines only.

**c2, the same note against the settled bill: created, stored as sent, with no group back in B.**
- R-CN2: Credit Note number `2`, GUID tail `0000002f`, ALTERID 71, the same flags and legs as c1's. The `BRIDGE Walk-in BC` leg holds one bill with `NAME`: `BP/26-27/0099`, `Agst Ref`, `1051.00`, BILLID 2: the bill c1 settled.
- No error, no exception, no LINEERROR, no dialog.
- B: no group for `BRIDGE Walk-in BC` and none in all, byte-equal to s5r's. So the first half of the sheet's "Created, with a group for `BP/26-27/0099` back in B" was not seen.
- **Not established here:** whether the bill now holds a credit. B is the Bills Receivable read, and this row's reads hold nothing else that shows a bill's balance.
- c1's note and the invoice did not move. `--diff` of each at s5r against s6r: lists compared 1, legs 6 -> 6, no field differs.

**c3, the note against a bill the customer does not hold: "stored as another type".**
- R-CN3: Credit Note number `3`, GUID tail `00000030`, ALTERID 72, the same flags and legs as c1's.
- The request named `Agst Ref` `BP/26-27/0098`. The `BRIDGE Walk-in BC` leg holds one bill with `NAME`: `BP/26-27/0098`, BILLTYPE **`New Ref`**, `1051.00`, BILLDATE and BILLCREATIONDATE 20260801, BILLID 3.
- No error, no exception, no LINEERROR, no dialog.
- B: no group for `BRIDGE Walk-in BC` and none in all, byte-equal to s6r's. **`BP/26-27/0098` is not among B's groups.**
- c1's note and the invoice did not move. `--diff` of each at s6r against s7r: lists compared 1, legs 6 -> 6, no field differs.
- The sheet's line for this case: "Stored as another bill type: record it. The read-back's check would read it as divergent, after the post."

**c2 was sent** because c1 met the sheet's three tests: c1 was created; R-CN1's customer leg held one bill `BP/26-27/0099`, `Agst Ref`, `1051.00`; B held no `BP/26-27/0099` group for the customer. R-CN2 was not read again at s7r, as c2 was sent and took the number 2.

**The set-up held**, as S3a's s5 read:
- s3 created `BRIDGE Walk-in BC` and s4 created `BP/26-27/0099`.
- s4r: N-0099 one row. R-0099 one row: GUID tail `0000002d`, ALTERID 69, six legs as sent. The `BRIDGE Walk-in BC` leg held one bill with `NAME`: `BP/26-27/0099`, `New Ref`, `-1051.00`, BILLID 2. Each other leg held one whitespace-only `BILLALLOCATIONS.LIST`.
- B (510 B): one group in all, for `BRIDGE Walk-in BC`: `BP/26-27/0099`, `1-Aug-26`, BILLCL `-1051.00`.

**The allocation fetch's cost.** Each R read that found a voucher answered 95,834 B (95,832 B for R-CN3, whose bill name differs by one character). That equals S3a's one-voucher read-back. A read that found none answered 3,022 B. B answered 46 B with no bill and 510 B with one. `run.py`'s log gives each send's start to the second. In every label but s6r, all of the label's sends start in the same second. In s6r (five reads, three of them read-backs of 95,834 B), they start across two seconds (logs below).

- **s1:** one COMPANY row, the book's: ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **BK:** taken before anything was sent but s1; restored at step 8.
- **s2, the book before:** every EXPECT met. N-0099 0 rows; R-0099, R-CN1, R-CN2 and R-CN3 0 rows each (3,022 B each, byte-equal to each other). B: 46 B, 0 groups in all, 0 for `BRIDGE Walk-in BC`.
- **The marks:** each import of a voucher raised ALTVCHID by one, from 68 (s1) to 72 (s7r). The ledger raised ALTMSTID from 245 to 246; the notes left it at 246.
- **Step 8, the restore:** by C6, with `restore-plan.txt` written first; the two expected prompts. s8: M equal to s1 by `--same-row` (68/245). N-0099, R-0099 and each R-CN 0 rows; B 0 groups. Every s8 answer's sha256 equal to s2's. The book is as it was found.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.

**Beyond the sheet, recorded and not judged:**
- The Gateway's company line carried "(e)" with the footnote "(e) Data exceptions exist", at step 0 and after the reload at step 8 alike.
- The DESC/CMPINFO count lines moved after the set-up and again after each note; the M read just before each note shows the same counts as the reads after the import before it. They were 0 again after the restore (LAB-COMMON C4: these are not rows):

  | R-0099's read | `LEDGER` | `CURRENCY` | `TAXUNIT` | `VOUCHERNUMBERSERIES` | `VOUCHER` |
  |---|---|---|---|---|---|
  | s2 | 0 | 0 | 0 | 0 | 0 |
  | s4r | 12 | 1 | 1 | 1 | 2 |
  | s5r | 23 | 2 | 2 | 2 | 4 |
  | s6r | 34 | 3 | 3 | 3 | 6 |
  | s7r | 45 | 4 | 4 | 4 | 8 |
  | s8 | 0 | 0 | 0 | 0 | 0 |

  Every answer's counts are listed in `evidence/cmpinfo.txt`. Every `--diff` of the same reads shows no field moved.
- `count20.py`'s header comment at its line 77 still names `BRIDGE Customer RJ` as the default ledger of the C- and Q- kinds; its code's default is `BRIDGE Walk-in BC` for every kind. This row sends no C- or Q- read.

## Step 1: load, check and back up
- Sheet files: all 24 equal to `SHA256.txt` at fd0aef60. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` and the two templates are byte-equal to row 2464-GN's. `I-CN.xml` and `R-CN1.xml` are byte-equal to row 2346-CD's sheet. `B-bills.xml`, `template-s3a-L-ledger.xml` and `template-s3a-I-0047.xml` are byte-equal to row 2162-S3a's sheet. `gen.py`, rerun in a scratch copy, writes its 9 requests byte-equal to the sheet's. `count20.py` differs from row 2464-GN's `count19.py` only by its header and the lines marked AR.
- Before the start, the company's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 03:14:51Z, from its desktop shortcut, after Get-Process showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter, no prompt).
- C2 at 03:15:50Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded.
- The book was loaded alone: `BRIDGE` and ` PILOT` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026, not before 1-Aug-2026; date of last entry 15-Sep-26.
- `s1` (03:16:35Z): as in the Result.
- **BK** (C5), by row 2346-CD's path, to a new folder:
  - Before the backup, `C:\lab\2346-ar\backup-1` was made new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times, checked), Enter, then `C:\lab\2346-ar\backup-1` typed in three chunks (Tally showed it as `C:\lab\2346-Ar\backup-1`, its own capital), and its row taken.
  - BRIDGE PILOT LAB alone, End of List (Up 3 times, checked). "Accept ?" was captured (`bk-accept`, 03:18:18Z) and answered y; no other message; the Gateway again.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `aa61b2c280a97b2e06561efd4f2943c6e638c1a1dc71ac9db234c9519f08e7c0`, written at 08:48 IST.

## Step 2: the book before
`s2` (03:18:39Z): `N-0099.xml R-0099.xml R-CN1.xml R-CN2.xml R-CN3.xml B-bills.xml M-marks.xml`, each request checked against `SHA256.txt` just before its send. Every EXPECT met, as in the Result.

## Step 3: the set-up
- `s3m` (03:18:58Z): M 68/245; the before-write check against s1 passed (one COMPANY row, the same GUID). No key had been pressed in Tally since BK's y.
- `s3` (03:19:06Z): `L-ledger.xml` sent once, answered as in the table; the window after it the Gateway, no dialog (`s3-after`).
- `s4` (03:19:15Z): `I-0099.xml` sent once, answered as in the table; the window after it the Gateway, no dialog (`s4-after`).
- `s4r` (03:19:27Z): `N-0099.xml R-0099.xml B-bills.xml M-marks.xml`. The set-up held, as in the Result. M 69/246.

## Steps 4 to 6: the three notes
- `s5m` (03:19:39Z): M 69/246; the before-write check against s1 passed.
- `s5` (03:19:47Z): `I-CN-1.xml` sent once; answered as in the table; the window after it the Gateway, no dialog (`s5-after`, 03:19:51Z).
- `s5r` (03:19:59Z): `R-CN1.xml R-0099.xml B-bills.xml M-marks.xml`. As in the Result. M 70/246.
- `s6m` (03:20:13Z): M 70/246; the before-write check against s1 passed.
- `s6` (03:20:14Z): `I-CN-2.xml` sent once; answered as in the table; the window after it the Gateway, no dialog (`s6-after`).
- `s6r` (03:20:30Z): `R-CN2.xml R-CN1.xml R-0099.xml B-bills.xml M-marks.xml`. As in the Result. M 71/246.
- `s7m` (03:21:13Z): M 71/246; the before-write check against s1 passed.
- `s7` (03:21:14Z): `I-CN-3.xml` sent once; answered as in the table; the window after it the Gateway, no dialog (`s7-after`).
- `s7r` (03:21:30Z): `R-CN3.xml R-CN1.xml R-0099.xml B-bills.xml M-marks.xml`. As in the Result. M 72/246.
- Step 7: nothing was sent or pressed between s7r and the C6 prep.

## Step 8: restore (C6) and compare
- 03:21:49Z, before the shut: `prep-c6.sh 7` re-hashed BK equal to `evidence/bk.txt`, found its folder holding only that file, wrote `restore-plan.txt` (kept on the lab PC, as it names the company number) and listed the Tally data folder (kept on the lab PC).
- The book shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded.
- The Data label, Backup & Restore, Restore: Restore Companies, Restore Data Path the Tally data folder; Specify Path (Up 3, checked); `C:\lab\2346-ar\backup-1` typed in three chunks (shown `C:\lab\2346-Ar\backup-1`); Enter: one company row, BRIDGE PILOT LAB, its number, 10-Nov-26 (8:48 am), as the plan says (`restore-list`, 03:23:11Z); Enter on it; Enter on End of List.
- "Accept ?" (`restore-accept`, 03:23:35Z): `y`. "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip (`restore-replace`, 03:23:51Z): `r`. The restore ran and Tally returned to Select Company with no other prompt.
- C2 at 03:24:12Z, with Select Company showing: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200.
- The book reloaded alone by the filter (`s8-gateway`): the Gateway as at step 0, the same period and date.
- `s8` (03:24:50Z): `M-marks.xml N-0099.xml R-0099.xml R-CN1.xml R-CN2.xml R-CN3.xml B-bills.xml`. Every compare equal, and every answer's sha256 equal to s2's.

## Step 9: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 03:25:24Z).
- C2 at 03:25:25Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`. The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count20.py` output and compare, the answers' sizes and sha256. Process and port listings, the backup-folder listing and the data-folder listings stay on the lab PC.
- `logs/`: `run.py`'s log of each send.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count20.py outputs, whole

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
== a-s2-B-bills.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s2-N-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-R-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-CN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-CN2.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-CN3.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
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
== a-s3-L-ledger.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### s4
```
== a-s4-I-0099.xml: 540 bytes
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
== a-s4r-B-bills.xml: 510 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '-1051.00'
== a-s4r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4r-N-0099.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0099' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s4r-R-0099.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
```

### s5m
```
== a-s5m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s5
```
== a-s5-I-CN-1.xml: 540 bytes
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

### s5r
```
== a-s5r-B-bills.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s5r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5r-R-0099.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5r-R-CN1.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
```

### s6m
```
== a-s6m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s6
```
== a-s6-I-CN-2.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '47'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### s6r
```
== a-s6r-B-bills.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s6r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6r-R-0099.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s6r-R-CN1.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s6r-R-CN2.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '2'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
```

### s7m
```
== a-s7m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s7
```
== a-s7-I-CN-3.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '48'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### s7r
```
== a-s7r-B-bills.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s7r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7r-R-0099.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s7r-R-CN1.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s7r-R-CN3.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '3'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   ALTERID '72'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0098'; BILLTYPE 'New Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
```

### s8
```
== a-s8-B-bills.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s8-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8-N-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s8-R-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s8-R-CN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s8-R-CN2.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s8-R-CN3.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
```

## Compares and checks, whole

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s5r: R-0099 against s4r (--diff)
```
== --diff a-s4r-R-0099.xml a-s5r-R-0099.xml
   lists compared: 1
   legs: 6 -> 6
```

### s6m: M against s1 (the before-write check)
```
== a-s6m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s6r: R-CN1 and R-0099 against s5r (--diff)
```
== --diff a-s5r-R-CN1.xml a-s6r-R-CN1.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
== --diff a-s5r-R-0099.xml a-s6r-R-0099.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s5r and s6r: the lines that differ from the earlier answer of the same read
```
a-s4r-R-0099.xml against a-s5r-R-0099.xml: lines a=1328, b=1328
  line 11: a <LEDGER>12</LEDGER> / b <LEDGER>23</LEDGER>
  line 19: a <CURRENCY>1</CURRENCY> / b <CURRENCY>2</CURRENCY>
  line 39: a <TAXUNIT>1</TAXUNIT> / b <TAXUNIT>2</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES>
  line 43: a <VOUCHER>2</VOUCHER> / b <VOUCHER>4</VOUCHER>
a-s5r-R-0099.xml against a-s6r-R-0099.xml: lines a=1328, b=1328
  line 11: a <LEDGER>23</LEDGER> / b <LEDGER>34</LEDGER>
  line 19: a <CURRENCY>2</CURRENCY> / b <CURRENCY>3</CURRENCY>
  line 39: a <TAXUNIT>2</TAXUNIT> / b <TAXUNIT>3</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>3</VOUCHERNUMBERSERIES>
  line 43: a <VOUCHER>4</VOUCHER> / b <VOUCHER>6</VOUCHER>
a-s5r-R-CN1.xml against a-s6r-R-CN1.xml: lines a=1328, b=1328
  line 11: a <LEDGER>23</LEDGER> / b <LEDGER>34</LEDGER>
  line 19: a <CURRENCY>2</CURRENCY> / b <CURRENCY>3</CURRENCY>
  line 39: a <TAXUNIT>2</TAXUNIT> / b <TAXUNIT>3</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>3</VOUCHERNUMBERSERIES>
  line 43: a <VOUCHER>4</VOUCHER> / b <VOUCHER>6</VOUCHER>
a-s5r-B-bills.xml against a-s6r-B-bills.xml: lines a=1, b=1
```

### s7m: M against s1 (the before-write check)
```
== a-s7m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s7r: R-CN1 and R-0099 against s6r (--diff)
```
== --diff a-s6r-R-CN1.xml a-s7r-R-CN1.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
== --diff a-s6r-R-0099.xml a-s7r-R-0099.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s7r: the lines that differ from the earlier answer of the same read
```
a-s6r-R-0099.xml against a-s7r-R-0099.xml: lines a=1328, b=1328
  line 11: a <LEDGER>34</LEDGER> / b <LEDGER>45</LEDGER>
  line 19: a <CURRENCY>3</CURRENCY> / b <CURRENCY>4</CURRENCY>
  line 39: a <TAXUNIT>3</TAXUNIT> / b <TAXUNIT>4</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>3</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>4</VOUCHERNUMBERSERIES>
  line 43: a <VOUCHER>6</VOUCHER> / b <VOUCHER>8</VOUCHER>
a-s6r-R-CN1.xml against a-s7r-R-CN1.xml: lines a=1328, b=1328
  line 11: a <LEDGER>34</LEDGER> / b <LEDGER>45</LEDGER>
  line 19: a <CURRENCY>3</CURRENCY> / b <CURRENCY>4</CURRENCY>
  line 39: a <TAXUNIT>3</TAXUNIT> / b <TAXUNIT>4</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>3</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>4</VOUCHERNUMBERSERIES>
  line 43: a <VOUCHER>6</VOUCHER> / b <VOUCHER>8</VOUCHER>
a-s6r-B-bills.xml against a-s7r-B-bills.xml: lines a=1, b=1
```

### s8: M against s1 (--same-row), and every answer against s2 by sha256
```
== a-s8-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
$ sha256 of each s8 answer against s2's
M-marks: s2 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6, s8 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6, equal
N-0099: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s8 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, equal
R-0099: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s8 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, equal
R-CN1: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s8 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, equal
R-CN2: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s8 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, equal
R-CN3: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s8 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, equal
B-bills: s2 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3, s8 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3, equal
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every read answer that has them (LAB-COMMON C4: not rows); the names with a count other than 0, then the rest as "others 0"
a-s1-M-marks.xml: all 0 (names in all: 35)
a-s2-N-0099.xml: all 0 (names in all: 35)
a-s2-R-0099.xml: all 0 (names in all: 35)
a-s2-R-CN1.xml: all 0 (names in all: 35)
a-s2-R-CN2.xml: all 0 (names in all: 35)
a-s2-R-CN3.xml: all 0 (names in all: 35)
a-s2-M-marks.xml: all 0 (names in all: 35)
a-s3m-M-marks.xml: all 0 (names in all: 35)
a-s4r-N-0099.xml: LEDGER 12; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4r-R-0099.xml: LEDGER 12; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4r-M-marks.xml: LEDGER 12; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s5m-M-marks.xml: LEDGER 12; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s5r-R-CN1.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s5r-R-0099.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s5r-M-marks.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s6m-M-marks.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s6r-R-CN2.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s6r-R-CN1.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s6r-R-0099.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s6r-M-marks.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s7m-M-marks.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s7r-R-CN3.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s7r-R-CN1.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s7r-R-0099.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s7r-M-marks.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s8-M-marks.xml: all 0 (names in all: 35)
a-s8-N-0099.xml: all 0 (names in all: 35)
a-s8-R-0099.xml: all 0 (names in all: 35)
a-s8-R-CN1.xml: all 0 (names in all: 35)
a-s8-R-CN2.xml: all 0 (names in all: 35)
a-s8-R-CN3.xml: all 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-N-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-CN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-CN2.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-CN3.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3-L-ledger.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s4-I-0099.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s4r-N-0099.xml 5608 B b04a2cd7467a4f966ef4daa1960d8acec8466423321e240f718e91afad863f30
a-s4r-R-0099.xml 95834 B d8c5bd127f294878280b9f5154f213924c3d2a43a5eae14199365755296d81c1
a-s4r-B-bills.xml 510 B 592c89acfa1452bab5084a10c9b59126e94fcf83a7859f7dfc600f40228cfcaa
a-s4r-M-marks.xml 3396 B 17c5bde53bf5e459f4ae36fce8ee62e8bfe5eb636f47f1970faff44c230a59ed
a-s5m-M-marks.xml 3396 B 17c5bde53bf5e459f4ae36fce8ee62e8bfe5eb636f47f1970faff44c230a59ed
a-s5-I-CN-1.xml 540 B 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
a-s5r-R-CN1.xml 95834 B 7e24d52766d92e78c129ea2556bbb62bc29dc350cc2ae22f07d8c85d1bd78b33
a-s5r-R-0099.xml 95834 B 12f34e988ae0f75ddd957b55872a60fba98e3d7a018fd4f12439d773554db974
a-s5r-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s5r-M-marks.xml 3396 B d55cf90ef81a5c355be9e694f8fae317da2e1701fa95feb6f4cc446cb4a7553e
a-s6m-M-marks.xml 3396 B d55cf90ef81a5c355be9e694f8fae317da2e1701fa95feb6f4cc446cb4a7553e
a-s6-I-CN-2.xml 540 B f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e
a-s6r-R-CN2.xml 95834 B 312b4d8a628f3636bcdcf51b8c5e818b9fead1ca41d6f02809d11dabe2637b9c
a-s6r-R-CN1.xml 95834 B 7fc9f067b46983b2e3f007aee5746e15d91c45b7541bdc5a9cf771d6600ffa98
a-s6r-R-0099.xml 95834 B 23568fb72aae4d27e24230b91561588e3a39222ff2f516fdf2213b6dffb8fd43
a-s6r-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s6r-M-marks.xml 3396 B 4ac186ec14f33c8066791cc711c6b86445781db3758c5abc3d982280fa4b5b7f
a-s7m-M-marks.xml 3396 B 4ac186ec14f33c8066791cc711c6b86445781db3758c5abc3d982280fa4b5b7f
a-s7-I-CN-3.xml 540 B e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec
a-s7r-R-CN3.xml 95832 B 53c276738619f4e7d390ca45c1d2d7d8b4cf69ac1ef3426345de1a63a6d18e65
a-s7r-R-CN1.xml 95834 B 986051ad0fa5d1431c83631b5eb570804af026c67b62412684f050f124e23b9c
a-s7r-R-0099.xml 95834 B 03f9422e5e6964bca0c54ef22d8353945d228fab29b41a48f3bfd563c923d2ed
a-s7r-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s7r-M-marks.xml 3396 B 419f6b56d7e3f2add5ccb53dede6f88457ffc8a20399d3d60a4786a0d48e062e
a-s8-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s8-N-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-R-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-R-CN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-R-CN2.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-R-CN3.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
08:46:35	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
08:48:40	N-0099.xml	a-s2-N-0099.xml	curl 0	3022 bytes
08:48:40	R-0099.xml	a-s2-R-0099.xml	curl 0	3022 bytes
08:48:40	R-CN1.xml	a-s2-R-CN1.xml	curl 0	3022 bytes
08:48:40	R-CN2.xml	a-s2-R-CN2.xml	curl 0	3022 bytes
08:48:40	R-CN3.xml	a-s2-R-CN3.xml	curl 0	3022 bytes
08:48:40	B-bills.xml	a-s2-B-bills.xml	curl 0	46 bytes
08:48:40	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3m
```
08:48:58	M-marks.xml	a-s3m-M-marks.xml	curl 0	3394 bytes
```

### s3
```
08:49:06	L-ledger.xml	a-s3-L-ledger.xml	curl 0	538 bytes
```

### s4
```
08:49:16	I-0099.xml	a-s4-I-0099.xml	curl 0	540 bytes
```

### s4r
```
08:49:28	N-0099.xml	a-s4r-N-0099.xml	curl 0	5608 bytes
08:49:28	R-0099.xml	a-s4r-R-0099.xml	curl 0	95834 bytes
08:49:28	B-bills.xml	a-s4r-B-bills.xml	curl 0	510 bytes
08:49:28	M-marks.xml	a-s4r-M-marks.xml	curl 0	3396 bytes
```

### s5m
```
08:49:39	M-marks.xml	a-s5m-M-marks.xml	curl 0	3396 bytes
```

### s5
```
08:49:47	I-CN-1.xml	a-s5-I-CN-1.xml	curl 0	540 bytes
```

### s5r
```
08:49:59	R-CN1.xml	a-s5r-R-CN1.xml	curl 0	95834 bytes
08:49:59	R-0099.xml	a-s5r-R-0099.xml	curl 0	95834 bytes
08:49:59	B-bills.xml	a-s5r-B-bills.xml	curl 0	46 bytes
08:49:59	M-marks.xml	a-s5r-M-marks.xml	curl 0	3396 bytes
```

### s6m
```
08:50:13	M-marks.xml	a-s6m-M-marks.xml	curl 0	3396 bytes
```

### s6
```
08:50:15	I-CN-2.xml	a-s6-I-CN-2.xml	curl 0	540 bytes
```

### s6r
```
08:50:30	R-CN2.xml	a-s6r-R-CN2.xml	curl 0	95834 bytes
08:50:30	R-CN1.xml	a-s6r-R-CN1.xml	curl 0	95834 bytes
08:50:30	R-0099.xml	a-s6r-R-0099.xml	curl 0	95834 bytes
08:50:30	B-bills.xml	a-s6r-B-bills.xml	curl 0	46 bytes
08:50:31	M-marks.xml	a-s6r-M-marks.xml	curl 0	3396 bytes
```

### s7m
```
08:51:13	M-marks.xml	a-s7m-M-marks.xml	curl 0	3396 bytes
```

### s7
```
08:51:15	I-CN-3.xml	a-s7-I-CN-3.xml	curl 0	540 bytes
```

### s7r
```
08:51:31	R-CN3.xml	a-s7r-R-CN3.xml	curl 0	95832 bytes
08:51:31	R-CN1.xml	a-s7r-R-CN1.xml	curl 0	95834 bytes
08:51:31	R-0099.xml	a-s7r-R-0099.xml	curl 0	95834 bytes
08:51:31	B-bills.xml	a-s7r-B-bills.xml	curl 0	46 bytes
08:51:31	M-marks.xml	a-s7r-M-marks.xml	curl 0	3396 bytes
```

### s8
```
08:54:51	M-marks.xml	a-s8-M-marks.xml	curl 0	3394 bytes
08:54:51	N-0099.xml	a-s8-N-0099.xml	curl 0	3022 bytes
08:54:51	R-0099.xml	a-s8-R-0099.xml	curl 0	3022 bytes
08:54:51	R-CN1.xml	a-s8-R-CN1.xml	curl 0	3022 bytes
08:54:51	R-CN2.xml	a-s8-R-CN2.xml	curl 0	3022 bytes
08:54:51	R-CN3.xml	a-s8-R-CN3.xml	curl 0	3022 bytes
08:54:51	B-bills.xml	a-s8-B-bills.xml	curl 0	46 bytes
```
