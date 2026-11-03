# Results: row 2162-IRN, the number read with the e-invoice tags (R1) and a Credit Note's dispatch details (R2), 4 Nov 2026

**R1: all three hold on both Sales rows.** The number read with `IRN, IRNACKNO, IRNACKDATE` added to its fetch list carries each tag once in every row, and is otherwise equal to the plain read. Its three values equal the vouchers window read's: invoice A (Sales 1) has `IRN` length 64, `IRNACKNO` 112233 and `IRNACKDATE` 20250403, and B (Sales 2) and the Payment have all three present and empty. Per the sheet's Decides: the number read can carry the e-invoice details with no change to what it already answers. [verified]

**R2: each of the six dispatch values keyed lands in exactly one element** of the Credit Note's full read:

| field | element |
|---|---|
| Dispatch Doc No. DD-72 | `BASICSHIPDOCUMENTNO` |
| Dispatched through ROADWAY | `BASICSHIPPEDBY` |
| Destination Mysuru | `BASICFINALDESTINATION` |
| Carrier Name/Agent CARRIER-73 | `EICHECKPOST` |
| Bill of Lading/LR-RR No. LR-74 | `BILLOFLADINGNO`, with `BILLOFLADINGDATE` 20250403 |
| Motor Vehicle No. KA-75 | `BASICSHIPVESSELNO` |

DN-71 is in no element: it was not keyed, because the screen offered no Delivery Note field. Per the Decides, each of those six elements is its field's home. One of them is not where its name points: the carrier goes to `EICHECKPOST`, which the sheet's dispatch-name filter does not catch. [verified]

**Step 5:** after the restore of the step 1 BK, M's GUID, `ALTVCHID` 3 and `ALTMSTID` 215 equal s1's, and L holds s1's three rows. The M and L answers are byte-identical to s1's. [verified]

Sheet: `lab/2162-irn-read-sheet` at 8b1573e8 (amended; a fast-forward from 2e17cba3), folder `lab/2162-irn-read/`. `SHA256.txt` hashes to e3fac80cb3d1…, and every file it lists is equal. The amendment changed `ROW.md`, `count.py` and `SHA256.txt` only. `gen.py` reproduced N-1, N-2, NI-1, NI-2 and F-note byte for byte. Every request is an Export. `run.py` sent everything to the gateway on 9001. No Bridge build was run. [verified]

Tally: one tally.exe (PID 1200) on 9001, TallyPrime Silver (title band). That it is release 7.2 comes from this PC's install and earlier rows, not from this sitting's screen. [partial]

## Steps
| step | as written, or what differed |
|---|---|
| 1 | Book loaded alone from Select Company by filter, with no prompt. Gateway: period 1-Apr-25 to 31-Mar-26, current date Thursday, 3-Apr-2025, date of last entry 3-Apr-25. The first sheet's checks (V empty, L 0 rows) failed: the book held A, B and a Payment. I stopped there with nothing keyed, and the amended sheet's step 1 was then met by these answers. `s1` sent M and L (02:30:24). V-named was sent once, under its own label `p1` (02:33:11), before the amendment; it was not resent as part of `s1`. All amended checks hold: GUID yes; ALTMSTID 215 with ALTVCHID 3; L 3 rows (1 Sales, 2 Sales, 1 Payment, all 20250403); V-named 3 rows, A filled, B and the Payment empty. BK taken on Tally's Backup screen after the reads, one company to a new empty folder, "Accept ?" answered y: one file, 32,101 B, sha256 f02d4b50b9d62af62f8358cc5fe48f10c78d513bce19b54066c01f6873319380 (kept local). |
| 2 | Nothing keyed (amended sheet). |
| 3 | `r1` as written (02:35:55). M after: ALTVCHID 3, ALTMSTID 215, the same as s1. **Display:** the Day Book for 3-Apr-25 offers only "Enter: Alter" for a row, and no Display, so A was not opened. The e-Invoice details were not seen on screen. Day Book left with Escape. |
| 4 | Credit Note keyed as below. Saving raised "Accept ?", which the sheet does not name. I stopped there with no key pressed and asked. On that answer I pressed y. "Do you want to generate e-Invoice?" followed, answered n. Tally then opened a blank Credit Note 2, left by Escape and "Quit ?" y (nothing saved). `r2` as written (02:42:33). |
| 5 | Company shut. Restore by Data > Backup & Restore > Restore, Specify Path, the BK folder: one row, BRIDGE EINVOICE LAB, 4-Nov-26 2:32 am. "Accept ?" y, then "Company data folder (<company number>) already exists", Replace (r). No other prompt. Loaded alone with no prompt. `s9` as written (02:47:05): checks hold. |

