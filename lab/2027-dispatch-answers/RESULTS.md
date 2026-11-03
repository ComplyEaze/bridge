# Results: row 2027-D, the vouchers window read with the seven dispatch words, on a Sales invoice and a Credit Note, 4 Nov 2026

**The window read carries the seven dispatch words with no change to what it already answers.** WD against W:
- each dispatch word once in every row: **yes**;
- rows otherwise equal: **yes**;
- whole answer otherwise equal: **yes**.

In WD, the new Sales invoice (no. 3) and the Credit Note (no. 1) each carry `BASICSHIPDOCUMENTNO` DD-72, `BASICSHIPPEDBY` ROADWAY, `BASICFINALDESTINATION` Mysuru, `EICHECKPOST` CARRIER-73, `BILLOFLADINGNO` LR-74, `BILLOFLADINGDATE` 20250403 and `BASICSHIPVESSELNO` KA-75. All are typed String, except `BILLOFLADINGDATE`, typed Date. The three earlier vouchers carry all seven present and empty. [verified]

**The Sales values land in the same seven elements as the note's:** one mapping serves both. [verified]

**The Sales screen offered `Delivery Note No(s)`.** DN-71 lands in exactly one element, `INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE`, with its date asked for and kept. The Credit Note's screen offered no Delivery Note field, as in R2. [verified]

By the sheet's Decides:
- building the seven words into `vouchers` behind their own opt-in argument is measured, for the rows this book holds;
- the Delivery Note's home is `INVOICEDELNOTES.LIST`, for a follow-up row.

**Step 5:** after the restore of this sitting's BK, M (GUID, `ALTVCHID` 3, `ALTMSTID` 215) and L (three rows) are byte-identical to s1's. [verified]

Sheet: `lab/2027-dispatch-sheet` at f6039d62, folder `lab/2027-dispatch/`.
- `SHA256.txt` hashes to f1e88ae6008b…, and all 13 files it lists are equal.
- `run.py`, `M-marks.xml` and `L-list.xml` are byte-equal to `lab/2162-irn-read`'s, and `F-note.xml` to that row's `F-note.xml`.
- `gen.py` reproduced W, WD, F-sales and F-note byte for byte.
- Every request is an Export, sent by `run.py` to the gateway on 9001. No Bridge build was run.

[verified]

Tally: one tally.exe (PID 1200) on 9001, TallyPrime Silver (title band). That it is release 7.2 comes from this PC's install and earlier rows, not from this sitting's screen. [partial]

## Steps
| step | as written, or what differed |
|---|---|
| 0 | BRIDGE FOREXGROUP LAB shut by Company > Shut, with no question. BRIDGE EINVOICE LAB loaded alone by its name filter, with no prompt. Gateway: period 1-Apr-25 to 31-Mar-26, current date Thursday, 3-Apr-2025, date of last entry 3-Apr-25. |
| 1 | `s1` as written. All checks hold: GUID yes; `ALTMSTID` 215; `ALTVCHID` 3; L three rows (1 Sales, 2 Sales, 1 Payment, all 20250403). BK on Tally's Backup screen: one company, to a new empty folder, "Accept ?" y. One file, 32,101 B, sha256 943b32c1b8c8f61345f3229924d112ac6f10645d600a06390acebdd0a6559780 (kept local). |
| 2 | Sales invoice keyed as below. Tally numbered it **3**. "Accept ?" y, then "Do you want to generate e-Invoice?" n. Tally opened a blank Sales 4, left by Escape and "Quit ?" y. |
| 3 | Credit Note keyed as R2 of `lab/2162-irn-read` did. Tally numbered it **1**. "Accept ?" y, then "generate e-Invoice?" n. A blank Credit Note 2 opened; it was left by Escape and "Quit ?" y after the reads. |
| 4 | `r1` as written (03:51:58): all six answered, curl 0. |
| 5 | Company shut. Restore by Data > Backup & Restore > Restore, Specify Path, the BK folder: one row, BRIDGE EINVOICE LAB, 4-Nov-26 3:44 am. "Accept ?" y, then "Company data folder (<company number>) already exists", Replace (r). Loaded alone with no prompt. `s9` as written: checks hold. |

No prompt appeared beyond the ones the sheet names. [verified]

## The Sales invoice as keyed (step 2, observed)
- **Form:** Gateway > Vouchers > F8 Sales. "Accounting Voucher Creation", Sales No. 3, dated 3-Apr-25 (Thursday), Party A/c name `BRIDGE IRN Customer`, in the layout Tally opened.
- **Dispatch section:** a sub-screen titled **"Dispatch Details"** opened right after the party. Its labels, in order, and what was keyed:
  - `Delivery Note No(s)` DN-71. Tally then asked `Date`, already 3-Apr-25, accepted. A second, empty Delivery Note line followed, left empty.
  - `Dispatch Doc No.` DD-72.
  - `Dispatched through` ROADWAY.
  - `Destination` Mysuru.
  - `Carrier Name/Agent` CARRIER-73.
  - `Bill of Lading/LR-RR No.` LR-74, `Date` 3-Apr-25.
  - `Motor Vehicle No.` KA-75.
