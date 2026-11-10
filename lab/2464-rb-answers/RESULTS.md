# Row 2464-RB: with two GST registrations and `BRIDGE Sales` bound to `Rajasthan Registration`, both imports naming that unit were created; each reads uncertain, and each stored the unit's name with `CMPGSTSTATE` and `CMPGSTREGISTRATIONTYPE` empty

Run on 10 Nov 2026, 06:03:05Z to 06:20:35Z (11:33 to 11:50 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as rows 2464-GN and 2464-NR read it from the program's file; the Gateway shows no release number), by the sheet at `lab/2464-rb-sheet` (16e3593a), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no Bridge build. Two master changes on Tally's screen (row 2464-GN's steps 3 and 4, with the one key the sheet changes), two imports (`I-0105.xml` and `I-0107.xml`, each sent once), then the restore of this sitting's own backup. Every read was answered, each within a few seconds. The window after each import was the Gateway, no dialog.

The GSTINs in this folder are the synthetic family's, as the sheet names them: the company's own `08ZZZZZ0000Z1ZQ` and the second registration's `27ZZZZZ0000Z1ZQ`.

This sitting ran after row 2346-PB (Tally quit at 05:48:32Z), with a Tally start of its own.

## Result

PARTIAL by the sheet's Decides: one import of each shape, one sitting, one book, one release.

**Both imports were created.**

| counter | A'', `I-0105.xml` (s5) | B'', `I-0107.xml` (s7) |
|---|---|---|
| `CREATED` | 1 | 1 |
| `ALTERED` | 0 | 0 |
| `DELETED` | 0 | 0 |
| `LASTVCHID` | 45 | 46 |
| `LASTMID` | 0 | 0 |
| `COMBINED` | 0 | 0 |
| `IGNORED` | 0 | 0 |
| `ERRORS` | 0 | 0 |
| `CANCELLED` | 0 | 0 |
| `EXCEPTIONS` | 0 | 0 |

- A'''s answer (540 B, sha256 `fdcd8435...`) is byte-equal to row 2464-NR's created A' answer, not to row 2464-GN's refused A answer (538 B). B'''s answer differs from A'''s in one byte, `LASTVCHID`'s last digit.
- Neither answer carries a LINEERROR. Each window after the send was the Gateway, no dialog.
- N-0105 (s5r) and N-0107 (s7r) each read one row, `BRIDGEVCHISSALES` `Yes`, GUID tails `0000002d` and `0000002e`.

**The status: uncertain on both.** R-0105 and R-0107 each read `VCHGSTSTATUSISINCLUDED` `No`, `VCHGSTSTATUSISUNCERTAIN` `Yes`, `VCHGSTSTATUSISEXCLUDED` `No`, `ISGSTOVERRIDDEN` `No`, with the six legs of `BP/26-27/0010` (`BRIDGE Walk-in` -1051.00; 600.40, 400.40, 25.02, 25.02, 0.16). `--diff` of s2's R-0010 against R-0105: `VCHGSTSTATUSISINCLUDED` `Yes` -> `No` and `VCHGSTSTATUSISUNCERTAIN` `No` -> `Yes`, beside the ids, number, reference, `VOUCHERKEY` and `VOUCHERRETAINKEY`. `--diff` of R-0105 against R-0107: no status flag differs.

**What each invoice stored (V, Tally's native read of one voucher), beside `BP/26-27/0010`'s:**

| field | V-0010 (s2) | V-0105, A'' (s5r) | V-0107, B'' (s7r) |
|---|---|---|---|
| `GSTREGISTRATION` | `Rajasthan Registration` (no attribute) | `Rajasthan Registration` (no attribute) | `Rajasthan Registration` (no attribute) |
| `CMPGSTIN` | `08ZZZZZ0000Z1ZQ` | `''` | `08ZZZZZ0000Z1ZQ` |
| `CMPGSTSTATE` | `Rajasthan` | `''` | `''` |
| `CMPGSTREGISTRATIONTYPE` | `Regular` | `''` | `''` |
| `PLACEOFSUPPLY` | `Rajasthan` | `Rajasthan` | `Rajasthan` |
| `STATENAME` | `Rajasthan` | `Rajasthan` | `Rajasthan` |
| status flags (included, uncertain, excluded) | Yes, No, No | No, Yes, No | No, Yes, No |

**The read-back (RC).** RC-0105 returned `GSTREGISTRATION` `Rajasthan Registration` (attribute `TYPE="String"`) and `CMPGSTIN` `''`; RC-0107 returned `Rajasthan Registration` (`TYPE="String"`) and `08ZZZZZ0000Z1ZQ`. Each equals V's value for the same invoice. `--diff` of R against RC for each invoice: those two fields the only ones that differ, each from `<absent>`; legs 6 -> 6.

By the sheet's Decides, quoted:
- **A'':** "A'' is created but stores the registration fields some other way (empty, `Maharashtra Registration`, or values other than `0010`'s): Tally does not keep the element as named on this book. Record all six whole; [the run lead] decides." The name is kept; `CMPGSTIN`, `CMPGSTSTATE` and `CMPGSTREGISTRATIONTYPE` are empty where `0010`'s hold the unit's values. It also reads uncertain, the case of "A'' is created and stored so, but R-0105 reads uncertain: naming the fitting, bound unit does not clear the status on this book. The cause is not in the answer."
- **B'':** "B'' is created while A'' was: compare V-0107's six fields and R-0107's flags with A'''s. ... Different: record whole; [the run lead] decides." The flags are equal; one of the six differs: `CMPGSTIN`, empty on A'' and `08ZZZZZ0000Z1ZQ` on B'' (the value B'' wrote). `CMPGSTSTATE` and `CMPGSTREGISTRATIONTYPE` stay empty on both.
- **RC:** "RC returns `GSTREGISTRATION` and `CMPGSTIN` equal to V's, with every other field equal to R's: the candidate read-back reads the registration of a two-registration invoice."
- **V-0010:** it did not move at s5r or s7r: `--diff` against s2 finds no field that differs. Nor did V-0105 between s5r and s7r.

**Step 6 (before B''):** the one move the sheet allows, `BRIDGE Sales` `/LASTNUMBER[1]` `BP/26-27/0044` -> `BP/26-27/0105`, and nothing else: units 0 leaves differ, ALTMSTID 248 as at s4. So B'' was sent in this sitting.

**The set-up as the sheet expected:** s2 byte-equal to row 2464-GN's s2 in all eight answers. Step 3 as GN's: U byte-equal to GN's s3, T byte-equal to `a-46bs3-T-types.xml`, ALTMSTID 246. Step 4: `--type-diff` s3 against s4, one type line, `BRIDGE Sales` 2 leaves differ: `/ALTERID[1]` `212` -> `248` and `/DEFAULTGSTREGISTRATION[1]` `''` -> `Rajasthan Registration`; units 0 leaves differ; ALTMSTID 248.

**The way back (step 8).** Restored by C6 with only the two expected prompts. s8: every answer's sha256 equal to s2's (M, U, T, N-0105, N-0107, R-0010, RC-0010, V-0010); M equal to s1 by `--same-row` (68/245); two units, `BRIDGE Sales` 0 leaves differ from s2.

**The sheet's estimates:** s2, step 3, step 4 and s8 each as estimated. Step 5 and the status were not estimated.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **The start-up company.** Tally's start-up company loaded by itself, as the install is set to. It was shut by the K: Company label, Shut, Enter on it, before step 0, on the run lead's standing practice for these sittings, with no change to it.
3. **One key refused for focus.** At step 4.3, the first Enter on `Rajasthan Registration` was refused by the desktop-control tool because the desktop client had the keyboard focus; no key reached Tally. A click on Tally's title bar and a capture showed the screen unchanged, `Rajasthan Registration` still highlighted, before the Enter was pressed.
4. **One temporary file removed.** During the sheet checks, before Tally was started, one temporary file in the shell's temp folder (a `git show` copy of a committed row 2464-NR sheet file, written for a byte compare) was removed by `rm -f`, against the lab's no-delete rule. No lab file, answer or evidence was touched. Recorded in `evidence/notes.txt`.

**Beyond the sheet, recorded and not judged:**
- V-0010 and V-0105 answers at later reads differ in bytes from the earlier ones only in their DESC/CMPINFO count lines (LAB-COMMON C4: not rows; `evidence/cmpinfo.txt`). s8's V-0010 is byte-equal to s2's.
- The Gateway's company line carried "(e)" and "(Schedule Backup)" at step 1, as at GN's.

## Step 1: load, check and back up
- Sheet files: all 27 equal to `SHA256.txt` at 16e3593a (sizes too). The 13 files the sheet names as row 2464-NR's are byte-equal to NR's sheet (de14c060); `a-46bs2-T-types.xml` and `a-46bs3-T-types.xml` byte-equal to row 2464-GN's, and GN's s3 type answer byte-equal to `a-46bs3-T-types.xml`; `template-import-0010.utf16le.xml` sha256 `b333f0dc...`. `gen.py` was run again in a new folder with copies of its 4 inputs: the 12 files it writes are byte-equal to the sheet's.
- The six `a-gns` copies were made from row 2464-GN's answers, each size and sha256 as the sheet lists (`evidence/gns-copies.txt`); the two committed ones are also byte-equal to GN's answers branch.
- Before the start, the book's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`). The backup folder was made new and empty.
- Tally was started once, at 06:03:05Z, from its desktop shortcut, after Get-Process showed none. Its start-up company was shut first (deviation 2).
- C2 at 06:04:21Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE`, ` PILOT` and ` LAB` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s1-select`), Enter, no prompt. The Gateway (`s1-gateway`): current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; date of last entry 15-Sep-26.
- `s1`: one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245. EXPECT met.
- BK by C5 (06:05Z to 06:06Z) to `C:\lab\2464-rb\backup-1` (Tally showed `C:\lab\2464-Rb\backup-1`, as the sheet says it may): Data label, Backup & Restore, Backup; Backspace to the path; Specify Path; the path typed in two chunks; Enter, Enter; `BRIDGE`, ` PILOT` and ` LAB` typed, the one row, Enter; End of List, Enter; "Accept ?" (`bk-accept`, 06:06:29Z) = y. One file, 90,051 B; the Restore list showed it as 10-Nov-26 (11:36 am).

## Step 2: the book before
- `s2` (06:07Z): each of the eight answers byte-equal to row 2464-GN's s2 answer of the same kind (`evidence/compare-s2.txt`; N-0105 and N-0107 each against GN's `a-s2-N-0095.xml`, 3,022 B). So two units; `BRIDGE Sales` unbound; 0 rows for each number; `BP/26-27/0010` with `Rajasthan Registration` and `08ZZZZZ0000Z1ZQ` in V and RC. EXPECT met.