## The Credit Note as keyed (step 4, observed)
- **Form:** Gateway > Vouchers > F10 Other Vouchers > Credit Note. "Accounting Voucher Creation", Credit Note No. 1, dated 3-Apr-25 (Thursday), Party A/c name `BRIDGE IRN Customer`, then Particulars: the layout Tally opened. I did not change the mode.
- **Dispatch section:** a sub-screen titled **"Receipt Details"** opened right after the party. Its labels, in order, and what was keyed:
  - `Tracking No(s)`: blank. The table does not name it.
  - `Dispatch Doc No.` DD-72.
  - `Dispatched through` ROADWAY.
  - `Destination` Mysuru.
  - `Carrier Name/Agent` CARRIER-73.
  - `Bill of Lading/LR-RR No.` LR-74, `Date` 3-Apr-25.
  - `Motor Vehicle No.` KA-75.
  - **`Delivery Note No(s)`: not offered**, so DN-71 was not keyed.
- **Original invoice:** the same sub-screen has "Original Invoice Details": `Original Invoice No.` 2, `Date` 3-4-2025.
- **Party Details:** Buyer (Bill to) as offered, accepted with Ctrl+A. `Fetch Details Using GSTIN/UIN` not touched.
- **Lines:** `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00, typed; total 118.00.
- **The e-invoice field** is labelled **"Provide GST/e-Invoice details"**. It came up Yes and was set to No. **No reason screen appeared**, so `01-Sales Return` was not asked for and not keyed.
- **Saving:** narration blank, then "Accept ?" y and "Do you want to generate e-Invoice?" n. No other prompt.
- **Screens kept local:** the dispatch section before saving, the note before saving, the Accept box and the generate prompt.

## Other Bridge processes
The four Bridge connector servers of the desktop client were left running. Stopping them was refused here, and the row did not close them: PIDs 384 and 6668 (started 30 Oct 2026), 26480 and 26180 (started 2 Nov 2026), parent the desktop client. All four are unchanged before and after the sitting (`evidence/bridge-procs-before.txt`, `-after.txt`). No call was made to them from this sitting, and their calls were not measured. [verified as recorded]

## Kept local
Every answer (`a-*.xml`) and every screen stays on the lab PC, listed by sha256, bytes and name in `kept-local.txt`. The exceptions are `a-r1-NI-1.xml` and `a-r1-NI-2.xml`, which are in `answers/` byte for byte as test fixtures. The full read of the note holds three GSTIN shapes (count.py). The backup stays local too.

VOUCHERKEY carries 15-digit runs (Tally's internal key), kept as captured.

## `count.py` (the sheet's amended copy), whole output after s9

```
a-p1-V-named.xml: 10826 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 3
  row 1 Sales: 23 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 0
    BRIDGEVCHISSALES: 0
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '112233'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
  row 2 Sales: 23 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '2'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 0
    BRIDGEVCHISSALES: 0
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
  row 1 Payment: 23 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Payment'
    ISCANCELLED: 0
    BRIDGEVCHISSALES: 0
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
a-r1-M-marks.xml: 3398 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 0
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '3'; ALTMSTID '215'
a-r1-N-1.xml: 8106 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 2
  row 1 Sales: 22 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 1, TYPE 'Logical', 'No'
    BRIDGEVCHISSALES: 1, TYPE 'Logical', 'Yes'
    IRN: 0
    IRNACKNO: 0
    IRNACKDATE: 0
  row 1 Payment: 22 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Payment'
    ISCANCELLED: 1, TYPE 'Logical', 'No'
    BRIDGEVCHISSALES: 1, TYPE 'Logical', 'No'
    IRN: 0
    IRNACKNO: 0
    IRNACKDATE: 0
a-r1-N-2.xml: 5558 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 1
  row 2 Sales: 22 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '2'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 1, TYPE 'Logical', 'No'
    BRIDGEVCHISSALES: 1, TYPE 'Logical', 'Yes'
    IRN: 0
    IRNACKNO: 0
    IRNACKDATE: 0
a-r1-NI-1.xml: 8734 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 2
  row 1 Sales: 25 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 1, TYPE 'Logical', 'No'
    BRIDGEVCHISSALES: 1, TYPE 'Logical', 'Yes'
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '112233'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
  row 1 Payment: 25 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Payment'
    ISCANCELLED: 1, TYPE 'Logical', 'No'
    BRIDGEVCHISSALES: 1, TYPE 'Logical', 'No'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
