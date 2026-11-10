# Row 2464-GN: with two GST registrations and `BRIDGE Sales` bound to the second, neither import that names the registration was created (`CREATED` 0, `EXCEPTIONS` 1, no LINEERROR); the read-back that fetches the two fields returns them

Run on 10 Nov 2026, 02:42:02Z to 03:01:45Z (08:12 to 08:31 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/2464-gn-sheet` (23b2e57b), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no Bridge build. Two master changes on Tally's screen (row 2464-GR's second sitting, repeated), two imports (`I-0095.xml` and `I-0097.xml`, each sent once), then the restore of this sitting's own backup. Every value below is M, measured by this run, in one run. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

The GSTINs in this folder are the synthetic family's, as the sheet names them: the company's own `08ZZZZZ0000Z1ZQ` and the second registration's `27ZZZZZ0000Z1ZQ`.

## Result

**For the write, by the sheet's Decides (PARTIAL: one import of each shape, one sitting, one book, one release): "A or B is refused".** Both imports were answered alike, byte for byte (538 B each):

| counter | A, `I-0095.xml` (s5) | B, `I-0097.xml` (s7) | 46b's `I-0091.xml` (no registration named) |
|---|---|---|---|
| `CREATED` | 0 | 0 | 1 |
| `ALTERED` | 0 | 0 | 0 |
| `DELETED` | 0 | 0 | 0 |
| `LASTVCHID` | 0 | 0 | 45 |
| `LASTMID` | 0 | 0 | 0 |
| `COMBINED` | 0 | 0 | 0 |
| `IGNORED` | 0 | 0 | 0 |
| `ERRORS` | 0 | 0 | 0 |
| `CANCELLED` | 0 | 0 | 0 |
| `EXCEPTIONS` | 1 | 1 | 0 |

- Neither answer carries a LINEERROR or any other element: there are no words to record. Each window after the send was the Gateway, no dialog.
- After each, the number reads 0 rows by every read: N, R, RC and V for `0095` at s5r, and for `0097` and `0095` at s7r. ALTVCHID stayed 68 and ALTMSTID 248 through both.
- The sheet's line for this case: "The element, its place or its form is not accepted on this release." A adds one element to 46b's request, which Tally created: `<GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>`, right after `PARTYNAME`, with no attributes. B adds `<CMPGSTIN>27ZZZZZ0000Z1ZQ</CMPGSTIN>` after it. Which of element, place or form led to the exception is not among this row's reads.
- Step 6 found nothing A changed in the reads B depends on (T, U and ALTMSTID equal to s4's by the sheet's three checks), so B was sent in the same sitting.
- `V-0010` did not move at either later read: every voucher field equal to s2's by `--diff`.

**For the read: the candidate read-back returns both fields.** On `BP/26-27/0010` at s2, before any change, RC-0010 returned `GSTREGISTRATION` `Rajasthan Registration` and `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, the values Tally's native read (V-0010) gives. `--diff` of R-0010 against RC-0010: those two fields the only ones that differ, each from `<absent>`; legs 6 -> 6. One difference from the native read, recorded and not judged: in RC-0010, `GSTREGISTRATION` carries the attribute `TYPE="String"`, where V-0010's carries none. RC was not tried on an invoice with `Maharashtra Registration`, as neither import created one.

- **s1:** one COMPANY row, the book's: ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **BK:** taken before anything was keyed; restored at step 8.
- **s2, the book before:** every EXPECT met. U byte-equal to 46b's s2 (22,636 B, sha256 `e40ef83c...`): `Default Tax Unit` and `Rajasthan Registration`. T: `--type-diff` against 46b's s2, `BRIDGE Sales` 0 leaves differ. **N-0095 and N-0097: 0 rows each.** R-0010: `BRIDGE Walk-in`, included `Yes`, uncertain `No`, excluded `No`, 6 legs, `GSTREGISTRATION` and `CMPGSTIN` `<absent>`. RC-0010 as above. V-0010: `GSTREGISTRATION` `Rajasthan Registration` (no attributes), `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, `CMPGSTSTATE` `Rajasthan`, `CMPGSTREGISTRATIONTYPE` `Regular`, `PLACEOFSUPPLY` and `STATENAME` `Rajasthan`.
- **Step 3, the second registration:** the three named prompts, each in the sheet's words and answered with its key. s3: a third unit, `Maharashtra Registration`, `27ZZZZZ0000Z1ZQ`, GUID the book's with suffix `-000000e8`, one dated row from 20260401, Maharashtra, Regular, not inactive. `--unit-diff` and `--type-diff` against 46b's s3: 0 leaves differ in every unit and in `BRIDGE Sales`; both answers byte-equal to 46b's s3. ALTMSTID 245 to 246 (46b: 246).
- **Step 4, the binding:** `BRIDGE Sales`' Default GST Registration set from `Any` to `Maharashtra Registration`, no other field touched, saved with no prompt. s4: `--type "BRIDGE Sales" --type-diff` and `--unit-diff` against 46b's s4: 0 leaves differ, the three units on both sides; both answers byte-equal to 46b's s4. So `DEFAULTGSTREGISTRATION` `Maharashtra Registration`, `LASTGSTTAXUNIT` and `LASTGSTTAXUNITBYSERIES` `Rajasthan Registration`, `TAXUNITNAME` `Any`. ALTMSTID 246 to 248 (46b: 248). **The stop before import A did not apply.**
- **Step 5, import A:** the before-write check against s1 passed (s5m 68/248); `I-0095.xml` (sha256 equal to `SHA256.txt`) sent once, answered as in the table. s5r: N-0095, R-0095, RC-0095 and V-0095 0 rows each; `--diff` of R-0095 against RC-0095 printed "--diff needs one VOUCHER row in each: 0, 0" (exit 1). V-0010 unmoved. M 68/248.
- **Step 6, before B:** s6m: `--type-diff` and `--unit-diff` against s4: 0 leaves differ in `BRIDGE Sales` and in each of the three units; ALTMSTID 248, equal to s4's. The before-write check against s1 passed.
- **Step 7, import B:** `I-0097.xml` (sha256 equal to `SHA256.txt`) sent once, answered as in the table. s7r: N-0097, R-0097, RC-0097, V-0097 and V-0095 0 rows each; the two `--diff` compares that need a row each printed "--diff needs one VOUCHER row in each: 0, 0" (exit 1). V-0010 unmoved. M 68/248.
- **Step 8, the restore:** by C6, with `restore-plan.txt` written first; the two expected prompts. s8: every answer's sha256 equal to s2's (M, U, T, N-0095, N-0097, R-0010, RC-0010, V-0010), and M equal to s1 by `--same-row`. The book is as it was found: ALTVCHID 68, ALTMSTID 245, two units, `BRIDGE Sales` unbound.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** The sheet says "Close every other Bridge process first." Three Bridge processes were running at the start; on the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **A temp file removed by `rm -f`.** At 02:41:35Z, before Tally was started, a one-line temp file this row had made a minute earlier (`evidence/notes.txt.tmp`, holding only the bookcheck line, which is kept word for word in `evidence/notes.txt`) was removed by `rm -f`, not sent to the Recycle Bin as the run lead's rule asks. Nothing else was removed. It was reported to the run lead at once, who noted it and asked for it to be recorded here.

**Beyond the sheet, recorded and not judged:**
- At step 4.1 the first Up in Go To's list lit the heading row Saved Views; 46b recorded its first Up as lighting no visible row. The key and the rows that followed (Show More, Expand All, Alter Master) were the same.
- After the binding's Enter, the cursor moved to `Use common voucher numbering series for all GST Registrations` with that field's own Yes / No list open (No highlighted), as in 46b. No key was pressed on it; Ctrl+A saved.
- The Gateway's company line carried "(e)" with the footnote "(e) Data exceptions exist", at step 0 and after the reload at step 8 alike.
- The import answers' `EXCEPTIONS` 1 came with the DESC/CMPINFO count lines of the reads that followed moving (LAB-COMMON C4: these are not rows):

  | read | `LEDGER` | `VOUCHERTYPE` | `TAXUNIT` | `VOUCHERNUMBERSERIES` |
  |---|---|---|---|---|
  | s1, s2 (every read) | 0 | 0 | 0 | 0 |
  | s3, after the new unit | 0 | 0 | 1 | 0 |
  | s4, after the binding | 0 | 1 | 1 | 0 |
  | s5r, s6m, after A | 7 | 1 | 2 | 1 |
  | s7r, after B | 14 | 1 | 3 | 2 |
  | s8, after the restore | 0 | 0 | 0 | 0 |

  Every answer's counts are listed in `evidence/cmpinfo.txt`. 46b's s5r, after its created import, read 7, 1, 1 and 1. Every `--diff`, `--type-diff` and `--unit-diff` of the same reads shows no field or leaf moved.

## Step 1: load, check and back up
- Sheet files: all 28 equal to `SHA256.txt` at 23b2e57b. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `U-taxunits.xml`, `T-types.xml`, `R-0010.xml`, `V-0010.xml` and the three templates are byte-equal to row 2464-GR's; `a-46bs2-T-types.xml`, `a-46bs3-T-types.xml` and `a-46bs4-T-types.xml` byte-equal to 46b's committed answers. `gen.py`, rerun in a scratch copy, writes its 12 requests byte-equal to the sheet's. `count19.py` differs from row 1379-IT's `count18.py` only by its header and the lines marked GN.
- 46b's tax-unit answers at s3 and s4, kept on the lab PC, were copied beside this row's answers as `a-46bs3-U-taxunits.xml` and `a-46bs4-U-taxunits.xml`: 33,304 B each, sha256 `2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1` and `508dd8de3eb34f011a48debf7e86154657b129efc570d4ccd6a6ec5d96143621`, equal to 46b's `kept-local.txt`. They stay on the lab PC (they hold GSTINs).
- Before the start, the company's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 02:42:02Z, from its desktop shortcut, after Get-Process showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter, no prompt).
- C2 at 02:43:04Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded.
- The book was loaded alone: `BRIDGE` and ` PILOT` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; date of last entry 15-Sep-26.
- `s1`: as in the Result.
- **BK** (C5), by 46b's path, to a new folder:
  - Before the backup, `C:\lab\2464-gn\backup-1` was made new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times, checked), Enter, then `C:\lab\2464-gn\backup-1` typed in three chunks (Tally showed it as `C:\lab\2464-Gn\backup-1`, its own capital), and its row taken.
  - BRIDGE PILOT LAB alone, End of List (Up 3 times, checked). "Accept ?" was captured (`bk-accept`, 02:45:36Z) and answered y; no other message; the Gateway again.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `da9d3d6c568142e4440cf108ca1511ab61d479170ac7eb6e0590f1dffe64ace9`, written at 08:15 IST.

## Step 2: the book before
`s2` (02:46:13Z): `U-taxunits.xml T-types.xml N-0095.xml N-0097.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml`, each request checked against `SHA256.txt` just before its send. Every EXPECT met, as in the Result. `s3m` (02:46:40Z): M 68/245, and the before-write check against s1 passed; no key had been pressed in Tally since BK's y.

## Step 3: the second registration, keys and prompts
- 02:46:48Z: the Gateway (`s3-1-gateway`), Create highlighted; Enter: Master Creation, List of Masters, Group highlighted. Down 9 to Godown, Down 1 to the heading Statutory Masters, Down 1: GST Registration highlighted (each checked by zoom); Enter.
- **Named prompt 1** (02:47:41Z), word for word: "GST Registration already exists for the Company." / "Do you want to alter the existing GST Registration or create a new GST Registration?", with `C: Create New` and `A: Alter Existing` (`s3-2-exists-question`): the sheet's words. `c`.
- 02:48:00Z: GST Registration Creation (`s3-3-creation`), every field as Tally filled it: Registration status Active; State empty; Registration type Regular; Assessee of Other Territory No; GSTIN/UIN empty; Periodicity of GSTR-1 Monthly; GST Username empty; Mode of Filing Not Applicable; e-Invoicing applicable No; e-Way Bill applicable Yes; Applicable from 1-Apr-26; Applicable for intrastate Yes. As 46b's 3.3.
- State: `Maha` typed; List of States: New State, Not Applicable, Maharashtra (highlighted, the one state line); Enter. Registration type: list Regular, Composition, Regular - SEZ; Down once: Regular highlighted (checked); Enter. Assessee of Other Territory No, Enter. GSTIN/UIN: `27ZZZZZ000` and `0Z1ZQ` typed, checked after each; before Enter it read `27ZZZZZ0000Z1ZQ`, 15 characters, capitals (`s3-4-gstin-typed`, 02:48:58Z); Enter.
- Then Enter alone on each later field as Tally filled it: Periodicity of GSTR-1 (its list, Monthly highlighted) Monthly; GST Username empty; Mode of Filing (its list, Not Applicable highlighted) Not Applicable; e-Invoicing No; e-Way Bill Yes; Applicable from 1-Apr-26; intrastate Yes (`s3-5-form-filled`).
- **Named prompt 2:** on the last field's Enter, "Accept ?" Yes or No (`s3-6-accept`, 02:50:11Z). `y`.
- **Named prompt 3** (02:50:33Z, `s3-7-series-question`), three paragraphs on screen: "You can create a unique voucher numbering series to cater to multiple GST registrations." / "You can do this by going to the Chart of Accounts and selecting the relevant voucher types" / "Open Chart of Accounts and create now?", Yes or No. The sheet's words and punctuation, the second paragraph with no full stop. `n`.
- 02:50:49Z: a new, empty GST Registration Creation form (`s3-8-empty-form`), nothing typed in it. Ctrl+Q: Master Creation (`s3-9-master-creation`); Ctrl+Q: the Gateway, no dialog (`s3-10-gateway`). Two Ctrl+Q in all.
- `s3` (02:51:35Z): as in the Result.

## Step 4: the binding, keys and prompts
- `s4m` (02:51:52Z): ALTMSTID 246, equal to s3's.
- 02:52:00Z: the Gateway (`s4-1-gateway`); Go To by its label: List of Reports. Up 4: the heading Saved Views, then Show More, Expand All, Alter Master (each checked); Enter: Master Alteration, Group highlighted; Down 3 to Voucher Type (checked); Enter: List of Voucher Types, BRIDGE Sales highlighted (checked, `s4-2-type-list`); Enter.
- 02:53:28Z: Voucher Type Alteration (`s4-3-alteration`), every field as text: Name BRIDGE Sales; (alias) empty. General: Select type of voucher Sales; Abbreviation Sale; Activate this Voucher Type Yes; Method of Voucher Numbering Manual; Prevent creating duplicate Voucher Nos. Yes; Show unused vch nos. in transactions for Retain Original Voucher No. behaviour No (greyed); Use effective dates for vouchers No; Allow zero-valued transactions No; Make this voucher type as 'Optional' by default No; Allow narration in voucher Yes; Provide narrations for each ledger in voucher No; Enable default accounting allocations No; WhatsApp voucher after saving No. Printing: Print voucher after saving No; Use for POS invoicing No; Default template to print Not Applicable; Default title to print empty; Default bank Not Applicable; Default jurisdiction empty; Set/alter declaration No; Set/Alter Terms and Conditions No. Statutory Details: Default GST Registration Any; Use common voucher numbering series for all GST Registrations No. Name of Class empty. As 46b's 4.2.
- A click on the value of Default GST Registration: List of GST Registrations: Create; Any (highlighted); Maharashtra Registration; Rajasthan Registration. Down once: Maharashtra Registration highlighted (checked); Enter (`s4-4-bound`, 02:54:01Z). The cursor moved to Use common voucher numbering series for all GST Registrations: No, its own Yes / No list open; no other field touched.
- Ctrl+A: no prompt; the List of Voucher Types (`s4-5-saved-list`, 02:54:20Z). Ctrl+Q once: the Gateway, no dialog (`s4-6-gateway`).
- `s4` (02:54:50Z): as in the Result.

## Steps 5 to 7: the two imports
- `s5m` (02:55:09Z): ALTMSTID 248, equal to s4's; the before-write check against s1 passed (one COMPANY row, the same GUID). No key was pressed in Tally between s4's capture and the send.
- `s5` (02:55:10Z): `I-0095.xml` sent once; answered as in the Result; the window after it the Gateway, no dialog (`s5-after`).
- `s5r` (02:55:35Z): `N-0095.xml R-0095.xml RC-0095.xml V-0095.xml V-0010.xml M-marks.xml`.
- `s6m` (02:56:23Z): `T-types.xml U-taxunits.xml M-marks.xml`; the three checks as in the Result; the before-write check against s1 passed. No key was pressed in Tally before the send.
- `s7` (02:56:45Z): `I-0097.xml` sent once; answered as in the Result; the window after it the Gateway, no dialog (`s7-after`).
- `s7r` (02:57:09Z): `N-0097.xml R-0097.xml RC-0097.xml V-0097.xml V-0095.xml V-0010.xml M-marks.xml`.

## Step 8: restore (C6) and compare
- 02:57:40Z, before the shut: `prep-c6.sh 7` re-hashed BK equal to `evidence/bk.txt`, found its folder holding only that file, wrote `restore-plan.txt` (kept on the lab PC, as it names the company number) and listed the Tally data folder (kept on the lab PC).
- The book shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter: Select Company, no company loaded.
- The Data label, Backup & Restore, Restore: Restore Companies; Specify Path (Up 3, checked); `C:\lab\2464-gn\backup-1` typed in three chunks (shown `C:\lab\2464-Gn\backup-1`); Enter: one company row, BRIDGE PILOT LAB, its number, 10-Nov-26 (8:15 am), as the plan says (`c6-restore-list`); Enter on it; Enter on End of List.
- "Accept ?" (`c6-accept`): `y`. "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip (`c6-folder-exists`): `r`. The restore ran and Tally returned to Select Company with no other prompt.
- The book reloaded alone by the filter (`s8-gateway`): the Gateway as at step 0, the same period, date and date of last entry.
- `s8` (03:00:49Z), after C2 at 03:00:44Z: `M-marks.xml U-taxunits.xml T-types.xml N-0095.xml N-0097.xml R-0010.xml RC-0010.xml V-0010.xml`. Every compare equal, and every answer's sha256 equal to s2's.

## Step 9: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`).
- C2 at 03:01:34Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`. The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count19.py` output and compare, the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listings stay on the lab PC.
- `logs/`: each send's console output.
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
== a-s2-N-0095.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0097.xml: 3022 bytes
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
== a-s5-I-0095.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '1'
```

### s5r
```
== a-s5r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5r-N-0095.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s5r-R-0095.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s5r-RC-0095.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
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
== a-s5r-V-0095.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
```

### s6m
```
== a-s6m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6m-T-types.xml: 307042 bytes
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
== a-s6m-U-taxunits.xml: 33304 bytes
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