## Step 3: a second GST registration
- `s3m` and the before-write check against s1 passed (`evidence/bw-s3m.txt`).
- Row 2464-GN's step 3, key for key and prompt for prompt (the keys, screens and times are in `evidence/notes.txt`, items 3.1 to 3.7):
  - Create, then Down 9, 1 and 1 to GST Registration, Enter.
  - Named prompt 1 (`s3-2-exists-question`): "GST Registration already exists for the Company." / "Do you want to alter the existing GST Registration or create a new GST Registration?", C: Create New and A: Alter Existing; `c`.
  - GST Registration Creation (`s3-3-creation`), every field as GN recorded it. State: `Maha` typed, Maharashtra; Registration type: Down once, Regular; Assessee of Other Territory No; GSTIN/UIN `27ZZZZZ000` and `0Z1ZQ` typed, zoomed after each (`s3-4-gstin-typed`); then Enter alone on each field (`s3-5-form-filled`).
  - Named prompt 2, "Accept ?" (`s3-6-accept`); `y`.
  - Named prompt 3 (`s3-7-series-question`), three paragraphs in GN's words; `n`.
  - A new, empty form (`s3-8-empty-form`); Ctrl+Q to Master Creation (`s3-9-master-creation`); Ctrl+Q to the Gateway, no dialog (`s3-10-gateway`).
