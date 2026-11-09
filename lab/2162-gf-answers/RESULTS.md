# Row 2162-GF: turning the company's GST off on F11 brings a message about the GST status of vouchers in Returns; answered Back, the setting was not saved, and the book was restored

Run on 9 Nov 2026, 19:47:58Z to 20:06:45Z (01:17 to 01:36 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-gf-sheet` (09bfc8d4), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no request writes and no Bridge build. One company setting was changed on Tally's screen and not saved; the book was then restored from the row's own backup. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran after row 2162-RU (Tally quit at 19:38:55Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The book was restored after step 3, so the marks it leaves are those it found (68/245).

The one GSTIN in this folder is the synthetic family's, as the sheet names it: the company's own `08ZZZZZ0000Z1ZQ`.

**Result.**
- **s2, before:** O one COMPANY row, the book's GUID, `ISGSTON` Yes, `ISGSTCLASSIFON` No, `GSTAPPLICABLEDATE` empty, one `CMPGSTDETAILS.LIST` and one `HSNDETAILS.LIST`. U two units, each the book's, byte-equal to rows 2162-SC's and 2162-RU's s2 U. T 28 VOUCHERTYPE rows, BRIDGE Sales one row with `TAXUNITNAME` Any, byte-equal to row 2162-RU's s2 T. R-0010 `BRIDGE Walk-in`, included Yes, every other status flag No, six legs, byte-equal to rows 2162-SC's and 2162-RU's. M 68/245. Every EXPECT met.
- **Step 3, the setting:** F11 at the Gateway opened "Company Features  Alteration". It has one field naming GST, `Enable Goods and Services Tax (GST)`, reading Yes. `n` and Enter set it to No; its sub-line `Set/Alter Company GST Rate and Other Details` went dimmed, still reading No. No message came, no sub-screen opened, and no other field changed.
- **On Ctrl+A, no "Accept ?" came. Instead a message box came, word for word:**
  - "GST-related changes in masters may impact the GST Status of vouchers in Returns."
  - "Y : Accept Master & Update GST Status of Vouchers"
  - "N : Back"
- **By the sheet's prompt rule it was answered No: `n` (Back).** The box closed and the features screen stayed open, still showing No, not saved. Nothing more was pressed on it. Ctrl+Q went to the Gateway with no question.
- **s3q, the state reads once (C8), before the restore:**
  - O `ISGSTON` still Yes, `ISGSTCLASSIFON` No, `GSTAPPLICABLEDATE` empty.
  - M 68/245, unchanged. U, T and R-0010 are byte-equal to s2's.
  - **The O answer grew from 85,654 B to 88,612 B: 56 leaves differ, all of one kind.** 28 elements absent at s2 are present on the COMPANY row at s3q, each with its TYPE attribute. None was removed and none changed. The 28 are:
    - `ACTIVEFROM` and `ACTIVETO`, both empty;
    - `MAILINGNAME` INR, `DECIMALSYMBOL` paise, `ISOCURRENCYCODE` empty, `ISSUFFIX` No, `HASSPACE` Yes, `INMILLIONS` No, `DECIMALPLACES` 2, `DECIMALPLACESFORPRINTING` 2;
    - `ALTERID` 238, `REMOTEALTERID` 0, `ERRKEY` 0, `SORTPOSITION` -1, `UPDATEDDATETIME` 000000000;
    - `NUMCOSTCENTERS` 0, `ISMULTIGODOWNON` No;
    - `NARRATION`, `REMOTEGUID`, `REMOTEALTGUID`, `ENTEREDBY`, `ALTEREDBY`, `TYPEOFUPDATEACTIVITY`, `OBJECTUPDATEACTION`, `ISMSTFROMSYNC`, `ISUPDATINGTARGETID`, `ISDELETED` and `ISSECURITYONWHENENTERED`, all empty.
- **The restore** (C6, by the prompt rule), then s5:
  - M, O, U, T and R-0010 are each byte-equal to s2's. O is back to 85,654 B, and the 28 elements are gone.
  - `--same-row` against s1 is equal (68/245). `--object-diff`, `--unit-diff` and `--type-diff` against s2 show 0 leaves. R-0010 shows `lists compared: 1` and `legs: 6 -> 6` only.

**By the sheet's Decides:**
- **O's `ISGSTON` reads No at s4:** not measured. The setting was not saved, because the prompt rule answers the message Back. There is no s4, so neither a No in the answer nor U, T or R with GST off was measured.
- **An unsaved No:** measured once. With the field reading No on screen, then Back and Ctrl+Q, the object answer still read `ISGSTON` Yes. Only a saved change can move it.
- **The message itself:** measured. Tally does not ask "Accept ?" for this change. It offers to accept the master and update the GST status of vouchers (Y), or to go back (N). What Y does to the book, to `ISGSTON` and to the vouchers' GST status was not measured.
- **The 28 elements at s3q:** they appeared in the object answer between s3m and s3q. Only the features screen was opened, changed without a save, answered Back and left in that time. The restore and a fresh load cleared them. Which act brings them, and whether Y would keep them, was not measured. The marks did not move.

## Step 1: load, check and back up
- Sheet files: all 10 equal to `SHA256.txt` at 09bfc8d4. `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml`, `U-taxunits.xml`, `T-types.xml` and `.gitattributes` are byte-equal to row 2162-RU's.
- C2 step 0 with no `tally.exe` running. Tally was started once, at 19:47:58Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2). Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt).
- C2 at 19:52:02Z: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID, and not the excluded GUID. No Bridge process was ended and no Bridge tool was called (a standing rule of the lab).
- The book was loaded alone from Select Company: the name typed, one row, Enter, no prompt. The Gateway showed (`s1-gateway`):
  - BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title;
  - current period 1-Apr-26 to 31-Mar-27;
  - current date Tuesday 15-Sep-2026;
  - date of last entry 15-Sep-26.
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5):
  - Before the backup, `C:\lab\2162-gf\backup-1` was new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`, not cloud storage. Backspace moved it back to Backup Data Path, whose list "List of Folders/Backup Location" offered Specify Path, Select from Drive, TallyDrive (Cloud Storage) and the folders of `C:\`.
  - Specify Path (Up 3 times), then `C:\lab\2162-gf\backup-1` typed. Tally shows it as `C:\lab\2162-Gf\backup-1`; Windows treats the two spellings as the same folder. Its row was taken.
  - BRIDGE PILOT LAB alone, End of List. "Accept ?" was captured (`s1-bk-accept`, 19:57:16Z) and answered y; no other message.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `fb4acbe484ac54bd0f50c2a5d2320fc5444e946615749423b0e1ea9142b4170e`, written at 01:27 IST. The Restore list showed it as 10-Nov-26 (1:27 am).

## Step 2: the book before
`s2`: O, U, T, R-0010 and M, as in the Result. Every EXPECT met. The outputs are below, whole.

## Step 3: the setting, on Tally's screen
`s3m`, then the before-write check against s1: one COMPANY row, the same GUID (68/245); passed. Every field, key and message is in `evidence/s3-screens.txt`. In short:
1. At the Gateway (`s3-gateway-before`), F11 once. The screen's title: "Company Features  Alteration", with BRIDGE PILOT LAB in the title bar. Row 1342-W14 had opened this screen the same way on this book. Its fields before the change, as text, are in `evidence/s3-screens.txt` (`s3-features-before`). They include Show more features Yes, Show all features Yes, Maintain Accounts Yes, Maintain Inventory Yes, Enable Goods and Services Tax (GST) Yes, and Set/Alter Company GST Rate and Other Details No. Every other Taxation line read No.
2. One field names GST and reads Yes: `Enable Goods and Services Tax (GST)`.
3. **NEW PATH.** Down 13 times from Maintain Accounts to that field (7, 5 and 1, captured after each group). Each field passed kept its value; the dimmed `Maintain Expiry Date for Batches` was passed over. `n` typed: the field showed `n` in its edit box, with no message (`s3-gst-n-typed`). Enter: the field read No (zoomed). Its sub-line went dimmed, reading No, and the cursor moved to Enable Tax Deducted at Source (TDS) (`s3-gst-no`). No message, no sub-screen, no list.
4. No other field was touched.
5. Ctrl+A: no "Accept ?". The message box in the Result came up at the bottom right of the window (`s3-prompt-desktop`, `s3-prompt-zoom`), and the Go To button read "G: Switch To" while it was up. By the prompt rule: `n` (N : Back). The box closed. The features screen stayed open with its title, the field still showing No unsaved and the cursor back on Maintain Accounts (`s3-after-back`, `s3-after-back-desktop`).
6. Ctrl+Q once, with no question: the Gateway, as before F11 (`s3-gateway-after`). Screens passed: the features screen, the message box, the features screen, the Gateway.

Keys pressed in step 3, in all: F11; Down 13 times; n; Enter; Ctrl+A; n; Ctrl+Q.

## Step 4: not run
The prompt rule replaces it: stop, restore, compare, close. No change was saved. The state reads were sent once each by C8, as `s3q`, with the Gateway showing the book and no dialog up. Their outputs and the four diffs against s2 are below, whole.

## Step 5: restore (C6), then the book after
- **Before the shut:**
  - BK re-hashed, equal to step 1's, its folder holding only that file.
  - `restore-plan.txt` written.
  - The data folder listed: 23 entries.
- **The restore:**
  - Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt.
  - The Data menu (by its label), Backup & Restore, Restore (Down 2 times). Restore Data Path read Tally's data folder.
  - Specify Path (Up 3 times) to the BK's folder. The list showed BRIDGE PILOT LAB, its number and 10-Nov-26 (1:27 am), the BK's time.
  - The company line, then End of List.
- **Two prompts, the only ones answered, each captured first:**
  - "Accept ?": y (`s3-restore-accept`, 20:04:20Z).
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", with R Replace and S Skip: r (`s3-restore-replace`, 20:04:34Z).
- **After the restore:**
  - Select Company came back with no other message.
  - The data folder held 17 entries. Its `Company.1800` held the book's GUID and name, and not the excluded GUID.
  - The book was loaded alone (the name typed, one row, Enter, no prompt). The whole window showed no dialog, and the Gateway showed the book as the one company (`s5-window`, `s5-gateway`).
- **`s5`:** as in the Result. Every EXPECT met; each answer's sha256 equals s2's (`evidence/s2-s5-sha.txt`).

## Step 6: close (C10)
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`).
- At 20:06:22Z: one `tally.exe`, which alone owns 9001, and `GET /status` 200. No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **The prompt rule ended the change at step 3.5.** The one message was answered Back. Steps 3.6 to 4 ran as the rule says: Ctrl+Q, the state reads, the restore, the compare, the close. The run lead was told before the restore.
- **The message box sits outside the window captures.** `shot.sh`'s capture reaches only the window's left part, so the message box is not in `s3-prompt.png`. Two of the desktop tool's own captures, `s3-prompt-desktop.jpg` (the screen) and `s3-prompt-zoom.jpg` (the box), were copied into `shots/` instead.
- **One capture was not kept.** A whole-screen capture taken at 20:00:38Z had another window over the box. It was moved to the Recycle Bin and is not listed.
- **The Backup screen** opened with Backup Data Path at `C:\`, not on cloud storage as C5 says, and with the cursor on Company Name. Backspace took the cursor back to the path field, and no other field was changed by it.
- **The `s3q` reads** were sent once each, by C8, after the stop. They are not a step of the sheet.
- **Go To and the menus** were opened by clicking their labels (K: Company, Y: Data), as the lab's standing rule asks. Before each key, Tally's own title bar was clicked (its empty part) so that the key reached Tally.
- **The Gateway's right-hand panel** showed a red line "N: Manage Rental License" at every Gateway of this sitting, the start-up company's included. It was not pressed.
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.

## Not measured
Every item the sheet lists as not measured. Also not measured:
- `ISGSTON` with GST off, and U, T and R-0010 with GST off. These need the setting saved, which the message's Y would do.
- What Y ("Accept Master & Update GST Status of Vouchers") does to the vouchers' GST status.
- Which act on the features screen brought the 28 elements into the object answer.
- Whether the features read (`render_company_features_request`) moves where the object answer did not.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC. Those are listed by size and sha256 in `kept-local.txt`, with the reason. The O answers hold the company number, so they stay local by that rule.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), each count, the before-write, `--same-row`, `--object-diff`, `--unit-diff`, `--type-diff` and `--diff` outputs, the send times, the screens written down, the BK record and the s2/s5 hash pairs. The process, port and data-folder listings and the restore plan stay local.
- `scripts/`: the step 0 helper, the window capture helper, the send helper and the C6 prep helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

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
== a-s2-O-object.xml: 85654 bytes
  STATUS '1'
  COMPANY 'BRIDGE PILOT LAB' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ; the book's: yes
   ISGSTON 'Yes'; ISGSTCLASSIFON 'No'; GSTAPPLICABLEDATE ''
   CMPGSTDETAILS.LIST: 1 HSNDETAILS.LIST: 1
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

### s3m
```
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s3q
```
== a-s3q-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3q-O-object.xml: 88612 bytes
  STATUS '1'
  COMPANY 'BRIDGE PILOT LAB' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ; the book's: yes
   ISGSTON 'Yes'; ISGSTCLASSIFON 'No'; GSTAPPLICABLEDATE ''
   CMPGSTDETAILS.LIST: 1 HSNDETAILS.LIST: 1
== a-s3q-R-0010.xml: 94474 bytes
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
== a-s3q-T-types.xml: 306946 bytes
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
== a-s3q-U-taxunits.xml: 22636 bytes
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
== a-s5-O-object.xml: 85654 bytes
  STATUS '1'
  COMPANY 'BRIDGE PILOT LAB' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ; the book's: yes
   ISGSTON 'Yes'; ISGSTCLASSIFON 'No'; GSTAPPLICABLEDATE ''
   CMPGSTDETAILS.LIST: 1 HSNDETAILS.LIST: 1
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
```

