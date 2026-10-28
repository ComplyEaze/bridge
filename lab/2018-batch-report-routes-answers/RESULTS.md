# Results: #2018 batch report routes, 28 Oct 2026. On TallyPrime 7.2 no batch-route screen title answered with data as a report ID in the Stock Summary request shape. "Stock Query" (both spellings) answered 200 with an empty envelope, not refused like the control. "Batch Vouchers", "Stock Item Monthly Summary" and "Stock Item Vouchers" were each refused in band like the control. Reads only; the marks did not move

The sheet is `lab/2018-batch-report-routes` at 1b07f8a1a4a1e7456d2cec066ba6baf18a902edb.
- **Sheet checks:** `SHA256.txt` 5 of 5; `make_report_request.py --check` ok.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, `verified_tuple`.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after; `ledger_masters` in `tools/list`).
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed. Current date 20-Apr-2026 throughout.
- **Marks:** 292 / 112 at step 1, before and after the raw reads (`ledger_masters` compliance, `reused` false).

## The routes (step 3; titles exactly as shown)
| route | screens on the way | report titles | batch-wise keys |
|---|---|---|---|
| 3 | Stock Summary > Stock Group Summary > Stock Item Monthly Summary > Change View > Stock Query | **Stock Query** | Change View offers Daily to Yearly views and Stock Query, Movement Analysis, Vouchers Details; Stock Query's side bar has none |
| 1 | Display More Reports > Inventory Books > Batch: "Batch Items" (selection; List of Batches shows no expiry) > H-2504-A | **Batch Vouchers**; F8 Batch-wise gives **Batch Summary** (already sent by the 2017/2018 row: excluded) | F8 Batch-wise; Batch Summary's Change View offers Stock Query, Monthly Summary, Movement Analysis, Vouchers Details |
| 2 | Inventory Books > Stock Item: "Select Stock Item" (selection) > Hydrochloric Acid 33pc; Enter on April | **Stock Item Monthly Summary**, **Stock Item Vouchers** | none; Change View offers Stock Query, Daily Breakup, Monthly Summary, Movement Analysis |

Screens as opened showed the current period (1-Apr-26 to 20-Apr-26 or to 30-Apr-26). F2 set 1-4-2025 to 31-3-2026.

| screen (FY 2025-26) | lines |
|---|---|
| Stock Query | Closing 816.667 Kgs, 15,500.00. Godown/Batch Details: Main Location / Primary Batch 150, Main Location / H-2504-A 300, Factory Floor / H-2504-B 366.667 (quantity only). Purchase 24-Apr-25 Chem Supplier 2, 666.667 Kgs @ 30.00, 20,000.00 |
| Batch Vouchers (H-2504-A) | 24-Apr-25 Chem Supplier 2, Purchase 21: in 300 Kgs, 9,000.00; closing 300 Kgs, 9,000.00 |
| Batch Summary | H-2504-A 300 @ 30.00, 9,000.00; H-2504-B 366.667 @ 30.00, 11,000.00; Primary Batch 150 @ 30.00, 4,500.00. Lines sum to 24,500.00; Grand Total 816.667 Kgs, 15,500.00 |
| Stock Item Monthly Summary | Opening 150 Kgs, (-)4,500.00; April in 666.667 Kgs, 20,000.00; closing 816.667 Kgs, 15,500.00 |
| Stock Item Vouchers | Opening 150 Kgs, (-)4,500.00; 24-Apr-25 Purchase 21, 666.667 Kgs, 20,000.00; closing 816.667 Kgs, 15,500.00 |

## The raw reads (step 4)
No spelling 2 was written, because no title holds `&` or `/`.

| request (ID) | answer |
|---|---|
| Stock Summary | 200, 1,370 B. Three groups: DSPCLAMTA 18750.00, 14500.00, -30249.99 (sum 3000.01); quantity and rate empty |
| Bridge Probe Unknown (control) | 200, 434 B, STATUS 0, "Could not find Report 'Bridge Probe Unknown'!" |
| Stock Query / StockQuery | 200, 46 B each, byte-equal, RESPSTATUS 1: an empty `<ENVELOPE></ENVELOPE>`, with no STATUS and no line error |
| Batch Vouchers / BatchVouchers | 200, STATUS 0, "Could not find Report '…'!" |
| Stock Item Monthly Summary / StockItemMonthlySummary | 200, STATUS 0, "Could not find Report '…'!" |
| Stock Item Vouchers / StockItemVouchers | 200, STATUS 0, "Could not find Report '…'!" |

After each send, Tally showed the Gateway with no dialog. All ten captures are byte-equal.

## Comparisons (step 6)
No title answered with data, so no lines, batch amounts, expiry or manufacturing elements were read to compare. "Stock Query" is the one title that is not refused like the control. The empty answer may mean the report needs an item chosen, and this sheet adds no variable. The screen's batch lines sum to 24,500.00 against the item's 15,500.00, the 9,000.00 gap the 2017/2018 row recorded.

**Confidence:** one observation, PARTIAL until repeated.

## Files
- `answers/`: tool answers 000 to 004.
- `reqs/`: the ten requests as written.
- `raw/`: every answer with its headers.
- `readans.py` and `raw-structure.txt`: the structure as read.
- `send-log.txt`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
