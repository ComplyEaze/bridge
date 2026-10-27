# Results: #692 NONINT, a stock company with inventory not integrated with accounts, TallyPrime 7.2, 28 Oct 2026

The request is `lab-evidence/692-nonint-lab-requests/README.md` at f22e68d9a482be557445eb6744c696cbadba42b9 (MANIFEST.sha256: 10 of 10 equal; request-ID census ok 3, skipped 3, FAIL 0).

## How the book was built
- **K0 and K1 were done on 21 Oct 2026** on TallyPrime 7.1. That sitting created `BRIDGE NONINT LAB` (company number 100076, GUID 567653b1-c8be-4deb-b4fc-3ea5de801884) and stopped before its Features were set, leaving Tally's defaults: Integrate Accounts with Inventory Yes, GST Yes.
- **K1 was completed on 28 Oct (21 Oct left Tally's defaults).** On TallyPrime 7.2 the company opened with no prompt. BK2 was taken (`TDBK1800_100076.001`, 22,954 bytes, sha256 4a740cb99d50355da4976e9dbf8bedfc079ab02eb9647189866ebfc3ffa0e928; it stays on the lab PC). Then Integrate Accounts with Inventory and GST were set to No on the Features screen, and nothing else was changed. The marks read ISINTEGRATED Yes → **No** and ALTMSTID 205 → 206.
- **K2 to K6 were keyed on 28 Oct on 7.2.** The masters were the unit Nos; the ledgers Customer, Supplier, Sales, Purchases, Lab Capital (1,000.00 Cr) and Closing Stock under Stock-in-Hand (1,000.00 Dr); and the item Lab Gadget (10 Nos at 100.00). The vouchers were a Purchase of 1,200.00 on 5 May 2025 and a Sales of 1,600.00 on 20 June 2025, both in Accounting Invoice mode.
- **K6, closing stock.** A Journal does not offer the Stock-in-Hand ledger in this mode, so none was keyed. Tally's own screen offers the Stock-in-Hand ledger's **Closing Balance** field instead, and 1,400.00 Dr was entered there as of 31 March 2026. On the inventory side a **Stock Journal** (destination only: Lab Gadget 2 Nos at 100.00) was accepted without integration. The item therefore shows 12 Nos at 100.00 = 1,200.00.
- Tally's Party Details and Bill-wise Details sub-screens were left on their defaults: New Ref, named by the voucher number. The full steps are in `notes.txt`.

## The reads against Tally's screens (1 April 2025 to 31 March 2026)
All five reads returned HTTP 200 and the marks were byte-equal before and after: ALTVCHID 3, ALTMSTID 215, **ISINTEGRATED No**, ISINVENTORYON Yes.

| | answer | Tally's screen |
|---|---|---|
| Profit and Loss (n-r1) | Sales 1,600.00; Cost of Sales 800.00 = Opening Stock 1,000.00 + Purchase Accounts 1,200.00 − **Less: Closing Stock 1,400.00** | Opening Stock 1,000.00; Purchase Accounts 1,200.00; Sales 1,600.00; Closing Stock 1,400.00; Nett Profit 800.00 |
| Balance Sheet (n-r2) | Capital 1,000.00; Current Liabilities 1,200.00; P&L A/c 800.00; Current Assets 3,000.00 (**balances**) | the same; totals 3,000.00 = 3,000.00 |
| Ledger collection (n-r3) | 8 ledgers; `Closing Stock` under Stock-in-Hand with TBALOPENING -1000.00, **TBALCLOSING -1000.00**, DEBITTOTALS -400.00 | (the ledger's Closing Balance field holds 1,400.00 Dr at 31-Mar-26) |
| Group tree (n-r4) | 28 groups, Stock-in-Hand among them | |
| Stock Summary (n-r5) | Lab Gadget 12 Nos at 100.00 = **1,200.00** | Lab Gadget 12 Nos, 100.00, 1,200.00 |

- **Opening Stock and Less: Closing Stock lines.** The Profit and Loss answer has both. Their figures come from the Stock-in-Hand ledger: opening 1,000.00, closing 1,400.00 from the ledger's Closing Balance field. They do not come from the item, whose value is 1,200.00. The answer and the screen agree.
- **FINDING: the `Closing Stock` ledger in the ledger collection.** It is present, but its `TBALCLOSING` reads -1000.00, its opening value. The 1,400.00 entered in its Closing Balance field does not reach that field; the 400.00 difference appears only as `DEBITTOTALS` -400.00. A reader that takes TBALCLOSING as the Stock-in-Hand closing value would get 1,000.00 where the statements use 1,400.00.
- **The Balance Sheet answer balances:** 1,000 + 1,200 + 800 = 3,000.
- **Stock Summary against the Stock-in-Hand ledger.** The Stock Summary closing value (1,200.00) does **not** equal the Stock-in-Hand ledger's closing (1,400.00). With integration off, the item and the ledger are kept apart. This is the case the request asked to capture.

## Not measured
The items the request lists: GST, batches, godowns, costing methods other than the default, and a window other than the year. Also, K0 and K1 ran on 7.1 on 21 Oct, so this row does not show a company created with these Features set from the start on 7.2.

## Files
- `answers/`: every answer as received, with its response headers. The marks reads are `k1check`, `k1-m-before`, `k1-m-after`, `k2`… `k6b-m-after`, `n-m-before` and `n-m-after`; the reads are `n-r1` to `n-r5`.
- `requests/m-marks-N.utf16le.xml`: the marks template with the company GUID filled in.
- `send-log.txt`: each curl send, with the request's sha256 and the answer's file, bytes and sha256.
- `notes.txt`: one line per step, in UTC.
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256. The captures, BK1, BK2 and the data-folder listings stay on the lab PC.
