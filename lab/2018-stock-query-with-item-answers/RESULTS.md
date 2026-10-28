# Results: #2018 Stock Query with the item named, 28 Oct 2026. On TallyPrime 7.2 `Stock Query` with the static variable `STOCKITEMNAME` set to `Hydrochloric Acid 33pc` answered with data. It returned the item's last purchase and one godown / batch / quantity line per batch, equal to Tally's screen. `SVSTOCKITEM`, the unknown variable `BRIDGEPROBEVAR` and an unknown item under `STOCKITEMNAME` each answered with the same empty envelope as the bare request. Reads only; the marks did not move

The sheet is `lab/2018-stock-query-with-item` at ec7e721ccfaa3466966b69ddb3de5186602c315f.
- **Sheet checks:** `SHA256.txt` 12 of 12; `gen_requests.py --check` ok.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, `verified_tuple`. Current date 20-Apr-2026 throughout.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after).
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed.
- **Marks:** 292 / 112 at step 1, and before and after the sends (`ledger_masters` compliance, `reused` false).

## The screen (step 2)
"Stock Query", item "Hydrochloric Acid 33pc", F2 1-4-2025 to 31-3-2026:
- **Closing:** 816.667 Kgs, 15,500.00.
- **Purchase:** 24-Apr-25, Chem Supplier 2, 666.667 Kgs @ 30.00, 20,000.00.
- **Godown/Batch Details:**

| godown | batch | quantity |
|---|---|---|
| Main Location | Primary Batch | 150.000 Kgs |
| Main Location | H-2504-A | 300.000 Kgs |
| Factory Floor | H-2504-B | 366.667 Kgs |
| total | | 816.667 Kgs |

## The sends (step 3)
| request | answer | sha256 |
|---|---|---|
| r-1 Stock Summary | 200, 1,370 B; DSPCLAMTA 18750.00, 14500.00, -30249.99 | 862c3206... |
| r-2 Bridge Probe Unknown | 200, 434 B; STATUS 0, "Could not find Report 'Bridge Probe Unknown'!" | 09b30fb7... |
| r-3 Stock Query | 200, 46 B; empty envelope, RESPSTATUS 1 | 8d37111f... |
| **r-4 Stock Query + STOCKITEMNAME = Hydrochloric Acid 33pc** | **200, 1,354 B, RESPSTATUS 1; lines** | c23c6d41... |
| r-5 Stock Query + SVSTOCKITEM = Hydrochloric Acid 33pc | 200, 46 B, byte-equal to r-3 | 8d37111f... |
| r-6 Stock Query + BRIDGEPROBEVAR (control) | 200, 46 B, byte-equal to r-3 | 8d37111f... |
| r-7 Stock Query + STOCKITEMNAME = Bridge Probe Item (control) | 200, 46 B, byte-equal to r-3; no refusal | 8d37111f... |

After each send, Tally showed the Gateway with no dialog. All seven captures are byte-equal.

## r-4 by structure (step 4)
The answer is a flat ENVELOPE with no STATUS or line error. Its elements, in order:
- `STQPURCDATE` 24-Apr-25, `STQPURCPARTY` Chem Supplier 2, `STQPURCVCHQTY` 666.667 Kgs, `STQPURCVCHRATE` 30.00, `STQPURCVCHDISC` empty, `STQPURCVCHAMOUNT` -20000.00;
- three repeats of `STQGODOWNNAME`, `STQBATCHNAME`, `STQGODOWNQTY`:
  - Main Location / Primary Batch / 150.000 Kgs;
  - Main Location / H-2504-A / 300.000 Kgs;
  - Factory Floor / H-2504-B / 366.667 Kgs.

**Against the screen:**
- All three godown, batch and quantity lines are equal, and so is the purchase line.
- The batch quantities sum to 816.667 Kgs, the item's closing.
- The Factory Floor line for this item is 366.667 Kgs.

**Absent:**
- per-batch rate or value, so the 11,000.00 Factory Floor value and the 9,000.00 value gap are not compared;
- any expiry or manufacturing date element;
- any echo of the item's name;
- any closing or total element.

**Controls:**
- `SVSTOCKITEM` and `BRIDGEPROBEVAR` change nothing.
- An unknown item under `STOCKITEMNAME` gives the empty answer, with no refusal that names the variable.

**Confidence:** one observation, PARTIAL until repeated.

**Not covered:**
- other variable names;
- a batch named as well;
- a period other than the year;
- whether the answer depends on the period variables;
- a collection route;
- the item masters' opening batch allocations;
- the tax-audit book.

## Files
- `requests/`: the seven requests as sent.
- `raw/`: every answer with its headers.
- `readans.py` and `raw-structure.txt`.
- `answers/`: tool answers 000 to 004.
- `send-log.txt`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