### s7
```
== a-s7-I-0097.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '1'
```

### s7r
```
== a-s7r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7r-N-0097.xml: 3024 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s7r-R-0097.xml: 3024 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s7r-RC-0097.xml: 3024 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s7r-V-0010.xml: 52786 bytes
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
== a-s7r-V-0095.xml: 3024 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s7r-V-0097.xml: 3024 bytes
  STATUS '1'
  VOUCHER rows: 0
```

### s8
```
== a-s8-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8-N-0095.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s8-N-0097.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s8-R-0010.xml: 94474 bytes
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
== a-s8-RC-0010.xml: 94744 bytes
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
== a-s8-T-types.xml: 306946 bytes
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
== a-s8-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
== a-s8-V-0010.xml: 52784 bytes
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

### s2: U against 46b's s2, T against 46b's s2 (--type-diff), R-0010 against RC-0010 (--diff)
```
== U vs 46b s2
e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 *a-s2-U-taxunits.xml
byte-equal to 46b's a-s2-U-taxunits.xml
== count19.py --type-diff a-46bs2-T-types.xml a-s2-T-types.xml
== --type-diff a-46bs2-T-types.xml a-s2-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
== count19.py --diff a-s2-R-0010.xml a-s2-RC-0010.xml
== --diff a-s2-R-0010.xml a-s2-RC-0010.xml
   CMPGSTIN: <absent> -> '08ZZZZZ0000Z1ZQ'
   GSTREGISTRATION: <absent> -> 'Rajasthan Registration'
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s3: U and T against 46b's s3 (--unit-diff, --type-diff)
```
== count19.py --unit-diff a-46bs3-U-taxunits.xml a-s3-U-taxunits.xml
== --unit-diff a-46bs3-U-taxunits.xml a-s3-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
== count19.py --type-diff a-46bs3-T-types.xml a-s3-T-types.xml
== --type-diff a-46bs3-T-types.xml a-s3-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
```

