# W14: the company-level HSN/SAC read, and the tax-amount tolerance (Windows lab, 12 Oct 2026)

Book: synthetic BRIDGE PILOT LAB itself, not a copy. Company number 100033, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85. It was loaded alone. TallyPrime 7.1 Silver.
- The book is in the state W13 left it in:
  - 0005, 0006 and 0016 accepted;
  - 0002, 0003, 0008 and the W11 vouchers No. 5 and No. 6 uncertain.
- Request set: lab/1342-capture-requests @ 99e750ac. W14-README.md sha256 e30b7bb9….
- Every request matched SHA256.txt (which includes the W12 and W13 requests) or SHA256-top.txt (`r02-marks.xml`) before it was sent.
- Mouse-click mode. Accept As Is was never used, and no master or voucher was changed.
- Fresh backup before the run: `<lab>\w14\backup-pre\TDBK1800_100033.001`, 85,433 B, sha256 7b2507452202066f1d0b71222ad1da65175998ed710571bcbc1c1af0ede980a9, 12:41:18Z. It stays on the lab PC.
- Screenshots stay on the lab PC, listed by hash.
- Sends: 8, one at a time, each with curl exit 0 and printed `200 <bytes>`. `send-log.txt` holds every line.

**Result: the run stopped at step 2, under the README's own rule.** No screen offers a tax-amount tolerance. Steps 6–12 were not done, and nothing in the book was changed.

## Step 0 (file only)
- `aw11-taxunits-0.xml` (22,304 B, sha256 1b2b3f87525686652a2aae14fce7c8b7f255de2fd2e4189d6a275de88191b8aa) was parsed.
- Elements whose name contains HSN: two `HSNDETAILS.LIST`, both empty (no text, no children).
- No HSN code or applicability element is present.

## Step 1: reads, nothing changed
| Request | Answer | Bytes | sha256 (first 8) | Time |
|---|---|---|---|---|
| w14-company-object.xml | aw14-company-object-0.xml | 84,634 | 96416059 | 0.32 s |
| w14-company-fetch.xml | aw14-company-fetch-0.xml | 3,574 | 81cdb8b7 | 0.20 s |
| r02-marks.xml | aw14-marks-0.xml | 3,404 | 1e603b13 | |
| w13-status-window-aug.xml | aw14-status-window-0.xml | 86,790 | 0a9c623e | |
| w12-status-0002.xml | aw14-status-0002-0.xml | 64,950 | 1a69fcfc | |
| w14-status-0008.xml | aw14-status-0008-0.xml | 81,688 | 41085908 | |

**1. Object read.**
- Size and shape: under 500 KB and under 1 minute; ENVELOPE with BODY; 735 elements; no LINEERROR.
- The README expected "about 41 KB". This book's object is about twice that.
- **It carries the company-level HSN/SAC with its date:**
  - `HSNDETAILS.LIST`: APPLICABLEFROM `20260401`, HSNCODE `998341`, HSN `Geological and geophysical consulting services`, HSNCLASSIFICATIONNAME empty, SRCOFHSNDETAILS `Specify Details Here`.
  - This is the company HSN set in W11.
- Other company GST elements:
  - CMPTAXAMOUNTTOLERENCE `0`.
  - HSNAPPLICABILITY, GSTAPPLICABILITY and GSTAPPLICABLEDATE: all empty.
  - GSTCLASSIFICATION `0`; ISGSTCLASSIFON `No`.
  - `GSTDETAILS.LIST`: APPLICABLEFROM 20260401, empty HSN fields, SRCOFGSTDETAILS `Not Defined`.
  - `CMPGSTDETAILS.LIST`: APPLICABLEFROM 20260401, GSTSOURCEPRIORITY `Ledger`, HSNSOURCEPRIORITY `Ledger`, BASISOFCALC `Stock Item/Ledger-wise`, IGNOREROUNDINGTYPE `Greater or Lesser than Computed Tax`, HSNAPPLICABLESECTION `All Sections`, THRESHOLDLIMIT 0, HSNCODELENGTH 4.
  - `HSNSUMMARYDETAILS.LIST`: HSNSUMMARYAPPLFROM 20250501, ISHSNAPPLICABILITYON Yes.

**2. Fetch read.**
- COLLECTION/COMPANY returned only NAME, GUID, ISGSTCLASSIFON (`No`) and CMPTAXAMOUNTTOLERENCE (`0`).
- These did **not** come back: HSN applicability, GST classification, and the two HSN and GST detail lists.

**3. Other reads.**
- marks-0 is byte-identical to W13's last marks: ALTVCHID 57, ALTMSTID 243.
- The window shows no voucher change since W13's last read.
- 0002: ALTERID 11, uncertain. 0008: ALTERID 9, uncertain.

**4. Screens.** GSTR-1, 1-Aug-26 to 31-Aug-26:
- Total 35: included 24 (No Action Required 24), not relevant 6, uncertain 5.
- B2C (Small) 24 (2): taxable 10,506.70, CGST 308.20, SGST 308.20, tax 616.40.
- Uncertain reasons:
  - HSN/SAC invalid, mismatched, or not specified: 2 (No. 5, No. 6).
  - Tax mismatch: 3.
    - 0002: 1,001.00; expected 50.06, modified 50.05, difference 0.01 (SGST 25.03/25.02).
    - 0003: 50.06 / 50.04 / 0.02.
    - 0008: 1,001.00; 50.06 / 50.05 / 0.01 (SGST 25.03/25.02).
- **0002 and 0008 are uncertain for a tax mismatch, as expected.**

## Step 2: the tolerance — STOP
I looked in these places, changing nothing and quitting each with Ctrl+Q (screenshots `s2-look-*`):
- **Alter > Company GST Details** ("GST Rate and Other Details"):
  - HSN/SAC details; GST rate details; e-Way Bill thresholds.
  - Additional Configuration: HSN/SAC summary for All Sections; minimum HSN length 4; Show GST Advances No; Update GST Status of Vouchers after Master Alteration No; Set/Alter details for downloading GST Returns No.
  - Its More Details list has GST - HSN/SAC & Related Details (and History), GST Rate & Related Details (and History), and e-Way Bill Details.
  - No tolerance.
- **Alter > GST Registration > Rajasthan Registration** ("GST Details"): status, state, type, GSTIN, periodicity, connected GST, e-Invoice and e-Way Bill. No tolerance.
- **F11 Company Features** (show more and all features Yes):
  - Accounting, Inventory, Taxation, Online Access, Payroll, Others.
  - Its More Details list, with Show More, has GST Registration, Company GST Details, PAN/CIN, MSME Registration, and Credit Days for Micro & Small Parties (45).
  - No tolerance.
- F12 Configure screens were not opened. They are configuration, not company data.

A marks read after the search (`aw14-x-marks-after-tolerance-search.xml`, 3,404 B, 1e603b13) is byte-identical to marks-0, so the search changed nothing.

## Not done, and differences from the README
- Steps 6–12 (set the tolerance, re-check, re-save 0008, set it back) were **not done**, because of the step 2 STOP.
- One extra marks read was taken after the search.
- Screenshots and the backup stay on the lab PC, listed by hash.
- Pre-registered expectation (a), that the object read carries the company-level HSN/SAC, is **met**: code 998341, dated 20260401.
- Expectations (b) to (e) were not reached.