## Checks and diffs, whole

### s3m against s1 (before-write)
```
$ python -I count15.py <the before-write option> a-s1-M-marks.xml a-s3m-M-marks.xml
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### O, s2 against s3q
```
$ python -I count15.py --object-diff a-s2-O-object.xml a-s3q-O-object.xml
== --object-diff a-s2-O-object.xml a-s3q-O-object.xml: 56 leaves differ
     /ACTIVEFROM[1]: '<absent>' -> ''
     /ACTIVEFROM[1]@TYPE: '<absent>' -> 'Date'
     /ACTIVETO[1]: '<absent>' -> ''
     /ACTIVETO[1]@TYPE: '<absent>' -> 'Date'
     /ALTEREDBY[1]: '<absent>' -> ''
     /ALTEREDBY[1]@TYPE: '<absent>' -> 'String'
     /ALTERID[1]: '<absent>' -> '238'
     /ALTERID[1]@TYPE: '<absent>' -> 'Number'
     /DECIMALPLACESFORPRINTING[1]: '<absent>' -> '2'
     /DECIMALPLACESFORPRINTING[1]@TYPE: '<absent>' -> 'Number'
     /DECIMALPLACES[1]: '<absent>' -> '2'
     /DECIMALPLACES[1]@TYPE: '<absent>' -> 'Number'
     /DECIMALSYMBOL[1]: '<absent>' -> 'paise'
     /DECIMALSYMBOL[1]@TYPE: '<absent>' -> 'String'
     /ENTEREDBY[1]: '<absent>' -> ''
     /ENTEREDBY[1]@TYPE: '<absent>' -> 'String'
     /ERRKEY[1]: '<absent>' -> '0'
     /ERRKEY[1]@TYPE: '<absent>' -> 'Number'
     /HASSPACE[1]: '<absent>' -> 'Yes'
     /HASSPACE[1]@TYPE: '<absent>' -> 'Logical'
     /INMILLIONS[1]: '<absent>' -> 'No'
     /INMILLIONS[1]@TYPE: '<absent>' -> 'Logical'
     /ISDELETED[1]: '<absent>' -> ''
     /ISDELETED[1]@TYPE: '<absent>' -> 'Logical'
     /ISMSTFROMSYNC[1]: '<absent>' -> ''
     /ISMSTFROMSYNC[1]@TYPE: '<absent>' -> 'Logical'
     /ISMULTIGODOWNON[1]: '<absent>' -> 'No'
     /ISMULTIGODOWNON[1]@TYPE: '<absent>' -> 'Logical'
     /ISOCURRENCYCODE[1]: '<absent>' -> ''
     /ISOCURRENCYCODE[1]@TYPE: '<absent>' -> 'String'
     /ISSECURITYONWHENENTERED[1]: '<absent>' -> ''
     /ISSECURITYONWHENENTERED[1]@TYPE: '<absent>' -> 'Logical'
     /ISSUFFIX[1]: '<absent>' -> 'No'
     /ISSUFFIX[1]@TYPE: '<absent>' -> 'Logical'
     /ISUPDATINGTARGETID[1]: '<absent>' -> ''
     /ISUPDATINGTARGETID[1]@TYPE: '<absent>' -> 'Logical'
     /MAILINGNAME[1]: '<absent>' -> 'INR'
     /MAILINGNAME[1]@TYPE: '<absent>' -> 'String'
     /NARRATION[1]: '<absent>' -> ''
     /NARRATION[1]@TYPE: '<absent>' -> 'String'
     /NUMCOSTCENTERS[1]: '<absent>' -> '0'
     /NUMCOSTCENTERS[1]@TYPE: '<absent>' -> 'Number'
     /OBJECTUPDATEACTION[1]: '<absent>' -> ''
     /OBJECTUPDATEACTION[1]@TYPE: '<absent>' -> 'String'
     /REMOTEALTERID[1]: '<absent>' -> '0'
     /REMOTEALTERID[1]@TYPE: '<absent>' -> 'Number'
     /REMOTEALTGUID[1]: '<absent>' -> ''
     /REMOTEALTGUID[1]@TYPE: '<absent>' -> 'String'
     /REMOTEGUID[1]: '<absent>' -> ''
     /REMOTEGUID[1]@TYPE: '<absent>' -> 'String'
     /SORTPOSITION[1]: '<absent>' -> '-1'
     /SORTPOSITION[1]@TYPE: '<absent>' -> 'Number'
     /TYPEOFUPDATEACTIVITY[1]: '<absent>' -> ''
     /TYPEOFUPDATEACTIVITY[1]@TYPE: '<absent>' -> 'String'
     /UPDATEDDATETIME[1]: '<absent>' -> '000000000'
     /UPDATEDDATETIME[1]@TYPE: '<absent>' -> 'DateTime'
