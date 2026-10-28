# Results: #2017 and #2018, 28 Oct 2026. On TallyPrime 7.2, Tally's godown-wise report is the screen "Godown Summary", and a request with that title as its ID answers with one closing line per godown that sums to the Stock Summary's total. The batch-wise report is the screen "Batch Summary" (reached from Batch Vouchers with F8), but the gateway does not know that title, with or without its space. Reads only

The sheet is `lab/2017-2018-stock-by-godown-batch` at 30e0ded5b6209fdfa84363abaa776bf192affd87.
- **Sheet checks:** `SHA256.txt` 5 of 5; `make_report_request.py --check` ok.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after).
- **Tally:** TallyPrime 7.2, Silver, Licensed.
- **Marks:** 292 / 112 at start, before the raw reads and after them. The book did not change.
- **Marks read:** `ledger_masters` basic refused on this book (`company_several_currency_masters`). Every marks read is `fields=compliance`, the read the refusal names.

## The tool's side (step 2)
- **`masters` kind `godowns`:** two godowns, Factory Floor (master_id 212) and Main Location (99). Each parent is given as "�#4; Primary".
- **`stock_summary` as_of 20260331:**
  - 11 items; state `value_total_matched`; `tie_out.total` 3000.01 matched.
  - 4 items have an empty closing value.
  - Inventory is integrated and batch-wise.

## Tally's screens (step 3), 1-Apr-25 to 31-Mar-26
- **Godown-wise report.** Path: Display More Reports > Inventory Books (Stock Item, Batch, Godowns, Stock Group Summary, Stock Category Summary, and two registers) > Godowns > Select Godown: Primary.
  - Title **"Godown Summary"**.
  - Factory Floor: 316.667 Kgs, 34.74, 11,000.00.
  - Main Location: (-)14,000.01, no quantity.
  - Grand Total (-)3,000.01.
  - Factory Floor drill-down → Raw Chemicals → Hydrochloric Acid 33pc 366.667 Kgs 11,000.00; Soda Ash Light (-)50.000 Kgs.
- **Batch-wise report.** Path: Inventory Books > Batch > Batch Items (item Hydrochloric Acid 33pc, batch H-2504-A) gives "Batch Vouchers"; its F8 "Batch-wise" gives the title **"Batch Summary"**:

| batch | quantity | rate | value |
|---|---|---|---|
| H-2504-A | 300.000 Kgs | 30.00 | 9,000.00 |
| H-2504-B | 366.667 Kgs | 30.00 | 11,000.00 |
| Primary Batch | 150.000 Kgs | 30.00 | 4,500.00 |
| Grand Total | 816.667 Kgs | | 15,500.00 |

  - No expiry or manufacturing date column. The batch list has an Expiry Date column, empty for all three.
- **Stock Summary:** Grand Total (-)3,000.01. Hydrochloric Acid 33pc 816.667 Kgs 15,500.00, of which the opening is 150 Kgs at **(-)4,500.00**.

## The raw reads (step 4), the Stock Summary request with only the ID changed
| ID | answer | |
|---|---|---|
| Stock Summary (control) | 200, 1,370 B | three groups, sum 3000.01 = the tool's tie-out |
| Bridge Probe Unknown (control) | 200, 434 B | STATUS 0, "Could not find Report 'Bridge Probe Unknown'!" |
| **Godown Summary** | **200, 968 B, data** | `DSPACCNAME/DSPDISPNAME` per godown, `DSPSTKINFO/DSPSTKCL` with `DSPCLQTY`, `DSPCLRATE`, `DSPCLAMTA` (see below). The list ends here. |
| Batch Summary | 200, 420 B | STATUS 0, "Could not find Report 'Batch Summary'!" |
| BatchSummary | 200, 418 B | STATUS 0, "Could not find Report 'BatchSummary'!" |

- **Godown Summary lines:**
  - Factory Floor: 316.667 Kgs, 34.74, -11000.00.
  - Main Location: quantity and rate empty, 14000.01.
- **Candidate (2)** (& → and, / dropped) is identical to (1) for both titles and was not sent separately. No dialog appeared at any send.

## Comparisons (step 6)
- **Godown:** the two godown amounts sum to 3000.01, equal to the Stock Summary answer and the tool's tie-out.
  - The raw lines equal Tally's Godown Summary screen, with the signs reversed as in the Stock Summary.
  - The answer carries godown lines only (no item or batch lines), so per-item godown amounts are not in it.
- **Batch:** no gateway answer under either title.
  - On screen, the batch values sum to 24,500.00 against the item's 15,500.00.
  - The difference is the item's opening 150 Kgs, valued (-)4,500.00 on the item and 4,500.00 as Primary Batch.
- **Expiry:** no date column in the Batch Summary. #2018's expiry question stays open.

**Confidence:** one observation, PARTIAL until repeated.

## Files
- `answers/` (tool, 000 to 007).
- `raw/`: every raw answer with headers.
- `reqs/`: the six requests written.
- `send-log.txt`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
