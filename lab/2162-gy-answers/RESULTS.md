# Row 2162-GY: with Tally's GST message answered Y, the company object reads `ISGSTON` No; the tax units, voucher types and R-0010's voucher row read as before; restored

Run on 9 Nov 2026, 20:54:20Z to 21:07:28Z (02:24 to 02:37 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-gy-sheet` (0b8f09d8), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no request writes and no Bridge build. One company setting was changed on Tally's screen and saved through Tally's own message; the book was then restored from the row's own backup. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran after row 683-LM (Tally quit at 20:45:50Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The book was restored after step 4, so the marks it leaves are those it found (68/245).

The one GSTIN in this folder is the synthetic family's, as the sheet names it: the company's own `08ZZZZZ0000Z1ZQ`.

**Result.**
- **s2, before:** each of the five answers' sha256 equals row 2162-GF's s2 answer of the same request (`evidence/s2-vs-gf-sha.txt`). O `ISGSTON` Yes, `ISGSTCLASSIFON` No, `GSTAPPLICABLEDATE` empty. M 68/245. Every EXPECT met.
- **Step 3, the setting:** F11 at the Gateway opened "Company Features  Alteration", its fields the same as row 2162-GF's, line for line. Row 2162-GF's path: Down 13, `n`, Enter. The one GST field, `Enable Goods and Services Tax (GST)`, read No; its sub-line went dimmed, reading No; no message.
- **On Ctrl+A, the message box came, word for word as row 2162-GF's:**
  - "GST-related changes in masters may impact the GST Status of vouchers in Returns."
  - "Y : Accept Master & Update GST Status of Vouchers"
  - "N : Back"
- **NEW PATH: `y` was pressed once.** Tally then showed a busy window over the features screen, titled "Updating the GST status in the vouchers", with a progress bar: "0%" and "1 of 44 vouchers" right after the key, "66%" and "30 of 44 vouchers" about 2 seconds later. Nothing was pressed while it showed. The Gateway came back by itself within 15 seconds of `y`, with no other message. Its lines and menu were the same as before the change; it shows no GST line either way.
- **s4, the book with GST off** (each read answered, STATUS 1; none failed):
  - **O: `ISGSTON` reads No** (Yes at s2). `ISGSTCLASSIFON` still No, `GSTAPPLICABLEDATE` still empty; still one `CMPGSTDETAILS.LIST` and one `HSNDETAILS.LIST`.
  - O against s2, 28 leaves differ in all: `ISGSTON` Yes -> No; `ALTVCHID` 68 -> 71; `ALTMSTID` 245 -> 246; `RETTRANSALTERID` 11 -> 17; `NUMCOSTCENTERS` newly present (0). `CLOSINGGROSSPROFIT` (14911.90) and `CLOSINGPROFIT` (14912.06) are absent at s4, and so are ten Amount elements that were empty at s2: opening and closing stock, opening and closing inward and outward track, opening profit, and total opening credits and debits. The answer shrank from 85,654 B to 84,454 B.
  - Of the 28 elements row 2162-GF's s3q held after an unsaved change, only `NUMCOSTCENTERS` is in s4.
  - **U: unchanged.** The same two tax units, `Default Tax Unit` and `Rajasthan Registration` (GST, `08ZZZZZ0000Z1ZQ`, from 20260401, Regular, `ISINACTIVE` No), each with 0 leaves differing.
  - **T: unchanged.** 28 VOUCHERTYPE rows; `BRIDGE Sales` 0 leaves differ.
  - **R-0010: the voucher row unchanged.** `lists compared: 1`, `legs: 6 -> 6` only. Its three GST status flags read the same at s2 and s4: `VCHGSTSTATUSISINCLUDED` Yes, `VCHGSTSTATUSISUNCERTAIN` No, `VCHGSTSTATUSISEXCLUDED` No.
  - **M:** 71/246 against s3m's 68/245: ALTVCHID +3, ALTMSTID +1.
  - **The bytes:** every s4 answer carries the same seven header counters, which were all 0 at s2: COMPANY 1, GROUP 6, LEDGER 588, CURRENCY 45, TAXUNIT 88, VOUCHERNUMBERSERIES 88, VOUCHER 82. In U, T and R-0010 those seven lines are the only lines that differ from s2 (`evidence/linediff-s2-s4-*.txt`). Each of those three answers grew by 12 B.
- **The restore** (C6), then s5:
  - M, O, U, T and R-0010 are each byte-equal to s2's (`evidence/s2-s5-sha.txt`). O reads `ISGSTON` Yes again, and the counters are 0 again.
  - `--same-row` against s1 is equal (68/245). `--object-diff`, `--unit-diff` and `--type-diff` against s2 show 0 leaves. R-0010 shows `lists compared: 1` and `legs: 6 -> 6` only.

**By the sheet's Decides:**
- **O's `ISGSTON` reads No at s4, and Yes at s2 and s5:** measured. A saved change moves it, and the setting reads in the object answer the build already fetches. By the sheet, the build then reads `ISGSTON` beside `ISGSTCLASSIFON` and refuses an invoice by name unless it reads exactly Yes. How the s2 and s4 object answers reach the build's fixtures is for the run lead: they stay on the lab PC under the present rules, as they name the company number.
- **O's `ISGSTON` does not move, or is absent:** not seen.
- **U is unchanged at s4:** measured. The registration stays on the books with GST off, with nothing marked inactive. By the sheet's code reading, today's build would parse a valid registration on this company and admit the invoice, so the named refusal closes a real gap.
- **U changes at s4:** not seen.
- **T changes at s4:** not seen.
- **R-0010 changes at s4:** not seen. Its GST status flags did not move, although Tally's busy window counted 44 vouchers. What the update did to the book's other vouchers was not read.
- **Tally shows anything at 3.5 or 3.6 that this sheet does not name:** 3.5 was the named message, word for word. At 3.6 the sheet names a progress bar, a count or a busy window in general terms. The one Tally showed is written down above, word for word.

**The sheet's estimates:** s2 equal to GF's, the message word for word, the Gateway after a short pause with no other message, O's `ISGSTON` No, and U unchanged: each was so. Of row 2162-GF's 28 elements, only `NUMCOSTCENTERS` came. R-0010's GST status flags did not move. s5 equals s2 and s1.

## Step 1: load, check and back up
- Sheet files: all 10 equal to `SHA256.txt` at 0b8f09d8. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count15.py`, `M-marks.xml`, `O-object.xml`, `R-0010.xml`, `U-taxunits.xml` and `T-types.xml` are byte-equal to row 2162-GF's.
- Tally was started once, at 20:54:20Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2), after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 at 20:55:25Z: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID, and not the excluded GUID.
- The book was loaded alone from Select Company: the name typed, one row, Enter, no prompt. The Gateway showed (`s1-gateway`):
  - BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title;
  - current period 1-Apr-26 to 31-Mar-27;
  - current date Tuesday 15-Sep-2026;
  - date of last entry 15-Sep-26.
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5):
  - Before the backup, `C:\lab\2162-gy\backup-1` was new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. As in row 2162-GF, the cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times), then `C:\lab\2162-gy\backup-1` typed, and its row taken.
  - BRIDGE PILOT LAB alone, End of List. "Accept ?" was captured (`s1-bk-accept`, 20:57:30Z) and answered y; no other message.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `f673e5b8ae29919f7a0ae1aa717a63a2e359536a00538e0f8a15d4d333c118ab`, written at 02:27 IST. The Restore list showed it as 10-Nov-26 (2:27 am).