### s4: T, BRIDGE Sales, and U against 46b's s4 (the check before import A)
```
== count19.py --type "BRIDGE Sales" --type-diff a-46bs4-T-types.xml a-s4-T-types.xml
== --type-diff a-46bs4-T-types.xml a-s4-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
== count19.py --unit-diff a-46bs4-U-taxunits.xml a-s4-U-taxunits.xml
== --unit-diff a-46bs4-U-taxunits.xml a-s4-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s5r: R-0095 against RC-0095, V-0010 against s2 (--diff)
```
== count19.py --diff a-s5r-R-0095.xml a-s5r-RC-0095.xml
--diff needs one VOUCHER row in each: 0, 0
exit 1
== count19.py --diff a-s2-V-0010.xml a-s5r-V-0010.xml
== --diff a-s2-V-0010.xml a-s5r-V-0010.xml
   lists compared: 2
   legs: 0 -> 0
exit 0
--- the DESC/CMPINFO counters of a-s2-V-0010.xml against a-s5r-V-0010.xml (every line that differs; 788 lines each)
line 11 <LEDGER>0</LEDGER> -> <LEDGER>7</LEDGER>
line 18 <VOUCHERTYPE>0</VOUCHERTYPE> -> <VOUCHERTYPE>1</VOUCHERTYPE>
line 39 <TAXUNIT>0</TAXUNIT> -> <TAXUNIT>2</TAXUNIT>
line 42 <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES> -> <VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>
46b's a-s5r-V-0010.xml and a-s5r-V-0091.xml (after its import) read LEDGER 7, VOUCHERTYPE 1, TAXUNIT 1, VOUCHERNUMBERSERIES 1; its s2 and s6 read 0 in each.
```

### s6m: T and U against s4 (--type-diff, --unit-diff)
```
== count19.py --type-diff a-s4-T-types.xml a-s6m-T-types.xml
== --type-diff a-s4-T-types.xml a-s6m-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
== count19.py --unit-diff a-s4-U-taxunits.xml a-s6m-U-taxunits.xml
== --unit-diff a-s4-U-taxunits.xml a-s6m-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
T-types: lines a=4445, b=4445; the lines that differ (line number, a, b): [(11, '<LEDGER>0</LEDGER>', '<LEDGER>7</LEDGER>'), (39, '<TAXUNIT>1</TAXUNIT>', '<TAXUNIT>2</TAXUNIT>'), (42, '<VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>', '<VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>')]
U-taxunits: lines a=447, b=447; the lines that differ (line number, a, b): [(11, '<LEDGER>0</LEDGER>', '<LEDGER>7</LEDGER>'), (39, '<TAXUNIT>1</TAXUNIT>', '<TAXUNIT>2</TAXUNIT>'), (42, '<VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>', '<VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>')]
(the lines above, from a line-by-line compare: every line that differs between a-s4 and a-s6m, all in DESC/CMPINFO)
```

### s6m: M against s1 (the before-write check)
```
== a-s6m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s7r: R-0097 against RC-0097, V-0095 against s5r, V-0010 against s2 (--diff)
```
== count19.py --diff a-s7r-R-0097.xml a-s7r-RC-0097.xml
--diff needs one VOUCHER row in each: 0, 0
exit 1
== count19.py --diff a-s5r-V-0095.xml a-s7r-V-0095.xml
--diff needs one VOUCHER row in each: 0, 0
exit 1
== count19.py --diff a-s2-V-0010.xml a-s7r-V-0010.xml
== --diff a-s2-V-0010.xml a-s7r-V-0010.xml
   lists compared: 2
   legs: 0 -> 0
exit 0
s5r-V-0010 against s7r-V-0010: lines a=788, b=788; the lines that differ (line number, a, b): [(11, '<LEDGER>7</LEDGER>', '<LEDGER>14</LEDGER>'), (39, '<TAXUNIT>2</TAXUNIT>', '<TAXUNIT>3</TAXUNIT>'), (42, '<VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>', '<VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES>')]
s5r-V-0095 against s7r-V-0095: lines a=50, b=50; the lines that differ (line number, a, b): [(11, '<LEDGER>7</LEDGER>', '<LEDGER>14</LEDGER>'), (39, '<TAXUNIT>2</TAXUNIT>', '<TAXUNIT>3</TAXUNIT>'), (42, '<VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>', '<VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES>')]
s5r-M-marks against s7r-M-marks: lines a=56, b=56; the lines that differ (line number, a, b): [(11, '<LEDGER>7</LEDGER>', '<LEDGER>14</LEDGER>'), (39, '<TAXUNIT>2</TAXUNIT>', '<TAXUNIT>3</TAXUNIT>'), (42, '<VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>', '<VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES>')]
s2-V-0010 against s7r-V-0010: lines a=788, b=788; the lines that differ (line number, a, b): [(11, '<LEDGER>0</LEDGER>', '<LEDGER>14</LEDGER>'), (18, '<VOUCHERTYPE>0</VOUCHERTYPE>', '<VOUCHERTYPE>1</VOUCHERTYPE>'), (39, '<TAXUNIT>0</TAXUNIT>', '<TAXUNIT>3</TAXUNIT>'), (42, '<VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>', '<VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES>')]
(the lines above, from a line-by-line compare: every line that differs, all in DESC/CMPINFO)
```

