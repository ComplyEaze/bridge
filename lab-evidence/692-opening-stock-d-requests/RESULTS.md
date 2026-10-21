# Results: BRIDGE TIEOUT D, built by hand and read (21 Oct 2026)

Windows lab PC, TallyPrime 7.1 Silver (licensed; every company-list read reports Education mode No), XML gateway on port 9001. Every key was made by hand on the Tally screen; nothing was imported or posted. Request files were checked against `MANIFEST.sha256` (17/17, sha256 and size) before the sitting. Answers are saved as received (BOM-less UTF-16LE) in `answers/`; every request, its HTTP status, size and sha256 is in `send-log.txt` in send order; sizes and sha256 of everything added here are in `ANSWERS.sha256`.

New company: **BRIDGE TIEOUT D**, company number 100075, GUID `6a9518b2-a88c-417c-a700-40243f9d9119`, books from 1-Apr-2025. It is left in place, loaded alone.

## Added request files
- `requests/m-marks-D.utf16le.xml`: `m-marks-TEMPLATE` with `@@GUID@@` replaced by the GUID above, nothing else changed (1,638 bytes, BOM kept).
- `requests/r00-company-list.utf16le.xml`: the earlier sitting's `r00` company-list read, byte-identical (sha256 9df2a53f…, 2,524 bytes). It lists loaded companies only and carries no marks, so "every other company unchanged" is also shown by a listing of the data folder taken before K0 (nothing loaded) and after the last read: per company folder the file count, bytes, newest modification time and a hash of the name/size/time listing (`data-folders-before.txt`, `data-folders-after.txt`). The two differ only by the new line for folder 100075; all 75 other company folders are unchanged.

## K0
- K0a: r00 before (only the previous lab company loaded). The Select Company list, filtered "TIEOUT", shows BRIDGE TIE OUT LAB 1/2/3 and BRIDGE TIEOUT A, B, C, and no BRIDGE TIEOUT D (`shots/k0a-no-tieout-d.png`).
- K0b: BK1 with no company loaded: one tar of the whole Tally data folder (815,288,320 bytes, sha256 8fd1e3afb6f59cca9243fdd40836837c3705201b7eac5717abae8097bf9a42ff, 1,724 files). It stays on the lab PC. It was not restored: no stop rule was met and no other company changed.

## D1 key steps and marks (`m-marks-D` after each step)
| Step | What was keyed | ALTVCHID | ALTMSTID |
|---|---|---|---|
| K1 | Company created; F11 GST changed Yes -> No (default was Yes); Accounts, Inventory, Integrate Yes; Batches, Cost Centres No; no godown option is offered on the Features screen even with "Show all features" (a view toggle) set Yes; Bill-wise left at its default Yes | (empty) | 206 |
| K2 | Unit Nos | (empty) | 207 |
| K3 | Lab Capital (Capital Account, opening 1,000.00 Cr), Customer, Supplier, Sales, Purchases | (empty) | 212 |
| K4 | Lab Gadget under Primary, Nos, opening 10 Nos @ 100.00 = 1,000.00. The item creation screen shows no costing-method field (nothing changed) | (empty) | 213 |
| K5 | Purchase 5-May-2025, Supplier / Purchases, 10 Nos @ 120.00 = 1,200.00 | 1 | 213 |
| K6 | Purchase 5-Jun-2025, Supplier / Purchases, 10 Nos @ 140.00 = 1,400.00 | 2 | 213 |
| K7 | Sales 20-Jun-2025, Customer / Sales, 8 Nos @ 200.00 = 1,600.00 | 3 | 213 |

The `m-marks-D` rows read ISINTEGRATED Yes, ISINVENTORYON Yes, ISBATCHWISEON No throughout. K1 also showed "Company created successfully. (Enable the features as per your business needs.)", the only prompt of the build. One slip, before any save: a click meant for the Company Name field landed in Company Data Path and cleared it; the path was picked again (the standard data folder) before the company was accepted.

Tally screens, 1-Apr-2025 to 31-Mar-2026 (`shots/d1-*`): P&L Opening Stock 1,000.00, Purchase Accounts 2,600.00, Sales Accounts 1,600.00, Closing Stock 2,640.00, Nett Profit 640.00. Balance Sheet as at 31-Mar-2026: Capital Account 1,000.00, Current Liabilities 2,600.00, Profit & Loss A/c 640.00, Current Assets 4,240.00, totals 4,240.00 each side, and **no "Difference in opening balances" line**. Stock Summary: Lab Gadget 22 Nos @ 120.00 = 2,640.00.