- `s1b`, the marks after the backup: 68/245, unchanged.

## Step 2: the book before
`s2`: O, U, T, R-0010 and M, as in the Result. Every EXPECT met. The outputs are below, whole.

## Step 3: the setting, on Tally's screen
`s3m`, then the before-write check against s1: one COMPANY row, the same GUID (68/245); passed. Every field, key and message is in `evidence/s3-screens.txt`. In short:
1. At the Gateway (`s3-gateway-before`), F11 once (rows 1342-W14 and 2162-GF opened it this way). The screen's title: "Company Features  Alteration", with BRIDGE PILOT LAB in the title bar. Its fields before the change, as text (`s3-features-before`), are the same as row 2162-GF's, line for line.
2. One field names GST and reads Yes: `Enable Goods and Services Tax (GST)`.
3. Row 2162-GF's step 3.3 path: Down 13 times from Maintain Accounts (7, 5 and 1, captured after each group); each field passed kept its value. `n` typed: the field showed `n` in its edit box, no message (`s3-gst-n-typed`). Enter: the field read No, its sub-line dimmed reading No, and the cursor on Enable Tax Deducted at Source (TDS) (`s3-gst-no`). No message, no sub-screen, no list.
4. No other field was touched.
5. Ctrl+A: no "Accept ?". The message box in the Result came up at the bottom right of the window (`s3-prompt-desktop`, `s3-prompt-zoom`), the same three lines as row 2162-GF's. It was captured and zoomed before any key. **NEW PATH:** `y`, once, at 21:01:2xZ.
6. The busy window, then the Gateway, as in the Result (`s3-y-busy-0pct-desktop`, `s3-y-busy-66pct-desktop`, `s3-gateway-after`, `s3-gateway-after-desktop`). Screens passed: the features screen, the message box, the busy window, the Gateway.

