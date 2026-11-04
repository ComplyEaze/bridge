# Results: row 2027-DN2, the e-invoice, voucher-type and combined window reads with the Delivery Note list, 4 Nov 2026

**All three pairs are as Expected.** EGDN, CGDN and CEGDN each carry the Delivery Notes exactly as GDN did in 2027-DN. [verified]
- **Sales 3:** one `INVOICEDELNOTES.LIST` entry, `BASICSHIPPINGDATE` 20250401 then `BASICSHIPDELIVERYNOTE` DN-81.
- **Sales 4:** two entries, DN-82 / 20250402 then DN-83 / 20250401, in keyed order.
- **Every other row** (Sales 1, Sales 2, Payment 1): one entry with no children.
- **Type attributes:** neither child carries a `TYPE` attribute.

**Against the reads without the list:**

| pair | rows | list in every row | rows otherwise equal | whole answer otherwise equal | keyed numbers in the read without the list |
|---|---|---|---|---|---|
| EGDN against EGD | 5 and 5 | yes | yes | yes | 0 |
| CGDN against CGD | 5 and 5 | yes | yes | yes | 0 |
| CEGDN against CEGD | 5 and 5 | yes | yes | yes | 0 |

By the sheet's Decides: **the list is measured on every read that names the GST words, on this book.** The #2027 Delivery Note change keeps the list on all four reads (with 2027-DN's GDN), and its partial mark narrows to one book and one release. [verified]

**Content checks:** every answer passed, and count.py was run after each request before the next was sent. **s9:** M and L are byte-identical to s1's. [verified]

Sheet: `lab/2027-dn2-sheet` at d4ec6901, folder `lab/2027-dn2/`.
- `SHA256.txt` hashes to 3227e3b8d8b9…, and all 13 files it lists are equal.
- `gen.py` printed `checked: run.py M-marks.xml L-list.xml EGD.xml EGDN.xml CGD.xml CGDN.xml CEGD.xml CEGDN.xml`.
- Every request is an Export, sent by `run.py` to the gateway on 9001. No Bridge build was run.

[verified]

Tally: one tally.exe (PID 1200) on 9001, TallyPrime Silver (title band). That it is release 7.2 comes from this PC's install and earlier rows, not from this sitting's screen. [partial]

## Steps
| step | as written, or what differed |
|---|---|
| 0 | `gen.py` printed its check line. The Gateway showed BRIDGE EINVOICE LAB loaded alone: period 1-Apr-25 to 31-Mar-26, current date Thursday, 3-Apr-2025, date of last entry 3-Apr-25. Nothing pressed. |
| 1 | `s1` as written: M, then `count.py`; L, then `count.py`. All checks hold: GUID yes; `ALTMSTID` 215; `ALTVCHID` 3; L three rows (1 Sales, 2 Sales, 1 Payment, all 20250403); content checks pass on both. BK on Tally's Backup screen: one company, to a new empty folder, "Accept ?" y. One file, 32,101 B, sha256 84132d4933b564c7cda3d830fc6f8fec10af05e70fbdbd0c2914427e9882115a (kept local). |
| 2 | Sales A keyed as 2027-DN's step 2. Tally numbered it **3**, as expected. "Accept ?" y, "generate e-Invoice?" n. A blank Sales 4 opened; it was left by Escape and "Quit ?" y. |
| 3 | Sales B keyed as 2027-DN's step 3. Tally numbered it **4**, as expected. "Accept ?" y, "generate e-Invoice?" n. A blank Sales 5 opened; it was left by Escape and "Quit ?" y. |
| 4 | `r1` in the sheet's order, one request per `run.py` call: L, EGD, EGDN, CGD, CGDN, CEGD, CEGDN, M. `count.py` ran after each, and every check passed before the next was sent. All curl 0. L has five rows (+ Sales 3 and 4, 20250403). M: `ALTVCHID` 5, above s1's. |
| 5 | Company shut. Restore by Data > Backup & Restore > Restore, Specify Path, the BK folder: one row, BRIDGE EINVOICE LAB, 4-Nov-26 8:23 am. "Accept ?" y, then "Company data folder (<company number>) already exists", Replace (r). Loaded alone with no prompt. `s9` as written, one request per call with `count.py` after each: content checks pass, and M and L are byte-identical to s1's. |

