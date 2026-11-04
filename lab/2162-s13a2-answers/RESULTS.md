# 2162-S13a2 sitting 1: the S13 invoice to a registered customer, with a GSTIN that is not the company's own: results

**First line: PARTIAL, the invoice reads included.** `BP/26-27/0053` was sent as S13 sent it, with one change: the customer's GSTIN, 08ZZZZZ0000Z3ZO, which is not the company's own (08ZZZZZ0000Z1ZQ) but shares its PAN part. Tally stored it as sent, and its read-back reads **included Yes, uncertain No**, excluded No, overridden No, post-dated No. Against S13's `0053`, only `PARTYGSTIN` and those two flags differ. This is the Decides case "Included Yes, uncertain No":
- S13's uncertain status came from the customer's GSTIN being the company's own: one variable, one voucher, one book.
- The shared PAN is not a cause, because this customer shares it.
- Item 18's first element set is stored as the build writes it and is listed as included. PARTIAL: a raw import, not a build, and a customer under the company's own PAN.
- Tally's GSTR-1 screen counts the invoice under **B2B Invoices - 4A, 4B, 4C, 6B, 6C**: 1 voucher, 1,051.00. That is item 19's placement for a registered customer (PARTIAL: a screen read).
- The look at GSTR-1 changed nothing: the marks are equal and the read-back is byte-equal before and after it.
- The restore put the book back: the marks are 68/245 with the book's GUID, byte-equal to step 1's answer, and the catalogue is byte-equal to step 2's. (verified)

Sheet `lab/2162-s13a2-sheet` at 6e157faf (SHA256.txt 19 of 19 equal, by sha256 and size). LAB-COMMON v3 (5370c6d2…). Book: BRIDGE PILOT LAB, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85. TallyPrime 7.2 Silver. Each step and its time is in `notes.txt` (UTC). Each request file was checked against SHA256.txt just before its send. count.py's whole output for every answer is at the foot of this file, and in `evidence/count-*.txt`.

## Steps (EXPECT against what was read)

| step | EXPECT | read | |
|---|---|---|---|
| 1 load | BRIDGE PILOT LAB alone, no prompt; date not before 1-Aug-2026 | alone, no prompt; Tuesday, 15-Sep-2026; TallyPrime SILVER (7.2) | = |
| 1 s1 | one COMPANY row, the GUID, 68 / 245 | one row, the GUID, 68 / 245 | = |
| 1 BK | one file in a new empty folder | TDBK1800_&lt;company number&gt;.001, 90,051 B, sha256 4e80aa88347bee9d67d7c63a764c286387030a6de3457eafd8c0b6e090a3ba7c; marks after it 68 / 245 | = |
| 2 s2 | 0 rows named BRIDGE Customer RJ in C and Q; 0 rows in N-0053; R-0010 1 row | C 18, 0 named; Q 18, 0 named; N-0053 0; R-0010 1 row: BRIDGE Walk-in, Unregistered/Consumer, included Yes, every other flag No. Each answer byte-equal to S13's s2 | = |
| 3 s3 | CREATED 1, every other counter 0, counted from a-s3-L-seed.xml itself | CREATED 1; ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED, EXCEPTIONS 0; LASTVCHID 0; LASTMID 0; no LINEERROR | = |
| 3 s3r | C: one row, Sundry Debtors, ISBILLWISEON No. Q: one row, one registration row: 20260401, Regular, Rajasthan, 08ZZZZZ0000Z3ZO | C 19: one, Sundry Debtors, No. Q 19: one, Sundry Debtors, PARTYGSTIN and STATENAME absent, one non-empty LEDGSTREGDETAILS.LIST with exactly those four children; M 68 / 246 | = |
| 4 s4 | CREATED 1, every other counter 0 except LASTVCHID, no LINEERROR | CREATED 1; LASTVCHID 45; LASTMID 0; the rest 0; no LINEERROR | = |
| 4 s4r | R-0053 1 row; N-0053 1 row of Sales class | R-0053 1 row; N-0053 1 row, BRIDGEVCHISSALES Yes; M 69 / 246 | = |
| 4 diff, control | identity fields; party, PARTYGSTIN Z3ZO, Regular; leg 1's ledger; nothing else except possibly the flags; state, place of supply, overridden, post-dated unchanged | exactly those; the flags unchanged (included Yes, uncertain No) | = |
| 4 diff, S13 | PARTYGSTIN Z1ZQ → Z3ZO; identity fields recorded; anything else is the result | PARTYGSTIN, **VCHGSTSTATUSISINCLUDED No → Yes, VCHGSTSTATUSISUNCERTAIN Yes → No**; identity fields equal to S13's | result |
| 5 GSTR-1 | read only; 0053 included → its section, B2B expected | B2B Invoices - 4A, 4B, 4C, 6B, 6C: one party line, 08ZZZZZ0000Z3ZO, BRIDGE Customer RJ, 1 voucher, 1,051.00 | = |
| 5 s5m | --same-row against s4r passes | equal (69 / 246); the answer byte-equal to s4r's | = |
| 5 s5r | --diff against s4r prints only `legs: 6 -> 6` | only `legs: 6 -> 6`; the answer byte-equal to s4r's | = |
| 6 restore | only "Accept ?" (y) and "Company data folder (…) already exists" (r) | those two, nothing else; the list showed the BK alone, 4-Nov-26 (11:46 pm) | = |
| 6 s9 | --same-row passes (GUID, 68, 245); C's rows equal to s2's | passes; M byte-equal to s1's; C byte-equal to s2's (18 rows) | = |
| 7 close | one tally.exe owns 9001; no company loaded | tally.exe 1200 LISTENING on 9001; GET /status 200; Select Company, none loaded | = |

