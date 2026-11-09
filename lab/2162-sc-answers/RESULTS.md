# Row 2162-SC: with the company GSTIN's state code changed to 27 and its registration's state kept Rajasthan, Tally asked to recheck voucher status, and GSTR-1 for 2026-08 then counted 32 uncertain vouchers under a registration mismatch

Run on 9 Nov 2026, 16:46:29Z to 17:05:40Z (22:16 to 22:35 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-sc-sheet` (c4881cf2), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no request writes and no Bridge build. One master alteration was keyed on Tally's screen, and the book was then restored from the row's own backup. Every value below is M, measured by this run, in one run: **partial** (one change, one sitting, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran in the gap between row 2162-PB (Tally quit at 16:36:15Z) and sitting 2 of row 2360-L (after 00:00 IST on 10 Nov), on the run lead's word. Tally was started for it once from its desktop shortcut and closed by its own Quit at the end. The book was restored, so the marks it leaves are those it found (68/245).

The two GSTINs in this folder are the synthetic family's, as the sheet names them: the company's own `08ZZZZZ0000Z1ZQ` and the changed value `27ZZZZZ0000Z1ZQ`.

**Result.**
- **s2 and step 3, before:** U two units, each the book's: `Default Tax Unit`, and `Rajasthan Registration` (GST, `08ZZZZZ0000Z1ZQ`, GSTOLDREGNUMBER empty, one dated row 20260401, Rajasthan, Rajasthan, Regular, not inactive). R-0010 `BRIDGE Walk-in`, included Yes, uncertain No, excluded No. W 40 rows, 34 with BRIDGEVCHISSALES Yes. GSTR-1 (`08ZZZZZ0000Z1ZQ`, 1-Aug-26 to 31-Aug-26): Total Vouchers 39; Included in Return 27; Not Relevant 7; Uncertain 5 (HSN/SAC is invalid, mismatched, or not specified 2; Mismatch between Expected Tax Amount and Modified Tax Amount in transaction 3). B2C (Small) Invoices - 7: 27 (2), taxable 13,508.70, tax 766.50.
- **Step 4, the change:** on "GST Registration Alteration" for `Rajasthan Registration`, the field `GSTIN/UIN` was changed from `08ZZZZZ0000Z1ZQ` to `27ZZZZZ0000Z1ZQ`; State stayed Rajasthan, and no other field was touched. **Tally took it with no message**: no date was asked, and Ctrl+A brought no "Accept ?" prompt and no other prompt; Tally went back to the List of GST Registrations.
- **s5, after the change:** ALTMSTID 245 to 246, ALTVCHID 68 unchanged. `--unit-diff` printed three leaves of `Rajasthan Registration`, `ALTERID` 206 to 246, `GSTREGNUMBER` and `@TAXREGISTRATION` 08ZZZZZ0000Z1ZQ to 27ZZZZZ0000Z1ZQ, and 0 for `Default Tax Unit`. **GSTOLDREGNUMBER stayed empty**, and the dated row is unchanged. R-0010 `--diff` printed only `lists compared: 1` and `legs: 6 -> 6` (included Yes, uncertain No). W's 40 rows are as at s2.
- **Step 6, GSTR-1 after:** before the report opened, a dialog with the title bar "GSTR-1" appeared, word for word:
  > GST-related details were altered in the masters, hence voucher status for GST reporting will change.
  > Number of Vouchers in the Current Period : 39
  > Number of Vouchers in All Periods : 40
  > Select the period for which you want to check the vouchers.

  It offered "A: All Periods" and "C: Current Period". The row stopped there, and on the run lead's word C was pressed. GSTR-1 then showed GST Registration `27ZZZZZ0000Z1ZQ`: Total Vouchers 39; Included in Return with no count; Not Relevant 7; **Uncertain Transactions (Corrections needed) 32**; Marked for Deletion on Portal 4. Every Return View line shown was empty. The uncertain list read, word for word: "Transactions with Incomplete/Mismatch in Information 32", then Outward Supplies, "Mismatch in Information Between Masters and Transactions", and "Mismatch in GST Registration Details between the GST Registration master and the transaction 32".
- **s7, after the look:** `--same-row` s5/s7 equal (68/246). **R-0010's status moved:** `VCHGSTSTATUSISINCLUDED` Yes to No and `VCHGSTSTATUSISUNCERTAIN` No to Yes, with ALTERID 17 unchanged, `lists compared: 1` and `legs: 6 -> 6`.
- **Header counts:** each answer's `DESC/CMPINFO` block moved twice. `TAXUNIT` went from 0 to 1 at s5, in M, U, R-0010 and W. `VOUCHER` went from 0 to 68 at s7, in M and R-0010. Both read 0 again at s8.
- **Restore:** by C6. s8: M `--same-row` against s1 equal (68/245); U `--unit-diff` against s2, 0 leaves in each unit; R-0010 `--diff` against s2 printed only `lists compared: 1` and `legs: 6 -> 6`; M, U, R-0010 and W each byte-equal to s2's.

**By the sheet's Decides:**
- **Step 6 counts the month's vouchers as uncertain:** measured. With this one change, GSTR-1 counted 32 of the period's 39 vouchers as uncertain and none as included (step 3: 5 and 27). Every one of the 32 sits under one line, "Mismatch in GST Registration Details between the GST Registration master and the transaction". The screen's words are recorded as shown. They do not include the words "invalid or not specified" that the sheet gives for the rehearsal's message, and whether the two messages are the same is not decided here. PARTIAL: one change, one sitting, one book, one release. The rehearsal company's cause stays not established.
- **Tally refuses the number:** not seen. It saved the number with no message at the field or on Ctrl+A. The status dialog came only when GSTR-1 was opened.
- **s5's GSTOLDREGNUMBER:** empty. Tally did not keep the earlier number on this change.
- **s5 or s7's R-0010 status flags differ from s2's:** s5 equal to s2. **s7 differs**: after the dialog was answered C and GSTR-1 was looked at, R-0010 read included No and uncertain Yes. This row does not separate the dialog's answer from the look itself. On the run lead's word, this was recorded as its own before-and-after, and the restore followed as written.
- **s8 equal to s2 and s1:** measured.

## Step 1: load, check and back up
- Sheet files: all 9 equal to `SHA256.txt` at c4881cf2; `LAB-COMMON.md` and `run.py` byte-equal to row 2162-PB's.
- C2 step 0 with no `tally.exe` running. Tally was started once, at 16:46:29Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2). Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt).
- C2 at 16:47:25Z: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID. No Bridge process was ended and no Bridge tool was called (a standing rule of the lab).
- Loaded alone from Select Company (the name typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26 (`s1-gateway`).
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5): the Data menu (by its label), Backup & Restore, Backup, Specify Path to the new empty folder `C:\lab\2162-sc\backup-1`. BRIDGE PILOT LAB alone, End of List; "Accept ?" captured (`s1-bk-accept`) and answered y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `80dffaf7856567c135f48b3ee6e4f107eb5ee3dd3dd0d5a67dabbab3e582d9b5`, 22:19 IST; the Restore list was to show 9-Nov-26 (10:19 pm).

## Step 2: the book before
`s2`: U, R-0010, W and M, as in the Result. EXPECT met. The outputs are below, whole.

## Step 3: GSTR-1 before, read only
Every screen, line and key is written down in `evidence/s3-gstr1-screens.txt`; the captures are listed in `SCREENSHOTS.txt`. In short:
1. The Gateway (`s3-1-gateway`). Go To by clicking its "G: Go To" label (the sheet says Alt+G; see Deviations); `GSTR-1` typed; the List of Reports, GST Reports: GSTR-1, GSTR-1 Reconciliation, GSTR-1 Filed Summary - Downloaded, no line highlighted. One Down highlighted GSTR-1; Enter. No list of GST registrations appeared.
2. GSTR-1, GST Registration `08ZZZZZ0000Z1ZQ`, Status Not Filed, 1-Aug-26 to 31-Aug-26, with F2: Period offered. No period key was pressed.
3. The summary (`s3-3-gstr1-summary`): Total Vouchers 39; Included in Return 27 (Ready for Upload 27; Modified in Books After Upload/Export 27); Not Relevant for This Return 7; Uncertain Transactions (Corrections needed) 5; Check Vouchers Having Potential Conflicts with Masters: Yes. Of the Return View's lines shown, only B2C (Small) Invoices - 7 holds values: 27 (2), taxable 13,508.70, CGST 383.26, SGST/UTGST 383.24, tax 766.50; the total the same.
4. Uncertain above 0: Down 5 times to Uncertain Transactions, Enter once. "GSTR-1 - Uncertain Transactions" (`s3-4-uncertain`): Transactions with Incomplete/Mismatch in Information 5; Outward Supplies; Invalid or Missing Information: "HSN/SAC is invalid, mismatched, or not specified" 2; Mismatch in Information Between Masters and Transactions: "Mismatch between Expected Tax Amount and Modified Tax Amount in transaction" 3. Only Esc was pressed on it.
5. Esc, Esc: the Gateway (`s3-5-gateway`). No prompt and no dialog appeared.

## Step 4: the change, on Tally's screen
`s4m`, then `<the before-write option>` against s1: one COMPANY row, the same GUID (68/245). Every screen, field and key is in `evidence/s4-screens.txt`. In short:
1. At the Gateway (`s4-1-gateway`), "G: Go To" clicked and `GST Registration` typed. The field kept only `GSTRe`. The list offered Create Voucher, Create Master, Alter Master, Expand All, Show More, GST Reports (highlighted) and Track GST Return Activities, and an Error box read "Oops!" / "Spelling Error" (`s4-1-goto-stop`). No line named Alter and GST Registration, so the row stopped by step 4.1 and asked. On the run lead's word: Esc once closed the Error box; Backspace 5 times cleared the field; `Alter Master` was typed (the field showed `AlterMaster`) and Enter pressed on that line. "Master Alteration" (`s4-1b-master-alteration`) listed, among its Statutory Masters, one line reading exactly `GST Registration`; Down 11 times to it, Enter.
2. "List of GST Registrations" (`s4-2-registrations`): Create, Back, and one line, `Rajasthan Registration`. Enter on it.
3. "GST Registration Alteration" (`s4-3-alteration`). Its fields, before the change: Registration status Active; State Rajasthan; Registration type Regular; Assessee of Other Territory No; GSTIN/UIN `08ZZZZZ0000Z1ZQ`; Periodicity of GSTR-1 Monthly; GST Username empty; Mode of Filing Not Applicable; e-Invoicing applicable No; e-Way Bill applicable Yes, from 1-Apr-26, for intrastate Yes. One field held `08ZZZZZ0000Z1ZQ`: `GSTIN/UIN`.
4. Enter, Enter to `GSTIN/UIN`, its value highlighted. One Backspace moved the cursor back to Assessee of Other Territory and changed nothing (see Deviations). Enter back to `GSTIN/UIN`; `27ZZZZZ0000Z1ZQ` typed over the highlighted value; zoomed before any Enter, it read `27ZZZZZ0000Z1ZQ`, 15 characters, capitals (`s4-4-gstin-typed`).
5. No other field was touched. State read Rajasthan. No date was asked and no other field or screen opened.
6. Ctrl+A: no "Accept ?" prompt and no other prompt or message. Tally went back to the List of GST Registrations (`s4-6-after-accept`, 16:56:37Z).
7. Ctrl+Q on that list: the Gateway (`s4-7-gateway`).

## Step 5: the book after the change
`s5`: M, U, R-0010 and W, as in the Result. The `--unit-diff` and `--diff` outputs are below, whole. W's rows as counted equal s2's. The answer differs from s2's in one header line only, `DESC/CMPINFO/TAXUNIT` 0 to 1 (`evidence/w-s2-s5-compare.txt`).

## Step 6: GSTR-1 after, read only
Every screen, line and key is in `evidence/s6-gstr1-screens.txt`. In short:
1. The Gateway (`s6-1-gateway`). Go To, `GSTR-1`, the same List of Reports, Down once, Enter. No list of GST registrations appeared. The dialog in the Result appeared over the Gateway (`s6-1b-status-dialog`), with "C: Current Period" framed as its default. The sheet does not name it, so the row stopped with no key pressed and asked. On the run lead's word, `c` was pressed.
2. GSTR-1, GST Registration `27ZZZZZ0000Z1ZQ`, Status Not Filed, 1-Aug-26 to 31-Aug-26. No period key was pressed.
3. The summary (`s6-3-gstr1-summary`), as in the Result. The line "Check Vouchers Having Potential Conflicts with Masters" was not shown.
4. Uncertain above 0: Down 3 times, Enter once. "GSTR-1 - Uncertain Transactions" (`s6-4-uncertain`), its lines as in the Result. Only Esc was pressed on it.
5. Esc, Esc: the Gateway (`s6-5-gateway`). No other prompt or dialog appeared.

## Step 7: the reads after the look
`s7`: M and R-0010. `--same-row` s5/s7 equal. `--diff` s5/s7 printed the two status flags in the Result. The line compares are in `evidence/m-r-s5-s7-compare.txt`.

## Step 8: the restore (C6)
Before the shut: BK re-hashed, equal to step 1's, its folder holding only that file; the restore plan written (17:00:49Z); the data folder listed (24 entries). Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. The Data menu (by its label), Backup & Restore, Restore, Specify Path to the BK's folder: the list showed BRIDGE PILOT LAB, its number and 9-Nov-26 (10:19 pm), the BK's time. The company line, End of List. Two prompts, the only ones answered:
- "Accept ?" (`s8-restore-accept`, 17:02:10Z): y.
- "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (`s8-restore-replace`, 17:02:28Z): r.

Select Company came back with no other message; the data folder then held 17 entries, each written at 22:32:36 IST. Loaded alone (the name typed, one row, Enter, no prompt; `s8-gateway`). `s8`: as in the Result; the sha256 of each answer against s2's is in `evidence/s2-s8-compare.txt`. EXPECT met.

## Step 9: close (C10)
Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). At 17:05:17Z, one `tally.exe`, which alone owns 9001, and `GET /status` 200. No XML request was sent after the shut. Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company): no prompt appeared, and at 17:05:40Z no `tally.exe` ran and nothing listened on 9001.

## Deviations and notes
- **Go To** was opened by clicking its "G: Go To" label, not by Alt+G as the sheet says. Menus were opened by clicking their labels throughout (K: Company, Y: Data, G: Go To), as the lab's standing rule asks.
- **Step 4.1's path:** Go To did not take `GST Registration`. It kept `GSTRe` and showed a Spelling Error box. The row stopped, and on the run lead's word reached the registration by Go To, `Alter Master`, then the `GST Registration` line of Master Alteration. Esc was pressed once, on the Error box only. The Go To field drops spaces as they are typed.
- **Step 4.4's Backspaces:** on `GSTIN/UIN` with its value highlighted, one Backspace moved the cursor to the field before and changed nothing. The sheet's 15 Backspaces would have walked back through the fields, so they were not pressed. The new value was typed over the highlighted one and zoomed before any Enter. The run lead agreed to this afterwards.
- **Step 4.6:** Ctrl+A brought no "Accept ?" prompt, so none was answered.
- **Step 6's dialog** is not one the sheet names. The row stopped there; the key pressed (`c`) was the run lead's. The other choice, "A: All Periods", was not pressed.
- **The sitting's place:** a Tally start of its own in the gap before row 2360-L's sitting 2, on the run lead's word; the install's start-up company was shut by C9 before anything else.
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.
- **GSTR-1's Return View** had more lines below those shown, at step 3 and at step 6. They were not scrolled to, as the sheet asks only for the summary and the uncertain list.

## Not measured
Every item the sheet lists as not measured. Also: what "A: All Periods" would have done; whether the status moved because of the dialog's answer or because of the look itself; which 32 vouchers the uncertain line counts; what "Marked for Deletion on Portal 4" counts; the Return View's lines below those shown; and what "(2)" beside B2C (Small)'s count of 27 counts.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`, with the reason).
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count, the `<the before-write option>`, `--same-row`, `--unit-diff` and `--diff` outputs, the byte and line compares, the send times, the screens written down and the BK record. The process, port and data-folder listings and the restore plan stay local.
- `scripts/`: the step 0 helper, the window capture helper and the send helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## count13.py outputs, whole

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
== a-s2-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
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

