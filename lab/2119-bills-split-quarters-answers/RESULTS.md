# Results: #2119, Bills Receivable to 31 March 2026 with four first days, BRIDGE CORPUS DENSE, 29 Oct 2026. **The first day (`SVFROMDATE`) changes nothing.** All six answers are the same 13,679,490 bytes (one sha256), with all 29,900 bills, including those dated before the first day. Every request took 33.0 to 39.6 s. So no first day makes a part smaller or faster on this report. Reads only; the marks did not move

The sheet is `lab/2119-bills-split-quarters` at 53e219e14d2191494ce6cd52d3d2342c8baf40dc. `SHA256.txt` matched 9 of 9, and `gen_requests.py` checked ok and reproduced the four requests byte for byte.

**Who acted.** The lab, on the owner's standing word of 27 Oct, on the synthetic company.

## Setup
- **Build:** the published 0.6.0 `bridge_mcp.exe` (sha256 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44), the same before and after. It was used only for the company and marks reads (steps 1 and 4) and was stopped during the raw reads.
- **Company:** BRIDGE CORPUS DENSE, 100012, d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, `verified_tuple`, books from 20250401, loaded alone. Tally's current date 31-Mar-2026.
- **Marks:** M 328 and V 29900 before the first raw read and after the last (`reused` false).
- **One Tally:** at step 0 and at the end, one tally.exe (13992) owned port 9001, and `/status` answered 200.

## The six reads
Each is curl by C3's line with the sheet's timing fields. Each answered 200, with bytes equal to CONTENT-LENGTH and curl exit 0.

| read | first day | start (UTC) | to first byte | total | bytes | sha256 | `read.py` |
|---|---|---|---|---|---|---|---|
| A1 | 20250401 | 06:40:20.199 | 39.499 s | 39.642 s | 13,679,490 | 5a406cab… | BILLFIXED 29900, sum -1425845144.50 |
| A2 | 20260101 | 06:41:12.904 | 33.989 s | 34.134 s | 13,679,490 | 5a406cab… | the same |
| A3 | 20251001 | 06:42:00.980 | 34.485 s | 34.635 s | 13,679,490 | 5a406cab… | the same |
| A4 | 20250701 | 06:42:41.892 | 33.293 s | 33.442 s | 13,679,490 | 5a406cab… | the same |
| A5 | 20250401 | 06:43:26.663 | 33.601 s | 33.764 s | 13,679,490 | 5a406cab… | the same |
| A6 | 20260101 | 06:44:06.481 | 33.046 s | 33.218 s | 13,679,490 | 5a406cab… | the same |

A1 is also byte for byte the timing sitting's R4 (`lab/2119-bills-read-timing-answers`), which took 41.5 s then.

## The comparisons (`read.py compare`, against A1 and again against A5)
| part | first day | rows | rows before the first day | full report's rows from that day | matched | only in the part | only in the full | subset |
|---|---|---|---|---|---|---|---|---|
| A2, A6 | 20260101 | 29900 | 22520 | 7380 | 7380 | 22520 | 0 | no |
| A3 | 20251001 | 29900 | 14976 | 14924 | 14924 | 14976 | 0 | no |
| A4 | 20250701 | 29900 | 7432 | 22468 | 22468 | 7432 | 0 | no |
| A5 (the full report twice) | 20250401 | 29900 | 0 | 29900 | 29900 | 0 | 0 | yes |

## Reading
- **(1) does not hold.** The first day does not limit the rows. Each "part" is the whole report, byte for byte, holding the bills dated before its first day too. The bills from each first day on are matched exactly, because they are simply all there.
- **(2) cannot be measured with this key.** No part returned fewer rows, so time against rows has nothing to compare. The six times differ only by sitting effects: A1, the first request, at 39.6 s; the other five at 33.0 to 34.6 s, two of them repeats of the same request.
- **For #2119:** a split of the Bills Receivable report by `SVFROMDATE` returns nothing smaller on this build of Tally. Each request still takes well over the 20-second limit on this book. The sheet's note stands: a split by another key (by ledger, under `Ledger Outstandings`) is still to be measured.

**Not covered:**
- a split with an end date;
- the `Ledger Outstandings` report;
- the Payable report;
- other `as_of` days, other books, Gold;
- any change to the tool.

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `answers/`: the three tool calls, whole.
- `client-log.txt`.
- `evidence/`:
  - each read's headers, curl's printed fields and its error stream (empty);
  - `times.txt`;
  - `compare.txt`;
  - the step 0 and C10 checks.
- `REQUESTS.txt`: the four requests by sha256.
- `RAW-ANSWERS.txt`: the six answers by size and sha256 (kept on the lab PC).
- `scripts/rawt.ps1`.
- `substitutions.txt`, `notes.txt` (UTC), and `SCREENSHOTS.txt` (the captures, kept on the lab PC).