After both writes, the whole Tally window showed the Gateway with no dialog; both captures are byte-equal to step 1's Gateway capture. No prompt appeared at any step except the backup's and the restore's own, each named above. (verified)

## The three diffs, whole

**Against the control**, `count.py --diff a-s2-R-0010.xml a-s4r-R-0053.xml`. The voucher keys are grouped here; the captured file holds them as digit runs and stays on the lab PC.
```
   ALTERID: '17' -> '69'
   GSTREGISTRATIONTYPE: 'Unregistered/Consumer' -> 'Regular'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   PARTYGSTIN: '' -> '08ZZZZZ0000Z3ZO'
   PARTYLEDGERNAME: 'BRIDGE Walk-in' -> 'BRIDGE Customer RJ'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0053'
   VOUCHERKEY: '198,573,517,963,352' -> '198,573,517,963,384'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0053'
   VOUCHERRETAINKEY: '41' -> '129'
   legs: 6 -> 6
   leg 1: ["'BRIDGE Walk-in'", "'Yes'", "'-1051.00'"] -> ["'BRIDGE Customer RJ'", "'Yes'", "'-1051.00'"]
```

**Against S13's `0053`**, `count.py --diff S13-a-s4r-R-0053.xml a-s4r-R-0053.xml` (`evidence/s4r-diff-s13.txt`):
```
   PARTYGSTIN: '08ZZZZZ0000Z1ZQ' -> '08ZZZZZ0000Z3ZO'
   VCHGSTSTATUSISINCLUDED: 'No' -> 'Yes'
   VCHGSTSTATUSISUNCERTAIN: 'Yes' -> 'No'
   legs: 6 -> 6
```

**Before and after the GSTR-1 look**, `count.py --diff a-s4r-R-0053.xml a-s5r-R-0053.xml` (`evidence/s5r-diff.txt`): `legs: 6 -> 6` only. (verified)

## Step 5: Tally's GSTR-1 screen, as text

- **Opening it.** At the Gateway: Alt+G, then `GSTR-1` typed. The List of Reports showed, under GST Reports, `GSTR-1`, `GSTR-1 Reconciliation` and `GSTR-1 Filed Summary - Downloaded`, with no line highlighted. One Down put the highlight on `GSTR-1`, then Enter. No list of GST registrations came up: the report opened directly on GST Registration `08ZZZZZ0000Z1ZQ`.
- **The period.** The report opened on 1-Aug-26 to 31-Aug-26, the period the step sets. Its button bar shows `F2: Period` and no Alt+F2. On the recorded word, no period key was pressed.
- **The summary**, as shown:
  - Status: Not Filed. "Last online GST activity: No Activity Found".
  - Total Vouchers 40.
  - Included in Return 28: Ready for Upload 28, Not Uploaded 1, Modified in Books After Upload/Export 27.
  - Not Relevant for This Return 7.
  - Uncertain Transactions (Corrections needed) 5.
  - "Check Vouchers Having Potential Conflicts with Masters": Yes.
