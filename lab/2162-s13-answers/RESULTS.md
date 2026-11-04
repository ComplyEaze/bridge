# 2162-S13: a Sales invoice to a registered customer that is not bill-wise, and one with its round off on the debit side: results

**First line: PARTIAL, both sub-rows stored and read back; sub-row a with its GST status uncertain.** One book (BRIDGE PILOT LAB, synthetic), one release (TallyPrime 7.2 Silver), raw imports and not a build.
- **Sub-row a** (`BP/26-27/0053`, `BRIDGE Customer RJ`, GSTIN 08ZZZZZ0000Z1ZQ, not bill-wise): stored as sent, CREATED 1 and clean. `PARTYGSTIN` and `GSTREGISTRATIONTYPE` (Regular) came back as sent. But the read-back's GST status reads **included No, uncertain Yes**, where the control `BP/26-27/0010` reads included Yes, uncertain No. This is the sheet's Decides case "a with the flags uncertain": the post would end every bill to a registered customer in a stop. Item 18's first element set stays owed. Whether the cause is the seeded ledger or the invoice is the next row's question. (verified)
- **Sub-row b** (`BP/26-27/0054`, `BRIDGE Walk-in`, round off debit 0.84): stored as sent, CREATED 1 and clean. The read-back differs from the control only in the identity fields, the party's amount (-1050.00) and the round-off leg (`ISDEEMEDPOSITIVE` Yes, -0.84). The flags are included Yes, the rest No. This is the Decides case "b as expected": the second element set of item 18 is stored as sent and reads back clean. PARTIAL, as above. (verified)
- The restore put the book back: the marks are 68/245 with the book's GUID, byte-equal to step 1's answer, and the catalogue is byte-equal to step 2's. (verified)

Sheet `lab/2162-s13-sheet` at c4744e43 (SHA256.txt 21 of 21 equal, by sha256 and size). LAB-COMMON v3 (5370c6d2…). The book's GUID is 6b43e498-430c-4d5c-bfef-d32e2ab93c85. Each step and its time is in `notes.txt` (UTC). Each request file was checked against SHA256.txt just before its send. count.py's whole output for every answer is at the foot of this file, and in `evidence/count-*.txt`.

## One defect in the sheet, and the route taken

count.py has no kind for L answers. It takes the kind from the first letter of the request's name, and knows M, C, Q, N, I and R. So step 3's answer could not be counted as written. The row stopped before step 3's send and reported. On the recorded word:
- s3m was read again (as s3m2) with --before-write;
- L-seed.xml was sent once;
- its answer was copied byte for byte to `a-s3-I-seed.xml` (both sha256 1b72152a…, equal);
- count.py, unchanged, was run on the copy only. The original is kept as captured.

## Steps (EXPECT against what was read)

| step | EXPECT | read | |
|---|---|---|---|
| 1 load | BRIDGE PILOT LAB alone, no prompt; date not before 1-Aug-2026 | alone, no prompt; Tuesday, 15-Sep-2026; TallyPrime SILVER (7.2) | = |
| 1 s1 | one COMPANY row, the GUID, 68 / 245 | one row, the GUID, 68 / 245 | = |
| 1 BK | one file in a new empty folder | TDBK1800_&lt;company number&gt;.001, 90,051 B, sha256 c38086232e766e7a7701fee0d7070815cdacbcdb43ff744383be0a84e027ea01; marks after it 68 / 245 | = |
| 2 s2 | 0 rows named BRIDGE Customer RJ in C and Q; 0 rows in each N; R-0010 1 row | C 18 rows, 0 named; Q 18, 0 named; N-0053 0; N-0054 0; R-0010 1 row: BRIDGE Walk-in, Unregistered/Consumer, included Yes, every other flag No | = |
| 3 s3 | CREATED 1, every other counter 0 | CREATED 1; ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED, EXCEPTIONS 0; LASTVCHID 0; LASTMID 0; no LINEERROR (from the renamed copy) | = |
| 3 s3r | C: one row, Sundry Debtors, ISBILLWISEON No. Q: one row, one registration row: 20260401, Regular, Rajasthan, 08ZZZZZ0000Z1ZQ | C 19 rows: one, Sundry Debtors, No. Q 19 rows: one, Sundry Debtors, PARTYGSTIN and STATENAME absent, one LEDGSTREGDETAILS.LIST, non-empty, with exactly those four children; M 68 / 246 | = |
| 4 s4 | CREATED 1, every other counter 0 except LASTVCHID, no LINEERROR | CREATED 1; LASTVCHID 45; LASTMID 0; the rest 0; no LINEERROR | = |
| 4 s4r | R-0053 1 row; N-0053 1 row of Sales class | R-0053 1 row; N-0053 1 row, BRIDGEVCHISSALES Yes; M 69 / 246 | = |
| 4 diff | identity fields; party, PARTYGSTIN, Regular; leg 1's ledger; nothing else; the four status flags as R-0010's | all of those, **and VCHGSTSTATUSISINCLUDED Yes → No, VCHGSTSTATUSISUNCERTAIN No → Yes** | **differs (flags)** |
| 5 s5 | as step 4 | CREATED 1; LASTVCHID 46; LASTMID 0; the rest 0; no LINEERROR | = |
| 5 s5r | R-0054 1 row; N-0054 1 row of Sales class; R-0010 the same fields as s2's | R-0054 1 row; N-0054 1 row, BRIDGEVCHISSALES Yes; R-0010 every field equal to s2's; M 70 / 246 | = |
| 5 diff | identity fields; leg 1 -1050.00; leg 6 Round Off Yes -0.84; nothing else, the flags included | exactly those | = |
| 6 restore | only "Accept ?" (y) and "Company data folder (…) already exists" (r) | those two, nothing else; the list showed the BK alone, 4-Nov-26 (11:02 pm) | = |
| 6 s9 | --same-row passes (GUID, 68, 245); C's rows equal to s2's | passes; M byte-equal to s1's; C byte-equal to s2's (18 rows) | = |
| 7 close | one tally.exe owns 9001; no company loaded | tally.exe 1200 LISTENING on 9001; GET /status 200; Select Company, none loaded | = |

