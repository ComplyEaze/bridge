# Row 2464-GR, second sitting: the import that names no registration was created with none: `GSTREGISTRATION` and `CMPGSTIN` empty, with `BRIDGE Sales` bound to `Maharashtra Registration`

Run on 10 Nov 2026, 01:31:14Z to 01:50:37Z (07:01 to 07:20 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the revised sheet at `lab/2464-gr-sheet` (3508daa0), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no Bridge build. Two master changes on Tally's screen, one import (`I-0091.xml`, sent once), then the restore of this sitting's own backup. Every value below is M, measured by this run, in one run. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

The GSTINs in this folder are the synthetic family's, as the sheet names them: the company's own `08ZZZZZ0000Z1ZQ` and the second registration's `27ZZZZZ0000Z1ZQ`.

**Result, by the sheet's Decides (PARTIAL: one import, one sitting, one book, one release): "V-0091 holds no registration".** With two GST registrations in force and `BRIDGE Sales` bound to the second, `Maharashtra Registration`, the import of the build's request shape (renumbered `BP/26-27/0091`, naming no registration) was created (`CREATED` 1, `ERRORS` 0, no LINEERROR), and Tally gave the new invoice no registration. V-0091, as `count16.py` prints it:

| field | V-0091 (imported at s5) | V-0010 (the 9 Oct import, at s5r) |
|---|---|---|
| `GSTREGISTRATION` | `''` (no attributes) | `Rajasthan Registration` (no attributes) |
| `CMPGSTIN` | `''` | `08ZZZZZ0000Z1ZQ` |
| `CMPGSTSTATE` | `''` | `Rajasthan` |
| `CMPGSTREGISTRATIONTYPE` | `''` | `Regular` |
| `PLACEOFSUPPLY` | `Rajasthan` | `Rajasthan` |
| `STATENAME` | `Rajasthan` | `Rajasthan` |

The sheet's line for this case: "with two registrations, Tally gives such an import none. The build must name the registration itself before any two-registration book is invoiced." Neither the bound unit (`Maharashtra Registration`) nor the first unit or `LASTGSTTAXUNIT` (both `Rajasthan Registration`) was given to it. Also read, not judged (the sheet records the status flags only): R-0091 reads the voucher's GST status as uncertain (`VCHGSTSTATUSISUNCERTAIN` `Yes`, included `No`, excluded `No`), where V-0010 and R-0010 read included `Yes`. V-0010's own registration fields did not move at s5r (`--diff` against s2: no field differs).

- **s1:** one COMPANY row, the book's: ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **BK:** taken before anything was keyed; restored at step 6.
- **s2, the book before:** every EXPECT met. U byte-equal to row 2162-RU's s2 (sha256 `e40ef83c...`) and to this row's first sitting: `Default Tax Unit` and `Rajasthan Registration`. T, `BRIDGE Sales`: no `DEFAULTGSTREGISTRATION`; `LASTGSTTAXUNIT` `Rajasthan Registration`; `TAXUNITNAME` `Any`. **N-0091: 0 rows** (the number is unused). R-0010: `BRIDGE Walk-in`, included `Yes`, 6 legs. V-0010 as in the table above.
- **Step 3, the second registration:** the three named prompts, each answered with the sheet's key; then s3: a third unit, `Maharashtra Registration`, `27ZZZZZ0000Z1ZQ`, GUID the book's with suffix `-000000e8`, one dated row from 20260401, Maharashtra, Regular, not inactive; the other two units 0 leaves differ; `BRIDGE Sales` 0 leaves differ. ALTMSTID 245 to 246.
- **Step 4, the binding:** `BRIDGE Sales`' Default GST Registration set from `Any` to `Maharashtra Registration`, no other field touched, saved with no prompt. s4: `BRIDGE Sales` 2 leaves differ: `ALTERID` 212 to 248 and `DEFAULTGSTREGISTRATION` `''` to `Maharashtra Registration`. As they now read: `LASTGSTTAXUNIT` `Rajasthan Registration`, `LASTGSTTAXUNITBYSERIES` `Rajasthan Registration`, `TAXUNITNAME` `Any`, `ALLOWMULTIPLETAXUNITSPERSERIES` `No`. Every unit 0 leaves differ. ALTMSTID 246 to 248.
- **Step 5, the import:** the before-write check against s1 passed; `I-0091.xml` (sha256 equal to `SHA256.txt`) sent once: `CREATED` 1, `ALTERED` 0, `DELETED` 0, `LASTVCHID` 45, `LASTMID` 0, `COMBINED` 0, `IGNORED` 0, `ERRORS` 0, `CANCELLED` 0, `EXCEPTIONS` 0, no LINEERROR; the Gateway after it, no dialog. s5r: N-0091 one row (`BP/26-27/0091`, 20260801); R-0091 one row, `BRIDGE Walk-in`, the six legs of `0010`'s shape; V-0091 as above. ALTVCHID 68 to 69, ALTMSTID 248 to 248.
- **Step 6, the restore:** by C6, with `restore-plan.txt` written first; the two expected prompts. s6: every answer byte-equal to s2's (M, U, T, N-0091, R-0010, V-0010), and M equal to s1 by `--same-row`. The book is as it was found: ALTVCHID 68, ALTMSTID 245, two units, `BRIDGE Sales` unbound, no `0091`.

**Deviations from the sheet:**
1. **Bridge processes.** The sheet says "Close every other Bridge process first." Three Bridge processes were running at the start; on the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **Named prompt 3's punctuation.** On screen the prompt is three paragraphs. Every word is the sheet's, in its order, but the second paragraph ends "the relevant voucher types" with no full stop, where the sheet's joined text has one. The key, `n`, is the one either branch gives (the named prompt's, and the prompt rule's No). It was reported at once; the run lead's word was that it is the named prompt and the row goes on, with the punctuation recorded here.

**Beyond the sheet, recorded and not judged:**
- ALTMSTID moved by 2 at step 4 (246 to 248), and `BRIDGE Sales` took ALTERID 248. Of the 28 voucher types only `BRIDGE Sales` changed, and no type or unit read carries ALTERID 247. Which master took 247 is not among this row's reads.
- After the binding's Enter, the cursor moved to `Use common voucher numbering series for all GST Registrations` with that field's own Yes / No list open (No highlighted). No key was pressed on it; Ctrl+A saved.
- The Gateway's company line carried "(e)" with the footnote "(e) Data exceptions exist", at step 0 and after the reload at step 6 alike.
- V-0010's bytes at s5r differ from s2's only in the CMPINFO count lines (LEDGER, VOUCHERTYPE, CURRENCY, TAXUNIT, VOUCHERNUMBERSERIES, VOUCHER), which are not rows (LAB-COMMON C4); its `--diff` shows no field moved.

## Step 1: load, check and back up
- Sheet files: all 18 equal to `SHA256.txt` at 3508daa0. `count16.py`, `run.py`, `LAB-COMMON.md`, `M-marks.xml`, `U-taxunits.xml`, `T-types.xml`, `R-0010.xml` and `V-0010.xml` are byte-equal to the first sitting's; the four `0091` files replace its `0041` files.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 01:31:14Z, from its desktop shortcut, after Get-Process showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter, no prompt).
- C2 at 01:32:13Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded.
- The book was loaded alone: `BRIDGE` and ` PILOT` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; date of last entry 15-Sep-26.
- `s1`: as in the Result.
- **BK** (C5), by the first sitting's path, to a new folder:
  - Before the backup, `C:\lab\2464-gr\backup-2` was made new and empty (0 entries); the first sitting's `backup-1` was left as it is.
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times, checked), then `C:\lab\2464-gr\backup-2` typed in three chunks (Tally showed it as `C:\lab\2464-Gr\backup-2`, its own capital), and its row taken.
  - BRIDGE PILOT LAB alone, End of List (Up 3 times, checked). "Accept ?" was captured (`bk-accept`, 01:34:38Z) and answered y; no other message; the Gateway again.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `7d3e84a54e21927419d0a4e4f0c6752f5a7ffb15a47b58b54f88f5bd33bc17a4`, written at 07:04 IST.

