# Results: #2018 row R3, `stock_by_batch` as of a past date, 3 Nov 2026

**CLOSING. As of 30 April 2026, with a window of 1 to 30 April, the tool answers the 13 lines of the book's stock, each equal to Stock Query for that date and to the first row's step 2. No item's closing moved in that window, so lines of movement alone would have been empty.**

**One EXPECT does not hold as written: steps 2 and 4 are not equal apart from read time. `evidence/bytes` and `evidence/response_sha256` differ too. The cause is named below, and every row and every Stock Query answer is equal.**

Build: head 0d1b704a on fix/2018-stock-by-batch, `bridge_mcp.exe` sha256 6c2e270490d7a68835221679f0c9760041b66fe86fbaccd02e1873893743d70c, equal to the package's smoke file, before and after. Artifact digest equal. BRIDGE SHAPE LAB, synthetic, the book and 11 items of the first row's step 2, TallyPrime 7.2 Silver, licensed, loaded alone. Posting off.

The build ran through a loopback relay on 9002 that passes every byte and saved each request and answer. No request was changed; no answer was held.

## Steps
| step | call | `isError` | evidence | `state` | seconds |
|---|---|---|---|---|---|
| 1 | marks, raw | | | | |
| 2 | `stock_by_batch`, 2026-04-30, 11 items | false | complete | complete | 1.84 |
| 3 | Stock Query screens, 1-4-2026 to 30-4-2026, 11 items | | | | |
| 4 | step 2 again | false | complete | complete | 1.74 |
| added | `vouchers`, 1 to 30 April 2026 | false | complete | complete | 2.22 |
| 5 | marks, raw | | | | |

Marks V 112, M 292; steps 1 and 5 are the same bytes. [verified]

`period` in the answer reads from 20260401 to 20260430. The 22 Stock Query requests as sent carry the same two dates. [verified]

## Item by item
Counted from the saved answer: 11 items, 13 lines. Stock Query reads 13 lines. The screens were read once.

| item | tool | row 2 | Stock Query |
|---|---|---|---|
| Carton Box Small | 100 | 100 | 100 |
| Caustic Soda Flakes | 400.000 | 400.000 | 400.000 |
| Cleaning Kit A | 12 | 12 | 12 |
| Cleaning Kit B | no line, `no_movement` | no line, `no_movement` | no line, closing 0 |
| Descaler Concentrate | no line, `no_movement` | no line, `no_movement` | no line, closing 0 |
| HDPE Drum 50L | 40 | 40 | 40 |
| Hydrochloric Acid 33pc | 816.667 | 816.667 | 816.667 |
| Label Roll | no line, `no_movement` | no line, `no_movement` | no line, closing 0 |
| Soda Ash Light | 1300.000 | 1300.000 | 1300.000 |
| Sulphuric Acid 98pc | 100.000 | 100.000 | 100.000 |
| Zero Stock Item | -50.000 | -50.000 | -50.000 |

Each batch line is equal too, godown, batch and quantity, against both: `evidence/check.txt`, differences 0, against `evidence/screen-lines.txt` and the first row's answer. [verified]

## Which it is
The window holds one voucher: Purchase 1, 20 April 2026, 1,234.00, Chem Supplier 1, to `Purchase - Local`. The `vouchers` tool does not show stock lines, so that voucher's own stock is not read. All 11 closings equal those of 31 March, so it moved none on balance, and the Purchases and Sales blocks of all 11 Stock Query screens are empty for the period. [verified]

So nothing, read this way, moved inside the window, and the tool still answers Hydrochloric Acid 33pc at 816.667 on three lines, and Carton Box Small at 100, which shows no last purchase or sale at all. Those are closings. [verified]

Not separated by this row: a voucher with stock inside the window. The window held none, and the row is reads only.

## Step 4
Not equal as saved. The differences are `read_at` (two places), `evidence/duration_ms`, and two that are not read time:

| key | step 2 | step 4 |
|---|---|---|
| `evidence/bytes` | 63,772 | 65,884 |
| `evidence/response_sha256` | a836189d… | 77f73d1e… |

Named by the relay's bytes: the paired read `Bridge Audit Stock Items`, the same request bytes in both calls, answers 16,946 B in step 2 and 18,002 B in step 4. The only change is one added line `<LANGUAGEID TYPE="Number">0</LANGUAGEID>` on each of the 11 items: 1,056 B a read, 2,112 B over the pair. All 11 Stock Query answers are byte-equal between the steps. `result`, with its rows, is equal. [verified]

That is the first read after the book loads, the effect row #2160-L1 recorded, here on stock items. The first row's own two calls differ by the same 2,112 B. That the load is the cause is read from those two rows and not tested here by a third call: hypothesis. Stock Query's screens were opened between steps 2 and 4, so they are not excluded as the trigger.

## Left behind
Nothing was written. Tally's date unchanged, 20-Apr-2026. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** one synthetic book, one sitting, no stock voucher inside the window: PARTIAL.
