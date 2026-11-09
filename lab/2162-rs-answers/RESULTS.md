# Row 2162-RS: an unchanged re-save on Tally's screen keeps the invoice's GUID and MASTERID, moves its ALTERID by one, and fills two leg fields the read-back fetches

Run on 9 Nov 2026, 14:52Z to 15:04Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-rs-sheet` (073144e5), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic). Raw requests through `run.py` and one save made by hand on Tally's screen; no Bridge build. Every value below is M, measured by this run, in one run: **partial**. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

**Result.**
- **The write (step 3):** `I-0057` answered CREATED 1, LASTVCHID 45, every other counter 0, no LINEERROR. `s3r`: R one row, stored as sent (GUID `...-0000002d`, ALTERID 69, included Yes, uncertain No, excluded No, 6 legs); N one row of Sales class; V one row (MASTERID 45, ALTERID 69); ALTVCHID 68 -> 69.
- **The control (step 4):** `s4`, with no key pressed since `s3r`, is byte-equal to `s3r` in R, V and M. Both diffs printed no field and no list; legs 6 -> 6.
- **The save (step 5):** Day Book, 1-Aug-26, the `BP/26-27/0057` line, Enter: "Accounting Voucher Alteration (Secondary)", BRIDGE Sales, No. BP/26-27/0057. Nothing changed; Ctrl+A once. **No prompt appeared**: no "Accept ?" and no other message. Tally went straight back to the Day Book.
- **After the save (step 6), s4 against s6:**
  - GUID `...-0000002d` and MASTERID 45: the same. VOUCHERNUMBER, DATE, REFERENCE, party, state, place of supply, registration type and the three GST status flags (included Yes, uncertain No, excluded No): the same.
  - **ALTERID 69 -> 70** in R and V. **ALTVCHID 69 -> 70** (by one); ALTMSTID 245 -> 245.
  - **R, the ledger entries:** `list ALLLEDGERENTRIES.LIST: 6 -> 6, 5 differ`. The leg fields `count10.py` compares (ledger name, deemed positive, amount) are unchanged on all six; legs 6 -> 6. What differs, by the line diff: `VATEXPAMOUNT`, empty on s4, holds each leg's own amount on s6 on the five non-party legs (Svc 998313 600.40, Svc 998314 400.40, CGST 25.02, SGST 25.02, Round Off 0.16); and a `RATEOFINVOICETAX.LIST` with `RATEOFINVOICETAX` 2.50 is added on the CGST 2.5% and SGST 2.5% legs. The party leg is unchanged.
  - **V, a field R does not fetch:** `REMOTEALTGUID`, empty on s4, holds the book's GUID on s6. V's line diff shows no `VATEXPAMOUNT` or `RATEOFINVOICETAX` change (V's answer holds neither tag).
  - Both answers' `DESC/CMPINFO` counts moved (LEDGER 11 -> 33, CURRENCY 1 -> 2, TAXUNIT 1 -> 3, VOUCHERNUMBERSERIES 1 -> 4, VOUCHER 2 -> 4). These are object counts in the answer's header, not voucher fields.
  - No second voucher numbered `BP/26-27/0057`: N on s6 has one row.
- **s6 against s6b, 60 s apart with no key pressed:** byte-equal in R, V and M; both diffs printed no field and no list.
- **The restore:** `s9` M byte-equal to `s1` (68 / 245, `--same-row` equal); N-0057 0 rows, byte-equal to `s2`'s.

**By the sheet's Decides** (the code reading is the write-up's, not this run's):
- GUID and MASTERID the same, ALTERID changed: measured.
- A V difference R does not fetch: measured, one field, `REMOTEALTGUID` ('' -> the book's GUID).
- In R, two fields inside the ledger entries changed (`VATEXPAMOUNT` on five legs, `RATEOFINVOICETAX.LIST` added on two). Whether `invoice_readback_differences` compares either is for the write-up to read from the code; the leg fields `count10.py` compares did not change.
- ALTVCHID moved by one; ALTMSTID did not move.
- s6 against s6b: no difference.
- No prompt other than "Accept ?" appeared at the save; "Accept ?" itself did not appear.

## Step 1: load and check
- Sheet files: all 17 equal to `SHA256.txt` at 073144e5; `LAB-COMMON.md` byte-equal to row 2433 N3's. `gen8.py`, re-run in a separate folder, rebuilt `I-0057`, `R-0057`, `N-0057` and `V-0057` byte for byte (`evidence/gen-check.txt`).
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID. Two Bridge server processes were running; they were recorded and not ended (a standing rule of the lab; no Bridge tool was called).
- Loaded alone from Select Company (one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026 (after 1-Aug-2026), date of last entry 15-Sep-26.
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5): the Data menu (by its label), Backup & Restore, Backup, Specify Path to a new empty folder. BRIDGE PILOT LAB alone, End of List; "Accept ?" captured and answered y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `948ba6b3d05d293658ca6f6a86db98ce9faa49bcae44e7e482779d0101bf0ad8`, 20:24 local. `s1bk` marks: byte-equal to `s1`.

## Step 2: the book as found
`s2`: N-0057, R-0057 and V-0057 0 rows; R-0010 one row. Every EXPECT met.

## Step 3: the invoice
`s3m` 68 / 245, the before-write check passed. `s3`: `I-0057` sent once, answered as above; the capture after it (`s3-after-write`) shows the Gateway with no dialog. `s3r` met every EXPECT.

## Step 4: the reads again
`s4`: R, V and M, no key pressed in Tally since `s3r`; each answer byte-equal to `s3r`'s. EXPECT met.

## Step 5: the save, on Tally's screen
No request was sent during this step. The prompts: none.
1. The Gateway (`s5-1-gateway`). Go To opened by clicking the "G: Go To" label (the sheet says Alt+G; see Deviations). `Day Book` typed (the field shows `DayBook`); the list offered Day Book under Common Reports and under Voucher Reports, and Enter took the first. The Day Book opened For 15-Sep-26, one line (`s5-1-daybook`).
2. Alt+F2 opened "Change Period" (From 15-9-2026, To 15-9-2026). From `1-8-2026`, Enter, To `1-8-2026`, Enter. F2 was not pressed. The Day Book For 1-Aug-26 (`s5-2-daybook-1aug`), 12 lines, each 1-Aug-26, BRIDGE Walk-in, BRIDGE Sales, by Vch No. and Debit Amount: BP/26-27/0001 1,050.00; 0002 1,051.05; 0003 1,051.04; 0004 1,051.05; 0005 1,051.05; 0006 1,050.02; 0007 1,050.00; 0008 1,051.05; 0009 1,456.00; 0011 1,051.06; 0010 1,051.00; 0057 1,051.00 (`evidence/s5-daybook-lines.txt`).
3. Down 11 times, one line each: `BP/26-27/0057` highlighted (`s5-3-0057-highlighted`).
4. Enter: "Accounting Voucher Alteration (Secondary)", BRIDGE PILOT LAB; voucher type BRIDGE Sales; No. BP/26-27/0057; 1-Aug-26, Saturday; Party A/c name BRIDGE Walk-in, Current balance 18,199.70 Dr; Particulars: BRIDGE Svc 998313 5% 600.40; BRIDGE Svc 998314 5% 400.40; BRIDGE CGST 2.5% rate 2.50 % 25.02; BRIDGE SGST 2.5% rate 2.50 % 25.02; BRIDGE Round Off 0.16; Narration empty; total 1,051.00 (`s5-4-alteration`; `evidence/s5-alteration-fields.txt`).
5. Nothing changed. Ctrl+A once. No prompt and no message appeared.
6. The Day Book For 1-Aug-26, the same 12 lines, 0057 highlighted (`s5-6-after-ctrl-a`, byte-equal to `s5-3-0057-highlighted`).
7. Esc once on the Day Book: the Gateway (`s5-7-gateway`, byte-equal to `s5-1-gateway`).

## Step 6: the reads after the save
`s6` at 14:59:37Z; `s6b` at 15:00:38Z, with no key pressed between. The outputs are below; `evidence/s6-changes.txt` names each change and `evidence/rawdiff-s4-s6.txt` is the line diff of R and V, s4 against s6.

## Step 7: the restore (C6)
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt.
- Restore under the Data menu (by its label), Backup & Restore, Restore. The Restore Data Path was Tally's data folder; the source was Specify Path, the BK folder. The list showed one row: BRIDGE PILOT LAB, THE COMPANY's number, 9-Nov-26 (8:24 pm), the BK's time. It was selected alone, then End of List. The prompts, word for word:
  - "Accept ?" captured (`s7-restore-accept`) and answered y.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." captured (`s7-restore-replace`) and answered r (Replace).
  - No other prompt. Tally went to Select Company.
- The restored `Company.1800` holds BRIDGE PILOT LAB and its GUID. Loaded alone (the name typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, current date Tuesday 15-Sep-2026 (`s7-gateway-after-restore`).
- `s9`: M 68 / 245, `--same-row` against s1 equal, byte-equal to s1; N-0057 0 rows, byte-equal to s2's. PASS.

## Step 8: close (C10)
Shut Company by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-window`, byte-equal to step 0's capture). One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. PASS.