- `s3` (06:11Z): `--unit-diff` against GN's s3: `Default Tax Unit`, `Maharashtra Registration` and `Rajasthan Registration` on both sides, 0 leaves differ in each. `--type-diff` against `a-46bs3-T-types.xml`: `BRIDGE Sales` 0 leaves differ, rows 28 -> 28. ALTMSTID 246 (GN: 246).

## Step 4: bind BRIDGE Sales to `Rajasthan Registration`
- `s4m`: ALTVCHID 68, ALTMSTID 246.
- 4.1: Go To clicked by its label; Up 4 to Alter Master (each zoomed); Enter; Down 3 to Voucher Type; Enter; `BRIDGE Sales` highlighted (`s4-2-type-list`); Enter.
- 4.2: Voucher Type Alteration (`s4-3-alteration`), every field as GN's 4.2 (written out in `evidence/notes.txt`); Default GST Registration read `Any`, as EXPECT.
- 4.3: a click on the value of Default GST Registration: List of GST Registrations: Create; Any (highlighted); Maharashtra Registration; Rajasthan Registration. Down twice: `Rajasthan Registration` highlighted, `Maharashtra Registration` not (zoomed after each). Enter (after deviation 3) (`s4-4-bound`): the cursor moved to "Use common voucher numbering series for all GST Registrations: No" with its own Yes / No list open and No highlighted. No key was pressed on that list, and no other field touched.
- 4.4: Ctrl+A, no prompt, the List of Voucher Types (`s4-5-saved-list`); Ctrl+Q once, the Gateway (`s4-6-gateway`).
- `s4` (06:14Z): as in the Result. Both of step 4's checks held.