## Step 2: the book before
`s2`: `U-taxunits.xml T-types.xml N-0091.xml R-0010.xml V-0010.xml M-marks.xml`, each request checked against `SHA256.txt` just before its send. Every EXPECT met, as in the Result. `s3m`: M 68/245, and the before-write check against s1 passed; no key had been pressed in Tally since BK's y.

## Step 3: the second registration (row 2162-RU's part C2), keys and prompts
- 01:36:19Z: the Gateway (`s3-1-gateway`), Create highlighted; Enter: Master Creation, List of Masters, Group highlighted. Down 9 to Godown, Down 1 to the heading, Down 1: GST Registration highlighted (each checked by zoom); Enter.
- **Named prompt 1**, word for word: "GST Registration already exists for the Company." / "Do you want to alter the existing GST Registration or create a new GST Registration?", with `C Create New` and `A Alter Existing` (`s3-2-exists-question`): the sheet's words. `c`.
- 01:36:36Z: GST Registration Creation (`s3-3-creation`), every field as Tally filled it: Registration status Active; State empty; Registration type Regular; Assessee of Other Territory No; GSTIN/UIN empty; Periodicity of GSTR-1 Monthly; GST Username empty; Mode of Filing Not Applicable; e-Invoicing applicable No; e-Way Bill applicable Yes; Applicable from 1-Apr-26; Applicable for intrastate Yes. As RU's 5.2.
- 01:37:41Z: State: `Maha` typed; List of States: New State, Not Applicable, Maharashtra (highlighted, the one state line); Enter. Registration type: list Regular, Composition, Regular - SEZ; Down once: Regular highlighted (checked); Enter. Assessee of Other Territory No, Enter. GSTIN/UIN: `27ZZZZZ000` and `0Z1ZQ` typed, checked after each; before Enter it read `27ZZZZZ0000Z1ZQ`, 15 characters, capitals (`s3-4-gstin-typed`); Enter.
- Then Enter alone on each later field as Tally filled it: Periodicity of GSTR-1 (its list, Monthly highlighted) Monthly; GST Username empty; Mode of Filing (its list, Not Applicable highlighted) Not Applicable; e-Invoicing No; e-Way Bill Yes; Applicable from 1-Apr-26; intrastate Yes (`s3-5-form-filled`).
- **Named prompt 2:** on the last field's Enter, "Accept ?" Yes or No (`s3-6-accept`). `y`.
- **Named prompt 3** (01:40:36Z, `s3-7-series-question`), three paragraphs on screen: "You can create a unique voucher numbering series to cater to multiple GST registrations." / "You can do this by going to the Chart of Accounts and selecting the relevant voucher types" / "Open Chart of Accounts and create now?", Yes or No. `n` (deviation 2).
- 01:40:53Z: a new, empty GST Registration Creation form (`s3-8-empty-form`), nothing typed in it. Ctrl+Q: Master Creation (`s3-9-master-creation`); Ctrl+Q: the Gateway, no dialog (`s3-10-gateway`). Two Ctrl+Q in all, as RU's 5.8.
- `s3`: as in the Result.