No prompt appeared beyond the six the sheet names. [verified]

## The two Sales invoices as keyed (observed)
- **Both:** as 2027-DN.
  - Gateway > Vouchers (it opened on Sales), dated 3-Apr-25 (Thursday), party `BRIDGE IRN Customer`, section "Dispatch Details".
  - Every dispatch field other than the Delivery Notes was left blank.
  - Party Details accepted as offered with Ctrl+A. `Fetch Details Using GSTIN/UIN` not touched.
  - `BRIDGE IRN Sales` 100.00 and `BRIDGE IRN IGST` 18.00 typed; total 118.00.
  - "Provide e-Invoice details" set from Yes to No. Narration blank.
- **Sales A (no. 3):** DN-81, date typed 1-4-2025 over the offered 3-Apr-25 (shows 1-Apr-25). The second line was left empty.
- **Sales B (no. 4):** DN-82 dated 2-Apr-25, DN-83 dated 1-Apr-25. The third line was left empty.
- **Screens kept local:** the dispatch section of each voucher before saving.

## Fixture
`answers/a-r1-CEGDN.xml`: 238,592 B, sha256 1bb93749d8c744f7091c5899a610b4630f5a68964e5775128f405b01e9e3db17, byte for byte as captured. Its checks, unedited:
- runs of 10 or more digits only in VOUCHERKEY (5);
- one IRN of 64 characters (Sales 1) and four empty, with no 10+ digit run inside any;
- company number 0 in UTF-8 and in UTF-16LE;
- 4 GSTIN shapes, all in PARTYGSTIN, all the lab's invented all-Z value;
- PAN 0, phone 0.

PARTYGSTIN holds the lab's invented all-Z GSTIN; VOUCHERKEY 15-digit runs; kept as captured.

## Other Bridge processes
The four Bridge connector servers of the desktop client were left running, and the row did not close them: PIDs 384 and 6668 (started 30 Oct 2026), 26480 and 26180 (started 2 Nov 2026), parent the desktop client. All four are unchanged before and after the sitting (`evidence/bridge-procs-before.txt`, `-after.txt`). No call was made to them from this sitting, and their calls were not measured. [verified as recorded]

## Kept local
Every answer (`a-*.xml`) except `a-r1-CEGDN.xml` (in `answers/`), every screen and the backup stay on the lab PC, listed by sha256, bytes and name in `kept-local.txt`.

## The book now
BRIDGE EINVOICE LAB restored to its state before the sitting: three vouchers, `ALTVCHID` 3, `ALTMSTID` 215. Loaded alone at the Gateway. One tally.exe (PID 1200) on 9001.

**Confidence:** one synthetic book, one sitting, two Sales invoices keyed by hand, three Delivery Note numbers: PARTIAL on everything past this book.

## `count.py --company-number N`, whole output after s9

