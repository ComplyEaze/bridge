# Lab request: five experiments on BRIDGE TIEOUT D, each from the same starting state (hand-keyed, then READ-ONLY reads)

Why: the book `BRIDGE TIEOUT D` (opening stock with a matching capital entry, one item, one October sale) is the one book on which the statements add up. Before anything is built on it, five things that real books do and D does not are tried one at a time, each from the same starting state, so that a difference can only come from the one change. The expected figures below are a record to compare, not a pass/fail.

## Rules for the whole sitting
- Only `BRIDGE TIEOUT D`, which is already loaded. Never any other company. No real names or numbers.
- Hand-keyed only: no import, no Bridge tool posting, nothing that writes except your own keys on the Tally screen. The files in `requests/` are reads.
- One step at a time. Save every answer as received (BOM-less UTF-16LE) with size and sha256; a failed request is reported as failed, never as an empty answer. Request files are UTF-16LE with a BOM exactly as here (`shasum -c MANIFEST.sha256`); do not edit them except `m-marks-TEMPLATE` (company identifier at `@@GUID@@`, saved as `m-marks-D`).
- Marks: send `m-marks-D` before and after every key step and every read group; equal around a read group; changing only around a key step.
- No folder listings and no file counts in anything committed.
- STOP (stop, write down the screen, report, change nothing more): a prompt these steps do not describe; a restore (R) that does not bring the book back to the starting figures below; any read fails.

## Starting state (S) and the way back
S is D after its October sale: Lab Gadget 17 Nos at 120.00 (2,040.00); year P&L Sales 2,850, Opening Stock 1,000, Purchases 2,600, Closing Stock 2,040, nett 1,290; Balance Sheet Capital 1,000, Current Liabilities 2,600, P&L A/c 1,290, Current Assets 4,890.
BK3: before anything else, a Tally backup of `BRIDGE TIEOUT D` alone (name, size, sha256).
R: restore BK3 (this company only) before experiments X2 to X5, and after X5. After each R read the year P&L Closing Stock and Current Assets, and send `m-marks-D`: they must equal S, or STOP.

## X0, July to September 2025 on S (no key)
Reads, marks around: j-r1 (Profit and Loss, 1 Jul to 30 Sep 2025), j-r2 (Balance Sheet, same), j-r3 (ledger collection, same), j-r4 (Stock Summary, 1 Apr to 30 Sep 2025).
Record: is the Balance Sheet the same bytes as the year's (an "as at 31 March" answer) or does it show the position at 30 September (Current Assets then 4,240, P&L A/c 640)?

The experiments X1 to X5 each use the five reads g-r1 (Profit and Loss, year), g-r2 (Balance Sheet, year), g-r3 (ledger collection, year), g-r4 (Stock Summary, year), g-r5 (Negative Stock, year), marks around the group, saved under the experiment's name (for example `x1-g-r1`). Take them also once on S (name `x0s-g-r1` etc.) before X1.

## X1, an item sold before it was bought (negative stock)
Key: stock item `Lab Short` under Primary, unit Nos, no opening. Sales 10-7-2025: Customer / Sales, 3 Nos Lab Short at 150.00 (450.00), no purchase of it. Screenshot of Tally's Stock Summary (the item shows a negative quantity) and Negative Stock.
Expected to record: Negative Stock lists `Lab Short` (-3 Nos); P&L, Balance Sheet, ledgers: write down what Closing Stock, Current Assets and the nett profit become, and whether the Balance Sheet still balances.

## X2 (after R), the only item sold out
Key: Sales 20-11-2025, Customer / Sales, 17 Nos Lab Gadget at 250.00 (4,250.00), so that nothing is left.
Expected to record: whether the P&L answer still has a "Less: Closing Stock" line (empty, zero, or absent), what the Stock Summary shows, whether Negative Stock is an empty envelope.

## X3 (after R), the item inside a stock group
Key: stock group `Lab Group` under Primary; alter `Lab Gadget` so that it sits under `Lab Group`. Screenshot of the Stock Summary.
Expected to record: how many "Opening Stock" and "Less: Closing Stock" lines the P&L answer has, and under which names; whether the figures change from S.

## X4 (after R), FIFO
Key: set the costing method of Lab Gadget to FIFO, if Tally offers the field (the item screen showed none when D was built; the field may need a feature switched on in F11, for example stock valuation methods). If it is not offered after one feature switch, STOP this experiment and report what the screen offers; do not look for another way.
Expected to record: P&L Closing Stock and Cost of Sales, Stock Summary value (the average-cost figure is 2,040; on first-in first-out the 13 Nos sold come from the oldest lots, leaving 7 Nos at 120.00 and 10 Nos at 140.00 = 2,240.00: write down what the screen shows), whether Current Assets and the nett profit follow.

## X5 (after R), opening stock edited after the books started
Key: change Lab Gadget's opening value from 10 Nos at 100.00 to 10 Nos at 110.00 (1,100.00), nothing else.
Expected to record: P&L Opening Stock and Closing Stock, nett profit, Balance Sheet (does it still balance, or does a "Difference in opening balances" appear on the screen), and whether the ledger collection changes.
After X5: R once more, and send `m-marks-D`.

## Not measured by this request
A split or carried-forward second year, godowns, batches, GST, another release.