## Step 5: import A''
- `s5m` (68/248) and the before-write check against s1 passed. `I-0105.xml` (sha256 equal to `SHA256.txt`) sent once at 06:14:59Z; answered as in the Result; `s5-after`: the Gateway, no dialog.
- `s5r`: as in the Result (`evidence/count-s5r.txt`; the diffs in `evidence/diff-s5r-*.txt`). M ALTVCHID 69 (s5m 68), ALTMSTID 248 (as s5m).

## Step 6: before B''
- `s6m`: as in the Result (`evidence/typediff-s6m.txt`, `evidence/unitdiff-s6m.txt`); ALTVCHID 69, ALTMSTID 248. The before-write check against s1 passed.

## Step 7: import B''
- `I-0107.xml` (sha256 equal to `SHA256.txt`) sent once at 06:16:03Z; answered as in the Result; `s7-after`: the Gateway, no dialog.
- `s7r`: as in the Result (`evidence/count-s7r.txt`; the diffs in `evidence/diff-s7r-*.txt`). M ALTVCHID 70 (s6m 69), ALTMSTID 248 (as s6m).

## Step 8: restore (C6) and compare
- `prep-c6.sh 8` first: BK re-hashed equal to `evidence/bk.txt`, its folder holding only that file; `restore-plan.txt` written; the data folder listed (kept on the lab PC).
- K: Company label, Shut, Enter on BRIDGE PILOT LAB, no prompt; Data label, Backup & Restore, Restore; Specify Path; the path typed in two chunks; the list (`restore-list`) showed one row, BRIDGE PILOT LAB, 10-Nov-26 (11:36 am), as the plan; Enter on it; End of List, Enter; "Accept ?" (`restore-accept`, 06:18:25Z) = y; "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (`restore-replace`, 06:18:40Z) = r. No other message.
- The book loaded alone (`s8-gateway`): period, date and last entry as at step 1. C2 (s8pre) as at step 1.
- `s8` (06:19Z): as in the Result (`evidence/compares-s8.txt`, `evidence/compare-s8.txt`).

## Step 9: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 06:20:16Z).
- C2 at 06:20:16Z, after the last request: one `tally.exe` (the same process as at step 1), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`, and no line listened on 9001 (06:20:35Z). The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason; under the lab's standing rules that covers every answer holding a GSTIN shape (each U, each V but V-0105, RC-0010 at s2 and s8, and RC-0107) and both V-0105 answers (a run of 10 or more digits in `REMOTEGUID`); RC-0105 goes, its `CMPGSTIN` being empty. The six `a-gns` copies of row 2464-GN's answers are listed there too, and are not pushed again).
- `evidence/`: notes, every `count19.py` output, the diffs and compares, the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listings stay on the lab PC; the BK record (`bk.txt`) goes with the company number replaced.
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
== a-s2-N-0105.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0107.xml: 3022 bytes
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
== a-s4-T-types.xml: 307038 bytes
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
== a-s5-I-0105.xml: 540 bytes
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
== a-s5r-N-0105.xml: 5606 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0105' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s5r-R-0105.xml: 87968 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0105'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0105'
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
== a-s5r-RC-0105.xml: 88208 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0105'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0105'
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
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN ''
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
== a-s5r-V-0105.xml: 52630 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0105'
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
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN ''
   CMPGSTSTATE ''
   CMPGSTREGISTRATIONTYPE ''
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0105'
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

### s6m
```
== a-s6m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6m-T-types.xml: 307038 bytes
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
== a-s7-I-0107.xml: 540 bytes
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

