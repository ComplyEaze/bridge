# Results: row 2027-DN, where a keyed Delivery Note number lands in the vouchers window read, 4 Nov 2026

**In GDN each Delivery Note is one `INVOICEDELNOTES.LIST` entry holding its number in `BASICSHIPDELIVERYNOTE` and its date in the sibling `BASICSHIPPINGDATE`, for both vouchers.** [verified]
- **Sales A (no. 3):** one entry, `BASICSHIPDELIVERYNOTE` DN-81 and `BASICSHIPPINGDATE` 20250401.
- **Sales B (no. 4):** two entries, in the order keyed: DN-82 / 20250402, then DN-83 / 20250401.
- **The other vouchers:** each carries one empty `INVOICEDELNOTES.LIST`, with 0 children.
- **Type attributes:** neither child carries a `TYPE` attribute; count.py prints `TYPE ''` for both.
- **Empty lines:** the empty Delivery Note line Tally offers after the last number is not saved as an entry.

**GDN against GD:** rows 5 and 5. `INVOICEDELNOTES.LIST` is in every row (Sales 1, Sales 2 and Payment 1 one each, Sales 3 one, Sales 4 two). Rows otherwise equal: yes. Whole answer otherwise equal: yes. Keyed numbers anywhere in GD: 0. [verified]

**F-sales (the full read)** shows the same layout: the same entries, children, values and order for Sales 3 and 4. [verified]

By the sheet's Decides:
- **Each number in one element of one list entry, with its date in a named sibling, for both vouchers:** the parse reads a list of `{number, date}` from `INVOICEDELNOTES.LIST`. Sales B gives the shape of two entries.
- **The numbers are in GDN but not in GD, and both "otherwise equal" are yes:** the window read must name the list, so GDN's word goes into the build, after the seven dispatch words.

**Content checks:** every answer passed (`all answers parse and pass the content checks: yes`), and count.py was run after each request before the next was sent. **s9:** M and L are byte-identical to s1's. [verified]

Sheet: `lab/2027-dn-sheet` at 6f0c0b42, folder `lab/2027-dn/`.
- `SHA256.txt` hashes to 530041bec666…, and all 10 files it lists are equal.
- `gen.py` printed `checked: run.py M-marks.xml L-list.xml GD.xml GDN.xml F-sales.xml`.
- Every request is an Export, sent by `run.py` to the gateway on 9001. No Bridge build was run.

[verified]

Tally: one tally.exe (PID 1200) on 9001, TallyPrime Silver (title band). That it is release 7.2 comes from this PC's install and earlier rows, not from this sitting's screen. [partial]

## Steps
| step | as written, or what differed |
|---|---|
| 0 | `gen.py` printed its check line. The Gateway showed BRIDGE EINVOICE LAB loaded alone: period 1-Apr-25 to 31-Mar-26, current date Thursday, 3-Apr-2025, date of last entry 3-Apr-25. Nothing pressed. |
| 1 | `s1` as written, one request per `run.py` call with `count.py` after each. All checks hold: GUID yes; `ALTMSTID` 215; `ALTVCHID` 3; L three rows (1 Sales, 2 Sales, 1 Payment, all 20250403); content checks pass on both. BK on Tally's Backup screen: one company, to a new empty folder, "Accept ?" y. One file, 32,101 B, sha256 65adc25289eeb2d7362bef380b2ffc77c51f7dc84735e05472172b7b93cf4f51 (kept local). |
| 2 | Sales A keyed as below. Tally numbered it **3**, as expected. "Accept ?" y, "generate e-Invoice?" n. A blank Sales 4 opened; it was left by Escape and "Quit ?" y. |
| 3 | Sales B keyed as below. Tally numbered it **4**, as expected. "Accept ?" y, "generate e-Invoice?" n. A blank Sales 5 opened; it was left by Escape and "Quit ?" y. |
| 4 | `r1` as written, one request per `run.py` call (L, GD, GDN, F-sales, M), with `count.py` after each: every check passed before the next was sent. All curl 0. |
| 5 | Company shut. Restore by Data > Backup & Restore > Restore, Specify Path, the BK folder: one row, BRIDGE EINVOICE LAB, 4-Nov-26 7:31 am. "Accept ?" y, then "Company data folder (<company number>) already exists", Replace (r). Loaded alone with no prompt. `s9` as written, one request per call with `count.py` after each: content checks pass, and M and L are byte-identical to s1's. |