## Deviations and notes
- **Go To** was opened by clicking its "G: Go To" label, not by Alt+G as the sheet says. Menus were opened by clicking their labels throughout (K: Company, Y: Data, G: Go To), as the lab's standing rule asks.
- **C6, before the restore:** `restore-plan.txt` was not written before the restore and no listing of THE COMPANY's data folder was kept before it. The BK was not re-hashed before it. After the restore, the BK re-hashed equal to its record and its folder holds that one file; the Restore list showed the BK's date and time; `restore-plan.txt` was written after the restore and says so (kept local). The data folder held 17 entries after the restore and 20 after the close (listing kept local).
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.
- **Labels.** Each label was used once. `s1bk` is the marks read after the BK that C5 asks for; it is not in the sheet.

## Not measured
Every item the sheet lists as not measured; whether the build's verification reads this invoice as divergent after the save; why Tally showed no "Accept ?" at this save.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`).
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count, check and diff output, the send times, the Day Book lines, the alteration screen's fields, the list of changes and the BK record. The process and port listings, the folder listing and `restore-plan.txt` stay local.
- `scripts/`: the step 0 helper and the window capture helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## count10.py outputs, whole

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
== a-s2-N-0057.xml: 3022 bytes
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
== a-s2-R-0057.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-V-0057.xml: 3022 bytes
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
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s3
```
== a-s3-I-0057.xml: 540 bytes
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
== a-s3r-N-0057.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0057' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s3r-R-0057.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0057'
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
== a-s3r-V-0057.xml: 52788 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
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
```

