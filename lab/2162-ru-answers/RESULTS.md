# Row 2162-RU: Tally wrote composition and inactive as new dated rows on the registration, a second registration as a third unit with the book's GUID prefix, and the Sales type's binding in DEFAULTGSTREGISTRATION

Run on 9 Nov 2026, 19:09:37Z to 19:38:55Z (00:39 to 01:08 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-ru-sheet` (b457776b), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no request writes and no Bridge build. Four master saves were keyed on Tally's screen, one variable per save, and the book was restored from the row's own backup after each part. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran after sitting 2 of row 2360-L (Tally quit at about 19:08:5xZ), on the run lead's word, with a Tally start of its own. The sheet b5418f19, which b457776b replaces, was not run. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The book was restored after the last part, so the marks it leaves are those it found (68/245).

The two GSTINs in this folder are the synthetic family's, as the sheet names them: the company's own `08ZZZZZ0000Z1ZQ` and the second registration's `27ZZZZZ0000Z1ZQ`.

The row stopped four times on a screen the sheet does not name, pressed no key, and went on only on the run lead's word each time. The words and keys are under each part and in Deviations.

**Result.**
- **s2, before:** U two units, each the book's: `Default Tax Unit`, and `Rajasthan Registration` (GST, `08ZZZZZ0000Z1ZQ`, GSTOLDREGNUMBER empty, one dated row 20260401, Rajasthan, Rajasthan, Regular, not inactive). T 28 VOUCHERTYPE rows; BRIDGE Sales one row with `TAXUNITNAME` Any, `LASTGSTTAXUNIT` Rajasthan Registration, `ALLOWMULTIPLETAXUNITSPERSERIES` No, `LASTSERIESBYGSTTAXUNIT` Default_BRIDGE Sales and `LASTGSTTAXUNITBYSERIES` Rajasthan Registration. R-0010 `BRIDGE Walk-in`, included Yes, every other status flag No. M 68/245. Every EXPECT met.
- **Part A, Regular to Composition:** Tally asked no date and showed no message; Ctrl+A saved with no "Accept ?". **U gained a second dated row on `Rajasthan Registration`: FROMDATE 20260915 (Tally's current date), Rajasthan, Rajasthan, `REGISTRATIONTYPE` Composition, `ISINACTIVE` No.** The 20260401 row is unchanged (Regular). U also gained a `GSTCOMPOSITIONDETAILS.LIST` on the unit: FROMDATE 20260401, TAXRATE 1, TURNOVERTYPE Taxable Value, ISTAXRATEZERO No. 15 leaves of the unit differ from s2, `ALTERID` 206 to 246 among them; `Default Tax Unit` 0. T: BRIDGE Sales 0 leaves differ. R-0010: no difference. M 68/246.
- **Part B, Active to Inactive:** choosing Inactive opened a box "Effective Date", one field "Effective Date for revised GST details", with a list of dates. The sheet's date, 1-9-2026, was typed, and a second box "New Effective Date" showed it as 1-Sep-26; Enter there was pressed on the run lead's word. The form's own "Accept ?" came at once and was answered Yes. **U gained a second dated row on `Rajasthan Registration`: FROMDATE 20260901 (the date entered), Rajasthan, Rajasthan, `REGISTRATIONTYPE` Regular, `ISINACTIVE` Yes, GSTR1PERIODICITY Monthly.** The 20260401 row is unchanged. 9 leaves of the unit differ from s2, `ALTERID` 206 to 246 among them. T: BRIDGE Sales 0 leaves differ. R-0010: no difference. M 68/246.
- **Part C2, a second registration:** Create, then `GST Registration`, brought the question "GST Registration already exists for the Company. Do you want to alter the existing GST Registration or create a new GST Registration?"; C (Create New) was pressed on the run lead's word. The form asked for no name and no date from which the registration applies. **U gained a third unit, `Maharashtra Registration` (named by Tally), TAXTYPE GST, `27ZZZZZ0000Z1ZQ`, GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8` (the book's GUID plus a suffix), `ISPRIMARYUNIT` No, with one dated row FROMDATE 20260401, Maharashtra, Maharashtra, Regular, not inactive.** The e-Way Bill date the form showed (Applicable from 1-Apr-26) is in U as `GSTEWAYBILLDETAILS.LIST` APPLICABLEFROM 20260401, beside a `GSTRECONCONFIGDETAILS.LIST` APPLICABLEFROM 20260401. `Rajasthan Registration` 0 leaves differ. T: BRIDGE Sales 0 leaves differ. R-0010: no difference. M 68/246.
- **Part C3, BRIDGE Sales bound to Rajasthan Registration:** "Voucher Type Alteration" for BRIDGE Sales shows, under Statutory Details, "Default GST Registration" (Any) and "Use common voucher numbering series for all GST Registrations" (No). Rajasthan Registration was chosen from the first field's list; Ctrl+A saved with no "Accept ?". **T, BRIDGE Sales: `DEFAULTGSTREGISTRATION` empty to `Rajasthan Registration` (and `ALTERID` 212 to 248). `TAXUNITNAME` stayed Any**; `LASTGSTTAXUNIT` and `ALLOWMULTIPLETAXUNITSPERSERIES` are unchanged. U: 0 leaves differ in all three units. R-0010: no difference. M 68/248.
- **The three restores:** each by C6 from the one BK. After each, M `--same-row` against s1 equal (68/245), U `--unit-diff` and T `--type-diff` against s2 0 leaves, and M, U and T each byte-equal to s2's.

**By the sheet's Decides:**
- **A or B lands in a dated row the parser reads:** measured for both. Part A wrote `REGISTRATIONTYPE` Composition in a new dated row from Tally's current date (20260915), with no date asked. Part B wrote `ISINACTIVE` Yes in a new dated row from the date entered (20260901). In each, the 20260401 row stayed Regular and not inactive. PARTIAL: one sitting, one book, one release.
- **A or B lands anywhere else, with the dated row still Regular and not inactive:** not seen. Part A also wrote a unit-level `GSTCOMPOSITIONDETAILS.LIST` dated 20260401; the dated row carries the type as well.
- **A or B edits the 20260401 row in place:** not seen. Tally kept the earlier row and added one.
- **C2 adds a unit whose GUID begins with the book's:** measured. The new unit's GUID is the book's GUID plus `-000000e8`, and its one dated row (20260401) is in force with Rajasthan Registration's.
- **C2 adds a unit whose GUID does not begin with the book's:** not seen.
- **C3 shows where the binding lands:** measured. In `DEFAULTGSTREGISTRATION`, a leaf s2's answer held empty; not in `TAXUNITNAME`, which stayed Any. This differs from the sheet's Expected line for C3.
- **Any part changes R-0010:** not seen. R-0010's `--diff` against s2 (and C3's against C2) printed only `lists compared: 1` and `legs: 6 -> 6` each time.

## Step 1: load, check and back up
- Sheet files: all 9 equal to `SHA256.txt` at b457776b; `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml`, `U-taxunits.xml` and `.gitattributes` byte-equal to row 2162-SC's.
- C2 step 0 with no `tally.exe` running. Tally was started once, at 19:09:37Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2). Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt).
- C2 at 19:10:24Z: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID. No Bridge process was ended and no Bridge tool was called (a standing rule of the lab).
- Loaded alone from Select Company (the name typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26 (`s1-gateway`).
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5): the Data menu (by its label), Backup & Restore, Backup, Specify Path to the new empty folder `C:\lab\2162-ru\backup-1`. BRIDGE PILOT LAB alone, End of List; "Accept ?" captured (`s1-bk-accept`, 19:11:53Z) and answered y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `dfb970d6ba94a2a00df030a42dc802a52dd9f8015a7c2705688c089eafbc360f`; the Restore list was to show 10-Nov-26 (12:41 am), and showed it as 10-Nov-26 (0:41 am) each time.

