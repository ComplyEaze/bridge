# Lab request: stock items and Stock Summary to two mid-year dates on BRIDGE TIEOUT D (READ-ONLY, no keying)

For #1233: `stock_summary` admits only a 31 March today, because no capture shows whether the stock items collection
and Tally's Stock Summary follow a period end inside the year. This request reads both, as `stock_summary` sends them,
to 30 September and to 31 May 2025 on a book whose stock changes after each date.

## Rules
- **LAB-COMMON v3 applies in full** (`LAB-COMMON.md` here, a byte copy from `lab/692-opening-stock-d3-requests` at
  a8e39f8c, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6); where it and this request
  differ, the stricter holds.
- Only `BRIDGE TIEOUT D`, already loaded, at its starting state S of the five-experiment request (`lab/692-opening-stock-d3-requests`):
  D after its October sale, Lab Gadget 17 Nos at 120.00 (2,040.00). Never any other company. **Nothing is keyed,
  imported, backed up or restored by this request.**
- **When:** inside the five-experiment request's X0 group (state S, after `j-r4`, before X1), or on its own at any time the book is at S (that request ends
  with a restore to S). If the book is not at S (its year Profit and Loss Closing Stock is not 2,040 on Tally's screen),
  STOP and send nothing.
- **Requests by curl** under the owner's decision of 26 Oct 2026 (requests sent by curl may be used to discover shapes).
  Every file in `requests/` is an existing builder's request with only the company and dates changed:
  `k1` and `k3` are the committed `stock_items_fy_request.utf16le.xml` (what `render_audit_stock_items` renders) with
  `SVCURRENTCOMPANY` `BRIDGE TIEOUT D` and `SVTODATE` 20250930 and 20250531; `k4` is the five-experiment request's `j-r4` (the committed
  `stock_summary_report_*` request shape) with `SVTODATE` 20250531. No new builder is made from them.
- **Request-ID check** before the first send, in this folder: `python -I check-request-ids.py requests`; EXPECT
  `census: ok 3, skipped 1, FAIL 0`. The skipped file is `k4`, a report request (TYPE Data) that defines no collection:
  check by eye that its ID is `Stock Summary` and that it defines no `COLLECTION`. Fill `@@GUID@@` in
  `m-marks-TEMPLATE` with TIEOUT D's identifier, save it as `m-marks-D` (or reuse the five-experiment request's `m-marks-D`), and run the check
  on it (EXPECT `ok`). Anything else = STOP, nothing sent.
- **Read bounds (C3):** the marks read 8,192 bytes; every other read 2,000,000 bytes. An answer above its bound = STOP.
- One request at a time. Save every answer as received (BOM-less UTF-16LE) with size and sha256; a failed request is
  reported as failed, never as an empty answer. Request files are UTF-16LE with a BOM exactly as here
  (`SHA256.txt`); do not edit them.
- Any dialog or message box these steps do not describe = STOP (C7): press no key, send nothing more, screenshot, copy
  its text, report.
- Release: record it from Tally's About screen (the lab PC runs TallyPrime 7.2 since 25 Oct 2026; TIEOUT D was built
  on 7.1; a prompt on opening it is a STOP, as in that request).

## Steps
1. `m-marks-D`. Record `ALTVCHID` and `ALTMSTID`.
2. `k1` stock items, 1 Apr to 30 Sep 2025.
3. The five-experiment request's `j-r4` Stock Summary, 1 Apr to 30 Sep 2025, if not already sent at S in this sitting (its answer is then reused
   under its own name; do not send it twice).
4. `k3` stock items, 1 Apr to 31 May 2025.
5. `k4` Stock Summary, 1 Apr to 31 May 2025.
6. `m-marks-D` again: must equal step 1, or STOP and report.
7. Screenshots of Tally's own Stock Summary for 1-4-2025 to 30-9-2025 and for 1-4-2025 to 31-5-2025 (period changed
   with Alt+F2 on the Stock Summary screen only; nothing else changed).

## What to record (a record to compare, not pass/fail)
For each of `k1` and `k3`: the `CLOSINGBALANCE` and `CLOSINGVALUE` of `Lab Gadget`, written exactly as sent. For `j-r4`
and `k4`: the item's closing quantity and amount (`DSPCLQTY`, `DSPCLAMTA`) and the report total.
- 30 September 2025: the book holds 22 Nos (10 opening, 10 bought 5 May, 10 bought 5 June, 8 sold 20 June; the next
  sale is 15 October). The year Profit and Loss read before the October sale closed at 2,640.00 (the committed
  `statement_profit_and_loss_opening_stock_book_fy_live`), so 22 Nos at 120.00 = 2,640.00 is expected; on the wire a
  value of stock held is written negative.
- 31 May 2025: 20 Nos (10 opening and 10 bought 5 May). Its value is **not known**: 2,200.00 if Tally averages over
  the inwards to that date, something else if not. Write down what the answers and the screen show.
- **The finding this request is for:** whether `k1`/`k3` show the closing at their own `SVTODATE` (22 Nos; 20 Nos) or
  the closing at another date (17 Nos would be the closing after the October sale); and whether each stock items
  answer and its Stock Summary give the same item value.

## Pass / fail
PASS: every read answered with HTTP 200, saved with size and sha256, under its bound; the marks equal before and after.
FAIL: anything else, reported as it happened.

## Evidence
On a new branch `lab-evidence/1233-stock-mid-year-answers`: the answers, `SHA256.txt` listing every file, and the
recorded values above, after the pre-push scan and the run coordinator's yes. Screenshots stay on the lab PC, listed by
name, size and sha256.

## Not measured by this request
A costing method other than the default, a second item, a stock group, godowns, batches, a window across two
financial years, another book.