No prompt appeared beyond the six the sheet names. [verified]

## The two Sales invoices as keyed (observed)
- **Both:** Gateway > Vouchers > F8 Sales, dated 3-Apr-25 (Thursday), party `BRIDGE IRN Customer`. Section **"Dispatch Details"**.
  - Every dispatch field other than the Delivery Notes was left blank: Dispatch Doc No., Dispatched through, Destination, Carrier Name/Agent, Bill of Lading/LR-RR No. and its date, Motor Vehicle No.
  - Party Details accepted as offered with Ctrl+A. `Fetch Details Using GSTIN/UIN` not touched.
  - `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00 typed; total 118.00.
  - "Provide e-Invoice details" came up Yes and was set to No. Narration blank.
- **Sales A (no. 3):**
  - `Delivery Note No(s)` DN-81.
  - Its `Date`: Tally offered 3-Apr-25; 1-4-2025 was typed over it and shows 1-Apr-25.
  - The second line that followed was left empty.
- **Sales B (no. 4):**
  - Line 1 DN-82, date typed 2-4-2025 (shows 2-Apr-25).
  - Line 2 DN-83, date typed 1-4-2025 (shows 1-Apr-25).
  - Line 3 left empty.
- **Screens kept local:** the dispatch section of each voucher before saving.

## Other Bridge processes
The four Bridge connector servers of the desktop client were left running, and the row did not close them: PIDs 384 and 6668 (started 30 Oct 2026), 26480 and 26180 (started 2 Nov 2026), parent the desktop client. All four are unchanged before and after the sitting (`evidence/bridge-procs-before.txt`, `-after.txt`). No call was made to them from this sitting, and their calls were not measured. [verified as recorded]

## Kept local
Every answer (`a-*.xml`), every screen and the backup stay on the lab PC, listed by sha256, bytes and name in `kept-local.txt`. The exception is `a-r1-GDN.xml`, the fixture the sheet names: its content checks passed, and it is in `answers/` byte for byte.

`a-r1-GDN.xml`: PARTYGSTIN holds the lab's invented all-Z GSTIN; VOUCHERKEY 15-digit runs; kept as captured.

## The book now
BRIDGE EINVOICE LAB restored to its state before the sitting: three vouchers, `ALTVCHID` 3, `ALTMSTID` 215. Loaded alone at the Gateway. One tally.exe (PID 1200) on 9001.

**Confidence:** one synthetic book, one sitting, two Sales invoices keyed by hand, three Delivery Note numbers: PARTIAL on everything past this book.

## `count.py --company-number N`, whole output after s9

```
a-r1-F-sales.xml: 202676 bytes, rows with VOUCHERNUMBER 4
  10+ digit runs: 4 in the text, 4 by the parser, in VOUCHERKEY 4, elsewhere 0
  GSTIN shapes 12, outside the synthetic family 0; PAN shapes 12, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 729 direct children, 733 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 2 Sales: 729 direct children, 733 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 3 Sales: 729 direct children, 735 elements, INVOICEDELNOTES.LIST 1
    DN-81: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250401'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-81'
  voucher 4 Sales: 730 direct children, 738 elements, INVOICEDELNOTES.LIST 2
    DN-81: 0 elements
    DN-82: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DN-83: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    20250401: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    20250402: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    INVOICEDELNOTES.LIST 1: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250402'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-82'
    INVOICEDELNOTES.LIST 2: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250401'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-83'