## Step 2: the book before
`s2`: U, T, R-0010 and M, as in the Result. EXPECT met. The outputs are below, whole.

## Step 3: Part A, the registration's type
`sAm`, then the before-write check against s1: one COMPANY row, the same GUID (68/245). Every screen, list and key is in `evidence/sA-screens.txt`. In short:
1. The Gateway, Down once to Alter, Enter. "Master Alteration" listed, among its Statutory Masters, `GST Registration`; Down 11 times to it, Enter. "List of GST Registrations": Create, Back, and one line, `Rajasthan Registration`. Enter on it. This is row 2162-SC's path to the same screen.
2. "GST Registration Alteration" (`sA-2-alteration`). Its fields, before the change: Registration status Active; State Rajasthan; Registration type Regular, with a list "Registration Types": Regular, Composition, Regular - SEZ; Assessee of Other Territory No; GSTIN/UIN `08ZZZZZ0000Z1ZQ`; Periodicity of GSTR-1 Monthly; GST Username empty; Mode of Filing Not Applicable; e-Invoicing applicable No; e-Way Bill applicable Yes, from 1-Apr-26, for intrastate Yes. The field naming the type: `Registration type`.
3. Down, Down (`Composition` highlighted, zoomed), Enter. No prompt and no date field. The form then dropped "Periodicity of GSTR-1" and the e-Invoice section, and showed "Tax Rate Details for Turnover": Tax Rate for taxable turnover 1 %, Calculate tax based on Taxable Value (`sA-3-after-type`). No other field was touched.
4. Ctrl+A: no "Accept ?" and no message; back on the List of GST Registrations (`sA-4-after-accept`). Ctrl+Q once: the Gateway.
5. `sA`: M, U, T and R-0010, as in the Result; the three diffs against s2 are below, whole.
6. **Restore** by C6 (Step 6 below has the keys), then `sAr`: as in the Result. EXPECT met.

