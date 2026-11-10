# Row 2464-GR: stopped before step 3, by the sheet's own stop; the number `BP/26-27/0041` is already held by a cancelled voucher; nothing changed, nothing written

Run on 10 Nov 2026, 00:02:34Z to 00:08:44Z (05:32 to 05:38 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the program's file, as the Gateway shows no release number), by the sheet at `lab/2464-gr-sheet` (feb88d30), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw reads only, one at a time through `run.py`; no Bridge build. **Nothing was keyed after the backup and nothing was written**: no master change, no import. Every value below is M, measured by this run, in one run. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran after row 683-TW, on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end. The marks it leaves are those it found (ALTVCHID 68, ALTMSTID 245).

The GSTIN in this folder is the synthetic family's, as the sheet names it: the company's own `08ZZZZZ0000Z1ZQ`.

**Result: stopped at step 2, before step 3.** The sheet's step 2 reads: "EXPECT N-0041 0 rows: no voucher holds the number. One row or more: stop before step 3." `N-0041` answered one row:

- `BP/26-27/0041`, VOUCHERTYPENAME `BRIDGE Sales`, DATE 20260820, MASTERID 40, ISCANCELLED `Yes`, ISDELETED `No`, BRIDGEVCHISSALES `Yes`, GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028`.

It is not a voucher this sitting made: the same cancelled voucher (the same number, date, type and status) is read in the results of rows 2162-B2C, 2162-KC, 2162-SC and 2162-S5. The book held it before this row began. So the row's import, which uses that number, was not sent, and the sheet's question (which registration an import that names none is given) is not answered here. The run lead's word after the stop: read MARKS once; if it is byte-equal to s1, close by C10 without a restore, as nothing was keyed after BK; keep BK on the lab PC; a revised sheet, with a number checked as unused, follows as its own item.

- **s1:** one COMPANY row, the book's: ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **BK:** taken, as below. It is kept on the lab PC and was not restored.
- **s2, the book before** (every other EXPECT met):
  - **U:** byte-equal to row 2162-RU's s2 answer (22,636 B, sha256 `e40ef83c...`): `Default Tax Unit`, and `Rajasthan Registration`, the one GST unit, `08ZZZZZ0000Z1ZQ`, one dated row (20260401, Rajasthan, Rajasthan, Regular, not inactive).
  - **T, `BRIDGE Sales`:** no `DEFAULTGSTREGISTRATION` element (empty, as the sheet expects); `LASTGSTTAXUNIT` `Rajasthan Registration`; `LASTGSTTAXUNITBYSERIES` `Rajasthan Registration`; `TAXUNITNAME` `Any`; `ALLOWMULTIPLETAXUNITSPERSERIES` `No`.
  - **N-0041:** one row, as above: the stop.
  - **R-0010:** one row, `BRIDGE Walk-in`, included `Yes`, uncertain and excluded `No`, 6 legs.
  - **V-0010:** one VOUCHER: `GSTREGISTRATION` `Rajasthan Registration` (no attributes), `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, `CMPGSTSTATE` `Rajasthan`, `CMPGSTREGISTRATIONTYPE` `Regular`, `PLACEOFSUPPLY` `Rajasthan`, `STATENAME` `Rajasthan`.
  - **M:** ALTVCHID 68, ALTMSTID 245.
- **m2, after the stop (the run lead's word):** MARKS byte-equal to s1 (sha256 `8a0c4268...`). So the book is as BK holds it, and no restore was made.

**By the sheet's Decides:** none applies; the row stopped before its first change. What this run does measure: the book's state before step 3 (s2) is as the sheet expected in every read but N-0041, and `BP/26-27/0041` is not an unused number in this book.

## Step 1: load, check and back up
- Sheet files: all 18 equal to `SHA256.txt` at feb88d30. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` and the two shared templates are byte-equal to row 2162-RP's; `U-taxunits.xml` and `T-types.xml` to row 2162-GY's; `template-full-0057` to row 2162-RS's `V-0057`. `count16.py` differs from `count15.py` only by the lines for the six registration fields.
- Before the start, THE COMPANY's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`).
- Tally was started once, at 00:02:34Z, from its desktop shortcut, after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 at 00:03:38Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded.
- The book was loaded alone: `BRIDGE PILOT` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): the book alone, SILVER on the title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; date of last entry 15-Sep-26.
- `s1`: as in the Result.
- **BK** (C5), by row 2162-GY's step 1 path:
  - Before the backup, `C:\lab\2464-gr\backup-1` was new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. The cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times), then `C:\lab\2464-gr\backup-1` typed, and its row taken.
  - BRIDGE PILOT LAB alone, End of List (Up 3 times). "Accept ?" was captured (`bk-accept`, 00:05:54Z) and answered y; no other message; the Gateway again.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `8cf8030428523f79f99a42fce21cb76bf0cb607e4e662f2a582a2095945bea7c`, written at 05:35 IST.

## Step 2: the book before, and the stop
`s2`: `U-taxunits.xml T-types.xml N-0041.xml R-0010.xml V-0010.xml M-marks.xml`, each request checked against `SHA256.txt` just before its send. N-0041 answered one row: the sheet's stop before step 3. The Gateway was then captured (`stop-gateway`, 00:07:21Z): the book loaded, no dialog. Nothing more was sent or keyed until the run lead's word; then `m2` (`M-marks.xml`) once, byte-equal to s1.

## Steps 3 to 6: not run
No master was created or altered, no import was sent, and no restore was made. `I-0041.xml`, `R-0041.xml` and `V-0041.xml` were never sent. `prep-c6.sh` was made for step 6 but not run, as no restore was needed.

## Step 7: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 00:08:26Z).
- C2 at 00:08:26Z, after the last request: one `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; no `tally.exe` ran after.

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists any that stay on the lab PC, by size and sha256).
- `evidence/`: notes, the outputs of `count16.py`, the start-up check. Process and port listings stay on the lab PC.
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
== a-s2-N-0041.xml: 5596 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0041' DATE '20260820' BRIDGEVCHISSALES 'Yes'
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

### m2
```
== a-m2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

## Checks, whole

### M, m2 against s1 (--same-row)
```
== a-m2-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### sha256 of every answer
```
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6  a-m2-M-marks.xml
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6  a-s1-M-marks.xml
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6  a-s2-M-marks.xml
983b07c79ef066bb6f6becea2be4254a6f776c99d5d4d20b5c27a46048155cf4  a-s2-N-0041.xml
b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5  a-s2-R-0010.xml
cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed  a-s2-T-types.xml
e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989  a-s2-U-taxunits.xml
4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456  a-s2-V-0010.xml
```