### s4m
```
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s5
```
== a-s5-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
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
== a-s5-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '27ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '27ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
== a-s5-W-2026-08.xml: 115168 bytes
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

### s7
```
== a-s7-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7-R-0010.xml: 94476 bytes
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
```

### s8
```
== a-s8-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
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
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s8-U-taxunits.xml: 22636 bytes
  STATUS '1'
  TAXUNIT rows: 2
   NAME 'Default Tax Unit'; TAXTYPE ''; TAXREGISTRATION None ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000064' ; the book's: yes
    GSTREGNUMBER ''; GSTOLDREGNUMBER ''; USEDFOR ''
   NAME 'Rajasthan Registration'; TAXTYPE 'GST'; TAXREGISTRATION '08ZZZZZ0000Z1ZQ' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000cd' ; the book's: yes
    GSTREGNUMBER '08ZZZZZ0000Z1ZQ'; GSTOLDREGNUMBER ''; USEDFOR 'GST'
    dated: FROMDATE '20260401'; STATE 'Rajasthan'; PLACEOFSUPPLY 'Rajasthan'; REGISTRATIONTYPE 'Regular'; ISINACTIVE 'No'
== a-s8-W-2026-08.xml: 115168 bytes
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

## The <the before-write option>, --same-row, --unit-diff and --diff outputs, whole

### s4m before-write against s1
```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s5 unit-diff against s2
```
== --unit-diff a-s2-U-taxunits.xml a-s5-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 3 leaves differ
     /ALTERID[1]: '206' -> '246'
     /GSTREGNUMBER[1]: '08ZZZZZ0000Z1ZQ' -> '27ZZZZZ0000Z1ZQ'
     @TAXREGISTRATION: '08ZZZZZ0000Z1ZQ' -> '27ZZZZZ0000Z1ZQ'
```