## Step 4: the binding (row 2162-RU's part C3, with the other line), keys and prompts
- `s4m`: ALTMSTID 246, equal to s3's.
- 01:42:20Z: the Gateway (`s4-1-gateway`); Go To by its label: List of Reports. Up 4: the first Up lit no visible row, then Show More, Expand All, Alter Master (each checked); Enter: Master Alteration, Group highlighted; Down 3 to Voucher Type (checked); Enter: List of Voucher Types, BRIDGE Sales highlighted (checked, `s4-2-type-list`); Enter.
- 01:43:29Z: Voucher Type Alteration (`s4-3-alteration`), every field as text: Name BRIDGE Sales; (alias) empty. General: Select type of voucher Sales; Abbreviation Sale; Activate this Voucher Type Yes; Method of Voucher Numbering Manual; Prevent creating duplicate Voucher Nos. Yes; Show unused vch nos. in transactions for Retain Original Voucher No. behaviour No (greyed); Use effective dates for vouchers No; Allow zero-valued transactions No; Make this voucher type as 'Optional' by default No; Allow narration in voucher Yes; Provide narrations for each ledger in voucher No; Enable default accounting allocations No; WhatsApp voucher after saving No. Printing: Print voucher after saving No; Use for POS invoicing No; Default template to print Not Applicable; Default title to print empty; Default bank Not Applicable; Default jurisdiction empty; Set/alter declaration No; Set/Alter Terms and Conditions No. Statutory Details: Default GST Registration Any; Use common voucher numbering series for all GST Registrations No. Name of Class empty. As RU's 5.10, with Default GST Registration reading Any.
- A click on the value of Default GST Registration: List of GST Registrations: Create; Any (highlighted); Maharashtra Registration; Rajasthan Registration. Down once: Maharashtra Registration highlighted (checked); Enter (`s4-4-bound`). The cursor moved to Use common voucher numbering series for all GST Registrations: No, its own Yes / No list open; no other field touched.
- 01:44:05Z: Ctrl+A: no prompt; the List of Voucher Types (`s4-5-saved-list`). Ctrl+Q once: the Gateway, no dialog (`s4-6-gateway`).
- `s4`: as in the Result.

