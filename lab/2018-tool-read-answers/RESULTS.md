# Results: #2018 live, stock_by_batch through a build, 2 Nov 2026

**Every EXPECT holds, and the row does not fail on any count. No call errored, no `state` was partial, 14 lines across seven item-and-date comparisons equal Tally's Stock Query screen, and the marks are the same bytes before and after. The named stock is unchanged: Hydrochloric Acid 33pc `tied`, 816.667 Kgs, on its three lines.**

Build: head fix/2018-stock-by-batch 11dd9215, `bridge_mcp.exe` sha256 3beb5b0221a42486baf3f720c5fbd541b912225d2ca3016b59b9b7992243c2d9, equal to the package's smoke file, before and after. BRIDGE SHAPE LAB as left, TallyPrime 7.2 Silver, licensed, loaded alone. No write setting on. Marks V 112, M 292, sha256 c71056a0dcd9deb501b80363ae1dbc9f711e7659f01f491b074b22e992407c59 both times.

| step | call | `isError` | evidence | `state` | seconds |
|---|---|---|---|---|---|
| 1 | `list_companies` | false | | | 0.16 |
| 1 | `stock_summary` as of 2026-03-31 | false | complete | value_total_matched | 0.98 |
| 2 | `stock_by_batch` as of 2026-03-31, 11 items | false | complete | complete | 1.76 |
| 3 | `stock_by_batch` as of 2025-08-31, 11 items | false | complete | complete | 1.70 |
| 4 | the same as step 2, `mask_parties` on | false | complete | complete | 2.59 |

The book holds 11 items, within the limit of 20, so every call asked for all of them: that covers each class the row names without choosing. [verified]

## Step 2, as of 31 March 2026
`batch_wise` yes, the book's setting.

| item | state | lines |
|---|---|---|
| Hydrochloric Acid 33pc | tied, 816.667 Kgs | Main Location / Primary Batch 150.000; Main Location / H-2504-A 300.000; Factory Floor / H-2504-B 366.667 |
| Caustic Soda Flakes | tied, 400.000 Kgs | Main Location / Primary Batch 200.000; Main Location / B-2504-A 200.000 |
| Cleaning Kit A | tied, 12 Nos | Main Location / Primary Batch 25; Main Location / K-2504-A -13 |
| Soda Ash Light | tied, 1300.000 Kgs | Main Location / Primary Batch 1350.000; Factory Floor / Primary Batch -50.000 |
| Zero Stock Item | tied, -50.000 Kgs | Main Location / Primary Batch -50.000 |
| Carton Box Small | tied, 100 Box | Main Location / Primary Batch 100 |
| HDPE Drum 50L | tied, 40 Nos | Main Location / Primary Batch 40 |
| Sulphuric Acid 98pc | tied, 100.000 Kgs | Main Location / Primary Batch 100.000 |
| Cleaning Kit B, Descaler Concentrate, Label Roll | no_movement | no line |

- Items not kept in batches read `Primary Batch`, Soda Ash Light among them. Each default line carries `kind` default.
- A negative line under a godown: Soda Ash Light at Factory Floor, and a negative batch on Cleaning Kit A.
- No stock: `no_movement`, with no zero line. Tally's screen for Label Roll shows no line and closing `0 Nos`; the tool's `closing` reads `empty`.
- `Zero Stock Item` holds stock in spite of its name: -50.000 Kgs, tied. The book holds no item `no_stock`, so that state was not reached.

## Step 3, as of 31 August 2025
Each item's lines tie, none `differs`. All 11 items are equal to step 2's, field for field. That is the book and not a date ignored: Stock Query at 1-4-2025 to 31-8-2025 shows the same lines for the three items read, and these items' purchases and sales on those screens are all dated April 2025. A date that changes an item's stock was not exercised on this book. [verified for what the screens show]

## Step 4, `mask_parties` on
Restarted with `BRIDGE_AGENT_REDACTION=mask_parties`, by a client that allows that one setting. Item, godown and named-batch names read `Hy…pc`, `Ma…on`, `H-…-A`; none of those names appears in the answer. Every quantity, unit, state, closing and total equals step 2, and the GUIDs stay. [verified]

Two things stay unmasked, recorded and not judged: Tally's own `Primary Batch`, and the `company` block's name and number. The block is outside the result, and Bridge's other reads were not compared here.

## Against Tally's screen
Stock Query, Godown/Batch Details, period set by F2. `evidence/screen-lines.txt` holds what was read, and a script compared it with the answers: 7 comparisons, 14 lines, none differs.

| to | items | second pass |
|---|---|---|
| 31-3-2026 | Hydrochloric Acid 33pc, Soda Ash Light, Cleaning Kit A, Zero Stock Item; Label Roll for no line | Hydrochloric opened again after step 4 |
| 31-8-2025 | Hydrochloric Acid 33pc, Soda Ash Light, Zero Stock Item | Hydrochloric re-read from its saved capture |

Step 4's answer has the same quantities as step 2's, so the same screens stand for it, Hydrochloric read again after the call. Captures stay on the lab PC. So the two-pass reading covers Hydrochloric at both dates; the other items were read once at each date read.

## Left behind
Nothing was written. Tally's date unchanged, 20-Apr-2026. Company shut, no company loaded, one tally.exe on 9001. Both client sessions ended, exit 0.

**Confidence:** one book, one Tally, one sitting; every stock movement read is in April 2025: PARTIAL on dates.