## Step 4: Part B, the registration made inactive
`sBm`, then the before-write check against s1: passed (68/245). Every screen, list and key is in `evidence/sB-screens.txt`. In short:
1. As 3.1; the alteration screen was byte-equal to Part A's capture of it (`sB-2-alteration`).
2. Shift+Tab to State (no list), Shift+Tab to `Registration status`, with a list "Registration status": Active (highlighted), Inactive (`sB-2-status-list`). Down (Inactive, zoomed), Enter.
3. A box "Effective Date" came up, one field "Effective Date for revised GST details", empty, with a list "List of Effective Dates" (`sB-3-effective-date`), word for word:
   - New Effective Date
   - 15-Sep-26, Current Date of Company (highlighted)
   - 15-Sep-26, Date of Last Entry
   - 10-Nov-26, Today (Computer Date)
   - 11-Nov-26, Tomorrow (Computer Date)
4. `1-9-2026` typed by the sheet's date rule; the list narrowed to one line, "1-9-2026, New Effective Date". Enter. A second box came up, titled "New Effective Date", one field reading 1-Sep-26 (`sB-4-new-effective-date`). By the sheet's rule on a second date, the row stopped with no key pressed and asked. On the run lead's word (the box takes the one date chosen, the same date): zoomed, it read exactly 1-Sep-26; Enter once. No further date box or field came up.
5. The form's own "Accept ?" came at once, with no Ctrl+A pressed (`sB-5-accept`); the form read Registration status Inactive and every other field as before. Answered y; no other message; back on the List of GST Registrations (`sB-6-after-accept`). Ctrl+Q once: the Gateway.
6. `sB`: as in the Result; the three diffs against s2 are below, whole.
7. **Restore** by C6, then `sBr`: as in the Result. EXPECT met.

