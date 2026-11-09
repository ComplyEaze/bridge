# Row 2162-IRN9 results: an import sets an invoice's e-invoice details, and Tally then lists it as IRN generated

Run on 9 Nov 2026, 08:30Z to 08:41Z, by the sheet at `lab/2162-irn-import-sheet` (463fbea9). Its 17 files are equal to its `SHA256.txt` by sha256 and size. Book: `BRIDGE EINVOICE LAB` (synthetic, GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`), loaded alone. TallyPrime 7.2 Silver (tally.exe ProductVersion 1.1.7.2). The Gateway showed current date Thursday 3-Apr-2025 and period 1-Apr-25 to 31-Mar-26. No Bridge build: raw requests by the sheet's `run.py`, one at a time, 21 in all, each with curl exit 0. Every value below is M, measured by this run. PARTIAL: one book, one release, invented details.

## Answer

| check | result |
|---|---|
| s1, the book as the sheet expects (V 3, M 215; Sales 1, Sales 2, Payment 1; A's details on Sales 1; K one customer row, one GSTIN) | PASS |
| BK before the first write (C5) | PASS: `TDBK1800_<company number>.001`, 32,101 B, sha256 `24cb1f3f1b5dd419fc7b27a0daea2c4bef9edf6b09143a6a8949528b4517ef9b` |
| step 2, `fill.py` | PASS: two sent files written, hashes below; no stop |
| s3, invoice 3 with `IRN`, `IRNACKNO`, `IRNACKDATE` | CREATED 1, LASTVCHID 4, every other counter 0, no LINEERROR |
| s3r, what Tally stored for 3 | all three as sent: `IRN` length 64 and equal to the IRN sent, `IRNACKNO` 445566, `IRNACKDATE` 20250403 |
| s4, invoice 4 without them | CREATED 1, LASTVCHID 5, every other counter 0, no LINEERROR |
| s4r, what Tally stored for 4 | all three present and empty, as B |
| F-3 against F-4 | they differ in the three elements and the voucher's own identifiers only (below) |
| step 5, the e-Invoice report | 3 is listed under `IRN Generated` > `IRN updated without QR Code`, beside 1 (A). 4 is listed under `Uncertain Transactions (Corrections needed)` > `Applicable Tax Ledger is not selected`, beside 2 (B) |
| s5m, the book after the report | unchanged: V 5, M 215, equal to s4r's |
| step 6, the restore of BK (C6) | PASS: s9's M, L and V answers are byte-equal to s1's (V 3, M 215) |
| C10 | one tally.exe owning 9001, /status 200, no company loaded |

**By the sheet's Decides:** `3` carries the three as sent, and the report lists it under `IRN Generated`. So an import can carry an invoice's e-invoice details, and Tally then treats the invoice as e-invoiced. Item 9 is then buildable, through a builder of its own, reviewed and sent to a lab book first (P7). PARTIAL: one book, one release, invented details.

- The report's line under `IRN Generated` is `IRN updated without QR Code`. It holds 1 (A, keyed) and 3 (imported) alike. Neither carries a QR code: s3r's F shows `IRNQRCODE` empty for 3.
- The import set no e-invoice element besides the three. F-3 shows `IRNIRPSOURCE` empty and `ISEINVOVERRIDDEN` No, with the other e-invoice family values empty, No or 0. The two `...DATETIME` elements carry a 9-digit value each, as count.py prints them. The same 37 elements are on 4.
- `4`, imported without details, lists with B under `Applicable Tax Ledger is not selected`. That is the IGST ledger's effect that 1378 saw. As the sheet says, this is not item 10's answer: a clean tax line is the variable to change next.
- s5m equals s4r: looking at the report did not change the book.

## The sent files (step 2, kept on the lab PC)

`fill.py a-s1-K-customer.xml` printed:

```
c9d8fbbd1a79e60d28eada2b705e4badc4d95ff4958feeea924e3e95868a92b9  4198  I-3-sent.xml
af135fcaed74f669888b2e123731168586857310544334d13088175e809ed3f7  3928  I-4-sent.xml
```

Each write's file was checked against these lines before its send, and its marks read passed the before-write check in a `run.py` call of its own before the write was sent alone.

## Step 5, the e-Invoice report (read only)

Opened by Go To (`e-Invoice` under GST Reports). It opened directly on the one registration. Its period already read 1-Apr-25 to 3-Apr-25, the sheet's period, so Alt+F2 was not pressed. The words as on the screen, with every count, are in `evidence/s5-screens.txt`. In short:

| line | count | vouchers it names |
|---|---|---|
| Uncertain Transactions (Corrections needed) | 2 | |
| . Transactions with Incomplete/Mismatch in Information > Outward Supplies > Invalid or Missing Information > Applicable Tax Ledger is not selected | 2 | Sales 2 (1,180.00), Sales 4 (118.00) |
| IRN Generated > IRN updated without QR Code | 2 | Sales 1 (1,180.00), Sales 3 (118.00) |

- Per invoice: 1 and 3 under `IRN Generated` > `IRN updated without QR Code`; 2 and 4 under `Applicable Tax Ledger is not selected`.
- The register under `IRN Generated` offered `F10: Cancel IRN`, `Manually Update IRN Info` and `Get IRN Info`. The cause list offered `S: Set GST Status`. None was pressed, no Enter went on a voucher line, and no prompt came up.

## Prompts and keys (BK and restore)

- BK: `Accept ?` answered y.
- Restore: `Accept ?` answered y. Then `Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.` answered r (Replace).
- The Restore list showed `BRIDGE EINVOICE LAB` with its number and `9-Nov-26 (2:03 pm)`, the backup's time. No other prompt came up.

## Kept on the lab PC

Listed by size and sha256 in `kept-local.txt`:
- the backup;
- the two sent files;
- every K and F answer (they hold the book's GSTINs);
- the marks answers the sheet does not list for committing (s1b, s3m, s4m, s5m);
- the folder and process listings, the restore plan and the one-write helper.

The captures are listed in `SCREENSHOTS.txt`. The full notes, one line per step with UTC times, are `evidence/notes.txt`.

## count.py, whole

Each block is count.py's output for that step's answers (`evidence/count-*.txt`, `same-row-*.txt`, `diff-s4-3-4.txt`, `before-write-*.txt`).

### count-s1

```
== a-s1-K-customer.xml: 35872 bytes, GSTIN shapes 2
  STATUS '1'
  LEDGER rows: 1; named 'BRIDGE IRN Customer': 1
   PARENT 'Sundry Debtors' ISBILLWISEON 'No' GSTIN values held: 2 distinct: 1
== a-s1-L-list.xml: 10214 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 3
   '1' '20250403' 'Sales' MASTERID '1'
   '2' '20250403' 'Sales' MASTERID '2'
   '1' '20250403' 'Payment' MASTERID '3'
== a-s1-M-marks.xml: 3398 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '3' ALTMSTID '215'
  COMPANY rows: 1
== a-s1-V-named.xml: 10826 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 3
   row '1' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '112233'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
   row '2' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
   row '1' 'Payment' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
```

### count-s1b

```
== a-s1b-M-marks.xml: 3398 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '3' ALTMSTID '215'
  COMPANY rows: 1
```

### count-s3m

```
== a-s3m-M-marks.xml: 3398 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '3' ALTMSTID '215'
  COMPANY rows: 1
```

### before-write-s3

```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '3', '215') against ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '3', '215')
  one COMPANY row, the same GUID: the write may be sent
