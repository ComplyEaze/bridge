# Row 2162-KC: the build's control reads as it looks for it, a delete on Tally's screen removes its row from the month read, and BRIDGE Sales' guard does not count a number another Sales type holds

Run on 9 Nov 2026, 15:13Z to 15:30Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-kc-sheet` (604bd4e0), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic). Raw requests through `run.py`, one delete and one voucher type made by hand on Tally's screen; no Bridge build. Every value below is M, measured by this run, in one run: **partial**. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1. Every write answered CREATED 1, ERRORS 0, EXCEPTIONS 0, with no LINEERROR, and no dialog appeared after any write.

**Result.**
- **Part A, the control (steps 3 and 4):** `I-0062` and then `I-0063` were each created (LASTVCHID 45, then 46). The month read `W-2026-08` shows `BP/26-27/0062` as the build looks for it: 20260801, `BRIDGE Sales`, BRIDGEVCHISSALES Yes, ISCANCELLED No, ISDELETED No, with R's GUID `...-0000002d`. After `I-0063` it shows 0062 unchanged beside 0063 (`...-0000002e`). `--guid ...-0000002d` printed 1 row in s3r and in s4r. Every EXPECT met.
- **Part B, the delete (steps 5 and 6):** on the Day Book for 1-Aug-26, the 0062 line, Enter: "Accounting Voucher Alteration (Secondary)"; Alt+D: "Delete ?" with "Yes or No", answered y; no other prompt. The Day Book went from 13 lines to 12. `s6`: R-0062 and V-0062 0 rows; W 41 rows, s4r's 42 without 0062 and every other row unchanged; **`--guid ...-0000002d`: 0 rows**. V-0063 byte-equal to s4r's. ALTVCHID 70 -> 71 (by the delete), ALTMSTID 245 unchanged. `s6b`, 61 s later with no key pressed: M and W byte-equal to s6.
- **Part C, a second Sales type (steps 7 and 8):** `BRIDGE Sales Keyed` keyed on Voucher Type Creation (Sales, Manual, Prevent creating duplicate Voucher Nos. Yes); Ctrl+A, no prompt. `s7r`: T 29 rows; the new type PARENT Sales, GUID `...-000000e8`, one series Default, Manual, PREVENTDUPLICATES Yes; BRIDGE Sales as at s2; ALTMSTID 245 -> 247. `I-0064K` (BP/26-27/0064 on the new type): CREATED 1; W shows it on `BRIDGE Sales Keyed`, BRIDGEVCHISSALES Yes. Every EXPECT met.
- **Step 8.2, the guard:** `I-0064` (the same number on `BRIDGE Sales`) answered **CREATED 1**, LASTVCHID 48, ALTERED 0, ERRORS 0, EXCEPTIONS 0, no LINEERROR, no dialog. `s8br`: V-0064 two rows, `...-0000002f` on BRIDGE Sales Keyed (MASTERID 47) and `...-00000030` on BRIDGE Sales (MASTERID 48); W 43 rows.
- **The restore (step 9):** `s9` M 68 / 245, `--same-row` against s1 equal, byte-equal to s1; T byte-equal to s2's (28 rows, `BRIDGE Sales Keyed` 0); W byte-equal to s2's (40 rows). PASS.

**By the sheet's Decides** (the code reading is the write-up's, not this run's):
- s3r's W shows 0062 with its R GUID, and s4r's shows it unchanged beside 0063: measured.
- s6 shows no row carrying 0062's GUID: measured. No row with ISDELETED Yes appeared and 0062's GUID did not appear under another number, date or type.
- s6 against s6b: no difference.
- s8r's W shows 0064 on `BRIDGE Sales Keyed` as Sales: measured.
- s8b created: measured, one number, two types, one book.

## Step 0 and step 1: load and check
- Sheet files: all 22 equal to `SHA256.txt` at 604bd4e0; `LAB-COMMON.md` and `run.py` byte-equal to row 2162-RS's. `gen9.py`, re-run in a separate folder, rebuilt all ten generated files byte for byte (`evidence/gen-check.txt`).
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID. Two Bridge server processes and one Bridge communication process were running; they were recorded and not ended (a standing rule of the lab; no Bridge tool was called).
- Loaded alone from Select Company (the name typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26 (`s1-gateway`).
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5): the Data menu (by its label), Backup & Restore, Backup, Specify Path to a new empty folder. BRIDGE PILOT LAB alone, End of List; "Accept ?" captured (`s1-bk-accept`) and answered y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `3f67cf1d3aac5c96f4cdab780ca5e25f9cb1edd03805c8e26310643d8c7207a6`, 20:45 local. `s1bk` marks: byte-equal to `s1`.

## Step 2: the book as found
`s2`: T 28 rows; `BRIDGE Sales` one row (one series Default, Manual, PREVENTDUPLICATES Yes); `BRIDGE Sales Keyed` 0. V-0062, V-0063 and V-0064 0 rows each. W 40 rows, none numbered BP/26-27/0062, 0063 or 0064. Every EXPECT met.

## Step 3: the control, 0062
`s3m` 68 / 245, the before-write check passed. `s3`: `I-0062` sent once: CREATED 1, LASTVCHID 45, every other counter 0, no LINEERROR; the capture after it (`s3-after-write`) shows the Gateway with no dialog. `s3r`: M 69 / 245; R one row, stored as sent (GUID `...-0000002d`, ALTERID 69, included Yes, uncertain No, excluded No, 6 legs); V one row (MASTERID 45, ALTERID 69); W 41 rows, s2's 40 unchanged and BP/26-27/0062 (`evidence/w-s2-s3r.txt`); `--guid ...-0000002d` 1 row. Every EXPECT met. GUID `...-0000002d` and MASTERID 45 are the ones row 2162-RS's 0057 took before its restore.

## Step 4: a second invoice, 0063
`s4m` 69 / 245, check passed. `s4`: `I-0063` sent once: CREATED 1, LASTVCHID 46, every other counter 0, no LINEERROR; `s4-after-write` shows the Gateway, no dialog. `s4r`: M 70 / 245; R one row (GUID `...-0000002e`, ALTERID 70, 6 legs); V one row (MASTERID 46, ALTERID 70); W 42 rows, s3r's 41 unchanged and BP/26-27/0063 (`evidence/w-s3r-s4r.txt`); `--guid ...-0000002d` 1 row, the same fields as in s3r. Every EXPECT met.

## Step 5: the delete, on Tally's screen
`s5m` 70 / 245, the before-write check passed, sent before the delete was keyed. No request was sent during this step.
1. The Gateway (`s5-1-gateway`). Go To opened by clicking the "G: Go To" label (the sheet says Alt+G; see Deviations). `Day Book` typed (the field shows `DayBook`); Enter took Day Book under Common Reports. The Day Book opened For 15-Sep-26, one line (`s5-1-daybook`).
2. Alt+F2 opened "Change Period" (From 15-9-2026, To 15-9-2026). From `1-8-2026`, Enter, To `1-8-2026`, Enter. The Day Book For 1-Aug-26 (`s5-2-daybook-1aug`), 13 lines, each 1-Aug-26, BRIDGE Walk-in, BRIDGE Sales, by Vch No. and Debit Amount: BP/26-27/0001 1,050.00; 0002 1,051.05; 0003 1,051.04; 0004 1,051.05; 0005 1,051.05; 0006 1,050.02; 0007 1,050.00; 0008 1,051.05; 0009 1,456.00; 0011 1,051.06; 0010 1,051.00; 0062 1,051.00; 0063 1,051.00 (`evidence/s5-daybook-lines.txt`).
3. Down 11 times, one line each: `BP/26-27/0062` highlighted (`s5-3-0062-highlighted`).
4. Enter: "Accounting Voucher Alteration (Secondary)", BRIDGE PILOT LAB; voucher type BRIDGE Sales; No. BP/26-27/0062; 1-Aug-26, Saturday; Party A/c name BRIDGE Walk-in, Current balance 19,250.70 Dr; Particulars: BRIDGE Svc 998313 5% 600.40; BRIDGE Svc 998314 5% 400.40; BRIDGE CGST 2.5% rate 2.50 % 25.02; BRIDGE SGST 2.5% rate 2.50 % 25.02; BRIDGE Round Off 0.16; Narration empty; total 1,051.00 (`s5-4-alteration-0062`; `evidence/s5-alteration-fields.txt`). Nothing changed.
5. Alt+D once: a box "Delete ?" with "Yes or No" (`s5-5-delete-prompt`); answered y. No other prompt or message.
6. The Day Book For 1-Aug-26, 12 lines: the same, without BP/26-27/0062; BP/26-27/0010 highlighted (`s5-6-daybook-after-delete`).
7. Esc once on the Day Book: the Gateway (`s5-7-gateway`).

## Step 6: the reads after the delete
`s6` at 15:20:15Z; `s6b` at 15:21:16Z, 61 s later, with no key pressed between.
- `s6`: M 71 / 245 (ALTVCHID 70 -> 71). R-0062 0 rows; V-0062 0 rows; V-0063 one row, byte-equal to s4r's. W 41 rows: s4r's 42 without BP/26-27/0062, every other row unchanged (`evidence/w-s4r-s6.txt`). `--guid ...-0000002d`: 0 rows.
- `s6b`: M and W byte-equal to s6's; `--guid ...-0000002d`: 0 rows.

## Step 7: a second Sales type, on Tally's screen
`s7m` 71 / 245, the before-write check passed, sent before the type was keyed. No request was sent while it was keyed.
- From the Gateway, Create (clicked in the Gateway menu): Master Creation, List of Masters; Down 3 to Voucher Type, Enter: Voucher Type Creation.
- Name `BRIDGE Sales Keyed`, Enter; (alias) empty, Enter; Select type of voucher: the List of Voucher Types, `Sales` typed (Sales and Sales Order listed), Enter; Abbreviation `Sale` (Tally's), Enter; Activate this Voucher Type Yes (Tally's), Enter; Method of Voucher Numbering: the Methods of Numbering list (Automatic, Automatic (Manual Override), Manual, Multi-user Auto, None), Down 2 to Manual, Enter; Prevent creating duplicate Voucher Nos. appeared under it showing No, `y` typed, Enter: Yes. Captured before saving (`s7-vt-creation-before-save`; every field as shown in `evidence/s7-voucher-type-fields.txt`).
- Ctrl+A once: no prompt and no message. Tally opened a new blank Voucher Type Creation (`s7-blank-creation-after-save`). Ctrl+Q: Master Creation, List of Masters (`s7-master-creation-list`); Ctrl+Q: the Gateway (`s7-gateway`). No Escape was pressed on either screen.
- `s7r`: T 29 rows. `BRIDGE Sales Keyed` one row: PARENT Sales, GUID `...-000000e8`, NUMBERINGMETHOD None at the type, one series Default, NUMBERINGMETHOD Manual, PREVENTDUPLICATES Yes, BEGINNINGNUMBER 1, RESTARTFROMLIST with 0 children. `BRIDGE Sales` as at s2 (its count lines identical). M 71 / 247 (ALTVCHID as at s6b, ALTMSTID 245 -> 247). Every EXPECT met.

## Step 8: one number on each type
1. `s8m` 71 / 247, check passed. `s8`: `I-0064K` sent once: CREATED 1, LASTVCHID 47, every other counter 0, no LINEERROR; `s8-after-write` shows the Gateway, no dialog. `s8r`: M 72 / 247; V-0064 one row, BP/26-27/0064 on BRIDGE Sales Keyed, GUID `...-0000002f`, MASTERID 47, ALTERID 72; W 42 rows, s6b's 41 unchanged and BP/26-27/0064, 20260801, BRIDGE Sales Keyed, BRIDGEVCHISSALES Yes, ISCANCELLED No, ISDELETED No (`evidence/w-s6b-s8r.txt`). Every EXPECT met.
2. `s8bm` 72 / 247, check passed. `s8b`: `I-0064` (BP/26-27/0064 on BRIDGE Sales) sent once: **CREATED 1**, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0; no LINEERROR. `s8b-after-write` shows the Gateway, no dialog. `s8br`: M 73 / 247; V-0064 two rows, `...-0000002f` on BRIDGE Sales Keyed (MASTERID 47, ALTERID 72) and `...-00000030` on BRIDGE Sales (MASTERID 48, ALTERID 73); W 43 rows, s8r's 42 unchanged and BP/26-27/0064 on BRIDGE Sales, BRIDGEVCHISSALES Yes, ISCANCELLED No, ISDELETED No (`evidence/w-s8r-s8br.txt`).

## Step 9: the restore (C6)
- Before the shut: the BK re-hashed equal to its record and its folder holds that one file; `restore-plan.txt` written at 15:25:29Z (kept local); THE COMPANY's data folder listed, 24 entries (kept local).
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt.
- Restore under the Data menu (by its label), Backup & Restore, Restore. The Restore Data Path was Tally's data folder; the source was Specify Path, the BK folder. The list showed one row: BRIDGE PILOT LAB, THE COMPANY's number, 9-Nov-26 (8:45 pm), the BK's time. It was selected alone, then End of List. The prompts, word for word:
  - "Accept ?" captured (`s9-restore-accept`) and answered y.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." captured (`s9-restore-replace`) and answered r (Replace).
  - No other prompt. Tally went to Select Company.
- After the restore the data folder held 17 entries (listing kept local); the restored `Company.1800` holds BRIDGE PILOT LAB and its GUID. Loaded alone (the name typed, one row, Enter, no prompt). The Gateway (`s9-gateway-after-restore`, byte-equal to `s1-gateway`).
- `s9`: M 68 / 245, `--same-row` against s1 equal, byte-equal to s1; T byte-equal to s2's, 28 rows, `BRIDGE Sales Keyed` 0; W byte-equal to s2's, 40 rows. PASS.

## Step 10: close (C10)
Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. The data folder held 20 entries after the close (listing kept local): against the listing after the restore, four schedule files (`CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`) were added and `TMESSAGE.TSF` was gone, by the load and the close. Row 2162-RS's close also left 20 entries. PASS.

## Deviations and notes
- **Go To** was opened by clicking its "G: Go To" label, not by Alt+G as the sheet says. Menus were opened by clicking their labels throughout (K: Company, Y: Data, G: Go To), as the lab's standing rule asks.
- **s6b's wait:** the 60-second wait was scripted from s6's send time; the first script stopped on a time-zone parse error before it sent anything. It was re-run with the time written out, and s6b was sent 61 s after s6 with no key pressed between.
- **s5m:** step 5 names no marks read, but the sheet's before-write rule covers the delete, so `s5m` was sent and checked before it.
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.
- **Labels.** Each label was used once. `s1bk` is the marks read after the BK that C5 asks for; it is not in the sheet.

## Not measured
Every item the sheet lists as not measured; why BRIDGE Sales' guard did not count the number BRIDGE Sales Keyed holds (one number, two types, one book, one release); whether the build would have refused s8b's number (no build was run).

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`). The five V answers that hold a voucher (s3r's, s4r's, s6's, s8r's and s8br's) carry the book's GSTIN and stay on the lab PC; their counts are below.
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count, check and row-compare output, the `--guid` outputs, the send times, the Day Book lines, the alteration and voucher type screens' fields, the generator check and the BK record. The process and port listings, the data-folder listings and `restore-plan.txt` stay local.
- `scripts/`: the step 0 helper and the window capture helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## count11.py outputs, whole
Each step's answers in send order; for a before-write marks read, the check's output follows the count.

### s1
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s1bk
```
== a-s1bk-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s2
```
== a-s2-T-types.xml: 306946 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
  named 'BRIDGE Sales Keyed': 0
== a-s2-V-0062.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-V-0063.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-V-0064.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
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
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s3
```
== a-s3-I-0062.xml: 540 bytes
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
== a-s3r-R-0062.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0062'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0062'
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
== a-s3r-V-0062.xml: 52788 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0062'
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
   direct children: 729
== a-s3r-W-2026-08.xml: 117986 bytes
  STATUS '1'
  VOUCHER rows: 41
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
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
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
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s4
```
== a-s4-I-0063.xml: 540 bytes
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
== a-s4r-R-0063.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0063'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0063'
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
== a-s4r-V-0063.xml: 52788 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID '46'
   ALTERID '70'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0063'
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
   direct children: 729
== a-s4r-W-2026-08.xml: 120802 bytes
  STATUS '1'
  VOUCHER rows: 42
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
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0063'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s5m
```
== a-s5m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s6
```
== a-s6-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6-R-0062.xml: 3024 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s6-V-0062.xml: 3024 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s6-V-0063.xml: 52788 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID '46'
   ALTERID '70'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0063'
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
   direct children: 729
== a-s6-W-2026-08.xml: 117986 bytes
  STATUS '1'
  VOUCHER rows: 41
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
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0063'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s6b
```
== a-s6b-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6b-W-2026-08.xml: 117986 bytes
  STATUS '1'
  VOUCHER rows: 41
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
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0063'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s7m
```
== a-s7m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s7r
```
== a-s7r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7r-T-types.xml: 319888 bytes
  STATUS '1'
  VOUCHERTYPE rows: 29
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
  named 'BRIDGE Sales Keyed': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
```

### s8m
```
== a-s8m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s8
```
== a-s8-I-0064K.xml: 540 bytes
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

### s8r
```
== a-s8r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8r-V-0064.xml: 52832 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   MASTERID '47'
   ALTERID '72'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0064'
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
   direct children: 729
== a-s8r-W-2026-08.xml: 120822 bytes
  STATUS '1'
  VOUCHER rows: 42
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
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0063'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0064'; VOUCHERTYPENAME 'BRIDGE Sales Keyed'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s8bm
```
== a-s8bm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8bm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s8b
```
== a-s8b-I-0064.xml: 540 bytes
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

### s8br
```
== a-s8br-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8br-V-0064.xml: 102594 bytes
  STATUS '1'
  VOUCHER rows: 2
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   MASTERID '47'
   ALTERID '72'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0064'
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
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   MASTERID '48'
   ALTERID '73'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0064'
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
   direct children: 729
== a-s8br-W-2026-08.xml: 123640 bytes
  STATUS '1'
  VOUCHER rows: 43
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
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0063'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0064'; VOUCHERTYPENAME 'BRIDGE Sales Keyed'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0064'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s9
```
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s9-T-types.xml: 306946 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  named 'BRIDGE Sales': 1
   PARENT 'Sales'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ce'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
  named 'BRIDGE Sales Keyed': 0
== a-s9-W-2026-08.xml: 115168 bytes
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
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

## The --guid outputs, whole

### guid-s3r
```
== --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d a-s3r-W-2026-08.xml: 1 rows
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No'
```

### guid-s4r
```
== --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d a-s3r-W-2026-08.xml: 1 rows
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No'
== --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d a-s4r-W-2026-08.xml: 1 rows
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No'
```

### guid-s6
```
== --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d a-s3r-W-2026-08.xml: 1 rows
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No'
== --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d a-s6-W-2026-08.xml: 0 rows
== --guid 6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d a-s6b-W-2026-08.xml: 0 rows
```

## The W row compares, whole

### w-s2-s3r
```
s2 40 s3r 41 s2 rows all in s3r unchanged: True | new: ["DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; "]
```

### w-s3r-s4r
```
s3r 41 s4r 42 s3r rows all in s4r unchanged: True | new: ["DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0063'; VOUCHERTYPENAME 'BRIDGE Sales'; "]
```

### w-s4r-s6
```
s4r 42 s6 41 s6 rows all in s4r unchanged: True | gone: ["DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0062'; VOUCHERTYPENAME 'BRIDGE Sales'; "]
```

### w-s6b-s8r
```
s6b 41 s8r 42 s6b rows all in s8r unchanged: True | new: ["DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0064'; VOUCHERTYPENAME 'BRIDGE Sales Keyed'; BRIDGEVCHISSAL"]
```

### w-s8r-s8br
```
s8r 42 s8br 43 s8r rows all in s8br unchanged: True | new: ["DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0064'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANC"]
```

### s9 against s1 and s2, byte compare
```
s1 M-marks byte-equal
s2 T-types byte-equal
s2 W-2026-08 byte-equal
```