Keys pressed in step 3, in all: F11; Down 13 times; n; Enter; Ctrl+A; y.

## Step 4: the book with GST off
- C2 at 21:02:04Z, with the Gateway showing: one `tally.exe`, which alone owns 9001; `GET /status` 200.
- `s4` sent at 21:02:15Z: M, O, U, T and R-0010, each answered with STATUS 1. The four diffs against s2 and the line compares are below, whole.

## Step 5: restore (C6), then the book after
- **Before the shut:**
  - BK re-hashed, equal to step 1's, its folder holding only that file.
  - `restore-plan.txt` written.
  - The data folder listed: 24 entries.
- **The restore:**
  - Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt.
  - The Data menu (by its label), Backup & Restore, Restore (Down 2 times). Restore Data Path read Tally's data folder.
  - Specify Path (Up 3 times) to the BK's folder. The list showed BRIDGE PILOT LAB, its number and 10-Nov-26 (2:27 am), the BK's time (`s5-restore-list-desktop`).
  - The company line, then End of List.
- **Two prompts, the only ones answered, each captured first:**
  - "Accept ?": y (`s5-restore-accept-desktop`, 21:05:0xZ).
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", with R Replace and S Skip: r (`s5-restore-replace-desktop`, 21:05:1xZ).
- **After the restore:**
  - Select Company came back with no other message.
  - The data folder held 17 entries. The seven names not there were the same seven as after row 2162-HS's restore (`CmpSchMetaData.1800`, `Index.1800` and five `.TSF` files). Its `Company.1800` held the book's GUID and name, and not the excluded GUID.
  - The book was loaded alone (the name typed, one row, Enter, no prompt). The Gateway showed the book as the one company, with the same period and date (`s5-gateway`).
  - C2 at 21:06:07Z: one `tally.exe` owning 9001, `GET /status` 200.
- **`s5`:** as in the Result. Every EXPECT met; each answer's sha256 equals s2's.

## Step 6: close (C10)
- At 21:06:35Z (C11, after the last request): one `tally.exe`, which alone owns 9001, and `GET /status` 200.
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). At 21:07:03Z: one `tally.exe` owning 9001, and `GET /status` 200. No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **The typed name:** at Select Company, both times, the typing tool sent the name key by key and the spaces did not register, so the field showed `BRIDGEPILOTLAB`. Each time the list held one row, BRIDGE PILOT LAB, and that row was taken.
- **The message box and the busy window sit outside the window captures.** `shot.sh`'s capture reaches only the window's left part. The desktop tool's own captures were copied into `shots/` instead, as row 2162-GF did.
- **Times written as `xZ`** (for example 21:01:2xZ) are known to the ten seconds, from the captures and the clock reads around them.
- **The line compares** (`evidence/linediff-s2-s4-*.txt`) were made by a short lab script, not by `count15.py`. Each prints both answers' size and sha256 and every line that differs.
- **Go To and the menus** were opened by clicking their labels (K: Company, Y: Data), as the lab's standing rule asks. Before each key, Tally's own title bar was clicked (its empty part) so that the key reached Tally.
- **The Gateway's right-hand panel** showed a red line "N: Manage Rental License" at every Gateway of this sitting, the start-up company's included. It was not pressed.
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.

