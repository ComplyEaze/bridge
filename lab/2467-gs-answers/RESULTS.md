# Row 2467-GS: a same-state GSTIN change saved on Tally's screen shows nothing but the new number; `GSTOLDREGNUMBER` stays empty and no dated row is added; restored

Run on 9 Nov 2026, 23:10:34Z to 23:25:16Z (04:40 to 04:55 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/2467-gs-sheet` (8524cc0c), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no request writes and no Bridge build. One master was altered on Tally's screen: the company's GST registration, its GSTIN from `08ZZZZZ0000Z1ZQ` to `08ZZZZZ0000Z3ZO`. The book was then restored from the row's own backup. Every value below is M, measured by this run, in one run: **partial** (one change, one sitting, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran after row 2377-NB (Tally quit at 22:58:28Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The book was restored after step 4, so the marks it leaves are those it found (68/245).

The GSTINs in this folder are the synthetic family's, as the sheets name them: the company's own `08ZZZZZ0000Z1ZQ`, this row's changed value `08ZZZZZ0000Z3ZO` (the same state code and PAN part; only the entity code and check character differ), and row 2162-SC's changed value `27ZZZZZ0000Z1ZQ`, named where its answer is compared.

**Result.**
- **s2, before:** U byte-equal to row 2162-RU's s2 answer (22,636 B, sha256 `e40ef83c...`). Two TAXUNIT rows, each the book's: `Default Tax Unit`, and `Rajasthan Registration` with TAXTYPE `GST`, `@TAXREGISTRATION` and `GSTREGNUMBER` `08ZZZZZ0000Z1ZQ`, `GSTOLDREGNUMBER` empty, one dated row (`FROMDATE` 20260401, Rajasthan, Rajasthan, Regular, `ISINACTIVE` No). M 68/245. Every EXPECT met.
- **Step 3, the change, on row 2162-SC's step 4 path, screen for screen:** Go To, Alter Master, Down 11 to GST Registration, the one line `Rajasthan Registration`, then "GST Registration Alteration" with its fields the same as SC's 4.3. Enter twice to `GSTIN/UIN` (no Backspace), `08ZZZZZ0000Z3ZO` typed over the highlighted value and zoomed (15 characters, capitals), no other field touched.
- **Ctrl+A: no prompt and no message.** No "Accept ?", no date asked, no GST status message. Tally went straight back to the List of GST Registrations, as SC's did. Ctrl+Q: the Gateway, the same date.
- **s4, the book after the change** (each read answered, STATUS 1; none failed):
  - **U, `--unit-diff` against s2:** `Default Tax Unit` 0 leaves; `Rajasthan Registration` 3 leaves: `/ALTERID` 206 -> 246, `/GSTREGNUMBER` and `@TAXREGISTRATION` `08ZZZZZ0000Z1ZQ` -> `08ZZZZZ0000Z3ZO`.
  - **`GSTOLDREGNUMBER`: empty**, as at s2.
  - **The dated rows: one**, unchanged (20260401, Rajasthan, Rajasthan, Regular, No). No second row and no new `FROMDATE`.
  - **No new unit**, and none inactive: the same two TAXUNIT rows, the same GUIDs.
  - **Line by line against s2** (`evidence/linediff-s2-s4-U.txt`): 309 lines each, the same 22,636 B; four lines differ: the three above and the header counter `<TAXUNIT>` in `CMPINFO`, 0 -> 1.
  - **Against row 2162-SC's changed answer** (`lab/2162-sc-answers` at 87963e77, `a-s5-U-taxunits.xml`, taken after SC's change to `27ZZZZZ0000Z1ZQ`): line for line the same, except the two lines that carry the GSTIN value (`evidence/linediff-sc-s5-s4-U.txt`). SC's answer holds the same `ALTERID` 246 and the same `<TAXUNIT>` counter, 1.
  - **M:** 68/246 against s3m's 68/245: ALTVCHID unchanged, ALTMSTID +1.
- **The restore** (C6), then s5:
  - M s5 byte-equal to s1 (sha256 `8a0c4268...`); `--same-row` against s1 equal (68/245).
  - U s5 byte-equal to s2 (sha256 `e40ef83c...`); `--unit-diff` against s2: 0 leaves in each unit; the `<TAXUNIT>` counter 0 again; the GSTIN `08ZZZZZ0000Z1ZQ` again.

**By the sheet's Decides: nothing visible but the number.** `GSTREGNUMBER` and `@TAXREGISTRATION` changed and `ALTERID` moved; nothing else in either unit moved. By the sheet's reading of the build's consistency term, the changed unit passes it: `GSTOLDREGNUMBER` is empty, `TAXREGISTRATION` equals `GSTREGNUMBER`, the GSTIN is well formed, and its state code (08) is the dated row's state. Nothing in the answer the build reads tells the old number from the new. So #2467's two gaps are measured on this book and release, not only reasoned: the re-read before a post cannot see this change, and an invoice dated before it would be issued under the new number.
- **`GSTOLDREGNUMBER` filled:** not seen.
- **A dated row added:** not seen.
- **A new unit, or the first inactive:** not seen.
- **A prompt or a date at step 3:** not seen.
- **One more leaf, outside the units:** the `CMPINFO` header counter `<TAXUNIT>` read 1 at s4 and 0 at s2 and s5, as in row 2162-SC's answers (0, 1, 0). It counts, not names: it does not say which unit changed or what it held.

**The sheet's estimates:** s2 equal to RU's; step 3 as SC's step 4 (no date, no prompt, the list again after Ctrl+A); s4 as SC's s5 with the new number, `ALTERID` moved, `GSTOLDREGNUMBER` empty and the dated row unchanged; s5 equal to s2 and s1. Each was so.

## Step 1: load, check and back up
- Sheet files: all 7 equal to `SHA256.txt` at 8524cc0c. `LAB-COMMON.md`, `run.py`, `count15.py`, `U-taxunits.xml`, `M-marks.xml` and `.gitattributes` are byte-equal to row 2162-GY's.
- Tally was started once, at 23:10:34Z, from its desktop shortcut, after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 at 23:11:36Z: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID, and not the excluded GUID.
- The book was loaded alone from Select Company: the name typed, one row, Enter, no prompt. The Gateway showed (`s1-gateway`): BRIDGE PILOT LAB alone, SILVER on the title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday 15-Sep-2026; date of last entry 15-Sep-26.
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5):
  - Before the backup, `C:\lab\2467-gs\backup-1` was new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times), then `C:\lab\2467-gs\backup-1` typed, and its row taken.
  - BRIDGE PILOT LAB alone, End of List. "Accept ?" was captured (`bk-accept`, 23:14:21Z) and answered y; no other message.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `9593e56454a01c33f0783ee559eab51fc4b54923bb3409b3989bc9485f5b456e`, written at 04:44 IST. The Restore list showed it as 10-Nov-26 (4:44 am).

## Step 2: the book before
`s2`: U and M, as in the Result. Every EXPECT met. The outputs are below, whole.

## Step 3: the change, on Tally's screen
`s3m`, then the before-write check against s1: one COMPANY row, the same GUID (68/245); passed. Every screen and key is in `evidence/notes.txt`. In short:
1. At the Gateway (`s3-1-gateway`, 23:15:27Z), the G: Go To label clicked (the sheet says Alt+G; row 2162-SC clicked it too). `Alter Master` typed (the field showed `AlterMaster`), Enter on that line. "Master Alteration", the List of Masters, the same lines as SC's (`s3-1b-master-alteration`).
2. Down 11 times from Group (Godown at 9, the Statutory Masters heading at 10, GST Registration at 11; zoomed before Enter), Enter. "List of GST Registrations": Create, Back, and one line, `Rajasthan Registration` (`s3-2-gst-reg-list`). Enter on it.
3. "GST Registration Alteration", panel "GST Details" (`s3-3-gst-reg-alteration`, 23:18:11Z). Every field, as shown, the same as SC's 4.3:
   - Registration status: Active
   - GST Registration Details: State Rajasthan; Registration type Regular (the cursor here, with a "Registration Types" list: Regular, Composition, Regular - SEZ); Assessee of Other Territory No; GSTIN/UIN `08ZZZZZ0000Z1ZQ`; Periodicity of GSTR-1 Monthly
   - Connected GST Details: GST Username (empty); Mode of Filing Not Applicable (with a diamond mark)
   - e-Invoice Details: e-Invoicing applicable No
   - e-Way Bill Details: e-Way Bill applicable Yes; Applicable from 1-Apr-26; Applicable for intrastate Yes
   - Button bar: F2: Period (greyed), F3: Company, F10: Other Masters, I: More Details, A: Accept, D: Delete, Q: Quit, F12: Configure.
   One field held `08ZZZZZ0000Z1ZQ`: `GSTIN/UIN`.
4. Enter (Registration type kept Regular), Enter (Assessee of Other Territory kept No): the cursor on `GSTIN/UIN`, its value highlighted. No Backspace. `08ZZZZZ0000Z3ZO` typed as one string over it; zoomed before any Enter: 15 characters, capitals (`s3-4-gstin-typed`, 23:19:14Z).
5. No other field touched. State Rajasthan, Registration type Regular, Assessee of Other Territory No, Periodicity of GSTR-1 Monthly. No date was asked and no other field or screen opened.
6. The field zoomed once more, then Ctrl+A. No "Accept ?", no other prompt, no message. Tally returned to the List of GST Registrations, `Rajasthan Registration` highlighted (`s3-6-after-accept`, 23:19:37Z). The wait rule was not needed.
7. Ctrl+Q on that list: the Gateway, BRIDGE PILOT LAB, current date Tuesday, 15-Sep-2026, last entry 15-Sep-26 (`s3-7-gateway`, 23:19:58Z). Screens passed: the Gateway, Go To, Master Alteration, List of GST Registrations, GST Registration Alteration, List of GST Registrations, the Gateway.

Keys pressed in step 3, in all: the Go To label (clicked); `Alter Master`; Enter; Down 11 times; Enter; Enter; Enter; Enter; `08ZZZZZ0000Z3ZO`; Ctrl+A; Ctrl+Q. Each key went after a click on Tally's title bar.

## Step 4: the book after the change
- C2 at 23:20:05Z, with the Gateway showing: one `tally.exe`, which alone owns 9001; `GET /status` 200.
- `s4` sent at 23:20:22Z: M and U, each answered with STATUS 1. The diffs are below, whole.

## Step 5: restore (C6), then the book after
- **Before the shut** (23:20:40Z): BK re-hashed, equal to step 1's, its folder holding only that file; `restore-plan.txt` written; the data folder listed (24 entries).
- **The restore:**
  - Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt.
  - The Data menu (by its label), Backup & Restore, Restore (Down 2 times). Restore Data Path read Tally's data folder.
  - Specify Path (Up 3 times) to the BK's folder. The list showed BRIDGE PILOT LAB, its number and 10-Nov-26 (4:44 am), the BK's time.
  - The company line, then End of List.
- **Two prompts, the only ones answered, each captured first:**
  - "Accept ?": y (`c6-accept`, 23:22:33Z).
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", with R Replace and S Skip: r (`c6-replace`, 23:22:51Z).
- **After the restore:**
  - Select Company came back with no other message.
  - The data folder held 17 entries. The seven names not there: `CmpSchMetaData.1800` and six `.TSF` files (`TCMPSCHACCESS`, `TCMPSCHMESSAGE`, `TCMPSCHSTATE`, `TCMPSCHUPDATE`, `TINTMSG`, `TUPDATE`); no new name. Its `Company.1800` held the book's GUID and name, and not the excluded GUID.
  - The book was loaded alone (the name typed, one row, Enter, no prompt). The Gateway showed the book as the one company, with the same period and date (`s5-gateway`).
  - C2 at 23:24:02Z: one `tally.exe`, which alone owns 9001; `GET /status` 200.
- `s5` sent at 23:24:06Z: M and U, as in the Result.

## Step 6: close (C10)
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 23:24:49Z).
- C2 at 23:24:50Z: one `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; no `tally.exe` ran after.

## Files
- `answers/`: every answer, byte for byte. The three U answers were first kept on the lab PC by the GSTIN rule; they are pushed on the run lead's word that the synthetic `08ZZZZZ0000Z1ZQ` and `08ZZZZZ0000Z3ZO` may go out (`kept-local.txt`). Their sha256 are also in `evidence/answers-sha.txt`.
- `evidence/`: notes, the count15 outputs, the diffs, the backup's record. Folder, process and port listings stay on the lab PC.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

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
== a-s2-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

### s3m
```
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s4
```
== a-s4-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z3ZO' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z3ZO'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

### s5
```
== a-s5-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
```

## Checks and diffs, whole

### s3m against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### U, s2 against s4
```
== --unit-diff a-s2-U-taxunits.xml a-s4-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 3 leaves differ
     /ALTERID[1]: '206' -> '246'
     /GSTREGNUMBER[1]: '08ZZZZZ0000Z1ZQ' -> '08ZZZZZ0000Z3ZO'
     @TAXREGISTRATION: '08ZZZZZ0000Z1ZQ' -> '08ZZZZZ0000Z3ZO'
```

### U, s2 against s4, line by line
```
lines 309 309 | differing hunks 4
--- a-s2-U-taxunits.xml
+++ a-s4-U-taxunits.xml
@@ -39 +39 @@
-    <TAXUNIT>0</TAXUNIT>
+    <TAXUNIT>1</TAXUNIT>
@@ -163 +163 @@
-    <TAXUNIT NAME="Rajasthan Registration" RESERVEDNAME="" TAXTYPE="GST" TAXREGISTRATION="08ZZZZZ0000Z1ZQ">
+    <TAXUNIT NAME="Rajasthan Registration" RESERVEDNAME="" TAXTYPE="GST" TAXREGISTRATION="08ZZZZZ0000Z3ZO">
@@ -205 +205 @@
-     <GSTREGNUMBER>08ZZZZZ0000Z1ZQ</GSTREGNUMBER>
+     <GSTREGNUMBER>08ZZZZZ0000Z3ZO</GSTREGNUMBER>
@@ -241 +241 @@
-     <ALTERID> 206</ALTERID>
+     <ALTERID> 246</ALTERID>
```

### U, row 2162-SC's changed answer (s5) against this row's s4, line by line
```
lines 309 309 | differing hunks 2
--- row 2162-SC a-s5-U-taxunits.xml
+++ a-s4-U-taxunits.xml
@@ -163 +163 @@
-    <TAXUNIT NAME="Rajasthan Registration" RESERVEDNAME="" TAXTYPE="GST" TAXREGISTRATION="27ZZZZZ0000Z1ZQ">
+    <TAXUNIT NAME="Rajasthan Registration" RESERVEDNAME="" TAXTYPE="GST" TAXREGISTRATION="08ZZZZZ0000Z3ZO">
@@ -205 +205 @@
-     <GSTREGNUMBER>27ZZZZZ0000Z1ZQ</GSTREGNUMBER>
+     <GSTREGNUMBER>08ZZZZZ0000Z3ZO</GSTREGNUMBER>
```

### M, s5 against s1 (--same-row)
```
== a-s5-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### U, s2 against s5
```
== --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### sha256 of every answer
```
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6  a-s1-M-marks.xml
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6  a-s2-M-marks.xml
e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989  a-s2-U-taxunits.xml
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6  a-s3m-M-marks.xml
f1845d9f7649984fc39f0929f60c86eb6b255f0def12cb8c1e867ef7350b14ea  a-s4-M-marks.xml
85551b5d3c318a569fa0863c1c0fa127b5793cbe4cb29b953eff7b50833125da  a-s4-U-taxunits.xml
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6  a-s5-M-marks.xml
e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989  a-s5-U-taxunits.xml
```