```

### count-s3

```
== a-s3-I-3-sent.xml: 538 bytes, GSTIN shapes 0
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '4'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### count-s3r

```
== a-s3r-F-3.xml: 52992 bytes, GSTIN shapes 2
  STATUS '1'
  VOUCHER rows: 1
   voucher '3' 'Sales' DATE '20250403' direct children 729, elements 733
    PARTYGSTIN <GSTIN shape>
    CONSIGNEEGSTIN ''
    STATENAME 'Karnataka'
    PLACEOFSUPPLY 'Karnataka'
    GSTREGISTRATIONTYPE 'Regular'
    ISINVOICE 'Yes'
    PERSISTEDVIEW 'Invoice Voucher View'
    VCHENTRYMODE 'Accounting Invoice'
    NUMBERINGSTYLE 'Auto Retain'
    VCHGSTCLASS 'Not Applicable'
    VCHGSTSTATUSISAPPLICABLE 'Yes'
    VCHGSTSTATUSISINCLUDED 'No'
    VCHGSTSTATUSISUNCERTAIN 'Yes'
    VCHGSTSTATUSISEXCLUDED 'No'
    ISEWAYBILLAPPLICABLE 'No'
    e-invoice family, direct children: 37
     IRNCANCELDATE TYPE '' ''
     IRNACKDATE TYPE '' '20250403'
     UAEEINVBILLDATE TYPE '' ''
     IRN TYPE '' length 64
     IRNCANCELREASON TYPE '' ''
     IRNCANCELCODE TYPE '' ''
     IRNQRCODE TYPE '' length 0
     IRNACKNO TYPE '' '445566'
     IRNIRPSOURCE TYPE '' ''
     UAEEINVAGENCYID TYPE '' ''
     UAEEINVAGENCYNAME TYPE '' ''
     UAEEINVAGENCYNUM TYPE '' ''
     UAEEINVUUID TYPE '' ''
     UAEEINVREFERENCEID TYPE '' ''
     UAEEINVALLOWANCEREASON TYPE '' ''
     UAEEINVCHARGEREASON TYPE '' ''
     UAEEINVQRCODE TYPE '' ''
     UAEEINVBUYERAGENCYID TYPE '' ''
     UAEEINVBUYERAGENCYNAME TYPE '' ''
     UAEEINVBUYERAGENCYNUM TYPE '' ''
     UAEEINVALLOWANCETAXEXEMPTIONREASON TYPE '' ''
     UAEEINVCHARGETAXEXEMPTIONREASON TYPE '' ''
     IGNOREEINVVALIDATION TYPE '' 'No'
     IRNJSONEXPORTED TYPE '' 'No'
     IRNCANCELLED TYPE '' 'No'
     ISEINVOVERRIDDEN TYPE '' 'No'
     UAEEINVISBILLPREPAIDAMTOVRDN TYPE '' 'No'
     RESETIRNQRCODE TYPE '' 'No'
     QRCODECRC TYPE '' '0'
     UAEEINVCRC TYPE '' '0'
     IGNEINVVALIDCRC TYPE '' '0'
     UAEEINVBILLPREPAIDAMOUNT TYPE '' ''
     UAEEINVBILLPENDINGAMT TYPE '' ''
     UAEEINVBILLTOTALAMT TYPE '' ''
     IRNACKUPDATEDATETIME TYPE '' <9 digits>
     UAEEINVUPLOADDATETIME TYPE '' <9 digits>
     IRNERRORLIST.LIST TYPE '' ''
== a-s3r-L-list.xml: 12604 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 4
   '1' '20250403' 'Sales' MASTERID '1'
   '2' '20250403' 'Sales' MASTERID '2'
   '1' '20250403' 'Payment' MASTERID '3'
   '3' '20250403' 'Sales' MASTERID '4'
== a-s3r-M-marks.xml: 3398 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '4' ALTMSTID '215'
  COMPANY rows: 1
== a-s3r-V-named.xml: 13524 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 4
   row '1' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '112233'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
   row '2' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
   row '1' 'Payment' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
   row '3' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '445566'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
```