## Step 5: Part C, a second registration, then the Sales type bound to one
`sCm`, then the before-write check against s1: passed (68/245). Every screen, list and key is in `evidence/sC-screens.txt`. In short:
1. **C2, NEW PATH.** The Gateway, Create (highlighted), Enter. "Master Creation", List of Masters: Change Company, Show More, then the same masters as Master Alteration (no Expand All line). Down to `GST Registration` (the heading "Statutory Masters" is a stop on the way), Enter.
2. A question came up (`sC2-2-exists-question`), word for word: "GST Registration already exists for the Company. Do you want to alter the existing GST Registration or create a new GST Registration?", with "C Create New" (the focus) and "A Alter Existing". The sheet does not name it: the row stopped with no key pressed and asked. On the run lead's word, `c`.
3. "GST Registration Creation" (`sC2-3-creation`), as Tally filled it: Registration status Active; State empty; Registration type Regular; Assessee of Other Territory No; GSTIN/UIN empty; Periodicity of GSTR-1 Monthly; GST Username empty; Mode of Filing Not Applicable; e-Invoicing applicable No; e-Way Bill applicable Yes, from 1-Apr-26, for intrastate Yes. No name field.
4. State: `Maha` typed; "List of States": New State, Not Applicable, Maharashtra (highlighted); Enter. Registration type: its list as at Part A; Down to `Regular`, Enter. GSTIN/UIN: `27ZZZZZ0000Z1ZQ` typed, zoomed before any Enter: 15 characters, capitals (`sC2-4-gstin-typed`). Every other field kept by Enter alone, each list seen written down. No field asked for the date from which the registration applies, so the sheet's `1-4-2026` was not typed.
5. After the last field, the form's own "Accept ?" (`sC2-6-accept`): y. A dialog then came up (`sC2-7-series-question`), word for word: "You can create a unique voucher numbering series to cater to multiple GST registrations. You can do this by going to the Chart of Accounts and selecting the relevant voucher types. Open Chart of Accounts and create now?", Yes or No. The row stopped with no key pressed and asked. On the run lead's word, `n`.
6. Tally then opened a new, empty "GST Registration Creation" form (`sC2-8-after-no`). The row stopped with no key pressed and asked. On the run lead's word: Ctrl+Q once, with no question, to "Master Creation" (`sC2-9-master-creation`), and Ctrl+Q once more to the Gateway (`sC2-10-gateway`).
7. `sC2`: as in the Result; the three diffs against s2 are below, whole.
8. **C3.** The G: Go To label clicked; the List of Reports: Create Voucher, Create Master, Alter Master, Expand All, Show More, then saved views and reports. Up 4 times to `Alter Master`, Enter. "Master Alteration", Down 3 times to `Voucher Type`, Enter. "List of Voucher Types" (`sC3-1-voucher-types`): Create, Show Inactive, Back, then 24 types, `BRIDGE Sales` first and highlighted. Enter.
9. "Voucher Type Alteration" (`sC3-2-alteration`). Every field before the change is in `evidence/sC-screens.txt`. Under "Statutory Details": `Default GST Registration` Any (diamond), and `Use common voucher numbering series for all GST Registrations` No. The field naming a GST registration: `Default GST Registration`.
10. A click on its value: a list "List of GST Registrations": Create; Any (highlighted); Maharashtra Registration; Rajasthan Registration. Down twice (`Rajasthan Registration`, zoomed), Enter (`sC3-3-after-choice`). No other field was touched.
11. Ctrl+A: no "Accept ?" and no message; back on the List of Voucher Types (`sC3-4-after-accept`). Ctrl+Q once: the Gateway.
12. `sC3`: as in the Result; `--type-diff` and `--unit-diff` against sC2 are below, whole, with R-0010's `--diff` against sC2.
13. **Restore** by C6, then `sCr`: as in the Result. EXPECT met.

## Step 6: the three restores (C6)
Before each shut: BK re-hashed, equal to step 1's, its folder holding only that file; the restore plan written; the data folder listed (24 entries each time). Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. The Data menu (by its label), Backup & Restore, Restore, Specify Path to the BK's folder: the list showed BRIDGE PILOT LAB, its number and 10-Nov-26 (0:41 am), the BK's time. The company line, End of List. Two prompts, the only ones answered, each captured first:
- "Accept ?": y (after A 19:18:32Z, after B 19:25:16Z, after C 19:36:41Z).
- "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", with R Replace and S Skip: r (19:18:52Z, 19:25:30Z, 19:36:55Z).

