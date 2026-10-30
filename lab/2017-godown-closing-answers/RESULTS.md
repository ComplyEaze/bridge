# #2017 closing stock by godown: results

Sheet `lab/2017-godown-closing` at `6afef5d9` (LAB-COMMON v3). TallyPrime 7.2 Silver, synthetic book BRIDGE SHAPE LAB, the published 0.7.0 `bridge_mcp.exe` (sha256 `48d2f743...`, unchanged at the close) over one stdio launch. Every step ran; no stop rule was met. **Confidence: one observation, PARTIAL until repeated.** Amounts are written with the wire sign (held stock negative) unless marked "screen".

## What the row records

| Question | Result |
|---|---|
| Does any answer carry a value per item per godown? | **Yes, for items directly under the root only.** The exploded Stock Summary (`EXPLODEFLAG` Yes) follows each root item's line with one `SSBATCHNAME` element per godown (`SSBATCH` empty, `SSGODOWN` the godown), each followed by its own `DSPSTKINFO` with quantity, rate and amount. The exploded Godown Summary lists the root item under each godown as a line with quantity, rate and amount in that godown. Items inside a stock group get no godown line in any raw answer (Part A, and Part B for the same items). |
| On which valuation reading? | **Each godown at its own cost.** At 31-3-2026 Main Location's 22 Nos are 220.00 (22 x 10.00), not 242.00 (22 x 11.00, the item's average). The godown values sum to the item's own line: 88.00 + 250.00 + 220.00 = 558.00, and Tally's Stock Summary, Stock Query and `stock_summary` all value the item at 558.00 (rate 11.16), not 550.00. |
| Do the top-level godown amounts sum to the Stock Summary total? | **Yes, at both dates, before and after the keying.** Part A 3000.01 = 3000.01; Part B 2300.01 = 2300.01 (31-8-2025) and 2292.01 = 2292.01 (31-3-2026). |
| Is a child godown its own line, or folded into its parent? | **Folded in the plain report; its own line in the exploded one.** The plain Godown Summary (raw and screen) shows only Factory Floor and Main Location, and Main Location's amount includes Godown Lab C's. The exploded answer and the Detailed screen show Godown Lab C as a line under Main Location (20 Nos, 12.50, -250.00). Main Location's change against Part A is -550.00 at 31-8-2025 (300 + 250) and -470.00 at 31-3-2026 (220 + 250). |
| How does the transfer show? | Before it (31-8-2025): Main Location 30 at 10.00, Godown Lab C 20 at 12.50, item 550.00. After it (31-3-2026): Main Location 22 at 10.00, Factory Floor 8 at 11.00, Godown Lab C 20 at 12.50, item 558.00. Tally offered the Stock Journal's source rate as 10.00 (Main Location's own cost) and the destination rate as 11.00 (the source's typed rate); 11 was typed on both sides, so 8 Nos left at a cost of 80.00 and arrived at 88.00. |

## Part A (the book as it stood)

- Marks 292/112 throughout. `a-00`, `a-01` and `a-04` are byte-equal to the 28 Oct captures; the 31-8-2025 answers are byte-equal to the 31-3-2026 ones (no movement between those dates).
- `EXPLODEFLAG` Yes expands one level: Stock Summary groups into their items (item totals only), Godown Summary godowns into their stock groups (group totals per godown). No item line per godown.
- `a-04` (unknown report): `STATUS` 0, "Could not find Report 'Bridge Probe Unknown'!".
- Screens (Detailed, every level expanded) do show item, batch and godown lines with values. Drill-down titles: Enter on Factory Floor and then on Raw Chemicals both read exactly `Godown Summary`, so no drill-down candidate was sent.

## Part B (keyed, then restored)

| Step | Act | Marks after |
|---|---|---|
| 6 | Backup BK1 (102,253 B, sha256 `8fa3e3a4...`) | 292 / 112 |
| 7.1 | Godown `Godown Lab C` under `Main Location` | 293 / 112 |
| 7.2 | `Godown Split Item`, opening 50 Nos: Main Location 30 at 10, Godown Lab C 20 at 12.50 (550.00, 11.00) | 294 / 112 |
| 7.3 | `Godown Single Item`, opening Factory Floor 15 at 10 | 295 / 112 |
| 7.4 | Period 1-4-2025 to 31-3-2027, date 15-9-2025 | 295 / 112 |
| 7.5 | Stock Journal 15-Sep-2025: 8 Nos Main Location to Factory Floor at 11 | 295 / 113 |
| 7.6 | Period and date set back (Gateway = step 0's) | 295 / 113 |
| 8 | `b-01` to `b-12`, every answer 200, no dialog | 295 / 113 before and after |
| 9 | Screens; Stock Query EXPECTs all met | 295 / 113 |
| 12 | Restore BK1 | 292 / 112, godowns and `a-00`/`a-01` equal to Part A |

- Stock Query (raw `b-05`, `b-06`, `b-11`, `b-12` and the screen): Godown Split Item at 31-8-2025 Main Location 30, Godown Lab C 20; at 31-3-2026 Main Location 22, Factory Floor 8, Godown Lab C 20; Godown Single Item Factory Floor 15 at both dates. Raw and screen lines are equal; the raw answers carry quantity only (no rate or value).
- `masters` kind `godowns` after the keying: three godowns; `Godown Lab C`'s parent is written `Main Location`. After the restore: the two of Part A with the same master_ids.
- `stock_summary` after the keying: 13 items, `tie_out` matched (2300.01 and 2292.01), one value per item and none per godown.

## Recorded alongside

- `parse5.py` read `b-03`/`b-09` wrongly (it folded each `SSBATCHNAME` sub-line's values into the item line). `parse6.py` replaces it for every Part B answer; Part A's answers have no `SSBATCHNAME` and read the same under both.
- At step 9, in the Stock Summary's F12 Configure, one Down too many moved the Type of Grouping list and an Enter accepted "Stock Item-wise"; it was set back to "Stock Group-wise" before anything else was read. A report setting only.
- `rawa.py` gained a label suffix (`RAWA_SUFFIX`) so the step 12 re-sends of `a-00`/`a-01` are kept beside the step 3 answers (`-r12`), not over them.
- The F2 date box offered 21-4-2026 at step 7.4 and 2-4-2026 at step 7.6, not the Gateway's date; the dates typed were the sheet's. Changing the period at step 7.6 moved the current date to 1-Apr-2026 by itself before F2 set it to 20-4-2026.
- The Stock Query screen opens on the Stock Summary's period (F2 offered 1-4-2025 to 31-3-2026), not the current period.

## Not covered

As the sheet lists: an item inside a stock group in more than one godown (except through Stock Query), more than one level of godowns, a batch-kept item split over godowns, a compound unit, another costing method, another release, the tax-audit book.

## Evidence

`notes.txt` (one line per step, UTC), `answers/` (every raw answer with headers and the parsed reading), `bridge/` (every tool answer, `company_number` redacted), `evidence/` (send log, tools log, marks, C2 at step 0 and at the close, backup record, request checks), `restore-plan.txt`, `scripts/`, `SCREENSHOTS.txt` and `kept-local.txt` (by size and sha256; never pushed), `substitutions.txt`, `SHA256.txt`.