a-r1-NI-2.xml: 5794 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 1
  row 2 Sales: 25 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '2'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 1, TYPE 'Logical', 'No'
    BRIDGEVCHISSALES: 1, TYPE 'Logical', 'Yes'
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
a-r1-V-named.xml: 10826 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 3
  row 1 Sales: 23 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 0
    BRIDGEVCHISSALES: 0
    IRN: 1, TYPE 'String', length 64
    IRNACKNO: 1, TYPE 'String', '112233'
    IRNACKDATE: 1, TYPE 'Date', '20250403'
  row 2 Sales: 23 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '2'
    VOUCHERTYPENAME: 1, TYPE '', 'Sales'
    ISCANCELLED: 0
    BRIDGEVCHISSALES: 0
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
  row 1 Payment: 23 children
    DATE: 1, TYPE 'Date', '20250403'
    VOUCHERNUMBER: 1, TYPE '', '1'
    VOUCHERTYPENAME: 1, TYPE '', 'Payment'
    ISCANCELLED: 0
    BRIDGEVCHISSALES: 0
    IRN: 1, TYPE 'String', length 0
    IRNACKNO: 1, TYPE 'String', ''
    IRNACKDATE: 1, TYPE 'Date', ''
a-r2-F-note.xml: 53090 bytes, GSTIN shapes 3, rows with VOUCHERNUMBER 1
  voucher 1 Credit Note: 729 direct children, 733 elements
    DN-71: 0 elements
    DD-72: 1 elements; BASICSHIPDOCUMENTNO x1
    ROADWAY: 1 elements; BASICSHIPPEDBY x1
    Mysuru: 1 elements; BASICFINALDESTINATION x1
    CARRIER-73: 1 elements; EICHECKPOST x1
    LR-74: 1 elements; BILLOFLADINGNO x1
    KA-75: 1 elements; BASICSHIPVESSELNO x1
    direct children matching the dispatch names, non-empty: 9
      BILLOFLADINGDATE '20250403'
      BILLOFLADINGNO 'LR-74'
      BASICSHIPPEDBY 'ROADWAY'
      BASICSHIPDOCUMENTNO 'DD-72'
      BASICFINALDESTINATION 'Mysuru'
      BASICSHIPVESSELNO 'KA-75'
      ISSHIPPINGWITHINSTATE 'No'
      ISDELIVERYSAMEASCONSIGNEE 'No'
      ISDISPATCHSAMEASCONSIGNOR 'No'
a-r2-L-list.xml: 12628 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 4
  1 20250403 Sales
  2 20250403 Sales
  1 20250403 Payment
  1 20250403 Credit Note
a-r2-M-marks.xml: 3398 bytes, GSTIN shapes 0, rows with VOUCHERNUMBER 0
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '4'; ALTMSTID '215'
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
NI-1 (r1) against N-1: rows 2 and 2; each IRN tag once in every row: yes; otherwise equal: yes
  against V-named row 1 Sales: equal on IRN, IRNACKNO, IRNACKDATE
  against V-named row 1 Payment: equal on IRN, IRNACKNO, IRNACKDATE
NI-2 (r1) against N-2: rows 1 and 1; each IRN tag once in every row: yes; otherwise equal: yes
  against V-named row 2 Sales: equal on IRN, IRNACKNO, IRNACKDATE
```

## run.py logs

`log-p1.txt`
```
02:33:11	V-named.xml	a-p1-V-named.xml	curl 0
```

`log-r1.txt`
```
02:35:55	N-1.xml	a-r1-N-1.xml	curl 0
02:35:55	NI-1.xml	a-r1-NI-1.xml	curl 0
02:35:55	N-2.xml	a-r1-N-2.xml	curl 0
02:35:55	NI-2.xml	a-r1-NI-2.xml	curl 0
02:35:55	V-named.xml	a-r1-V-named.xml	curl 0
02:35:55	M-marks.xml	a-r1-M-marks.xml	curl 0
```

`log-r2.txt`
```
02:42:33	L-list.xml	a-r2-L-list.xml	curl 0
02:42:33	F-note.xml	a-r2-F-note.xml	curl 0
02:42:34	M-marks.xml	a-r2-M-marks.xml	curl 0
```

`log-s1.txt`
```
02:30:24	M-marks.xml	a-s1-M-marks.xml	curl 0
02:30:24	L-list.xml	a-s1-L-list.xml	curl 0
```

`log-s9.txt`
```
02:47:05	M-marks.xml	a-s9-M-marks.xml	curl 0
02:47:05	L-list.xml	a-s9-L-list.xml	curl 0
```