Select Company came back with no other message each time; the data folder then held 17 entries, and its `Company.1800` held the book's GUID. Loaded alone (the name typed, one row, Enter, no prompt); each Gateway capture byte-equal to `s1-gateway`. Then the `r` reads, as in the Result.

## Step 7: close (C10)
Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). At 19:38:38Z, one `tally.exe`, which alone owns 9001, and `GET /status` 200. No XML request was sent after the shut. Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company): no prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **Four stops on screens the sheet does not name,** each with no key pressed until the run lead's word: Part B's second date box (Enter); C2's exists question (C); C2's numbering-series question after the save (n); the new empty creation form after it (Ctrl+Q twice to the Gateway). None on Part A or C3. The book held no change from the part at each of the first two stops, and the saved C2 unit at the last two.
- **Part B's "Accept ?"** came by itself after the date, without Ctrl+A. **Part A, C3:** Ctrl+A brought no "Accept ?", so none was answered there.
- **No registration date was asked** in Part A (Tally wrote its current date, 20260915) or in C2 (Tally wrote 20260401). The sheet's `1-4-2026` for C2 was never typed.
- **C3's field:** the sheet's Expected line named `TAXUNITNAME`. The screen's field "Default GST Registration" wrote `DEFAULTGSTREGISTRATION`, and `TAXUNITNAME` stayed Any. This was sent to the run lead before the restore.
- **C3's field was reached by a mouse click** on its value, not by Enter through the fields above it; each Enter on the way could have opened a field's own list. No other field was touched.
- **Go To and the menus** were opened by clicking their labels (G: Go To, K: Company, Y: Data), as the lab's standing rule asks.
- **Focus:** before each key from 19:31Z on, Tally's own title bar was clicked (its empty part), as one key was refused by the desktop tool when the focus was not Tally's; no key reached Tally then.
- **After the C restore,** the right-hand button panel of the Gateway showed a red line "N: Manage Rental License", not seen there earlier in the row. It was not pressed. The Gateway captures hold the window's left part only, so it is not in them.
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.

## Not measured
Every item the sheet lists as not measured. Also: what "A Alter Existing" or the numbering-series "Yes" would have done; what "Use common voucher numbering series for all GST Registrations" writes; Part A with a date entered (none was asked) and Part B from Tally's current date; which unit `parse_company_registration` takes as in force on any date after these changes (the build's tests read that from the U answers, not this row); and what the "N: Manage Rental License" line offers.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`, with the reason).
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count, the before-write, `--same-row`, `--unit-diff`, `--type-diff` and `--diff` outputs, the send times, the screens written down, the BK record and the answer and capture hash lists. The process, port and data-folder listings and the restore plans stay local.
- `scripts/`: the step 0 helper, the window capture helper, the send helper and the C6 prep helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## count14.py outputs, whole

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
```

### sAm
```
== a-sAm-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### sA
```
== a-sA-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-sA-R-0010.xml: 94474 bytes
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
== a-sA-T-types.xml: 306946 bytes
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
== a-sA-U-taxunits.xml: 23782 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
    dated: FROMDATE '20260915'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Composition'; ISINACTIVE 'No'
```

### sAr
```
== a-sAr-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-sAr-T-types.xml: 306946 bytes
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
== a-sAr-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

### sBm
```
== a-sBm-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### sB
```
== a-sB-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-sB-R-0010.xml: 94474 bytes
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
== a-sB-T-types.xml: 306946 bytes
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
== a-sB-U-taxunits.xml: 23444 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
    dated: FROMDATE '20260901'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'Yes'
```

### sBr
```
== a-sBr-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-sBr-T-types.xml: 306946 bytes
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
== a-sBr-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

### sCm
```
== a-sCm-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### sC2
```
== a-sC2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-sC2-R-0010.xml: 94474 bytes
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
== a-sC2-T-types.xml: 306946 bytes
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
== a-sC2-U-taxunits.xml: 33304 bytes
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