### s5 R-0010 diff against s2
```
== --diff a-s2-R-0010.xml a-s5-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### s7 same-row against s5
```
== a-s7-M-marks.xml against a-s5-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'")
  equal
```

### s7 R-0010 diff against s5
```
== --diff a-s5-R-0010.xml a-s7-R-0010.xml
   VCHGSTSTATUSISINCLUDED: 'Yes' -> 'No'
   VCHGSTSTATUSISUNCERTAIN: 'No' -> 'Yes'
   lists compared: 1
   legs: 6 -> 6
```

### s8 same-row against s1
```
== --same-row a-s1-M-marks.xml a-s8-M-marks.xml
== a-s8-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### s8 unit-diff against s2
```
== --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml
== --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### s8 R-0010 diff against s2
```
== --diff a-s2-R-0010.xml a-s8-R-0010.xml
== --diff a-s2-R-0010.xml a-s8-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### s8 against s2, byte compares
```
M-marks s2 vs s8: equal; 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 / 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
U-taxunits s2 vs s8: equal; e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 / e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
R-0010 s2 vs s8: equal; b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 / b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
W-2026-08 s2 vs s8: equal; f67cf46f57e7f4ae65d864f7fc73ed3b20db7027d7fbf048f1730cc691387e97 / f67cf46f57e7f4ae65d864f7fc73ed3b20db7027d7fbf048f1730cc691387e97
```