## Step 5: the import
`s5m` (01:45:04Z): ALTMSTID 248, equal to s4's; the before-write check against s1 passed (one COMPANY row, the same GUID). No key was pressed in Tally between s4's capture and the send. `s5` (01:45:14Z): `I-0091.xml` sent once; answered within a second, as in the Result; the window after it the Gateway, no dialog (`s5-1-after-send`). `s5r` (01:45:35Z): `N-0091.xml R-0091.xml V-0091.xml V-0010.xml M-marks.xml`.

## Step 6: restore (C6) and compare
- 01:46:08Z, before the shut: `prep-c6.sh 5` re-hashed BK equal to `evidence/bk.txt`, found its folder holding only that file, wrote `restore-plan.txt` (kept on the lab PC, as it names the company number) and listed the Tally data folder (kept on the lab PC).
- The book shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter: Select Company, no company loaded.
- The Data label, Backup & Restore, Restore: Restore Companies; Specify Path (Up 3, checked); `C:\lab\2464-gr\backup-2` typed in three chunks (shown `C:\lab\2464-Gr\backup-2`); Enter: one company row, BRIDGE PILOT LAB, its number, 10-Nov-26 (7:04 am), as the plan says (`s6-1-restore-list`); Enter on it; Enter on End of List.
- "Accept ?" (`s6-2-accept`): `y`. "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip (`s6-3-folder-exists`): `r`. The restore ran and Tally returned to Select Company with no other prompt.
- The book reloaded alone by the filter (`s6-4-reloaded`): the Gateway as at step 0, the same period, date and date of last entry.
- `s6` (01:49:21Z): `M-marks.xml U-taxunits.xml T-types.xml N-0091.xml R-0010.xml V-0010.xml`. Every compare equal, and every answer's sha256 equal to s2's.

## Step 7: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-1-no-company`, 01:50:05Z).
- C2 at 01:50:05Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`. The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count16.py` output and compare, the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listing stay on the lab PC.
- `logs/`: each send's console output.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count16.py outputs, whole

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
== a-s2-N-0091.xml: 3022 bytes
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
   GSTREGISTRATION attributes {}
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
== a-s3-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3-T-types.xml: 306946 bytes
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
== a-s3-U-taxunits.xml: 33304 bytes
  STATUS '1'
  TAXUNIT rows: 3
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Maharashtra Registration'; TAXTYPE 'GST'; TAXREGISTRATION '27ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8' ; the book's: yes
    GSTREGNUMBER '27ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Maharashtra'; PLACEOFSUPPLY 'Maharashtra'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

### s4m
```
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s4
```
== a-s4-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4-T-types.xml: 307042 bytes
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
   unit leaf /DEFAULTGSTREGISTRATION[1] 'Maharashtra Registration'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 0