## Follow-ups
- None from R-0010: its voucher row and GST status flags did not move.
- The 44 vouchers Tally counted while updating the GST status, and what it changed on them, were not read. ALTVCHID moved by 3.

## Not measured
Every item the sheet lists as not measured. Also not measured:
- The company features collection's `ISGSTON` on this book: by the Decides, it is not needed, as O carries the setting.
- Which three vouchers moved ALTVCHID, and which master moved ALTMSTID.
- Why `CLOSINGPROFIT`, `CLOSINGGROSSPROFIT` and the ten empty Amount elements are absent from O with GST off.
- What the seven header counters count.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC. Those are listed by size and sha256 in `kept-local.txt`, with the reason. The O answers hold the company number, so they stay local by that rule.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), each count, the before-write, `--same-row`, `--object-diff`, `--unit-diff`, `--type-diff` and `--diff` outputs, the line compares, the send times, the screens written down, the BK record and the hash pairs. The process, port and data-folder listings and the restore plan stay local.
- `scripts/`: the step 0 helper, the window capture helper, the send helper, the C6 prep helper and the line compare script.
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

### s1b
```
== a-s1b-M-marks.xml: 3394 bytes
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

### s4
```
== a-s4-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4-O-object.xml: 84454 bytes
  STATUS '1'
  COMPANY 'BRIDGE PILOT LAB' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ; the book's: yes
   ISGSTON 'No'; ISGSTCLASSIFON 'No'; GSTAPPLICABLEDATE ''
   CMPGSTDETAILS.LIST: 1 HSNDETAILS.LIST: 1
== a-s4-R-0010.xml: 94486 bytes
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
== a-s4-T-types.xml: 306958 bytes
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
== a-s4-U-taxunits.xml: 22648 bytes
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

### sha256, s2 against row 2162-GF's s2, per answer
```
O-object 85654 B 40afcf4dcd4ca633731ce3415c42d2086b1bb323afd05392136b518c4a73d596 GF 40afcf4dcd4ca633731ce3415c42d2086b1bb323afd05392136b518c4a73d596 equal
U-taxunits 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 GF e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 equal
T-types 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed GF cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed equal
R-0010 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 GF b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 equal
M-marks 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 GF 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 equal
```

### s3m against s1 (before-write)
```
$ python -I count15.py <the before-write option> a-s1-M-marks.xml a-s3m-M-marks.xml
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### O, s2 against s4
```
== --object-diff a-s2-O-object.xml a-s4-O-object.xml: 28 leaves differ
     /ALTMSTID[1]: '245' -> '246'
     /ALTVCHID[1]: '68' -> '71'
     /CLOSINGGROSSPROFIT[1]: '14911.90' -> '<absent>'
     /CLOSINGGROSSPROFIT[1]@TYPE: 'Amount' -> '<absent>'
     /CLOSINGINWARDTRACK[1]: '' -> '<absent>'
     /CLOSINGINWARDTRACK[1]@TYPE: 'Amount' -> '<absent>'
     /CLOSINGOUTWARDTRACK[1]: '' -> '<absent>'
     /CLOSINGOUTWARDTRACK[1]@TYPE: 'Amount' -> '<absent>'
     /CLOSINGPROFIT[1]: '14912.06' -> '<absent>'
     /CLOSINGPROFIT[1]@TYPE: 'Amount' -> '<absent>'
     /CLOSINGSTOCK[1]: '' -> '<absent>'
     /CLOSINGSTOCK[1]@TYPE: 'Amount' -> '<absent>'
     /ISGSTON[1]: 'Yes' -> 'No'
     /NUMCOSTCENTERS[1]: '<absent>' -> '0'
     /NUMCOSTCENTERS[1]@TYPE: '<absent>' -> 'Number'
     /OPENINGINWARDTRACK[1]: '' -> '<absent>'
     /OPENINGINWARDTRACK[1]@TYPE: 'Amount' -> '<absent>'
     /OPENINGOUTWARDTRACK[1]: '' -> '<absent>'
     /OPENINGOUTWARDTRACK[1]@TYPE: 'Amount' -> '<absent>'
     /OPENINGPROFIT[1]: '' -> '<absent>'
     /OPENINGPROFIT[1]@TYPE: 'Amount' -> '<absent>'
     /OPENINGSTOCK[1]: '' -> '<absent>'
     /OPENINGSTOCK[1]@TYPE: 'Amount' -> '<absent>'
     /RETTRANSALTERID[1]: '11' -> '17'
     /TOTALOPENINGCREDITS[1]: '' -> '<absent>'
     /TOTALOPENINGCREDITS[1]@TYPE: 'Amount' -> '<absent>'
     /TOTALOPENINGDEBITS[1]: '' -> '<absent>'
     /TOTALOPENINGDEBITS[1]@TYPE: 'Amount' -> '<absent>'
