# Results: #2018 Stock Query for every item and three period ends, 28 Oct 2026. On TallyPrime 7.2 `Stock Query` with `STOCKITEMNAME` follows the period end. At 1 Apr 2025 Hydrochloric Acid 33pc returns only its opening line (Primary Batch 150 Kgs) and no purchase fields; Cleaning Kit A returns its opening 25 Nos; Zero Stock Item returns the empty envelope. Every item with stock answers with godown/batch/quantity lines equal to Tally's Stock Query screen and summing to the item's closing. Items kept without batches show the batch "Primary Batch". Negative stock is written "-50.000 Kgs". Items with no stock return the empty envelope. Reads only; the marks did not move

The sheet is `lab/2018-stock-query-items-periods` at 6b3eba0de66c1a32ff2a1b264bbb9f0745c7c4ba.
- **Sheet checks:** `SHA256.txt` 21 of 21; `gen_requests.py --check` ok.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, `verified_tuple`. Current date 20-Apr-2026 throughout.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after).
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed.
- **Marks:** 292 / 112 at step 1, and before and after the sends (`ledger_masters` compliance, `reused` false).
- **P0:** met. Zero Stock Item (-)50.000 Kgs on the Stock Summary; Soda Ash Light (-)50.000 Kgs under Factory Floor > Raw Chemicals on the Godown Summary.

## The answers (step 4)
Each answer is a flat ENVELOPE with no STATUS and no line error.

| request | bytes | sha256 | STQ content |
|---|---|---|---|
| r-00 Stock Summary | 1,370 | 862c3206... | three groups, as before |
| r-07 Hydrochloric Acid 33pc (year) | 1,354 | c23c6d41... (= earlier row) | purchase 24-Apr-25 Chem Supplier 2 666.667 Kgs 30.00 -20000.00; Main Location / Primary Batch 150.000; Main Location / H-2504-A 300.000; Factory Floor / H-2504-B 366.667 Kgs |
| **r-12 Hydrochloric Acid 33pc (to 1 Apr 2025)** | 320 | 6e8d7e5e... | Main Location / Primary Batch 150.000 Kgs only; no STQPURC element |
| **r-14 Cleaning Kit A (to 1 Apr 2025)** | 310 | b78d85c0... | Main Location / Primary Batch 25 Nos only; no STQSALES element |
| **r-13 Zero Stock Item (to 1 Apr 2025)** | 46 | 8d37111f... | empty envelope |
| r-02 Caustic Soda Flakes | 1,094 | ece7ccb5... | purchase 21-Apr-25 Chem Supplier 1 200.000 Kgs 50.00 -10000.00; Main Location / Primary Batch 200.000; Main Location / B-2504-A 200.000 Kgs |
| r-03 Cleaning Kit A | 1,086 | b4d01f26... | **sale** STQSALESDATE 23-Apr-25, STQSALESVCHNO Shape Buyer 1, 13 Nos, 750.00, 10000.00; Main Location / Primary Batch 25 Nos; Main Location / K-2504-A -13 Nos |
| r-10 Sulphuric Acid 98pc | 320 | 92214a04... | Main Location / Primary Batch 100.000 Kgs |
| r-09 Soda Ash Light | 3,152 | 592f19e8... | five purchases (11 to 15 Apr 25, Chem Supplier 4, 100.000 Kgs each at 25.00, -2500.00 each); Main Location / Primary Batch 1350.000 Kgs; Factory Floor / Primary Batch -50.000 Kgs |
| r-01 Carton Box Small | 312 | 53fe972f... | Main Location / Primary Batch 100 Box |
| r-06 HDPE Drum 50L | 310 | d87aefbc... | Main Location / Primary Batch 40 Nos |
| r-05 Descaler Concentrate | 46 | 8d37111f... | empty envelope |
| r-04 Cleaning Kit B | 46 | 8d37111f... | empty envelope |
| r-08 Label Roll | 46 | 8d37111f... | empty envelope |
| r-11 Zero Stock Item (year) | 848 | 83acc289... | **sale** 20-Apr-25 Shape Buyer 6 50.000 Kgs 30.00 1500.00; Main Location / Primary Batch -50.000 Kgs |
| r-15 Bridge Probe Item (control) | 46 | 8d37111f... | empty envelope |