```
a-r1-CEGD.xml: 237206 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 59 direct children, 657 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 2 Sales: 59 direct children, 657 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 1 Payment: 58 direct children, 441 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 3 Sales: 59 direct children, 657 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 4 Sales: 59 direct children, 657 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
a-r1-CEGDN.xml: 238592 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 60 direct children, 658 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 2 Sales: 60 direct children, 658 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 1 Payment: 59 direct children, 442 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 3 Sales: 60 direct children, 660 elements, INVOICEDELNOTES.LIST 1
    DN-81: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250401'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-81'
  voucher 4 Sales: 61 direct children, 663 elements, INVOICEDELNOTES.LIST 2
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
a-r1-CGD.xml: 235870 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 56 direct children, 654 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 2 Sales: 56 direct children, 654 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 1 Payment: 55 direct children, 438 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 3 Sales: 56 direct children, 654 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 4 Sales: 56 direct children, 654 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
a-r1-CGDN.xml: 237256 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 57 direct children, 655 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 2 Sales: 57 direct children, 655 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 1 Payment: 56 direct children, 439 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 3 Sales: 57 direct children, 657 elements, INVOICEDELNOTES.LIST 1
    DN-81: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250401'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-81'
  voucher 4 Sales: 58 direct children, 660 elements, INVOICEDELNOTES.LIST 2
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
a-r1-EGD.xml: 230152 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 49 direct children, 647 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 2 Sales: 49 direct children, 647 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 1 Payment: 48 direct children, 431 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 3 Sales: 49 direct children, 647 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
  voucher 4 Sales: 49 direct children, 647 elements, INVOICEDELNOTES.LIST 0
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
a-r1-EGDN.xml: 231538 bytes, rows with VOUCHERNUMBER 5
  10+ digit runs: 5 in the text, 5 by the parser, in VOUCHERKEY 5, elsewhere 0
  GSTIN shapes 4, outside the synthetic family 0; PAN shapes 4, outside the synthetic family 0; company number in UTF-16 0, in UTF-8 0
  content checks: pass
  voucher 1 Sales: 50 direct children, 648 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 2 Sales: 50 direct children, 648 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 1 Payment: 49 direct children, 432 elements, INVOICEDELNOTES.LIST 1
    DN-81: 0 elements
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 0 elements
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 0 children, 0 elements
  voucher 3 Sales: 50 direct children, 650 elements, INVOICEDELNOTES.LIST 1
    DN-81: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPDELIVERYNOTE x1
    DN-82: 0 elements
    DN-83: 0 elements
    20250401: 1 elements; INVOICEDELNOTES.LIST/BASICSHIPPINGDATE x1
    20250402: 0 elements
    INVOICEDELNOTES.LIST 1: 2 children, 2 elements
      BASICSHIPPINGDATE TYPE '' '20250401'
      BASICSHIPDELIVERYNOTE TYPE '' 'DN-81'
  voucher 4 Sales: 51 direct children, 653 elements, INVOICEDELNOTES.LIST 2
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
EGDN against EGD (r1): rows 5 and 5; INVOICEDELNOTES.LIST in every row: yes (1 Sales 1, 2 Sales 1, 1 Payment 1, 3 Sales 1, 4 Sales 2); rows otherwise equal: yes; whole answer otherwise equal: yes; keyed numbers anywhere in EGD: 0
CGDN against CGD (r1): rows 5 and 5; INVOICEDELNOTES.LIST in every row: yes (1 Sales 1, 2 Sales 1, 1 Payment 1, 3 Sales 1, 4 Sales 2); rows otherwise equal: yes; whole answer otherwise equal: yes; keyed numbers anywhere in CGD: 0
CEGDN against CEGD (r1): rows 5 and 5; INVOICEDELNOTES.LIST in every row: yes (1 Sales 1, 2 Sales 1, 1 Payment 1, 3 Sales 1, 4 Sales 2); rows otherwise equal: yes; whole answer otherwise equal: yes; keyed numbers anywhere in CEGD: 0
all answers parse and pass the content checks: yes
```

## run.py logs

`log-r1.txt`
```
08:28:30	L-list.xml	a-r1-L-list.xml	curl 0
08:28:31	EGD.xml	a-r1-EGD.xml	curl 0
08:28:32	EGDN.xml	a-r1-EGDN.xml	curl 0
08:28:34	CGD.xml	a-r1-CGD.xml	curl 0
08:28:36	CGDN.xml	a-r1-CGDN.xml	curl 0
08:28:38	CEGD.xml	a-r1-CEGD.xml	curl 0
08:28:40	CEGDN.xml	a-r1-CEGDN.xml	curl 0
08:28:42	M-marks.xml	a-r1-M-marks.xml	curl 0
```

`log-s1.txt`
```
08:22:06	M-marks.xml	a-s1-M-marks.xml	curl 0
08:22:07	L-list.xml	a-s1-L-list.xml	curl 0
```

`log-s9.txt`
```
08:31:03	M-marks.xml	a-s9-M-marks.xml	curl 0
08:31:05	L-list.xml	a-s9-L-list.xml	curl 0
```