### s8: against s1 and s2 (--same-row, --unit-diff, --type-diff, --diff, sha256)
```
== count19.py --same-row a-s1-M-marks.xml a-s8-M-marks.xml
== a-s8-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
== count19.py --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
== count19.py --type-diff a-s2-T-types.xml a-s8-T-types.xml
== --type-diff a-s2-T-types.xml a-s8-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
== count19.py --diff a-s2-R-0010.xml a-s8-R-0010.xml
== --diff a-s2-R-0010.xml a-s8-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
== count19.py --diff a-s2-RC-0010.xml a-s8-RC-0010.xml
== --diff a-s2-RC-0010.xml a-s8-RC-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
== sha256 s2 against s8
M-marks equal 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
U-taxunits equal e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
T-types equal cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
N-0095 equal 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
N-0097 equal 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
R-0010 equal b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
RC-0010 equal 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
V-0010 equal 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every answer that are not 0 (LAB-COMMON C4: not rows)
a-s1-M-marks.xml: all 0
a-s2-M-marks.xml: all 0
a-s2-N-0095.xml: all 0
a-s2-N-0097.xml: all 0
a-s2-R-0010.xml: all 0
a-s2-RC-0010.xml: all 0
a-s2-T-types.xml: all 0
a-s2-U-taxunits.xml: all 0
a-s2-V-0010.xml: all 0
a-s3-M-marks.xml: TAXUNIT 1
a-s3-T-types.xml: TAXUNIT 1
a-s3-U-taxunits.xml: TAXUNIT 1
a-s3m-M-marks.xml: all 0
a-s4-M-marks.xml: VOUCHERTYPE 1, TAXUNIT 1
a-s4-T-types.xml: VOUCHERTYPE 1, TAXUNIT 1
a-s4-U-taxunits.xml: VOUCHERTYPE 1, TAXUNIT 1
a-s4m-M-marks.xml: TAXUNIT 1
a-s5-I-0095.xml: no CMPINFO (an import answer)
a-s5m-M-marks.xml: VOUCHERTYPE 1, TAXUNIT 1
a-s5r-M-marks.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s5r-N-0095.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s5r-R-0095.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s5r-RC-0095.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s5r-V-0010.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s5r-V-0095.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s6m-M-marks.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s6m-T-types.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s6m-U-taxunits.xml: LEDGER 7, VOUCHERTYPE 1, TAXUNIT 2, VOUCHERNUMBERSERIES 1
a-s7-I-0097.xml: no CMPINFO (an import answer)
a-s7r-M-marks.xml: LEDGER 14, VOUCHERTYPE 1, TAXUNIT 3, VOUCHERNUMBERSERIES 2
a-s7r-N-0097.xml: LEDGER 14, VOUCHERTYPE 1, TAXUNIT 3, VOUCHERNUMBERSERIES 2
a-s7r-R-0097.xml: LEDGER 14, VOUCHERTYPE 1, TAXUNIT 3, VOUCHERNUMBERSERIES 2
a-s7r-RC-0097.xml: LEDGER 14, VOUCHERTYPE 1, TAXUNIT 3, VOUCHERNUMBERSERIES 2
a-s7r-V-0010.xml: LEDGER 14, VOUCHERTYPE 1, TAXUNIT 3, VOUCHERNUMBERSERIES 2
a-s7r-V-0095.xml: LEDGER 14, VOUCHERTYPE 1, TAXUNIT 3, VOUCHERNUMBERSERIES 2
a-s7r-V-0097.xml: LEDGER 14, VOUCHERTYPE 1, TAXUNIT 3, VOUCHERNUMBERSERIES 2
a-s8-M-marks.xml: all 0
a-s8-N-0095.xml: all 0
a-s8-N-0097.xml: all 0
a-s8-R-0010.xml: all 0
a-s8-RC-0010.xml: all 0
a-s8-T-types.xml: all 0
a-s8-U-taxunits.xml: all 0
a-s8-V-0010.xml: all 0
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s2-N-0095.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0097.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s2-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
a-s2-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3-U-taxunits.xml 33304 B 2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1
a-s3-T-types.xml 306946 B 3239e41dfed99766ac1ee6da5724eea72ca206f55113d729bb0c998d56402425
a-s3-M-marks.xml 3394 B f1845d9f7649984fc39f0929f60c86eb6b255f0def12cb8c1e867ef7350b14ea
a-s4m-M-marks.xml 3394 B f1845d9f7649984fc39f0929f60c86eb6b255f0def12cb8c1e867ef7350b14ea
a-s4-T-types.xml 307042 B 16bbd91c375cc6f75ee5913b66b7c46c254fd41a1b869caa33941b49f912e655
a-s4-U-taxunits.xml 33304 B 508dd8de3eb34f011a48debf7e86154657b129efc570d4ccd6a6ec5d96143621
a-s4-M-marks.xml 3394 B 6ae4f3a78b93fb3e59bdbb822868d4826850cf823173d7d998c1aaf213f048e6
a-s5m-M-marks.xml 3394 B 6ae4f3a78b93fb3e59bdbb822868d4826850cf823173d7d998c1aaf213f048e6
a-s5-I-0095.xml 538 B 09a713e2674c7016c2cb10f194f6e0fc7f6bf9dee3b523f4f6bbf1b07ea0f874
a-s5r-N-0095.xml 3022 B cd07dc8e9cc541b8c761a937e23ae7220282fa24524903b6593705bf00c585df
a-s5r-R-0095.xml 3022 B cd07dc8e9cc541b8c761a937e23ae7220282fa24524903b6593705bf00c585df
a-s5r-RC-0095.xml 3022 B cd07dc8e9cc541b8c761a937e23ae7220282fa24524903b6593705bf00c585df
a-s5r-V-0095.xml 3022 B cd07dc8e9cc541b8c761a937e23ae7220282fa24524903b6593705bf00c585df
a-s5r-V-0010.xml 52784 B 2fecfdf8d541074534cdc83b1d09fefcc17399c3eea363c17a6e44ee543d1510
a-s5r-M-marks.xml 3394 B 8cc84a741f950e24d62e9888f8fe85ceae1d00ac1d9ec269021ece3d98b53ca0
a-s6m-T-types.xml 307042 B 6c63d04e008a4ce44c71b1267bf35ffeda2e289011d3a1a20314784aa02ce7cb
a-s6m-U-taxunits.xml 33304 B e64781dd8f7de579809aae1acafbc39e42ec75697141c58f3f1cfd7ad0937363
a-s6m-M-marks.xml 3394 B 8cc84a741f950e24d62e9888f8fe85ceae1d00ac1d9ec269021ece3d98b53ca0
a-s7-I-0097.xml 538 B 09a713e2674c7016c2cb10f194f6e0fc7f6bf9dee3b523f4f6bbf1b07ea0f874
a-s7r-N-0097.xml 3024 B 1e37e2bd243fef95abd2d6cae45ebd6ca5001d7b61f763cc494556b75d18280a
a-s7r-R-0097.xml 3024 B 1e37e2bd243fef95abd2d6cae45ebd6ca5001d7b61f763cc494556b75d18280a
a-s7r-RC-0097.xml 3024 B 1e37e2bd243fef95abd2d6cae45ebd6ca5001d7b61f763cc494556b75d18280a
a-s7r-V-0097.xml 3024 B 1e37e2bd243fef95abd2d6cae45ebd6ca5001d7b61f763cc494556b75d18280a
a-s7r-V-0095.xml 3024 B 1e37e2bd243fef95abd2d6cae45ebd6ca5001d7b61f763cc494556b75d18280a
a-s7r-V-0010.xml 52786 B e65c6bc0502afeda558fcf60033f9d796479d506ec0584a24d147c326819830e
a-s7r-M-marks.xml 3396 B e54f6a22a1cda6a924cad10574f2aeae6bfd8b53cc6de61ad9d89deed411b44b
a-s8-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s8-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s8-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s8-N-0095.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-N-0097.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s8-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
a-s8-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
```