== a-s4-U-taxunits.xml: 33304 bytes
  STATUS '1'
  TAXUNIT rows: 3
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Maharashtra Registration'; TAXTYPE 'GST'; TAXREGISTRATION '27ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8' ; the book's: yes
    GSTREGNUMBER '27ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Maharashtra'; PLACEOFSUPPLY 'Maharashtra'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

### s5m
```
== a-s5m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s5
```
== a-s5-I-0091.xml: 540 bytes
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

### s5r
```
== a-s5r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5r-N-0091.xml: 5606 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0091' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s5r-R-0091.xml: 87968 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0091'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0091'
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
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s5r-V-0010.xml: 52784 bytes
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
   GSTREGISTRATION attributes {}
   direct children: 729
== a-s5r-V-0091.xml: 52472 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0091'
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
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION ''
   CMPGSTIN ''
   CMPGSTSTATE ''
   CMPGSTREGISTRATIONTYPE ''
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   GSTREGISTRATION attributes {}
   direct children: 729
```

### s6
```
== a-s6-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6-N-0091.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s6-R-0010.xml: 94474 bytes
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
== a-s6-T-types.xml: 306946 bytes
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
== a-s6-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
== a-s6-V-0010.xml: 52784 bytes
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
   GSTREGISTRATION attributes {}
   direct children: 729
```

## Compares and checks, whole

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3: U against s2 (--unit-diff)
```
== --unit-diff a-s2-U-taxunits.xml a-s3-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': in the second only
     (87 empty leaves not listed)
     /ALTERID[1]: '246'
     /ASORIGINAL[1]: 'No'
     /ERRKEY[1]: '0'
     /ESIGNMETHOD[1]: 'Not Applicable'
     /GSTEINVREPORTPERIOD[1]: '0'
     /GSTEWAYBILLDETAILS.LIST[1]/APPLICABLEFROM[1]: '20260401'
     /GSTEWAYBILLDETAILS.LIST[1]/EWAYBILLAPPLICABLE[1]: 'Yes'
     /GSTRECONCONFIGDETAILS.LIST[1]/APPLICABLEFROM[1]: '20260401'
     /GSTRECONCONFIGDETAILS.LIST[1]/IGNOREDIFFUPTO[1]: '0'
     /GSTRECONCONFIGDETAILS.LIST[1]/IGNORESPECIALCHARINDOCNO[1]: 'No'
     /GSTRECONCONFIGDETAILS.LIST[1]/IGNOREZERODOCNOPREFIX[1]: 'No'
     /GSTREGISTRATIONDETAILS.LIST[1]/FROMDATE[1]: '20260401'
     /GSTREGISTRATIONDETAILS.LIST[1]/GSTR1PERIODICITY[1]: 'Monthly'
     /GSTREGISTRATIONDETAILS.LIST[1]/ISINACTIVE[1]: 'No'
     /GSTREGISTRATIONDETAILS.LIST[1]/ISSTATECESSON[1]: 'No'
     /GSTREGISTRATIONDETAILS.LIST[1]/PLACEOFSUPPLY[1]: 'Maharashtra'
     /GSTREGISTRATIONDETAILS.LIST[1]/REGISTRATIONTYPE[1]: 'Regular'
     /GSTREGISTRATIONDETAILS.LIST[1]/STATE[1]: 'Maharashtra'
     /GSTREGNUMBER[1]: '27ZZZZZ0000Z1ZQ'
     /GUID[1]: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'
     /ISEWAYBILLAPPLICABLEFORINTRA[1]: 'Yes'
     /ISEWAYBILLAPPLICABLEPREV[1]: 'Yes'
     /ISEWAYBILLPRINTAPPLICABLE[1]: 'No'
     /ISGSTADVRCPTON[1]: 'No'
     /ISGSTEINVAPPLICABLEPREV[1]: 'No'
     /ISGSTEINVINCLEWAYBILL[1]: 'Yes'
     /ISGSTRCON[1]: 'No'
     /ISGSTRDOWNLOADAPPLICABLE[1]: 'No'
     /ISOTHTERRITORYASSESSEE[1]: 'No'
     /ISPRIMARYUNIT[1]: 'No'
     /ISSTATECESSAPPLICABLE[1]: 'No'
     /LANGUAGENAME.LIST[1]/LANGUAGEID[1]: '0'
     /LANGUAGENAME.LIST[1]/NAME.LIST[1]/NAME[1]: 'Maharashtra Registration'
     /LANGUAGENAME.LIST[1]/NAME.LIST[1]@TYPE: 'String'
     /MIGSRCIDFORGSTREG[1]: '0'
     /PARENT[1]: 'Primary'
     /REMOTEALTERID[1]: '0'
     /REMOTEALTGUID[1]: '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
     /UPDATEDDATETIME[1]: '000000000'
     /USEDFOR[1]: 'GST'
     /USEFORJEWELLERY[1]: 'No'
     @NAME: 'Maharashtra Registration'
     @TAXREGISTRATION: '27ZZZZZ0000Z1ZQ'
     @TAXTYPE: 'GST'
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
```