### s7r
```
== a-s7r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7r-N-0107.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0107' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s7r-R-0107.xml: 87970 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0107'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0107'
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
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
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
== a-s7r-RC-0107.xml: 88240 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0107'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0107'
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
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
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
== a-s7r-V-0105.xml: 52632 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0105'
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
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN ''
   CMPGSTSTATE ''
   CMPGSTREGISTRATIONTYPE ''
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0105'
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
== a-s7r-V-0107.xml: 52682 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID '46'
   ALTERID '70'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0107'
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
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE ''
   CMPGSTREGISTRATIONTYPE ''
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0107'
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

### s8
```
== a-s8-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8-N-0105.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s8-N-0107.xml: 3022 bytes
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

### the a-gns copies of row 2464-GN's answers
```
a-gns2-U-taxunits.xml: copied from row 2464-GN's a-s2-U-taxunits.xml, 22636 B, sha256 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 (as the sheet lists)
a-gns2-RC-0010.xml: copied from row 2464-GN's a-s2-RC-0010.xml, 94744 B, sha256 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399 (as the sheet lists)
a-gns2-V-0010.xml: copied from row 2464-GN's a-s2-V-0010.xml, 52784 B, sha256 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456 (as the sheet lists)
a-gns3-U-taxunits.xml: copied from row 2464-GN's a-s3-U-taxunits.xml, 33304 B, sha256 2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1 (as the sheet lists)
a-gns2-T-types.xml: copied from row 2464-GN's a-s2-T-types.xml, 306946 B, sha256 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed (as the sheet lists)
a-gns2-R-0010.xml: copied from row 2464-GN's a-s2-R-0010.xml, 94474 B, sha256 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 (as the sheet lists)
```

### s2: each answer against row 2464-GN's s2 (sha256)
```
a-s2-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 against a-gns2-U-taxunits.xml: equal
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed against a-gns2-T-types.xml: equal
a-s2-N-0105.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 against a-s2-N-0095.xml: equal
a-s2-N-0107.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 against a-s2-N-0095.xml: equal
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 against a-gns2-R-0010.xml: equal
a-s2-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399 against a-gns2-RC-0010.xml: equal
a-s2-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456 against a-gns2-V-0010.xml: equal
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 against a-s2-M-marks.xml: equal
```

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s3: U against row 2464-GN's s3 (--unit-diff)
```
== --unit-diff a-gns3-U-taxunits.xml a-s3-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### s3: T against a-46bs3-T-types.xml (--type-diff)
```
== --type-diff a-46bs3-T-types.xml a-s3-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### s4: T against s3 (--type-diff)
```
== --type-diff a-s3-T-types.xml a-s4-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 2 leaves differ
     /ALTERID[1]: '212' -> '248'
     /DEFAULTGSTREGISTRATION[1]: '' -> 'Rajasthan Registration'
```

### s4: U against s3 (--unit-diff)
```
== --unit-diff a-s3-U-taxunits.xml a-s4-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s5r: R-0010 (s2) against R-0105 (--diff)
```
== --diff a-s2-R-0010.xml a-s5r-R-0105.xml
   ALTERID: '17' -> '69'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0105'
   VCHGSTSTATUSISINCLUDED: 'Yes' -> 'No'
   VCHGSTSTATUSISUNCERTAIN: 'No' -> 'Yes'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0105'
   VOUCHERRETAINKEY: '41' -> '129'
   list ALLLEDGERENTRIES.LIST: 6 -> 6, 2 differ
   lists compared: 1
   legs: 6 -> 6
```

### s5r: R-0105 against RC-0105 (--diff)
```
== --diff a-s5r-R-0105.xml a-s5r-RC-0105.xml
   CMPGSTIN: <absent> -> ''
   GSTREGISTRATION: <absent> -> 'Rajasthan Registration'
   lists compared: 1
   legs: 6 -> 6
```

### s5r: V-0010 against s2 (--diff)
```
== --diff a-s2-V-0010.xml a-s5r-V-0010.xml
   lists compared: 2
   legs: 0 -> 0
```

### s6m: T against s4 (--type-diff)
```
== --type-diff a-s4-T-types.xml a-s6m-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 1 leaves differ
     /LASTNUMBER[1]: 'BP/26-27/0044' -> 'BP/26-27/0105'