```

### U, s2 against s4
```
== --unit-diff a-s2-U-taxunits.xml a-s4-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### T, s2 against s4
```
== --type-diff a-s2-T-types.xml a-s4-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### R-0010, s2 against s4
```
== --diff a-s2-R-0010.xml a-s4-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### M, s2 against s4, line by line
```
== a-s2-M-marks.xml (3394 B, sha256 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6) vs a-s4-M-marks.xml (3406 B, sha256 aa41215c98cc11871981b6a2a0ed15a2c760a94b219d2f4977c3291b1bbea463)
  @@ -9,3 +9,3 @@
  -    <COMPANY>0</COMPANY>
  -    <GROUP>0</GROUP>
  -    <LEDGER>0</LEDGER>
  +    <COMPANY>1</COMPANY>
  +    <GROUP>6</GROUP>
  +    <LEDGER>588</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>45</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>88</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>88</VOUCHERNUMBERSERIES>
  +    <VOUCHER>82</VOUCHER>
  @@ -50,2 +50,2 @@
  -     <ALTVCHID TYPE="Number"> 68</ALTVCHID>
  -     <ALTMSTID TYPE="Number"> 245</ALTMSTID>
  +     <ALTVCHID TYPE="Number"> 71</ALTVCHID>
  +     <ALTMSTID TYPE="Number"> 246</ALTMSTID>
  23 diff lines
```

### O, s2 against s4, line by line
```
== a-s2-O-object.xml (85654 B, sha256 40afcf4dcd4ca633731ce3415c42d2086b1bb323afd05392136b518c4a73d596) vs a-s4-O-object.xml (84454 B, sha256 c67e612ddff6582033dadcd005bb02f0541accd05fa61ed970080de2f6953dfc)
  @@ -14,3 +14,3 @@
  -    <COMPANY>0</COMPANY>
  -    <GROUP>0</GROUP>
  -    <LEDGER>0</LEDGER>
  +    <COMPANY>1</COMPANY>
  +    <GROUP>6</GROUP>
  +    <LEDGER>588</LEDGER>
  @@ -24 +24 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>45</CURRENCY>
  @@ -44 +44 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>88</TAXUNIT>
  @@ -47,2 +47,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>88</VOUCHERNUMBERSERIES>
  +    <VOUCHER>82</VOUCHER>
  @@ -411 +411 @@
  -     <ISGSTON TYPE="Logical">Yes</ISGSTON>
  +     <ISGSTON TYPE="Logical">No</ISGSTON>
  @@ -464,0 +465 @@
  +     <NUMCOSTCENTERS TYPE="Number">0</NUMCOSTCENTERS>
  @@ -485 +486 @@
  -     <ALTVCHID TYPE="Number"> 68</ALTVCHID>
  +     <ALTVCHID TYPE="Number"> 71</ALTVCHID>
  @@ -494 +495 @@
  -     <ALTMSTID TYPE="Number"> 245</ALTMSTID>
  +     <ALTMSTID TYPE="Number"> 246</ALTMSTID>
  @@ -498 +499 @@
  -     <RETTRANSALTERID TYPE="Number"> 11</RETTRANSALTERID>
  +     <RETTRANSALTERID TYPE="Number"> 17</RETTRANSALTERID>
  @@ -503,11 +503,0 @@
  -     <TOTALOPENINGCREDITS TYPE="Amount"></TOTALOPENINGCREDITS>
  -     <TOTALOPENINGDEBITS TYPE="Amount"></TOTALOPENINGDEBITS>
  -     <OPENINGPROFIT TYPE="Amount"></OPENINGPROFIT>
  -     <CLOSINGPROFIT TYPE="Amount">14912.06</CLOSINGPROFIT>
  -     <CLOSINGGROSSPROFIT TYPE="Amount">14911.90</CLOSINGGROSSPROFIT>
  -     <OPENINGSTOCK TYPE="Amount"></OPENINGSTOCK>
  -     <CLOSINGSTOCK TYPE="Amount"></CLOSINGSTOCK>
  -     <OPENINGINWARDTRACK TYPE="Amount"></OPENINGINWARDTRACK>
  -     <OPENINGOUTWARDTRACK TYPE="Amount"></OPENINGOUTWARDTRACK>
  -     <CLOSINGINWARDTRACK TYPE="Amount"></CLOSINGINWARDTRACK>
  -     <CLOSINGOUTWARDTRACK TYPE="Amount"></CLOSINGOUTWARDTRACK>
  44 diff lines
```