After every write, the whole Tally window showed the Gateway with no dialog. The three captures are byte-equal to each other. No prompt appeared at any step except the backup's and the restore's own, each named above. (verified)

## The two diffs, whole

The voucher keys are grouped here. The captured diff files hold them as digit runs, so those files stay on the lab PC (see below).

**s4r**, `count.py --diff a-s2-R-0010.xml a-s4r-R-0053.xml`:
```
   ALTERID: '17' -> '69'
   GSTREGISTRATIONTYPE: 'Unregistered/Consumer' -> 'Regular'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   PARTYGSTIN: '' -> '08ZZZZZ0000Z1ZQ'
   PARTYLEDGERNAME: 'BRIDGE Walk-in' -> 'BRIDGE Customer RJ'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0053'
   VCHGSTSTATUSISINCLUDED: 'Yes' -> 'No'
   VCHGSTSTATUSISUNCERTAIN: 'No' -> 'Yes'
   VOUCHERKEY: '198,573,517,963,352' -> '198,573,517,963,384'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0053'
   VOUCHERRETAINKEY: '41' -> '129'
   legs: 6 -> 6
   leg 1: ["'BRIDGE Walk-in'", "'Yes'", "'-1051.00'"] -> ["'BRIDGE Customer RJ'", "'Yes'", "'-1051.00'"]
```

**s5r**, `count.py --diff a-s2-R-0010.xml a-s5r-R-0054.xml`:
```
   ALTERID: '17' -> '70'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID: '11' -> '46'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0054'
   VOUCHERKEY: '198,573,517,963,352' -> '198,573,517,963,392'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0054'
   VOUCHERRETAINKEY: '41' -> '133'
   legs: 6 -> 6
   leg 1: ["'BRIDGE Walk-in'", "'Yes'", "'-1051.00'"] -> ["'BRIDGE Walk-in'", "'Yes'", "'-1050.00'"]
   leg 6: ["'BRIDGE Round Off'", "'No'", "'0.16'"] -> ["'BRIDGE Round Off'", "'Yes'", "'-0.84'"]
```

**R-0010 at s5r against s2**, `count.py --diff a-s2-R-0010.xml a-s5r-R-0010.xml`, prints only `legs: 6 -> 6`, so every field and leg is equal. The s5r answer is 2 B longer. Its text differs from s2's only in the CMPINFO block's bare count lines: LEDGER 0 → 23, CURRENCY 0 → 2, TAXUNIT 0 → 2, VOUCHERNUMBERSERIES 0 → 2, VOUCHER 0 → 4. These are not rows (LAB-COMMON C4). (verified)

## What each sub-row decides

- **a, flags uncertain.** The post's `gst_status_differences` refuses a read-back unless the status reads included and not overridden. Tally stored the registered customer's invoice with included No and uncertain Yes, so every such post would end in a stop. Nothing else in the read-back differs beyond the expected fields. Party, GSTIN, registration type, state, place of supply, every leg's ledger, sign and amount, overridden and post-dated are all as sent or as R-0010. Whether the post's field comparison passes on this answer was not run here: no build read it. Not measured here: why Tally holds it uncertain (the seeded ledger, or the invoice). That is the next row's question.
- **b, as expected.** Tally stores a round off on the debit side as the build renders it (`ISDEEMEDPOSITIVE` Yes, -0.84), the party closes at 1,050.00, and the status reads included, not overridden, not post-dated.

## Kept on the lab PC

Each kept file is listed in `kept-local.txt` by size and sha256, with its reason. No captured byte is altered. A captured file is pushed whole unless it holds the company number or a run of 10 or more digits. Two places are exempt: a run inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element.
- The BK.
- The company-folder listings.
- The two diff outputs (their voucher keys are digit runs outside a VOUCHERKEY element; they are given whole above, grouped).
- Any answer or file the stage script found holding such a value (named in `kept-local.txt`).

Screens stay here too, listed in `SCREENSHOTS.txt`.

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.

## count.py's output, whole

Each block is the file in `evidence/` of the same name, byte for byte as kept.