- **Other labels:** every label in the table was offered. None was offered that the table does not name: there was no `Tracking No(s)` on the Sales screen.
- **Party Details:** as offered, accepted with Ctrl+A. `Fetch Details Using GSTIN/UIN` not touched.
- **Lines:** `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00, typed; total 118.00.
- **The e-invoice field:** on the Sales voucher it is labelled **"Provide e-Invoice details"**. It came up Yes and was set to No. Narration blank.

## The Credit Note as keyed (step 3, observed)
- **Form:** Credit Note No. 1, dated 3-Apr-25, party `BRIDGE IRN Customer`.
- **Dispatch section:** **"Receipt Details"**, the same as R2:
  - `Tracking No(s)` blank; the table does not name it;
  - DD-72, ROADWAY, Mysuru, CARRIER-73, LR-74 dated 3-Apr-25, KA-75 under their labels;
  - **`Delivery Note No(s)` not offered**, so DN-71 was not keyed.
- **Original invoice:** "Original Invoice Details": `Original Invoice No.` 2, `Date` 3-4-2025.
- **Party Details:** accepted as offered.
- **Lines:** 100.00 and 18.00, total 118.00.
- **The e-invoice field:** "Provide GST/e-Invoice details" set to No. No reason screen appeared. Narration blank.

## What count.py shows beyond the expected
- **F-sales:** count.py's list of dispatch-named direct `.LIST` children prints 0 for Sales 3. The Delivery Note's list is `INVOICEDELNOTES.LIST`, whose name the filter does not catch, so its home shows only in the keyed-value line (`INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1`). Sales 3 has 735 elements, against 733 for the others.
- **F-sales, Sales 1 and 2:** each has two other non-empty dispatch-named children, `DISPATCHFROMNAME` and `DISPATCHFROMSTATENAME`, printed by length only. Sales 3 does not have them in that list.
- **F-note against R2:** the Credit Note's full read again holds each of the six keyed values in one element, as R2 found. Its non-empty dispatch-named children are 10, where R2 counted 9. The difference is that `CHECKPOST` is in this sheet's filter, so `EICHECKPOST` is now listed.

[verified]

## Other Bridge processes
The four Bridge connector servers of the desktop client were left running, and the row did not close them: PIDs 384 and 6668 (started 30 Oct 2026), 26480 and 26180 (started 2 Nov 2026), parent the desktop client. All four are unchanged before and after the sitting (`evidence/bridge-procs-before.txt`, `-after.txt`). No call was made to them from this sitting, and their calls were not measured. [verified as recorded]

## Kept local
Every answer (`a-*.xml`), every screen and the backup stay on the lab PC, listed by sha256, bytes and name in `kept-local.txt`. The full reads and W/WD hold the book's invented GSTINs (count.py counts them).

## The book now
BRIDGE EINVOICE LAB restored to its state before the sitting: three vouchers, `ALTVCHID` 3, `ALTMSTID` 215. Loaded alone at the Gateway. One tally.exe (PID 1200) on 9001.

**Confidence:** one synthetic book, one Tally, one sitting, one Sales invoice and one Credit Note keyed by hand: PARTIAL on everything past this book.

## `count.py`, whole output after s9

```
a-r1-F-note.xml: 53092 bytes, GSTIN shapes 3, rows with VOUCHERNUMBER 1
  voucher 1 Credit Note: 729 direct children, 733 elements
    DN-71: 0 elements
    DD-72: 1 elements; BASICSHIPDOCUMENTNO x1
    ROADWAY: 1 elements; BASICSHIPPEDBY x1
    Mysuru: 1 elements; BASICFINALDESTINATION x1
    CARRIER-73: 1 elements; EICHECKPOST x1
    LR-74: 1 elements; BILLOFLADINGNO x1
    KA-75: 1 elements; BASICSHIPVESSELNO x1
    direct children matching the dispatch names, non-empty: 10
      BILLOFLADINGDATE '20250403'
      BILLOFLADINGNO 'LR-74'
      EICHECKPOST 'CARRIER-73'
      BASICSHIPPEDBY 'ROADWAY'
      BASICSHIPDOCUMENTNO 'DD-72'
      BASICFINALDESTINATION 'Mysuru'
      BASICSHIPVESSELNO 'KA-75'
      ISSHIPPINGWITHINSTATE 'No'
      ISDELIVERYSAMEASCONSIGNEE 'No'
      ISDISPATCHSAMEASCONSIGNOR 'No'
    direct .LIST children matching the dispatch names: 0
