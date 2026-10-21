# Lab request: a new synthetic stock company whose opening stock has a matching capital entry (SETUP by hand, then READ-ONLY reads)

Why: the closing-stock work on #692 declines on BRIDGE TIEOUT C because C carries an opening difference of 1,000 (opening stock keyed with no matching capital). Whether the statements can be established on a book that does not have that difference cannot be tested on any captured book. This request builds one, changing exactly one thing against C: a capital entry of 1,000 Cr beside the 1,000 Dr of opening stock. Every file here is a read; every key is made by hand on the Tally screen.

## Rules for the whole sitting
- Synthetic company only, a NEW company named exactly `BRIDGE TIEOUT D`. Never C, never any other loaded company, no real names or numbers.
- Hand-keyed only: no import, no Bridge tool posting, nothing sent to Tally that writes. The files in `requests/` are reads.
- One step at a time; save every answer as received (BOM-less UTF-16LE) with its size and sha256 in the sitting's MANIFEST; a failed request is reported as failed, never as an empty answer; count structure, not substrings.
- Request files are UTF-16LE with a BOM exactly as here (MANIFEST.sha256). The header ID of every read equals its report or collection name; do not edit them, except that `m-marks-TEMPLATE` takes the company identifier (below).
- Voucher marks: the `m-marks` read (a `Company` collection row with ALTVCHID and ALTMSTID) is sent before and after every key step and every read group below; the two values must be equal around a read group and may change only around a key step, and only for `BRIDGE TIEOUT D`. Also read the company list once before K0 and once after the last step (the same read the earlier sitting called r00): every other company's marks must be unchanged.
- STOP rules (stop, write down what is on the screen, report, change nothing more): the name `BRIDGE TIEOUT D` already exists in the company list (do not pick another name); a dialog or prompt these steps do not describe; any other company's marks differ; a read fails; a total on the Tally screen differs from the expected one below by more than a recorded explanation. After a STOP, restore BK1 if any other company changed; do not restore if only `BRIDGE TIEOUT D` is affected (report it).

## K0, check and backup (BK1)
K0a. Read the company list; confirm `BRIDGE TIEOUT D` is NOT present. Save the list.
K0b. BK1: take a Tally backup of the data (whole data folder or each loaded company, as the lab usually does), record file name, size and sha256, and where it is kept. This is before the first key. Restore rule at the end: BK1 is restored only on a STOP that changed another company.

## D1, build the book and read it
K1. Create company `BRIDGE TIEOUT D`: books beginning 1-4-2025, financial year 1-4-2025 to 31-3-2026, base currency INR. Features (F11): accounting and inventory on, inventory integrated with accounts, batches off, godowns off, GST off, cost centres off; if a prompt is not about these, answer with the default and write it down. Read the company list to get the company's identifier; fill it into `requests/m-marks-TEMPLATE.utf16le.xml` where it says `@@GUID@@`, save that as `m-marks-D.utf16le.xml` with its sha256, and send it (the baseline marks). Screenshot of the Features screen.
K2. Unit `Nos`.
K3. Ledgers: `Lab Capital` under Capital Account with opening balance 1,000.00 Cr; `Customer` under Sundry Debtors; `Supplier` under Sundry Creditors; `Sales` under Sales Accounts; `Purchases` under Purchase Accounts. No other opening balance. (C had no capital ledger; this is the one change.)
K4. Stock item `Lab Gadget` directly under Primary, unit Nos, opening balance 10 Nos at 100.00 per Nos (value 1,000.00) as on 1-4-2025. Do not change the costing method; write down what the item screen shows for it.
K5. Purchase 5-5-2025: party Supplier, ledger Purchases, 10 Nos Lab Gadget at 120.00 (1,200.00).
K6. Purchase 5-6-2025: party Supplier, ledger Purchases, 10 Nos Lab Gadget at 140.00 (1,400.00).
K7. Sales 20-6-2025: party Customer, ledger Sales, 8 Nos Lab Gadget at 200.00 (1,600.00).
Send `m-marks-D` after each of K1 to K7 and note the values. Screenshots of Tally's own Profit & Loss and Balance Sheet for 1-4-2025 to 31-3-2026 and of the Stock Summary as on 31-3-2026 (the Balance Sheet screen must show no "Difference in opening balances").
Reads, in this order, `m-marks-D` before and after the group: d1-r1 (Profit and Loss, year), d1-r2 (Balance Sheet, year), d1-r3 (ledger collection, year), d1-r4 (group tree), d1-r5 (Stock Summary, year), d1-r6 (Profit and Loss, Jul-Sep 2025), d1-r7 (Balance Sheet, Jul-Sep 2025), d1-r8 (ledger collection, Jul-Sep 2025), d1-r9 (Stock Summary, 1 Apr to 30 Sep 2025), and the inventory flags are the `m-marks-D` rows themselves (ISINTEGRATED, ISINVENTORYON).
Expected on average costing (a record, not a pass/fail: write down every difference): year P&L: Sales 1,600; Opening Stock -1,000; Purchases -2,600; Closing Stock -2,640; Cost of Sales -960; P&L A/c 640. Balance Sheet: Lab Capital line 1,000; Current Liabilities 2,600; P&L A/c 640; Current Assets -4,240 (a sheet that balances). Ledger collection: Customer -1,600, Purchases -2,600, Sales 1,600, Supplier 2,600, Lab Capital 1,000 Cr, and NO Stock-in-Hand row. Stock Summary 22 Nos, 2,640.00. Jul-Sep: Opening Stock and Closing Stock -2,640, Sales and Purchases empty.
Pass for D1: every read answered, HTTP 200, saved with size and sha256; marks equal around each read group.

## D2, one more sale, then read
Before D2: BK2, a backup of `BRIDGE TIEOUT D` (name, size, sha256).
K8. Sales 15-10-2025: party Customer, ledger Sales, 5 Nos Lab Gadget at 250.00 (1,250.00). Send `m-marks-D` before and after. Screenshots of Tally's Profit & Loss and Balance Sheet for the year and for 1-10-2025 to 31-3-2026.
Reads, `m-marks-D` before and after the group: d2-r1 (Profit and Loss, year), d2-r2 (Balance Sheet, year), d2-r3 (ledger collection, year), d2-r4 (Stock Summary, year), d2-r5 (Profit and Loss, 1 Oct 2025 to 31 Mar 2026), d2-r6 (Balance Sheet, same), d2-r7 (ledger collection, same).
Expected on average costing (record every difference): year: Sales 2,850; Opening Stock -1,000; Purchases -2,600; Closing Stock -2,040 (17 Nos at 120); Cost of Sales -1,560; P&L A/c 1,290. 1 Oct to 31 Mar: Opening Stock -2,640 (22 Nos), Closing Stock -2,040, Sales 1,250, Purchases empty, Cost of Sales -600. Both opening and closing stock non-zero and different, with movement in the window.
Pass for D2: as D1.

## At the end
Read the company list again (every other company's marks unchanged). Restore BK1 only under the rule above. Leave `BRIDGE TIEOUT D` in place; report its identifier.
Not measured by this request: a non-integrated book, FIFO or last-purchase costing, an item inside a stock group, a window crossing two years, negative stock, godowns, batches.