### U, s2 against s4, line by line
```
== a-s2-U-taxunits.xml (22636 B, sha256 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989) vs a-s4-U-taxunits.xml (22648 B, sha256 5dabc0e73867512e454758f43bd0f31797d4b8b9cec319c2be9629e08d029887)
  @@ -9,3 +9,3 @@
  -    <COMPANY>0</COMPANY>
  -    <GROUP>0</GROUP>
  -    <LEDGER>0</LEDGER>
  +    <COMPANY>1</COMPANY>
  +    <GROUP>6</GROUP>
  +    <LEDGER>588</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>45</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>88</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>88</VOUCHERNUMBERSERIES>
  +    <VOUCHER>82</VOUCHER>
  18 diff lines
```

### T, s2 against s4, line by line
```
== a-s2-T-types.xml (306946 B, sha256 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed) vs a-s4-T-types.xml (306958 B, sha256 7a61c5de1388c808446c52727b78a8012a567eabe80c431a3f9cc89b9515adde)
  @@ -9,3 +9,3 @@
  -    <COMPANY>0</COMPANY>
  -    <GROUP>0</GROUP>
  -    <LEDGER>0</LEDGER>
  +    <COMPANY>1</COMPANY>
  +    <GROUP>6</GROUP>
  +    <LEDGER>588</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>45</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>88</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>88</VOUCHERNUMBERSERIES>
  +    <VOUCHER>82</VOUCHER>
  18 diff lines
```

### R-0010, s2 against s4, line by line
```
== a-s2-R-0010.xml (94474 B, sha256 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5) vs a-s4-R-0010.xml (94486 B, sha256 10abd6cbac991097f2e53fea53852628f6a11a78afe6c91723984b235d81a86f)
  @@ -9,3 +9,3 @@
  -    <COMPANY>0</COMPANY>
  -    <GROUP>0</GROUP>
  -    <LEDGER>0</LEDGER>
  +    <COMPANY>1</COMPANY>
  +    <GROUP>6</GROUP>
  +    <LEDGER>588</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>45</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>88</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>88</VOUCHERNUMBERSERIES>
  +    <VOUCHER>82</VOUCHER>
  18 diff lines
```

### M, s5 against s1 (--same-row)
```
== a-s5-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### O, s2 against s5
```
== --object-diff a-s2-O-object.xml a-s5-O-object.xml: 0 leaves differ
```

### U, s2 against s5
```
== --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### T, s2 against s5
```
== --type-diff a-s2-T-types.xml a-s5-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### R-0010, s2 against s5
```
== --diff a-s2-R-0010.xml a-s5-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### sha256, s2 then s5, per answer
```
M-marks s2 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 s5 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 EQUAL
O-object s2 40afcf4dcd4ca633731ce3415c42d2086b1bb323afd05392136b518c4a73d596 s5 40afcf4dcd4ca633731ce3415c42d2086b1bb323afd05392136b518c4a73d596 EQUAL
U-taxunits s2 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 s5 e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 EQUAL
T-types s2 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed s5 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed EQUAL
R-0010 s2 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 s5 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 EQUAL
```