### count-s4m

```
== a-s4m-M-marks.xml: 3398 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '4' ALTMSTID '215'
  COMPANY rows: 1
```

### before-write-s4

```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '4', '215') against ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '3', '215')
  one COMPANY row, the same GUID: the write may be sent
```

### count-s4

```
== a-s4-I-4-sent.xml: 538 bytes, GSTIN shapes 0
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '5'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### count-s4r

```
== a-s4r-F-4.xml: 52786 bytes, GSTIN shapes 2
  STATUS '1'
  VOUCHER rows: 1
   voucher '4' 'Sales' DATE '20250403' direct children 729, elements 733
    PARTYGSTIN <GSTIN shape>
    CONSIGNEEGSTIN ''
    STATENAME 'Karnataka'
    PLACEOFSUPPLY 'Karnataka'
    GSTREGISTRATIONTYPE 'Regular'
    ISINVOICE 'Yes'
    PERSISTEDVIEW 'Invoice Voucher View'
    VCHENTRYMODE 'Accounting Invoice'
    NUMBERINGSTYLE 'Auto Retain'
    VCHGSTCLASS 'Not Applicable'
    VCHGSTSTATUSISAPPLICABLE 'Yes'
    VCHGSTSTATUSISINCLUDED 'No'
    VCHGSTSTATUSISUNCERTAIN 'Yes'
    VCHGSTSTATUSISEXCLUDED 'No'
    ISEWAYBILLAPPLICABLE 'No'
    e-invoice family, direct children: 37
     IRNCANCELDATE TYPE '' ''
     IRNACKDATE TYPE '' ''
     UAEEINVBILLDATE TYPE '' ''
     IRN TYPE '' length 0
     IRNCANCELREASON TYPE '' ''
     IRNCANCELCODE TYPE '' ''
     IRNQRCODE TYPE '' length 0
     IRNACKNO TYPE '' ''
     IRNIRPSOURCE TYPE '' ''
     UAEEINVAGENCYID TYPE '' ''
     UAEEINVAGENCYNAME TYPE '' ''
     UAEEINVAGENCYNUM TYPE '' ''
     UAEEINVUUID TYPE '' ''
     UAEEINVREFERENCEID TYPE '' ''
     UAEEINVALLOWANCEREASON TYPE '' ''
     UAEEINVCHARGEREASON TYPE '' ''
     UAEEINVQRCODE TYPE '' ''
     UAEEINVBUYERAGENCYID TYPE '' ''
     UAEEINVBUYERAGENCYNAME TYPE '' ''
     UAEEINVBUYERAGENCYNUM TYPE '' ''
     UAEEINVALLOWANCETAXEXEMPTIONREASON TYPE '' ''
     UAEEINVCHARGETAXEXEMPTIONREASON TYPE '' ''
     IGNOREEINVVALIDATION TYPE '' 'No'
     IRNJSONEXPORTED TYPE '' 'No'
     IRNCANCELLED TYPE '' 'No'
     ISEINVOVERRIDDEN TYPE '' 'No'
     UAEEINVISBILLPREPAIDAMTOVRDN TYPE '' 'No'
     RESETIRNQRCODE TYPE '' 'No'
     QRCODECRC TYPE '' '0'
     UAEEINVCRC TYPE '' '0'
     IGNEINVVALIDCRC TYPE '' '0'
     UAEEINVBILLPREPAIDAMOUNT TYPE '' ''
     UAEEINVBILLPENDINGAMT TYPE '' ''
     UAEEINVBILLTOTALAMT TYPE '' ''
     IRNACKUPDATEDATETIME TYPE '' <9 digits>
     UAEEINVUPLOADDATETIME TYPE '' <9 digits>
     IRNERRORLIST.LIST TYPE '' ''