exit 0
```

### U, s2 against s3q
```
$ python -I count15.py --unit-diff a-s2-U-taxunits.xml a-s3q-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-s3q-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
```

### T, s2 against s3q
```
$ python -I count15.py --type-diff a-s2-T-types.xml a-s3q-T-types.xml
== --type-diff a-s2-T-types.xml a-s3q-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
```

### R-0010, s2 against s3q
```
$ python -I count15.py --diff a-s2-R-0010.xml a-s3q-R-0010.xml
== --diff a-s2-R-0010.xml a-s3q-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### M, s5 against s1 (--same-row)
```
$ python -I count15.py --same-row a-s1-M-marks.xml a-s5-M-marks.xml
== a-s5-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### O, s2 against s5
```
$ python -I count15.py --object-diff a-s2-O-object.xml a-s5-O-object.xml
== --object-diff a-s2-O-object.xml a-s5-O-object.xml: 0 leaves differ
exit 0
```

### U, s2 against s5
```
$ python -I count15.py --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
exit 0
```

### T, s2 against s5
```
$ python -I count15.py --type-diff a-s2-T-types.xml a-s5-T-types.xml
== --type-diff a-s2-T-types.xml a-s5-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
exit 0
```

### R-0010, s2 against s5
```
$ python -I count15.py --diff a-s2-R-0010.xml a-s5-R-0010.xml
== --diff a-s2-R-0010.xml a-s5-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### sha256, s2 then s5, per answer
```
M-marks 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
O-object 40afcf4dcd4ca633731ce3415c42d2086b1bb323afd05392136b518c4a73d596 40afcf4dcd4ca633731ce3415c42d2086b1bb323afd05392136b518c4a73d596
U-taxunits e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
T-types cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
R-0010 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
```
