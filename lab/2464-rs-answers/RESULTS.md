# Row 2464-RS: on the book with two GST registrations and `BRIDGE Sales` bound to `Rajasthan Registration`, one save of the imported, uncertain invoice on Tally's screen, with no change, filled both sales lines' GST and HSN/SAC details and moved its status to included

Run on 10 Nov 2026, 07:25:33Z to 07:46Z (12:55 to 13:16 IST). TallyPrime Silver: the Gateway on 9001; `tally.exe` file version 1.1.7.2, as rows 2464-GN, 2464-NR and 2464-RB read it from the program's file; the Gateway shows no release number. The sheet is `lab/2464-rs-sheet` (29b06f41). The book is `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own.

What ran:
- Raw reads only, one at a time through `run.py`; no Bridge build.
- Row 2464-RB's two master changes on Tally's screen.
- RB's import A'' (`I-0105.xml`), sent once under the sheet's named C3 exception, on the run lead's yes.
- One save of that invoice on Tally's screen with no change, by row 2162-RS's path.
- The restore of this sitting's own backup.

Every read was answered, each label's reads within 2 s by the `run.py` logs. The window after the import was the Gateway, no dialog. The Gateway read 1-Apr-26 to 31-Mar-27 and Tuesday, 15-Sep-2026, as RB's did.

The GSTINs in this folder are the synthetic family's, as the sheet names them: the company's own `08ZZZZZ0000Z1ZQ` and the second registration's `27ZZZZZ0000Z1ZQ`.

This sitting ran after row 2346-TD (Tally quit at 07:12:46Z), with a Tally start of its own.

## Result

PARTIAL by the sheet's Decides: one save, one invoice, one sitting, one book, one release.

**Before the save, the invoice was RB's A'' (steps 1 to 5).** Every answer the sheet compares with RB's is byte-equal to it:
- s1, all seven of s2's, s3's U and T, s4's T and U, s5's import answer, and all six of s5r's.
- The `a-rb-` copies they were compared with each match the sheet's size and sha256 (`evidence/rb-copies.txt`).

So:
- s2's N-0105 read 0 rows, the condition of the C3 exception.
- Step 3 raised RB's three named prompts, answered `c`, `y` and `n`; ALTMSTID 246.
- Step 4 raised no prompt; ALTMSTID 248.
- The import read `CREATED` 1, `LASTVCHID` 45, `ERRORS` 0, `EXCEPTIONS` 0, no LINEERROR (540 B, byte-equal to RB's).
- s5r: N-0105 one row; R-0105 included `No`, uncertain `Yes`, excluded `No`, `ISGSTOVERRIDDEN` `No`, ALTERID 69, MASTERID 45.
- `legs.py` on R-0105 and on RC-0105: every leg's seven GST and HSN/SAC fields empty and 0 rate rows.
- The control s5b, sent 78.8 s after s5r with no key pressed: byte-equal to s5r in all four answers.

**The save (step 6).**
- The path was row 2162-RS's: Go To, Day Book, Alt+F2 to 1-8-2026. The Day Book for 1-Aug-26 showed 12 lines: 0001 to 0009, 0011, 0010, and 0105 (1,051.00) where 2162-RS's 0057 was.
- Down 11 times highlighted 0105, and Enter opened `Accounting Voucher Alteration (Secondary)`: BRIDGE Sales, No. BP/26-27/0105, 1-Aug-26 Saturday, BRIDGE Walk-in, the five lines and total 1,051.00, as the sheet expects.
- The screen showed one line row 2162-RS's did not: `GST Registration : Rajasthan Registration`, above the voucher type. The F3 button read `Company/Tax Registration` where 2162-RS's read `Company`. Both are recorded, not touched (`evidence/s6-alteration-fields.txt`).
- Nothing was changed. Ctrl+A once: no prompt and no message. Tally returned to the Day Book with the same 12 lines; Esc once there gave the Gateway.

**After the save (s6), the status:**

| field | R-0105 at s5b | R-0105 at s6 |
|---|---|---|
| `VCHGSTSTATUSISINCLUDED` | `No` | `Yes` |
| `VCHGSTSTATUSISUNCERTAIN` | `Yes` | `No` |
| `VCHGSTSTATUSISEXCLUDED` | `No` | `No` |
| `ISGSTOVERRIDDEN` | `No` | `No` |
| `ALTERID` | 69 | 70 |
| `MASTERID` | 45 | 45 |

The GUID tail `0000002d` is kept. N-0105 holds one row; there is no second voucher numbered 0105. The marks are ALTVCHID 70 (s5b: 69) and ALTMSTID 248.

**After the save, the lines** (`legs.py --diff` s5b against s6, the same on R and on RC):
- Leg 0, `BRIDGE Walk-in`: EQUAL.
- Legs 1 and 2, `BRIDGE Svc 998313 5%` and `BRIDGE Svc 998314 5%`, 57 lines each:
  - `GSTSOURCETYPE` `Ledger`; `GSTLEDGERSOURCE` the leg's ledger;
  - `HSNSOURCETYPE` `Ledger`; `HSNLEDGERSOURCE` the leg's ledger;
  - `GSTHSNNAME` `998313` and `998314`;
  - `GSTOVRDNTAXABILITY` `Taxable`; `GSTOVRDNTYPEOFSUPPLY` `Services`;
  - rate rows 0 -> 5; `VATEXPAMOUNT` 600.40 and 400.40.
- Legs 3 and 4, CGST and SGST: a `RATEOFINVOICETAX` list, 2.50, added; `VATEXPAMOUNT` 25.02 each.
- Leg 5, Round Off: `VATEXPAMOUNT` 0.16.

Against `BP/26-27/0010`'s legs (s2's R-0010, `legs.py --diff`):
- Leg 0 is EQUAL.
- Every other leg differs only in `VATEXPAMOUNT` (empty on 0010, filled on 0105) and, on the two tax legs, in the `RATEOFINVOICETAX` list 0105 now holds.
- So the two sales lines' GST and HSN/SAC fields and five rate rows are 0010's.
- The two fields that differ are the ones row 2162-RS's save filled on the one-registration book.

