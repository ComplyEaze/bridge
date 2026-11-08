# Row 692-R6: does the stock summary that `stock_summary` sends serve rule 2 for every window the opening-stock run read? A look-only check of kept answers

Not for merge. Part of #692. Written 8 Nov 2026. Lines below are at master 76705cfb. Look only: nothing is sent to Tally, and nothing is loaded, keyed or restored. Confidence until it runs: unverified.

## Question
Rule 2 of the 5 Nov comment refuses a statement when any Stock Summary row has no value. It needs a Stock Summary read for the statement's window, through a request a release already sends.

The 21 Oct decision recorded two limits on `stock_summary`: a 31 March date only, and a master mark at most 874. Neither stands on master:
- **#2153 (af764d3a)** reads it as of the last day of any month (`StockSummaryAsOf`, `bridge-tally-protocol/src/native_stock_summary.rs:94-102`). The period always starts on 1 April of the as-of date's year (`agent_stock_summary.rs:28`).
- **#2180 (b8eefae1)** admits a book past that mark on a census count of its items and groups (`admit_counted_stock`, `tally/runtime_stock_summary.rs:257-292`). A book whose estimate does not fit is refused as `stock_summary_too_large`.

The shipped request is the built-in report envelope, with the period's two dates and nothing else (`render_native_stock_summary_request`, `native_stock_summary.rs:87`; `render_report_envelope`, `native_statement_reports.rs:236-246`). The opening-stock run's `w*-stock-summary` requests have that same envelope with their window's dates (`lab/692-opening-stock` at 991c3ec4, `requests/`).

So the open point is narrow. For a window that does not start on 1 April, or that spans two years, the shipped tool would send a different period from the one the run sent:
- **w4 (1 Oct 2025 to 31 Mar 2026) and R5's w5 (1 Apr 2024 to 31 Mar 2026):** the shipped tool sends 1 Apr 2025 to 31 Mar 2026, which is w2's request.
- **w1, w2 and w3:** the shipped request is the run's own.

Does the stock summary as of a window's end carry the same rows, quantities and values whatever the period's start? If so, rule 2 can use the shipped request as it is.

## Kit
On the lab PC, the kept answers of `lab/692-opening-stock-answers` at 09287f70, each checked first against its line in that branch's `SHA256.txt` by bytes and sha256:
- `answers/r1-w1-stock-summary`, `r1-w2-…`, `r1-w3-…` and `r1-w4-…`;
- `r5-w5-stock-summary`;
- `r3-w2-stock-summary`;
- `r1-w2-negative-stock`, `r1-w4-negative-stock`, `r5-w5-negative-stock` and `r3-w2-negative-stock`;
- the eight requests behind them, from the sheet branch.

A script `same6.py` reads each answer with expat, never a text search. It prints only verdicts, counts and lengths, never a name or an amount:
- for each pair, whether the two answers hold the same top-level rows, element for element, and their row counts;
- the count of rows with a quantity and no value;
- whether each request file equals the shipped envelope with its window's dates. The script rebuilds the envelope from the format string at `native_statement_reports.rs:240-246`.

## Steps
0. Check every kept file against `SHA256.txt`. A mismatch: stop, the row has no evidence.
1. **w4 against w2:** `r1-w4-stock-summary` against `r1-w2-stock-summary`, and `r1-w4-negative-stock` against `r1-w2-negative-stock`.
2. **w5 against w2:** `r5-w5-stock-summary` against `r1-w2-stock-summary`, and the same for negative stock. R5 was read straight after R1 on the same state (RESULTS.md, "R5").
3. **R3's row without a value:** in `r3-w2-stock-summary`, the count of rows with a quantity and no value. EXPECT 1 (RESULTS.md, "R3": 3 rows, 1 without a value).
4. **The requests:** `w1-`, `w2-` and `w3-stock-summary` against the shipped envelope for as-of 31 Mar 2025, 31 Mar 2026 and 30 Sep 2025, with the book's name.

## Stop rules
- A kept file missing or not equal to its line: stop, and record it.
- Nothing here is sent to Tally. A step that would need a send is a new row, not this one.

## Record
For each comparison: equal or not, the row counts on each side, and the counts of rows without a value. For step 4, equal or not for each request.

## Decides
- **Both of steps 1 and 2 equal:** rule 2 uses `stock_summary`'s shipped request as of the window's end, as it is. A window that does not end on a month's last day is refused, because `StockSummaryAsOf` refuses it, and the statement is then not established. Nothing new is sent.
- **Either unequal:** the shipped period does not serve that window. Rule 2 then refuses every window that does not start on 1 April of one financial year. Serving those windows would need a candidate request with the window's own start, reviewed as a new shape (P7) and measured live before a release sends it.
- **Step 3 equal to 1:** the R3 answer, once committed as a fixture, is the test that the build's parser reads a row without a value as one. That test is part of (i).
- **Step 4 not equal:** the run's verdicts do not carry to the shipped request, and a read row is needed.

## Not covered (and why no Tally row is asked)
- **A book past the 874 mark with opening stock.** No synthetic book with both is known. Rule 2 inherits `stock_summary`'s census admission, so such a book that does not fit is refused, never misread.
- **The other unmeasured cases** (TallyPrime 7.1, batches, godowns, GST, a company split into a new year) stay with the real-book release gate.

Keep: `lab/692-r6-check-answers`, with same6.py's whole output and `SHA256.txt` of what it read. No amount and no name is committed.