== a-s4r-L-list.xml: 14998 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 5
   '1' '20250403' 'Sales' MASTERID '1'
   '2' '20250403' 'Sales' MASTERID '2'
   '1' '20250403' 'Payment' MASTERID '3'
   '3' '20250403' 'Sales' MASTERID '4'
   '4' '20250403' 'Sales' MASTERID '5'
== a-s4r-M-marks.xml: 3400 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '5' ALTMSTID '215'
  COMPANY rows: 1
== a-s4r-V-named.xml: 16070 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 5
   row '1' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '112233'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
   row '2' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
   row '1' 'Payment' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
   row '3' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '445566'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
   row '4' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
```

### diff-s4-3-4

```
== --diff a-s3r-F-3.xml a-s4r-F-4.xml
   ALTERID: '4' -> '5'
   GUID: <45 chars> -> <45 chars>
   IRN: <64 chars> -> ''
   IRNACKDATE: '20250403' -> ''
   IRNACKNO: '445566' -> ''
   MASTERID: '4' -> '5'
   REFERENCE: '3' -> '4'
   REMOTEGUID: <36 chars> -> <36 chars>
   VOUCHERKEY: <15 digits> -> <15 digits>
   VOUCHERNUMBER: '3' -> '4'
   VOUCHERRETAINKEY: '9' -> '13'
```

### count-s5m

```
== a-s5m-M-marks.xml: 3400 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '5' ALTMSTID '215'
  COMPANY rows: 1
```

### same-row-s5m

```
== a-s5m-M-marks.xml against a-s4r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '5', '215') against ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '5', '215')
  equal
```

### count-s9

```
== a-s9-L-list.xml: 10214 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 3
   '1' '20250403' 'Sales' MASTERID '1'
   '2' '20250403' 'Sales' MASTERID '2'
   '1' '20250403' 'Payment' MASTERID '3'
== a-s9-M-marks.xml: 3398 bytes, GSTIN shapes 0
  STATUS '1'
  COMPANY: GUID is the book's: yes ALTVCHID '3' ALTMSTID '215'
  COMPANY rows: 1
== a-s9-V-named.xml: 10826 bytes, GSTIN shapes 0
  STATUS '1'
  VOUCHER rows: 3
   row '1' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '112233'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
   row '2' 'Sales' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
   row '1' 'Payment' DATE '20250403'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
```

### same-row-s9

```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '3', '215') against ('eb2f5190-292e-4ed8-bfd3-eace839fe699', '3', '215')
  equal
```