## D1 reads (marks 3 / 213 before and after the group: equal)
| Read | Bytes | What the answer holds | Against expected |
|---|---|---|---|
| d1-r1 P&L year | 1,622 | Sales Accounts 1600.00; Cost of Sales -960.00; Opening Stock -1000.00; Purchase Accounts -2600.00; Closing Stock -2640.00 | equal, except that the answer has no "P&L A/c" leaf (the expected list names P&L A/c 640) |
| d1-r2 BS year | 1,848 | Capital Account 1000.00; Loans (Liability) empty; Current Liabilities 2600.00; Profit & Loss A/c 640.00; Current Assets -4240.00 | equal |
| d1-r3 ledgers year | 11,130 | Customer -1600; Lab Capital opening and closing 1000.00; Purchases -2600; Sales 1600; Supplier 2600; Cash 0; Profit & Loss A/c 0.00; no Stock-in-Hand row | equal |
| d1-r4 group tree | 54,282 | 28 groups, each with BRIDGECOMPANYGUID 6a9518b2-… | answered |
| d1-r5 Stock Summary year | 510 | Lab Gadget 22 Nos @ 120.00, -2640.00 | equal |
| d1-r6 P&L Jul-Sep | 1,578 | Opening Stock -2640.00; Closing Stock -2640.00; Sales, Purchases and Cost of Sales empty | equal |
| d1-r7 BS Jul-Sep | 1,848 | byte-identical to d1-r2 | - |
| d1-r8 ledgers Jul-Sep | 11,062 | openings: Customer -1600, Lab Capital 1000, Profit & Loss A/c 640.00, Supplier 2600; Sales and Purchases 0 with no movement; no Stock-in-Hand row | - |
| d1-r9 Stock Summary 1 Apr-30 Sep | 510 | byte-identical to d1-r5 (22 Nos, 2640) | - |

D1: PASS (every read HTTP 200, saved with size and sha256, marks equal around the group).

## D2
- BK2 before K8: Tally backup of BRIDGE TIEOUT D, `TDBK1800_100075.001`, 28,812 bytes, sha256 76b1266c678e318854381d35e96eec17543986c712a73d54364b83dfc0e07cf5 (kept on the lab PC).
- K8: Sales 15-Oct-2025, Customer / Sales, 5 Nos @ 250.00 = 1,250.00. Marks 3 / 213 before, 4 / 213 after.
- Tally screens (`shots/d2-*`). Year: P&L Opening Stock 1,000.00, Purchases 2,600.00, Sales 2,850.00, Closing Stock 2,040.00, Nett Profit 1,290.00; Balance Sheet Capital 1,000.00, Current Liabilities 2,600.00, P&L A/c 1,290.00, Current Assets 4,890.00, totals equal, no difference line. 1-Oct-2025 to 31-Mar-2026: P&L Opening Stock 2,640.00, Purchases empty, Sales 1,250.00, Closing Stock 2,040.00, Nett Profit 650.00; Balance Sheet P&L A/c 1,290.00 split Opening Balance 640.00 and Current Period 650.00.

## D2 reads (marks 4 / 213 before and after the group: equal)
| Read | Bytes | What the answer holds | Against expected |
|---|---|---|---|
| d2-r1 P&L year | 1,624 | Sales 2850.00; Cost of Sales -1560.00; Opening Stock -1000.00; Purchases -2600.00; Closing Stock -2040.00 | equal (again no P&L A/c leaf; expected names 1,290) |
| d2-r2 BS year | 1,850 | Capital Account 1000.00; Current Liabilities 2600.00; Profit & Loss A/c 1290.00; Current Assets -4890.00 | answered |
| d2-r3 ledgers year | 11,130 | Customer -2850; Lab Capital 1000; Purchases -2600; Sales 2850; Supplier 2600; Profit & Loss A/c 0.00; no Stock-in-Hand row | answered |
| d2-r4 Stock Summary year | 510 | Lab Gadget 17 Nos @ 120.00, -2040.00 | equal |
| d2-r5 P&L Oct-Mar | 1,606 | Sales 1250.00; Cost of Sales -600.00; Opening Stock -2640.00; Purchases empty; Closing Stock -2040.00 | equal |
| d2-r6 BS Oct-Mar | 1,850 | byte-identical to d2-r2 | - |
| d2-r7 ledgers Oct-Mar | 11,106 | Customer opening -1600, debit -1250, closing -2850; Sales credit 1250; Profit & Loss A/c opening 640.00; Lab Capital 1000; Supplier 2600 | - |

D2: PASS.

## At the end
r00 again: BRIDGE TIEOUT D loaded alone, Education mode No. Data-folder listing: only folder 100075 is new; every other company folder is unchanged. No stop rule was met, so BK1 was not restored. No Education mode or licence dialog appeared.