**After the save, the registration fields** (`count19.py --diff` s5b against s6; V is Tally's native read of one voucher):

| field | V-0105 at s5b | V-0105 at s6 | V-0010 (s2) |
|---|---|---|---|
| `GSTREGISTRATION` | `Rajasthan Registration` | `Rajasthan Registration` | `Rajasthan Registration` |
| `CMPGSTIN` | `''` | `08ZZZZZ0000Z1ZQ` | `08ZZZZZ0000Z1ZQ` |
| `CMPGSTSTATE` | `''` | `Rajasthan` | `Rajasthan` |
| `CMPGSTREGISTRATIONTYPE` | `''` | `Regular` | `Regular` |
| `PLACEOFSUPPLY` | `Rajasthan` | `Rajasthan` | `Rajasthan` |
| `STATENAME` | `Rajasthan` | `Rajasthan` | `Rajasthan` |

- V-0105's diff also names `REMOTEALTGUID` `''` -> the book's GUID, `ALTERID` and the two status flags; nothing else.
- RC-0105's diff names `ALTERID`, the two status flags, `CMPGSTIN` `''` -> `08ZZZZZ0000Z1ZQ` and its ledger entries (6 -> 6, 5 differ). RC-0105 still equals V-0105 on `GSTREGISTRATION` (`TYPE="String"`) and `CMPGSTIN`.
- V-0010 did not move: `--diff` s2 against s6 finds no field and no list that differs.
- The control s6b, sent 76.5 s after s6 with no key pressed: byte-equal to s6 in all four answers.

By the sheet's Decides, quoted: "After the save, R-0105 reads included `Yes`, uncertain `No`, excluded `No`, and both sales lines hold their GST and HSN/SAC fields and rate rows as `BP/26-27/0010`'s do: on this book a save on screen, with no change, fills the lines and clears the status. The status and the lines move together again, and the import alone leaves both. It does not say which causes which."

On the registration fields, the Decides asks to record what the save filled: `CMPGSTIN`, `CMPGSTSTATE` and `CMPGSTREGISTRATIONTYPE`, each now 0010's value. `GSTREGISTRATION` is unchanged. RC-0105 still equals V-0105 on the two fields it fetches.

**The sheet's estimates:**
- s2 to s5r: byte-equal to RB's, as estimated.
- The save, as estimated:
  - no prompt;
  - GUID and MASTERID kept;
  - ALTERID 69 to 70 and ALTVCHID up by one;
  - `VATEXPAMOUNT` and `RATEOFINVOICETAX` filled.
- The status and the sales lines were not estimated.
- s8: equal to s2 and s1, as estimated.

**The way back (step 8).**
- C6 ran with only the two expected prompts.
- s8:
  - every answer's sha256 equals s2's (M, U, T, N-0105, R-0010, RC-0010, V-0010);
  - M equals s1 by `--same-row` (68/245);
  - two units, 0 leaves differ in each; `BRIDGE Sales` 0 leaves differ from s2;
  - N-0105 0 rows; R-0010 and RC-0010 show `lists compared` and `legs: 6 -> 6` only.
- The book is as it was before step 3.
- C10 closed it, and Tally quit with no prompt.

**Recorded, not judged:**
- The data folder held 25 entries before the restore (row 2464-RB's: 24) and 17 after.
- The entries gone: `CmpSchMetaData.1800`, `Index.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF` and `TUPDATE.TSF`.
- The listings stay on the lab PC.

**Deviations:**
- The three Bridge processes were left running, on the run lead's standing word; none was called.
- The start-up company was shut first (C9).
- Go To was opened by clicking its label (the standing menu rule), as row 2162-RS's 5.1.
- Two send times in `evidence/notes.txt` were first written from a clock line; a later note line corrects them from `evidence/sends-utc.txt`.

## Files

- `answers/`: every answer of this row, byte for byte, except those `kept-local.txt` lists. They are listed by size and sha256 in `evidence/answers-sha.txt`.
- `evidence/`: the console outputs, compares and notes. `notes.txt` records every key, prompt and screen, with UTC times.
- `logs/`: the `run.py` logs.
- `scripts/`: this row's helpers. `step0.py`, `shot.sh`, `send.sh` and `prep-c6.sh` are made from row 2464-RB's with this row's folder; `rbcopies.py` made the `a-rb-` copies; `fields-rs.py` printed `evidence/fields-s5r.txt` and `evidence/fields-s6.txt`.
- `kept-local.txt`, `SCREENSHOTS.txt`, `substitutions.txt` and `SHA256.txt`.
- The `a-rb-` copies of row 2464-RB's answers are not pushed again; `kept-local.txt` lists them.

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

### s5b
```
== a-s5b-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5b-R-0105.xml: 87968 bytes
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
== a-s5b-RC-0105.xml: 88208 bytes
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
== a-s5b-V-0105.xml: 52630 bytes
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

### s6
```
== a-s6-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6-N-0105.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0105' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s6-R-0105.xml: 95182 bytes
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
   ALTERID '70'
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
== a-s6-RC-0105.xml: 95452 bytes
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
   ALTERID '70'
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
== a-s6-V-0010.xml: 52786 bytes
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
== a-s6-V-0105.xml: 52890 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '70'
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
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
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

### s6b
```
== a-s6b-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6b-R-0105.xml: 95182 bytes
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
   ALTERID '70'
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
== a-s6b-RC-0105.xml: 95452 bytes
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
   ALTERID '70'
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
== a-s6b-V-0105.xml: 52890 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '70'
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
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
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

### step 1: the a-rb copies of row 2464-RB's answers (size and sha256 against the sheet's table)
```
a-rb-a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 from committed: equal to the sheet
a-rb-a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 from committed: equal to the sheet
a-rb-a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed from committed: equal to the sheet
a-rb-a-s2-N-0105.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 from committed: equal to the sheet
a-rb-a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 from committed: equal to the sheet
a-rb-a-s2-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989 from lab PC: equal to the sheet
a-rb-a-s2-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399 from lab PC: equal to the sheet
a-rb-a-s2-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456 from lab PC: equal to the sheet
a-rb-a-s3-T-types.xml 306946 B 3239e41dfed99766ac1ee6da5724eea72ca206f55113d729bb0c998d56402425 from committed: equal to the sheet
a-rb-a-s3-U-taxunits.xml 33304 B 2ec479c6d234d5d5d130a362268c042adf35a05a6993d6c09ad71071519196f1 from lab PC: equal to the sheet
a-rb-a-s4-T-types.xml 307038 B eef7b1cc48eecd1fbc654b94379015dd87820579331d0b8cbf3222ffe2cf5987 from committed: equal to the sheet
a-rb-a-s4-U-taxunits.xml 33304 B 508dd8de3eb34f011a48debf7e86154657b129efc570d4ccd6a6ec5d96143621 from lab PC: equal to the sheet
a-rb-a-s5-I-0105.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea from committed: equal to the sheet
a-rb-a-s5r-N-0105.xml 5606 B 23929638cd0c6b653435a33a9903f8e5e15b2efb481d0a5d7f71fb5a4792a2ff from committed: equal to the sheet
a-rb-a-s5r-R-0105.xml 87968 B 56e3fff1e5c6b0d7594e7c863bb6e795f66eff7ae0e7e49dfe3df7e26146d075 from committed: equal to the sheet
a-rb-a-s5r-RC-0105.xml 88208 B 5e4b0537c33d4d87ed53e37cee5162db1db7ccbe6eea41cf30d0f2c89a4f3265 from committed: equal to the sheet
a-rb-a-s5r-V-0105.xml 52630 B 81ede927c72c679406a5197ce6a8b41a199b8ecf479fd49affb2cbe1b5d8e94a from lab PC: equal to the sheet
a-rb-a-s5r-V-0010.xml 52784 B 456ddfbe553487a466c405e963e4f094e50507a8fa4f2665c0be7a7a30893a0e from lab PC: equal to the sheet
a-rb-a-s5r-M-marks.xml 3394 B eaa698ccbf94cb0145d6f3c656c6672f9bfa0631020026d175bc33a1df9d3aad from committed: equal to the sheet
copies 19, equal 19, differ 0
```

### step 1: the book's data folder and the excluded GUID
```
BRIDGE PILOT LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
```

### s1: M against RB's s1 (--same-row)
```
== a-s1-M-marks.xml against a-rb-a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### s2: each answer against RB's s2 (byte compare)
```
s2 U-taxunits byte-equal to RB's s2
s2 T-types byte-equal to RB's s2
s2 N-0105 byte-equal to RB's s2
s2 R-0010 byte-equal to RB's s2
s2 RC-0010 byte-equal to RB's s2
s2 V-0010 byte-equal to RB's s2
s2 M-marks byte-equal to RB's s2
```

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s3: U against RB's s3 (--unit-diff)
```
== --unit-diff a-rb-a-s3-U-taxunits.xml a-s3-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### s3: T against RB's s3 (--type-diff)
```
== --type-diff a-rb-a-s3-T-types.xml a-s3-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### s4: U against RB's s4 (--unit-diff)
```
== --unit-diff a-rb-a-s4-U-taxunits.xml a-s4-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Maharashtra Registration', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Maharashtra Registration': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
```

### s4: T against RB's s4 (--type-diff)
```
== --type-diff a-rb-a-s4-T-types.xml a-s4-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
```

### s4: T against s3 (--type-diff; a call beyond the sheet's)
```
== --type-diff a-s3-T-types.xml a-s4-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 2 leaves differ
     /ALTERID[1]: '212' -> '248'
     /DEFAULTGSTREGISTRATION[1]: '' -> 'Rajasthan Registration'
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s5r: each answer against RB's s5r (byte compare)
```
s5r N-0105 byte-equal to RB's s5r
s5r R-0105 byte-equal to RB's s5r
s5r RC-0105 byte-equal to RB's s5r
s5r V-0105 byte-equal to RB's s5r
s5r V-0010 byte-equal to RB's s5r
s5r M-marks byte-equal to RB's s5r
```

### s5r: the status, registration and id fields (fields-rs.py)
```
a-s5r-R-0105.xml: VCHGSTSTATUSISINCLUDED 'No' (attr TYPE="Logical"); VCHGSTSTATUSISUNCERTAIN 'Yes' (attr TYPE="Logical"); VCHGSTSTATUSISEXCLUDED 'No' (attr TYPE="Logical"); ISGSTOVERRIDDEN 'No' (attr TYPE="Logical"); ALTERID ' 69' (attr TYPE="Number"); MASTERID ' 45' (attr TYPE="Number")
a-s5r-RC-0105.xml: GSTREGISTRATION 'Rajasthan Registration' (attr TYPE="String"); CMPGSTIN '' (attr TYPE="String")
a-s5r-V-0105.xml: GSTREGISTRATION 'Rajasthan Registration'; CMPGSTIN empty element; CMPGSTSTATE empty element; CMPGSTREGISTRATIONTYPE empty element; PLACEOFSUPPLY 'Rajasthan'; STATENAME 'Rajasthan'; ALTERID ' 69'; MASTERID ' 45'
a-s5r-M-marks.xml: ALTVCHID ' 69' (attr TYPE="Number"); ALTMSTID ' 248' (attr TYPE="Number")
```

### s5b: R, RC and V against s5r (--diff); M against s5r (--same-row)
```
== --diff a-s5r-R-0105.xml a-s5b-R-0105.xml
   lists compared: 1
   legs: 6 -> 6
diff R-0105 exit 0
== --diff a-s5r-RC-0105.xml a-s5b-RC-0105.xml
   lists compared: 1
   legs: 6 -> 6
diff RC-0105 exit 0
== --diff a-s5r-V-0105.xml a-s5b-V-0105.xml
   lists compared: 2
   legs: 0 -> 0
diff V-0105 exit 0
== a-s5b-M-marks.xml against a-s5r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'248'")
  equal
same-row M exit 0
```

### step 6: the Day Book lines, as text
```
Day Book, BRIDGE PILOT LAB, For 1-Aug-26 (Alt+F2 Change Period: From 1-8-2026, To 1-8-2026), as listed top to bottom:
Date | Particulars | Vch Type | Vch No. | Debit Amount
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0001 | 1,050.00
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0002 | 1,051.05
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0003 | 1,051.04
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0004 | 1,051.05
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0005 | 1,051.05
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0006 | 1,050.02
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0007 | 1,050.00
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0008 | 1,051.05
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0009 | 1,456.00
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0011 | 1,051.06
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0010 | 1,051.00
1-Aug-26 | BRIDGE Walk-in | BRIDGE Sales | BP/26-27/0105 | 1,051.00
12 lines; Credit Amount column empty on every line; row 2162-RS's twelve with BP/26-27/0105 where its BP/26-27/0057 was. Before the period change the Day Book opened For 15-Sep-26 with one line (BRIDGE Walk-in, BRIDGE Sales, BP/26-27/0020, 210.20).
```

### step 6: the voucher screen before Ctrl+A, as text
```
Voucher screen opened by Enter on the Day Book line BP/26-27/0105, as shown before Ctrl+A (nothing changed):
Title bar: Accounting Voucher Alteration (Secondary) | BRIDGE PILOT LAB
GST Registration: Rajasthan Registration (a line above the voucher type, centred; row 2162-RS's screen on the one-registration book had no such line; recorded, not touched)
Voucher type: BRIDGE Sales
No.: BP/26-27/0105 (cursor on the No. field)
Date: 1-Aug-26, Saturday
Party A/c name: BRIDGE Walk-in
Current balance: 18,199.70 Dr
Particulars | Rate | per | Amount
BRIDGE Svc 998313 5% | | | 600.40
BRIDGE Svc 998314 5% | | | 400.40
BRIDGE CGST 2.5% | 2.50 | % | 25.02
BRIDGE SGST 2.5% | 2.50 | % | 25.02
BRIDGE Round Off | | | 0.16
Narration: (empty)
Total: 1,051.00
No tax, HSN/SAC or other GST line is shown on the screen beyond the GST Registration line above.
Button bar: Q: Quit, A: Accept, D: Delete, X: Cancel Vch, F12: Configure; right panel F2: Date, F3: Company/Tax Registration (row 2162-RS's read F3: Company), F4: Contra, F5: Payment, F6: Receipt, F7: Journal, F8: Sales, F9: Purchase, F10: Other Vouchers, H: Change Mode, I: More Details, O: Related Reports, L: Optional, T: Post-Dated (F: Autofill greyed)
```

### s6: send.sh console
```
N-0105.xml: 5608 bytes
R-0105.xml: 95182 bytes
RC-0105.xml: 95452 bytes
V-0105.xml: 52890 bytes
V-0010.xml: 52786 bytes
M-marks.xml: 3396 bytes
== a-s6-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6-N-0105.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0105' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s6-R-0105.xml: 95182 bytes
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
   ALTERID '70'
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
== a-s6-RC-0105.xml: 95452 bytes
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
   ALTERID '70'
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
== a-s6-V-0010.xml: 52786 bytes
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
== a-s6-V-0105.xml: 52890 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '70'
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
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
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
count19 exit 0
```

### s6: the status, registration and id fields (fields-rs.py)
```
a-s6-R-0105.xml: VCHGSTSTATUSISINCLUDED 'Yes' (attr TYPE="Logical"); VCHGSTSTATUSISUNCERTAIN 'No' (attr TYPE="Logical"); VCHGSTSTATUSISEXCLUDED 'No' (attr TYPE="Logical"); ISGSTOVERRIDDEN 'No' (attr TYPE="Logical"); ALTERID ' 70' (attr TYPE="Number"); MASTERID ' 45' (attr TYPE="Number")
a-s6-RC-0105.xml: GSTREGISTRATION 'Rajasthan Registration' (attr TYPE="String"); CMPGSTIN '08ZZZZZ0000Z1ZQ' (attr TYPE="String")
a-s6-V-0105.xml: GSTREGISTRATION 'Rajasthan Registration'; CMPGSTIN '08ZZZZZ0000Z1ZQ'; CMPGSTSTATE 'Rajasthan'; CMPGSTREGISTRATIONTYPE 'Regular'; PLACEOFSUPPLY 'Rajasthan'; STATENAME 'Rajasthan'; ALTERID ' 70'; MASTERID ' 45'
a-s6-M-marks.xml: ALTVCHID ' 70' (attr TYPE="Number"); ALTMSTID ' 248' (attr TYPE="Number")
```

### s6: R, RC and V against s5b; V-0010 against s2 (--diff)
```
== --diff a-s5b-R-0105.xml a-s6-R-0105.xml
   ALTERID: '69' -> '70'
   VCHGSTSTATUSISINCLUDED: 'No' -> 'Yes'
   VCHGSTSTATUSISUNCERTAIN: 'Yes' -> 'No'
   list ALLLEDGERENTRIES.LIST: 6 -> 6, 5 differ
   lists compared: 1
   legs: 6 -> 6
diff R-0105 exit 0
== --diff a-s5b-RC-0105.xml a-s6-RC-0105.xml
   ALTERID: '69' -> '70'
   CMPGSTIN: '' -> '08ZZZZZ0000Z1ZQ'
   VCHGSTSTATUSISINCLUDED: 'No' -> 'Yes'
   VCHGSTSTATUSISUNCERTAIN: 'Yes' -> 'No'
   list ALLLEDGERENTRIES.LIST: 6 -> 6, 5 differ
   lists compared: 1
   legs: 6 -> 6
diff RC-0105 exit 0
== --diff a-s5b-V-0105.xml a-s6-V-0105.xml
   ALTERID: '69' -> '70'
   CMPGSTIN: '' -> '08ZZZZZ0000Z1ZQ'
   CMPGSTREGISTRATIONTYPE: '' -> 'Regular'
   CMPGSTSTATE: '' -> 'Rajasthan'
   REMOTEALTGUID: '' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
   VCHGSTSTATUSISINCLUDED: 'No' -> 'Yes'
   VCHGSTSTATUSISUNCERTAIN: 'Yes' -> 'No'
   lists compared: 2
   legs: 0 -> 0
diff V-0105 exit 0
== --diff a-s2-V-0010.xml a-s6-V-0010.xml
   lists compared: 2
   legs: 0 -> 0
diff V-0010 s2-s6 exit 0
```

### s6: R-0105 legs against s5b (legs.py --diff)
```
a-s5b-R-0105.xml: 87968 B sha256 56e3fff1e5c6b0d7594e7c863bb6e795f66eff7ae0e7e49dfe3df7e26146d075, 6 legs
a-s6-R-0105.xml: 95182 B sha256 76b29528a7e1704c16153d10b04b5bf1536c4ef9362cba11fd6f297304946e16, 6 legs
a-s5b-R-0105.xml:
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
a-s6-R-0105.xml:
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998313 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998313 5%', GSTHSNNAME '998313', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998314 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998314 5%', GSTHSNNAME '998314', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
leg 0: EQUAL
leg 1: DIFFER, 57 line(s)
  @@ -51,2 +51,2 @@
  -      <GSTOVRDNISREVCHARGEAPPL/>
  -      <GSTOVRDNTAXABILITY/>
  +      <GSTOVRDNISREVCHARGEAPPL>&#4; Not Applicable</GSTOVRDNISREVCHARGEAPPL>
  +      <GSTOVRDNTAXABILITY>Taxable</GSTOVRDNTAXABILITY>
  @@ -54 +54 @@
  -      <GSTSOURCETYPE/>
  +      <GSTSOURCETYPE>Ledger</GSTSOURCETYPE>
  @@ -56 +56 @@
  -      <GSTLEDGERSOURCE/>
  +      <GSTLEDGERSOURCE>BRIDGE Svc 998313 5%</GSTLEDGERSOURCE>
  @@ -59 +59 @@
  -      <HSNSOURCETYPE/>
  +      <HSNSOURCETYPE>Ledger</HSNSOURCETYPE>
  @@ -61 +61 @@
  -      <HSNLEDGERSOURCE/>
  +      <HSNLEDGERSOURCE>BRIDGE Svc 998313 5%</HSNLEDGERSOURCE>
  @@ -64,3 +64,3 @@
  -      <GSTOVRDNSTOREDNATURE/>
  -      <GSTOVRDNTYPEOFSUPPLY/>
  -      <GSTRATEINFERAPPLICABILITY/>
  +      <GSTOVRDNSTOREDNATURE>&#4; System Inferred</GSTOVRDNSTOREDNATURE>
  +      <GSTOVRDNTYPEOFSUPPLY>Services</GSTOVRDNTYPEOFSUPPLY>
  +      <GSTRATEINFERAPPLICABILITY>As per Masters/Company</GSTRATEINFERAPPLICABILITY>
  @@ -68,3 +68,3 @@
  -      <GSTHSNNAME/>
  -      <GSTHSNDESCRIPTION/>
  -      <GSTHSNINFERAPPLICABILITY/>
  +      <GSTHSNNAME>998313</GSTHSNNAME>
  +      <GSTHSNDESCRIPTION>Information technology consulting and support services</GSTHSNDESCRIPTION>
  +      <GSTHSNINFERAPPLICABILITY>As per Masters/Company</GSTHSNINFERAPPLICABILITY>
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>600.40</VATEXPAMOUNT>
  @@ -179 +179,30 @@
  -      <RATEDETAILS.LIST>      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>SGST/UTGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>IGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 5</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>&#4; Not Applicable</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>State Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
leg 2: DIFFER, 57 line(s)
  @@ -51,2 +51,2 @@
  -      <GSTOVRDNISREVCHARGEAPPL/>
  -      <GSTOVRDNTAXABILITY/>
  +      <GSTOVRDNISREVCHARGEAPPL>&#4; Not Applicable</GSTOVRDNISREVCHARGEAPPL>
  +      <GSTOVRDNTAXABILITY>Taxable</GSTOVRDNTAXABILITY>
  @@ -54 +54 @@
  -      <GSTSOURCETYPE/>
  +      <GSTSOURCETYPE>Ledger</GSTSOURCETYPE>
  @@ -56 +56 @@
  -      <GSTLEDGERSOURCE/>
  +      <GSTLEDGERSOURCE>BRIDGE Svc 998314 5%</GSTLEDGERSOURCE>
  @@ -59 +59 @@
  -      <HSNSOURCETYPE/>
  +      <HSNSOURCETYPE>Ledger</HSNSOURCETYPE>
  @@ -61 +61 @@
  -      <HSNLEDGERSOURCE/>
  +      <HSNLEDGERSOURCE>BRIDGE Svc 998314 5%</HSNLEDGERSOURCE>
  @@ -64,3 +64,3 @@
  -      <GSTOVRDNSTOREDNATURE/>
  -      <GSTOVRDNTYPEOFSUPPLY/>
  -      <GSTRATEINFERAPPLICABILITY/>
  +      <GSTOVRDNSTOREDNATURE>&#4; System Inferred</GSTOVRDNSTOREDNATURE>
  +      <GSTOVRDNTYPEOFSUPPLY>Services</GSTOVRDNTYPEOFSUPPLY>
  +      <GSTRATEINFERAPPLICABILITY>As per Masters/Company</GSTRATEINFERAPPLICABILITY>
  @@ -68,3 +68,3 @@
  -      <GSTHSNNAME/>
  -      <GSTHSNDESCRIPTION/>
  -      <GSTHSNINFERAPPLICABILITY/>
  +      <GSTHSNNAME>998314</GSTHSNNAME>
  +      <GSTHSNDESCRIPTION>Information technology design and development services</GSTHSNDESCRIPTION>
  +      <GSTHSNINFERAPPLICABILITY>As per Masters/Company</GSTHSNINFERAPPLICABILITY>
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>400.40</VATEXPAMOUNT>
  @@ -179 +179,30 @@
  -      <RATEDETAILS.LIST>      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>SGST/UTGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>IGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 5</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>&#4; Not Applicable</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>State Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
leg 3: DIFFER, 5 line(s)
  @@ -4,0 +5,3 @@
  +      <RATEOFINVOICETAX.LIST TYPE="Number">
  +       <RATEOFINVOICETAX> 2.50</RATEOFINVOICETAX>
  +      </RATEOFINVOICETAX.LIST>
  @@ -157 +160 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>25.02</VATEXPAMOUNT>
leg 4: DIFFER, 5 line(s)
  @@ -4,0 +5,3 @@
  +      <RATEOFINVOICETAX.LIST TYPE="Number">
  +       <RATEOFINVOICETAX> 2.50</RATEOFINVOICETAX>
  +      </RATEOFINVOICETAX.LIST>
  @@ -157 +160 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>25.02</VATEXPAMOUNT>
leg 5: DIFFER, 2 line(s)
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>0.16</VATEXPAMOUNT>
```

### s6: RC-0105 legs against s5b (legs.py --diff)
```
a-s5b-RC-0105.xml: 88208 B sha256 5e4b0537c33d4d87ed53e37cee5162db1db7ccbe6eea41cf30d0f2c89a4f3265, 6 legs
a-s6-RC-0105.xml: 95452 B sha256 f3aeace6efb67af946e318d3fe64ac21483939311539d56598b0d0d4a0c901e4, 6 legs
a-s5b-RC-0105.xml:
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
a-s6-RC-0105.xml:
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998313 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998313 5%', GSTHSNNAME '998313', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998314 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998314 5%', GSTHSNNAME '998314', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
leg 0: EQUAL
leg 1: DIFFER, 57 line(s)
  @@ -51,2 +51,2 @@
  -      <GSTOVRDNISREVCHARGEAPPL/>
  -      <GSTOVRDNTAXABILITY/>
  +      <GSTOVRDNISREVCHARGEAPPL>&#4; Not Applicable</GSTOVRDNISREVCHARGEAPPL>
  +      <GSTOVRDNTAXABILITY>Taxable</GSTOVRDNTAXABILITY>
  @@ -54 +54 @@
  -      <GSTSOURCETYPE/>
  +      <GSTSOURCETYPE>Ledger</GSTSOURCETYPE>
  @@ -56 +56 @@
  -      <GSTLEDGERSOURCE/>
  +      <GSTLEDGERSOURCE>BRIDGE Svc 998313 5%</GSTLEDGERSOURCE>
  @@ -59 +59 @@
  -      <HSNSOURCETYPE/>
  +      <HSNSOURCETYPE>Ledger</HSNSOURCETYPE>
  @@ -61 +61 @@
  -      <HSNLEDGERSOURCE/>
  +      <HSNLEDGERSOURCE>BRIDGE Svc 998313 5%</HSNLEDGERSOURCE>
  @@ -64,3 +64,3 @@
  -      <GSTOVRDNSTOREDNATURE/>
  -      <GSTOVRDNTYPEOFSUPPLY/>
  -      <GSTRATEINFERAPPLICABILITY/>
  +      <GSTOVRDNSTOREDNATURE>&#4; System Inferred</GSTOVRDNSTOREDNATURE>
  +      <GSTOVRDNTYPEOFSUPPLY>Services</GSTOVRDNTYPEOFSUPPLY>
  +      <GSTRATEINFERAPPLICABILITY>As per Masters/Company</GSTRATEINFERAPPLICABILITY>
  @@ -68,3 +68,3 @@
  -      <GSTHSNNAME/>
  -      <GSTHSNDESCRIPTION/>
  -      <GSTHSNINFERAPPLICABILITY/>
  +      <GSTHSNNAME>998313</GSTHSNNAME>
  +      <GSTHSNDESCRIPTION>Information technology consulting and support services</GSTHSNDESCRIPTION>
  +      <GSTHSNINFERAPPLICABILITY>As per Masters/Company</GSTHSNINFERAPPLICABILITY>
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>600.40</VATEXPAMOUNT>
  @@ -179 +179,30 @@
  -      <RATEDETAILS.LIST>      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>SGST/UTGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>IGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 5</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>&#4; Not Applicable</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>State Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
leg 2: DIFFER, 57 line(s)
  @@ -51,2 +51,2 @@
  -      <GSTOVRDNISREVCHARGEAPPL/>
  -      <GSTOVRDNTAXABILITY/>
  +      <GSTOVRDNISREVCHARGEAPPL>&#4; Not Applicable</GSTOVRDNISREVCHARGEAPPL>
  +      <GSTOVRDNTAXABILITY>Taxable</GSTOVRDNTAXABILITY>
  @@ -54 +54 @@
  -      <GSTSOURCETYPE/>
  +      <GSTSOURCETYPE>Ledger</GSTSOURCETYPE>
  @@ -56 +56 @@
  -      <GSTLEDGERSOURCE/>
  +      <GSTLEDGERSOURCE>BRIDGE Svc 998314 5%</GSTLEDGERSOURCE>
  @@ -59 +59 @@
  -      <HSNSOURCETYPE/>
  +      <HSNSOURCETYPE>Ledger</HSNSOURCETYPE>
  @@ -61 +61 @@
  -      <HSNLEDGERSOURCE/>
  +      <HSNLEDGERSOURCE>BRIDGE Svc 998314 5%</HSNLEDGERSOURCE>
  @@ -64,3 +64,3 @@
  -      <GSTOVRDNSTOREDNATURE/>
  -      <GSTOVRDNTYPEOFSUPPLY/>
  -      <GSTRATEINFERAPPLICABILITY/>
  +      <GSTOVRDNSTOREDNATURE>&#4; System Inferred</GSTOVRDNSTOREDNATURE>
  +      <GSTOVRDNTYPEOFSUPPLY>Services</GSTOVRDNTYPEOFSUPPLY>
  +      <GSTRATEINFERAPPLICABILITY>As per Masters/Company</GSTRATEINFERAPPLICABILITY>
  @@ -68,3 +68,3 @@
  -      <GSTHSNNAME/>
  -      <GSTHSNDESCRIPTION/>
  -      <GSTHSNINFERAPPLICABILITY/>
  +      <GSTHSNNAME>998314</GSTHSNNAME>
  +      <GSTHSNDESCRIPTION>Information technology design and development services</GSTHSNDESCRIPTION>
  +      <GSTHSNINFERAPPLICABILITY>As per Masters/Company</GSTHSNINFERAPPLICABILITY>
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>400.40</VATEXPAMOUNT>
  @@ -179 +179,30 @@
  -      <RATEDETAILS.LIST>      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>SGST/UTGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 2.50</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>IGST</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE> 5</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>&#4; Not Applicable</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
  +      <RATEDETAILS.LIST>
  +       <GSTRATEDUTYHEAD>State Cess</GSTRATEDUTYHEAD>
  +       <GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>
  +       <GSTRATE>0</GSTRATE>
  +       <GSTRATEPERUNIT>0</GSTRATEPERUNIT>
  +      </RATEDETAILS.LIST>
leg 3: DIFFER, 5 line(s)
  @@ -4,0 +5,3 @@
  +      <RATEOFINVOICETAX.LIST TYPE="Number">
  +       <RATEOFINVOICETAX> 2.50</RATEOFINVOICETAX>
  +      </RATEOFINVOICETAX.LIST>
  @@ -157 +160 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>25.02</VATEXPAMOUNT>
leg 4: DIFFER, 5 line(s)
  @@ -4,0 +5,3 @@
  +      <RATEOFINVOICETAX.LIST TYPE="Number">
  +       <RATEOFINVOICETAX> 2.50</RATEOFINVOICETAX>
  +      </RATEOFINVOICETAX.LIST>
  @@ -157 +160 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>25.02</VATEXPAMOUNT>
leg 5: DIFFER, 2 line(s)
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>0.16</VATEXPAMOUNT>
```

### s6: R-0105 legs against s2's R-0010 (legs.py --diff)
```
a-s2-R-0010.xml: 94474 B sha256 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5, 6 legs
a-s6-R-0105.xml: 95182 B sha256 76b29528a7e1704c16153d10b04b5bf1536c4ef9362cba11fd6f297304946e16, 6 legs
a-s2-R-0010.xml:
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998313 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998313 5%', GSTHSNNAME '998313', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998314 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998314 5%', GSTHSNNAME '998314', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
a-s6-R-0105.xml:
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998313 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998313 5%', GSTHSNNAME '998313', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998314 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998314 5%', GSTHSNNAME '998314', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
leg 0: EQUAL
leg 1: DIFFER, 2 line(s)
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>600.40</VATEXPAMOUNT>
leg 2: DIFFER, 2 line(s)
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>400.40</VATEXPAMOUNT>
leg 3: DIFFER, 5 line(s)
  @@ -4,0 +5,3 @@
  +      <RATEOFINVOICETAX.LIST TYPE="Number">
  +       <RATEOFINVOICETAX> 2.50</RATEOFINVOICETAX>
  +      </RATEOFINVOICETAX.LIST>
  @@ -157 +160 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>25.02</VATEXPAMOUNT>
leg 4: DIFFER, 5 line(s)
  @@ -4,0 +5,3 @@
  +      <RATEOFINVOICETAX.LIST TYPE="Number">
  +       <RATEOFINVOICETAX> 2.50</RATEOFINVOICETAX>
  +      </RATEOFINVOICETAX.LIST>
  @@ -157 +160 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>25.02</VATEXPAMOUNT>
leg 5: DIFFER, 2 line(s)
  @@ -157 +157 @@
  -      <VATEXPAMOUNT/>
  +      <VATEXPAMOUNT>0.16</VATEXPAMOUNT>
```

### s6b: R, RC and V against s6 (--diff); M against s6 (--same-row)
```
== --diff a-s6-R-0105.xml a-s6b-R-0105.xml
   lists compared: 1
   legs: 6 -> 6
diff R-0105 exit 0
== --diff a-s6-RC-0105.xml a-s6b-RC-0105.xml
   lists compared: 1
   legs: 6 -> 6
diff RC-0105 exit 0
== --diff a-s6-V-0105.xml a-s6b-V-0105.xml
   lists compared: 2
   legs: 0 -> 0
diff V-0105 exit 0
== a-s6b-M-marks.xml against a-s6-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'")
  equal
same-row M exit 0
```

### s8: M against s1 (--same-row); U, T, R-0010 and RC-0010 against s2
```
== a-s8-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
same-row exit 0
== --unit-diff a-s2-U-taxunits.xml a-s8-U-taxunits.xml: TAXUNIT names ['Default Tax Unit', 'Rajasthan Registration'] -> ['Default Tax Unit', 'Rajasthan Registration']
   unit 'Default Tax Unit': 0 leaves differ
   unit 'Rajasthan Registration': 0 leaves differ
unit exit 0
== --type-diff a-s2-T-types.xml a-s8-T-types.xml: VOUCHERTYPE rows 28 -> 28
   type 'BRIDGE Sales': 0 leaves differ
type exit 0
== --diff a-s2-R-0010.xml a-s8-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
diff R exit 0
== --diff a-s2-RC-0010.xml a-s8-RC-0010.xml
   lists compared: 1
   legs: 6 -> 6
diff RC exit 0
```

### s8: each answer against s2 (sha256)
```
M-marks s8 sha256 equal to s2's (8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6)
U-taxunits s8 sha256 equal to s2's (e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989)
T-types s8 sha256 equal to s2's (cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed)
N-0105 s8 sha256 equal to s2's (947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974)
R-0010 s8 sha256 equal to s2's (b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5)
RC-0010 s8 sha256 equal to s2's (5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399)
V-0010 s8 sha256 equal to s2's (4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456)
```

### step 8: the book's data folder and the excluded GUID, after the restore
```
BRIDGE PILOT LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
```

### each send's start, UTC
```
2026-11-10T07:27:30.211Z s1 M-marks.xml (hashes equal to the lists)
2026-11-10T07:29:37.262Z s2 U-taxunits.xml T-types.xml N-0105.xml R-0010.xml RC-0010.xml V-0010.xml M-marks.xml (hashes equal to the lists)
2026-11-10T07:29:47.474Z s3m M-marks.xml (hashes equal to the lists)
2026-11-10T07:33:48.309Z s3 U-taxunits.xml T-types.xml M-marks.xml (hashes equal to the lists)
2026-11-10T07:34:06.599Z s4m M-marks.xml (hashes equal to the lists)
2026-11-10T07:36:21.864Z s4 T-types.xml U-taxunits.xml M-marks.xml (hashes equal to the lists)
2026-11-10T07:36:39.108Z s5m M-marks.xml (hashes equal to the lists)
2026-11-10T07:36:48.531Z s5 I-0105.xml (hashes equal to the lists)
2026-11-10T07:37:06.184Z s5r N-0105.xml R-0105.xml RC-0105.xml V-0105.xml V-0010.xml M-marks.xml (hashes equal to the lists)
2026-11-10T07:38:24.956Z s5b R-0105.xml RC-0105.xml V-0105.xml M-marks.xml (hashes equal to the lists)
2026-11-10T07:41:15.992Z s6 N-0105.xml R-0105.xml RC-0105.xml V-0105.xml V-0010.xml M-marks.xml (hashes equal to the lists)
2026-11-10T07:42:32.444Z s6b R-0105.xml RC-0105.xml V-0105.xml M-marks.xml (hashes equal to the lists)
2026-11-10T07:45:39.978Z s8 M-marks.xml U-taxunits.xml T-types.xml N-0105.xml R-0010.xml RC-0010.xml V-0010.xml (hashes equal to the lists)
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s2-N-0105.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
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
a-s5b-R-0105.xml 87968 B 56e3fff1e5c6b0d7594e7c863bb6e795f66eff7ae0e7e49dfe3df7e26146d075
a-s5b-RC-0105.xml 88208 B 5e4b0537c33d4d87ed53e37cee5162db1db7ccbe6eea41cf30d0f2c89a4f3265
a-s5b-V-0105.xml 52630 B 81ede927c72c679406a5197ce6a8b41a199b8ecf479fd49affb2cbe1b5d8e94a
a-s5b-M-marks.xml 3394 B eaa698ccbf94cb0145d6f3c656c6672f9bfa0631020026d175bc33a1df9d3aad
a-s6-N-0105.xml 5608 B ca1d9c835b626832f0ceded7b5fa368b7b6b5453d8ac26dc4dddce98461662fb
a-s6-R-0105.xml 95182 B 76b29528a7e1704c16153d10b04b5bf1536c4ef9362cba11fd6f297304946e16
a-s6-RC-0105.xml 95452 B f3aeace6efb67af946e318d3fe64ac21483939311539d56598b0d0d4a0c901e4
a-s6-V-0105.xml 52890 B b6d79fedb24c0a960006e91aac3f88713674394988f7d7546e137a8e9537788a
a-s6-V-0010.xml 52786 B 1a2126de7311cb600cc4cd4c08c52eedc1a6aafdb558f7c56165bd2cb90fc203
a-s6-M-marks.xml 3396 B 999bcac47bdab6c99f3b804d9764be1e045e7a64993fea047b58986a21dd25da
a-s6b-R-0105.xml 95182 B 76b29528a7e1704c16153d10b04b5bf1536c4ef9362cba11fd6f297304946e16
a-s6b-RC-0105.xml 95452 B f3aeace6efb67af946e318d3fe64ac21483939311539d56598b0d0d4a0c901e4
a-s6b-V-0105.xml 52890 B b6d79fedb24c0a960006e91aac3f88713674394988f7d7546e137a8e9537788a
a-s6b-M-marks.xml 3396 B 999bcac47bdab6c99f3b804d9764be1e045e7a64993fea047b58986a21dd25da
a-s8-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s8-U-taxunits.xml 22636 B e40ef83cc01ffc18ee38a48745dd712d2f0f89f282cd2fe1e716a7ff71835989
a-s8-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s8-N-0105.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s8-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s8-RC-0010.xml 94744 B 5805c4173cce5e6fb0be6cf2990575e81cbf9c39b6aa9a3616c10254e5574399
a-s8-V-0010.xml 52784 B 4bf8b111c33edc1378933a2583021ce677d8ae4641972f88b8dc6343cfc60456
```

## legs.py outputs: the summary lines (each output is whole in evidence/)

### s5r: legs.py on R-0105 (evidence/legs-s5r-R.txt, 1171 lines)
```
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
```

### s5r: legs.py on RC-0105 (evidence/legs-s5r-RC.txt, 1171 lines)
```
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
```

### s6: legs.py on R-0105 (evidence/legs-s6-R.txt, 1235 lines)
```
  leg 0 'BRIDGE Walk-in': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 1 'BRIDGE Svc 998313 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998313 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998313 5%', GSTHSNNAME '998313', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 2 'BRIDGE Svc 998314 5%': GSTSOURCETYPE 'Ledger', GSTLEDGERSOURCE 'BRIDGE Svc 998314 5%', HSNSOURCETYPE 'Ledger', HSNLEDGERSOURCE 'BRIDGE Svc 998314 5%', GSTHSNNAME '998314', GSTOVRDNTAXABILITY 'Taxable', GSTOVRDNTYPEOFSUPPLY 'Services', rate rows 5
  leg 3 'BRIDGE CGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 4 'BRIDGE SGST 2.5%': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
  leg 5 'BRIDGE Round Off': GSTSOURCETYPE '', GSTLEDGERSOURCE '', HSNSOURCETYPE '', HSNLEDGERSOURCE '', GSTHSNNAME '', GSTOVRDNTAXABILITY '', GSTOVRDNTYPEOFSUPPLY '', rate rows 0
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
12:57:30	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
12:59:37	U-taxunits.xml	a-s2-U-taxunits.xml	curl 0	22636 bytes
12:59:37	T-types.xml	a-s2-T-types.xml	curl 0	306946 bytes
12:59:37	N-0105.xml	a-s2-N-0105.xml	curl 0	3022 bytes
12:59:37	R-0010.xml	a-s2-R-0010.xml	curl 0	94474 bytes
12:59:37	RC-0010.xml	a-s2-RC-0010.xml	curl 0	94744 bytes
12:59:38	V-0010.xml	a-s2-V-0010.xml	curl 0	52784 bytes
12:59:38	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3m
```
12:59:47	M-marks.xml	a-s3m-M-marks.xml	curl 0	3394 bytes
```

### s3
```
13:03:48	U-taxunits.xml	a-s3-U-taxunits.xml	curl 0	33304 bytes
13:03:48	T-types.xml	a-s3-T-types.xml	curl 0	306946 bytes
13:03:48	M-marks.xml	a-s3-M-marks.xml	curl 0	3394 bytes
```

### s4m
```
13:04:06	M-marks.xml	a-s4m-M-marks.xml	curl 0	3394 bytes
```

### s4
```
13:06:22	T-types.xml	a-s4-T-types.xml	curl 0	307038 bytes
13:06:22	U-taxunits.xml	a-s4-U-taxunits.xml	curl 0	33304 bytes
13:06:22	M-marks.xml	a-s4-M-marks.xml	curl 0	3394 bytes
```

### s5m
```
13:06:39	M-marks.xml	a-s5m-M-marks.xml	curl 0	3394 bytes
```

### s5
```
13:06:48	I-0105.xml	a-s5-I-0105.xml	curl 0	540 bytes
```

### s5r
```
13:07:06	N-0105.xml	a-s5r-N-0105.xml	curl 0	5606 bytes
13:07:06	R-0105.xml	a-s5r-R-0105.xml	curl 0	87968 bytes
13:07:06	RC-0105.xml	a-s5r-RC-0105.xml	curl 0	88208 bytes
13:07:06	V-0105.xml	a-s5r-V-0105.xml	curl 0	52630 bytes
13:07:06	V-0010.xml	a-s5r-V-0010.xml	curl 0	52784 bytes
13:07:07	M-marks.xml	a-s5r-M-marks.xml	curl 0	3394 bytes
```

### s5b
```
13:08:25	R-0105.xml	a-s5b-R-0105.xml	curl 0	87968 bytes
13:08:25	RC-0105.xml	a-s5b-RC-0105.xml	curl 0	88208 bytes
13:08:25	V-0105.xml	a-s5b-V-0105.xml	curl 0	52630 bytes
13:08:25	M-marks.xml	a-s5b-M-marks.xml	curl 0	3394 bytes
```

### s6
```
13:11:16	N-0105.xml	a-s6-N-0105.xml	curl 0	5608 bytes
13:11:16	R-0105.xml	a-s6-R-0105.xml	curl 0	95182 bytes
13:11:16	RC-0105.xml	a-s6-RC-0105.xml	curl 0	95452 bytes
13:11:16	V-0105.xml	a-s6-V-0105.xml	curl 0	52890 bytes
13:11:17	V-0010.xml	a-s6-V-0010.xml	curl 0	52786 bytes
13:11:17	M-marks.xml	a-s6-M-marks.xml	curl 0	3396 bytes
```

### s6b
```
13:12:32	R-0105.xml	a-s6b-R-0105.xml	curl 0	95182 bytes
13:12:32	RC-0105.xml	a-s6b-RC-0105.xml	curl 0	95452 bytes
13:12:33	V-0105.xml	a-s6b-V-0105.xml	curl 0	52890 bytes
13:12:33	M-marks.xml	a-s6b-M-marks.xml	curl 0	3396 bytes
```

### s8
```
13:15:40	M-marks.xml	a-s8-M-marks.xml	curl 0	3394 bytes
13:15:40	U-taxunits.xml	a-s8-U-taxunits.xml	curl 0	22636 bytes
13:15:40	T-types.xml	a-s8-T-types.xml	curl 0	306946 bytes
13:15:40	N-0105.xml	a-s8-N-0105.xml	curl 0	3022 bytes
13:15:40	R-0010.xml	a-s8-R-0010.xml	curl 0	94474 bytes
13:15:40	RC-0010.xml	a-s8-RC-0010.xml	curl 0	94744 bytes
13:15:41	V-0010.xml	a-s8-V-0010.xml	curl 0	52784 bytes
```