**Classes:**
- **Batch lines:** Hydrochloric Acid 33pc, Caustic Soda Flakes, Cleaning Kit A.
- **One "Primary Batch" line:**
  - Sulphuric Acid 98pc;
  - items not kept in batches: Carton Box Small, HDPE Drum 50L, and Soda Ash Light, which has two godowns.
- **Empty envelope:** Descaler Concentrate, Cleaning Kit B, Label Roll (no stock), and Zero Stock Item at 1 Apr 2025.
- **Purchase or sale fields only:** none.

**Formats:**
- No triple has an empty godown or batch name.
- Negative quantities are written with a leading minus, where the screen writes "(-)".

**New elements, beside the earlier row:**
- the `STQSALES*` group, where `STQSALESVCHNO` holds the party name;
- repeated `STQPURC*` groups (Soda Ash Light).

**Period:**
- r-12 against r-07: the two purchase-batch lines and the purchase fields drop away, leaving the opening line.
- r-14 against r-03: the K-2504-A line and the sale fields drop away.
- r-13 against r-11: everything drops away.
- So the lines follow the period end. The purchase/sale fields are absent at the opening day, not still showing 24-Apr-25.

## Tally's Stock Query screens (step 5; F2 set, dialog captured)
| item, period | closing | Godown/Batch Details on screen | against the answer |
|---|---|---|---|
| Caustic Soda Flakes, year | 400.000 Kgs | Main / Primary Batch 200.000; Main / B-2504-A 200.000 | equal; sum 400 |
| Hydrochloric Acid 33pc, year | 816.667 Kgs | Primary Batch 150; H-2504-A 300; Factory / H-2504-B 366.667 | equal; sum 816.667 |
| Hydrochloric Acid 33pc, 1-4-2025 to 1-4-2025 | 150.000 Kgs, value (-)4,500.00; no last purchase | Main / Primary Batch 150.000 | equal to r-12 |
| Soda Ash Light, year | 1,300.000 Kgs (cost price 13.89/Kgs on this screen) | Main / Primary Batch 1,350.000; Factory Floor / Primary Batch (-)50.000; five purchase lines 11 to 15 Apr | equal; sum 1,300 |
| Sulphuric Acid 98pc, year | 100.000 Kgs | Main / Primary Batch 100.000 | equal |
| Zero Stock Item, year | (-)50.000 Kgs; last sold 20-Apr-25 Shape Buyer 6 50 Kgs @ 30.00 | Main / Primary Batch (-)50.000 | equal |
| Zero Stock Item, 1-4-2025 to 1-4-2025 | 0 Kgs | no lines | equal to r-13 (empty) |
| Cleaning Kit A, year | 12 Nos; last sold 23-Apr-25 Shape Buyer 1 13 Nos @ 750.00 | Main / Primary Batch 25; Main / K-2504-A (-)13 | equal; sum 12 |
| Cleaning Kit A, 1-4-2025 to 1-4-2025 | 25 Nos | Main / Primary Batch 25 | equal to r-14 |
| Carton Box Small, year | 100 Box | Main / Primary Batch 100 | equal |
| HDPE Drum 50L, year | 40 Nos | Main / Primary Batch 40 | equal |
| Cleaning Kit B, year (via Select Stock Item) | 0 Nos | no lines | consistent with r-04 (empty) |
| Descaler Concentrate, year (via Select Stock Item; group Finished Kits) | 0 Kgs | no lines | consistent with r-05 (empty) |

Label Roll's screen was not opened: its answer was empty, and the sheet does not list it.

**Confidence:** one observation per item, PARTIAL until repeated.

**Not covered:**
- item names with `&`, `/`, quotes or parentheses;
- a batch named as well;
- other units;
- value or expiry;
- a book that is not batch-wise;
- the tax-audit book.

## Files
- `requests/`: the sixteen requests as sent.
- `raw/`: every answer with its headers.
- `readans.py` and `raw-structure.txt`.
- `answers/`: tool answers 000 to 004.
- `send-log.txt`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