`evidence/count-s1.txt`
```
python -I count.py a-s1-M-marks.xml
exit 0
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
```

`evidence/count-s1bk.txt`
```
python -I count.py a-s1bk-M-marks.xml
exit 0
== a-s1bk-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
```

`evidence/count-s2.txt`
```
python -I count.py a-s2-C-catalogue.xml a-s2-Q-rates.xml a-s2-N-0053.xml a-s2-N-0054.xml a-s2-R-0010.xml
exit 0
== a-s2-C-catalogue.xml: 25010 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
== a-s2-Q-rates.xml: 82392 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
== a-s2-N-0053.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0054.xml: 3022 bytes
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
```

`evidence/count-s3m.txt`
```
python -I count.py a-s3m-M-marks.xml
exit 0
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
```

`evidence/s3m-before-write.txt`
```
2026-11-04T17:33:00Z python -I count.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml
exit 0
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

`evidence/count-s3m2.txt`
```
python -I count.py a-s3m2-M-marks.xml
exit 0
== a-s3m2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
```

`evidence/s3m2-before-write.txt`
```
2026-11-04T17:34:24Z python -I count.py --before-write a-s1-M-marks.xml a-s3m2-M-marks.xml
exit 0
== a-s3m2-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

`evidence/count-s3-I-copy.txt`
```
2026-11-04T17:34:35Z python -I count.py a-s3-I-seed.xml
exit 0
== a-s3-I-seed.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

`evidence/count-s3r.txt`
```
python -I count.py a-s3r-C-catalogue.xml a-s3r-Q-rates.xml a-s3r-M-marks.xml
exit 0
== a-s3r-C-catalogue.xml: 26224 bytes
  STATUS '1'
  LEDGER rows: 19; named 'BRIDGE Customer RJ': 1
   PARENT 'Sundry Debtors'; ISBILLWISEON 'No'
== a-s3r-Q-rates.xml: 85190 bytes
  STATUS '1'
  LEDGER rows: 19; named 'BRIDGE Customer RJ': 1
   PARENT 'Sundry Debtors'; PARTYGSTIN <absent>; STATENAME <absent>
   LEDGSTREGDETAILS.LIST: APPLICABLEFROM '20260401'; GSTREGISTRATIONTYPE 'Regular'; PLACEOFSUPPLY 'Rajasthan'; GSTIN '08ZZZZZ0000Z1ZQ'
   LEDGSTREGDETAILS.LIST elements: 1 of which non-empty: 1
== a-s3r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  COMPANY rows: 1
```

`evidence/count-s4m.txt`
```
python -I count.py a-s4m-M-marks.xml
exit 0
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  COMPANY rows: 1
```

`evidence/s4m-before-write.txt`
```
2026-11-04T17:34:49Z python -I count.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml
exit 0
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

`evidence/count-s4.txt`
```
python -I count.py a-s4-I-0053.xml
exit 0
== a-s4-I-0053.xml: 540 bytes
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

`evidence/count-s4r.txt`
```
python -I count.py a-s4r-R-0053.xml a-s4r-N-0053.xml a-s4r-M-marks.xml
exit 0
== a-s4r-R-0053.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0053'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0053'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08ZZZZZ0000Z1ZQ'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
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
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s4r-N-0053.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0053' BRIDGEVCHISSALES 'Yes'
== a-s4r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  COMPANY rows: 1
```

`evidence/count-s5m.txt`
```
python -I count.py a-s5m-M-marks.xml
exit 0
== a-s5m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  COMPANY rows: 1
```

`evidence/s5m-before-write.txt`
```
2026-11-04T17:35:52Z python -I count.py --before-write a-s1-M-marks.xml a-s5m-M-marks.xml
exit 0
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

`evidence/count-s5.txt`
```
python -I count.py a-s5-I-0054.xml
exit 0
== a-s5-I-0054.xml: 540 bytes
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

`evidence/count-s5r.txt`
```
python -I count.py a-s5r-R-0054.xml a-s5r-N-0054.xml a-s5r-R-0010.xml a-s5r-M-marks.xml
exit 0
== a-s5r-R-0054.xml: 94484 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0054'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0054'
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
   leg 'BRIDGE Walk-in' | 'Yes' | '-1050.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.84'
== a-s5r-N-0054.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0054' BRIDGEVCHISSALES 'Yes'
== a-s5r-R-0010.xml: 94476 bytes
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
== a-s5r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '246'
  COMPANY rows: 1
```

`evidence/s5r-r0010-diff.txt`
```
2026-11-04T17:36:00Z python -I count.py --diff a-s2-R-0010.xml a-s5r-R-0010.xml
exit 0
== --diff a-s2-R-0010.xml a-s5r-R-0010.xml
   legs: 6 -> 6
```

`evidence/count-s9.txt`
```
python -I count.py a-s9-M-marks.xml a-s9-C-catalogue.xml
exit 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  COMPANY rows: 1
== a-s9-C-catalogue.xml: 25010 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
```

`evidence/s9-same-row.txt`
```
2026-11-04T17:40:08Z python -I count.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml
exit 0
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```