a-r1-F-sales.xml: 152820 bytes, GSTIN shapes 9, rows with VOUCHERNUMBER 3
  voucher 1 Sales: 729 direct children, 733 elements
    DN-71: 0 elements
    DD-72: 0 elements
    ROADWAY: 0 elements
    Mysuru: 0 elements
    CARRIER-73: 0 elements
    LR-74: 0 elements
    KA-75: 0 elements
    direct children matching the dispatch names, non-empty: 5
      DISPATCHFROMNAME '<19 chars>'
      DISPATCHFROMSTATENAME '<11 chars>'
      ISSHIPPINGWITHINSTATE 'No'
      ISDELIVERYSAMEASCONSIGNEE 'No'
      ISDISPATCHSAMEASCONSIGNOR 'No'
    direct .LIST children matching the dispatch names: 0
  voucher 2 Sales: 729 direct children, 733 elements
    DN-71: 0 elements
    DD-72: 0 elements
    ROADWAY: 0 elements
    Mysuru: 0 elements
    CARRIER-73: 0 elements
    LR-74: 0 elements
    KA-75: 0 elements
    direct children matching the dispatch names, non-empty: 5
      DISPATCHFROMNAME '<19 chars>'
      DISPATCHFROMSTATENAME '<11 chars>'
      ISSHIPPINGWITHINSTATE 'No'
      ISDELIVERYSAMEASCONSIGNEE 'No'
      ISDISPATCHSAMEASCONSIGNOR 'No'
    direct .LIST children matching the dispatch names: 0
  voucher 3 Sales: 729 direct children, 735 elements
    DN-71: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DD-72: 1 elements; BASICSHIPDOCUMENTNO x1
    ROADWAY: 1 elements; BASICSHIPPEDBY x1
    Mysuru: 1 elements; BASICFINALDESTINATION x1
    CARRIER-73: 1 elements; EICHECKPOST x1
    LR-74: 1 elements; BILLOFLADINGNO x1
    KA-75: 1 elements; BASICSHIPVESSELNO x1
    direct children matching the dispatch names, non-empty: 10
      BILLOFLADINGDATE '20250403'
      BILLOFLADINGNO 'LR-74'
      EICHECKPOST 'CARRIER-73'
      BASICSHIPPEDBY 'ROADWAY'
      BASICSHIPDOCUMENTNO 'DD-72'
      BASICFINALDESTINATION 'Mysuru'
      BASICSHIPVESSELNO 'KA-75'
      ISSHIPPINGWITHINSTATE 'No'
      ISDELIVERYSAMEASCONSIGNEE 'No'
      ISDISPATCHSAMEASCONSIGNOR 'No'
    direct .LIST children matching the dispatch names: 0
a-r1-L-list.xml: 15020 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 5
  1 20250403 Sales
  2 20250403 Sales
  1 20250403 Payment
  3 20250403 Sales
  1 20250403 Credit Note
a-r1-M-marks.xml: 3400 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 0
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '5'; ALTMSTID '215'
a-r1-W.xml: 224824 bytes, GSTIN shapes 4, rows with VOUCHERNUMBER 5
  row 1 Sales: 39 direct children
    BASICSHIPDOCUMENTNO: 0
    BASICSHIPPEDBY: 0
    BASICFINALDESTINATION: 0
    EICHECKPOST: 0
    BILLOFLADINGNO: 0
    BILLOFLADINGDATE: 0
    BASICSHIPVESSELNO: 0
  row 2 Sales: 39 direct children
    BASICSHIPDOCUMENTNO: 0
    BASICSHIPPEDBY: 0
    BASICFINALDESTINATION: 0
    EICHECKPOST: 0
    BILLOFLADINGNO: 0
    BILLOFLADINGDATE: 0
    BASICSHIPVESSELNO: 0
  row 1 Payment: 38 direct children
    BASICSHIPDOCUMENTNO: 0
    BASICSHIPPEDBY: 0
    BASICFINALDESTINATION: 0
    EICHECKPOST: 0
    BILLOFLADINGNO: 0
    BILLOFLADINGDATE: 0
    BASICSHIPVESSELNO: 0
  row 3 Sales: 39 direct children
    BASICSHIPDOCUMENTNO: 0
    BASICSHIPPEDBY: 0
    BASICFINALDESTINATION: 0
    EICHECKPOST: 0
    BILLOFLADINGNO: 0
    BILLOFLADINGDATE: 0
    BASICSHIPVESSELNO: 0
  row 1 Credit Note: 39 direct children
    BASICSHIPDOCUMENTNO: 0
    BASICSHIPPEDBY: 0
    BASICFINALDESTINATION: 0
    EICHECKPOST: 0
    BILLOFLADINGNO: 0
    BILLOFLADINGDATE: 0
    BASICSHIPVESSELNO: 0