### s4
```
== a-s4-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4-R-0057.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0057'
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
== a-s4-V-0057.xml: 52788 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
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
```

### s6
```
== a-s6-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6-N-0057.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0057' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s6-R-0057.xml: 95182 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0057'
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
== a-s6-V-0057.xml: 52890 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '70'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
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
```

### s6b
```
== a-s6b-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6b-R-0057.xml: 95182 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0057'
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
== a-s6b-V-0057.xml: 52890 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '70'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0057'
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
```

### s9
```
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s9-N-0057.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

## The six diffs, whole

### diff-s4
```
== --diff a-s3r-R-0057.xml a-s4-R-0057.xml
   lists compared: 1
   legs: 6 -> 6
== --diff a-s3r-V-0057.xml a-s4-V-0057.xml
   lists compared: 2
   legs: 0 -> 0
== a-s4-M-marks.xml against a-s3r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'245'")
  equal
```

### diff-s4-s6
```
== --diff a-s4-R-0057.xml a-s6-R-0057.xml
   ALTERID: '69' -> '70'
   list ALLLEDGERENTRIES.LIST: 6 -> 6, 5 differ
   lists compared: 1
   legs: 6 -> 6
== --diff a-s4-V-0057.xml a-s6-V-0057.xml
   ALTERID: '69' -> '70'
   REMOTEALTGUID: '' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
   lists compared: 2
   legs: 0 -> 0
== a-s6-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
before-write exit 0
```

### diff-s6-s6b
```
== --diff a-s6-R-0057.xml a-s6b-R-0057.xml
   lists compared: 1
   legs: 6 -> 6
== --diff a-s6-V-0057.xml a-s6b-V-0057.xml
   lists compared: 2
   legs: 0 -> 0
```
