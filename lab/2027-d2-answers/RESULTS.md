# Results: row 2027-D2, each admitted window read with and without the seven dispatch words, 4 Nov 2026

**All four pairs: each added word once in every row, rows otherwise equal, and the whole answer otherwise equal: yes.** That holds for GD against G (the 2027-D measurement, repeated in this sitting), EGD against EG, CGD against CG and CEGD against CEG. Every answer has 3 rows. The seven words are present and empty on each row, since no voucher in the book holds a dispatch detail. By the sheet's Decides: adding the seven words to every read that names the GST words is measured for the calls `vouchers` admits, on this book, with the values empty. [verified]

**GDN against GD:** each added word once in every row (the one `INVOICEDELNOTES.LIST` in each), with both "otherwise equal" yes. Each row has one `INVOICEDELNOTES.LIST` with 0 children. So the window read accepts the list and is otherwise unchanged. With no Delivery Note in the book, nothing can land in it here. [verified]

**s9 against s1:** M and L are byte-identical. The reads did not change the book. [verified]

Sheet: `lab/2027-d2-sheet` at 31c51237, folder `lab/2027-d2/`.
- `SHA256.txt` hashes to 3d7624f10d89…, and all 16 files it lists are equal.
- `gen.py` reproduced G, EG, CG, CEG and GDN byte for byte, and asserted GD's sha256.
- `run.py`, `M-marks.xml` and `L-list.xml` are byte-equal to `lab/2027-dispatch`'s, and `G.xml` to that row's `W.xml`.
- Every request is an Export, sent by `run.py` to the gateway on 9001. No Bridge build was run.

[verified]

Tally: one tally.exe (PID 1200) on 9001, TallyPrime Silver (title band). That it is release 7.2 comes from this PC's install and earlier rows, not from this sitting's screen. [partial]

## Steps
| step | as written, or what differed |
|---|---|
| 0 | The Gateway showed BRIDGE EINVOICE LAB loaded alone: period 1-Apr-25 to 31-Mar-26, current date Thursday, 3-Apr-2025, date of last entry 3-Apr-25. Nothing pressed. |
| 1 | `s1` as written (04:13:33). All checks hold: GUID yes; `ALTMSTID` 215; `ALTVCHID` 3; L three rows (1 Sales, 2 Sales, 1 Payment, all 20250403). |
| 2 | `r1` as written (04:13:40-41): all nine answered, curl 0. |
| 3 | `s9` as written (04:13:41-42): M and L byte-identical to s1's. |

No prompt or screen appeared at any point. Tally stayed at the Gateway. [verified]

## Other Bridge processes
The four Bridge connector servers of the desktop client were left running, and the row did not close them: PIDs 384 and 6668 (started 30 Oct 2026), 26480 and 26180 (started 2 Nov 2026), parent the desktop client. All four are unchanged before and after the sitting (`evidence/bridge-procs-before.txt`, `-after.txt`). No call was made to them from this sitting, and their calls were not measured. [verified as recorded]

## Kept local
Every answer (`a-*.xml`) and the screenshots stay on the lab PC, listed by sha256, bytes and name in `kept-local.txt`. The exception is `a-r1-CEGD.xml`, which is in `answers/` byte for byte as a test fixture. Each window answer holds 2 GSTIN shapes (count.py), the book's invented ones.

`a-r1-CEGD.xml`: PARTYGSTIN holds the lab's invented all-Z GSTIN; VOUCHERKEY 15-digit runs; kept as captured.

## The book now
BRIDGE EINVOICE LAB, unchanged: three vouchers, `ALTVCHID` 3, `ALTMSTID` 215. Loaded alone at the Gateway. One tally.exe (PID 1200) on 9001.

**Confidence:** one synthetic book, three vouchers with no dispatch values, one sitting: PARTIAL on everything past this book.

## `count.py`, whole output after s9

```
a-r1-CEG.xml: 134510 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
a-r1-CEGD.xml: 136934 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
  row 1 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 2 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 1 Payment: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
a-r1-CG.xml: 133646 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
a-r1-CGD.xml: 136070 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
  row 1 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 2 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 1 Payment: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
a-r1-EG.xml: 130276 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
a-r1-EGD.xml: 132700 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
  row 1 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 2 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 1 Payment: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
a-r1-G.xml: 129412 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
a-r1-GD.xml: 131836 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
  row 1 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 2 Sales: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
  row 1 Payment: BASICSHIPDOCUMENTNO 1 '', BASICSHIPPEDBY 1 '', BASICFINALDESTINATION 1 '', EICHECKPOST 1 '', BILLOFLADINGNO 1 '', BILLOFLADINGDATE 1 '', BASICSHIPVESSELNO 1 ''
a-r1-GDN.xml: 132178 bytes, GSTIN shapes 2, rows with VOUCHERNUMBER 3
  row 1 Sales: INVOICEDELNOTES.LIST 1; one with 0 children: 
  row 2 Sales: INVOICEDELNOTES.LIST 1; one with 0 children: 
  row 1 Payment: INVOICEDELNOTES.LIST 1; one with 0 children: 
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
GD against G (r1): rows 3 and 3; each added word once in every row: yes; rows otherwise equal: yes; whole answer otherwise equal: yes
EGD against EG (r1): rows 3 and 3; each added word once in every row: yes; rows otherwise equal: yes; whole answer otherwise equal: yes
CGD against CG (r1): rows 3 and 3; each added word once in every row: yes; rows otherwise equal: yes; whole answer otherwise equal: yes
CEGD against CEG (r1): rows 3 and 3; each added word once in every row: yes; rows otherwise equal: yes; whole answer otherwise equal: yes
GDN against GD (r1): rows 3 and 3; each added word once in every row: yes; rows otherwise equal: yes; whole answer otherwise equal: yes
```

## run.py logs

`log-r1.txt`
```
04:13:40	G.xml	a-r1-G.xml	curl 0
04:13:40	GD.xml	a-r1-GD.xml	curl 0
04:13:40	EG.xml	a-r1-EG.xml	curl 0
04:13:40	EGD.xml	a-r1-EGD.xml	curl 0
04:13:41	CG.xml	a-r1-CG.xml	curl 0
04:13:41	CGD.xml	a-r1-CGD.xml	curl 0
04:13:41	CEG.xml	a-r1-CEG.xml	curl 0
04:13:41	CEGD.xml	a-r1-CEGD.xml	curl 0
04:13:41	GDN.xml	a-r1-GDN.xml	curl 0
```

`log-s1.txt`
```
04:13:33	M-marks.xml	a-s1-M-marks.xml	curl 0
04:13:33	L-list.xml	a-s1-L-list.xml	curl 0
```

`log-s9.txt`
```
04:13:41	M-marks.xml	a-s9-M-marks.xml	curl 0
04:13:42	L-list.xml	a-s9-L-list.xml	curl 0
```
