# Row 692-R6: the kept w4 and w5 stock-summary answers equal w2's, so the shipped request as of the window's end serves rule 2 for every window the opening-stock run read

**First line: steps 1 and 2 equal, step 3 counts 1 (EXPECT 1), and step 4 is equal for w1, w2 and w3.** Look only. Nothing was sent to Tally, and nothing was loaded, keyed or restored. (verified)

Sheet `lab/692-r6/RUN-SHEET.md` at 44287240 (sha256 993f460c…, equal to its `SHA256.txt`). The kept answers are those of `lab/692-opening-stock-answers` at 09287f70. The requests are from that branch's `lab/692-opening-stock/requests/`. `same6.py` reads each answer with expat and prints only verdicts, counts and lengths; its whole output is `evidence/same6-output.txt`.

## Step 0: the kept files

All ten kept answers the sheet names are on the lab PC. Each equals its line in `SHA256.txt` at 09287f70, by bytes and sha256 (`evidence/step0-check.txt`, from `check0.py`): 10 files, 10 equal. (verified)

## Steps 1 to 3

| step | comparison | rows each side | rows with a quantity and no value | element for element |
|---|---|---|---|---|
| 1a | r1-w4 stock summary against r1-w2 | 2 and 2 | 0 and 0 | equal: yes |
| 1b | r1-w4 negative stock against r1-w2 | 0 and 0 | 0 and 0 | equal: yes |
| 2a | r5-w5 stock summary against r1-w2 | 2 and 2 | 0 and 0 | equal: yes |
| 2b | r5-w5 negative stock against r1-w2 | 0 and 0 | 0 and 0 | equal: yes |
| 3 | r3-w2 stock summary | 3 | **1** (EXPECT 1: met) | – |

For the record: r3-w2 negative stock has 1 row, with a quantity and no value. r1-w1 and r1-w3 stock summary each have 2 rows, none without a value. (verified)

The pairs of steps 1 and 2 are also byte-equal: r1-w4, r5-w5 and r1-w2 stock summary are each 962 B, d981168c…, and the three negative-stock answers are each 46 B, 8d37111f…, by the listed sha256. (verified)

## Step 4: the requests

The shipped envelope was rebuilt from `render_report_envelope`'s format string (`native_statement_reports.rs:240-246` at 76705cfb). It uses the id `Stock Summary` and no extra variables (`render_built_in_report_request`). Its period runs from 1 April of the financial year that holds the as-of date to the as-of date (`financial_year_start`, `tally/runtime_stock_summary.rs:117-127`). The company is the one name all five requests carry; only its length is printed.

| request | shipped request as of | its period | equal |
|---|---|---|---|
| w1 stock summary | 31 Mar 2025 | 1 Apr 2024 to 31 Mar 2025 | yes |
| w2 stock summary | 31 Mar 2026 | 1 Apr 2025 to 31 Mar 2026 | yes |
| w3 stock summary | 30 Sep 2025 | 1 Apr 2025 to 30 Sep 2025 | yes |
| w4 stock summary (for the record) | 31 Mar 2026 | 1 Apr 2025 to 31 Mar 2026 | no: w4 starts 1 Oct 2025, as the sheet says |
| w5 stock summary (for the record) | 31 Mar 2026 | 1 Apr 2025 to 31 Mar 2026 | no: w5 starts 1 Apr 2024, as the sheet says |

(verified)

**Not established:** `stock_summary_period` also moves the start to the book's own first day when that is later (`tally/runtime_stock_summary.rs:146-148`). This row did not read the book's first day. The rebuild assumes it is on or before 1 April 2024. The run's w1 and w5 read from 1 April 2024, but that does not show where the book starts.

## What it decides, by the sheet's rules

- **Steps 1 and 2 both equal:** on this book, the stock summary and negative stock as of 31 March 2026 hold the same rows whether the period starts on 1 October 2025, 1 April 2025 or 1 April 2024. So rule 2 can use `stock_summary`'s shipped request as of the window's end, as it is. A window that does not end on a month's last day is refused by `StockSummaryAsOf`, and its statement is not established. Nothing new is sent. One book, one as-of date: PARTIAL.
- **Step 3 equal to 1:** once committed as a fixture, the R3 answer is the test that the build's parser reads a row without a value as one.
- **Step 4 equal:** the run's verdicts for w1, w2 and w3 carry over to the shipped request. They are subject to the book's first day above.

## Kept

Only verdicts, counts, lengths and hashes are committed. No name and no amount. The kept answers stay on the lab PC under their 09287f70 `SHA256.txt` lines. `SHA256.txt` here lists what this row read (the sheet, the ten answers, the five requests) and the files it commits.