```

### s6m: U against s4 (--unit-diff)
```
== --unit-diff a-s4-U-taxunits.xml a-s6m-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### s6m: M against s1 (the before-write check)
```
== a-s6m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s7r: R-0105 (s5r) against R-0107 (--diff)
```
== --diff a-s5r-R-0105.xml a-s7r-R-0107.xml
   ALTERID: '69' -> '70'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID: '45' -> '46'
   REFERENCE: 'BP/26-27/0105' -> 'BP/26-27/0107'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0105' -> 'BP/26-27/0107'
   VOUCHERRETAINKEY: '129' -> '133'
   lists compared: 1
   legs: 6 -> 6
```

### s7r: R-0107 against RC-0107 (--diff)
```
== --diff a-s7r-R-0107.xml a-s7r-RC-0107.xml
   CMPGSTIN: <absent> -> '08ZZZZZ0000Z1ZQ'
   GSTREGISTRATION: <absent> -> 'Rajasthan Registration'
   lists compared: 1
   legs: 6 -> 6
```

### s7r: V-0105 against s5r (--diff)
```
== --diff a-s5r-V-0105.xml a-s7r-V-0105.xml
   lists compared: 2
   legs: 0 -> 0
```

### s7r: V-0010 against s2 (--diff)
```
== --diff a-s2-V-0010.xml a-s7r-V-0010.xml
   lists compared: 2
   legs: 0 -> 0
```

### s8: M against s1 (--same-row); U, T, R-0010 and RC-0010 against s2
```
== a-s8-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
== --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
== --type-diff a-s2-T-types.xml a-s8-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
== --diff a-s2-R-0010.xml a-s8-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
== --diff a-s2-RC-0010.xml a-s8-RC-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s8: each answer against s2 (sha256)
```
$ each answer of s8 against the same request's answer at s2, by sha256
M-marks.xml: s2 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6, s8 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6: equal
U-taxunits.xml: s2 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989, s8 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989: equal
T-types.xml: s2 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed, s8 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed: equal
N-0105.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s8 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
N-0107.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s8 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
R-0010.xml: s2 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5, s8 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5: equal
RC-0010.xml: s2 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399, s8 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399: equal
V-0010.xml: s2 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456, s8 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456: equal
```