### sC3
```
== a-sC3-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-sC3-R-0010.xml: 94474 bytes
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
== a-sC3-T-types.xml: 307038 bytes
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
   unit leaf /DEFAULTGSTREGISTRATION[1] 'Rajasthan Registration'
   unit leaf /LASTGSTTAXUNIT[1] 'Rajasthan Registration'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Sales Keyed': 0
== a-sC3-U-taxunits.xml: 33304 bytes
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

### sCr
```
== a-sCr-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-sCr-T-types.xml: 306946 bytes
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
== a-sCr-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

## Checks and diffs, whole

### sAm against s1 (before-write)
```
$ python -I count14.py <the before-write option> a-s1-M-marks.xml a-sAm-M-marks.xml
== a-sAm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### U, s2 against sA
```
$ python -I count14.py --unit-diff a-s2-U-taxunits.xml a-sA-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-sA-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 15 leaves differ
     /ALTERID[1]: '206' -> '246'
     /GSTCOMPOSITIONDETAILS.LIST[1]: '' -> '<absent>'
     /GSTCOMPOSITIONDETAILS.LIST[1]/BASISOFTAXCALCULATION[1]: '<absent>' -> ''
     /GSTCOMPOSITIONDETAILS.LIST[1]/FROMDATE[1]: '<absent>' -> '20260401'
     /GSTCOMPOSITIONDETAILS.LIST[1]/ISTAXRATEZERO[1]: '<absent>' -> 'No'
     /GSTCOMPOSITIONDETAILS.LIST[1]/TAXRATE[1]: '<absent>' -> '1'
     /GSTCOMPOSITIONDETAILS.LIST[1]/TURNOVERTYPE[1]: '<absent>' -> 'Taxable Value'
     /GSTREGISTRATIONDETAILS.LIST[2]/FROMDATE[1]: '<absent>' -> '20260915'
     /GSTREGISTRATIONDETAILS.LIST[2]/GSTR1PERIODICITY[1]: '<absent>' -> ''
     /GSTREGISTRATIONDETAILS.LIST[2]/ISINACTIVE[1]: '<absent>' -> 'No'
     /GSTREGISTRATIONDETAILS.LIST[2]/ISOTHTERRITORYASSESSEE[1]: '<absent>' -> ''
     /GSTREGISTRATIONDETAILS.LIST[2]/ISSTATECESSON[1]: '<absent>' -> 'No'
     /GSTREGISTRATIONDETAILS.LIST[2]/PLACEOFSUPPLY[1]: '<absent>' -> 'Rajasthan'
     /GSTREGISTRATIONDETAILS.LIST[2]/REGISTRATIONTYPE[1]: '<absent>' -> 'Composition'
     /GSTREGISTRATIONDETAILS.LIST[2]/STATE[1]: '<absent>' -> 'Rajasthan'
```

### T, s2 against sA
```
$ python -I count14.py --type-diff a-s2-T-types.xml a-sA-T-types.xml
== --type-diff a-s2-T-types.xml a-sA-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### R-0010, s2 against sA
```
$ python -I count14.py --diff a-s2-R-0010.xml a-sA-R-0010.xml
== --diff a-s2-R-0010.xml a-sA-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### M, sAr against s1 (--same-row)
```
$ python -I count14.py --same-row a-s1-M-marks.xml a-sAr-M-marks.xml
== a-sAr-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### U, s2 against sAr
```
$ python -I count14.py --unit-diff a-s2-U-taxunits.xml a-sAr-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-sAr-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### T, s2 against sAr
```
$ python -I count14.py --type-diff a-s2-T-types.xml a-sAr-T-types.xml
== --type-diff a-s2-T-types.xml a-sAr-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### sBm against s1 (before-write)
```
$ python -I count14.py <the before-write option> a-s1-M-marks.xml a-sBm-M-marks.xml
== a-sBm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### U, s2 against sB
```
$ python -I count14.py --unit-diff a-s2-U-taxunits.xml a-sB-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-sB-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 9 leaves differ
     /ALTERID[1]: '206' -> '246'
     /GSTREGISTRATIONDETAILS.LIST[2]/FROMDATE[1]: '<absent>' -> '20260901'
     /GSTREGISTRATIONDETAILS.LIST[2]/GSTR1PERIODICITY[1]: '<absent>' -> 'Monthly'
     /GSTREGISTRATIONDETAILS.LIST[2]/ISINACTIVE[1]: '<absent>' -> 'Yes'
     /GSTREGISTRATIONDETAILS.LIST[2]/ISOTHTERRITORYASSESSEE[1]: '<absent>' -> ''
     /GSTREGISTRATIONDETAILS.LIST[2]/ISSTATECESSON[1]: '<absent>' -> 'No'
     /GSTREGISTRATIONDETAILS.LIST[2]/PLACEOFSUPPLY[1]: '<absent>' -> 'Rajasthan'
     /GSTREGISTRATIONDETAILS.LIST[2]/REGISTRATIONTYPE[1]: '<absent>' -> 'Regular'
     /GSTREGISTRATIONDETAILS.LIST[2]/STATE[1]: '<absent>' -> 'Rajasthan'