a-r1-GD.xml: 228816 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 46 direct children, 644 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 2 Sales: 46 direct children, 644 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 1 Payment: 45 direct children, 428 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 3 Sales: 46 direct children, 644 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 4 Sales: 46 direct children, 644 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
a-r1-GDN.xml: 230202 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 47 direct children, 645 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 2 Sales: 47 direct children, 645 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 1 Payment: 46 direct children, 429 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 3 Sales: 47 direct children, 647 elements, INVOICEDELNOTES.LIST 1
    DN-81: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250401'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-81'
  voucher 4 Sales: 48 direct children, 650 elements, INVOICEDELNOTES.LIST 2
    DN-81: 0 elements
    DN-82: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DN-83: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    20250401: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    20250402: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    INVOICEDELNOTES.LIST 1: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250402'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-82'
    INVOICEDELNOTES.LIST 2: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250401'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-83'
a-r1-L-list.xml: 14998 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 0, outside the synthetic family 0; PAN shapes 0, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  1 20250403 Sales
  2 20250403 Sales
  1 20250403 Payment
  3 20250403 Sales
  4 20250403 Sales
a-r1-M-marks.xml: 3400 bytes, rows with VOUCHERNUMBER 0
  10+ digit runs: 0 in the text, 0 by the parser, in VOUCHERKEY 0, elsewhere 0
  GSTIN shapes 0, outside the synthetic family 0; PAN shapes 0, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '5'; ALTMSTID '215'
a-s1-L-list.xml: 10214 bytes, rows with VOUCHERNUMBER 3
  10+ digit runs: 3 in the text, 3 by the parser, in VOUCHERKEY 3, elsewhere 0
  GSTIN shapes 0, outside the synthetic family 0; PAN shapes 0, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  1 20250403 Sales
  2 20250403 Sales
  1 20250403 Payment
a-s1-M-marks.xml: 3398 bytes, rows with VOUCHERNUMBER 0
  10+ digit runs: 0 in the text, 0 by the parser, in VOUCHERKEY 0, elsewhere 0
  GSTIN shapes 0, outside the synthetic family 0; PAN shapes 0, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '3'; ALTMSTID '215'
a-s9-L-list.xml: 10214 bytes, rows with VOUCHERNUMBER 3
  10+ digit runs: 3 in the text, 3 by the parser, in VOUCHERKEY 3, elsewhere 0
  GSTIN shapes 0, outside the synthetic family 0; PAN shapes 0, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  1 20250403 Sales
  2 20250403 Sales
  1 20250403 Payment
a-s9-M-marks.xml: 3398 bytes, rows with VOUCHERNUMBER 0
  10+ digit runs: 0 in the text, 0 by the parser, in VOUCHERKEY 0, elsewhere 0
  GSTIN shapes 0, outside the synthetic family 0; PAN shapes 0, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  COMPANY elements with a GUID 1
  GUID is the book's: yes; ALTVCHID '3'; ALTMSTID '215'
GDN against GD (r1): rows 5 and 5; INVOICEDELNOTES.LIST in every row: yes (1 Sales 1, 2 Sales 1, 1 Payment 1, 3 Sales 1, 4 Sales 2); rows otherwise equal: yes; whole answer otherwise equal: yes; keyed numbers anywhere in GD: 0
all answers parse and pass the content checks: yes
```

## run.py logs

`log-r1.txt`
```
07:38:01	L-list.xml	a-r1-L-list.xml	curl 0
07:38:02	GD.xml	a-r1-GD.xml	curl 0
07:38:03	GDN.xml	a-r1-GDN.xml	curl 0
07:38:05	F-sales.xml	a-r1-F-sales.xml	curl 0
07:38:07	M-marks.xml	a-r1-M-marks.xml	curl 0
```

`log-s1.txt`
```
07:30:24	M-marks.xml	a-s1-M-marks.xml	curl 0
07:30:24	L-list.xml	a-s1-L-list.xml	curl 0
```

`log-s9.txt`
```
07:40:24	M-marks.xml	a-s9-M-marks.xml	curl 0
07:40:25	L-list.xml	a-s9-L-list.xml	curl 0
```