### V answers read again: the DESC/CMPINFO count lines
```
$ cmpinfo.py a-s2-V-0010.xml a-s5r-V-0010.xml
a-s2-V-0010.xml: 52784 B sha256 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s5r-V-0010.xml: 52784 B sha256 456ddfbe553487a466c405e963e4f094e50507a8fa4f2665c0be7a7a30893a0e
NOT BYTE-EQUAL; outside CMPINFO: EQUAL
  CMPINFO LEDGER: 0 -> 7
  CMPINFO VOUCHERTYPE: 0 -> 1
  CMPINFO CURRENCY: 0 -> 1
  CMPINFO TAXUNIT: 0 -> 2
  CMPINFO VOUCHERNUMBERSERIES: 0 -> 1
  CMPINFO VOUCHER: 0 -> 2
exit 1
$ cmpinfo.py a-s2-V-0010.xml a-s7r-V-0010.xml
a-s2-V-0010.xml: 52784 B sha256 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s7r-V-0010.xml: 52786 B sha256 930f23bb383bf7a8fcb498f24bde077bd2bb7eb3954ff7313376d6139e2a5b18
NOT BYTE-EQUAL; outside CMPINFO: EQUAL
  CMPINFO LEDGER: 0 -> 14
  CMPINFO VOUCHERTYPE: 0 -> 1
  CMPINFO CURRENCY: 0 -> 2
  CMPINFO TAXUNIT: 0 -> 3
  CMPINFO VOUCHERNUMBERSERIES: 0 -> 2
  CMPINFO VOUCHER: 0 -> 4
exit 1
$ cmpinfo.py a-s5r-V-0105.xml a-s7r-V-0105.xml
a-s5r-V-0105.xml: 52630 B sha256 81ede927c72c679406a5197ce6a8b41a199b8ecf479fd49affb2cbe1b5d8e94a
a-s7r-V-0105.xml: 52632 B sha256 d9c7055cbc873788d9d78ac35ada8f6ba58316e46e9a7fbe2146f63937a2e7b5
NOT BYTE-EQUAL; outside CMPINFO: EQUAL
  CMPINFO LEDGER: 7 -> 14
  CMPINFO CURRENCY: 1 -> 2
  CMPINFO TAXUNIT: 2 -> 3
  CMPINFO VOUCHERNUMBERSERIES: 1 -> 2
  CMPINFO VOUCHER: 2 -> 4
exit 1
$ cmpinfo.py a-s2-V-0010.xml a-s8-V-0010.xml
a-s2-V-0010.xml: 52784 B sha256 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s8-V-0010.xml: 52784 B sha256 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
BYTE-EQUAL
exit 0
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s2-N-0105.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0107.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s2-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
a-s2-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3-U-taxunits.xml 33304 B 2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1
a-s3-T-types.xml 306946 B 3239e41dfed99766ac1ee6da5724eea72ca206f55113d729bb0c998d56402425
a-s3-M-marks.xml 3394 B f1845d9f7649984fc39f0929f60c86eb6b255f0def12cb8c1e867ef7350b14ea
a-s4m-M-marks.xml 3394 B f1845d9f7649984fc39f0929f60c86eb6b255f0def12cb8c1e867ef7350b14ea
a-s4-T-types.xml 307038 B eef7b1cc48eecd1fbc654b94379015dd87820579331d0b8cbf3222ffe2cf5987
a-s4-U-taxunits.xml 33304 B 508dd8de3eb34f011a48debf7e86154657b129efc570d4ccd6a6ec5d96143621
a-s4-M-marks.xml 3394 B 6ae4f3a78b93fb3e59bdbb822868d4826850cf823173d7d998c1aaf213f048e6
a-s5m-M-marks.xml 3394 B 6ae4f3a78b93fb3e59bdbb822868d4826850cf823173d7d998c1aaf213f048e6
a-s5-I-0105.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s5r-N-0105.xml 5606 B 23929638cd0c6b653435a33a9903f8e5e15b2efb481d0a5d7f71fb5a4792a2ff
a-s5r-R-0105.xml 87968 B 56e3fff1e5c6b0d7594e7c863bb6e795f66eff7ae0e7e49dfe3df7e26146d075
a-s5r-RC-0105.xml 88208 B 5e4b0537c33d4d87ed53e37cee5162db1db7ccbe6eea41cf30d0f2c89a4f3265
a-s5r-V-0105.xml 52630 B 81ede927c72c679406a5197ce6a8b41a199b8ecf479fd49affb2cbe1b5d8e94a
a-s5r-V-0010.xml 52784 B 456ddfbe553487a466c405e963e4f094e50507a8fa4f2665c0be7a7a30893a0e
a-s5r-M-marks.xml 3394 B eaa698ccbf94cb0145d6f3c656c6672f9bfa0631020026d175bc33a1df9d3aad
a-s6m-T-types.xml 307038 B bb0f93b2fab11fcf0a089182f0cdefbcdd20de662cb21b7753c8576950d12a1e
a-s6m-U-taxunits.xml 33304 B d12c3e78e5f1f6a7ecdeab7db60d2713f3708ba6d4f1b481efcb73b6a6e2d6b8
a-s6m-M-marks.xml 3394 B eaa698ccbf94cb0145d6f3c656c6672f9bfa0631020026d175bc33a1df9d3aad
a-s7-I-0107.xml 540 B 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
a-s7r-N-0107.xml 5608 B dcecf6cf4d98198ae26c40116b7651f9bdb024a08f8cd55a397ae7dbadb63d9d
a-s7r-R-0107.xml 87970 B 8f2b1f5ce4f2e0cee796f296b45660b563dcea80b92cce73757385e9be672661
a-s7r-RC-0107.xml 88240 B 06ee5ac651f6aa7578b6f0716f25f9340f56af0b5555570381be4b32678116f0
a-s7r-V-0107.xml 52682 B 6aadd8670ad088e1c427e70caa4025bad8ae5724236ef81d3ebcbd819644b55c
a-s7r-V-0105.xml 52632 B d9c7055cbc873788d9d78ac35ada8f6ba58316e46e9a7fbe2146f63937a2e7b5
a-s7r-V-0010.xml 52786 B 930f23bb383bf7a8fcb498f24bde077bd2bb7eb3954ff7313376d6139e2a5b18
a-s7r-M-marks.xml 3396 B 5812ed97832536da100aef4ecb5fa1a925370407dbb531228084450ca045878e
a-s8-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s8-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s8-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s8-N-0105.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-N-0107.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s8-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
a-s8-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
```