```

### T, s2 against sB
```
$ python -I count14.py --type-diff a-s2-T-types.xml a-sB-T-types.xml
== --type-diff a-s2-T-types.xml a-sB-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### R-0010, s2 against sB
```
$ python -I count14.py --diff a-s2-R-0010.xml a-sB-R-0010.xml
== --diff a-s2-R-0010.xml a-sB-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### M, sBr against s1 (--same-row)
```
$ python -I count14.py --same-row a-s1-M-marks.xml a-sBr-M-marks.xml
== a-sBr-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### U, s2 against sBr
```
$ python -I count14.py --unit-diff a-s2-U-taxunits.xml a-sBr-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-sBr-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### T, s2 against sBr
```
$ python -I count14.py --type-diff a-s2-T-types.xml a-sBr-T-types.xml
== --type-diff a-s2-T-types.xml a-sBr-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### sCm against s1 (before-write)
```
$ python -I count14.py <the before-write option> a-s1-M-marks.xml a-sCm-M-marks.xml
== a-sCm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### U, s2 against sC2
```
$ python -I count14.py --unit-diff a-s2-U-taxunits.xml a-sC2-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-sC2-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
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
```

### T, s2 against sC2
```
$ python -I count14.py --type-diff a-s2-T-types.xml a-sC2-T-types.xml
== --type-diff a-s2-T-types.xml a-sC2-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### R-0010, s2 against sC2
```
$ python -I count14.py --diff a-s2-R-0010.xml a-sC2-R-0010.xml
== --diff a-s2-R-0010.xml a-sC2-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### T, sC2 against sC3
```
$ python -I count14.py --type-diff a-sC2-T-types.xml a-sC3-T-types.xml
== --type-diff a-sC2-T-types.xml a-sC3-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 2 leaves differ
     /ALTERID[1]: '212' -> '248'
     /DEFAULTGSTREGISTRATION[1]: '' -> 'Rajasthan Registration'
```

### U, sC2 against sC3
```
$ python -I count14.py --unit-diff a-sC2-U-taxunits.xml a-sC3-U-taxunits.xml
== --unit-diff a-sC2-U-taxunits.xml a-sC3-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### R-0010, sC2 against sC3
```
$ python -I count14.py --diff a-sC2-R-0010.xml a-sC3-R-0010.xml
== --diff a-sC2-R-0010.xml a-sC3-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### M, sCr against s1 (--same-row)
```
$ python -I count14.py --same-row a-s1-M-marks.xml a-sCr-M-marks.xml
== a-sCr-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### U, s2 against sCr
```
$ python -I count14.py --unit-diff a-s2-U-taxunits.xml a-sCr-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-sCr-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### T, s2 against sCr
```
$ python -I count14.py --type-diff a-s2-T-types.xml a-sCr-T-types.xml
== --type-diff a-s2-T-types.xml a-sCr-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```