### s3: T, BRIDGE Sales, against s2 (--type-diff)
```
== --type-diff a-s2-T-types.xml a-s3-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
```

### s4: T, BRIDGE Sales, against s3 (--type-diff)
```
== --type-diff a-s3-T-types.xml a-s4-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 2 leaves differ
     /ALTERID[1]: '212' -> '248'
     /DEFAULTGSTREGISTRATION[1]: '' -> 'Maharashtra Registration'
exit 0
```

### s4: U against s3 (--unit-diff)
```
== --unit-diff a-s3-U-taxunits.xml a-s4-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5r: V-0010 against s2 (--diff)
```
== --diff a-s2-V-0010.xml a-s5r-V-0010.xml
   lists compared: 2
   legs: 0 -> 0
exit 0
```

### s6: M against s1 (--same-row)
```
== a-s6-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### s6: U against s2 (--unit-diff)
```
== --unit-diff a-s2-U-taxunits.xml a-s6-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
```

### s6: T, BRIDGE Sales, against s2 (--type-diff)
```
== --type-diff a-s2-T-types.xml a-s6-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
```

### s6: R-0010 against s2 (--diff)
```
== --diff a-s2-R-0010.xml a-s6-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-N-0091.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s2-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s2-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3-M-marks.xml 3394 B f1845d9f7649984fc39f0929f60c86eb6b255f0def12cb8c1e867ef7350b14ea
a-s3-T-types.xml 306946 B 3239e41dfed99766ac1ee6da5724eea72ca206f55113d729bb0c998d56402425
a-s3-U-taxunits.xml 33304 B 2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1
a-s4m-M-marks.xml 3394 B f1845d9f7649984fc39f0929f60c86eb6b255f0def12cb8c1e867ef7350b14ea
a-s4-M-marks.xml 3394 B 6ae4f3a78b93fb3e59bdbb822868d4826850cf823173d7d998c1aaf213f048e6
a-s4-T-types.xml 307042 B 16bbd91c375cc6f75ee5913b66b7c46c254fd41a1b869caa33941b49f912e655
a-s4-U-taxunits.xml 33304 B 508dd8de3eb34f011a48debf7e86154657b129efc570d4ccd6a6ec5d96143621
a-s5m-M-marks.xml 3394 B 6ae4f3a78b93fb3e59bdbb822868d4826850cf823173d7d998c1aaf213f048e6
a-s5-I-0091.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s5r-M-marks.xml 3394 B 9f4cfdbc778acfcd05ce38ba9f338a0af0918a7039ffa5daa3ddcff4ac32395b
a-s5r-N-0091.xml 5606 B cdf8656726d64f6b5da604903e05151534fe1d33f3a53a8ad46f35123427b03a
a-s5r-R-0091.xml 87968 B 4ff6efbb169f400a26f0eae8780de9e2e5f58c17398cd50104350693c43af0cc
a-s5r-V-0010.xml 52784 B b615d69c2fa4eea2ea0f57600e40d2b019a02917cb6aa2f07c219d49a89ceb67
a-s5r-V-0091.xml 52472 B e78cebd024b28450cfaa6a4ebf12d389027447941a6b87e1f417308070ef02e0
a-s6-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s6-N-0091.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s6-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s6-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s6-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s6-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
```