a-r1-WD.xml: 229048 bytes, GSTIN shapes 4, rows with VOUCHERNUMBER 5
  row 1 Sales: 46 direct children
    BASICSHIPDOCUMENTNO: 1, TYPE 'String', ''
    BASICSHIPPEDBY: 1, TYPE 'String', ''
    BASICFINALDESTINATION: 1, TYPE 'String', ''
    EICHECKPOST: 1, TYPE 'String', ''
    BILLOFLADINGNO: 1, TYPE 'String', ''
    BILLOFLADINGDATE: 1, TYPE 'Date', ''
    BASICSHIPVESSELNO: 1, TYPE 'String', ''
  row 2 Sales: 46 direct children
    BASICSHIPDOCUMENTNO: 1, TYPE 'String', ''
    BASICSHIPPEDBY: 1, TYPE 'String', ''
    BASICFINALDESTINATION: 1, TYPE 'String', ''
    EICHECKPOST: 1, TYPE 'String', ''
    BILLOFLADINGNO: 1, TYPE 'String', ''
    BILLOFLADINGDATE: 1, TYPE 'Date', ''
    BASICSHIPVESSELNO: 1, TYPE 'String', ''
  row 1 Payment: 45 direct children
    BASICSHIPDOCUMENTNO: 1, TYPE 'String', ''
    BASICSHIPPEDBY: 1, TYPE 'String', ''
    BASICFINALDESTINATION: 1, TYPE 'String', ''
    EICHECKPOST: 1, TYPE 'String', ''
    BILLOFLADINGNO: 1, TYPE 'String', ''
    BILLOFLADINGDATE: 1, TYPE 'Date', ''
    BASICSHIPVESSELNO: 1, TYPE 'String', ''
  row 3 Sales: 46 direct children
    BASICSHIPDOCUMENTNO: 1, TYPE 'String', 'DD-72'
    BASICSHIPPEDBY: 1, TYPE 'String', 'ROADWAY'
    BASICFINALDESTINATION: 1, TYPE 'String', 'Mysuru'
    EICHECKPOST: 1, TYPE 'String', 'CARRIER-73'
    BILLOFLADINGNO: 1, TYPE 'String', 'LR-74'
    BILLOFLADINGDATE: 1, TYPE 'Date', '20250403'
    BASICSHIPVESSELNO: 1, TYPE 'String', 'KA-75'
  row 1 Credit Note: 46 direct children
    BASICSHIPDOCUMENTNO: 1, TYPE 'String', 'DD-72'
    BASICSHIPPEDBY: 1, TYPE 'String', 'ROADWAY'
    BASICFINALDESTINATION: 1, TYPE 'String', 'Mysuru'
    EICHECKPOST: 1, TYPE 'String', 'CARRIER-73'
    BILLOFLADINGNO: 1, TYPE 'String', 'LR-74'
    BILLOFLADINGDATE: 1, TYPE 'Date', '20250403'
    BASICSHIPVESSELNO: 1, TYPE 'String', 'KA-75'
a-s1-L-list.xml: 10214 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 3
  1 20250403 Sales
  2 20250403 Sales
  1 20250403 Payment
a-s1-M-marks.xml: 3398 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 0
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '3'; ALTMSTID '215'
a-s9-L-list.xml: 10214 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 3
  1 20250403 Sales
  2 20250403 Sales
  1 20250403 Payment
a-s9-M-marks.xml: 3398 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 0
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '3'; ALTMSTID '215'
WD (r1) against W: rows 5 and 5; each dispatch word once in every row: yes; rows otherwise equal: yes; whole answer otherwise equal: yes
```

## run.py logs

`log-r1.txt`
```
03:51:58	L-list.xml	a-r1-L-list.xml	curl 0
03:51:58	W.xml	a-r1-W.xml	curl 0
03:51:58	WD.xml	a-r1-WD.xml	curl 0
03:51:58	F-sales.xml	a-r1-F-sales.xml	curl 0
03:51:59	F-note.xml	a-r1-F-note.xml	curl 0
03:51:59	M-marks.xml	a-r1-M-marks.xml	curl 0
```

`log-s1.txt`
```
03:43:15	M-marks.xml	a-s1-M-marks.xml	curl 0
03:43:16	L-list.xml	a-s1-L-list.xml	curl 0
```

`log-s9.txt`
```
03:54:21	M-marks.xml	a-s9-M-marks.xml	curl 0
03:54:21	L-list.xml	a-s9-L-list.xml	curl 0
```