- **The Return View**, every line in order (arrow keys only; screenshots `s5-1` and `s5-2`). Only two lines carry counts:
  - B2B Invoices - 4A, 4B, 4C, 6B, 6C: Vch Count 1, Taxable 1,000.80, CGST 25.02, SGST/UTGST 25.02, Tax 50.04, Invoice 1,051.00.
  - B2C (Small) Invoices - 7: 27 (2), Taxable 13,508.70, CGST 383.26, SGST/UTGST 383.24, Tax 766.50.
  - The other lines, with no counts or amounts: B2C (Large) Invoices - 5A, 5B; Exports Invoices - 6A; Credit or Debit Notes (Registered) - 9B; Credit or Debit Notes (Unregistered) - 9B; Amended B2B Invoices - 9A; Amended B2C (Large) Invoices - 9A; Amended Exports Invoices - 9A; Amended Credit or Debit Notes (Registered) - 9C; Amended Credit or Debit Notes (Unregistered) - 9C; Nil Rated Invoices - 8A, 8B, 8C, 8D; Amendment B2C (Small) Invoices - 10; Tax Liability (Advances Received) - 11A(1), 11A(2); Adjustment of Advances - 11B(1), 11B(2); Amended Tax Liability (Advances Received) - 11A; Amendment of Adjusted Advances - 11B; HSN Summary - 12 (B2B - B2C Supplies); Document Summary - 13.
  - Total: 14,509.50, 408.28, 408.26, 816.54, 1,051.00.
- **B2B, opened with Enter** (`GSTR-1 - Summary`, Details of B2B Invoices - 4A, 4B, 4C, 6B, 6C): one line, `08ZZZZZ0000Z3ZO`, Party Name `BRIDGE Customer RJ`, Voucher Count 1, Taxable 1,000.80, CGST 25.02, SGST/UTGST 25.02, Tax 50.04, Invoice 1,051.00.
  - The list is party-wise and shows no voucher number. No further key was pressed (the screen's F5 Voucher-wise is not one the step names).
  - Its one voucher is `BP/26-27/0053` by the reads: the only voucher of `BRIDGE Customer RJ`, a ledger made at step 3, with the same amounts as R-0053.
  - No cause is listed: it reads included.
- **The 5 uncertain transactions** are the book's state before this row. `0053` is counted under B2B. They were not opened, on the recorded word.
- **Leaving.** Esc twice, one screen at a time, back to the Gateway (Tuesday, 15-Sep-2026, no dialog).

## Kept on the lab PC

Each kept file is listed in `kept-local.txt` by size and sha256, with its reason. No captured byte is altered. A captured file is pushed whole unless it holds the company number or a run of 10 or more digits. Two places are exempt: a run inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element.
- The BK.
- The company-folder listings, and the backup folder's listing before.
- Any file the stage script found holding such a value, named in `kept-local.txt`: the diff against the control, whose voucher keys are digit runs, is given whole above, grouped.

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
python -I count.py a-s2-C-catalogue.xml a-s2-Q-rates.xml a-s2-N-0053.xml a-s2-R-0010.xml
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
2026-11-04T18:16:59Z python -I count.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml
exit 0
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

`evidence/count-s3.txt`
```
python -I count.py a-s3-L-seed.xml
exit 0
== a-s3-L-seed.xml: 538 bytes
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
   LEDGSTREGDETAILS.LIST: APPLICABLEFROM '20260401'; GSTREGISTRATIONTYPE 'Regular'; PLACEOFSUPPLY 'Rajasthan'; GSTIN '08ZZZZZ0000Z3ZO'
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
2026-11-04T18:17:22Z python -I count.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml
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
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
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

`evidence/s4r-diff-s13.txt`
```
2026-11-04T18:17:35Z python -I count.py --diff S13-a-s4r-R-0053.xml a-s4r-R-0053.xml
exit 0
== --diff S13-a-s4r-R-0053.xml a-s4r-R-0053.xml
   PARTYGSTIN: '08ZZZZZ0000Z1ZQ' -> '08ZZZZZ0000Z3ZO'
   VCHGSTSTATUSISINCLUDED: 'No' -> 'Yes'
   VCHGSTSTATUSISUNCERTAIN: 'Yes' -> 'No'
   legs: 6 -> 6
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

`evidence/s5m-same-row.txt`
```
2026-11-04T18:21:01Z python -I count.py --same-row a-s4r-M-marks.xml a-s5m-M-marks.xml
exit 0
== a-s5m-M-marks.xml against a-s4r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'")
  equal
```

`evidence/count-s5r.txt`
```
python -I count.py a-s5r-R-0053.xml
exit 0
== a-s5r-R-0053.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0053'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0053'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08ZZZZZ0000Z3ZO'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
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
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

`evidence/s5r-diff.txt`
```
2026-11-04T18:21:03Z python -I count.py --diff a-s4r-R-0053.xml a-s5r-R-0053.xml
exit 0
== --diff a-s4r-R-0053.xml a-s5r-R-0053.xml
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
2026-11-04T18:24:16Z python -I count.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml
exit 0
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```
